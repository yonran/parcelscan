
//! Parser for SF Planning Zoning Districts Map
//! https://data.sfgov.org/Geographic-Locations-and-Boundaries/Zoning-Map-Zoning-Districts/xvjh-uu28
//! File name: Zoning_Map_-_Zoning_Districts_data.csv
//!
//! See also Zoning Height Map
//!

use rstar::RTree;
use crate::polygon_wrapper::PolygonWrapper;
use geo::{MultiPolygon, Point};
use geo::algorithm::centroid::Centroid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ZoningDistrict {
    pub the_geom: String,

    #[serde(rename = "OBJECTID")]
    pub objectid: String,

    pub zoning_sim: String,

    pub districtname: String,

    pub url: String,

    pub gen: String,

    pub zoning: String,

    pub codesection: String,

    #[serde(rename = "shape_Length")]
    pub shape_length: f64,

    #[serde(rename = "shape_Area")]
    pub shape_area: f64,
}

pub fn get_zoning<'a>(
    rtree: &'a RTree<PolygonWrapper<ZoningDistrict>>,
    shape: &MultiPolygon<f64>,
) -> Option<&'a ZoningDistrict> {
    let centroid: Point<f64> = shape.centroid()
        .expect("multipolygon should have at least one point");
    rtree.locate_at_point(&[centroid.0.x, centroid.0.y])
        .map(|polygon_wrapper| &polygon_wrapper.value)
}

#[cfg(test)]
mod test {
    use super::ZoningDistrict;
    use csv::Reader;
    const TEST_LINES: &str = "the_geom,OBJECTID,zoning_sim,districtname,url,gen,zoning,codesection,shape_Length,shape_Area
\"MULTIPOLYGON (((-122.395895701 37.784896928, -122.395160622 37.784311012, -122.394956564 37.784147886, -122.394951788 37.784144067, -122.394925604 37.784123136, -122.394854047 37.784065932, -122.394820682 37.784039259, -122.394814269 37.784034133, -122.394757559 37.783988798, -122.394718913 37.783957903, -122.394625576 37.783883287, -122.394726078 37.783804206, -122.395119253 37.783494832, -122.395217547 37.783417488, -122.395400943 37.78327318, -122.395384745 37.78326028, -122.395397059 37.783250634, -122.396841427 37.782119121, -122.399079634 37.780362887, -122.399616232 37.780791482, -122.400149637 37.78121752, -122.400613071 37.781587665, -122.400668574 37.781631995, -122.40031732 37.781908576, -122.400268489 37.781869654, -122.399802203 37.781497998, -122.398879207 37.782224234, -122.398951115 37.782281091, -122.39888968 37.782329429, -122.398650276 37.782517795, -122.399072481 37.782855996, -122.398518244 37.783293881, -122.398415805 37.783374814, -122.397942938 37.782997897, -122.397385539 37.782553592, -122.396754161 37.783052317, -122.396489274 37.78326155, -122.396455659 37.783288101, -122.396262401 37.783440752, -122.396678728 37.783770182, -122.397035043 37.784052123, -122.396569303 37.784419099, -122.396536412 37.78439277, -122.396532045 37.784396206, -122.396464808 37.784449114, -122.396031078 37.784790405, -122.396028035 37.784792799, -122.395895701 37.784896928)))\",18890,CMUO,CENTRAL SOMA-MIXED USE OFFICE,http://library.amlegal.com/nxt/gateway.dll/California/planning/article8mixedusedistricts?f=templates$fn=default.htm$3.0$vid=amlegal:sanfrancisco_ca$anc=JD_848,Mixed Use,CMUO,TBD,0.018678318753143,0.000007881758246
";
    #[test]
    fn test_parse_record() -> Result<(), csv::Error> {
        let mut rdr = Reader::from_reader(TEST_LINES.as_bytes());
        for line in rdr.deserialize::<ZoningDistrict>() {
            line?;
        }
        Ok(())
    }
}
