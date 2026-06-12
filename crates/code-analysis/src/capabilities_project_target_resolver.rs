// PURPOSE: capabilities_project_target_resolver — resolves project target paths, triggers codebase scans, evaluates compliance results, and counts LOC
use shared::code_analysis::contract_target_resolver_protocol::ITargetResolverProtocol;
use shared::output_report::taxonomy_result_vo::LintResult;
use shared::output_report::taxonomy_severity_vo::Severity;
use shared::output_report::taxonomy_score_vo::compute_score;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use std::path::{Path, PathBuf};

pub struct ProjectTargetResolver {}

impl ProjectTargetResolver {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for ProjectTargetResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl ITargetResolverProtocol for ProjectTargetResolver {
    fn resolve_target(&self, path: Option<String>) -> String {
        self.normalize_project_root(&path.unwrap_or_else(|| ".".to_string()))
    }

    fn walk_rs_files(&self, dir: &Path, cb: &mut dyn FnMut(PathBuf)) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for e in entries.flatten() {
                let p = e.path();
                if p.is_dir() {
                    self.walk_rs_files(&p, cb);
                } else if p.extension().map(|x| x == "rs").unwrap_or(false) {
                    cb(p);
                }
            }
        }
    }

    fn lint_path(&self, path: &str) -> Vec<LintResult> {
        let root = FilePath::new(self.normalize_project_root(path))
            .unwrap_or_else(|_| FilePath::new(".").unwrap_or_default());
        let orchestrator = crate::agent_codebase_scan_orchestrator::CodebaseScanOrchestrator::new();
        orchestrator.run_self_lint(&root.value)
    }

    fn compute_score(&self, results: &[LintResult]) -> f64 {
        compute_score(results)
    }
}

impl ProjectTargetResolver {
    pub fn normalize_project_root(&self, path: &str) -> String {
        let p = std::path::Path::new(path);
        if p.join("src-rust").exists() {
            return path.to_string();
        }
        if p.file_name().map(|n| n == "src-rust").unwrap_or(false) {
            let parent = p.parent().unwrap_or(std::path::Path::new("."));
            let parent_str = parent.to_string_lossy();
            return if parent_str.is_empty() {
                ".".to_string()
            } else {
                parent_str.to_string()
            };
        }
        path.to_string()
    }

    pub fn count_loc(&self, path: &str) -> usize {
        let src = std::path::Path::new(path);
        let mut count = 0usize;
        let mut cb = |p| {
            if let Ok(c) = std::fs::read_to_string(&p) {
                count += c.lines().count();
            }
        };
        let resolver = Self::new();
        resolver.walk_rs_files(src, &mut cb);
        count.max(1)
    }

    pub fn has_critical(&self, results: &[LintResult]) -> bool {
        results.iter().any(|r| r.severity == Severity::CRITICAL)
    }
}
