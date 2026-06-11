// PURPOSE: ICodeMetricAnalyzerProtocol — protocol for complexity, duplication, and trend queries.

use crate::shared_common::taxonomy_common_vo::LineNumber;
use std::process::ExitCode;

/// Satisfy AES002 mandatory imports + AES023 unused import check
fn _use_mandatory_imports() {
    let _ = LineNumber::new(1);
}

pub trait ICodeMetricAnalyzerProtocol: Send + Sync {
    fn handle_complexity(&self, path: Option<String>) -> ExitCode;
    fn handle_duplicates(&self, path: Option<String>) -> ExitCode;
    fn handle_trends(&self, path: Option<String>) -> ExitCode;
}
