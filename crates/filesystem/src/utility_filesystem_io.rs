// File I/O primitives — thin wrappers over std::fs
// Used by: config-system, auto-fix, git-hooks, tui, maintenance, project-setup
//
// Utility: stateless standalone functions

use std::path::{Path, PathBuf};

// ═══════════════════════════════════════════════════════════════
// File Reading
// ═══════════════════════════════════════════════════════════════

/// Read file content synchronously.
pub fn read_file<P: AsRef<Path>>(path: P) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}

/// Read file content, returning empty string on error.
pub fn read_file_safe<P: AsRef<Path>>(path: P) -> String {
    std::fs::read_to_string(path).unwrap_or_default()
}

/// Read file content to string.
pub fn read_to_string(path: &Path) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}

/// Read file for linting: checks cache, enforces 2MiB size limit.
pub fn read_lintable_file(path: &str) -> Result<Option<String>, String> {
    let meta = std::fs::metadata(path).map_err(|e| format!("{}: {}", path, e))?;
    if meta.len() > 2 * 1024 * 1024 {
        return Ok(None);
    }
    std::fs::read_to_string(path)
        .map(Some)
        .map_err(|e| format!("{}: {}", path, e))
}

// ═══════════════════════════════════════════════════════════════
// File Writing
// ═══════════════════════════════════════════════════════════════

/// Write content to a file.
pub fn write_file<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> std::io::Result<()> {
    std::fs::write(path, contents)
}

/// Write string to file.
pub fn write_string(path: &Path, content: &str) -> Result<(), std::io::Error> {
    std::fs::write(path, content)
}

/// Write text content to a file.
pub fn write_text_to_file(path: &Path, text: &str) -> Result<(), String> {
    write_file(path, text.as_bytes()).map_err(|e| format!("Failed to write file: {e}"))
}

/// Copy file from src to dst.
pub fn copy_file(src: &Path, dst: &Path) -> Result<u64, std::io::Error> {
    std::fs::copy(src, dst)
}

/// Remove a file.
pub fn remove_file(path: &Path) -> std::io::Result<()> {
    std::fs::remove_file(path)
}

/// Set file permissions (Unix mode bits).
pub fn set_permissions(path: &Path, mode: u32) -> std::io::Result<()> {
    let mut perms = std::fs::metadata(path)?.permissions();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        perms.set_mode(mode);
    }
    std::fs::set_permissions(path, perms)
}

// ═══════════════════════════════════════════════════════════════
// Path Operations
// ═══════════════════════════════════════════════════════════════

/// Check if path exists.
pub fn path_exists<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().exists()
}

/// Check if path is a directory.
pub fn is_dir<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().is_dir()
}

/// Check if path is a file.
pub fn is_file<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().is_file()
}

/// Check if path is a symlink.
pub fn is_symlink<P: AsRef<Path>>(path: P) -> bool {
    std::fs::symlink_metadata(path)
        .map(|m| m.file_type().is_symlink())
        .unwrap_or(false)
}

/// Get file metadata.
pub fn metadata<P: AsRef<Path>>(path: P) -> Result<std::fs::Metadata, std::io::Error> {
    std::fs::metadata(path)
}

/// Get symlink metadata (does not follow symlinks).
pub fn symlink_metadata<P: AsRef<Path>>(path: P) -> Result<std::fs::Metadata, std::io::Error> {
    std::fs::symlink_metadata(path)
}

/// Canonicalize path (resolve symlinks).
pub fn canonicalize(path: &Path) -> Result<PathBuf, std::io::Error> {
    std::fs::canonicalize(path)
}

/// Canonicalize path to absolute string.
pub fn canonicalize_path_str(path_str: &str) -> String {
    canonicalize_path(path_str).to_string_lossy().to_string()
}

/// Canonicalize a path string to absolute.
pub fn canonicalize_path(path_str: &str) -> PathBuf {
    let path = Path::new(path_str);
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join(path)
    }
}

/// Get file basename (filename without directory).
pub fn get_basename(path: &str) -> &str {
    Path::new(path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(path)
}

/// Get file stem (filename without extension).
pub fn get_file_stem(path: &str) -> &str {
    Path::new(path)
        .file_stem()
        .and_then(|n| n.to_str())
        .unwrap_or(path)
}

/// Get parent directory path.
pub fn get_parent(path: &str) -> &str {
    Path::new(path)
        .parent()
        .and_then(|p| p.to_str())
        .unwrap_or(path)
}

/// Check if a path has a source file extension.
pub fn is_source_file(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|e| e.to_str()),
        Some("rs" | "py" | "ts" | "js" | "tsx" | "jsx")
    )
}

/// Check if an extension string is a recognized source file extension.
pub fn is_source_ext(ext: &str) -> bool {
    matches!(ext, "rs" | "py" | "ts" | "js" | "tsx" | "jsx")
}

/// Return true if rel_path should be skipped based on ignored patterns.
pub fn is_path_ignored(rel_path: &str, ignored: &[String]) -> bool {
    if rel_path.is_empty() {
        return false;
    }
    let segments: Vec<&str> = rel_path
        .split(['/', '\\'])
        .filter(|s| !s.is_empty())
        .collect();
    for pat in ignored {
        if pat.is_empty() {
            continue;
        }
        if let Some(stripped) = pat.strip_prefix('/') {
            if stripped.is_empty() {
                continue;
            }
            let pat_segments: Vec<&str> = stripped
                .split(['/', '\\'])
                .filter(|s| !s.is_empty())
                .collect();
            if pat_segments.is_empty() {
                continue;
            }
            let n_pat = pat_segments.len();
            let n_seg = segments.len();
            if n_seg < n_pat {
                continue;
            }
            for start in 0..=(n_seg - n_pat) {
                if segments[start..start + n_pat] == pat_segments[..] {
                    return true;
                }
            }
            continue;
        }
        if pat.starts_with("**/") {
            let suffix = pat.strip_prefix("**/").unwrap_or(pat);
            if let Some(ext_pattern) = suffix.strip_prefix("*.") {
                let ext = ext_pattern.trim_start_matches('.');
                if !ext.is_empty() {
                    let basename = segments.last().copied().unwrap_or_default();
                    if basename.ends_with(&format!(".{ext}")) {
                        return true;
                    }
                }
            }
            continue;
        }
        if let Some(prefix) = pat.strip_suffix("/*") {
            if !prefix.is_empty() && segments.first() == Some(&prefix) {
                return true;
            }
            continue;
        }
        if let Some(suffix) = pat.strip_prefix("*.") {
            let suffix = suffix.trim_start_matches('.');
            if suffix.is_empty() {
                continue;
            }
            let basename = segments.last().copied().unwrap_or_default();
            if basename.ends_with(&format!(".{suffix}")) {
                return true;
            }
            continue;
        }
        if pat.starts_with('.') {
            if segments.iter().any(|seg| *seg == pat) {
                return true;
            }
            continue;
        }
        let pat_segments: Vec<&str> = pat.split(['/', '\\']).filter(|s| !s.is_empty()).collect();
        if pat_segments.len() == 1 {
            if segments.contains(&pat_segments[0]) {
                return true;
            }
        } else if pat_segments.len() > 1 {
            let n_pat = pat_segments.len();
            let n_seg = segments.len();
            if n_seg >= n_pat {
                for start in 0..=(n_seg - n_pat) {
                    if segments[start..start + n_pat] == pat_segments[..] {
                        return true;
                    }
                }
            }
        }
    }
    false
}

// ═══════════════════════════════════════════════════════════════
// Directory Operations
// ═══════════════════════════════════════════════════════════════

/// Create directory and all parents.
pub fn create_dir_all(path: &Path) -> Result<(), std::io::Error> {
    std::fs::create_dir_all(path)
}

/// Remove directory recursively.
pub fn remove_dir_all(path: &Path) -> Result<(), std::io::Error> {
    std::fs::remove_dir_all(path)
}

/// List directory entries as Vec<PathBuf>.
pub fn scan_directory(dir: &Path) -> Vec<PathBuf> {
    let mut entries = Vec::new();
    if let Ok(read_dir) = dir.read_dir() {
        for entry in read_dir.flatten() {
            entries.push(entry.path());
        }
    }
    entries
}

/// List directory entries with ignore filter.
pub fn scan_directory_with_ignored(dir: &Path, ignored: &[String]) -> Vec<PathBuf> {
    scan_directory(dir)
        .into_iter()
        .filter(|p| {
            let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
            !is_path_ignored(name, ignored)
        })
        .collect()
}

/// Read directory entries as Vec<PathBuf>.
pub fn read_dir_entries_as_pathbuf(dir: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut entries = Vec::new();
    for e in std::fs::read_dir(dir)?.flatten() {
        entries.push(e.path());
    }
    Ok(entries)
}

// ═══════════════════════════════════════════════════════════════
// Process Execution
// ═══════════════════════════════════════════════════════════════

/// Execute a git command and return stdout/stderr/success status.
pub fn run_git_command(args: &[&str], dir: &str) -> (String, String, bool) {
    let output = std::process::Command::new("git")
        .args(args)
        .current_dir(dir)
        .output();

    match output {
        Ok(o) => (
            String::from_utf8_lossy(&o.stdout).to_string(),
            String::from_utf8_lossy(&o.stderr).to_string(),
            o.status.success(),
        ),
        Err(e) => (
            String::new(),
            format!("Failed to execute git: {e}"),
            false,
        ),
    }
}

/// Execute an external command with working directory.
pub fn run_external_command_in(
    name: &str,
    args: &[&str],
    current_dir: &str,
) -> (String, String, bool) {
    shared::common::utility_command_runner::run_command_in_dir(name, args, Some(current_dir))
}

/// Parse command output into trimmed non-empty lines.
pub fn parse_output_lines(output: &str) -> Vec<String> {
    output
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect()
}

/// No-op apply_fix for linters that cannot auto-fix.
pub async fn noop_apply_fix() -> Result<
    shared::common::taxonomy_message_vo::ComplianceStatus,
    shared::common::taxonomy_operation_error::LinterOperationError,
> {
    Ok(shared::common::taxonomy_message_vo::ComplianceStatus::new(
        false,
    ))
}

/// No-op apply_fix sync version for non-async contexts.
pub fn noop_apply_fix_sync() -> Result<
    shared::common::taxonomy_message_vo::ComplianceStatus,
    shared::common::taxonomy_operation_error::LinterOperationError,
> {
    Ok(shared::common::taxonomy_message_vo::ComplianceStatus::new(
        false,
    ))
}
