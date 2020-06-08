use proj::Proj;

/// projection for converting latitude and longitude (in degrees) into feet
/// Project using Azimuthal Equidistant, centered on Market and Van Ness
/// https://proj4.org/operations/projections/utm.html
pub fn default_projection() -> Proj {
    let proj = Proj::new(
    "
        +proj=pipeline
        +step +proj=unitconvert +xy_in=deg +xy_out=rad
        +step +proj=aeqd +lat_0=37.773972 +lon_0=-122.431297
        +step +proj=unitconvert +xy_in=m +xy_out=us-ft
        ",
    )
    .expect("Failed to create projection");
    proj
}
