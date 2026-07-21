// PURPOSE: NamingUtils — shared stem/suffix extraction for naming checkers

/// Extract the file stem using the last dot (rfind), consistent across all checkers.
///
/// For multi-dot filenames like `foo.spec.rs`, this returns `foo.spec`.
/// For single-dot files like `checker.rs`, this returns `checker`.
/// For dotfiles like `.gitignore`, the entire filename is returned.
/// If there is no dot, the entire filename is returned.
pub fn get_stem(filename: &str) -> Option<&str> {
    match filename.rfind('.') {
        Some(pos) if pos > 0 => Some(&filename[..pos]),
        _ => Some(filename),
    }
}

/// Extract the suffix (word after the last underscore) from a stem.
pub fn get_suffix(stem: &str) -> Option<&str> {
    stem.rfind('_').map(|pos| &stem[pos + 1..])
}

// ─── Regression Tests for Phase 3 Fixes ──────────────────────────────────────────

#[cfg(test)]
mod phase3_regression_tests {
    use super::*;

    /// Regression test for Phase 3.4: get_stem handles dotfiles correctly.
    /// Dotfiles like `.gitignore` should return the entire filename (not empty string).
    #[test]
    fn get_stem_dotfile_returns_full_name() {
        assert_eq!(get_stem(".gitignore"), Some(".gitignore"));
        assert_eq!(get_stem(".eslintrc"), Some(".eslintrc"));
        assert_eq!(get_stem(".prettierrc"), Some(".prettierrc"));
    }

    /// Regression test: get_stem handles normal files correctly.
    #[test]
    fn get_stem_normal_file() {
        assert_eq!(get_stem("checker.rs"), Some("checker"));
        assert_eq!(get_stem("mod.rs"), Some("mod"));
        assert_eq!(get_stem("foo.spec.rs"), Some("foo.spec"));
    }

    /// Regression test: get_stem handles files with no extension.
    #[test]
    fn get_stem_no_extension() {
        assert_eq!(get_stem("Makefile"), Some("Makefile"));
        assert_eq!(get_stem("README"), Some("README"));
    }

    /// Regression test: get_stem handles empty string.
    #[test]
    fn get_stem_empty_string() {
        assert_eq!(get_stem(""), Some(""));
    }
}
