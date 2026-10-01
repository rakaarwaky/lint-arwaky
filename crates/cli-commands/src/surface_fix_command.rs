// PURPOSE: Fix command — CLI thin wrapper
// Calls dispatcher for fix business logic, only adds CLI output.
use shared_auto_fix::IFixAggregate;
use shared_common::{ExitCode, FilePath};
use shared_quality_rules::ICodeAnalysisAggregate;
use std::sync::Arc;
use tracing::{error, info};

use crate::utility_output_text_formatter::format_location;

pub fn handle_fix(
    path: Option<FilePath>,
    dry_run: bool,
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    fix_orchestrator_factory: Arc<dyn Fn(bool) -> Arc<dyn IFixAggregate> + Send + Sync>,
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
                    let loc = format_location(r.file.value(), r.line.value(), r.column.value());
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
                if report.has_failed {
                    error!("one or more fix previews failed with a runtime error");
                    ExitCode::RUNTIME_ERROR
                } else {
                    ExitCode::OK
                }
            } else {
                println!(
                    "Fixed {} violations ({} remaining)",
                    report.fixed_count, report.after_count
                );
                if report.has_failed {
                    // Any per-item Failed(reason) is a runtime error, per the
                    // PRD Exit Code Contract `fix` aggregation rule. It must
                    // outrank the policy-fail branch below.
                    error!("one or more fix attempts failed with a runtime error");
                    ExitCode::RUNTIME_ERROR
                } else if report.success {
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
