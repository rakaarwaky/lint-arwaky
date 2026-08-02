use serde::Serialize;

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
