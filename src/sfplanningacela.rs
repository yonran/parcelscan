//! Parser for SF Planning PPTS records of planning permits
//! https://data.sfgov.org/Housing-and-Buildings/PPTS-Records/7yuw-98m5
//! File name: PPTS_Records_data.csv
//!
//! See also DBI monthly permits
//!

/// Fields Reference: http://default.sfplanning.org/GIS/DataSF_PPTS_Fields.xlsx
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PPTSRecord {
    pub the_geom: String,
    /// Estimated construction cost in dollars of the project
    #[serde(rename = "constructcost")]
    pub constructcost: String,

    /// The system ID generated by Esri ArcGIS software. Not an administrative ID.
    #[serde(rename = "OBJECTID")]
    pub objectid: String,

    /// Planning Department unique identifier for the record
    #[serde(rename = "record_id")]
    pub record_id: String,

    /// Date the record was created
    #[serde(rename = "date_opened")]
    pub date_opened: String,

    /// Record status
    #[serde(rename = "record_status")]
    pub record_status: String,

    /// Date the record was closed
    #[serde(rename = "date_closed")]
    pub date_closed: String,

    /// The address of the project.  There may be multiple addresses related to a project, in these cases the primary address is displayed in this field.
    #[serde(rename = "address")]
    pub address: String,

    /// Record type
    #[serde(rename = "record_type")]
    pub record_type: String,

    /// Record type category
    #[serde(rename = "record_type_category")]
    pub record_type_category: String,

    /// Record type group
    #[serde(rename = "record_type_group")]
    pub record_type_group: String,

    /// Record type subgroup
    #[serde(rename = "record_type_subtype")]
    pub record_type_subtype: String,

    /// Record type 2
    #[serde(rename = "record_type_type")]
    pub record_type_type: String,

    /// Record type 4 level
    #[serde(rename = "record_type_4level")]
    pub record_type_4level: String,

    /// Name of the record
    #[serde(rename = "record_name")]
    pub record_name: String,

    /// Description
    #[serde(rename = "description")]
    pub description: String,

    /// The ID of the planner assigned to this record
    #[serde(rename = "planner_id")]
    pub planner_id: String,

    /// Module.  This is used to display the Ctiy department that owns this record
    #[serde(rename = "module")]
    pub module: String,

    /// Unique system identifier for the record
    #[serde(rename = "templateid")]
    pub templateid: String,

    /// Parent record ID if this record is a child of another record
    #[serde(rename = "parent")]
    pub parent: String,

    /// The children record IDs if this record is a parent to other records
    #[serde(rename = "children")]
    pub children: String,

    /// Name of the planner assigned to this record
    #[serde(rename = "planner_name")]
    pub planner_name: String,

    /// Email address of the planner assigned to this record
    #[serde(rename = "planner_email")]
    pub planner_email: String,

    /// Phone number of the planner assigned to this record
    #[serde(rename = "planner_phone")]
    pub planner_phone: String,

    /// Link to this record in Accela Citizen Access
    #[serde(rename = "acalink")]
    pub acalink: String,

    /// Link to this record in Accela Automation
    #[serde(rename = "aalink")]
    pub aalink: String,

    /// Project Description - Change of Use
    #[serde(rename = "CHANGE_OF_USE")]
    pub change_of_use: String,

    /// Project Description - Additions
    #[serde(rename = "ADDITIONS")]
    pub additions: String,

    /// Project Description - New Construction
    #[serde(rename = "NEW_CONSTRUCTION")]
    pub new_construction: String,

    /// Project Description - Legislative/Zoning Change
    #[serde(rename = "LEG_ZONE_CHANGE")]
    pub leg_zone_change: String,

    /// Project Description - Demolition
    #[serde(rename = "DEMOLITION")]
    pub demolition: String,

    /// Project Description - Lot Line Adjustment-Subdivision
    #[serde(rename = "LOT_LINE_ADJUST")]
    pub lot_line_adjust: String,

    /// Project Description - Facade Alterations
    #[serde(rename = "FACADE_ALT")]
    pub facade_alt: String,

    /// Project Description - ROW Improvements
    #[serde(rename = "ROW_IMPROVE")]
    pub row_improve: String,

    /// Project Description - Other
    #[serde(rename = "OTHER_PRJ_DESC")]
    pub other_prj_desc: String,

    /// Project Description - Special Needs Housing
    #[serde(rename = "SPECIAL_NEEDS")]
    pub special_needs: String,

    /// Project Description - Senior Housing
    #[serde(rename = "SENIOR")]
    pub senior: String,

    /// Project Description - 100% Affordable Housing
    #[serde(rename = "AFFORDABLE_UNITS")]
    pub affordable_units: String,

    /// Project Description - Student Housing
    #[serde(rename = "STUDENT")]
    pub student: String,

    /// Project Description - Inclusionary Housing Required
    #[serde(rename = "INCLUSIONARY")]
    pub inclusionary: String,

    /// Project Description - State Density Bonus
    #[serde(rename = "STATE_DENSITY_BONUS")]
    pub state_density_bonus: String,

    /// Project Description - Accessory Dwelling Unit
    #[serde(rename = "ADU")]
    pub adu: String,

    /// Project Description - Formula Retail
    #[serde(rename = "FORMULA_RETAIL")]
    pub formula_retail: String,

    /// Project Description - Medical Cannabis Dispensary
    #[serde(rename = "MCD")]
    pub mcd: String,

    /// Project Description - Tobacco Paraphernalia Est
    #[serde(rename = "TOBACCO")]
    pub tobacco: String,

    /// Project Description - Financial Services
    #[serde(rename = "FINANCIAL")]
    pub financial: String,

    /// Project Description - Massage Establishment
    #[serde(rename = "MASSAGE")]
    pub massage: String,

    /// Project Description - Public Health Review - MCD
    #[serde(rename = "MCD_REFERRAL")]
    pub mcd_referral: String,

    /// Project Description - Non-Residential Use Type - Other
    #[serde(rename = "OTHER_NON_RES")]
    pub other_non_res: String,

    /// Project Description - Environmental Review
    #[serde(rename = "ENVIRONMENTAL_REVIEW_TYPE")]
    pub environmental_review_type: String,

    /// Land Use - Retail/Commercial (sq ft) - Existing
    #[serde(rename = "LAND_USE_RC_EXIST")]
    pub land_use_rc_exist: Option<f64>,

    /// Land Use - Retail/Commercial (sq ft) - Proposed
    #[serde(rename = "LAND_USE_RC_PROP")]
    pub land_use_rc_prop: Option<f64>,

    /// Land Use - Retail/Commercial (sq ft) - Net
    #[serde(rename = "LAND_USE_RC_NET")]
    pub land_use_rc_net: Option<f64>,

    /// Land Use - Residential (sq ft) - Existing
    #[serde(rename = "LAND_USE_RESIDENTIAL_EXIST")]
    pub land_use_residential_exist: Option<f64>,

    /// Land Use - Residential (sq ft) - Proposed
    #[serde(rename = "LAND_USE_RESIDENTIAL_PROP")]
    pub land_use_residential_prop: Option<f64>,

    /// Land Use - Residential (sq ft) - Net
    #[serde(rename = "LAND_USE_RESIDENTIAL_NET")]
    pub land_use_residential_net: Option<f64>,

    /// Land Use - CIE (Cultural, Institutional, Educational) - Existing
    #[serde(rename = "LAND_USE_CIE_EXIST")]
    pub land_use_cie_exist: Option<f64>,

    /// Land Use - CIE (Cultural, Institutional, Educational) - Proposed
    #[serde(rename = "LAND_USE_CIE_PROP")]
    pub land_use_cie_prop: Option<f64>,

    /// Land Use - CIE (Cultural, Institutional, Educational) - Net
    #[serde(rename = "LAND_USE_CIE_NET")]
    pub land_use_cie_net: Option<f64>,

    /// Land Use - Industrial-PDR (sq ft) - Existing
    #[serde(rename = "LAND_USE_PDR_EXIST")]
    pub land_use_pdr_exist: Option<f64>,

    /// Land Use - Industrial-PDR (sq ft) - Proposed
    #[serde(rename = "LAND_USE_PDR_PROP")]
    pub land_use_pdr_prop: Option<f64>,

    /// Land Use - Industrial-PDR (sq ft) - Net
    #[serde(rename = "LAND_USE_PDR_NET")]
    pub land_use_pdr_net: Option<f64>,

    /// Land Use - Office (sq ft) - Existing
    #[serde(rename = "LAND_USE_OFFICE_EXIST")]
    pub land_use_office_exist: Option<f64>,

    /// Land Use - Office (sq ft) - Proposed
    #[serde(rename = "LAND_USE_OFFICE_PROP")]
    pub land_use_office_prop: Option<f64>,

    /// Land Use - Office (sq ft) - Net
    #[serde(rename = "LAND_USE_OFFICE_NET")]
    pub land_use_office_net: Option<f64>,

    /// Land Use - Medical (sq ft) - Existing
    #[serde(rename = "LAND_USE_MEDICAL_EXIST")]
    pub land_use_medical_exist: Option<f64>,

    /// Land Use - Medical (sq ft) - Proposed
    #[serde(rename = "LAND_USE_MEDICAL_PROP")]
    pub land_use_medical_prop: Option<f64>,

    /// Land Use - Medical (sq ft) - Net
    #[serde(rename = "LAND_USE_MEDICAL_NET")]
    pub land_use_medical_net: Option<f64>,

    /// Land Use - Visitor (sq ft) - Existing
    #[serde(rename = "LAND_USE_VISITOR_EXIST")]
    pub land_use_visitor_exist: Option<f64>,

    /// Land Use - Visitor (sq ft) - Proposed
    #[serde(rename = "LAND_USE_VISITOR_PROP")]
    pub land_use_visitor_prop: Option<f64>,

    /// Land Use - Visitor (sq ft) - Net
    #[serde(rename = "LAND_USE_VISITOR_NET")]
    pub land_use_visitor_net: Option<f64>,

    /// Land Use - Parking Spaces (sq ft) - Existing
    #[serde(rename = "LAND_USE_PARKING_SPACES_EXIST")]
    pub land_use_parking_spaces_exist: Option<f64>,

    /// Land Use - Parking Spaces (sq ft) - Proposed
    #[serde(rename = "LAND_USE_PARKING_SPACES_PROP")]
    pub land_use_parking_spaces_prop: Option<f64>,

    /// Land Use - Parking Spaces (sq ft) - Net
    #[serde(rename = "LAND_USE_PARKING_SPACES_NET")]
    pub land_use_parking_spaces_net: Option<f64>,

    /// Project Features - Dwelling Units-Affordable - Existing Unit(s)
    #[serde(rename = "PRJ_FEATURE_AFFORDABLE_EXIST")]
    pub prj_feature_affordable_exist: String,

    /// Project Features - Dwelling Units-Affordable - Proposed Unit(s)
    #[serde(rename = "PRJ_FEATURE_AFFORDABLE_PROP")]
    pub prj_feature_affordable_prop: String,

    /// Project Features - Dwelling Units-Affordable - Net Unit(s)
    #[serde(rename = "PRJ_FEATURE_AFFORDABLE_NET")]
    pub prj_feature_affordable_net: String,

    /// Project Features - Hotel Rooms - Existing
    #[serde(rename = "PRJ_FEATURE_HOTEL_ROOMS_EXIST")]
    pub prj_feature_hotel_rooms_exist: String,

    /// Project Features - Hotel Rooms - Proposed
    #[serde(rename = "PRJ_FEATURE_HOTEL_ROOMS_PROP")]
    pub prj_feature_hotel_rooms_prop: String,

    /// Project Features - Hotel Rooms - Net
    #[serde(rename = "PRJ_FEATURE_HOTEL_ROOMS_NET")]
    pub prj_feature_hotel_rooms_net: String,

    /// Project Features - Dwelling Units-Market Rate - Existing Unit(s)
    #[serde(rename = "PRJ_FEATURE_MARKET_RATE_EXIST")]
    pub prj_feature_market_rate_exist: String,

    /// Project Features - Dwelling Units-Market Rate - Proposed Unit(s)
    #[serde(rename = "PRJ_FEATURE_MARKET_RATE_PROP")]
    pub prj_feature_market_rate_prop: String,

    /// Project Features - Dwelling Units-Market Rate - Net Unit(s)
    #[serde(rename = "PRJ_FEATURE_MARKET_RATE_NET")]
    pub prj_feature_market_rate_net: String,

    /// Project Features - Building Number - Existing
    #[serde(rename = "PRJ_FEATURE_BUILD_EXIST")]
    pub prj_feature_build_exist: String,

    /// Project Features - Building Number - Proposed
    #[serde(rename = "PRJ_FEATURE_BUILD_PROP")]
    pub prj_feature_build_prop: String,

    /// Project Features - Building Number - Net
    #[serde(rename = "PRJ_FEATURE_BUILD_NET")]
    pub prj_feature_build_net: String,

    /// Project Features - Stories Number - Existing
    #[serde(rename = "PRJ_FEATURE_STORIES_PROP")]
    pub prj_feature_stories_prop: String,

    /// Project Features - Stories Number - Proposed
    #[serde(rename = "PRJ_FEATURE_STORIES_NET")]
    pub prj_feature_stories_net: String,

    /// Project Features - Stories Number - Net
    #[serde(rename = "PRJ_FEATURE_PARKING_EXIST")]
    pub prj_feature_parking_exist: String,

    /// Project Features - Parking Spaces - Proposed
    #[serde(rename = "PRJ_FEATURE_PARKING_PROP")]
    pub prj_feature_parking_prop: String,

    /// Project Features - Parking Spaces - Net
    #[serde(rename = "PRJ_FEATURE_PARKING_NET")]
    pub prj_feature_parking_net: String,

    /// Project Features - Loading Spaces - Existing
    #[serde(rename = "PRJ_FEATURE_LOADING_EXIST")]
    pub prj_feature_loading_exist: String,

    /// Project Features - Loading Spaces - Proposed
    #[serde(rename = "PRJ_FEATURE_LOADING_PROP")]
    pub prj_feature_loading_prop: String,

    /// Project Features - Loading Spaces - Net
    #[serde(rename = "PRJ_FEATURE_LOADING_NET")]
    pub prj_feature_loading_net: String,

    /// Project Features - Bicycle Spaces - Existing
    #[serde(rename = "PRJ_FEATURE_BIKE_EXIST")]
    pub prj_feature_bike_exist: String,

    /// Project Features - Bicycle Spaces - Proposed
    #[serde(rename = "PRJ_FEATURE_BIKE_PROP")]
    pub prj_feature_bike_prop: String,

    /// Project Features - Bicycle Spaces - Net
    #[serde(rename = "PRJ_FEATURE_BIKE_NET")]
    pub prj_feature_bike_net: String,

    /// Project Features - Car Share Spaces - Existing
    #[serde(rename = "PRJ_FEATURE_CAR_SHARE_EXIST")]
    pub prj_feature_car_share_exist: String,

    /// Project Features - Car Share Spaces - Proposed
    #[serde(rename = "PRJ_FEATURE_CAR_SHARE_PROP")]
    pub prj_feature_car_share_prop: String,

    /// Project Features - Car Share Spaces - Net
    #[serde(rename = "PRJ_FEATURE_CAR_SHARE_NET")]
    pub prj_feature_car_share_net: String,

    /// Project Features - Usable Open Spaces - Existing
    #[serde(rename = "PRJ_FEATURE_USABLE_EXIST")]
    pub prj_feature_usable_exist: String,

    /// Project Features - Usable Open Spaces - Proposed
    #[serde(rename = "PRJ_FEATURE_USABLE_PROP")]
    pub prj_feature_usable_prop: String,

    /// Project Features - Usable Open Spaces - Existing
    #[serde(rename = "PRJ_FEATURE_USABLE_NET")]
    pub prj_feature_usable_net: String,

    /// Project Features - Public Open Space - Existing
    #[serde(rename = "PRJ_FEATURE_PUBLIC_EXIST")]
    pub prj_feature_public_exist: String,

    /// Project Features - Public Open Space - Proposed
    #[serde(rename = "PRJ_FEATURE_PUBLIC_PROP")]
    pub prj_feature_public_prop: String,

    /// Project Features - Public Open Space - Net
    #[serde(rename = "PRJ_FEATURE_PUBLIC_NET")]
    pub prj_feature_public_net: String,

    /// Project Features - Public Art - Existing
    #[serde(rename = "PRJ_FEATURE_ART_EXIST")]
    pub prj_feature_art_exist: String,

    /// Project Features - Public Art - Proposed
    #[serde(rename = "PRJ_FEATURE_ART_PROP")]
    pub prj_feature_art_prop: String,

    /// Project Features - Public Art - Net
    #[serde(rename = "PRJ_FEATURE_ART_NET")]
    pub prj_feature_art_net: String,

    /// Project Features - Better Roof - Total Roof Area - Existing
    #[serde(rename = "PRJ_FEATURE_ROOF_EXIST")]
    pub prj_feature_roof_exist: String,

    /// Project Features - Better Roof - Total Roof Area - Propsoed
    #[serde(rename = "PRJ_FEATURE_ROOF_PROP")]
    pub prj_feature_roof_prop: String,

    /// Project Features - Better Roof - Total Roof Area - Net
    #[serde(rename = "PRJ_FEATURE_ROOF_NET")]
    pub prj_feature_roof_net: String,

    /// Project Features - Better Roof - Solar Area - Existing
    #[serde(rename = "PRJ_FEATURE_SOLAR_EXIST")]
    pub prj_feature_solar_exist: String,

    /// Project Features - Better Roof - Solar Area - Proposed
    #[serde(rename = "PRJ_FEATURE_SOLAR_PROP")]
    pub prj_feature_solar_prop: String,

    /// Project Features - Better Roof - Solar Area - Net
    #[serde(rename = "PRJ_FEATURE_SOLAR_NET")]
    pub prj_feature_solar_net: String,

    /// Project Features - Better Roof - Living Roof Area - Existing
    #[serde(rename = "PRJ_FEATURE_LIVING_EXIST")]
    pub prj_feature_living_exist: String,

    /// Project Features - Better Roof - Living Roof Area - Proposed
    #[serde(rename = "PRJ_FEATURE_LIVING_PROP")]
    pub prj_feature_living_prop: String,

    /// Project Features - Better Roof - Living Roof Area - Net
    #[serde(rename = "PRJ_FEATURE_LIVING_NET")]
    pub prj_feature_living_net: String,

    /// Project Features - Other Project Feature
    #[serde(rename = "PRJ_FEATURE_OTHER")]
    pub prj_feature_other: String,

    /// Project Features - Other Project Feature - Existing Unit(s)
    #[serde(rename = "PRJ_FEATURE_OTHER_EXIST")]
    pub prj_feature_other_exist: String,

    /// Project Features - Other Project Feature - Proposed Unit(s)
    #[serde(rename = "PRJ_FEATURE_OTHER_PROP")]
    pub prj_feature_other_prop: String,

    /// Project Features - Other Project Feature - Net Unit(s)
    #[serde(rename = "PRJ_FEATURE_OTHER_NET")]
    pub prj_feature_other_net: String,

    /// Land Use - Residential - Dwelling Unit Type - Studios - Existing Unit(s)
    #[serde(rename = "RESIDENTIAL_STUDIO_EXIST")]
    pub residential_studio_exist: String,

    /// Land Use - Residential - Dwelling Unit Type - Studios - Proposed Unit(s)
    #[serde(rename = "RESIDENTIAL_STUDIO_PROP")]
    pub residential_studio_prop: String,

    /// Land Use - Residential - Dwelling Unit Type - Studios - Net Unit(s)
    #[serde(rename = "RESIDENTIAL_STUDIO_NET")]
    pub residential_studio_net: String,

    /// Land Use - Residential - Dwelling Unit Type - 1 Bedroom - Existing Unit(s)
    #[serde(rename = "RESIDENTIAL_1BR_EXIST")]
    pub residential_1br_exist: String,

    /// Land Use - Residential - Dwelling Unit Type - 1 Bedroom - Proposed Unit(s)
    #[serde(rename = "RESIDENTIAL_1BR_PROP")]
    pub residential_1br_prop: String,

    /// Land Use - Residential - Dwelling Unit Type - 1 Bedroom - Net Unit(s)
    #[serde(rename = "RESIDENTIAL_1BR_NET")]
    pub residential_1br_net: String,

    /// Land Use - Residential - Dwelling Unit Type - 2 Bedroom - Existing Unit(s)
    #[serde(rename = "RESIDENTIAL_2BR_EXIST")]
    pub residential_2br_exist: String,

    /// Land Use - Residential - Dwelling Unit Type - 2 Bedroom - Proposed Unit(s)
    #[serde(rename = "RESIDENTIAL_2BR_PROP")]
    pub residential_2br_prop: String,

    /// Land Use - Residential - Dwelling Unit Type - 2 Bedroom - Net Unit(s)
    #[serde(rename = "RESIDENTIAL_2BR_NET")]
    pub residential_2br_net: String,

    /// Land Use - Residential - Dwelling Unit Type - 3+ Bedroom - Existing Unit(s)
    #[serde(rename = "RESIDENTIAL_3BR_EXIST")]
    pub residential_3br_exist: String,

    /// Land Use - Residential - Dwelling Unit Type - 3+ Bedroom - Proposed Unit(s)
    #[serde(rename = "RESIDENTIAL_3BR_PROP")]
    pub residential_3br_prop: String,

    /// Land Use - Residential - Dwelling Unit Type - 3+ Bedroom - Net Unit(s)
    #[serde(rename = "RESIDENTIAL_3BR_NET")]
    pub residential_3br_net: String,

    /// Land Use - Residential - Dwelling Unit Type - Accessory Dwelling Unit Studio - Existing Unit(s)
    #[serde(rename = "RESIDENTIAL_ADU_STUDIO_EXIST")]
    pub residential_adu_studio_exist: String,

    /// Land Use - Residential - Dwelling Unit Type - Accessory Dwelling Unit Studio - Proposed Unit(s)
    #[serde(rename = "RESIDENTIAL_ADU_STUDIO_PROP")]
    pub residential_adu_studio_prop: String,

    /// Land Use - Residential - Dwelling Unit Type - Accessory Dwelling Unit Studio - Net Unit(s)
    #[serde(rename = "RESIDENTIAL_ADU_STUDIO_NET")]
    pub residential_adu_studio_net: String,

    /// Land Use - Residential - Dwelling Unit Type - Accessory Dwelling Unit Studio - Area
    #[serde(rename = "RESIDENTIAL_ADU_STUDIO_AREA")]
    pub residential_adu_studio_area: String,

    /// Land Use - Residential - Dwelling Unit Type - Accessory Dwelling Unit 1 Bedroom - Existing Unit(s)
    #[serde(rename = "RESIDENTIAL_ADU_1BR_EXIST")]
    pub residential_adu_1br_exist: String,

    /// Land Use - Residential - Dwelling Unit Type - Accessory Dwelling Unit 1 Bedroom - Proposed Unit(s)
    #[serde(rename = "RESIDENTIAL_ADU_1BR_PROP")]
    pub residential_adu_1br_prop: String,

    /// Land Use - Residential - Dwelling Unit Type - Accessory Dwelling Unit 1 Bedroom - Net Unit(s)
    #[serde(rename = "RESIDENTIAL_ADU_1BR_NET")]
    pub residential_adu_1br_net: String,

    /// Land Use - Residential - Dwelling Unit Type - Accessory Dwelling Unit 1 Bedroom - Area
    #[serde(rename = "RESIDENTIAL_ADU_1BR_AREA")]
    pub residential_adu_1br_area: String,

    /// Land Use - Residential - Dwelling Unit Type - Accessory Dwelling Unit 2 Bedroom - Existing Unit(s)
    #[serde(rename = "RESIDENTIAL_ADU_2BR_EXIST")]
    pub residential_adu_2br_exist: String,

    /// Land Use - Residential - Dwelling Unit Type - Accessory Dwelling Unit 2 Bedroom - Proposed Unit(s)
    #[serde(rename = "RESIDENTIAL_ADU_2BR_PROP")]
    pub residential_adu_2br_prop: String,

    /// Land Use - Residential - Dwelling Unit Type - Accessory Dwelling Unit 2 Bedroom - Net Unit(s)
    #[serde(rename = "RESIDENTIAL_ADU_2BR_NET")]
    pub residential_adu_2br_net: String,

    /// Land Use - Residential - Dwelling Unit Type - Accessory Dwelling Unit 2 Bedroom - Area
    #[serde(rename = "RESIDENTIAL_ADU_2BR_AREA")]
    pub residential_adu_2br_area: String,

    /// Land Use - Residential - Dwelling Unit Type - Accessory Dwelling Unit 3+ Bedroom - Existing Unit(s)
    #[serde(rename = "RESIDENTIAL_ADU_3BR_EXIST")]
    pub residential_adu_3br_exist: String,

    /// Land Use - Residential - Dwelling Unit Type - Accessory Dwelling Unit 3+ Bedroom - Proposed Unit(s)
    #[serde(rename = "RESIDENTIAL_ADU_3BR_PROP")]
    pub residential_adu_3br_prop: String,

    /// Land Use - Residential - Dwelling Unit Type - Accessory Dwelling Unit 3+ Bedroom - Net Unit(s)
    #[serde(rename = "RESIDENTIAL_ADU_3BR_NET")]
    pub residential_adu_3br_net: String,

    /// Land Use - Residential - Dwelling Unit Type - Accessory Dwelling Unit 3+ Bedroom - Area
    #[serde(rename = "RESIDENTIAL_ADU_3BR_AREA")]
    pub residential_adu_3br_area: String,

    /// Land Use - Residential - Dwelling Unit Type - Group Housing - Rooms - Existing
    #[serde(rename = "RESIDENTIAL_GH_ROOMS_EXIST")]
    pub residential_gh_rooms_exist: String,

    /// Land Use - Residential - Dwelling Unit Type - Group Housing - Rooms - Prosed
    #[serde(rename = "RESIDENTIAL_GH_ROOMS_PROP")]
    pub residential_gh_rooms_prop: String,

    /// Land Use - Residential - Dwelling Unit Type - Group Housing - Rooms - Net
    #[serde(rename = "RESIDENTIAL_GH_ROOMS_NET")]
    pub residential_gh_rooms_net: String,

    /// Land Use - Residential - Dwelling Unit Type - Group Housing - Beds - Existing
    #[serde(rename = "RESIDENTIAL_GH_BEDS_EXIST")]
    pub residential_gh_beds_exist: String,

    /// Land Use - Residential - Dwelling Unit Type - Group Housing - Beds - Prosed
    #[serde(rename = "RESIDENTIAL_GH_BEDS_PROP")]
    pub residential_gh_beds_prop: String,

    /// Land Use - Residential - Dwelling Unit Type - Group Housing - Beds - Net
    #[serde(rename = "RESIDENTIAL_GH_BEDS_NET")]
    pub residential_gh_beds_net: String,

    /// Land Use - Residential - Dwelling Unit Type - SRO - Existing Unit(s)
    #[serde(rename = "RESIDENTIAL_SRO_EXIST")]
    pub residential_sro_exist: String,

    /// Land Use - Residential - Dwelling Unit Type - SRO - Proposed Unit(s)
    #[serde(rename = "RESIDENTIAL_SRO_PROP")]
    pub residential_sro_prop: String,

    /// Land Use - Residential - Dwelling Unit Type - SRO - Net Unit(s)
    #[serde(rename = "RESIDENTIAL_SRO_NET")]
    pub residential_sro_net: String,

    /// Land Use - Residential - Dwelling Unit Type - Micro - Existing Unit(s)
    #[serde(rename = "RESIDENTIAL_MICRO_EXIST")]
    pub residential_micro_exist: String,

    /// Land Use - Residential - Dwelling Unit Type - Micro - Proposed Unit(s)
    #[serde(rename = "RESIDENTIAL_MICRO_PROP")]
    pub residential_micro_prop: String,

    /// Land Use - Residential - Dwelling Unit Type - Micro - Net Unit(s)
    #[serde(rename = "RESIDENTIAL_MICRO_NET")]
    pub residential_micro_net: String,

    /// Related Building Permit Number
    #[serde(rename = "RELATED_BUILDING_PERMIT")]
    pub related_building_permit: String,

    /// Full Board Hearing Date 1
    #[serde(rename = "BOS_1ST_READ")]
    pub bos_1st_read: String,

    /// Full Board Hearing Date 2
    #[serde(rename = "BOS_2ND_READ")]
    pub bos_2nd_read: String,

    /// Committee Hearing Date
    #[serde(rename = "COM_HEARING")]
    pub com_hearing: String,

    /// Mayoral Action - Ordinance Signed Date
    #[serde(rename = "MAYORAL_SIGN")]
    pub mayoral_sign: String,

    /// Materials Hearing to BOS Clerk Date
    #[serde(rename = "TRANSMIT_DATE_BOS")]
    pub transmit_date_bos: String,

    /// Committee Hearing Date - BOS Review
    #[serde(rename = "COM_HEARING_DATE_BOS")]
    pub com_hearing_date_bos: String,
}

#[cfg(test)]
mod test {
    use super::PPTSRecord;
    use csv::Reader;
    const TEST_LINES: &str = "the_geom,OBJECTID,record_id,date_opened,record_status,date_closed,address,record_type,record_type_category,record_type_group,record_type_subtype,record_type_type,record_type_4level,record_name,description,planner_id,module,templateid,parent,children,constructcost,planner_name,planner_email,planner_phone,acalink,aalink,CHANGE_OF_USE,ADDITIONS,NEW_CONSTRUCTION,LEG_ZONE_CHANGE,DEMOLITION,LOT_LINE_ADJUST,FACADE_ALT,ROW_IMPROVE,OTHER_PRJ_DESC,SPECIAL_NEEDS,SENIOR,AFFORDABLE_UNITS,STUDENT,INCLUSIONARY,STATE_DENSITY_BONUS,ADU,FORMULA_RETAIL,MCD,TOBACCO,FINANCIAL,MASSAGE,MCD_REFERRAL,OTHER_NON_RES,ENVIRONMENTAL_REVIEW_TYPE,LAND_USE_RC_EXIST,LAND_USE_RC_PROP,LAND_USE_RC_NET,LAND_USE_RESIDENTIAL_EXIST,LAND_USE_RESIDENTIAL_PROP,LAND_USE_RESIDENTIAL_NET,LAND_USE_CIE_EXIST,LAND_USE_CIE_PROP,LAND_USE_CIE_NET,LAND_USE_PDR_EXIST,LAND_USE_PDR_PROP,LAND_USE_PDR_NET,LAND_USE_OFFICE_EXIST,LAND_USE_OFFICE_PROP,LAND_USE_OFFICE_NET,LAND_USE_MEDICAL_EXIST,LAND_USE_MEDICAL_PROP,LAND_USE_MEDICAL_NET,LAND_USE_VISITOR_EXIST,LAND_USE_VISITOR_PROP,LAND_USE_VISITOR_NET,LAND_USE_PARKING_SPACES_EXIST,LAND_USE_PARKING_SPACES_PROP,LAND_USE_PARKING_SPACES_NET,PRJ_FEATURE_AFFORDABLE_EXIST,PRJ_FEATURE_AFFORDABLE_PROP,PRJ_FEATURE_AFFORDABLE_NET,PRJ_FEATURE_HOTEL_ROOMS_EXIST,PRJ_FEATURE_HOTEL_ROOMS_PROP,PRJ_FEATURE_HOTEL_ROOMS_NET,PRJ_FEATURE_MARKET_RATE_EXIST,PRJ_FEATURE_MARKET_RATE_PROP,PRJ_FEATURE_MARKET_RATE_NET,PRJ_FEATURE_BUILD_EXIST,PRJ_FEATURE_BUILD_PROP,PRJ_FEATURE_BUILD_NET,PRJ_FEATURE_STORIES_EXIST,PRJ_FEATURE_STORIES_PROP,PRJ_FEATURE_STORIES_NET,PRJ_FEATURE_PARKING_EXIST,PRJ_FEATURE_PARKING_PROP,PRJ_FEATURE_PARKING_NET,PRJ_FEATURE_LOADING_EXIST,PRJ_FEATURE_LOADING_PROP,PRJ_FEATURE_LOADING_NET,PRJ_FEATURE_BIKE_EXIST,PRJ_FEATURE_BIKE_PROP,PRJ_FEATURE_BIKE_NET,PRJ_FEATURE_CAR_SHARE_EXIST,PRJ_FEATURE_CAR_SHARE_PROP,PRJ_FEATURE_CAR_SHARE_NET,PRJ_FEATURE_USABLE_EXIST,PRJ_FEATURE_USABLE_PROP,PRJ_FEATURE_USABLE_NET,PRJ_FEATURE_PUBLIC_EXIST,PRJ_FEATURE_PUBLIC_PROP,PRJ_FEATURE_PUBLIC_NET,PRJ_FEATURE_ART_EXIST,PRJ_FEATURE_ART_PROP,PRJ_FEATURE_ART_NET,PRJ_FEATURE_ROOF_EXIST,PRJ_FEATURE_ROOF_PROP,PRJ_FEATURE_ROOF_NET,PRJ_FEATURE_SOLAR_EXIST,PRJ_FEATURE_SOLAR_PROP,PRJ_FEATURE_SOLAR_NET,PRJ_FEATURE_LIVING_EXIST,PRJ_FEATURE_LIVING_PROP,PRJ_FEATURE_LIVING_NET,PRJ_FEATURE_OTHER,PRJ_FEATURE_OTHER_EXIST,PRJ_FEATURE_OTHER_PROP,PRJ_FEATURE_OTHER_NET,RESIDENTIAL_STUDIO_EXIST,RESIDENTIAL_STUDIO_PROP,RESIDENTIAL_STUDIO_NET,RESIDENTIAL_1BR_EXIST,RESIDENTIAL_1BR_PROP,RESIDENTIAL_1BR_NET,RESIDENTIAL_2BR_EXIST,RESIDENTIAL_2BR_PROP,RESIDENTIAL_2BR_NET,RESIDENTIAL_3BR_EXIST,RESIDENTIAL_3BR_PROP,RESIDENTIAL_3BR_NET,RESIDENTIAL_ADU_STUDIO_EXIST,RESIDENTIAL_ADU_STUDIO_PROP,RESIDENTIAL_ADU_STUDIO_NET,RESIDENTIAL_ADU_STUDIO_AREA,RESIDENTIAL_ADU_1BR_EXIST,RESIDENTIAL_ADU_1BR_PROP,RESIDENTIAL_ADU_1BR_NET,RESIDENTIAL_ADU_1BR_AREA,RESIDENTIAL_ADU_2BR_EXIST,RESIDENTIAL_ADU_2BR_PROP,RESIDENTIAL_ADU_2BR_NET,RESIDENTIAL_ADU_2BR_AREA,RESIDENTIAL_ADU_3BR_EXIST,RESIDENTIAL_ADU_3BR_PROP,RESIDENTIAL_ADU_3BR_NET,RESIDENTIAL_ADU_3BR_AREA,RESIDENTIAL_GH_ROOMS_EXIST,RESIDENTIAL_GH_ROOMS_PROP,RESIDENTIAL_GH_ROOMS_NET,RESIDENTIAL_GH_BEDS_EXIST,RESIDENTIAL_GH_BEDS_PROP,RESIDENTIAL_GH_BEDS_NET,RESIDENTIAL_SRO_EXIST,RESIDENTIAL_SRO_PROP,RESIDENTIAL_SRO_NET,RESIDENTIAL_MICRO_EXIST,RESIDENTIAL_MICRO_PROP,RESIDENTIAL_MICRO_NET,RELATED_BUILDING_PERMIT,BOS_1ST_READ,BOS_2ND_READ,COM_HEARING,MAYORAL_SIGN,TRANSMIT_DATE_BOS,COM_HEARING_DATE_BOS,Shape_Length,Shape_Area
\"MULTIPOLYGON (((-122.456226949 37.736700243, -122.456221274 37.736446106, -122.456301489 37.736446023, -122.456389601 37.736445931, -122.456432624 37.736451404, -122.456433604 37.73672754, -122.456398805 37.736721721, -122.456363884 37.736716386, -122.45632885 37.736711535, -122.456293716 37.73670717, -122.456226949 37.736700243)))\",2,2018-015340ENV,11/08/2019 12:00:00 AM +0000,Under Review,,124 ROBINHOOD DR 94127,Environmental (ENV),ENV,Planning,Environmental,Applications,Planning/Applications/Environmental/ENV,124 ROBINHOOD DR,\"Interior remodel including kitchen and baths, small addition at rear under (E) roof and rebuild (E) deck, new bay and roof at master bedroom. Replacement of all doors & windows.\",LLYNCH,Planning,19CAP-00000-0016E,2018-015340PRJ,,400000,Laura Lynch,laura.lynch@sfgov.org,415-575-9045,https://aca.accela.com/ccsf/Cap/CapDetail.aspx?Module=Planning&TabName=Planning&capID1=19CAP&capID2=00000&capID3=0016E&agencyCode=CCSF,https://av.accela.com/portlets/cap/capsummary/CapTabSummary.do?mode=tabSummary&serviceProviderCode=CCSF&ID1=19CAP&ID2=00000&ID3=0016E&requireNotice=YES&clearForm=clearForm&module=Planning&isGeneralCAP=N,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,0.00095054097651,0.00000005554121
\"MULTIPOLYGON (((-122.43713213 37.745567486, -122.437162248 37.745879656, -122.43711879 37.745882227, -122.437076022 37.745884757, -122.437045904 37.745572586, -122.43713213 37.745567486)))\",1,2018-015993ENV,11/27/2019 12:00:00 AM +0000,Under Review,,762 DUNCAN ST 94131,Environmental (ENV),ENV,Planning,Environmental,Applications,Planning/Applications/Environmental/ENV,762 DUNCAN ST,\"The purpose of the project is to add additional living space, bedrooms and bathrooms to a small 2 bedroom, 1 bath residence of 1093 s.f. to accommodate  an elderly parent winning to move in with property owners.\",JCLEEMAN,Planning,19CAP-00000-000YG,2018-015993PRJ,,725000,Jorgen Cleemann,Jorgen.Cleemann@sfgov.org,415-575-8763,https://aca.accela.com/ccsf/Cap/CapDetail.aspx?Module=Planning&TabName=Planning&capID1=19CAP&capID2=00000&capID3=000YG&agencyCode=CCSF,https://av.accela.com/portlets/cap/capsummary/CapTabSummary.do?mode=tabSummary&serviceProviderCode=CCSF&ID1=19CAP&ID2=00000&ID3=000YG&requireNotice=YES&clearForm=clearForm&module=Planning&isGeneralCAP=N,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,0.000799993478698,0.000000027070834";
    #[test]
    fn test_parse_record() -> Result<(), csv::Error> {
        let mut rdr = Reader::from_reader(TEST_LINES.as_bytes());
        for line in rdr.deserialize::<PPTSRecord>() {
            line?;
        }
        Ok(())
    }
}
