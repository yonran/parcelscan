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
use geo::prelude::Area;
use geo::Geometry;
use geo::MultiPolygon;
use geo::Point;
use parcelscan::sflanduse::LandUseRecord;
use parcelscan::sfplanningacela::PPTSRecord;
use proj::Proj;
use rstar::RTree;
use rstar::RTreeObject;
use rstar::AABB;
use std::error::Error;
use std::fs::File;
use wkt::Wkt;

struct ParcelWrapper {
    multi_polygon: MultiPolygon<f64>,
    bounding_box: AABB<[f64; 2]>,
    land_use_record: LandUseRecord,
}
impl ParcelWrapper {
    fn new(multi_polygon: MultiPolygon<f64>, land_use_record: LandUseRecord) -> Self {
        let bounding_rect =
            BoundingRect::bounding_rect(&multi_polygon).expect("bounding_rect failed");
        let bounding_box: AABB<[f64; 2]> = AABB::from_corners(
            [bounding_rect.min.x, bounding_rect.min.y],
            [bounding_rect.max.x, bounding_rect.max.y],
        );
        ParcelWrapper {
            multi_polygon,
            bounding_box,
            land_use_record,
        }
    }
}
impl RTreeObject for ParcelWrapper {
    type Envelope = AABB<[f64; 2]>;

    fn envelope(&self) -> Self::Envelope {
        self.bounding_box.clone()
    }
}

fn parse_wkt_to_multipolygon(the_geom: &str) -> Result<MultiPolygon<f64>, Box<Error>> {
    // TODO: take out the expect()s, and return Err instead
    let parsed_wkt = Wkt::from_str(the_geom).expect("Could not parse WKT");
    let wkt_shape = parsed_wkt
        .items
        .into_iter()
        .next()
        .expect("Expected one multipolygon; got none");
    let multi_polygon = match wkt::conversion::try_into_geometry(&wkt_shape)
        .expect("Failed to convert polygon wkt to geo_types")
    {
        Geometry::MultiPolygon(multi_polygon) => {
            trace!(
                "multipolygon {:?} area = {}sqft",
                multi_polygon,
                multi_polygon.area().round()
            );
            multi_polygon
        }
        _ => {
            panic!("Expected only multipolygons in parcel file");
        }
    };
    Ok(multi_polygon)
}
fn get_neighboring_parcels<'a>(
    rtree: &'a RTree<ParcelWrapper>,
    shape: &MultiPolygon<f64>,
    deg_to_ft_proj: &Proj,
    radius_ft: f64,
) -> Result<Vec<&'a LandUseRecord>, Box<Error>> {
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
            neighboring_parcels.push(&parcel_wrapper.land_use_record);
        }
    }
    Ok(neighboring_parcels)
}

fn expansions(
    mut planning_rdr: csv::Reader<File>,
    mut land_use_rdr: csv::Reader<File>,
) -> Result<(), Box<Error>> {
    info!("Expansions");

    // projection for converting latitude and longitude (in degrees) into feet
    // Project using Azimuthal Equidistant, centered on Market and Van Ness
    // https://proj4.org/operations/projections/utm.html
    let proj = Proj::new(
        "
    +proj=pipeline
    +step +proj=unitconvert +xy_in=deg +xy_out=rad
    +step +proj=aeqd +lat_0=37.773972 +lon_0=-122.431297
    +step +proj=unitconvert +xy_in=m +xy_out=us-ft
    ",
    )
    .expect("Failed to create projection");

    info!("Scanning LandUse table of all parcels");
    let mut parcels_vec: Vec<ParcelWrapper> = vec![];
    for result in land_use_rdr.deserialize::<LandUseRecord>() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        let multi_polygon = parse_wkt_to_multipolygon(&record.the_geom)?;
        parcels_vec.push(ParcelWrapper::new(multi_polygon, record));
    }
    info!("Generating LandUse rtree of all parcels");
    let rtree = RTree::bulk_load_parallel(parcels_vec);
    info!("Scanning PPTS records of applications");
    let mut num_prohibited_expansions = 0;
    let mut num_ok_expansions = 0;
    for result in planning_rdr.deserialize::<PPTSRecord>() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
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
        let shape_ft = shape.map_coords(&|&(lat, lon)| {
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
        let neighbors_mean_far: f64 = neighbors
            .iter()
            .map(|land_use_record| {
                land_use_record.bldgsqft as f64 / land_use_record.shape_area as f64
            })
            .sum::<f64>()
            / neighbors.len() as f64;

        let major_expansion_threshold_pct = if record.prj_feature_stories_net.unwrap_or(0.0) >= 1.0 {
            10.0
        } else {
            20.0
        };
        let building_expand_pct = approx_net_bldg_area / approx_old_bldg_area *100.0;
        let is_major_expansion = building_expand_pct >= major_expansion_threshold_pct;
        if far > neighbors_mean_far && approx_net_bldg_area > 360.0 && (is_major_expansion || is_demolition)  {
            info!("Prohibited: {} address: {}, far {:.02}, neighbor far: {:.02} ({} neighbors), bldg growth {:.0}sqft ({:.0}%)",
                  &record.date_opened[0..10], record.address, far, neighbors_mean_far, neighbors.len(), approx_net_bldg_area, building_expand_pct);
            num_prohibited_expansions += 1;
        } else {
            info!("Probably OK: {} address: {}, far {:.02}, neighbor far: {:.02} ({} neighbors), bldg growth {:.0}sqft ({:.0}%)",
                  &record.date_opened[0..10], record.address, far, neighbors_mean_far, neighbors.len(), approx_net_bldg_area, building_expand_pct);
            num_ok_expansions += 1;
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
    Ok(())
}

fn main() -> Result<(), Box<Error>> {
    env_logger::init();
    let matches = App::new("peskinexpansionsimpact")
        .version("0.0")
        .about("Show numbers of residential expansions that are probably prohibited by proposal")
        .author("Yonathan.")
        .after_help("Print projects most likely affected by Peskin ordinance (Board File 181216). Note: everything is output using the logger, so you should set RUST_LOG=peskinexpansionsimpact=info to see the output.")
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
        .subcommand(SubCommand::with_name("expansions")
            .about("Show information about expansions")
        )
        .setting(AppSettings::SubcommandRequired)
        .get_matches();

    let planning = matches
        .value_of_os("planning")
        .expect("Expected planning file");
    let land_use_path = matches
        .value_of_os("land-use")
        .expect("Expected land-use file");
    info!(
        "Opening {} and {}",
        planning.to_string_lossy(),
        land_use_path.to_string_lossy()
    );
    let planning_file = File::open(planning)?;
    let planning_rdr = csv::Reader::from_reader(planning_file);
    let land_use_file = File::open(land_use_path)?;
    let land_use_rdr = csv::Reader::from_reader(land_use_file);

    if let Some(_matches) = matches.subcommand_matches("expansions") {
        expansions(planning_rdr, land_use_rdr)?;
        Ok(())
    } else {
        panic!("Should not happen");
    }
}
