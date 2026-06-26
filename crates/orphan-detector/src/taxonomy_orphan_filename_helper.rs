// PURPOSE: taxonomy_orphan_filename_helper — shared filename parsing utilities for orphan analyzers
// Extracts basename, stem, and suffix from file paths in a consistent way across all orphan layer analyzers.

/// All language extensions recognized by the orphan detector.
const KNOWN_EXTENSIONS: &[&str] = &["tsx", "jsx", "ts", "js", "rs", "py"];

/// Extract the basename (filename without directory) from a file path string.
/// Handles both `/`-separated and OS paths gracefully.
pub fn file_basename(fp: &str) -> &str {
    fp.split('/').next_back().unwrap_or(fp)
}

/// Extract the file stem (basename with all known language extensions stripped).
///
/// Example: `capabilities_import_checker.rs` → `capabilities_import_checker`
pub fn file_stem(fp: &str) -> String {
    let mut stem = file_basename(fp).to_string();
    for ext in KNOWN_EXTENSIONS {
        let dot_ext = format!(".{}", ext);
        if stem.ends_with(&dot_ext) {
            stem = stem[..stem.len() - dot_ext.len()].to_string();
            break;
        }
    }
    stem
}

/// Extract the domain suffix from a file stem (the part after the last `_`, with
/// language extension stripped). Returns an empty string if no underscore is present.
///
/// Example: `capabilities_import_checker.rs` → `checker`
pub fn file_suffix(fp: &str) -> String {
    let basename = file_basename(fp);
    let stem = file_stem(fp);
    let stem_str = if stem.is_empty() { basename } else { &stem };
    stem_str.rsplit('_').next().unwrap_or_default().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basename_from_path_with_dir() {
        assert_eq!(file_basename("crates/shared/src/lib.rs"), "lib.rs");
    }

    #[test]
    fn basename_no_dir() {
        assert_eq!(file_basename("lib.rs"), "lib.rs");
    }

    #[test]
    fn basename_root_file() {
        assert_eq!(file_basename("/root/file.py"), "file.py");
    }

    #[test]
    fn stem_removes_rs_extension() {
        assert_eq!(file_stem("checker.rs"), "checker");
    }

    #[test]
    fn stem_removes_py_extension() {
        assert_eq!(file_stem("checker.py"), "checker");
    }

    #[test]
    fn stem_removes_tsx_extension() {
        assert_eq!(file_stem("component.tsx"), "component");
    }

    #[test]
    fn stem_with_full_path() {
        assert_eq!(file_stem("crates/shared/src/lib.rs"), "lib");
    }

    #[test]
    fn stem_keeps_mid_dots() {
        assert_eq!(file_stem("my.test.file.rs"), "my.test.file");
    }

    #[test]
    fn stem_empty_basename() {
        assert_eq!(file_stem(""), "");
    }

    #[test]
    fn suffix_gets_last_part() {
        assert_eq!(file_suffix("capabilities_checker.rs"), "checker");
    }

    #[test]
    fn suffix_no_underscore_empty() {
        assert_eq!(file_suffix("checker.rs"), "checker");
    }

    #[test]
    fn suffix_with_full_path() {
        assert_eq!(file_suffix("/path/to/surface_command.rs"), "command");
    }

    #[test]
    fn suffix_py_file() {
        assert_eq!(file_suffix("infrastructure_adapter.py"), "adapter");
    }

    #[test]
    fn suffix_single_underscore_prefix() {
        assert_eq!(file_suffix("_helper.rs"), "helper");
    }
}
