use async_trait::async_trait;
use futures::future::join_all;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::contract_multi_project_orchestrator_aggregate::MultiProjectOrchestratorAggregate;
use shared::config_system::contract_reader_protocol::IConfigReaderProtocol;
use shared::config_system::contract_workspace_detector_protocol::IWorkspaceDetectorProtocol;
use shared::config_system::taxonomy_config_vo::default_config_for_language;
use shared::config_system::taxonomy_config_vo::parse_config_yaml;
use shared::config_system::taxonomy_multi_project_workspace_info_vo::WorkspaceInfo;
use std::sync::Arc;

pub struct MultiProjectOrchestrator {
    workspace_detector: Arc<dyn IWorkspaceDetectorProtocol>,
    config_reader: Arc<dyn IConfigReaderProtocol>,
}

impl MultiProjectOrchestrator {
    pub fn new(
        workspace_detector: Arc<dyn IWorkspaceDetectorProtocol>,
        config_reader: Arc<dyn IConfigReaderProtocol>,
    ) -> Self {
        Self {
            workspace_detector,
            config_reader,
        }
    }

    fn collect_subdirs(dir: &std::path::Path) -> Vec<FilePath> {
        let mut results = Vec::new();
        let entries = match std::fs::read_dir(dir) {
            Ok(e) => e,
            Err(e) => {
                eprintln!(
                    "Warning: Failed to read directory '{}': {}",
                    dir.display(),
                    e
                );
                return results;
            }
        };
        for entry in entries {
            match entry {
                Ok(entry) => {
                    if let Ok(ft) = entry.file_type() {
                        if ft.is_dir() {
                            let sub = entry.path();
                            if let Ok(fp) = FilePath::new(sub.to_string_lossy().to_string()) {
                                results.push(fp);
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!(
                        "Warning: Failed to read directory entry in '{}': {}",
                        dir.display(),
                        e
                    );
                }
            }
        }
        results
    }

    fn scan_workspace_dirs(root: &std::path::Path) -> Vec<FilePath> {
        let workspace_dirs = ["crates", "packages", "modules"];

        // Case 1: root itself is a workspace-member root (e.g. "crates/")
        let is_root_workspace_dir = match root.file_name() {
            Some(name) => {
                let name_str = name.to_string_lossy();
                workspace_dirs.contains(&name_str.as_ref())
            }
            None => false,
        };

        if is_root_workspace_dir {
            return Self::collect_subdirs(root);
        }

        // Case 2: root is itself a single member (e.g. "crates/import-rules/")
        if let Some(parent) = root.parent() {
            if let Some(parent_name) = parent.file_name() {
                let parent_str = parent_name.to_string_lossy();
                if workspace_dirs.contains(&parent_str.as_ref()) && root.is_dir() {
                    if let Ok(fp) = FilePath::new(root.to_string_lossy().to_string()) {
                        return vec![fp];
                    }
                }
            }
        }

        // Case 3: collect all members under workspace directories
        let mut results = Vec::new();
        for dir in &workspace_dirs {
            let dir_path = root.join(dir);
            if dir_path.is_dir() {
                results.extend(Self::collect_subdirs(&dir_path));
            }
        }
        results
    }
}

#[async_trait]
impl MultiProjectOrchestratorAggregate for MultiProjectOrchestrator {
    async fn discover_workspaces(&self, root: &FilePath) -> Vec<WorkspaceInfo> {
        let root_path = std::path::Path::new(&root.value);
        let workspaces = Self::scan_workspace_dirs(root_path);

        if workspaces.is_empty() {
            eprintln!(
                "Warning: No AES-compliant workspace members (crates/, packages/, or modules/) found in '{}'. \
                This system mandates a multi-module structure. Please refactor your project.",
                root.value
            );
            return Vec::new();
        }

        let futures = workspaces.iter().map(|ws| {
            let ws = ws.clone();
            let detector = self.workspace_detector.clone();
            let reader = self.config_reader.clone();
            async move {
                let ws_type = detector.detect(&ws);
                let language = ws_type.as_str();
                let config = match reader.read_config(&ws, language).await {
                    Some(source) => {
                        let mut parsed = parse_config_yaml(&source.raw_content);
                        if parsed.layers.is_empty() {
                            parsed.layers = default_config_for_language(language).layers;
                        }
                        parsed
                    }
                    None => default_config_for_language(language),
                };
                WorkspaceInfo::new(ws, language.to_string(), config)
            }
        });

        join_all(futures).await
    }
}
