// PURPOSE: ImportHelperPort — trait for import-specific helper utilities used by capabilities and infrastructure

use crate::common::taxonomy_path_vo::FilePath;

/// Import helper port — provides filepath and string manipulation utilities shared across
/// import-rules capabilities and infrastructure. Implements layer prefix detection,
/// filepath resolution, and safe string fallbacks.
pub trait IImportHelperPort: Send + Sync {
    /// Returns `fp` if `result` is `Ok`, otherwise returns `FilePath::default()`.
    fn filepath_or_default<T>(&self, result: Result<FilePath, T>) -> FilePath;

    /// Returns `s` if `opt` is `Some`, otherwise returns `fallback`.
    fn str_or<'a>(&self, opt: Option<&'a str>, fallback: &'a str) -> &'a str;

    /// Returns `s` if `opt` is `Some`, otherwise returns empty string.
    fn str_or_empty(&self, opt: Option<&str>) -> &str;

    /// Canonical layer prefixes for filename-to-layer mapping.
    fn layer_prefixes(&self) -> &'static [&'static str];

    /// Canonical layer names matching layer_prefixes (without trailing underscore).
    fn layer_names(&self) -> &'static [&'static str];
}
