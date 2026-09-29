// PURPOSE: Report formatter value objects — JSON report DTOs and the SARIF 2.1.0 object model.
use serde::Serialize;

// ─── JSON report DTOs ─────────────────────────────────────────────────

#[derive(Serialize)]
pub struct JsonViolation {
    pub file: String,
    pub line: i64,
    pub code: String,
    pub severity: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct JsonDiagnostic {
    pub source: String,
    pub severity: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct JsonSummary {
    pub total_violations: usize,
    pub critical: usize,
    pub high: usize,
    pub medium: usize,
    pub low: usize,
    pub score: Option<f64>,
}

#[derive(Serialize)]
pub struct JsonReportDto {
    pub violations: Vec<JsonViolation>,
    pub external_results: Vec<JsonViolation>,
    pub diagnostics: Vec<JsonDiagnostic>,
    pub summary: JsonSummary,
}

// ─── SARIF driver and rules ───────────────────────────────────────────

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

// ─── SARIF locations ──────────────────────────────────────────────────

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

// ─── SARIF log ────────────────────────────────────────────────────────

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
    pub results: Vec<SarifResult>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<SarifRule>,
}

#[derive(Serialize)]
pub struct SarifTool {
    pub driver: SarifDriver,
}

// ─── SARIF results ────────────────────────────────────────────────────

#[derive(Serialize)]
pub struct SarifResult {
    pub rule_id: String,
    pub level: String,
    pub message: SarifMessage,
    pub locations: Vec<SarifLocation>,
}

#[derive(Serialize)]
pub struct SarifMessage {
    pub text: String,
}
