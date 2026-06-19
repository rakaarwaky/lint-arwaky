// PURPOSE: MultiProjectOrchestrator — discovers workspaces and loads their configs
use async_trait::async_trait;
use shared::config_system::contract_reader_port::IConfigReaderPort;
use shared::config_system::contract_workspace_detector_port::IWorkspaceDetectorPort;
use shared::config_system::taxonomy_config_vo::default_config_for_language;
use shared::multi_project::contract_orchestrator_aggregate::MultiProjectOrchestratorAggregate;
use shared::multi_project::taxonomy_workspace_info_vo::WorkspaceInfo;
use shared::source_parsing::taxonomy_path_vo::FilePath;
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

    fn scan_workspace_dirs(root: &std::path::Path) -> Vec<FilePath> {
        let workspace_dirs = ["crates", "packages", "services", "modules"];
        let mut results = Vec::new();

        for dir in &workspace_dirs {
            let dir_path = root.join(dir);
            if dir_path.is_dir() {
                if let Ok(entries) = std::fs::read_dir(&dir_path) {
                    for entry in entries.flatten() {
                        let sub = entry.path();
                        if sub.is_dir() {
                            if let Ok(fp) = FilePath::new(sub.to_string_lossy().to_string()) {
                                results.push(fp);
                            }
                        }
                    }
                }
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

        let mut results = Vec::new();
        for ws in &workspaces {
            let ws_type = self.workspace_detector.detect(ws);
            let language = ws_type.as_str();

            let config = match self.config_reader.read_config(ws, language).await {
                Some(source) => {
                    let parsed = shared::config_system::taxonomy_config_vo::parse_config_yaml(
                        &source.raw_content,
                    );
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
