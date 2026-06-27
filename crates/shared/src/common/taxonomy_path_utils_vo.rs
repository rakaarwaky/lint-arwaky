pub struct PathUtils;

impl PathUtils {
    /// Walk a directory recursively, collecting files while skipping ignored patterns.
    pub fn walk_recursive(dir: &std::path::Path, ignored: &[&str]) -> Vec<std::path::PathBuf> {
        use std::fs;

        let mut results = Vec::new();

        if !dir.is_dir() {
            if dir.is_file() {
                if let Some(name_str) = dir.file_name().and_then(|s| s.to_str()) {
                    if !ignored.contains(&name_str) {
                        results.push(dir.to_path_buf());
                    }
                }
            }
            return results;
        }

        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();

                if let Some(name_str) = entry.file_name().to_str() {
                    if ignored.contains(&name_str) {
                        continue;
                    }

                    if path.is_dir() {
                        results.extend(Self::walk_recursive(&path, ignored));
                    } else {
                        results.push(path);
                    }
                }
            }
        }

        results
    }

    /// Convenience wrapper used by OSFileSystemAdapter and workspace helpers.
    pub fn collect_paths(start: &str, ignored: &[&str]) -> Vec<std::path::PathBuf> {
        let root = std::path::Path::new(start);
        Self::walk_recursive(root, ignored)
    }
}
