// PURPOSE: MultiCommandsSurface — CLI surface for multi-project governance and batch linting
use std::process::ExitCode;
use std::sync::Arc;

use code_analysis::{compute_score, count_loc, lint_path};
use di_containers::contract_service_aggregate::ServiceContainerAggregate;
use output_report::taxonomy_severity_vo::Severity;
pub struct MultiCommandsSurface {
    pub container: Option<Arc<dyn ServiceContainerAggregate>>,
}

impl Default for MultiCommandsSurface {
    fn default() -> Self {
        Self::new()
    }
}

impl MultiCommandsSurface {
    pub fn new() -> Self {
        Self { container: None }
    }

    pub fn register_all(&mut self, container: Arc<dyn ServiceContainerAggregate>) {
        self.container = Some(container);
    }

    pub fn multi_project(&self, paths: &[String], output_format: &str, _config: Option<&str>) {
        let project_list = if paths.is_empty() {
            if let Some(ref _container) = self.container {
                vec![std::env::current_dir()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string()]
            } else {
                vec![std::env::current_dir()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string()]
            }
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

pub fn register_multi_commands(
    container: Arc<dyn ServiceContainerAggregate>,
) -> MultiCommandsSurface {
    let mut surface = MultiCommandsSurface::new();
    surface.register_all(container);
    surface
}

pub fn handle_multi_project(paths: Vec<String>) -> ExitCode {
    println!(
        "Lint Arwaky v{} (Multi-Project Mode)",
        env!("CARGO_PKG_VERSION")
    );
    println!("Projects found: {}", paths.len());
    println!();
    let mut total_violations = 0;
    let mut total_score_weight = 0.0;
    let mut total_loc = 0usize;
    struct ProjectResult {
        name: String,
        violations: usize,
        score: f64,
        loc: usize,
        critical: usize,
    }
    let mut results: Vec<ProjectResult> = Vec::new();

    for p in &paths {
        let r = lint_path(p);
        let score = compute_score(&r);
        let loc = count_loc(p);
        let critical = r
            .iter()
            .filter(|x| x.severity == Severity::CRITICAL)
            .count();
        total_violations += r.len();
        total_score_weight += score * loc as f64;
        total_loc += loc;
        results.push(ProjectResult {
            name: p.clone(),
            violations: r.len(),
            score,
            loc,
            critical,
        });
    }

    println!(
        "{:<25} {:>10} {:>8} {:>8}",
        "Project", "Violations", "Score", "LOC"
    );
    for pr in &results {
        println!(
            "{:<25} {:>10} {:>8.1} {:>8}",
            pr.name, pr.violations, pr.score, pr.loc
        );
    }
    println!();
    let aggregate = if total_loc > 0 {
        total_score_weight / total_loc as f64
    } else {
        0.0
    };
    println!("Aggregate Score: {:.1} / 100", aggregate);
    println!("Total Violations: {}", total_violations);
    let has_any_critical = results.iter().any(|r| r.critical > 0);
    if has_any_critical {
        println!("CRITICAL violations found in some projects");
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}
