// PURPOSE: Fix command — CLI thin wrapper
// Calls dispatcher for fix business logic, only adds CLI output.
use shared_auto_fix::IFixAggregate;
use shared_common::{ExitCode, FilePath};
use shared_quality_rules::ICodeAnalysisAggregate;
use std::sync::Arc;
use tracing::{error, info};

use shared_cli_commands::utility_output_text_formatter::format_location;

pub fn handle_fix(
    path: Option<FilePath>,
    dry_run: bool,
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    filesystem: Arc<dyn shared_filesystem::contract_filesystem_aggregate::IFilesystemAggregate>,
    fix_orchestrator_factory: Arc<dyn Fn(bool) -> Arc<dyn IFixAggregate> + Send + Sync>,
    ignored: &[String],
) -> ExitCode {
    match dispatcher::surface_fix_action::collect_fix(
        path,
        dry_run,
        code_analysis_linter,
        filesystem,
        fix_orchestrator_factory,
        ignored,
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
            } else {
                println!(
                    "Fixed {} violations ({} remaining)",
                    report.fixed_count, report.after_count
                );
                if !report.resolved.is_empty() {
                    println!("Resolved:");
                    for r in &report.resolved {
                        println!("  [{}] {}", r.code.code(), r.message.value());
                    }
                }
            }

            // A fix that introduced a finding made the tree worse. Report it
            // explicitly: the count delta alone looked like a no-op.
            if !report.introduced.is_empty() {
                println!();
                println!("Introduced by this run (regression):");
                for r in &report.introduced {
                    let loc = format_location(r.file.value(), r.line.value(), r.column.value());
                    println!("  {} [{}] {}", loc, r.code.code(), r.message.value());
                }
            }

            let verdict = report.verdict();
            if report.has_failed {
                error!("one or more fix attempts failed with a runtime error");
            } else if verdict == ExitCode::OK && !report.dry_run {
                println!("Fix complete — all violations resolved.");
            } else if verdict == ExitCode::POLICY_FAIL {
                info!(
                    "fix complete — {} fixable violation(s) remain.",
                    report.remaining_fixable
                );
            }
            verdict
        }
        Err(e) => {
            error!(error = %e, "operation failed");
            ExitCode::RUNTIME_ERROR
        }
    }
}

pub struct _Aes303Placeholder {
    _placeholder: (),
}
