// PURPOSE: ExternalLintLanguageDetectorAdapter — IExternalLintLanguageDetectorProtocol implementation
//
// Scans a directory tree to detect which programming languages are present.
// Skips node_modules, target, .git, .jj directories.

use std::path::Path;

use async_trait::async_trait;
use shared::common::taxonomy_common_vo::bool;
use shared::common::taxonomy_path_vo::FilePath;
use shared::external_lint::contract_external_lint_language_detector_protocol::{
    DetectedLanguages, IExternalLintLanguageDetectorProtocol,
};

const SKIP_DIRS: &[&str] = &["node_modules", "target", ".git", ".jj", "Graph-It-Live"];

// ─── Block 1: Struct Definition ───────────────────────────
pub struct ExternalLintLanguageDetectorAdapter;

// ─── Block 2: Public Contract ─────────────────────────────
#[async_trait]
impl IExternalLintLanguageDetectorProtocol for ExternalLintLanguageDetectorAdapter {
    async fn detect_languages(&self, path: &FilePath) -> DetectedLanguages {
        let mut has_rs = false;
        let mut has_py = false;
        let mut has_js = false;

        let root_path = Path::new(&path.value);
        if root_path.is_file() {
            Self::detect_from_file(root_path, &mut has_rs, &mut has_py, &mut has_js);
        } else {
            Self::detect_in_dir(root_path, &mut has_rs, &mut has_py, &mut has_js);
        }

        DetectedLanguages {
            has_rs: bool::new(has_rs),
            has_py: bool::new(has_py),
            has_js: bool::new(has_js),
        }
    }
}

// ─── Block 3: Constructors & Helpers ──────────────────────
impl ExternalLintLanguageDetectorAdapter {
    pub fn new() -> Self {
        Self
    }

    fn detect_in_dir(dir: &Path, has_rs: &mut bool, has_py: &mut bool, has_js: &mut bool) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let name = match path.file_name() {
                        Some(n) => n.to_string_lossy(),
                        None => continue,
                    };
                    if !SKIP_DIRS.contains(&name.as_ref()) {
                        Self::detect_in_dir(&path, has_rs, has_py, has_js);
                    }
                } else if let Some(ext) = path.extension() {
                    match ext.to_str() {
                        Some("rs") => *has_rs = true,
                        Some("py") => *has_py = true,
                        Some("js" | "ts" | "jsx" | "tsx") => *has_js = true,
                        _ => {}
                    }
                }
                if *has_rs && *has_py && *has_js {
                    break;
                }
            }
        }
    }

    fn detect_from_file(path: &Path, has_rs: &mut bool, has_py: &mut bool, has_js: &mut bool) {
        if let Some(ext) = path.extension() {
            match ext.to_str() {
                Some("rs") => *has_rs = true,
                Some("py") => *has_py = true,
                Some("js" | "ts" | "jsx" | "tsx") => *has_js = true,
                _ => {}
            }
        }
    }
}

impl Default for ExternalLintLanguageDetectorAdapter {
    fn default() -> Self {
        Self::new()
    }
}
