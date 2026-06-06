use std::path::Path;

use crate::capabilities::{
    collect_source_files, format_report, load_config, ArchComplianceAnalyzer,
};
use crate::taxonomy::{LintResult, LintResultList};

pub fn detect_source_dir(project_root: &Path) -> std::path::PathBuf {
    for name in &["src-rust", "src-python", "src-javascript", "src"] {
        let candidate = project_root.join(name);
        if candidate.is_dir() {
            return candidate;
        }
    }
    project_root.join("src-rust")
}

pub struct ArchitectureLintOrchestrator;

impl ArchitectureLintOrchestrator {
    pub fn new() -> Self {
        Self
    }

    pub fn run_self_lint(&self, project_root: &str) -> Vec<LintResult> {
        let root = Path::new(project_root);
        let src_dir = detect_source_dir(root);
        self.run_lint_at(&src_dir, Some(root))
    }

    pub fn run_self_lint_dir(&self, src_dir: &str) -> Vec<LintResult> {
        self.run_lint_at(Path::new(src_dir), None)
    }

    fn run_lint_at(&self, src_dir: &Path, project_root: Option<&Path>) -> Vec<LintResult> {
        let config = load_config(project_root, src_dir);
        let files = collect_source_files(src_dir);
        if files.is_empty() {
            return Vec::new();
        }
        let root_dir = src_dir.to_string_lossy().to_string();
        let coordinator = crate::agent::checking_coordinator::CheckingCoordinator::new();
        coordinator.run_all_checks(&config, &files, &root_dir)
    }

    pub fn format_report(&self, results: &[LintResult], project_root: &str) -> String {
        format_report(results, project_root)
    }
}

pub struct ArchLintPipelineOrchestrator {
    inner: ArchitectureLintOrchestrator,
}

impl ArchLintPipelineOrchestrator {
    pub fn new() -> Self {
        Self {
            inner: ArchitectureLintOrchestrator,
        }
    }

    pub fn execute_pipeline(&self, project_root: &str) -> LintResultList {
        let results = self.inner.run_self_lint(project_root);
        LintResultList::new(results)
    }

    pub fn execute_pipeline_dir(&self, src_dir: &str) -> LintResultList {
        let results = self.inner.run_self_lint_dir(src_dir);
        LintResultList::new(results)
    }
}
