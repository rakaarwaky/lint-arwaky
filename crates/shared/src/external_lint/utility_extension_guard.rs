// PURPOSE: Shared extension guard for external-lint adapters.
// Plain free function — no protocol / dependency injection.

/// Return true if `path_str` ends with any of `extensions`.
pub fn is_scannable_file(path_str: &str, extensions: &[&str]) -> bool {
    extensions.iter().any(|ext| path_str.ends_with(ext))
}
