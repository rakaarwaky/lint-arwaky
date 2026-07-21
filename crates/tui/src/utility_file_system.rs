// PURPOSE: utility_file_system — stateless filesystem utilities for TUI surfaces
use shared::common::taxonomy_display_content_vo::DisplayContent;
use shared::common::taxonomy_path_vo::FilePath;
use shared::tui::taxonomy_file_entry_vo::FileEntry;
use std::io::Write;
use std::path::Path;

/// List directory entries, skipping hidden files (starting with '.').
pub fn list_directory(path: &FilePath) -> Vec<FileEntry> {
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

/// Read up to `max_lines` lines of a file with line-numbered formatting.
/// Truncates with "... (N more lines)" note if the file exceeds max_lines.
pub fn read_file_preview(path: &FilePath, max_lines: usize) -> DisplayContent {
    let file_path = Path::new(path.value());
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

/// Check whether a path points to a valid directory.
pub fn is_valid_directory(path: &FilePath) -> bool {
    Path::new(path.value()).is_dir()
}

/// Resolve the parent directory of a path.
pub fn parent_directory(path: &FilePath) -> Option<FilePath> {
    Path::new(path.value())
        .parent()
        .and_then(|p| FilePath::new(p.to_string_lossy().to_string()).ok())
}

/// Format byte count as human-readable string (B, K, M, G).
pub fn file_size_human(bytes: u64) -> DisplayContent {
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

/// Split a file path into its individual components (file name, parent dir segments).
pub fn path_components(path: &FilePath) -> Vec<FilePath> {
    Path::new(path.value())
        .components()
        .filter_map(|c| {
            c.as_os_str()
                .to_str()
                .and_then(|s| FilePath::new(s.to_string()).ok())
        })
        .collect()
}

/// Copy text to the system clipboard.
/// Tries arboard first (if available), falls back to xclip/wl-copy shell commands.
/// Returns true if copy succeeded, false otherwise.
pub fn copy_text_to_clipboard(text: &str) -> bool {
    // Try arboard first
    #[cfg(not(test))]
    {
        if let Ok(mut clipboard) = arboard::Clipboard::new() {
            if clipboard.set_text(text).is_ok() {
                return true;
            }
        }
    }

    // Fallback to shell commands: xclip → wl-copy
    let success = std::process::Command::new("sh")
        .arg("-c")
        .arg("xclip -selection clipboard 2>/dev/null || wl-copy 2>/dev/null")
        .stdin(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            if let Some(ref mut stdin) = child.stdin {
                let _ = stdin.write_all(text.as_bytes());
            }
            child.wait()
        })
        .map(|status| status.success())
        .unwrap_or(false);

    success
}
