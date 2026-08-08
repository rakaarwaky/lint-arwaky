// PURPOSE: utility_file_system — stateless filesystem utilities for TUI surfaces
// Pure functions only — no DI, no trait params, no contract imports.
use shared::common::FilePath;
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
