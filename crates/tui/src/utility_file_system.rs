// PURPOSE: utility_file_system — stateless filesystem utilities for TUI surfaces
// Pure functions only — no DI, no trait params, no contract imports.
use shared::common::{DisplayContent, FilePath};
use std::io::Write;
use std::path::Path;

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
        if let Ok(mut clipboard) = arboard::Clipboard::new()
            && clipboard.set_text(text).is_ok()
        {
            return true;
        }
    }

    // Fallback to shell commands: xclip → wl-copy
    std::process::Command::new("sh")
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
        .unwrap_or(false)
}
