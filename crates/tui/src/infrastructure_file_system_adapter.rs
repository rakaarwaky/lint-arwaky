use shared::common::taxonomy_byte_count_vo::ByteCount;
use shared::common::taxonomy_display_content_vo::DisplayContent;
use shared::common::taxonomy_line_count_vo::LineCount;
use shared::common::taxonomy_path_vo::FilePath;
use shared::tui::contract_file_system_port::IFileSystemPort;
use shared::tui::taxonomy_file_entry_vo::FileEntry;
use std::path::Path;

pub struct FileSystemAdapter;

impl FileSystemAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for FileSystemAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl IFileSystemPort for FileSystemAdapter {
    fn list_directory(&self, path: &FilePath) -> Vec<FileEntry> {
        let dir_path = Path::new(path.value());
        let read_dir = match dir_path.read_dir() {
            Ok(rd) => rd,
            Err(_) => return Vec::new(),
        };

        let mut entries = Vec::new();
        for dir_entry in read_dir.flatten() {
            let entry_path = dir_entry.path();
            let name = match entry_path.file_name().and_then(|n| n.to_str()) {
                Some(n) => n.to_string(),
                None => continue,
            };
            if name.starts_with('.') {
                continue;
            }
            if let Some(file_entry) = FileEntry::from_path(&entry_path) {
                entries.push(file_entry);
            }
        }
        entries
    }

    fn read_file_preview(&self, path: &FilePath, max_lines: &LineCount) -> DisplayContent {
        let file_path = Path::new(path.value());
        let max_lines = max_lines.value();
        let content = match std::fs::read_to_string(file_path) {
            Ok(c) => c,
            Err(e) => return DisplayContent::new(format!("Cannot read file: {e}")),
        };

        let lines: Vec<&str> = content.lines().take(max_lines).collect();
        let mut output = String::new();
        for (i, line) in lines.iter().enumerate() {
            output.push_str(&format!("{:>4} \u{2502} {}\n", i + 1, line));
        }
        let total_lines = content.lines().count();
        if total_lines > max_lines {
            output.push_str(&format!("\n... ({} more lines)", total_lines - max_lines));
        }
        DisplayContent::new(output)
    }

    fn is_valid_directory(&self, path: &FilePath) -> bool {
        Path::new(path.value()).is_dir()
    }

    fn parent_directory(&self, path: &FilePath) -> Option<FilePath> {
        Path::new(path.value())
            .parent()
            .and_then(|p| FilePath::new(p.to_string_lossy().to_string()).ok())
    }

    fn file_size_human(&self, bytes: &ByteCount) -> DisplayContent {
        let bytes = bytes.value();
        const KB: u64 = 1024;
        const MB: u64 = KB * 1024;
        const GB: u64 = MB * 1024;

        DisplayContent::new(if bytes >= GB {
            format!("{:.1}G", bytes as f64 / GB as f64)
        } else if bytes >= MB {
            format!("{:.1}M", bytes as f64 / MB as f64)
        } else if bytes >= KB {
            format!("{:.1}K", bytes as f64 / KB as f64)
        } else {
            format!("{}B", bytes)
        })
    }

    fn path_components(&self, path: &FilePath) -> Vec<FilePath> {
        Path::new(path.value())
            .components()
            .filter_map(|c| {
                c.as_os_str()
                    .to_str()
                    .and_then(|s| FilePath::new(s.to_string()).ok())
            })
            .collect()
    }
}
