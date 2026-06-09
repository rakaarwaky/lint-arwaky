use crate::output_report::taxonomy_result_vo::LintResult;
use crate::output_report::taxonomy_severity_vo::Severity;
use crate::source_parsing::taxonomy_path_vo::FilePath;

pub fn resolve_target(path: Option<String>) -> String {
    normalize_project_root(&path.unwrap_or_else(|| ".".to_string()))
}

pub fn normalize_project_root(path: &str) -> String {
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

pub fn lint_path(path: &str) -> Vec<LintResult> {
    let root = FilePath::new(normalize_project_root(path))
        .unwrap_or_else(|_| FilePath::new(".").unwrap_or_default());
    let orchestrator =
        crate::code_analysis::agent_lint_orchestrator::ArchitectureLintOrchestrator::new();
    orchestrator.run_self_lint(&root)
}

pub fn has_critical(results: &[LintResult]) -> bool {
    results.iter().any(|r| r.severity == Severity::CRITICAL)
}

pub fn compute_score(results: &[LintResult]) -> f64 {
    let penalty: f64 = results.iter().map(|r| r.severity.score_impact()).sum();
    (100.0 - penalty).max(0.0)
}

pub fn count_loc(path: &str) -> usize {
    let src = std::path::Path::new(path);
    let mut count = 0usize;
    walk_rs_files(src, &mut |p| {
        if let Ok(c) = std::fs::read_to_string(&p) {
            count += c.lines().count();
        }
    });
    count.max(1)
}

pub fn walk_rs_files(dir: &std::path::Path, cb: &mut impl FnMut(std::path::PathBuf)) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for e in entries.flatten() {
            let p = e.path();
            if p.is_dir() {
                walk_rs_files(&p, cb);
            } else if p.extension().map(|x| x == "rs").unwrap_or(false) {
                cb(p);
            }
        }
    }
}
