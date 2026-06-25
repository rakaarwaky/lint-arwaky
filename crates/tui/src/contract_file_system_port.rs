use crate::taxonomy_file_entry_vo::FileEntry;
use shared::common::taxonomy_byte_count_vo::ByteCount;
use shared::common::taxonomy_display_content_vo::DisplayContent;
use shared::common::taxonomy_line_count_vo::LineCount;
use shared::source_parsing::taxonomy_path_vo::FilePath;

pub trait IFileSystemPort: Send + Sync {
    fn list_directory(&self, path: &FilePath) -> Vec<FileEntry>;
    fn read_file_preview(&self, path: &FilePath, max_lines: &LineCount) -> DisplayContent;
    fn is_valid_directory(&self, path: &FilePath) -> bool;
    fn parent_directory(&self, path: &FilePath) -> Option<FilePath>;
    fn file_size_human(&self, bytes: &ByteCount) -> DisplayContent;
    fn path_components(&self, path: &FilePath) -> Vec<FilePath>;
}
