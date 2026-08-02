// PURPOSE: ScanRequest VO — request payload for the analysis pipeline
use crate::cli_commands::taxonomy_format_vo::Format;

/// Target path for the scan.
pub struct ScanTarget {
    pub value: String,
}

impl ScanTarget {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

impl Default for ScanTarget {
    fn default() -> Self {
        Self {
            value: ".".to_string(),
        }
    }
}

/// Mode of analysis to run.
#[derive(Debug, Clone, Default)]
pub enum ScanMode {
    #[default]
    Check,
    Scan,
    Ci {
        threshold: u32,
    },
}

/// Request to run the full analysis pipeline.
pub struct ScanRequest {
    pub target: ScanTarget,
    pub mode: ScanMode,
    pub filter: Option<String>,
    pub member: Option<String>,
    pub format: Format,
}

impl ScanRequest {
    pub fn new(target: ScanTarget, mode: ScanMode) -> Self {
        Self {
            target,
            mode,
            filter: None,
            member: None,
            format: Format::Text,
        }
    }
}
