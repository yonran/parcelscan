/// sf parcels csv file LandUse2016.csv
/// https://data.sfgov.org/Housing-and-Buildings/Land-Use/us3s-fp9q
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LandUseRecord {
    #[serde(rename = "OBJECTID")]
    pub objectid: usize,
    #[serde(rename = "BLKLOT")]
    pub blklot: String,
    pub the_geom: String,
    #[serde(rename = "MAPBLKLOT")]
    pub mapblklot: String,
    #[serde(rename = "BLOCK_NUM")]
    pub block_num: String,
    #[serde(rename = "LOT_NUM")]
    pub lot_num: String,
    #[serde(rename = "FROM_ST")]
    pub from_st: Option<usize>,
    #[serde(rename = "TO_ST")]
    pub to_st: Option<usize>,
    #[serde(rename = "STREET")]
    pub street: String,
    #[serde(rename = "ST_TYPE")]
    pub st_type: String,
    #[serde(rename = "RESUNITS")]
    pub resunits: usize,
    #[serde(rename = "BLDGSQFT")]
    pub bldgsqft: usize,
    #[serde(rename = "YRBUILT")]
    pub yrbuilt: usize,
    #[serde(rename = "TOTAL_USES")]
    pub total_uses: usize,
    #[serde(rename = "LANDUSE")]
    pub landuse: String,
    #[serde(rename = "CIE")]
    pub cie: usize,
    #[serde(rename = "MED")]
    pub med: usize,
    #[serde(rename = "MIPS")]
    pub mips: usize,
    #[serde(rename = "RETAIL")]
    pub retail: usize,
    #[serde(rename = "PDR")]
    pub pdr: usize,
    #[serde(rename = "VISITOR")]
    pub visitor: usize,
    #[serde(rename = "SHAPE_Leng")]
    pub shape_leng: f64,
    #[serde(rename = "SHAPE_Area")]
    pub shape_area: f64,
}
