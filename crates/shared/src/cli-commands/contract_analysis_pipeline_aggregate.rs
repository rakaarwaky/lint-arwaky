// PURPOSE: IAnalysisPipelineAggregate — aggregate trait for the full analysis pipeline
//
// Defines the public API for running all linter groups in sequence and returning
// a unified ScanReport. This is what the surface layer depends on to orchestrate
// code-analysis, naming, imports, external adapters, roles, and orphan detection.
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::cli_commands::taxonomy_scan_report_vo::{PipelineError, ScanReport};
use crate::cli_commands::taxonomy_scan_request_vo::ScanRequest;

/// IAnalysisPipelineAggregate — aggregate port for full analysis pipeline orchestration.
///
/// Implemented by AnalysisPipelineOrchestrator (agent layer).
/// Provides methods for running the complete lint pipeline on a target,
/// with multi-workspace discovery support and single-file orphan checking.
#[async_trait::async_trait]
pub trait IAnalysisPipelineAggregate: Send + Sync {
    /// Run the full analysis pipeline on the request target.
    async fn run(&self, request: ScanRequest) -> Result<ScanReport, PipelineError>;

    /// Run the full analysis pipeline with multi-workspace discovery.
    ///
    /// Discovers workspace members (Cargo.toml, pyproject.toml, package.json workspaces),
    /// runs all 6 linter groups per member, runs cross-workspace orphan detection,
    /// filters results to each member's path, and aggregates into a single ScanReport.
    async fn run_with_discovery(&self) -> Result<ScanReport, PipelineError>;

    /// Check if a single file is an orphan.
    ///
    /// Scans ALL source files to build the import graph for reachability analysis,
    /// then filters results to only the specified file path.
    fn check_orphan_single_file(
        &self,
        file_path: &str,
        workspace_root: &str,
    ) -> Result<Vec<LintResult>, PipelineError>;
}
