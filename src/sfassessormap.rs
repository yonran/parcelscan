//! Secured Property Tax Data
//! by SF Office of the Assessor-Recorder
//! Assessor excel file e.g. 2019.1.15__SF_ASR_Secured_Roll_Data_2017-2018.xlsx
//! https://sfassessor.org/news-information/property-data-0

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TaxProperty {
    /// Property Location
    /// Situs/Location, as well as room/unit number
    #[serde(rename = "PROPLOC")]
    pub proploc: String,

    /// Neighborhood Code
    /// ASR neighborhood code based on MLS districts
    #[serde(rename = "RP1NBRCDE")]
    pub rp1nbrcde: String,

    /// Block and Lot Number
    /// Block and Lot Number (Full APN)
    #[serde(rename = "RP1PRCLID")]
    pub rp1prclid: String,

    /// Volume Number
    /// Volume number
    #[serde(rename = "RP1VOLUME")]
    pub rp1volume: String,

    /// Property Class Code
    /// Property type
    #[serde(rename = "RP1CLACDE")]
    pub rp1clacde: String,

    /// Year Property Built
    /// Year improvement was built (can be blend of original and newer constructon)
    #[serde(rename = "YRBLT")]
    pub yrblt: String,

    /// Number of Bathrooms
    /// Number of bathrooms (BA with no shower or tub is 1/2 bathroom)
    #[serde(rename = "BATHS")]
    pub baths: String,

    /// Number of Bedrooms
    /// Number of bedrooms (bedrooms have a closet)
    #[serde(rename = "BEDS")]
    pub beds: String,

    /// Number of Rooms
    /// Number of rooms, excluding bathrooms, halls, closets, etc.
    #[serde(rename = "ROOMS")]
    pub rooms: String,

    /// Number of Stories
    /// Number of stories
    #[serde(rename = "STOREYNO")]
    pub storeyno: String,

    /// Number of Units
    /// Number of units
    #[serde(rename = "UNITS")]
    pub units: Option<f64>,

    /// Zoning Code
    /// Zone code
    #[serde(rename = "ZONE")]
    pub zone: String,

    /// Construction Type
    /// Generally type of construction
    #[serde(rename = "CONSTTYPE")]
    pub consttype: String,

    /// Lot Depth
    /// Depth of lot in linear feet
    #[serde(rename = "DEPTH")]
    pub depth: String,

    /// Lot Frontage
    /// Linear footage of front facing side of lot (front foot)
    #[serde(rename = "FRONT")]
    pub front: String,

    /// Property Area in Square Feet
    /// Same as lot area
    #[serde(rename = "SQFT")]
    pub sqft: String,

    /// Basement Area
    /// Square footage of basement
    #[serde(rename = "FBA")]
    pub fba: String,

    /// Lot Area
    /// Square footage of lot
    #[serde(rename = "LAREA")]
    pub larea: f64,

    /// Lot Code
    /// lot shapes, could be Rectangle, Square or Other
    #[serde(rename = "LOTCODE")]
    pub lotcode: String,

    /// Prior Sales Date (YYMMDD)
    /// prior sale date
    #[serde(rename = "REPRISDATE")]
    pub reprisdate: String,

    /// Tax Rate Area Code
    /// Tax rate dependent on location within the City
    #[serde(rename = "RP1TRACDE")]
    pub rp1tracde: String,

    /// Percent of Ownership
    /// Percent of ownership
    #[serde(rename = "OWNRPRCNT")]
    pub ownrprcnt: String,

    /// Closed Roll Exemption Type Code
    /// Exemption Code (see below for descriptions)
    #[serde(rename = "EXEMPTYPE")]
    pub exemptype: String,

    /// Closed Roll Status Code
    /// i.e. Taxable, Non-Taxable, SBE, etc.
    #[serde(rename = "RP1STACDE")]
    pub rp1stacde: String,

    /// Closed Roll Misc. Exemption Value
    /// Exemptions such as welfare
    #[serde(rename = "RP1EXMVL2")]
    pub rp1exmvl2: String,

    /// Closed Roll Homeowner Exemption Value
    /// Homeowner's exemption
    #[serde(rename = "RP1EXMVL1")]
    pub rp1exmvl1: String,

    /// Closed Roll Year
    /// Roll Year
    #[serde(rename = "ROLLYEAR")]
    pub rollyear: String,

    /// Current Sales Date (YYMMDD)
    /// current sale date
    #[serde(rename = "RECURRSALD")]
    pub recurrsald: String,

    /// Closed Roll Assessed Fixtures Value
    /// Assessed value of fixtures
    #[serde(rename = "RP1FXTVAL")]
    pub rp1fxtval: String,

    /// Closed Roll Assessed Improvement Value
    /// Assessed value of improvements
    #[serde(rename = "RP1IMPVAL")]
    pub rp1impval: String,

    /// Closed Roll Assessed Land Value
    /// Assessed value of land
    #[serde(rename = "RP1LNDVAL")]
    pub rp1lndval: String,

    /// Closed Roll Assessed Personal Prop Value
    /// Assessed value of personal property
    #[serde(rename = "RP1PPTVAL")]
    pub rp1pptval: String,
}

#[cfg(test)]
mod test {
    use super::super::xlsxdeserialize;
    use super::TaxProperty;
    use calamine::Reader;
    use calamine::Xlsx;
    use std::error::Error;
    use std::fmt::Display;
    use std::fmt::Formatter;
    use std::fs::File;
    use std::io::BufReader;
    use std::path::Path;

    // XlsxError does not impl Error
    #[derive(Debug)]
    struct WrapXlsxError(calamine::XlsxError);
    impl Display for WrapXlsxError {
        fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
            write!(f, "XlsxError {:?}", self.0)
        }
    }
    impl Error for WrapXlsxError {}

    #[allow(unused)]
    fn scan_assessor<P: AsRef<Path>>(assessor: P) -> Result<(), Box<Error>> {
        let assessor: &Path = assessor.as_ref();
        let mut assessor_workbook = calamine::open_workbook::<Xlsx<BufReader<File>>, _>(assessor)
            .map_err(|e| Box::new(WrapXlsxError(e)))?;
        info!("Expansions");
        info!(
            "Worksheets: {:?}; reading first sheet...",
            assessor_workbook.sheet_names()
        );
        let sheet_name: String = assessor_workbook.sheet_names()[0].clone();
        let sheet = assessor_workbook
            .worksheet_range(&sheet_name)
            .expect("First worksheet should exist")
            .map_err(|e| Box::new(WrapXlsxError(e)))?;
        info!("Sheet has size {:?}", sheet.get_size());
        info!("Used cells: {}", sheet.used_cells().count());
        let rows = sheet.rows();
        for row_result in xlsxdeserialize::deserialize::<TaxProperty>(rows) {
            let row = row_result?;
            let _units = row.units.unwrap_or(0f64);
            let _area = row.larea;
            let _sqft = row.sqft;
        }
        info!("done");
        Ok(())
    }
}
