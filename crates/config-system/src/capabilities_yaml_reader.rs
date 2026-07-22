use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::contract_reader_protocol::IConfigReaderProtocol;
use shared::config_system::taxonomy_config_error::ConfigError;
use shared::config_system::taxonomy_config_language_vo::ConfigLanguage;
use shared::config_system::taxonomy_source_vo::ConfigSource;
use shared::config_system::utility_config_io as config_io;

// PURPOSE: ConfigYamlReader — reads and parses lint-arwaky YAML config files from disk
// XDG Base Directory Specification compliant config lookup
use async_trait::async_trait;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ConfigYamlReader;

// ─── Block 2: Protocol Trait Implementation ───────────────

#[async_trait]
impl IConfigReaderProtocol for ConfigYamlReader {
    async fn read_config(
        &self,
        project_root: &FilePath,
        language: ConfigLanguage,
    ) -> Result<Option<ConfigSource>, ConfigError> {
        // Try local project files first (in priority order)
        for filename in language.config_file_names() {
            let mut current = std::path::PathBuf::from(&project_root.value);
            let mut depth = 0;

            while !current.as_os_str().is_empty() && depth < 3 {
                let candidate = current.join(filename);
                match config_io::read_file_async(&candidate).await {
                    Ok(content) => {
                        return Ok(Some(ConfigSource::new(
                            language.as_str(),
                            candidate.to_string_lossy().to_string(),
                            content,
                        )));
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                        // keep searching upward
                    }
                    Err(e) => {
                        eprintln!(
                            "Warning: Failed to read config '{}': {}",
                            candidate.display(),
                            e
                        );
                    }
                }

                if let Some(parent) = current.parent() {
                    current = parent.to_path_buf();
                } else {
                    break;
                }
                depth += 1;
            }
        }

        // Fall back to XDG-compliant directories
        Self::read_any(language).await
    }

    async fn list_config_files(
        &self,
        project_root: &FilePath,
    ) -> Result<Vec<(ConfigLanguage, FilePath)>, ConfigError> {
        let mut found = Vec::new();
        for lang in &[
            ConfigLanguage::Rust,
            ConfigLanguage::Python,
            ConfigLanguage::TypeScript,
        ] {
            for filename in lang.config_file_names() {
                let candidate = std::path::PathBuf::from(&project_root.value).join(filename);
                match config_io::read_file_async(&candidate).await {
                    Ok(_content) => {
                        let path = FilePath::new(candidate.to_string_lossy().to_string()).map_err(
                            |e| {
                                ConfigError::new(
                                    shared::config_system::taxonomy_identifier_vo::ConfigKey::new(
                                        "config.list",
                                    ),
                                    shared::taxonomy_common_error::ErrorMessage::new(format!(
                                        "Failed to create FilePath: {}",
                                        e
                                    )),
                                )
                            },
                        )?;
                        if !found.iter().any(|(_, p)| *p == path) {
                            found.push((*lang, path));
                        }
                        break;
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::NotFound => continue,
                    Err(e) => {
                        eprintln!(
                            "Warning: Failed to list config '{}': {}",
                            candidate.display(),
                            e
                        );
                    }
                }
            }
        }
        Ok(found)
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for ConfigYamlReader {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigYamlReader {
    pub fn new() -> Self {
        Self { _p: PhantomData }
    }

    /// Read config from XDG-compliant directories in priority order.
    /// Returns `None` to fall back to compiled-in defaults.
    async fn read_any(language: ConfigLanguage) -> Result<Option<ConfigSource>, ConfigError> {
        let mut candidates: Vec<std::path::PathBuf> = Vec::new();

        if let Some(user_config) = dirs::config_dir() {
            for filename in language.config_file_names() {
                candidates.push(user_config.join("lint-arwaky").join(filename));
            }
        }

        // Harden XDG_CONFIG_DIRS: limit to 8 entries, require absolute paths
        if let Ok(system_dirs) = std::env::var("XDG_CONFIG_DIRS") {
            if !system_dirs.is_empty() {
                for dir in system_dirs.split(':').filter(|s| !s.is_empty()).take(8) {
                    let path = std::path::PathBuf::from(dir);
                    if !path.is_absolute() {
                        continue;
                    }
                    for filename in language.config_file_names() {
                        candidates.push(path.join("lint-arwaky").join(filename));
                    }
                }
            }
        } else {
            // Default system XDG path
            for filename in language.config_file_names() {
                candidates.push(
                    std::path::PathBuf::from("/etc/xdg")
                        .join("lint-arwaky")
                        .join(filename),
                );
            }
        }

        for path in &candidates {
            match config_io::read_file_async(path).await {
                Ok(content) => {
                    return Ok(Some(ConfigSource::new(
                        language.as_str(),
                        path.to_string_lossy().to_string(),
                        content,
                    )));
                }
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => continue,
                Err(e) => {
                    eprintln!("Warning: Failed to read config '{}': {}", path.display(), e);
                }
            }
        }
        Ok(None)
    }
}
