// PURPOSE: Shared utilities for JavaScript/TypeScript external linter adapters
use shared::common::taxonomy_path_vo::FilePath;
use std::path::Path;

fn is_bun_available() -> bool {
    match std::process::Command::new("bun")
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
    {
        Ok(s) => s.success(),
        Err(_) => false,
    }
}

pub fn resolve_js_cmd(executable: &str, args: Vec<String>, working_dir: &str) -> Vec<String> {
    let local_bin = Path::new(working_dir)
        .join("node_modules")
        .join(".bin")
        .join(executable);

    if local_bin.exists() {
        let mut cmd = vec![local_bin.to_string_lossy().to_string()];
        cmd.extend(args);
        return cmd;
    }

    let runner = if is_bun_available() { "bunx" } else { "npx" };
    let mut cmd = vec![runner.to_string(), executable.to_string()];
    cmd.extend(args);
    cmd
}

pub fn resolve_working_dir(path: &FilePath) -> FilePath {
    let path_str = &path.value;
    if let Ok(abs_path) = std::fs::canonicalize(path_str) {
        let mut current = if abs_path.is_file() {
            match abs_path.parent() {
                Some(p) => p.to_path_buf(),
                None => std::path::PathBuf::from("."),
            }
        } else {
            abs_path.clone()
        };

        for _ in 0..10 {
            if current.join("lint_arwaky.config.yaml").is_file()
                || current.join("lint_arwaky.config.python.yaml").is_file()
                || current.join("package.json").is_file()
                || current.join(".git").is_dir()
            {
                return FilePath::new(current.to_string_lossy().to_string()).unwrap_or_default();
            }
            match current.parent() {
                Some(parent) => current = parent.to_path_buf(),
                None => break,
            }
        }
        return FilePath::new(current.to_string_lossy().to_string()).unwrap_or_default();
    }
    FilePath::new(".".to_string()).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_working_dir_returns_valid_path() {
        let path = FilePath::new(std::env::current_dir()
            .unwrap()
            .to_string_lossy()
            .to_string())
        .unwrap_or_default();
        let result = resolve_working_dir(&path);
        assert!(!result.value.is_empty());
    }

    #[test]
    fn test_resolve_working_dir_nonexistent() {
        let path = FilePath::new("/nonexistent_path_xyz".to_string()).unwrap_or_default();
        let result = resolve_working_dir(&path);
        assert_eq!(result.value, ".");
    }

    #[test]
    fn test_resolve_js_cmd_nonexistent_local() {
        let cmd = resolve_js_cmd("eslint", vec!["--version".to_string()], "/nonexistent_dir");
        assert!(cmd.len() >= 3);
        assert!(cmd[1] == "eslint" || cmd[0].contains("eslint"));
    }
}
