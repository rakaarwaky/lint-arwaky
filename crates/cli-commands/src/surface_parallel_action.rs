// PURPOSE: SurfaceParallelAction — Runs 6 linter subcommands as parallel Rust subprocesses
// and merges their outputs into a single report with benchmark timing.
use std::process::ExitCode;
use std::time::Instant;
use tokio::process::Command;

pub async fn handle_scan_parallel_subprocesses(path: &str) -> ExitCode {
    println!("============================================================");
    println!("  Parallel Subprocess Action (6 Subprocesses in Rust)");
    println!("============================================================");
    println!("Target Path: {path}");
    println!("Spawning 6 parallel Rust subprocesses via tokio::process::Command...");
    println!();

    let exe_path = match std::env::current_exe() {
        Ok(p) => p,
        Err(_) => std::path::PathBuf::from("lint-arwaky-cli"),
    };

    let start = Instant::now();

    // Spawn 6 subprocesses concurrently in Rust
    let p_quality = Command::new(&exe_path)
        .args(["scan-quality", path])
        .output();
    let p_role = Command::new(&exe_path).args(["scan-role", path]).output();
    let p_import = Command::new(&exe_path).args(["scan-import", path]).output();
    let p_naming = Command::new(&exe_path).args(["scan-naming", path]).output();
    let p_orphan = Command::new(&exe_path).args(["orphan", path]).output();
    let p_external = Command::new(&exe_path)
        .args(["scan-external", path])
        .output();

    let (res_quality, res_role, res_import, res_naming, res_orphan, res_external) =
        tokio::join!(p_quality, p_role, p_import, p_naming, p_orphan, p_external);

    let duration = start.elapsed();

    println!("------------------------------------------------------------");
    println!("  Merged Output Report (6 Subprocesses)");
    println!("------------------------------------------------------------");

    println!("--- [1. Quality Scan Output] ---");
    if let Ok(out) = res_quality {
        print!("{}", String::from_utf8_lossy(&out.stdout));
    }

    println!("--- [2. Role Scan Output] ---");
    if let Ok(out) = res_role {
        print!("{}", String::from_utf8_lossy(&out.stdout));
    }

    println!("--- [3. Import Scan Output] ---");
    if let Ok(out) = res_import {
        print!("{}", String::from_utf8_lossy(&out.stdout));
    }

    println!("--- [4. Naming Scan Output] ---");
    if let Ok(out) = res_naming {
        print!("{}", String::from_utf8_lossy(&out.stdout));
    }

    println!("--- [5. Orphan Scan Output] ---");
    if let Ok(out) = res_orphan {
        print!("{}", String::from_utf8_lossy(&out.stdout));
    }

    println!("--- [6. External Scan Output] ---");
    if let Ok(out) = res_external {
        print!("{}", String::from_utf8_lossy(&out.stdout));
    }

    println!("============================================================");
    println!("Total Execution Time (6 Rust Subprocesses): {:?}", duration);
    println!("============================================================");

    ExitCode::SUCCESS
}

pub struct ParallelPipelineAction;

#[async_trait::async_trait]
impl shared::cli_commands::contract_analysis_pipeline_aggregate::IAnalysisPipelineAggregate
    for ParallelPipelineAction
{
    async fn run(
        &self,
        _request: shared::cli_commands::taxonomy_scan_request_vo::ScanRequest,
    ) -> Result<
        shared::cli_commands::taxonomy_scan_report_vo::ScanReport,
        shared::cli_commands::taxonomy_scan_report_vo::PipelineError,
    > {
        Ok(shared::cli_commands::taxonomy_scan_report_vo::ScanReport::new(vec![], vec![]))
    }

    async fn run_with_discovery(
        &self,
    ) -> Result<
        shared::cli_commands::taxonomy_scan_report_vo::ScanReport,
        shared::cli_commands::taxonomy_scan_report_vo::PipelineError,
    > {
        Ok(shared::cli_commands::taxonomy_scan_report_vo::ScanReport::new(vec![], vec![]))
    }

    fn check_orphan_single_file(
        &self,
        _file_path: &str,
        _workspace_root: &str,
    ) -> Result<
        Vec<shared::cli_commands::taxonomy_result_vo::LintResult>,
        shared::cli_commands::taxonomy_scan_report_vo::PipelineError,
    > {
        Ok(vec![])
    }
}
