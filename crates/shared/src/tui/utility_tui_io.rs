// PURPOSE: utility_tui_io — stateless I/O utilities for TUI capabilities
use std::process::Command;

/// Write text content to a file at the given path.
/// Returns Ok(()) on success, Err with OS error message on failure.
pub fn write_text_to_file(path: &std::path::Path, text: &str) -> Result<(), String> {
    std::fs::write(path, text).map_err(|e| format!("Failed to write file: {e}"))
}

/// Check if a binary is available in the system PATH.
pub fn is_binary_available(bin_name: &str) -> bool {
    let output = Command::new("sh")
        .args(["-c", &format!("command -v {} >/dev/null 2>&1", bin_name)])
        .status();

    match output {
        Ok(s) => s.success(),
        Err(_) => false,
    }
}
