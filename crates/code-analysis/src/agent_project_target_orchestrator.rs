// PURPOSE: ProjectTargetOrchestrator — resolve_target + lint_path + has_critical free functions for CLI surfaces
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::source_parsing::taxonomy_path_vo::FilePath;

/// Resolve target path: normalize "crates" → parent, keep "." as-is, etc.
pub fn resolve_target(path: Option<String>) -> String {
    let p = path.unwrap_or_else(|| ".".to_string());
    source_parsing::infrastructure_path_provider::normalize_project_root(&p)
}

/// Run a full AES self-lint on a path via CodeAnalysisOrchestrator.
pub fn lint_path(path: &str) -> Vec<LintResult> {
    let root =
        FilePath::new(source_parsing::infrastructure_path_provider::normalize_project_root(path))
            .unwrap_or_else(|_| FilePath::new(".").unwrap_or_default());
    let orchestrator = crate::agent_code_analysis_orchestrator::CodeAnalysisOrchestrator::new();
    orchestrator.run_self_lint(&root.value)
}

/// Check if any CRITICAL severity violations exist in results.
pub fn has_critical(results: &[LintResult]) -> bool {
    results.iter().any(|r| r.severity == Severity::CRITICAL)
}
