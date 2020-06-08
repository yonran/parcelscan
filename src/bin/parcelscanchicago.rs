extern crate clap;
extern crate csv;
extern crate env_logger;
extern crate geo;
extern crate geo_types;
#[macro_use]
extern crate log;
extern crate proj;
extern crate rstar;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate num_traits;
extern crate wkt;

use clap::AppSettings;
use clap::SubCommand;
use clap::{App, Arg};
use csv::Reader;
use geo::algorithm::area::Area;
use geo::algorithm::bounding_rect::BoundingRect;
use geo::algorithm::intersects::Intersects;
use geo::algorithm::map_coords::MapCoords;
use geo::Geometry;
use geo::{MultiPolygon, Polygon};
use proj::Proj;
use rstar::{RTree, RTreeObject, AABB};
use std::collections::BTreeMap;
use std::error::Error;
use std::fs::File;
use wkt::{ToWkt, Wkt};

#[derive(Debug, Deserialize)]
struct ParcelRecord {
    #[serde(rename = "OBJECTID")]
    objectid: String,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "the_geom")]
    the_geom: String,
    #[serde(rename = "PIN10")]
    pin10: String,
    #[serde(rename = "PINA")]
    pina: String,
    #[serde(rename = "PINSA")]
    pinsa: String,
    #[serde(rename = "PINB")]
    pinb: String,
    #[serde(rename = "PINP")]
    pinp: String,
    #[serde(rename = "PINU")]
    pinu: String,
    #[serde(rename = "PINAC")]
    pinac: String,
    #[serde(rename = "TAXCODE")]
    taxcode: String,
    #[serde(rename = "JOB_NO")]
    job_no: String,
    #[serde(rename = "UPPER_ELEV")]
    upper_elev: f64,
    #[serde(rename = "LOWER_ELEV")]
    lower_elev: f64,
    #[serde(rename = "SURVEY_CAL")]
    survey_cal: String,
    #[serde(rename = "PARCELTYPE")]
    parceltype: String,
    #[serde(rename = "SHAPE_STAr")]
    shape_star: String,
    #[serde(rename = "SHAPE_STLe")]
    shape_stle: String,
}

#[derive(Debug, Deserialize)]
struct BuildingRecord {
    the_geom: String,
    /// Internal Use Only
    #[serde(rename = "BLDG_ID")]
    bldg_id: String,
    /// Internal Use Only
    #[serde(rename = "CDB_CITY_I")]
    cdb_city_id: String,
    /// ACTIVE, PROPOSED, DEMOLISHED
    #[serde(rename = "BLDG_STATU")]
    bldg_status: String,
    /// Low house number
    #[serde(rename = "F_ADD1")]
    f_add1: Option<usize>,
    /// High house number
    #[serde(rename = "T_ADD1")]
    t_add1: Option<usize>,
    /// Address Street Direction
    #[serde(rename = "PRE_DIR1")]
    pre_dir1: String,
    /// Address Street Name
    #[serde(rename = "ST_NAME1")]
    st_name1: String,
    /// Address Street Type (see valid street types in Street Center Line section)
    #[serde(rename = "ST_TYPE1")]
    st_type1: String,
    /// House number ‘Unit’ such as ‘REAR’, A, B, C, etc
    #[serde(rename = "UNIT_NAME")]
    unit_name: String,
    /// Used for structures not usually considered ‘buildings’. RESIDENTIAL GARAGE, MONUMENT, CTA, PLATFORM, OTHER
    #[serde(rename = "NON_STANDA")]
    non_standard: String,
    /// Building name
    #[serde(rename = "BLDG_NAME1")]
    bldg_name1: String,
    /// Alternate building name
    #[serde(rename = "BLDG_NAME2")]
    bldg_name2: String,
    /// Comments
    #[serde(rename = "COMMENTS")]
    comments: String,
    /// Number of stories
    #[serde(rename = "STORIES")]
    stories: usize,
    /// Internal use only
    #[serde(rename = "ORIG_BLDG_")]
    orig_bldg_id: String,
    /// Internal Use Only
    #[serde(rename = "FOOTPRINT_")]
    footprint_source: String,
    /// Internal Use Only
    #[serde(rename = "CREATE_USE")]
    create_userid: String,
    /// Date footprint created
    #[serde(rename = "BLDG_CREAT")]
    bldg_create_date: String,
    /// Date footprint given ACTIVE status
    #[serde(rename = "BLDG_ACTIV")]
    bldg_active_date: String,
    /// Date footprint given DEMOLISHED status (Demolished buildings are removed from the BUILDINGS layer and moved to a ‘DEMOLISHED’ layer.
    #[serde(rename = "BLDG_END_D")]
    bldg_end_date: String,
    /// N/A
    #[serde(rename = "DEMOLISHED")]
    demolished_date: String,
    /// Internal Use Only
    #[serde(rename = "EDIT_DATE")]
    edit_date: String,
    /// Internal Use Only
    #[serde(rename = "EDIT_USERI")]
    edit_userid: String,
    /// Internal Use Only
    #[serde(rename = "EDIT_SOURC")]
    edit_source: String,
    /// Internal Use Only
    #[serde(rename = "QC_DATE")]
    qc_date: String,
    /// Internal Use Only
    #[serde(rename = "QC_USERID")]
    qc_userid: String,
    /// Internal Use Only
    #[serde(rename = "QC_SOURCE")]
    qc_source: String,
    /// State Plane X Coordinate of Footprint label point
    #[serde(rename = "X_COORD")]
    x_coord: f64,
    /// State Plane Y Coordinate of Footprint label point
    #[serde(rename = "Y_COORD")]
    y_coord: f64,
    /// Not maintained
    #[serde(rename = "Z_COORD")]
    z_coord: f64,
    /// Internal Use Only
    #[serde(rename = "HARRIS_STR")]
    harris_strucid: String,
    /// Number of residential units.
    #[serde(rename = "NO_OF_UNIT")]
    no_of_units: usize,
    /// Number of stories below ground.
    #[serde(rename = "NO_STORIES")]
    no_stories_below: usize,
    /// Year built
    #[serde(rename = "YEAR_BUILT")]
    year_built: usize,
    /// Not actively maintained
    #[serde(rename = "BLDG_SQ_FO")]
    bldg_sq_footage: f64,
    /// Not actively maintained
    #[serde(rename = "BLDG_CONDI")]
    bldg_condition: String,
    /// Not actively maintained
    #[serde(rename = "CONDITION_")]
    condition_as_of_date: String,
    /// Not actively maintained
    #[serde(rename = "VACANCY_ST")]
    vacancy_status: String,
    /// House number displayed on the actual house. (Defaults to F_ADD1)
    #[serde(rename = "LABEL_HOUS")]
    label_house_no: String,
    /// Address street name Suffix direction (goes with PRE_DIR1, ST_NAME1 and ST_TYPE1)
    #[serde(rename = "SUF_DIR1")]
    suf_dir1: String,
}

struct ParcelWrapper {
    multi_polygon: MultiPolygon<f64>,
    bounding_box: AABB<[f64; 2]>,
}
impl ParcelWrapper {
    fn new(multi_polygon: MultiPolygon<f64>) -> Self {
        let bounding_rect =
            BoundingRect::bounding_rect(&multi_polygon).expect("bounding_rect failed");
        let bounding_box: AABB<[f64; 2]> = AABB::from_corners(
            [bounding_rect.min().x, bounding_rect.min().y],
            [bounding_rect.max().x, bounding_rect.max().y],
        );
        ParcelWrapper {
            multi_polygon,
            bounding_box,
        }
    }
}
impl RTreeObject for ParcelWrapper {
    type Envelope = AABB<[f64; 2]>;

    fn envelope(&self) -> Self::Envelope {
        self.bounding_box.clone()
    }
}
struct JoinedEntry {
    building_record: BuildingRecord,
    parcel_multi_polygon: MultiPolygon<f64>,
    parcel_area: f64,
}

fn get_area(proj: &Proj, multi_polygon: &MultiPolygon<f64>) -> f64 {
    // https://proj4.org/operations/projections/aea.html
    let signed_area = multi_polygon
        .map_coords(|&(lat, lon)| {
            // Note: we are using geo_types here, which matches the version from proj.
            // This is different than the version of geo_types exported by wkt and geo
            use geo_types::Point;
            let coords_rad = Point::new(lat, lon).to_radians();
            let point = proj.project(coords_rad, false).unwrap_or_else(|err| {
                panic!(format!("Projection failed on ({} {}): {}", lat, lon, err));
            });
            (point.x(), point.y())
        })
        .area();
    signed_area.abs()
}

fn density_historgram(
    mut buildings_rdr: Reader<File>,
    mut parcels_rdr: Reader<File>,
) -> Result<(), Box<Error>> {
    // projection for converting latitude and longitude into sq. ft.
    // https://proj4.org/operations/projections/gn_sinu.html
    let proj = Proj::new(
        "
    +proj=pipeline
    +step +proj=gn_sinu +lon_0=87.6d +m=2 +n=3
    +step +proj=unitconvert +xy_in=m +xy_out=us-ft
    ",
    )
    //    +step +proj=aea +lat_1=29.5 +lat_2=42.5
    .expect("Failed to create projection");
    info!("Created projection {}", proj.def());
    let parcel_shapes = parcels_rdr
        .deserialize::<ParcelRecord>()
        .map(|result| {
            result.map(|record| {
                let parsed_wkt = Wkt::from_str(&record.the_geom).expect("Could not parse WKT");
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
                            Area::<f64>::area(&multi_polygon).round()
                        );
                        multi_polygon
                    }
                    _ => {
                        panic!("Expected only multipolygons in parcel file");
                    }
                };
                ParcelWrapper::new(multi_polygon)
            })
        })
        .collect::<Result<Vec<ParcelWrapper>, _>>()?;
    info!("Bulk loading {} parcels", parcel_shapes.len());
    let rtree = RTree::bulk_load(parcel_shapes);
    let mut num_lots = 0;
    let mut num_res_lots = 0;
    let mut num_buildings_without_parcel = 0;
    let mut num_parcels_0_area = 0;
    let mut map: BTreeMap<i64, Vec<JoinedEntry>> = BTreeMap::new();
    for result in buildings_rdr.deserialize::<BuildingRecord>() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        //        info!("{:?}", record);
        if record.no_of_units > 0 {
            num_res_lots += 1;
            let parsed_wkt = Wkt::from_str(&record.the_geom).expect("Failed to parse geometry");
            if parsed_wkt.items.len() != 1 {
                panic!(
                    "Expected building wkt to have exactly 1 geometry; got {}",
                    parsed_wkt.items.len()
                );
            }
            let item = &parsed_wkt.items[0];
            let geometry = wkt::conversion::try_into_geometry(item)
                .expect("Failed to convert polygon wkt to geo_types");
            let area_and_parcel_opt = match geometry {
                Geometry::MultiPolygon(multi_polygon) => {
                    let building_wrapper = ParcelWrapper::new(multi_polygon.clone());
                    let parcel_opt = rtree
                        .locate_in_envelope_intersecting(&building_wrapper.bounding_box)
                        .filter(|parcel_wrapper| {
                            building_wrapper.multi_polygon.0.iter().all(
                                |building_polygon: &Polygon<f64>| {
                                    parcel_wrapper.multi_polygon.0.iter().any(
                                        |parcel_polygon: &Polygon<f64>| {
                                            Intersects::intersects(parcel_polygon, building_polygon)
                                        },
                                    )
                                },
                            )
                        })
                        .next();
                    if let Some(parcel) = parcel_opt {
                        let area = get_area(&proj, &parcel.multi_polygon);
                        Some((area, parcel.multi_polygon.clone()))
                    } else {
                        None
                    }
                }
                _ => {
                    panic!("Unexpected building shape");
                }
            };
            if let Some((area, parcel_multi_polygon)) = area_and_parcel_opt {
                if area > 0.0 {
                    map.entry((record.no_of_units as f64 / area * 2500. * 4.0).round() as i64)
                        .or_insert(vec![])
                        .push(JoinedEntry {
                            building_record: record,
                            parcel_area: area,
                            parcel_multi_polygon,
                        });
                } else {
                    num_parcels_0_area += 1;
                }
            } else {
                num_buildings_without_parcel += 1;
            }
        }
        num_lots += 1;
    }

    info!(
        "{} total lots, {} residential, {} houses with no parcel, {} houses with 0 parcel area",
        num_lots, num_res_lots, num_buildings_without_parcel, num_parcels_0_area
    );
    for (units_times4, records) in map.iter() {
        let units = *units_times4 as f64 / 4.0;
        let mut records_sorted_by_date = records.iter().collect::<Vec<_>>();
        records_sorted_by_date.sort_by_key(|joined_entry| joined_entry.building_record.year_built);
        records_sorted_by_date.reverse();
        let records_fmt = records_sorted_by_date
            .iter()
            .take(5)
            .map(
                |&JoinedEntry {
                     building_record: record,
                     parcel_area,
                     parcel_multi_polygon,
                 }| {
                    format!(
                        "{}-{} {} {} ({}, {}) {} units, parcel area {} {}",
                        record
                            .f_add1
                            .map(|x| format!("{}", x))
                            .unwrap_or("".to_string()),
                        record
                            .t_add1
                            .map(|x| format!("{}", x))
                            .unwrap_or("".to_string()),
                        record.st_name1,
                        record.st_type1,
                        record.bldg_id,
                        record.year_built,
                        record.no_of_units,
                        parcel_area,
                        ToWkt::to_wkt(&Geometry::MultiPolygon(parcel_multi_polygon.clone())).items
                            [0],
                    )
                },
            )
            .collect::<Vec<_>>()
            .join(", ");
        info!(
            "{} units: {} records e.g. {}",
            units,
            records.len(),
            records_fmt
        )
    }
    Ok(())
}

fn main() -> Result<(), Box<Error>> {
    env_logger::init();
    let matches = App::new("parcelscanchicago")
        .version("0.0")
        .about("Scan parcels and print details")
        .author("Yonathan.")
        .after_help("Show stats on chicago parcels csv file buildings.csv https://data.cityofchicago.org/Buildings/Building-Footprints-current-/hz9b-7nh8")
        .arg(Arg::with_name("buildings")
            .long("buildings")
            .required(true)
            .takes_value(true)
        )
        .arg(Arg::with_name("parcels")
            .long("parcels")
            .help("Cook County Parcels file e.g. ccgisdata_-_Parcels_2016.csv from https://datacatalog.cookcountyil.gov/GIS-Maps/ccgisdata-Parcels-2016/a33b-b59u")
            .required(true)
            .takes_value(true)
        )
        .subcommand(SubCommand::with_name("density-historgram")
            .about("Show statistics about all residences")
        )
        .setting(AppSettings::SubcommandRequired)
        .get_matches();

    let buildings = matches
        .value_of_os("buildings")
        .expect("Expected buildings");
    let parcels = matches.value_of_os("parcels").expect("Expected parcels");
    // TODO: add parsing for parcels
    // https://datacatalog.cookcountyil.gov/GIS-Maps/ccgisdata-Parcels-2016/a33b-b59u
    info!(
        "Opening {} and {}",
        buildings.to_string_lossy(),
        parcels.to_string_lossy()
    );
    let buildings_file = File::open(buildings)?;
    let parcels_file = File::open(parcels)?;
    let rdr = csv::Reader::from_reader(buildings_file);
    let parcels_rdr = csv::Reader::from_reader(parcels_file);
    if let Some(_matches) = matches.subcommand_matches("density-historgram") {
        density_historgram(rdr, parcels_rdr)
    } else {
        panic!("Should not happen");
    }
}
