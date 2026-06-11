// PURPOSE: AppConfigVO, AppName, AppVersion — value objects for application configuration metadata
use std::env;

use crate::config_system::taxonomy_adapter_vo::AdapterNameList;
use crate::config_system::taxonomy_setting_vo::{AdapterStatus, ProjectConfig, Thresholds};
use crate::shared_common::taxonomy_common_vo::BooleanVO;
use crate::source_parsing::taxonomy_path_vo::DirectoryPath;

/// app_config_vo — Unified configuration for the application.
///
/// Unified configuration — transport, paths, and project settings.
#[derive(Debug, Clone)]
pub struct AppConfig {
    phantom_root: DirectoryPath,
    project: ProjectConfig,
}

impl AppConfig {
    /// Create a new AppConfig.
    ///
    /// # Arguments
    /// * `phantom_root` - Optional phantom root directory. Defaults to environment variable `PHANTOM_ROOT` or home directory.
    /// * `project_root` - Optional project root directory. Defaults to environment variable `PROJECT_ROOT` or current directory.
    /// * `project` - Optional project configuration. Defaults to `crate::config_system::taxonomy_setting_vo::ProjectConfig::default()`.
    pub fn create(
        phantom_root: Option<String>,
        project_root: Option<String>,
        project: Option<ProjectConfig>,
    ) -> Self {
        let p_root = phantom_root
            .or_else(|| env::var("PHANTOM_ROOT").ok())
            .unwrap_or_else(|| env::var("HOME").unwrap_or_else(|_| ".".to_string()));
        let _proj_root = project_root
            .or_else(|| env::var("PROJECT_ROOT").ok())
            .unwrap_or_else(|| {
                env::current_dir()
                    .map(|d| d.to_string_lossy().to_string())
                    .unwrap_or_else(|_| ".".to_string())
            });
        let proj = project.unwrap_or_default();

        Self {
            phantom_root: DirectoryPath::new(p_root).unwrap_or_default(),
            project: proj,
        }
    }

    /// Get the thresholds from the project configuration.
    pub fn thresholds(&self) -> &Thresholds {
        &self.project.thresholds
    }

    /// Get status for a named adapter.
    pub fn adapter_status(&self, name: &str) -> AdapterStatus {
        for entry in &self.project.adapters {
            if entry.name.value == name {
                return entry.status;
            }
        }
        AdapterStatus::NotInstalled
    }

    /// Check if an adapter is enabled.
    pub fn is_adapter_enabled(&self, name: &str) -> BooleanVO {
        let status = self.adapter_status(name);
        BooleanVO::new(status == AdapterStatus::Enabled)
    }

    /// Names of enabled adapters.
    pub fn active_adapters(&self) -> AdapterNameList {
        let mut values = Vec::new();
        for entry in &self.project.adapters {
            if entry.is_active() {
                values.push(entry.name.clone());
            }
        }
        AdapterNameList { values }
    }
}

impl std::fmt::Display for AppConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AppConfig(phantom={}, adapters={:?})",
            self.phantom_root,
            self.active_adapters()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::AppConfig;
    use super::ProjectConfig;
    use std::env;

    #[test]
    fn test_app_config_create() {
        let config = AppConfig::create(
            Some("/phantom".to_string()),
            Some("/project".to_string()),
            Some(ProjectConfig::default()),
        );
        assert_eq!(config.phantom_root.to_string(), "/phantom");
    }

    #[test]
    fn test_app_config_defaults() {
        // Set environment variables for deterministic test
        env::set_var("PHANTOM_ROOT", "/test/phantom");
        env::set_var("PROJECT_ROOT", "/test/project");
        let config = AppConfig::create(None, None, None);
        assert_eq!(config.phantom_root.to_string(), "/test/phantom");
        // Clean up
        env::remove_var("PHANTOM_ROOT");
        env::remove_var("PROJECT_ROOT");
    }
}
