use geo::MultiPolygon;
use rstar::{AABB, PointDistance, RTree, RTreeObject};
use geo::{Closest, Geometry, Point};
use geo::algorithm::bounding_rect::BoundingRect;
use geo::algorithm::contains::Contains;
use geo::algorithm::closest_point::ClosestPoint;
use geo::algorithm::euclidean_distance::EuclideanDistance;
use geo::prelude::Area;
use std::error::Error;
use wkt::Wkt;

pub struct PolygonWrapper<T> {
    pub multi_polygon: MultiPolygon<f64>,
    pub bounding_box: AABB<[f64; 2]>,
    pub value: T,
}
impl<T> PolygonWrapper<T> {
    pub fn new(multi_polygon: MultiPolygon<f64>, value: T) -> Self {
        let bounding_rect =
            BoundingRect::bounding_rect(&multi_polygon).expect("bounding_rect failed");
        let bounding_box: AABB<[f64; 2]> = AABB::from_corners(
            [bounding_rect.min().x, bounding_rect.min().y],
            [bounding_rect.max().x, bounding_rect.max().y],
        );
        PolygonWrapper {
            multi_polygon,
            bounding_box,
            value,
        }
    }
}
impl<T> RTreeObject for PolygonWrapper<T> {
    type Envelope = AABB<[f64; 2]>;

    fn envelope(&self) -> Self::Envelope {
        self.bounding_box.clone()
    }
}
impl<T> PointDistance for PolygonWrapper<T> {
    fn distance_2(&self, point: &[f64; 2]) -> f64 {
        let point = Point::new(point[0], point[1]);
        if self.multi_polygon.contains(&point) {
            0.0
        } else {
            match self.multi_polygon.closest_point(&point) {
                Closest::Intersection(_) => 0.0,
                Closest::SinglePoint(p) => point.euclidean_distance(&p),
                Closest::Indeterminate => panic!("MultiPolygon should contain at least 1 point"),
            }
        }
    }
}

pub fn parse_wkt_to_multipolygon(the_geom: &str) -> Result<MultiPolygon<f64>, Box<dyn Error + Send + Sync + 'static>> {
    // TODO: take out the expect()s, and return Err instead
    let parsed_wkt: Wkt<f64> = Wkt::from_str(the_geom).expect("Could not parse WKT");
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
