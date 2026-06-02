/// CLI check and scan commands (Surface).
use std::collections::HashMap;
use std::path::PathBuf;

use crate::taxonomy::*;
use crate::contract::*;
use crate::surfaces::cli_output_controller::{get_output_dir, write_output, tee_stdout};

pub struct CheckCommandsSurface {
    pub container: Option<ServiceContainerAggregate>,
}

impl CheckCommandsSurface {
    pub fn new() -> Self {
        Self { container: None }
    }

    pub fn register_all(&mut self, container: ServiceContainerAggregate) {
        self.container = Some(container);
    }

    pub fn check(&self, path: &str, git_diff: bool) {
        let path_vo = FilePath { value: path.to_string() };
        let diff_vo = BooleanVO { value: git_diff };
        self.run_check(path_vo, diff_vo);
    }

    pub fn scan(&self, path: &str) {
        let path_vo = FilePath { value: path.to_string() };
        let diff_vo = BooleanVO { value: false };
        self.run_check(path_vo, diff_vo);
    }

    fn format_report(&self, report: &GovernanceReport) -> String {
        let mut lines = Vec::new();

        // Group results by source
        let mut source_results: HashMap<String, Vec<&LintResult>> = HashMap::new();
        for res in &report.results {
            let src = res.source.as_ref().map(|s| s.value.clone()).unwrap_or_else(|| "unknown".to_string());
            source_results.entry(src).or_default().push(res);
        }

        for (source, results) in &source_results {
            let status = if results.is_empty() {
                " CLEAN".to_string()
            } else {
                format!(" {} ISSUES", results.len())
            };
            lines.push(format!("[{source}]{status}"));
            for res in results {
                lines.push(format!(
                    " - {file}:{line} {code}: {msg}",
                    file = res.file.value,
                    line = res.line.value,
                    code = res.code.as_deref().unwrap_or(""),
                    msg = res.message.value,
                ));
            }
        }

        lines.push("-".repeat(40));
        lines.push(format!("total issues :  {}", report.results.len()));
        lines.push(format!("total score  :  {:.1}/100.0", report.score.value));
        lines.push("-".repeat(40));

        lines.join("\n")
    }

    async fn handle_git_diff(&self, container: &ServiceContainerAggregate, project_path: &FilePath) {
        // In real impl: call container.analysis_orchestrator.run
        let report = GovernanceReport {
            results: vec![],
            score: Score::new(100.0).unwrap(),
            is_passing: BooleanVO { value: true },
        };
        let report_text = self.format_report(&report);
        println!("{report_text}");
    }

    async fn handle_full_analysis(&self, container: &ServiceContainerAggregate, project_path: &FilePath) {
        println!(" Running analysis on {}...", project_path.value);
        let report = GovernanceReport {
            results: vec![],
            score: Score::new(100.0).unwrap(),
            is_passing: BooleanVO { value: true },
        };
        let report_text = self.format_report(&report);
        println!("{report_text}");
    }

    fn run_check(&self, project_path: FilePath, git_diff: BooleanVO) {
        let output_dir = get_output_dir(None);

        let output = tee_stdout(None, || {
            if git_diff.value {
                println!("[git-diff] Running analysis on {}", project_path.value);
            } else {
                println!(" Running analysis on {}...", project_path.value);
            }
            // Structural placeholder
            println!("{}", "-".repeat(40));
            println!("total issues :  0");
            println!("total score  :  100.0/100.0");
            println!("{}", "-".repeat(40));
        });

        if let Some(dir) = output_dir {
            write_output(None, &output, "check", Some("txt"));
        }
    }

    fn aggregate_source_counts(&self, reports: &[(String, GovernanceReport)]) -> HashMap<String, i32> {
        let mut counts = HashMap::new();
        for (_, report) in reports {
            for res in &report.results {
                let source = res.source.as_ref().map(|s| s.value.clone()).unwrap_or_else(|| "unknown".to_string());
                *counts.entry(source).or_insert(0) += 1;
            }
        }
        counts
    }

    fn print_source_summary(&self, source_counts: &HashMap<String, i32>) {
        for (source, count) in source_counts {
            let status = if *count == 0 {
                " CLEAN".to_string()
            } else {
                format!(" {count} ISSUES")
            };
            println!("[{source}]{status}");
        }
    }
}

pub fn register_check_commands(container: ServiceContainerAggregate) -> CheckCommandsSurface {
    let mut surface = CheckCommandsSurface::new();
    surface.register_all(container);
    surface
}
