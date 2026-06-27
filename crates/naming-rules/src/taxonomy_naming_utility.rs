// PURPOSE: NamingUtils — shared stem/suffix extraction for naming checkers

/// Extract the file stem using the last dot (rfind), consistent across all checkers.
///
/// For multi-dot filenames like `foo.spec.rs`, this returns `foo.spec`.
/// For single-dot files like `checker.rs`, this returns `checker`.
/// If there is no dot, the entire filename is returned.
pub fn get_stem(filename: &str) -> Option<String> {
    if let Some(pos) = filename.rfind('.') {
        Some(filename[..pos].to_string())
    } else {
        Some(filename.to_string())
    }
}

/// Extract the suffix (word after the last underscore) from a stem.
pub fn get_suffix(stem: &str) -> Option<String> {
    stem.rfind('_').map(|pos| stem[pos + 1..].to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_stem_single_extension() {
        assert_eq!(get_stem("checker.rs"), Some("checker".to_string()));
    }

    #[test]
    fn get_stem_no_extension() {
        assert_eq!(get_stem("checker"), Some("checker".to_string()));
    }

    #[test]
    fn get_stem_multi_dot() {
        assert_eq!(get_stem("foo.spec.rs"), Some("foo.spec".to_string()));
    }

    #[test]
    fn get_stem_deeply_nested() {
        assert_eq!(get_stem("a.b.c.d"), Some("a.b.c".to_string()));
    }

    #[test]
    fn get_suffix_normal() {
        assert_eq!(
            get_suffix("capabilities_checker"),
            Some("checker".to_string())
        );
    }

    #[test]
    fn get_suffix_no_underscore() {
        assert_eq!(get_suffix("checker"), None);
    }

    #[test]
    fn get_suffix_multi_dot_stem() {
        assert_eq!(get_suffix("foo.spec_checker"), Some("checker".to_string()));
    }
}
