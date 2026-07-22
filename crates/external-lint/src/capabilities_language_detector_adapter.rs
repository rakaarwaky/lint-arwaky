
use std::marker::PhantomData;
use std::path::Path;

use async_trait::async_trait;
use shared::common::taxonomy_common_vo::bool;
use shared::common::taxonomy_path_vo::FilePath;
use shared::external_lint::contract_external_lint_language_detector_protocol::{
    DetectedLanguages, IExternalLintLanguageDetectorProtocol,
};
use shared::external_lint::utility_external_lint_io as ext_io;

// PURPOSE: ExternalLintLanguageDetectorAdapter — IExternalLintLanguageDetectorProtocol implementation
//
// Scans a directory tree to detect which programming languages are present.
// Skips node_modules, target, .git, .jj directories.

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ExternalLintLanguageDetectorAdapter {
    _p: PhantomData<()>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

#[async_trait]
impl IExternalLintLanguageDetectorProtocol for ExternalLintLanguageDetectorAdapter {
    async fn detect_languages(&self, path: &FilePath) -> DetectedLanguages {
        let mut has_rs = false;
        let mut has_py = false;
        let mut has_js = false;

        let root_path = Path::new(&path.value);
        if ext_io::is_file(root_path) {
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

// ─── Block 3: Constructors, Helpers, Private Methods ──────

const SKIP_DIRS: &[&str] = &["node_modules", "target", ".git", ".jj", "Graph-It-Live"];

impl ExternalLintLanguageDetectorAdapter {
    pub fn new() -> Self {
        Self { _p: PhantomData }
    }

    fn detect_in_dir(dir: &Path, has_rs: &mut bool, has_py: &mut bool, has_js: &mut bool) {
        let entries = ext_io::scan_directory(dir);
        for (name, path_str, is_dir_entry) in entries {
            if is_dir_entry {
                if !SKIP_DIRS.contains(&name.as_str()) {
                    Self::detect_in_dir(Path::new(&path_str), has_rs, has_py, has_js);
                }
            } else if let Some(ext) = Path::new(&path_str).extension() {
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

