// PURPOSE: SurfaceCheckCommand — Executes 6 surface action subcommands as parallel Rust subprocesses
// and merges their outputs into a unified report.
use std::process::ExitCode;
use std::sync::Arc;
use std::time::Instant;
use tokio::process::Command;

use shared::cli_commands::taxonomy_format_vo::Format;
use shared::report_formatter::contract_report_formatter_aggregate::IReportFormatterAggregate;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;

pub struct ScanOptions {
    pub path: Option<FilePath>,
    pub report_formatter: Arc<dyn IReportFormatterAggregate>,
    pub multi_project_orchestrator: Option<Arc<dyn IConfigOrchestratorAggregate>>,
    pub filter: Option<String>,
    pub member: Option<String>,
    pub format: Format,
}

pub struct CheckCommandsSurface;

impl CheckCommandsSurface {
    pub fn new(
        _report_formatter: Arc<dyn IReportFormatterAggregate>,
        _multi_project_orchestrator: Option<Arc<dyn IConfigOrchestratorAggregate>>,
    ) -> Self {
        Self
    }

    pub fn check_orphan_single_file(&self, _file_path: &str) {}
}

pub type CheckOptions = ScanOptions;

/// scan/check = runs 6 parallel subprocesses of the 6 surface actions (orphan, naming, quality, role, import, external)
pub fn handle_scan(opts: ScanOptions) -> ExitCode {
    let root = match &opts.path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    };
    if !std::path::Path::new(&root).exists() {
        eprintln!("Error: path '{}' does not exist", root);
        return ExitCode::from(2);
    }
    let rt = match crate::surface_common_command::create_current_thread_runtime() {
        Ok(r) => r,
        Err(_) => return ExitCode::from(2),
    };
    let format = opts.format;
    rt.block_on(handle_scan_parallel_subprocesses(&root, format))
}

pub use handle_scan as handle_check;

pub async fn handle_scan_parallel_subprocesses(path: &str, format: Format) -> ExitCode {
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
    let format_str = format.to_string();
    let p_quality = Command::new(&exe_path)
        .args(["scan-quality", path, "--format", &format_str])
        .output();
    let p_role = Command::new(&exe_path)
        .args(["scan-role", path, "--format", &format_str])
        .output();
    let p_import = Command::new(&exe_path)
        .args(["scan-import", path, "--format", &format_str])
        .output();
    let p_naming = Command::new(&exe_path)
        .args(["scan-naming", path, "--format", &format_str])
        .output();
    let p_orphan = Command::new(&exe_path)
        .args(["scan-orphan", path, "--format", &format_str])
        .output();
    let p_external = Command::new(&exe_path)
        .args(["scan-external", path, "--format", &format_str])
        .output();

    let (res_quality, res_role, res_import, res_naming, res_orphan, res_external) =
        tokio::join!(p_quality, p_role, p_import, p_naming, p_orphan, p_external);

    let duration = start.elapsed();

    println!("------------------------------------------------------------");
    println!("  Reports");
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
    println!("Total Execution Time : {:?}", duration);
    println!("============================================================");

    ExitCode::SUCCESS
}

pub fn handle_default_check(
    _project_root: &str,
    _code_analysis_linter: Arc<
        dyn shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate,
    >,
) -> ExitCode {
    let rt = match crate::surface_common_command::create_current_thread_runtime() {
        Ok(r) => r,
        Err(_) => return ExitCode::from(2),
    };
    rt.block_on(handle_scan_parallel_subprocesses(_project_root, Format::Text))
}
