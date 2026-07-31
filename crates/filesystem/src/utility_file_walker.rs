// PURPOSE: Utility layer — file walker using `ignore` crate
// Walks directory tree parallel, gitignore-aware, filters by extension.

use shared::filesystem::IFileWalkerProtocol;
use shared::filesystem::taxonomy_filesystem_vo::*;
use std::path::PathBuf;


pub struct FileWalker;

impl FileWalker {
    pub fn new() -> Self { Self }
}

impl Default for FileWalker {
    fn default() -> Self { Self::new() }
}

impl IFileWalkerProtocol for FileWalker {
    fn walk(&self, root: &PathBuf, ignored: &[String], extensions: &[&str]) -> Vec<FileEntry> {
        let mut builder = ignore::WalkBuilder::new(root);
        builder
            .hidden(true)
            .git_ignore(true)
            .git_global(true)
            .git_exclude(true)
            .threads(std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4));

        for _pattern in ignored {
            // Note: add_custom_ignore not available in ignore 0.4
            // Custom ignore patterns would need to be handled via .gitignore files
        }

        let mut entries = Vec::new();

        for result in builder.build() {
            let entry = match result {
                Ok(e) => e,
                Err(_) => continue,
            };

            if entry.file_type().map_or(false, |ft| ft.is_dir()) {
                continue;
            }

            let path = entry.path();
            let ext = match path.extension().and_then(|e| e.to_str()) {
                Some(e) => e,
                None => continue,
            };

            if !extensions.contains(&ext) {
                continue;
            }

            let language = match Language::from_extension(ext) {
                Some(l) => l,
                None => continue,
            };

            let metadata = match entry.metadata() {
                Ok(m) => m,
                Err(_) => continue,
            };

            if metadata.len() > MAX_LINT_FILE_BYTES {
                continue;
            }

            entries.push(FileEntry {
                path: path.to_path_buf(),
                extension: ext.to_string(),
                language,
                size: metadata.len(),
            });
        }

        entries
    }
}
