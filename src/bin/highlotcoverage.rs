#[macro_use]
extern crate clap;
extern crate csv;
extern crate env_logger;
extern crate geo;
extern crate geo_types;
extern crate geojson;
#[macro_use]
extern crate log;
extern crate num_traits;
extern crate ordered_float;
// the old version used by proj, not the new ones re-exported by geo
extern crate parcelscan;
extern crate proj;
extern crate rayon;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate thiserror;
extern crate wkt;

use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Stdin, StdinLock, Write};
use std::sync::{Mutex, RwLock};

use chrono::Datelike;
use clap::App;
use clap::AppSettings;
use clap::Arg;
use clap::SubCommand;
use conv::ConvUtil;
use geo::{Coordinate, Line, MultiPolygon, Point};
use geo::algorithm::area::Area;
use geo::algorithm::centroid::Centroid;
use geo::algorithm::euclidean_distance::EuclideanDistance;
use geo::algorithm::map_coords::MapCoords;
use geo::algorithm::orient::{Direction, Orient};
use geo::algorithm::winding_order::Winding;
use geojson::{Feature, FeatureCollection, Geometry, PolygonType, Position};
use geojson::feature::Id;
use ordered_float::OrderedFloat;
use proj::Proj;
use rayon::prelude::*;
use rstar::{DefaultParams, RTree};
use serde_json::Map;
use wkt::types::Coord;

use parcelscan::geo_util::default_projection;
use parcelscan::polygon_wrapper::{parse_wkt_to_multipolygon, PolygonWrapper};
use parcelscan::sfbuidingfootprints::BuildingFootprintsRecord;
use parcelscan::sflanduse::LandUseRecord;
use parcelscan::sfplanningacela::PPTSRecord;
use parcelscan::sfzoningdistricts::{get_zoning, ZoningDistrict};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct SetbackAndAnnotation {
    side_type: SideType,
    setback: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct OutputRow {
    mblr: String,
    building_area: f64,
    side_setbacks: Option<Vec<SetbackAndAnnotation>>,
    yrbuilt: Option<usize>,
    addr: Option<String>,
    lot_blklot: Option<String>,
    lot_area: Option<f64>,
    zoning_district_name: Option<String>,
    geojson: FeatureCollection,
    height: f64,
    building_wkt: String,
    resunits: usize,
    lot_wkt: Option<String>,
}


#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
enum SideType {
    Front,
    Side,
    Rear,
}
struct SideWithType {
    edge: Line<f64>,
    side_type: SideType,
}

fn get_rear_side(
    rtree: &RTree<PolygonWrapper<LandUseRecord>, DefaultParams>,
    land_use_polygon: &PolygonWrapper<LandUseRecord>,
) -> Vec<SideWithType> {
    land_use_polygon.multi_polygon.0.iter().flat_map(|polygon| {
        let mut exterior = polygon.exterior().clone();
        exterior.make_ccw_winding();
        let all_edges: Vec<Line<f64>> = exterior.lines().collect();
        let mut edges_face_street: Vec<bool> = vec![];
        for i in 0..all_edges.len() {
            let line = all_edges[i];
            let start: Point<f64> = line.start.into();
            let end: Point<f64> = line.end.into();
            let next: Point<f64> = all_edges[(i + 1) % all_edges.len()].end.into();

           let (start_ft, end_ft, next_ft) = PROJ.with(|proj| {
                // the following calculations are in a feet projection
                let start_ft: Point<f64> = proj.project(start, false)
                    .expect("failed to project to ft");
                let end_ft: Point<f64> = proj.project(end, false)
                    .expect("failed to project to ft");
                let next_ft: Point<f64> = proj.project(next, false)
                    .expect("failed to project to ft");
               (start_ft, end_ft, next_ft)
            });
            let vector: Point<f64> = end_ft - start_ft;
            let next_vector: Point<f64> = next_ft - end_ft;
            let next_vector_projection = (vector.dot(next_vector) / (vector.0.x.hypot(vector.0.y)));
            let next_vector_parallel_part = Point::new(vector.0.x * next_vector_projection, vector.0.y * next_vector_projection);
            let next_vector_perpendicular_part = next_vector - next_vector_parallel_part;
            let next_vector_perpendicular_part_len = next_vector_perpendicular_part.x().hypot(next_vector_perpendicular_part.y());
            // TODO: just use polygon CCW winding instead
            let perpendicular_unit_vec = Point::new(
                next_vector_perpendicular_part.x()/next_vector_perpendicular_part_len,
                next_vector_perpendicular_part.y()/next_vector_perpendicular_part_len);
            // note: we subtract to go outside the polygon rather than inside
            let point_right_outside_ft = start_ft +
                Point::new(vector.x()/2.0, vector.y()/2.0) -
                Point::new(perpendicular_unit_vec.x()*10.0, perpendicular_unit_vec.y()*10.0);

            let point_right_outside = PROJ.with(|proj| {
                proj
                .project(point_right_outside_ft, true)
                .expect("failed to project from ft")
            });
            let edge_faces_street = rtree.locate_at_point(&[point_right_outside.x(), point_right_outside.y()]).is_none();
            edges_face_street.push(edge_faces_street);
        }
        let mut rear_edges = vec![];
        for i in 0..edges_face_street.len() {
            let edge = all_edges[i];
            let annotation = if edges_face_street[i] {
                SideType::Front
            } else if edges_face_street[(i + 1) % edges_face_street.len()] {
                SideType::Side
            } else if edges_face_street[(i - 1 + edges_face_street.len()) % edges_face_street.len()] {
                SideType::Side
            } else {
                SideType::Rear
            };
            rear_edges.push(SideWithType {edge, side_type: annotation });
        }
        rear_edges
    }).collect()
}
fn multipolygon_to_geojson(multi_polygon: &MultiPolygon<f64>) -> Geometry {
    Geometry::new(geojson::Value::MultiPolygon(
        multi_polygon.0.iter().map(|polygon| {
            // only do the exterior ring, so the polygon_type has only one element
            let polygon_type: PolygonType = vec![
                polygon.exterior().0.iter().map(|coord| {
                    let pos: Position = vec![coord.x, coord.y];
                    pos
                }).collect()
            ];
            polygon_type
        })
            .collect()
    ))
}

#[derive(Error, Debug)]
pub enum LotCoverageError {
    #[error("csv parse error {0} (expected < {})", i32::max_value())]
    CsvParse(csv::Error),
    #[error("io error writing file")]
    Io {#[from] source: std::io::Error },
}

thread_local! {static PROJ: Proj = default_projection();}
fn lot_coverage(
    mut land_use_rdr: csv::Reader<File>,
    mut zoning_districts_rdr: csv::Reader<File>,
    mut footprints_rdr: csv::Reader<File>,
    min_coverage: f64,
    output_write: Option<File>,
) -> Result<(), LotCoverageError> {
    let proj = default_projection();

    info!("Scanning LandUse table of all parcels");
    let mut parcels_vec: Vec<PolygonWrapper<LandUseRecord>> = vec![];
    for result in land_use_rdr.deserialize::<LandUseRecord>() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result.map_err(LotCoverageError::CsvParse)?;
        let multi_polygon = parse_wkt_to_multipolygon(&record.the_geom)
            .expect("TODO: should never happen; this actually panics instead of ever returning error");
        parcels_vec.push(PolygonWrapper::new(multi_polygon, record));
    }
    info!("Generating LandUse rtree of all parcels");
    let rtree = RTree::bulk_load(parcels_vec);

    info!("Scanning Zoning_Districts to make lookup table of zoning");
    let mut zoning_districts_vec: Vec<PolygonWrapper<ZoningDistrict>> = vec![];
    for result in zoning_districts_rdr.deserialize::<ZoningDistrict>() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result.map_err(LotCoverageError::CsvParse)?;
        let multi_polygon = parse_wkt_to_multipolygon(&record.the_geom)
            .expect("TODO: should never happen; this actually panics instead of ever returning error");
        zoning_districts_vec.push(PolygonWrapper::new(multi_polygon, record));
    }
    info!("Generating LandUse rtree of all parcels");
    let zoning_districts_rtree = RTree::bulk_load(zoning_districts_vec);

    info!("Scanning footprints");
    let output_write: Option<Mutex<File>> = output_write.map(Mutex::new);
    footprints_rdr.deserialize::<BuildingFootprintsRecord>().par_bridge().map(|result| -> Result<(), LotCoverageError> {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result.map_err(LotCoverageError::CsvParse)?;
        let shape: MultiPolygon<f64> = parse_wkt_to_multipolygon(&record.shape)
            .expect("TODO: should never happen; this actually panics instead of ever returning error");
        let shape_ft: MultiPolygon<f64> = shape.map_coords(|&(lat, lon)| {
            // convert to the old version of geo_types used by proj, and then convert back
            // to the new re-exported one used by geo
            let point = PROJ.with(|proj| {
                proj
                    .project(geo_types::Point::new(lat, lon), false)
                    .unwrap_or_else(|err| {
                        panic!(format!("Projection failed on ({} {}): {}", lat, lon, err));
                    })
            });
            (point.x(), point.y())
        });
        let zoning_district_name = get_zoning(&zoning_districts_rtree, &shape)
            .map(|zoning_district| zoning_district.zoning.to_owned());
        // Centroid in latitude and longitude seems to be about a hundred feet incorrect
        // so we find the centroid in the ft projection
        let centroid: Point<f64> = PROJ.with(|proj| proj.project(
            shape_ft.centroid()
            .expect("multipolygon should have at least one point"), true
        ))
            .expect("failed to reverse project centroid");
        let land_use_polygon = rtree.locate_at_point(&[centroid.x(), centroid.y()]);
        let land_use_record = land_use_polygon
            .map(|polygon_wrapper| &polygon_wrapper.value);
        let sides_and_annotations = land_use_polygon
            .map(|land_use_polygon| get_rear_side(&rtree, land_use_polygon));
        let side_setbacks: Option<Vec<SetbackAndAnnotation>> = sides_and_annotations.map(|sides_and_annotations| {
            sides_and_annotations.iter()
                .flat_map(|side_and_annotation| {
                    let line_ft = PROJ.with(|proj| Line::new::<Point<f64>>(
                        proj.project(side_and_annotation.edge.start, false).expect("Failed to project"),
                        proj.project(side_and_annotation.edge.end, false).expect("Failed to project"),
                    ));
                    let setback = shape_ft.0.iter()
                        .flat_map(|polygon_ft|
                            polygon_ft
                                .exterior().points_iter()
                                .map(|building_pt_ft|
                                    line_ft.euclidean_distance(&building_pt_ft)
                                )
                                .map(Into::<OrderedFloat<f64>>::into)
                                .min()
                        )
                        .min();
                    setback.map(|setback| {
                        SetbackAndAnnotation {
                            side_type: side_and_annotation.side_type,
                            setback: setback.into_inner()
                        }
                    })
                })
                .collect::<Vec<SetbackAndAnnotation>>()
        });
        let lot_blklot = land_use_record
            .map(|land_use_record| land_use_record.blklot.to_owned());
        let lot_area = land_use_record
            .map(|land_use_record| land_use_record.shape_area);
        let yrbuilt = land_use_record
            .map(|land_use_record| land_use_record.yrbuilt);
        let addr = land_use_record
            .map(|land_use_record| format!("{}-{} {} {}", land_use_record.from_st.map(|x| format!("{}", x)).unwrap_or("?".to_owned()), land_use_record.to_st.map(|x| format!("{}", x)).unwrap_or("?".to_owned()), land_use_record.street, land_use_record.st_type));
        let building_area = shape_ft.area().abs(); // note: area() is signed area
        let resunits = land_use_record
            .map(|land_use_record| land_use_record.resunits).unwrap_or(0);


        let footprint_feature: Feature = Feature {
            bbox: None,
            geometry: Some(multipolygon_to_geojson(&shape)),
            id: None,
            properties: Some({
                let mut map = Map::new();
                map.insert("name".to_string(), serde_json::Value::String("footprint".to_string()));
                map
            }),
            foreign_members: None
        };
        let footprint_centroid_feature: Feature = Feature {
            bbox: None,
            geometry: Some(Geometry::new(geojson::Value::Point(vec![
                centroid.x(), centroid.y()
            ]))),
            id: None,
            properties: Some({
                let mut map = Map::new();
                map.insert("name".to_string(), serde_json::Value::String("footprint centroid".to_string()));
                map
            }),
            foreign_members: None
        };
        let lot_polygon = land_use_record
            .map(|land_use_record|
                parse_wkt_to_multipolygon(&land_use_record.the_geom))
            .map_or(Ok(None), |result: Result<MultiPolygon<f64>, Box<dyn Error + Send + Sync + 'static>>| result.map(Some))
            .expect("TODO: should never happen; this actually panics instead of ever returning error");
        let lot_feature: Option<Feature> = lot_polygon.as_ref().map(|lot_polygon| {
            Feature {
                bbox: None,
                geometry: Some(multipolygon_to_geojson(lot_polygon)),
                id: None,
                properties: Some({
                    let mut map = Map::new();
                    map.insert("name".to_string(), serde_json::Value::String("lot".to_string()));
                    map
                }),
                foreign_members: None
            }
        });
        let lot_centroid: Option<Feature> = lot_polygon.map(|lot_polygon| {
            let centroid = lot_polygon.centroid().expect("lot has no centroid");
            Feature {
                bbox: None,
                geometry: Some(Geometry::new(geojson::Value::Point(vec![
                    centroid.x(), centroid.y()
                ]))),
                id: None,
                properties: Some({
                    let mut map = Map::new();
                    map.insert("name".to_string(), serde_json::Value::String("lot centroid".to_string()));
                    map
                }),
                foreign_members: None
            }
        });
        let features: Vec<Feature> =  vec![
            Some(footprint_feature),
            Some(footprint_centroid_feature),
            lot_feature,
            lot_centroid,
        ].into_iter().flatten().collect();
        let geojson = FeatureCollection {
            bbox: None,
            features,
            foreign_members: None,
        };
        if ! side_setbacks.as_ref().map(|s| s.iter()
            .any(|side| side.side_type == SideType::Rear && side.setback <= 15.0)
        ).unwrap_or(false) {
            // setbacks are not registered closely enough; ignore
            // return Ok(()); // continue
        }
        if lot_area.map(|lot_area| building_area / lot_area < min_coverage).unwrap_or(true) {
            return Ok(()); // continue
        }
        if lot_area.map(|lot_area| lot_area < 2490.0).unwrap_or(true) {
            return Ok(()); // continue
        }
        if zoning_district_name.as_ref().map(|x| &**x != "RH-2").unwrap_or(true) {
            return Ok(()); // continue
        }
        if resunits <= 0 {
            return Ok(()); // continue
        }
        let o = OutputRow {
            mblr: record.sf_mblr,
            building_area,
            side_setbacks,
            yrbuilt,
            addr,
            lot_blklot,
            lot_area,
            zoning_district_name,
            geojson,
            height: record.hgt_maxcm/2.54/12.0,
            building_wkt: record.shape,
            resunits,
            lot_wkt: land_use_record.map(|land_use_record| land_use_record.the_geom.clone()),
        /*
            record_type_category: record.record_type_category,
            description: record.description,
            address: record.address,
            office_proposed,
            office_net,
            date_opened: record.date_opened.date().format("%Y-%m-%d").to_string(),
            date_closed: record.date_closed.map(|d| d.date().format("%Y-%m-%d").to_string()),
            record_status: record.record_status,
            related_building_permit: record.related_building_permit,
         */
        };
        print_row(&o);
        if let Some(output_write) = output_write.as_ref() {
            let mut output_write = output_write.lock()
                .expect("Failed to acquire lock on output file");
            write!(output_write, "{}\n", serde_json::to_string(&o).unwrap())
                .map_err(LotCoverageError::from)?;
        }
        Ok(())
    }).collect::<Result<(), LotCoverageError>>()?;
    Ok(())
}
fn print_row(o: &OutputRow) {

    println!(
    "{}",
        serde_json::to_string(&o).unwrap()
/*
        "mblr: {mblr}, building_name: {building_name:?} zoning_district: {zoning_district_name:?}, building_area: {building_area}, lot_blklot: {lot_blklot:?}, lot_area: {lot_area:?}",
        mblr = o.mblr,
        building_name = o.building_name,
        building_area = o.building_area,
        lot_blklot = o.lot_blklot,
        lot_area = o.lot_area,
        zoning_district_name = o.zoning_district_name,
*/
        // "* {record_type_category} opened: {date_opened}, closed: {date_closed}, address: {address}, office: {office_proposed}, office_net: {office_net}, status: {record_status}, building: {related_building_permit}, description: {description}",
        // record_type_category = o.record_type_category,
        // date_opened = o.date_opened,
        // date_closed = o.date_closed.as_ref().map(|x| x.as_str()).unwrap_or("none"),
        // address = o.address,
        // office_proposed = o.office_proposed,
        // office_net = o.office_net,
        // record_status = o.record_status,
        // related_building_permit = o.related_building_permit,
        // description = o.description,
    );
}

const MAIN_COMMAND: &str = "coverage";
fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let matches = App::new("lotcoverage")
        .version("0.0")
        .about("List of buildings that exceed max lot coverage")
        .author("Yonathan.")
        .after_help("Print buildings that exceed 75% of lot coverage")
        .subcommand(SubCommand::with_name(MAIN_COMMAND)
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
            .arg(Arg::with_name("footprints")
                .long("footprints")
                .help("Building_Footprints.csv file https://data.sfgov.org/Geographic-Locations-and-Boundaries/Building-Footprints/ynuv-fyni")
                .required(true)
                .takes_value(true)
            )
            .arg(Arg::with_name("min-coverage")
                .long("min-coverage")
                .help("Minimum lot coverage (0-1)")
                .takes_value(true)
                .required(false)
                .default_value("0.85")
            )
            .arg(Arg::with_name("out")
                .long("out")
                .help("jsonl file output")
                .takes_value(true)
            )
            .about("Show information about expansions")
        )
        .subcommand(SubCommand::with_name("geojson")
            .arg(Arg::with_name("file")
                .long("file")
                .help("jsonl file input, which was output by coverage")
                .required(true)
                .takes_value(true)
            )
        )
        .setting(AppSettings::SubcommandRequired)
        .get_matches();


    if let Some(matches) = matches.subcommand_matches(MAIN_COMMAND) {
        let zoning_districts = matches
            .value_of_os("zoning-districts")
            .expect("Expected zoning-districts file");
        let land_use_path = matches
            .value_of_os("land-use")
            .expect("Expected land-use file");
        let footprints = matches
            .value_of_os("footprints")
            .expect("Expected footprints file");
        let min_coverage = value_t!(matches.value_of("min-coverage"), f64)
            .expect("Expected value for min-coverage");

        let out_projects_path = matches.value_of_os("out");
        let land_use_file = File::open(land_use_path)?;
        let land_use_rdr = csv::Reader::from_reader(land_use_file);
        let zoning_districts_file = File::open(zoning_districts)?;
        let zoning_districts_rdr = csv::Reader::from_reader(zoning_districts_file);
        let footprints_file = File::open(footprints)?;
        let footprints_rdr = csv::Reader::from_reader(footprints_file);
        let out_projects_writer_opt = out_projects_path
            .map(
                |path| -> Result<Option<File>, std::io::Error> {
                    info!("Opening output file {}", path.to_string_lossy());
                    let writer: File = File::create(path)?;
                    Ok(Some(writer))
                },
            )
            .unwrap_or(Ok(None))?;
        lot_coverage(land_use_rdr, zoning_districts_rdr, footprints_rdr, min_coverage, out_projects_writer_opt)?;
    } else if let Some(matches) = matches.subcommand_matches("geojson") {
        let projects_path = matches.value_of_os("file").expect("required arg should exist");
        let mut projects_file = BufReader::new(File::open(projects_path)?);
        let mut lines: Vec<OutputRow> = vec![];

        for line in projects_file.lines().into_iter() {
            lines.push(serde_json::from_str(&line?)?);
        }

        let features = lines.into_iter().map(|o| {
            let building_shape: MultiPolygon<f64> = parse_wkt_to_multipolygon(&*o.building_wkt)
                .expect("TODO: should never happen; this actually panics instead of ever returning error");
            let lot_polygon: Option<MultiPolygon<f64>> = o.lot_wkt.as_ref()
                .map(|wkt| Ok(parse_wkt_to_multipolygon(&**wkt)?))
                .map_or(Ok(None), |x: Result<MultiPolygon<f64>, Box<dyn Error + Send + Sync + 'static>>| x.map(Some))
                .expect("TODO: should never happen; this actually panics instead of ever returning error");
            let properties = {
                let val = serde_json::to_value(o).unwrap();
                let mut map = match val {
                    serde_json::Value::Object(map) => map,
                    _ => panic!("object should have turned into json object"),
                };
                map.remove("building_wkt");
                map.remove("lot_wkt");
                map.remove("geojson");
                map
            };
            let lot_feature: Option<Feature> = lot_polygon.as_ref().map(|lot_polygon| {
                Feature {
                    bbox: None,
                    geometry: Some(multipolygon_to_geojson(lot_polygon)),
                    id: None,
                    properties: Some({
                        let mut map = Map::new();
                        map.insert("name".to_string(), serde_json::Value::String("lot".to_string()));
                        map
                    }),
                    foreign_members: None
                }
            });
            let footprint_feature: Feature = Feature {
                bbox: None,
                geometry: Some(Geometry::new(geojson::Value::GeometryCollection(vec![
                    lot_polygon.as_ref().map(multipolygon_to_geojson),
                    Some(multipolygon_to_geojson(&building_shape)),
                ].into_iter().flatten().collect()))),
                id: None,
                properties: Some(properties),
                foreign_members: None
            };
            Ok(footprint_feature)
        })
            .collect::<Result<Vec<_>, Box<dyn Error>>>()?;
        let geojson = FeatureCollection {
            bbox: None,
            features,
            foreign_members: None,
        };
        println!("{}", serde_json::to_string(&geojson).unwrap());
    } else {
        panic!("Should not happen");
    }
    Ok(())
}
