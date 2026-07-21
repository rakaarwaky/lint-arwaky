// PURPOSE: utility_tui_io — stateless I/O utilities for TUI capabilities

/// Write text content to a file at the given path.
/// Returns Ok(()) on success, Err with OS error message on failure.
pub fn write_text_to_file(path: &std::path::Path, text: &str) -> Result<(), String> {
    std::fs::write(path, text).map_err(|e| format!("Failed to write file: {e}"))
}

/// Check if a binary is available in the system PATH.
pub fn is_binary_available(bin_name: &str) -> bool {
    // Validate bin_name to prevent any shell-like characters
    if bin_name.is_empty()
        || bin_name
            .chars()
            .any(|c| !c.is_alphanumeric() && c != '_' && c != '-')
    {
        return false;
    }

    std::env::current_exe()
        .ok()
        .and_then(|exe| exe.parent().map(|p| p.to_path_buf()))
        .is_none_or(|dir| {
            let path = dir.join(bin_name);
            path.exists() || find_in_path(bin_name)
        })
}

fn find_in_path(bin_name: &str) -> bool {
    if let Some(paths) = std::env::var_os("PATH") {
        for dir in std::env::split_paths(&paths) {
            let path = dir.join(bin_name);
            if path.exists() {
                return true;
            }
        }
    }
    false
}
