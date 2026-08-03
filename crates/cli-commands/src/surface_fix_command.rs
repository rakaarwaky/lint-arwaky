// PURPOSE: Fix command — CLI thin wrapper
// Calls dispatcher for fix business logic, only adds CLI output.
use shared::auto_fix::LintFixOrchestratorAggregate;
use shared::common::{ExitCode, FilePath};
use shared::quality_rules::ICodeAnalysisAggregate;
use std::sync::Arc;
use tracing::{error, info};

pub fn handle_fix(
    path: Option<FilePath>,
    dry_run: bool,
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    fix_orchestrator_factory: Arc<
        dyn Fn(bool) -> Arc<dyn LintFixOrchestratorAggregate> + Send + Sync,
    >,
) -> ExitCode {
    match dispatcher::surface_fix_action::collect_fix(
        path,
        dry_run,
        code_analysis_linter,
        fix_orchestrator_factory,
    ) {
        Ok(report) => {
            if report.dry_run {
                println!("[DRY-RUN] Previewing fixes for {}...", report.project_path);
                for r in &report.fixable {
                    let loc = match (r.line.value(), r.column.value()) {
                        (l, c) if l > 0 && c > 0 => {
                            format!("{}:{}:{}", r.file.value(), l, c)
                        }
                        (l, _) if l > 0 => format!("{}:{}", r.file.value(), l),
                        _ => r.file.value().to_string(),
                    };
                    println!(
                        "  [fixable] {} [{}] {}",
                        loc,
                        r.code.code(),
                        r.message.value()
                    );
                }
            } else {
                println!("Applying safe fixes to {}...", report.project_path);
            }

            println!(
                "Found {} violations before fix (AES301-305 only)",
                report.before_count
            );
            println!("{}", report.output);

            if report.dry_run {
                println!("Dry-run complete — no changes applied.");
                ExitCode::OK
            } else {
                println!(
                    "Fixed {} violations ({} remaining)",
                    report.fixed_count, report.after_count
                );
                if report.success {
                    println!("Fix complete — all violations resolved.");
                    ExitCode::OK
                } else {
                    info!("fix complete — {} violations remain.", report.after_count);
                    ExitCode::POLICY_FAIL
                }
            }
        }
        Err(e) => {
            error!(error = %e, "operation failed");
            ExitCode::RUNTIME_ERROR
        }
    }
}
