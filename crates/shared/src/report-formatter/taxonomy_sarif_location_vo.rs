use serde::Serialize;

#[derive(Serialize)]
pub struct SarifLocation {
    pub physical_location: SarifPhysicalLocation,
}

#[derive(Serialize)]
pub struct SarifPhysicalLocation {
    pub artifact_location: SarifArtifactLocation,
    pub region: SarifRegion,
}

#[derive(Serialize)]
pub struct SarifArtifactLocation {
    pub uri: String,
}

#[derive(Serialize)]
pub struct SarifRegion {
    pub start_line: i64,
}
