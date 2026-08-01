use shared::common::{DescriptionVO, FilePath, SuccessStatus};

use shared::git_hooks::{IHookManagerProtocol, IHookProtocol};

use shared::git_hooks::GitHookError;
use shared::git_hooks::{GitDiffDataVO, GitDiffSideVO, GitDiffStatus, HookIgnoreUpdateVO};
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use std::sync::Arc;

// PURPOSE: HookManager — implements IHookProtocol for git hook management (capabilities layer)
use shared::common::Identity;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct HookManager {
    hook_adapter: Arc<dyn IHookManagerProtocol>,
    filesystem: Arc<dyn IFilesystemAggregate>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

#[async_trait::async_trait]
impl IHookProtocol for HookManager {
    async fn install_pre_commit(
        &self,
        executable_path: &FilePath,
    ) -> Result<SuccessStatus, GitHookError> {
        self.hook_adapter.install_pre_commit(executable_path)
    }

    async fn uninstall_pre_commit(&self) -> Result<SuccessStatus, GitHookError> {
        self.hook_adapter.uninstall_pre_commit()
    }

    fn get_hook_manager_identity(&self) -> Identity {
        Identity::new("git_hook_manager")
    }

    async fn initialize_config(&self, path: &str) -> DescriptionVO {
        let config_file = format!("{}/lint_arwaky.config.yaml", path);
        if self.filesystem.path_exists(&std::path::Path::new(&config_file)) {
            return DescriptionVO::new(format!("ALREADY_EXISTS:{}", config_file));
        }
        DescriptionVO::new(format!("Initialized {}", config_file))
    }

    fn update_ignore_rule(&self, request: HookIgnoreUpdateVO) -> DescriptionVO {
        let config_path = std::path::Path::new(&request.config_path);
        if !config_path.exists() {
            return DescriptionVO::new(format!(
                "Config file not found: {}. Run lint-arwaky-cli init first.",
                request.config_path
            ));
        }

        // Read YAML config
        let content = match self.filesystem.read_to_string(config_path) {
            Ok(c) => c,
            Err(e) => {
                return DescriptionVO::new(format!("Failed to read config: {}", e));
            }
        };

        // Parse as generic YAML value
        let mut doc: serde_yaml_ng::Value = match serde_yaml_ng::from_str(&content) {
            Ok(v) => v,
            Err(e) => {
                return DescriptionVO::new(format!("Failed to parse config YAML: {}", e));
            }
        };

        // Get or create the ignored_paths list
        let ignored_paths = doc
            .as_mapping_mut()
            .and_then(|m| m.get_mut(serde_yaml_ng::Value::String("ignored_paths".to_string())))
            .and_then(|v| v.as_sequence_mut());

        let ignored_paths = match ignored_paths {
            Some(p) => p,
            None => {
                return DescriptionVO::new("Config missing 'ignored_paths' key".to_string());
            }
        };

        let rule_value = serde_yaml_ng::Value::String(request.rule.clone());

        if request.remove {
            let before_len = ignored_paths.len();
            ignored_paths.retain(|v| v != &rule_value);
            if ignored_paths.len() == before_len {
                return DescriptionVO::new(format!("'{}' not found in ignore list", request.rule));
            }
        } else {
            if ignored_paths.contains(&rule_value) {
                return DescriptionVO::new(format!(
                    "'{}' already present in ignore list",
                    request.rule
                ));
            }
            ignored_paths.push(rule_value);
        }

        // Write back
        match serde_yaml_ng::to_string(&doc) {
            Ok(yaml_str) => {
                if let Err(e) = self.filesystem.write_string(config_path, &yaml_str) {
                    return DescriptionVO::new(format!("Failed to write config: {}", e));
                }
                let verb = if request.remove { "Removed" } else { "Added" };
                DescriptionVO::new(format!("{} '{}' from ignore list", verb, request.rule))
            }
            Err(e) => DescriptionVO::new(format!("Failed to serialize config: {}", e)),
        }
    }

    async fn get_diff_data(&self, path1: &str, path2: &str) -> GitDiffDataVO {
        let p1_exists = self.filesystem.path_exists(std::path::Path::new(path1));
        let p2_exists = self.filesystem.path_exists(std::path::Path::new(path2));

        // FR-005: Status determined by file existence
        if !p1_exists && !p2_exists {
            return GitDiffDataVO {
                version1: GitDiffSideVO::new(path1.to_string(), 1.0),
                version2: GitDiffSideVO::new(path2.to_string(), 1.0),
                difference: 0.0,
                status: GitDiffStatus::MissingFirst,
            };
        }
        if !p1_exists {
            return GitDiffDataVO {
                version1: GitDiffSideVO::new(path1.to_string(), 1.0),
                version2: GitDiffSideVO::new(path2.to_string(), 1.0),
                difference: 0.0,
                status: GitDiffStatus::MissingFirst,
            };
        }
        if !p2_exists {
            return GitDiffDataVO {
                version1: GitDiffSideVO::new(path1.to_string(), 1.0),
                version2: GitDiffSideVO::new(path2.to_string(), 1.0),
                difference: 0.0,
                status: GitDiffStatus::MissingSecond,
            };
        }

        let p1_is_file = self.filesystem.is_file(std::path::Path::new(path1));
        let p2_is_file = self.filesystem.is_file(std::path::Path::new(path2));

        if !p1_is_file || !p2_is_file {
            return GitDiffDataVO {
                version1: GitDiffSideVO::new(path1.to_string(), 1.0),
                version2: GitDiffSideVO::new(path2.to_string(), 1.0),
                difference: 0.0,
                status: GitDiffStatus::NotAFile,
            };
        }

        // Both exist and are files — compute byte-level difference score
        let (score, status) = match self.compute_diff_score(path1, path2) {
            Ok(s) if s == 0.0 => (s, GitDiffStatus::Unchanged),
            Ok(s) => (s, GitDiffStatus::Modified),
            Err(_) => (1.0, GitDiffStatus::Modified), // read failure → assume changed
        };

        GitDiffDataVO {
            version1: GitDiffSideVO::new(path1.to_string(), 1.0),
            version2: GitDiffSideVO::new(path2.to_string(), 1.0),
            difference: score,
            status,
        }
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl HookManager {
    pub fn new(hook_adapter: Arc<dyn IHookManagerProtocol>) -> Self {
        Self {
            hook_adapter,
            filesystem: Arc::new(filesystem::FilesystemOrchestrator::new()),
        }
    }

    /// Compute byte-level difference score between two files.
    /// Score = 1.0 − (matching bytes / max file size).
    /// Same file path → 0.0 (identical).
    fn compute_diff_score(&self, path1: &str, path2: &str) -> Result<f64, std::io::Error> {
        if path1 == path2 {
            return Ok(0.0);
        }
        let bytes1 = self.filesystem.read_to_string(std::path::Path::new(path1)).map(|s| s.into_bytes()).unwrap_or_default();
        let bytes2 = self.filesystem.read_to_string(std::path::Path::new(path2)).map(|s| s.into_bytes()).unwrap_or_default();
        let max_size = bytes1.len().max(bytes2.len());
        if max_size == 0 {
            return Ok(0.0); // both empty
        }
        let matching = bytes1
            .iter()
            .zip(bytes2.iter())
            .filter(|(a, b)| a == b)
            .count();
        Ok(1.0 - (matching as f64 / max_size as f64))
    }
}
