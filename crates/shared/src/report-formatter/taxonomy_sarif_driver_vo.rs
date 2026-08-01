use serde::Serialize;

#[derive(Serialize)]
pub struct SarifDriver {
    pub name: &'static str,
    pub version: &'static str,
    pub information_uri: &'static str,
}

#[derive(Serialize)]
pub struct SarifRule {
    pub id: String,
    #[serde(rename = "defaultConfiguration")]
    pub default_configuration_level: String,
}
