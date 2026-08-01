use serde::Serialize;

#[derive(Serialize)]
pub struct SarifResult {
    pub rule_id: String,
    pub level: String,
    pub message: SarifMessage,
    pub locations: Vec<super::taxonomy_sarif_location_vo::SarifLocation>,
}

#[derive(Serialize)]
pub struct SarifMessage {
    pub text: String,
}
