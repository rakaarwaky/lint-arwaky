// PURPOSE: ConfigYamlReader — reads and parses lint-arwaky YAML config files from disk
// XDG Base Directory Specification compliant config lookup
use async_trait::async_trait;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::contract_reader_protocol::IConfigReaderProtocol;
use shared::config_system::taxonomy_source_vo::ConfigSource;

pub struct ConfigYamlReader;

impl ConfigYamlReader {
    pub fn new() -> Self {
        Self
    }

    pub fn config_filename(language: &str) -> String {
        format!("lint_arwaky.config.{}.yaml", language)
    }

    /// Read config from XDG-compliant directories in priority order.
    /// Returns `None` to fall back to compiled-in defaults.
    async fn read_any(language: &str) -> Option<ConfigSource> {
        let filename = Self::config_filename(language);
        let mut candidates: Vec<std::path::PathBuf> = Vec::new();

        if let Some(user_config) = dirs::config_dir() {
            candidates.push(user_config.join("lint-arwaky").join(&filename));
        }

        let system_dirs = match std::env::var("XDG_CONFIG_DIRS") {
            Ok(dirs) if !dirs.is_empty() => dirs,
            _ => "/etc/xdg".to_string(),
        };
        for dir in system_dirs.split(':').filter(|s| !s.is_empty()) {
            candidates.push(
                std::path::PathBuf::from(dir)
                    .join("lint-arwaky")
                    .join(&filename),
            );
        }

        for path in &candidates {
            match tokio::fs::read_to_string(path).await {
                Ok(content) => {
                    return Some(ConfigSource::new(
                        language,
                        path.to_string_lossy().to_string(),
                        content,
                    ));
                }
                Err(_) => continue,
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
impl IConfigReaderProtocol for ConfigYamlReader {
    async fn read_config(&self, project_root: &FilePath, language: &str) -> Option<ConfigSource> {
        let filename = Self::config_filename(language);
        let mut current = std::path::PathBuf::from(&project_root.value);
        let mut depth = 0;

        while !current.as_os_str().is_empty() && depth < 2 {
            let candidate = current.join(&filename);
            if let Ok(content) = tokio::fs::read_to_string(&candidate).await {
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
            depth += 1;
        }

        Self::read_any(language).await
    }

    async fn list_config_files(&self, project_root: &FilePath) -> Vec<(String, String)> {
        let mut found = Vec::new();
        for lang in &["rust", "python", "typescript"] {
            if let Some(config) = self.read_config(project_root, lang).await {
                found.push((lang.to_string(), config.path.to_string()));
            }
        }
        found
    }
}
