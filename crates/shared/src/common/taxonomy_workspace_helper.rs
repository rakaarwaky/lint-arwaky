// PURPOSE: Free-function wrapper for workspace root detection.
//
// Walks up from the given path looking for Cargo.toml, crates/, packages/, or modules/.
// Used by mcp-server, cli-commands, and tui crates.

use std::path::PathBuf;

/// Walk up from `path` to find the workspace root (Cargo.toml, crates/, packages/, or modules/).
pub fn find_workspace_root(path: &str) -> Option<PathBuf> {
    let mut dir = std::path::Path::new(path).to_path_buf();
    if !dir.is_absolute() {
        dir = std::env::current_dir().ok()?.join(&dir);
    }
    // If the path is a file, start from its parent.
    if dir.is_file() {
        dir.pop();
    }
    loop {
        if dir.join("Cargo.toml").exists()
            || dir.join("crates").is_dir()
            || dir.join("packages").is_dir()
            || dir.join("modules").is_dir()
        {
            return Some(dir);
        }
        if !dir.pop() {
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_workspace_root_from_project_root() {
        let result = find_workspace_root(".");
        assert!(result.is_some());
        let root = result.unwrap();
        assert!(root.join("Cargo.toml").exists() || root.join("crates").is_dir());
    }

    #[test]
    fn find_workspace_root_nonexistent_path_returns_none() {
        let result = find_workspace_root("/nonexistent/path/xyz_123_test_only");
        assert!(result.is_none());
    }
}
