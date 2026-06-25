use crate::contract_file_system_port::IFileSystemPort;
use crate::taxonomy_file_entry_vo::FileEntry;
use std::sync::Arc;

pub struct FileBrowser {
    fs_port: Arc<dyn IFileSystemPort>,
}

impl FileBrowser {
    pub fn new(fs_port: Arc<dyn IFileSystemPort>) -> Self {
        Self { fs_port }
    }

    pub fn list_directory(&self, path: &str) -> Vec<FileEntry> {
        let mut entries = self.fs_port.list_directory(path);
        entries.sort_by(|a, b| match (a.is_dir, b.is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        });
        entries
    }

    pub fn filter_entries(&self, entries: &[FileEntry], query: &str) -> Vec<FileEntry> {
        if query.is_empty() {
            return entries.to_vec();
        }
        let query_lower = query.to_lowercase();
        entries
            .iter()
            .filter(|e| e.name.to_lowercase().contains(&query_lower))
            .cloned()
            .collect()
    }

    pub fn parent_path(&self, current: &str) -> Option<String> {
        self.fs_port.parent_directory(current)
    }

    pub fn is_valid_dir(&self, path: &str) -> bool {
        self.fs_port.is_valid_directory(path)
    }
}
