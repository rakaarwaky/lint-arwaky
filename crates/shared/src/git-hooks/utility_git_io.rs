// PURPOSE: Git I/O utility — stateless git command execution and file operation helpers
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

use std::path::Path;
use std::process::Command;

/// Execute a git command and return stdout/stderr/success status.
pub fn run_git_command(args: &[&str], dir: &str) -> (String, String, bool) {
    let output = Command::new("git").args(args).current_dir(dir).output();

    match output {
        Ok(o) => (
            String::from_utf8_lossy(&o.stdout).to_string(),
            String::from_utf8_lossy(&o.stderr).to_string(),
            o.status.success(),
        ),
        Err(e) => (
            String::new(),
            format!("Failed to execute git: {}", e),
            false,
        ),
    }
}

/// Parse successful command output into trimmed non-empty lines.
pub fn parse_output_lines(output: &str) -> Vec<String> {
    output
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect()
}

/// Create a directory (and all parent directories) for the given path.
pub fn create_dir_all<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    std::fs::create_dir_all(path)
}

/// Get metadata for a file/directory.
pub fn metadata<P: AsRef<Path>>(path: P) -> std::io::Result<std::fs::Metadata> {
    std::fs::metadata(path)
}

/// Set permissions on a file.
#[cfg(unix)]
pub fn set_permissions<P: AsRef<Path>>(path: P, mode: u32) -> std::io::Result<()> {
    let mut perms = std::fs::metadata(&path)?.permissions();
    perms.set_mode(mode);
    std::fs::set_permissions(path, perms)
}

/// Remove a file.
pub fn remove_file<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    std::fs::remove_file(path)
}
