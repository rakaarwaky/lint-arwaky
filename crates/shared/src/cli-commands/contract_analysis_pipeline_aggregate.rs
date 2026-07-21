// PURPOSE: IAnalysisPipelineAggregate — aggregate trait for the full analysis pipeline
//
// Defines the public API for running all linter groups in sequence and returning
// a unified ScanReport. This is what the surface layer depends on to orchestrate
// code-analysis, naming, imports, external adapters, roles, and orphan detection.
use crate::cli_commands::taxonomy_scan_report_vo::{PipelineError, ScanReport};
use crate::cli_commands::taxonomy_scan_request_vo::ScanRequest;

/// IAnalysisPipelineAggregate — aggregate port for full analysis pipeline orchestration.
///
/// Implemented by AnalysisPipelineOrchestrator (agent layer).
/// Provides a single method for running the complete lint pipeline on a target.
#[async_trait::async_trait]
pub trait IAnalysisPipelineAggregate: Send + Sync {
    /// Run the full analysis pipeline on the request target.
    async fn run(&self, request: ScanRequest) -> Result<ScanReport, PipelineError>;
}
