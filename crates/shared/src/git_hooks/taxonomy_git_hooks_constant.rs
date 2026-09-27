// PURPOSE: git-hooks constants — lintable source extensions shared by the git-hooks feature
// Used by: git-hooks (DiffChecker) to filter changed files to source code only

/// Lintable file extensions (source code only).
pub const LINTABLE_EXTENSIONS: &[&str] = &["rs", "py", "ts", "js", "jsx", "tsx"];
