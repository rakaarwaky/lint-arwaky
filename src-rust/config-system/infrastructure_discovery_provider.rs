// aes: wired-by-dispatch
/// config_discovery_provider — Provider for discovering configuration files in the filesystem.
use crate::config_system::contract_discovery_port::IConfigDiscoveryPort;
use crate::config_system::taxonomy_config_error::ConfigError;
use crate::config_system::taxonomy_config_vo::default_aes_config;
use crate::config_system::taxonomy_config_vo::parse_config_yaml;
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use crate::source_parsing::taxonomy_path_vo::DirectoryPath;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use std::env;
use std::path::Path;

pub struct ConfigDiscoveryProvider {}

impl Default for ConfigDiscoveryProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigDiscoveryProvider {
    pub fn new() -> Self {
        Self {}
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

    pub fn load_architecture_config(
        project_root: Option<&Path>,
        src_dir: &Path,
    ) -> ArchitectureConfig {
        let search_start = project_root.unwrap_or_else(|| src_dir.parent().unwrap_or(src_dir));
        const CONFIG_NAMES: &[&str] = &["lint_arwaky.config.rust.yaml", "lint_arwaky.config.yaml"];
        let mut dir = search_start;
        loop {
            for name in CONFIG_NAMES {
                let candidate = dir.join(name);
                if candidate.is_file() {
                    if let Ok(content) = std::fs::read_to_string(&candidate) {
                        let cfg = parse_config_yaml(&content);
                        if !cfg.layers.is_empty() {
                            return cfg;
                        }
                    }
                }
            }
            match dir.parent() {
                Some(p) if p != dir => dir = p,
                _ => break,
            }
        }
        default_aes_config()
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
                "lint_arwaky.config.rust.yaml",
                "lint_arwaky.config.python.yaml",
                "lint_arwaky.config.yaml",
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
