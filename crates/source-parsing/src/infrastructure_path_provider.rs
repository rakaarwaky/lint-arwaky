// PURPOSE: PathProvider — IPathNormalizationPort implementation for path normalization
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use crate::contract_path_normalization_port::IPathNormalizationPort;
use shared::source_parsing::taxonomy_barrel_provider::BarrelImportResolver;
use crate::taxonomy_path_vo::FilePath;

/// Implementation of path normalization services for infrastructure.
pub struct PathNormalizationProvider {}

impl IPathNormalizationPort for PathNormalizationProvider {
    /// Normalize path: fix slashes, resolve phantom roots.
    ///
    /// Reads PHANTOM_ROOT and PROJECT_ROOT from environment.
    /// Defaults to current working directory for PROJECT_ROOT.
    fn normalize_path(&self, path: FilePath) -> FilePath {
        let mut path_str = path.value.clone();
        if path_str.is_empty() {
            return path;
        }

        // 1. Normalize slashes and collapse separators
        path_str = path_str.replace("\\\\", "/");
        let normalized = Path::new(&path_str);
        let is_abs = normalized.is_absolute();
        let mut parts = Vec::new();
        for c in normalized.components() {
            if c == std::path::Component::RootDir {
                continue;
            }
            parts.push(c.as_os_str().to_string_lossy().into_owned());
        }
        path_str = parts.join("/");
        if is_abs {
            path_str = format!("/{}", path_str);
        }

        // 2. Handle phantom roots - only apply when path does NOT already exist
        if !Path::new(&path_str).exists() {
            let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
            let phantom_root = env::var("PHANTOM_ROOT")
                .unwrap_or(home)
                .replace("\\\\", "/");
            let actual_root = env::var("PROJECT_ROOT")
                .unwrap_or_else(|_| {
                    env::current_dir()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string()
                })
                .replace("\\\\", "/");

            if !phantom_root.is_empty() && path_str.starts_with(&phantom_root) {
                let suffix = &path_str[phantom_root.len()..];
                let suffix = suffix.strip_prefix('/').unwrap_or(suffix);
                path_str = format!("{}/{}", actual_root, suffix);
            }
        }

        // 3. Handle src/ and src-* pathing only if it's NOT explicitly relative or absolute
        if (path_str.starts_with("src/") || path_str.starts_with("src-"))
            && !Path::new(&path_str).exists()
        {
            if let Ok(project_root) = env::var("PROJECT_ROOT") {
                let candidate = Path::new(&project_root).join(&path_str);
                if candidate.exists() {
                    return FilePath {
                        value: candidate.to_string_lossy().replace("\\\\", "/"),
                    };
                }
            }
        }

        FilePath { value: path_str }
    }

    /// Unified path resolution for infrastructure adapters.
    ///
    /// 1. Normalizes the path using normalize_path.
    /// 2. If relative, tries to resolve against context_path.
    /// 3. Falls back to absolute path.
    fn resolve_infrastructure_path(
        &self,
        path: FilePath,
        context_path: Option<FilePath>,
    ) -> FilePath {
        let path_str = path.value.clone();
        let norm_path_vo = self.normalize_path(path);
        let norm_path = norm_path_vo.value.clone();

        if !norm_path.is_empty()
            && Path::new(&norm_path).is_absolute()
            && Path::new(&norm_path).exists()
        {
            return norm_path_vo;
        }

        if let Some(context) = context_path {
            let ctx_str = context.value;
            let base_dir = if Path::new(&ctx_str).is_file() {
                Path::new(&ctx_str)
                    .parent()
                    .unwrap_or_else(|| Path::new("."))
                    .to_string_lossy()
                    .to_string()
            } else {
                fs::canonicalize(&ctx_str)
                    .unwrap_or_else(|_| PathBuf::from(&ctx_str))
                    .to_string_lossy()
                    .to_string()
            };
            let possible = Path::new(&base_dir).join(&path_str);
            if possible.exists() {
                return FilePath {
                    value: possible.to_string_lossy().replace("\\\\", "/"),
                };
            }

            // Barrel-aware fallback: if direct context resolution fails,
            // check if the path comes from a barrel (mod.rs / __init__.py / index.ts).
            // Collect all project files from the context's project root, build a barrel
            // map, and find the absolute source file that the relative path refers to.
            let project_root = env::var("PROJECT_ROOT").unwrap_or_else(|_| {
                Path::new(&ctx_str)
                    .ancestors()
                    .find(|p| p.join("Cargo.toml").exists() || p.join("package.json").exists())
                    .unwrap_or_else(|| Path::new("."))
                    .to_string_lossy()
                    .to_string()
            });
            let project_files = Self::collect_source_files(&project_root);
            if !project_files.is_empty() {
                let barrel_map = BarrelImportResolver::build_barrel_map(&project_files);
                // stem to match: last component of path_str without extension
                let target_stem = Path::new(&path_str)
                    .file_stem()
                    .map(|s| s.to_string_lossy().replace('-', "_"))
                    .unwrap_or_default();
                for sources in barrel_map.values() {
                    for source in sources {
                        let source_stem = Path::new(source)
                            .file_stem()
                            .map(|s| s.to_string_lossy().replace('-', "_"))
                            .unwrap_or_default();
                        if source_stem == target_stem && Path::new(source).exists() {
                            return FilePath {
                                value: source.clone(),
                            };
                        }
                    }
                }
            }
        }

        let abs_path = fs::canonicalize(Path::new(&path_str))
            .unwrap_or_else(|_| PathBuf::from(&path_str))
            .to_string_lossy()
            .replace("\\\\", "/");
        FilePath { value: abs_path }
    }
}

impl PathNormalizationProvider {
    /// Collect all `.rs`, `.py`, `.ts`, `.js` source files under a directory.
    fn collect_source_files(root: &str) -> Vec<String> {
        let mut files = Vec::new();
        Self::walk_dir(Path::new(root), &mut files);
        files
    }

    fn walk_dir(dir: &Path, out: &mut Vec<String>) {
        let Ok(entries) = fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let name = path
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_default();
                // Skip build artifacts and hidden dirs
                if name == "target" || name.starts_with('.') {
                    continue;
                }
                Self::walk_dir(&path, out);
            } else if let Some(ext) = path.extension() {
                let ext = ext.to_string_lossy();
                if matches!(ext.as_ref(), "rs" | "py" | "ts" | "js" | "tsx") {
                    out.push(path.to_string_lossy().replace("\\\\", "/"));
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;

    #[test]
    fn test_path_normalization() {
        let provider = PathNormalizationProvider {};

        // Test normal path formatting
        let raw_path = FilePath::new("some\\\\nested\\\\path.rs".to_string()).unwrap();
        let norm = provider.normalize_path(raw_path);
        assert_eq!(norm.value, "some/nested/path.rs");

        // Test phantom roots
        let temp_dir = env::temp_dir();
        let temp_dir_str = temp_dir.to_string_lossy().to_string();

        env::set_var("PHANTOM_ROOT", "/phantom/root");
        env::set_var("PROJECT_ROOT", &temp_dir_str);

        // A path starting with phantom root that does NOT exist
        let phantom_path = FilePath::new("/phantom/root/my_file.rs".to_string()).unwrap();
        let resolved = provider.normalize_path(phantom_path);
        let expected = format!("{}/my_file.rs", temp_dir_str.replace("\\\\", "/"));
        assert_eq!(resolved.value.replace("//", "/"), expected.replace("//", "/"));

        // Clean up env variables
        env::remove_var("PHANTOM_ROOT");
        env::remove_var("PROJECT_ROOT");
    }

    #[test]
    fn test_resolve_infrastructure_path() {
        let provider = PathNormalizationProvider {};

        // Create a dummy file to test canonicalize/resolve
        let temp_file_path = "target/temp_resolve_test.rs";
        let context_file_path = "target/some_other_file.rs";
        fs::create_dir_all("target").unwrap();
        fs::write(temp_file_path, "// Resolve test").unwrap();
        fs::write(context_file_path, "// Context test").unwrap();

        let raw_path = FilePath::new(temp_file_path.to_string()).unwrap();
        let resolved = provider.resolve_infrastructure_path(raw_path, None);
        assert!(resolved.value.contains("temp_resolve_test.rs"));
        assert!(std::path::Path::new(&resolved.value).is_absolute());

        // Test resolution relative to context path
        let relative_path = FilePath::new("temp_resolve_test.rs".to_string()).unwrap();
        let context_abs_path = fs::canonicalize(context_file_path).unwrap();
        let context_path = FilePath::new(context_abs_path.to_string_lossy().to_string()).unwrap();
        let resolved_rel = provider.resolve_infrastructure_path(relative_path, Some(context_path));
        assert!(resolved_rel.value.contains("temp_resolve_test.rs"));
        assert!(std::path::Path::new(&resolved_rel.value).is_absolute());

        // Clean up
        let _ = fs::remove_file(temp_file_path);
        let _ = fs::remove_file(context_file_path);
    }
}
