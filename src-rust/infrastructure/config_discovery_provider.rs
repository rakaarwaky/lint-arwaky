/// config_discovery_provider — Provider for discovering configuration files in the filesystem.
use crate::contract::config_discovery_port::IConfigDiscoveryPort;
use crate::taxonomy::{ConfigError, DirectoryPath, FilePath};
use std::env;
use std::path::Path;

pub struct ConfigDiscoveryProvider;

impl Default for ConfigDiscoveryProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigDiscoveryProvider {
    pub fn new() -> Self {
        Self
    }

    fn walk_up(start: &Path, filenames: &[&str]) -> Option<FilePath> {
        let mut current = if start.is_file() {
            start.parent().unwrap_or(start).to_path_buf()
        } else {
            start.to_path_buf()
        };
        for _ in 0..5 {
            for name in filenames {
                let candidate = current.join(name);
                if candidate.is_file() {
                    return FilePath::new(candidate.to_string_lossy().to_string()).ok();
                }
            }
            if let Some(parent) = current.parent() {
                if parent == current {
                    break;
                }
                current = parent.to_path_buf();
            } else {
                break;
            }
        }
        None
    }
}

impl IConfigDiscoveryPort for ConfigDiscoveryProvider {
    fn find_env_file(
        &self,
        start: Option<&DirectoryPath>,
    ) -> Option<Result<FilePath, ConfigError>> {
        let base = start
            .map(|d| Path::new(&d.value).to_path_buf())
            .unwrap_or_else(|| std::env::current_dir().ok().unwrap_or_default());
        Self::walk_up(&base, &[".env"]).map(Ok)
    }

    fn find_yaml_config(
        &self,
        start: Option<&DirectoryPath>,
    ) -> Option<Result<FilePath, ConfigError>> {
        let explicit = env::var("AUTO_LINTER_CONFIG").ok();
        if let Some(ref path) = explicit {
            let p = Path::new(path);
            if p.is_file() {
                return FilePath::new(path.clone()).ok().map(Ok);
            }
        }
        let base = start
            .map(|d| Path::new(&d.value).to_path_buf())
            .unwrap_or_else(|| std::env::current_dir().ok().unwrap_or_default());
        Self::walk_up(
            &base,
            &[
                "lint_arwaky.config.jinja",
                "lint_arwaky.config.yaml",
                "lint_arwaky.config.python.yaml",
            ],
        )
        .map(Ok)
    }

    fn find_toml_config(
        &self,
        start: Option<&DirectoryPath>,
    ) -> Option<Result<FilePath, ConfigError>> {
        let base = start
            .map(|d| Path::new(&d.value).to_path_buf())
            .unwrap_or_else(|| std::env::current_dir().ok().unwrap_or_default());
        Self::walk_up(&base, &["pyproject.toml"]).map(Ok)
    }
}
