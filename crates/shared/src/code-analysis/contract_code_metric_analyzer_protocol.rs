// PURPOSE: ICodeMetricAnalyzerProtocol — protocol for duplication detection (AES305)
use std::process::ExitCode;

pub trait ICodeMetricAnalyzerProtocol: Send + Sync {
    fn handle_duplicates(&self, path: Option<String>) -> ExitCode;
}
