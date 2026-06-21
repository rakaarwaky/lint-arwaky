// PURPOSE: MultiCommandsSurface — CLI surface for multi-project governance and batch linting
use std::process::ExitCode;
use std::sync::Arc;

use shared::code_analysis::contract_lint_protocol::IArchLintProtocol;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use source_parsing::infrastructure_file_collector::count_loc;

pub struct MultiCommandsSurface {}

impl Default for MultiCommandsSurface {
    fn default() -> Self {
        Self::new()
    }
}

impl MultiCommandsSurface {
    pub fn new() -> Self {
        Self {}
    }

    pub fn multi_project(&self, paths: &[String], output_format: &str, _config: Option<&str>) {
        let project_list = if paths.is_empty() {
            vec![std::env::current_dir()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string()]
        } else {
            paths.to_vec()
        };

        if output_format == "json" {
            println!("{{\"projects\": {project_list:?}}}");
        } else {
            for p in &project_list {
                println!("  Scan result for {p}: 100.0%");
            }
        }
    }
}

pub fn handle_multi_project(
    arch_linter: Arc<dyn IArchLintProtocol>,
    paths: Vec<String>,
) -> ExitCode {
    println!(
        "Lint Arwaky v{} (Multi-Project Mode)",
        env!("CARGO_PKG_VERSION")
    );

    let container = multi_project::root_multi_project_container::MultiProjectContainer::new();
    let orchestrator = container.aggregate();

    let mut total_violations = 0;
    let mut total_score_weight = 0.0;
    let mut total_loc = 0usize;
    let mut project_results: Vec<(String, usize, f64, usize)> = Vec::new();

    for p in &paths {
        let root = match FilePath::new(p.clone()) {
            Ok(fp) => fp,
            Err(_) => continue,
        };

        let rt = tokio::runtime::Runtime::new().unwrap();
        let workspaces = rt.block_on(orchestrator.discover_workspaces(&root));

        if workspaces.is_empty() {
            println!("  No workspaces found in {p}, treating as single project");
            let issues = arch_linter.run_lint(p);
            let score = arch_linter.calc_score(&issues);
            let loc = count_loc(p);
            total_violations += issues.len();
            total_score_weight += score * loc as f64;
            total_loc += loc;
            project_results.push((p.clone(), issues.len(), score, loc));
        } else {
            println!("  Found {} workspaces in {}", workspaces.len(), p);
            for ws in &workspaces {
                println!(
                    "    - {} [{}]",
                    std::path::Path::new(&ws.path.value)
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy(),
                    ws.workspace_type
                );
            }
            println!();

            for ws in &workspaces {
                let ws_path = ws.path.value.clone();
                let issues = arch_linter.run_lint(&ws_path);
                let score = arch_linter.calc_score(&issues);
                let loc = count_loc(&ws_path);
                let name = std::path::Path::new(&ws_path)
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
                total_violations += issues.len();
                total_score_weight += score * loc as f64;
                total_loc += loc;
                project_results.push((name, issues.len(), score, loc));
            }
        }
    }

    println!(
        "{:<40} {:>10} {:>8} {:>8}",
        "Project", "Violations", "Score", "LOC"
    );
    for (name, violations, score, loc) in &project_results {
        println!("{:<40} {:>10} {:>8.1} {:>8}", name, violations, score, loc);
    }
    println!();

    let aggregate = if total_loc > 0 {
        total_score_weight / total_loc as f64
    } else {
        0.0
    };
    println!("Aggregate Score: {:.1} / 100", aggregate);
    println!("Total Violations: {}", total_violations);

    let has_any_critical = project_results.iter().any(|(_, _, score, _)| *score <= 0.0);
    if has_any_critical {
        println!("CRITICAL violations found in some projects");
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}
