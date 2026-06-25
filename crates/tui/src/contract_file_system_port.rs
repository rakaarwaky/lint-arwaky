use crate::taxonomy_file_entry_vo::FileEntry;

pub trait IFileSystemPort: Send + Sync {
    fn list_directory(&self, path: &str) -> Vec<FileEntry>;
    fn read_file_preview(&self, path: &str, max_lines: usize) -> String;
    fn is_valid_directory(&self, path: &str) -> bool;
    fn parent_directory(&self, path: &str) -> Option<String>;
    fn file_size_human(&self, bytes: u64) -> String;
    fn path_components(&self, path: &str) -> Vec<String>;
}
