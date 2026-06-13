// PURPOSE: ICodeMetricAnalyzerProtocol — protocol for complexity, duplication, and trend queries.
use std::process::ExitCode;

pub trait ICodeMetricAnalyzerProtocol: Send + Sync {
    fn handle_complexity(&self, path: Option<String>) -> ExitCode;
    fn handle_duplicates(&self, path: Option<String>) -> ExitCode;
    fn handle_trends(&self, path: Option<String>) -> ExitCode;
}
