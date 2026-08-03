// PURPOSE: CI command — CLI thin wrapper
// Calls dispatcher for CI business logic, only adds CLI output.
use shared::common::ExitCode;
use std::sync::Arc;
use tracing::{error, info};

use shared::common::{FilePath, Threshold};
use shared::config_system::IConfigOrchestratorAggregate;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::import_rules::IImportRunnerAggregate;
use shared::naming_rules::INamingRunnerAggregate;
use shared::orphan_rules::IOrphanAggregate;
use shared::quality_rules::ICodeAnalysisAggregate;

#[allow(clippy::too_many_arguments)]
pub fn handle_ci(
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    import_orchestrator: Arc<dyn IImportRunnerAggregate>,
    naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
    config_orchestrator: Arc<dyn IConfigOrchestratorAggregate>,
    orphan_orchestrator: Arc<dyn IOrphanAggregate>,
    filesystem: Arc<dyn IFilesystemAggregate>,
    path: Option<FilePath>,
    threshold: Threshold,
) -> ExitCode {
    match dispatcher::surface_ci_action::collect_ci(
        dispatcher::surface_ci_action::CiScanDeps {
            code_analysis_linter,
            import_orchestrator,
            naming_orchestrator,
            config_orchestrator,
            orphan_orchestrator,
            filesystem,
        },
        path,
        threshold,
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
