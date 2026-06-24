// PURPOSE: ConfigYamlReader — reads and parses lint-arwaky YAML config files from disk
// XDG Base Directory Specification compliant config lookup
use async_trait::async_trait;
use shared::config_system::contract_reader_port::IConfigReaderPort;
use shared::config_system::taxonomy_source_vo::ConfigSource;
use shared::source_parsing::taxonomy_path_vo::FilePath;

pub struct ConfigYamlReader;

impl ConfigYamlReader {
    pub fn new() -> Self {
        Self
    }

    fn config_filename(language: &str) -> String {
        format!("lint_arwaky.config.{}.yaml", language)
    }

    /// Get XDG data directory for lint-arwaky ($XDG_DATA_HOME/lint-arwaky/)
    pub fn data_dir() -> Option<std::path::PathBuf> {
        dirs::data_dir().map(|d| d.join("lint-arwaky"))
    }

    /// Get XDG cache directory for lint-arwaky ($XDG_CACHE_HOME/lint-arwaky/)
    pub fn cache_dir() -> Option<std::path::PathBuf> {
        dirs::cache_dir().map(|d| d.join("lint-arwaky"))
    }

    /// Get XDG state directory for lint-arwaky ($XDG_STATE_HOME/lint-arwaky/)
    pub fn state_dir() -> Option<std::path::PathBuf> {
        dirs::state_dir().map(|d| d.join("lint-arwaky"))
    }

    /// Read config from XDG-compliant directories in priority order.
    /// Called after project-local lookup fails. Returns `None` to fall back to
    /// compiled-in defaults.
    ///
    /// Priority order (XDG Base Directory Specification):
    /// 1) `$XDG_CONFIG_HOME/lint-arwaky/`  (default `~/.config/lint-arwaky/`)
    /// 2) Each dir in `$XDG_CONFIG_DIRS`    (default `/etc/xdg/lint-arwaky/`)
    fn read_any(language: &str) -> Option<ConfigSource> {
        let filename = Self::config_filename(language);
        let mut candidates: Vec<std::path::PathBuf> = Vec::new();

        // Priority 1: XDG user config dir — $XDG_CONFIG_HOME (default ~/.config)
        if let Some(user_config) = dirs::config_dir() {
            candidates.push(user_config.join("lint-arwaky").join(&filename));
        }

        // Priority 2: XDG system config dirs — $XDG_CONFIG_DIRS (default /etc/xdg)
        // dirs crate doesn't expose config_dirs(), so parse env var manually
        let system_dirs = match std::env::var("XDG_CONFIG_DIRS") {
            Ok(dirs) => dirs,
            Err(_) => "/etc/xdg".to_string(),
        };
        for dir in system_dirs.split(':').filter(|s| !s.is_empty()) {
            candidates.push(
                std::path::PathBuf::from(dir)
                    .join("lint-arwaky")
                    .join(&filename),
            );
        }

        for path in &candidates {
            if path.exists() {
                if let Ok(content) = std::fs::read_to_string(path) {
                    return Some(ConfigSource::new(
                        language,
                        path.to_string_lossy().to_string(),
                        content,
                    ));
                }
            }
        }
        None
    }
}

impl Default for ConfigYamlReader {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl IConfigReaderPort for ConfigYamlReader {
    /// Read config with priority order:
    /// 1) Project root directory
    /// 2) Parent of project root
    /// 3) XDG config dirs ($XDG_CONFIG_HOME, $XDG_CONFIG_DIRS)
    async fn read_config(&self, project_root: &FilePath, language: &str) -> Option<ConfigSource> {
        let filename = Self::config_filename(language);
        let mut current = std::path::PathBuf::from(&project_root.value);

        while !current.as_os_str().is_empty() {
            let candidate = current.join(&filename);
            if let Ok(content) = std::fs::read_to_string(&candidate) {
                return Some(ConfigSource::new(
                    language,
                    candidate.to_string_lossy().to_string(),
                    content,
                ));
            }

            if let Some(parent) = current.parent() {
                current = parent.to_path_buf();
            } else {
                break;
            }
        }

        // Fallback: XDG config dirs
        Self::read_any(language)
    }

    async fn list_config_files(&self, project_root: &FilePath) -> Vec<(String, String)> {
        let mut found = Vec::new();
        for lang in &["rust", "python", "javascript", "typescript"] {
            if let Some(config) = self.read_config(project_root, lang).await {
                found.push((lang.to_string(), config.path.to_string()));
            }
        }
        found
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xdg_config_dir() {
        let config_dir = dirs::config_dir();
        assert!(config_dir.is_some());
    }

    #[test]
    fn test_xdg_directories() {
        let data_dir = ConfigYamlReader::data_dir();
        assert!(data_dir.is_some());
        let data_path = data_dir.unwrap();
        assert!(data_path.ends_with("lint-arwaky"));

        let cache_dir = ConfigYamlReader::cache_dir();
        assert!(cache_dir.is_some());
        let cache_path = cache_dir.unwrap();
        assert!(cache_path.ends_with("lint-arwaky"));

        let state_dir = ConfigYamlReader::state_dir();
        assert!(state_dir.is_some());
        let state_path = state_dir.unwrap();
        assert!(state_path.ends_with("lint-arwaky"));
    }

    #[test]
    fn test_config_filename() {
        assert_eq!(
            ConfigYamlReader::config_filename("rust"),
            "lint_arwaky.config.rust.yaml"
        );
        assert_eq!(
            ConfigYamlReader::config_filename("python"),
            "lint_arwaky.config.python.yaml"
        );
    }
}
