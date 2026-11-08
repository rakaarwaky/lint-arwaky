// PURPOSE: git-hooks constants — lintable source extensions shared by the git-hooks feature
// Used by: git-hooks (DiffChecker) to filter changed files to source code only

/// Lintable file extensions (source code only).
pub const LINTABLE_EXTENSIONS: &[&str] = &["rs", "py", "ts", "js", "jsx", "tsx"];

/// Code carried by analysis-failure results, so "analysis could not run" is
/// reported distinctly from AES rule violations in `run_git_diff_check` output.
pub const ANALYSIS_FAILURE_CODE: &str = "GIT_HOOK_ANALYSIS_FAILURE";
