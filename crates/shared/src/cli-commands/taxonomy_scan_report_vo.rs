// PURPOSE: ScanReport VO — output of the analysis pipeline
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_common_vo::Score;

/// Severity level for pipeline diagnostics.
#[derive(Debug, Clone)]
pub enum DiagnosticSeverity {
    Info,
    Warning,
    Error,
}

/// A diagnostic message from a pipeline subsystem.
pub struct PipelineDiagnostic {
    pub source: String,
    pub message: String,
    pub severity: DiagnosticSeverity,
}

impl PipelineDiagnostic {
    pub fn new(source: String, message: String, severity: DiagnosticSeverity) -> Self {
        Self {
            source,
            message,
            severity,
        }
    }
}

/// Error types that can occur during pipeline execution.
#[derive(Debug, Clone)]
pub enum PipelineError {
    PathNotFound(String),
    InvalidPath(String),
    WorkspaceDiscovery(String),
    Analysis(String),
    Io(String),
}

impl std::fmt::Display for PipelineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PipelineError::PathNotFound(p) => write!(f, "path not found: {p}"),
            PipelineError::InvalidPath(p) => write!(f, "invalid path: {p}"),
            PipelineError::WorkspaceDiscovery(e) => write!(f, "workspace discovery failed: {e}"),
            PipelineError::Analysis(e) => write!(f, "analysis failed: {e}"),
            PipelineError::Io(e) => write!(f, "io error: {e}"),
        }
    }
}

impl std::error::Error for PipelineError {}

/// Results of the full analysis pipeline.
pub struct ScanReport {
    pub results: Vec<LintResult>,
    pub diagnostics: Vec<PipelineDiagnostic>,
    pub score: Option<Score>,
}

impl ScanReport {
    pub fn new(results: Vec<LintResult>, diagnostics: Vec<PipelineDiagnostic>) -> Self {
        Self {
            results,
            diagnostics,
            score: None,
        }
    }

    /// Return the number of violations (results with severity > INFO).
    pub fn violation_count(&self) -> usize {
        self.results
            .iter()
            .filter(|r| r.severity != crate::cli_commands::taxonomy_severity_vo::Severity::INFO)
            .count()
    }

    /// Attach a score to the report.
    pub fn with_score(mut self, score: Score) -> Self {
        self.score = Some(score);
        self
    }
}
