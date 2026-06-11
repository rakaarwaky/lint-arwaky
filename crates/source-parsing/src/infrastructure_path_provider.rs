// PURPOSE: PathProvider — IPathNormalizationPort implementation for path normalization
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use crate::contract_path_normalization_port::IPathNormalizationPort;
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
        path_str = normalized
            .components()
            .map(|c| c.as_os_str().to_string_lossy())
            .collect::<Vec<_>>()
            .join("/");

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
        }

        let abs_path = fs::canonicalize(Path::new(&path_str))
            .unwrap_or_else(|_| PathBuf::from(&path_str))
            .to_string_lossy()
            .replace("\\\\", "/");
        FilePath { value: abs_path }
    }
}
