extern crate clap;
extern crate csv;
extern crate env_logger;
extern crate geo;
extern crate geo_types; // the old version used by proj, not the new ones re-exported by geo
extern crate parcelscan;
#[macro_use]
extern crate log;
extern crate num_traits;
extern crate proj;
#[macro_use]
extern crate serde_derive;
extern crate wkt;

use clap::App;
use clap::AppSettings;
use clap::Arg;
use clap::SubCommand;
use geo::algorithm::bounding_rect::BoundingRect;
use geo::algorithm::centroid::Centroid;
use geo::algorithm::contains::Contains;
use geo::algorithm::euclidean_distance::EuclideanDistance;
use geo::algorithm::map_coords::MapCoords;
use geo::algorithm::closest_point::ClosestPoint;
use geo::prelude::Area;
use geo::Geometry;
use geo::MultiPolygon;
use geo::Point;
use geo::Closest;
use parcelscan::sflanduse::LandUseRecord;
use parcelscan::sfplanningacela::PPTSRecord;
use proj::Proj;
use rstar::RTree;
use rstar::RTreeObject;
use rstar::AABB;
use std::error::Error;
use std::fs::File;
use wkt::Wkt;
use parcelscan::polygon_wrapper::{PolygonWrapper, parse_wkt_to_multipolygon};
use parcelscan::sfzoningdistricts::{ZoningDistrict, get_zoning};
use rstar::PointDistance;
use std::io::Read;
use parcelscan::geo_util::default_projection;

fn get_neighboring_parcels<'a>(
    rtree: &'a RTree<PolygonWrapper<LandUseRecord>>,
    shape: &MultiPolygon<f64>,
    deg_to_ft_proj: &Proj,
    radius_ft: f64,
) -> Result<Vec<&'a LandUseRecord>, Box<Error + Send + Sync + 'static>> {
    let centroid: Point<f64> = shape
        .centroid()
        .expect("multipolygon should have at least one point");
    let centroid_ft = deg_to_ft_proj
        .project(geo_types::Point::new(centroid.0.x, centroid.0.y), false)
        .map(|proj_point| Point::new(proj_point.0.x, proj_point.0.y))
        .expect("Failed to project LandUse parcel");
    let p1_ft = centroid_ft - Point::from((radius_ft, radius_ft));
    let p2_ft = centroid_ft + Point::from((radius_ft, radius_ft));
    let p1_deg = deg_to_ft_proj
        .project(geo_types::Point::new(p1_ft.0.x, p1_ft.0.y), true)
        .map(|proj_point| Point::new(proj_point.0.x, proj_point.0.y))
        .expect("Failed to reverse project LandUse parcel");
    let p2_deg = deg_to_ft_proj
        .project(geo_types::Point::new(p2_ft.0.x, p2_ft.0.y), true)
        .map(|proj_point| Point::new(proj_point.0.x, proj_point.0.y))
        .expect("Failed to reverse project LandUse parcel");
    let bbox = AABB::from_corners([p1_deg.0.x, p1_deg.0.y], [p2_deg.0.x, p2_deg.0.y]);
    let mut neighboring_parcels = vec![];
    for parcel_wrapper in rtree.locate_in_envelope_intersecting(&bbox) {
        let other_centroid = parcel_wrapper
            .multi_polygon
            .centroid()
            .expect("PPTS multipolygon should have at least one point");
        let other_centroid_ft = deg_to_ft_proj
            .project(
                geo_types::Point::new(other_centroid.0.x, other_centroid.0.y),
                false,
            )
            .map(|proj_point| Point::new(proj_point.0.x, proj_point.0.y))
            .expect("Failed to project PPTS parcel");
        if shape.contains(&other_centroid) || parcel_wrapper.multi_polygon.contains(&centroid) {
            // skip self parcel
            continue;
        } else if centroid_ft.euclidean_distance(&other_centroid_ft) < radius_ft {
            neighboring_parcels.push(&parcel_wrapper.value);
        }
    }
    Ok(neighboring_parcels)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct OutputRow {
    address: String,
    zoning_district: String,
    date: String,
    num_neighbor_units: usize,
    mean_unit_size: f64,
    neighbor_mean_unit_size: f64,
    proposed_far: f64,
    building_sqft_exist: f64,
    building_sqft_prop: f64,
    is_demolition: bool,
    is_major_expansion: bool,
    market_rate_units_exist: i64,
    market_rate_units_prop: i64,
    affordable_units_exist: i64,
    affordable_units_prop: i64,
    is_prohibited: bool,
}

fn expansions(
    mut planning_rdr: csv::Reader<File>,
    mut land_use_rdr: csv::Reader<File>,
    mut zoning_districts_rdr: csv::Reader<File>,
    mut output_write: Option<csv::Writer<File>>,
) -> Result<(), Box<Error + Send + Sync + 'static>> {
    info!("Expansions");
    let proj = default_projection();

    info!("Scanning LandUse table of all parcels");
    let mut parcels_vec: Vec<PolygonWrapper<LandUseRecord>> = vec![];
    for result in land_use_rdr.deserialize::<LandUseRecord>() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        let multi_polygon = parse_wkt_to_multipolygon(&record.the_geom)?;
        parcels_vec.push(PolygonWrapper::new(multi_polygon, record));
    }
    info!("Generating LandUse rtree of all parcels");
    let rtree = RTree::bulk_load(parcels_vec);

    info!("Scanning Zoning_Districts to make lookup table of zoning");
    let mut zoning_districts_vec: Vec<PolygonWrapper<ZoningDistrict>> = vec![];
    for result in zoning_districts_rdr.deserialize::<ZoningDistrict>() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        let multi_polygon = parse_wkt_to_multipolygon(&record.the_geom)?;
        zoning_districts_vec.push(PolygonWrapper::new(multi_polygon, record));
    }
    info!("Generating LandUse rtree of all parcels");
    let zoning_districts_rtree = RTree::bulk_load(zoning_districts_vec);


    info!("Scanning PPTS records of applications");
    let mut num_prohibited_expansions = 0;
    let mut num_ok_expansions = 0;
    let mut num_prohibited_units = 0;
    let mut num_ok_units = 0;
    let mut num_prohibited_aff_units = 0;
    let mut num_ok_aff_units = 0;
    for result in planning_rdr.deserialize::<PPTSRecord>() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        if !(
            record.record_status.contains("Permitted")
            || record.record_status.contains("Issued")
            || record.record_status.contains("Complete")
            || record.record_status.contains("Approved")
            || record.record_status.contains("Accepted")
            || record.record_status.contains("Approval BOS")
        ) {
            continue;
        }
        let is_residential_expansion = record.land_use_residential_exist.unwrap_or(0.0) > 0.0
            && record.land_use_residential_net.unwrap_or(0.0) > 0.0;
        if !is_residential_expansion {
            continue;
        }
        let approx_old_bldg_area = record.land_use_rc_exist.unwrap_or(0.0)
            + record.land_use_residential_exist.unwrap_or(0.0)
            + record.land_use_cie_exist.unwrap_or(0.0)
            + record.land_use_pdr_exist.unwrap_or(0.0)
            + record.land_use_office_exist.unwrap_or(0.0)
            + record.land_use_medical_exist.unwrap_or(0.0)
            + record.land_use_visitor_exist.unwrap_or(0.0)
            + record.land_use_parking_spaces_exist.unwrap_or(0.0)
            + 0.0;
        let approx_new_bldg_area = record.land_use_rc_prop.unwrap_or(0.0)
            + record.land_use_residential_prop.unwrap_or(0.0)
            + record.land_use_cie_prop.unwrap_or(0.0)
            + record.land_use_pdr_prop.unwrap_or(0.0)
            + record.land_use_office_prop.unwrap_or(0.0)
            + record.land_use_medical_prop.unwrap_or(0.0)
            + record.land_use_visitor_prop.unwrap_or(0.0)
            + record.land_use_parking_spaces_prop.unwrap_or(0.0)
            + 0.0;
        let approx_net_bldg_area = approx_new_bldg_area - approx_old_bldg_area;
        let is_building_expansion = approx_net_bldg_area > 0.0;
        let is_demolition = record.demolition == "CHECKED";
        if !is_building_expansion && !is_demolition {
            continue;
        }
        let shape: MultiPolygon<f64> = parse_wkt_to_multipolygon(&record.the_geom)?;
        let shape_ft = shape.map_coords(|&(lat, lon)| {
            // convert to the old version of geo_types used by proj, and then convert back
            // to the new re-exported one used by geo
            let point = proj
                .project(geo_types::Point::new(lat, lon), false)
                .unwrap_or_else(|err| {
                    panic!(format!("Projection failed on ({} {}): {}", lat, lon, err));
                });
            (point.x(), point.y())
        });
        let parcel_area = shape_ft.area().abs(); // note: area() is signed area
        let far = approx_new_bldg_area / parcel_area;

        let neighbors = get_neighboring_parcels(&rtree, &shape, &proj, 300f64)?;
        let (num_neighbor_units, neighbor_bldgsqft) = neighbors
            .iter()
            .map(|land_use_record| {
                (land_use_record.resunits, land_use_record.bldgsqft)
            })
            .fold::<(usize, usize), _>((0usize, 0usize), |acc, item| {
                (acc.0 + item.0, acc.1 + item.1)
            })
            ;
        let neighbor_mean_unit_size: f64 = if num_neighbor_units > 0 {
            neighbor_bldgsqft as f64 / num_neighbor_units as f64
        } else {
            std::f64::INFINITY
        };

        let market_rate_units_exist = record.prj_feature_market_rate_exist.unwrap_or(0.0) as i64;
        let affordable_units_exist = record.prj_feature_affordable_exist.unwrap_or(0.0) as i64;
        let market_rate_units_prop = record.prj_feature_market_rate_prop.unwrap_or(0.0) as i64;
        let affordable_units_prop = record.prj_feature_affordable_prop.unwrap_or(0.0) as i64;
        let num_aff_units_change = affordable_units_prop - affordable_units_exist;
        let housing_units_prop = market_rate_units_prop + affordable_units_prop;
        let num_units_change =
            market_rate_units_prop - market_rate_units_exist + num_aff_units_change;
        let zoning_district = get_zoning(&zoning_districts_rtree, &shape)
            .map(|zoning_district| &*zoning_district.zoning_sim)
            .unwrap_or("");
        let adus = record.residential_adu_studio_prop.unwrap_or_default() +
            record.residential_adu_1br_prop.unwrap_or_default() +
            record.residential_adu_2br_prop.unwrap_or_default() +
            record.residential_adu_3br_prop.unwrap_or_default();

        let major_expansion_threshold_pct = 10.0;
        let building_expand_pct = approx_net_bldg_area / approx_old_bldg_area * 100.0;
        let is_major_expansion = building_expand_pct >= major_expansion_threshold_pct &&
            (
                zoning_district == "RH-1(D)" && far > 0.5 ||
                zoning_district == "RH-1" && far > 0.6 ||
                zoning_district == "RH-2" && housing_units_prop == 1 && far > 0.6 ||
                zoning_district == "RH-2" && far > 1.2 ||
                zoning_district == "RH-3" && housing_units_prop == 1 && far > 0.6 ||
                zoning_district == "RH-3" && housing_units_prop == 2 && far > 1.2 ||
                zoning_district == "RH-3" && far > 1.8
            ) &&
            adus == 0
        ;
        let mean_unit_size = if housing_units_prop > 0 {approx_new_bldg_area / housing_units_prop as f64} else {0.0};
        let is_prohibited = mean_unit_size > f64::min(neighbor_mean_unit_size, 1200.0)
            && (is_major_expansion && housing_units_prop >= 2 || is_demolition);
        if is_prohibited {
            num_prohibited_expansions += 1;
            num_prohibited_units += num_units_change;
            num_prohibited_aff_units += num_aff_units_change;
        } else {
            num_ok_expansions += 1;
            num_ok_units += num_units_change;
            num_ok_aff_units += num_aff_units_change;
        }
        let o = OutputRow {
            address: record.address,
            zoning_district: zoning_district.to_string(),
            date: record.date_opened.date().to_string(),
            num_neighbor_units,
            mean_unit_size,
            neighbor_mean_unit_size,
            proposed_far: far,
            building_sqft_exist: approx_old_bldg_area,
            building_sqft_prop: approx_new_bldg_area,
            is_demolition,
            is_major_expansion,
            market_rate_units_exist,
            market_rate_units_prop,
            affordable_units_exist,
            affordable_units_prop,
            is_prohibited,
        };
        print_row(&o);
        if let Some(output_write) = output_write.as_mut() {
            output_write.serialize(o)?;
        }
    }
    let frac =
        num_prohibited_expansions as f64 / (num_ok_expansions + num_prohibited_expansions) as f64;
    info!(
        "ok expansions: {}; prohibited expansions: {} ({}%)",
        num_ok_expansions,
        num_prohibited_expansions,
        (frac * 100.0) as i32
    );
    let net_units_frac = num_prohibited_units as f64 / (num_ok_units + num_prohibited_units) as f64;
    info!(
        "ok net units: {}; prohibited net units: {} ({}%)",
        num_ok_units,
        num_prohibited_units,
        (net_units_frac * 100.0) as i32
    );
    let net_aff_units_frac =
        num_prohibited_aff_units as f64 / (num_ok_aff_units + num_prohibited_aff_units) as f64;
    info!(
        "ok net “affordable” units: {}; prohibited net units: {} ({}%)",
        num_ok_aff_units,
        num_prohibited_aff_units,
        (net_aff_units_frac * 100.0) as i32
    );
    Ok(())
}
fn print_row(o: &OutputRow) {
    let prohibited_msg = if o.is_prohibited {"Prohibited"}
    else if o.is_major_expansion {"Conditional"}
    else {"Probably OK"};
    let project_type = if o.is_demolition {"demolition"}
    else if o.is_major_expansion {"major expansion"}
    else {"other"};
    println!(
        "{prohibited_msg} {project_type}: {date} address: {address}, zone {zoning_district}, far {proposed_far:.02}, unit size: {mean_unit_size:.02}, neighbor mean size: {neighbor_mean_unit_size:.02} ({num_neighbor_units} units), bldg growth {approx_net_bldg_area:.0}sqft ({building_expand_pct:.0}%), mktrateunit: {market_rate_units_exist}→{market_rate_units_prop}, affunit:{affordable_units_exist}→{affordable_units_prop}",
        prohibited_msg = prohibited_msg,
        project_type = project_type,
        date = o.date,
        address = o.address,
        zoning_district = o.zoning_district,
        proposed_far = o.proposed_far,
        mean_unit_size = o.mean_unit_size,
        neighbor_mean_unit_size = o.neighbor_mean_unit_size,
        num_neighbor_units = o.num_neighbor_units,
        approx_net_bldg_area = o.building_sqft_prop - o.building_sqft_exist,
        building_expand_pct = (o.building_sqft_prop - o.building_sqft_exist) / o.building_sqft_exist * 100.0,
        market_rate_units_exist = o.market_rate_units_exist,
        market_rate_units_prop = o.market_rate_units_prop,
        affordable_units_exist = o.affordable_units_exist,
        affordable_units_prop = o.affordable_units_prop,
    );
}

fn main() -> Result<(), Box<Error + Send + Sync + 'static>> {
    env_logger::init();
    let matches = App::new("peskinexpansionsimpact")
        .version("0.0")
        .about("Show numbers of residential expansions that are probably prohibited by proposal")
        .author("Yonathan.")
        .after_help("Print projects most likely affected by Peskin ordinance (Board File 181216). Note: everything is output using the logger, so you should set RUST_LOG=peskinexpansionsimpact=info to see the output.")
        .subcommand(SubCommand::with_name("expansions")
            .arg(Arg::with_name("planning")
                .long("planning")
                .help("Planning CSV file named PPTS_Records_data.csv from https://data.sfgov.org/Housing-and-Buildings/PPTS-Records/7yuw-98m5")
                .required(true)
                .takes_value(true)
            )
            .arg(Arg::with_name("land-use")
                .long("land-use")
                .help("parcels csv file LandUse2016.csv https://data.sfgov.org/Housing-and-Buildings/Land-Use/us3s-fp9q")
                .required(true)
                .takes_value(true)
            )
            .arg(Arg::with_name("zoning-districts")
                .long("zoning-districts")
                .help("zoning map file Zoning_Map_-_Zoning_Districts_data.csv https://data.sfgov.org/Geographic-Locations-and-Boundaries/Zoning-Map-Zoning-Districts/xvjh-uu28")
                .required(true)
                .takes_value(true)
            )
            .arg(Arg::with_name("out-projects")
                .long("out-projects")
                .help("csv file output")
                .takes_value(true)
            )
            .about("Show information about expansions")
        )
        .subcommand(SubCommand::with_name("reprint")
            .arg(Arg::with_name("projects")
                .long("projects")
                .help("csv file input, which was output by expansions")
                .required(true)
                .takes_value(true)
            )
        )
        .setting(AppSettings::SubcommandRequired)
        .get_matches();


    if let Some(matches) = matches.subcommand_matches("expansions") {
        let planning = matches
            .value_of_os("planning")
            .expect("Expected planning file");
        let zoning_districts = matches
            .value_of_os("zoning-districts")
            .expect("Expected zoning-districts file");
        let land_use_path = matches
            .value_of_os("land-use")
            .expect("Expected land-use file");
        let out_projects_path = matches.value_of_os("out-projects");
        info!(
            "Opening {} and {}",
            planning.to_string_lossy(),
            land_use_path.to_string_lossy()
        );
        let planning_file = File::open(planning)?;
        let planning_rdr = csv::Reader::from_reader(planning_file);
        let land_use_file = File::open(land_use_path)?;
        let land_use_rdr = csv::Reader::from_reader(land_use_file);
        let zoning_districts_file = File::open(zoning_districts)?;
        let zoning_districts_rdr = csv::Reader::from_reader(zoning_districts_file);
        let out_projects_writer_opt = out_projects_path
            .map(
                |path| -> Result<Option<csv::Writer<File>>, std::io::Error> {
                    info!("Opening output file {}", path.to_string_lossy());
                    let writer: File = File::create(path)?;
                    Ok(Some(csv::Writer::from_writer(writer)))
                },
            )
            .unwrap_or(Ok(None))?;
        expansions(planning_rdr, land_use_rdr, zoning_districts_rdr, out_projects_writer_opt)?;
    } else if let Some(matches) = matches.subcommand_matches("reprint") {
        let projects_path = matches.value_of_os("projects").expect("required arg should exist");
        let projects_file: Box<Read> = if projects_path == "-" {
            Box::new(std::io::stdin())
        } else {
            Box::new(File::open(projects_path)?)
        };
        let mut projects_rdr = csv::Reader::from_reader(projects_file);
        for result in projects_rdr.deserialize::<OutputRow>() {
            let o = result?;
            print_row(&o);
        }
    } else {
        panic!("Should not happen");
    }
    Ok(())
}
