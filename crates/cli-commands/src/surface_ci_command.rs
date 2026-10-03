// PURPOSE: CI command — CLI thin wrapper
// Calls shared_structure_rules::surface_ci_action for CI business logic, only adds CLI output.
use shared_common::ExitCode;
use std::sync::Arc;
use tracing::{error, info};

use shared_common::{FilePath, Threshold};
use shared_config_system::IConfigOrchestratorAggregate;
use shared_filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared_filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use shared_import_rules::IImportRunnerAggregate;
use shared_naming_rules::INamingRunnerAggregate;
use shared_orphan_rules::IOrphanAggregate;
use shared_quality_rules::ICodeAnalysisAggregate;

/// Parameters for the CI command — groups all linter aggregates + CLI args.
pub struct CiCommandParams {
    pub code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    pub import_orchestrator: Arc<dyn IImportRunnerAggregate>,
    pub naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
    pub config_orchestrator: Arc<dyn IConfigOrchestratorAggregate>,
    pub orphan_orchestrator: Arc<dyn IOrphanAggregate>,
    pub filesystem: Arc<dyn IFilesystemAggregate>,
    pub filesystem_io: Arc<dyn IFileSystemIOProtocol>,
    pub path: Option<FilePath>,
    pub threshold: Threshold,
}

pub fn handle_ci(params: CiCommandParams) -> ExitCode {
    match shared_structure_rules::surface_ci_action::collect_ci(
        shared_structure_rules::surface_ci_action::CiScanDeps {
            code_analysis_linter: params.code_analysis_linter,
            import_orchestrator: params.import_orchestrator,
            naming_orchestrator: params.naming_orchestrator,
            config_orchestrator: params.config_orchestrator,
            orphan_orchestrator: params.orphan_orchestrator,
            filesystem: params.filesystem,
            filesystem_io: params.filesystem_io,
        },
        params.path,
        params.threshold,
    ) {
        Ok(report) => {
            println!(
                "Lint Arwaky v{} — CI Architecture Compliance",
                report.version
            );
            println!("Score: {:.1} / 100", report.score);
            println!("Threshold: {}", report.threshold);
            println!();
            println!(
                "CRITICAL: {} | HIGH: {} | MEDIUM: {} | LOW: {}",
                report.critical, report.high, report.medium, report.low
            );
            println!();
            if report.pass {
                println!("Result: PASS (exit code 0)");
                ExitCode::OK
            } else {
                for r in &report.reasons {
                    info!(result = %r, "scan result");
                }
                info!("scan result: FAIL");
                ExitCode::POLICY_FAIL
            }
        }
        Err(e) => {
            error!(error = %e, "operation failed");
            ExitCode::RUNTIME_ERROR
        }
    }
}
