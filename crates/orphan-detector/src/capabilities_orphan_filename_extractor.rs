// PURPOSE: capabilities_orphan_filename_extractor — shared filename parsing utilities for orphan analyzers
// Extracts basename, stem, and suffix from file paths in a consistent way across all orphan layer analyzers.

use shared::common::taxonomy_layer_vo::Identity;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_protocol::IOrphanFilenameExtractorProtocol;

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
/// language extension stripped). Returns the full stem if no underscore is present.
///
/// Example: `capabilities_import_checker.rs` → `checker`
pub fn file_suffix(fp: &str) -> String {
    let basename = file_basename(fp);
    let stem = file_stem(fp);
    let stem_str = if stem.is_empty() { basename } else { &stem };
    stem_str.rsplit('_').next().unwrap_or_default().to_string()
}

pub struct OrphanFilenameExtractor;

impl Default for OrphanFilenameExtractor {
    fn default() -> Self {
        Self::new()
    }
}

impl OrphanFilenameExtractor {
    pub fn new() -> Self {
        Self
    }
}

impl IOrphanFilenameExtractorProtocol for OrphanFilenameExtractor {
    fn file_basename(&self, fp: &FilePath) -> Identity {
        Identity::new(file_basename(fp.value()))
    }

    fn file_stem(&self, fp: &FilePath) -> Identity {
        Identity::new(file_stem(fp.value()))
    }

    fn file_suffix(&self, fp: &FilePath) -> Identity {
        Identity::new(file_suffix(fp.value()))
    }
}
