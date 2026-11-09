// PURPOSE: CI command — CLI thin wrapper
// Calls dispatcher for CI business logic, only adds CLI output.
use shared_common::ExitCode;
use tracing::{error, info};

use shared_common::{FilePath, Threshold};

/// Parameters for the CI command — the same aggregate bundle `scan` uses,
/// because `ci` scores the same rule groups.
pub struct CiCommandParams {
    pub aggregates: dispatcher::surface_check_action::ScanAggregates,
    pub path: Option<FilePath>,
    pub threshold: Threshold,
}

pub fn handle_ci(params: CiCommandParams) -> ExitCode {
    match dispatcher::surface_ci_action::collect_ci(
        params.aggregates,
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
            // The gate's coverage is part of its result: a group missing from
            // this list is a group whose findings cannot fail the build.
            println!("Rule groups scored: {}", report.groups_run.join(", "));
            println!();
            if report.pass {
                println!("Result: PASS (exit code 0)");
                ExitCode::OK
            } else {
                // The failing run used to print nothing after the counters: no
                // result line, no exit code, and every reason routed to a
                // `tracing::info!` channel the default "warn" filter drops. A
                // CI engineer watching a red build could not see why it failed.
                println!("Result: FAIL (exit code {})", ExitCode::POLICY_FAIL.value());
                if !report.reasons.is_empty() {
                    println!();
                    println!("Reasons:");
                    for reason in &report.reasons {
                        println!("  - {reason}");
                        info!(result = %reason, "scan result");
                    }
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
