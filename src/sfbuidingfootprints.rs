//! Parser for SF Building Footprints csv file
//! https://data.sfgov.org/Geographic-Locations-and-Boundaries/Building-Footprints/ynuv-fyni
//! File name: Building_Footprints.csv
//!

use serde;

mod comma_float {
    use serde::{self, Deserialize, Serializer, Deserializer};
    use conv::TryFrom;

    #[cfg(test)]
    mod test {
        use super::deserialize;
        use super::serde::de::IntoDeserializer;
        use super::serde::de::value::Error;
        #[test]
        fn test() -> Result<(), Error> {
            deserialize("99,734".into_deserializer())?;
            deserialize("942".into_deserializer())?;
            Ok(())
        }
    }

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(
        data: &f64,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let s = format!("{}", data); // add comma separator
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<f64, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let s = s.replace(",", "");
        use crate::serde::de::Error;
        let result = s.parse::<f64>().map_err(|e| D::Error::custom(e.to_string()));
        result
    }
}
mod opt_comma_float {
    use serde::{self, Deserialize, Serializer, Deserializer};
    use super::comma_float;

    #[cfg(test)]
    mod test {
        use super::deserialize;
        use super::serde::de::IntoDeserializer;
        use super::serde::de::value::Error;
        #[test]
        fn test() -> Result<(), Error> {
            deserialize("99,734".into_deserializer())?;
            deserialize("942".into_deserializer())?;
            deserialize("".into_deserializer())?;
            Ok(())
        }
    }

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(
        data: &Option<f64>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        if let &Some(data) = data {
            let s = format!("{}", data); // add comma separator
            serializer.serialize_str(&s)
        } else {
            serializer.serialize_str("")
        }
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<Option<f64>, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.is_empty() {
            Ok(None)
        } else {
            let s = s.replace(",", "");
            use crate::serde::de::Error;
            s.parse::<f64>().map_err(|e| D::Error::custom(e.to_string())).map(Some)
        }
    }
}



/// Fields Reference: see SF_BldgFoot_2017-05_description.pdf from
/// https://data.sfgov.org/Geographic-Locations-and-Boundaries/Building-Footprints/ynuv-fyni
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BuildingFootprintsRecord {

    // /// auto-generated unique ID key
    // #[serde(rename = "OBJECTID")]
    // pub object_id: String, // ObjectID

    // /// Polygon-Z type
    // #[serde(rename = "Shape")]
    // pub shape: String, // Geometry

    /// San Francisco Building ID using criteria of 2016-09, 6-char epoch, '.' , 7-char zero-padded AreaID or new ID in editing epochs after initial '201006.'
    #[serde(rename = "sf16_bldgid")]
    pub sf16_bldg_id: String, // char-14

    /// Epoch 2010.06 Shape_Area sort of 177,023 building polygons with area > ~1 sq m
    #[serde(rename = "area_id")]
    pub area_id: f64,

    /// San Francisco property key: Assessor's Map-Block-Lot of land parcel, plus Right-of-way area identifier derived from street Centerline Node Network (CNN)
    #[serde(rename = "mblr")]
    pub sf_mblr: String, // char-20

    /// Pictometry 2010 building name, if any
    #[serde(rename = "p2010_name")]
    pub p2010_name: Option<String>,

    /// Input building mass (of 2010,) minimum Z vertex elevation, NAVD 1988 ft
    /// (aka P2010mass_ZminN88ft in documentation)
    #[serde(rename = "p2010_zminn88ft")]
    pub p2010_zminn88ft: Option<f64>, // Number

    /// Input building mass (of 2010,) maximum Z vertex elevation, NAVD 1988 ft
    /// (aka P2010mass_ZmaxN88ft in documentation)
    #[serde(rename = "p2010_zmaxn88ft")]
    pub p2010_zmaxn88ft: String, // Number

    /// zonal statistic: LiDAR-derived ground surface grid, population of 50cm square cells sampled in this building's zone, integer NAVD 1988 centimeters
    /// (aka gnd_cells50cm in documentation)
    #[serde(rename = "gnd_cells50cm", with = "comma_float")]
    pub gnd_cells50cm: f64, // Long

// 507,704,197,595.64277139000001,45.538494300000004,191,643,698,600,\"178,250\",946,\"2,385\",\"1,439\",\"1,532.2886171099999\",225.94217835000001,\"1,204\",\"1,405\",946,\"1,416\",\"178,250\",349,\"1,730\",\"1,381\",936.64459466999995,210.80556354999999,\"1,206\",813,349,850,5.0700000000000003,14.16,8.5,9.0899999999999999,23.850000000000001,{CF7EF595-68E6-4950-B361-CC82D77383A0},\"MULTIPOLYGON (((-122.37950387699998 37.73979945100002, -122.37951078200003 37.739791380999975, -122.37951088699998 37.739791260000004, -122.37942182299997 37.739742509999985, -122.37942180599998 37.73974246400002, -122.38034652099998 37.73871573500003, -122.38043307999999 37.73876573000001, -122.38044047900001 37.73875702700003, -122.38047091 37.738775042999976, -122.38048248100002 37.73876211999999, -122.38274123999997 37.74004335000001, -122.38274126300001 37.740043420999996, -122.38272735599999 37.740058594000004, -122.382815307 37.74010609700002, -122.38281534200001 37.740106208999975, -122.38255868700003 37.740391794999994, -122.38254232799999 37.74038284699998, -122.38253457500002 37.74039098200001, -122.38249726300002 37.74043014, -122.38251599 37.740440433, -122.38251602600002 37.74044054400002, -122.382075531 37.74092975000002, -122.38205777299999 37.74091976599999, -122.38167157499997 37.74134788800001, -122.381388389 37.74118806400003, -122.38133037 37.74119490300001, -122.38128900700002 37.741171742999995, -122.38128896 37.74117160700001, -122.38134406299999 37.74110932299999, -122.379374429 37.739994122999974, -122.37945644299998 37.73990475400001, -122.37949122200001 37.73986609299999, -122.37946334200002 37.73985024699999, -122.37946330400003 37.739850144, -122.37950387699998 37.73979945100002)))\"";

    /// zonal statistic: LiDAR-derived ground surface grid, minimum value of 50cm square cells sampled in this building's zone, integer NAVD 1988 centimeters
    /// (aka gnd_MINcm in documentation)
    #[serde(rename = "gnd_mincm", with = "comma_float")]
    pub gnd_mincm: f64, // Long

    /// zonal statistic: LiDAR-derived ground surface grid, maximum value of 50cm square cells sampled in this building's zone, integer NAVD 1988 centimeters
    /// (aka gnd_MAXcm in documentation)
    #[serde(rename = "gnd_maxcm", with = "comma_float")]
    pub gnd_maxcm: f64, // Long

    /// zonal statistic: LiDAR-derived ground surface grid, maximum value of 50cm square cells sampled in this building's zone, integer NAVD 1988 centimeters
    /// (aka gnd_RANGEcm in documentation)
    #[serde(rename = "gnd_rangecm", with = "comma_float")]
    pub gnd_rangecm: f64,

    /// zonal statistic: LiDAR-derived ground surface grid, mean value of 50cm square cells sampled in this building's zone, from integer NAVD 1988 centimeters
    /// (aka gnd_MEANcm in documentation)
    #[serde(rename = "gnd_meancm", with = "comma_float")]
    pub gnd_meancm: f64, // Number

    /// zonal statistic: LiDAR-derived ground surface grid, 1 standard deviation of 50cm square cells sampled in this building's zone, centimeters
    /// (aka gnd_STDcm in documentation)
    #[serde(rename = "gnd_stdcm", with = "comma_float")]
    pub gnd_stdcm: f64, // Number

    /// zonal statistic: LiDAR-derived ground surface grid, count of unique values of 50cm square cells sampled in this building's zone, integer NAVD 1988 centimeters
    /// (aka gnd_VARIETYcm in documentation)
    #[serde(rename = "gnd_varietycm", with = "comma_float")]
    pub gnd_varietycm: f64,

    /// zonal statistic: LiDAR-derived ground surface grid, most frequently occuring value of 50cm square cells sampled in this building's zone, integer NAVD 1988 centimeters
    #[serde(rename = "gnd_majoritycm", with = "comma_float")]
    pub gnd_majoritycm: f64,

    /// zonal statistic: LiDAR-derived ground surface grid, least frequently occuring value of 50cm square cells sampled in this building's zone, integer NAVD 1988 centimeters
    #[serde(rename = "gnd_minoritycm", with = "comma_float")]
    pub gnd_minoritycm: f64,

    /// zonal statistic: LiDAR-derived ground surface grid, median value of 50cm square cells sampled in this building's zone, integer NAVD 1988 centimeters
    #[serde(rename = "gnd_mediancm", with = "comma_float")]
    pub gnd_mediancm: f64,

    /// zonal statistic: LiDAR-derived first return surface grid, population of 50cm square cells sampled in this building's zone, integer NAVD 1988 centimeters
    #[serde(rename = "cells50cm_1st", with = "comma_float")]
    pub cells50cm_1st: f64,

    /// zonal statistic: LiDAR-derived first return surface grid, minimum value of 50cm square cells sampled in this building's zone, integer NAVD 1988 centimeters
    #[serde(rename = "mincm_1st", with = "comma_float")]
    pub mincm_1st: f64,

    /// zonal statistic: LiDAR-derived first return surface grid, maximum value of 50cm square cells sampled in this building's zone, integer NAVD 1988 centimeters
    #[serde(rename = "maxcm_1st", with = "comma_float")]
    pub maxcm_1st: f64,

    /// zonal statistic: LiDAR-derived first return surface grid, maximum value of 50cm square cells sampled in this building's zone, integer NAVD 1988 centimeters
    #[serde(rename = "rangecm_1st", with = "comma_float")]
    pub rangecm_1st: f64,

    /// zonal statistic: LiDAR-derived first return surface grid, mean value of 50cm square cells sampled in this building's zone, from integer NAVD 1988 centimeters
    #[serde(rename = "meancm_1st", with = "comma_float")]
    pub meancm_1st: f64, // Number

    /// zonal statistic: LiDAR-derived first return surface grid, 1 standard deviation of 50cm square cells sampled in this building's zone, centimeters
    #[serde(rename = "stdcm_1st", with = "comma_float")]
    pub stdcm_1st: f64, // Number

    /// zonal statistic: LiDAR-derived first return surface grid, count of unique values of 50cm square cells sampled in this building's zone, integer NAVD 1988 centimeters
    #[serde(rename = "varietycm_1st", with = "comma_float")]
    pub varietycm_1st: f64,

    /// zonal statistic: LiDAR-derived first return surface grid, most frequently occuring value of 50cm square cells sampled in this building's zone, integer NAVD 1988 centimeters
    #[serde(rename = "majoritycm_1st", with = "comma_float")]
    pub majoritycm_1st: f64,

    /// zonal statistic: LiDAR-derived first return surface grid, least frequently occuring value of 50cm square cells sampled in this building's zone, integer NAVD 1988 centimeters
    #[serde(rename = "minoritycm_1st", with = "comma_float")]
    pub minoritycm_1st: f64,

    /// zonal statistic: LiDAR-derived first return surface grid, median value of 50cm square cells sampled in this building's zone, integer NAVD 1988 centimeters
    #[serde(rename = "mediancm_1st", with = "comma_float")]
    pub mediancm_1st: f64,

    /// zonal statistic: LiDAR-derived height surface grid, population of 50cm square cells sampled in this building's zone, integer centimeters
    #[serde(rename = "hgt_cells50cm", with = "comma_float")]
    pub hgt_cells50cm: f64,

    /// zonal statistic: LiDAR-derived height surface grid, minimum value of 50cm square cells sampled in this building's zone, integer centimeters
    #[serde(rename = "hgt_mincm", with = "comma_float")]
    pub hgt_mincm: f64,

    /// zonal statistic: LiDAR-derived height surface grid, maximum value of 50cm square cells sampled in this building's zone, integer centimeters
    #[serde(rename = "hgt_maxcm", with = "comma_float")]
    pub hgt_maxcm: f64,

    /// zonal statistic: LiDAR-derived height surface grid, maximum value of 50cm square cells sampled in this building's zone, integer centimeters
    #[serde(rename = "hgt_rangecm", with = "comma_float")]
    pub hgt_rangecm: f64,

    /// zonal statistic: LiDAR-derived height surface grid, mean value of 50cm square cells sampled in this building's zone, from integer centimeters
    #[serde(rename = "hgt_meancm", with = "comma_float")]
    pub hgt_meancm: f64, // Number

    /// zonal statistic: LiDAR-derived height surface grid, 1 standard deviation of 50cm square cells sampled in this building's zone, centimeters
    #[serde(rename = "hgt_stdcm", with = "comma_float")]
    pub hgt_stdcm: f64, // Number

    /// zonal statistic: LiDAR-derived height surface grid, count of unique values of 50cm square cells sampled in this building's zone, integer centimeters
    #[serde(rename = "hgt_varietycm", with = "comma_float")]
    pub hgt_varietycm: f64,

    /// zonal statistic: LiDAR-derived height surface grid, most frequently occuring value of 50cm square cells sampled in this building's zone, integer centimeters
    #[serde(rename = "hgt_majoritycm", with = "comma_float")]
    pub hgt_majoritycm: f64,

    /// zonal statistic: LiDAR-derived height surface grid, least frequently occuring value of 50cm square cells sampled in this building's zone, integer centimeters
    #[serde(rename = "hgt_minoritycm", with = "comma_float")]
    pub hgt_minoritycm: f64,

    /// zonal statistic: LiDAR-derived height surface grid, median value of 50cm square cells sampled in this building's zone, integer centimeters
    #[serde(rename = "hgt_mediancm", with = "comma_float")]
    pub hgt_mediancm: f64,

    /// summary statistic: zonal minimum ground surface height, NAVD 1988 meters
    #[serde(rename = "gnd_min_m", with = "comma_float")]
    pub gnd_min_m: f64,

    /// summary statistic: zonal median first return surface height, NAVD 1988 meters
    #[serde(rename = "median_1st_m", with = "comma_float")]
    pub median_1st_m: f64,

    /// summary statistic: zonal median height surface value, meters
    #[serde(rename = "hgt_median_m", with = "comma_float")]
    pub hgt_median_m: f64,

    /// summary statistic: discrete difference of (median first return surface -- minimum bare earth surface) for the building's zone, meters
    #[serde(rename = "gnd1st_delta", with = "comma_float")]
    pub gnd1st_delta: f64, // Number

    /// summary statistic: highest cell value of first return surface in the building's zone, NAVD 1988 meters
    #[serde(rename = "peak_1st_m", with = "comma_float")]
    pub peak_1st_m: f64, // Number

    /// Global Identifier
    #[serde(rename = "globalid")]
    pub globalid: String,  // Plain Text

    /// Multi-Polygon geography
    #[serde(rename = "shape")]
    pub shape: String,  // Multi-Polygon
}

#[cfg(test)]
mod test {
    use super::BuildingFootprintsRecord;
    use csv::Reader;
    const TEST_LINES: &str = "\u{feff}sf16_bldgid,area_id,mblr,p2010_name,p2010_zminn88ft,p2010_zmaxn88ft,gnd_cells50cm,gnd_mincm,gnd_maxcm,gnd_rangecm,gnd_meancm,gnd_stdcm,gnd_varietycm,gnd_majoritycm,gnd_minoritycm,gnd_mediancm,cells50cm_1st,mincm_1st,maxcm_1st,rangecm_1st,meancm_1st,stdcm_1st,varietycm_1st,majoritycm_1st,minoritycm_1st,mediancm_1st,hgt_cells50cm,hgt_mincm,hgt_maxcm,hgt_rangecm,hgt_meancm,hgt_stdcm,hgt_varietycm,hgt_majoritycm,hgt_minoritycm,hgt_mediancm,gnd_min_m,median_1st_m,hgt_median_m,gnd1st_delta,peak_1st_m,globalid,shape
201006.0000001,1,SF4570025,SanfranF_4606.flt,16.3249,66.267099999999999,\"178,250\",507,704,197,595.64277139000001,45.538494300000004,191,643,698,600,\"178,250\",946,\"2,385\",\"1,439\",\"1,532.2886171099999\",225.94217835000001,\"1,204\",\"1,405\",946,\"1,416\",\"178,250\",349,\"1,730\",\"1,381\",936.64459466999995,210.80556354999999,\"1,206\",813,349,850,5.0700000000000003,14.16,8.5,9.0899999999999999,23.850000000000001,{CF7EF595-68E6-4950-B361-CC82D77383A0},\"MULTIPOLYGON (((-122.37950387699998 37.73979945100002, -122.37951078200003 37.739791380999975, -122.37951088699998 37.739791260000004, -122.37942182299997 37.739742509999985, -122.37942180599998 37.73974246400002, -122.38034652099998 37.73871573500003, -122.38043307999999 37.73876573000001, -122.38044047900001 37.73875702700003, -122.38047091 37.738775042999976, -122.38048248100002 37.73876211999999, -122.38274123999997 37.74004335000001, -122.38274126300001 37.740043420999996, -122.38272735599999 37.740058594000004, -122.382815307 37.74010609700002, -122.38281534200001 37.740106208999975, -122.38255868700003 37.740391794999994, -122.38254232799999 37.74038284699998, -122.38253457500002 37.74039098200001, -122.38249726300002 37.74043014, -122.38251599 37.740440433, -122.38251602600002 37.74044054400002, -122.382075531 37.74092975000002, -122.38205777299999 37.74091976599999, -122.38167157499997 37.74134788800001, -122.381388389 37.74118806400003, -122.38133037 37.74119490300001, -122.38128900700002 37.741171742999995, -122.38128896 37.74117160700001, -122.38134406299999 37.74110932299999, -122.379374429 37.739994122999974, -122.37945644299998 37.73990475400001, -122.37949122200001 37.73986609299999, -122.37946334200002 37.73985024699999, -122.37946330400003 37.739850144, -122.37950387699998 37.73979945100002)))\"";
    #[test]
    fn test_parse_record() -> Result<(), csv::Error> {
        let mut rdr = Reader::from_reader(TEST_LINES.as_bytes());
        for line in rdr.deserialize::<BuildingFootprintsRecord>() {
            line?;
        }
        Ok(())
    }
}
