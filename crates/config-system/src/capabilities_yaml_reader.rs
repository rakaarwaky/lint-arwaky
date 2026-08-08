use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::contract_reader_protocol::IConfigReaderProtocol;
use shared::config_system::taxonomy_config_error::ConfigError;
use shared::config_system::taxonomy_config_language_vo::ConfigLanguage;
use shared::config_system::taxonomy_source_vo::ConfigSource;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;

use tracing::warn;

// PURPOSE: ConfigYamlReader — reads and parses lint-arwaky YAML config files from disk
// XDG Base Directory Specification compliant config lookup
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ConfigYamlReader {
    filesystem: Arc<dyn IFilesystemAggregate>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IConfigReaderProtocol for ConfigYamlReader {
    fn read_config(
        &self,
        project_root: &FilePath,
        language: ConfigLanguage,
    ) -> Result<Option<ConfigSource>, ConfigError> {
        // Try local project files first (in priority order)
        for filename in language.config_file_names() {
            let mut current = std::path::PathBuf::from(&project_root.value);
            let mut depth = 0;

            while !current.as_os_str().is_empty() && depth < 5 {
                let candidate = current.join(filename);
                // FR-001: Reject symlinks pointing outside project root
                if let Ok(meta) = self.filesystem.symlink_metadata(&candidate)
                    && meta.file_type().is_symlink()
                    && let Ok(canonical) = self.filesystem.canonicalize(&candidate)
                {
                    let root_canonical = self
                        .filesystem
                        .canonicalize(std::path::Path::new(&project_root.value))
                        .unwrap_or_else(|_| std::path::PathBuf::from(&project_root.value));
                    if !canonical.starts_with(&root_canonical) {
                        warn!(path = %candidate.display(), "symlink points outside project root, rejected");
                        if let Some(parent) = current.parent() {
                            current = parent.to_path_buf();
                        } else {
                            break;
                        }
                        depth += 1;
                        continue;
                    }
                }
                match self.filesystem.read_to_string(&candidate) {
                    Ok(content) => {
                        return Ok(Some(ConfigSource::new(
                            language.as_str(),
                            candidate.to_string_lossy().to_string(),
                            content.value,
                        )));
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                        // keep searching upward
                    }
                    Err(e) => {
                        warn!(path = %candidate.display(), error = %e, "failed to read config");
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
        self.read_any(language)
    }

    fn list_config_files(
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
                match self.filesystem.read_to_string(&candidate) {
                    Ok(_content) => {
                        let path = FilePath::new(candidate.to_string_lossy().to_string()).map_err(
                            |e| {
                                ConfigError::new(
                                    shared::config_system::taxonomy_identifier_vo::ConfigKey::new(
                                        "config.list",
                                    ),
                                    shared::common::ErrorMessage::new(format!(
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
                        warn!(path = %candidate.display(), error = %e, "failed to read config");
                    }
                }
            }
        }
        Ok(found)
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl ConfigYamlReader {
    /// Create a new YAML config reader with filesystem IO dependency.
    pub fn new(filesystem: Arc<dyn IFilesystemAggregate>) -> Self {
        Self { filesystem }
    }

    /// Read config from XDG-compliant directories in priority order.
    /// Returns `None` to fall back to compiled-in defaults.
    fn read_any(&self, language: ConfigLanguage) -> Result<Option<ConfigSource>, ConfigError> {
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
            match self.filesystem.read_to_string(path) {
                Ok(content) => {
                    return Ok(Some(ConfigSource::new(
                        language.as_str(),
                        path.to_string_lossy().to_string(),
                        content.value,
                    )));
                }
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => continue,
                Err(e) => {
                    warn!(path = %path.display(), error = %e, "failed to read config");
                }
            }
        }
        Ok(None)
    }
}
