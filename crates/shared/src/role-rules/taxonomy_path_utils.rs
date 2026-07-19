// PURPOSE: Taxonomy path utility functions (AES401)
//
// These pure functions support taxonomy role auditing. They are extracted here
// so they can be unit-tested directly without needing to construct a
// TaxonomyRoleChecker or feed it a full SourceContentVO.

use std::path::Path;

/// Check if a file path ends with the given suffix (on the filename stem).
pub fn has_suffix(file: &str, suffix: &str) -> bool {
    let path = Path::new(file);
    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
        stem.ends_with(suffix)
    } else {
        false
    }
}
