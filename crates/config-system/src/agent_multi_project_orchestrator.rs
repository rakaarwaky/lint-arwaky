use async_trait::async_trait;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::contract_multi_project_orchestrator_aggregate::MultiProjectOrchestratorAggregate;
use shared::config_system::contract_reader_port::IConfigReaderPort;
use shared::config_system::contract_workspace_detector_port::IWorkspaceDetectorPort;
use shared::config_system::taxonomy_config_vo::default_config_for_language;
use shared::config_system::taxonomy_config_vo::parse_config_yaml;
use shared::config_system::taxonomy_multi_project_workspace_info_vo::WorkspaceInfo;
use std::sync::Arc;

pub struct MultiProjectOrchestrator {
    workspace_detector: Arc<dyn IWorkspaceDetectorPort>,
    config_reader: Arc<dyn IConfigReaderPort>,
}

impl MultiProjectOrchestrator {
    pub fn new(
        workspace_detector: Arc<dyn IWorkspaceDetectorPort>,
        config_reader: Arc<dyn IConfigReaderPort>,
    ) -> Self {
        Self {
            workspace_detector,
            config_reader,
        }
    }

    fn collect_subdirs(dir: &std::path::Path) -> Vec<FilePath> {
        let mut results = Vec::new();
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let sub = entry.path();
                if sub.is_dir() {
                    if let Ok(fp) = FilePath::new(sub.to_string_lossy().to_string()) {
                        results.push(fp);
                    }
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

        let mut results = Vec::new();
        for ws in &workspaces {
            let ws_type = self.workspace_detector.detect(ws);
            let language = ws_type.as_str();

            let config = match self.config_reader.read_config(ws, language).await {
                Some(source) => {
                    let parsed = parse_config_yaml(&source.raw_content);
                    if !parsed.layers.is_empty() {
                        parsed
                    } else {
                        default_config_for_language(language)
                    }
                }
                None => default_config_for_language(language),
            };

            results.push(WorkspaceInfo::new(ws.clone(), language.to_string(), config));
        }

        results
    }
}
