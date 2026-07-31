// PURPOSE: Utility layer — file walker using `ignore` crate
// Walks directory tree parallel, gitignore-aware, filters by extension.

use crate::contract_filesystem_protocol::IFileWalkerProtocol;
use crate::taxonomy_filesystem_vo::*;
use camino::Utf8PathBuf;
use ignore::WalkBuilder;

pub struct FileWalker;

impl FileWalker {
    pub fn new() -> Self {
        Self
    }
}

impl Default for FileWalker {
    fn default() -> Self {
        Self::new()
    }
}

impl IFileWalkerProtocol for FileWalker {
    fn walk(
        &self,
        root: &Utf8PathBuf,
        ignored: &[String],
        extensions: &[&str],
    ) -> Vec<FileEntry> {
        let mut builder = WalkBuilder::new(root.as_std_path());
        builder
            .hidden(true) // skip hidden dirs (.git, .venv)
            .git_ignore(true)
            .git_global(true)
            .git_exclude(true)
            .threads(num_cpus::get());

        // Add custom ignore patterns
        for pattern in ignored {
            builder.add_custom_ignore(pattern);
        }

        let supported: Vec<&str> = extensions.to_vec();
        let mut entries = Vec::new();

        for result in builder.build() {
            let entry = match result {
                Ok(e) => e,
                Err(_) => continue,
            };

            let path = match entry.path().to_str() {
                Some(p) => p,
                None => continue,
            };

            // Skip directories
            if entry.file_type().map_or(false, |ft| ft.is_dir()) {
                continue;
            }

            // Filter by extension
            let ext = match path.rsplit('.').next() {
                Some(e) => e,
                None => continue,
            };

            if !supported.contains(&ext) {
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

            // Skip files > 2 MiB
            if metadata.len() > MAX_LINT_FILE_BYTES {
                continue;
            }

            let utf8_path = match Utf8PathBuf::from_path_buf(entry.into_path()) {
                Ok(p) => p,
                Err(_) => continue,
            };

            entries.push(FileEntry {
                path: utf8_path,
                extension: ext.to_string(),
                language,
                size: metadata.len(),
            });
        }

        entries
    }
}

/// Number of logical CPUs (fallback to 4).
fn num_cpus() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
}
