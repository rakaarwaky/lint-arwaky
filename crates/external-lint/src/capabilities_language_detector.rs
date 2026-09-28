use shared::common::taxonomy_path_vo::FilePath;
use shared::external_lint::contract_external_lint_protocol::ILanguageDetectProtocol;
use shared::filesystem::FilesystemRequest;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use std::sync::Arc;

/// FR-ExternalLint-001: lightweight extension walk that classifies which
/// languages (Rust, Python, JS/TS) and content types (Markdown) are present
/// under `path`.
///
/// Delegates to the filesystem aggregate's `discover_files` request and inspects
/// the file extensions of every path returned.
pub struct LanguageDetector {
    filesystem: Arc<dyn IFilesystemAggregate>,
}

impl ILanguageDetectProtocol for LanguageDetector {
    fn detect_languages(&self, path: &FilePath) -> (bool, bool, bool, bool) {
        let files = self
            .filesystem
            .execute(FilesystemRequest::discover_files(std::path::Path::new(
                &path.value,
            )))
            .into_paths();
        let has_rust = files.iter().any(|f| f.ends_with(".rs"));
        let has_python = files.iter().any(|f| f.ends_with(".py"));
        let has_js = files.iter().any(|f| {
            f.ends_with(".js") || f.ends_with(".jsx") || f.ends_with(".ts") || f.ends_with(".tsx")
        });
        let has_markdown = files.iter().any(|f| f.ends_with(".md"));
        (has_rust, has_python, has_js, has_markdown)
    }
}

impl LanguageDetector {
    pub fn new(filesystem: Arc<dyn IFilesystemAggregate>) -> Self {
        Self { filesystem }
    }
}
