use serde::Serialize;

#[derive(Serialize)]
pub struct SarifLog {
    #[serde(rename = "$schema")]
    pub schema: &'static str,
    pub version: &'static str,
    pub runs: Vec<SarifRun>,
}

#[derive(Serialize)]
pub struct SarifRun {
    pub tool: SarifTool,
    pub results: Vec<super::taxonomy_sarif_result_vo::SarifResult>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<super::taxonomy_sarif_driver_vo::SarifRule>,
}

#[derive(Serialize)]
pub struct SarifTool {
    pub driver: super::taxonomy_sarif_driver_vo::SarifDriver,
}
