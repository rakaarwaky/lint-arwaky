// PURPOSE: ImportContainer — wiring for import-rules feature (root layer, wiring only)
use crate::agent_import_orchestrator::{ImportOrchestrator, ImportOrchestratorDeps};
use shared_common::FilePath;
use shared_config_system::{ArchitectureConfig, IConfigOrchestratorAggregate};

use shared_filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared_filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use shared_filesystem::contract_filesystem_protocol::IParserProtocol;
use shared_filesystem::contract_filesystem_protocol::IWorkspaceProtocol;
use shared_import_rules::IImportRunnerAggregate;
use std::sync::Arc;

pub struct ImportContainer {
    config: ArchitectureConfig,
    filesystem: Arc<dyn IFilesystemAggregate>,
    filesystem_io: Arc<dyn IFileSystemIOProtocol>,
    filesystem_workspace: Arc<dyn IWorkspaceProtocol>,
    filesystem_parser: Arc<dyn IParserProtocol>,
}

impl ImportContainer {
    pub fn new_with_config(
        config: ArchitectureConfig,
        filesystem: Arc<dyn IFilesystemAggregate>,
        filesystem_io: Arc<dyn IFileSystemIOProtocol>,
        filesystem_workspace: Arc<dyn IWorkspaceProtocol>,
        filesystem_parser: Arc<dyn IParserProtocol>,
    ) -> Self {
        Self {
            config,
            filesystem,
            filesystem_io,
            filesystem_workspace,
            filesystem_parser,
        }
    }

    /// Create from config orchestrator — the canonical way per AES architecture.
    pub fn from_orchestrator(
        orchestrator: &Arc<dyn IConfigOrchestratorAggregate>,
        project_root: &str,
        filesystem: Arc<dyn IFilesystemAggregate>,
        filesystem_io: Arc<dyn IFileSystemIOProtocol>,
        filesystem_workspace: Arc<dyn IWorkspaceProtocol>,
        filesystem_parser: Arc<dyn IParserProtocol>,
    ) -> Self {
        let fp = FilePath::new(project_root.to_string()).unwrap_or_default();
        let config = orchestrator
            .execute(shared_config_system::ConfigRequest::load_sync(&fp))
            .into_sync_config();
        Self::new_with_config(
            config,
            filesystem,
            filesystem_io,
            filesystem_workspace,
            filesystem_parser,
        )
    }

    pub fn orchestrator(&self) -> Arc<dyn IImportRunnerAggregate> {
        let ignored_paths: Vec<String> = self
            .config
            .ignored_paths
            .values
            .iter()
            .map(|fp| fp.value.clone())
            .collect();
        Arc::new(ImportOrchestrator::new(
            ImportOrchestratorDeps {
                mandatory: Arc::new(
                    crate::capabilities_import_mandatory_checker::ArchImportMandatoryChecker::new(),
                ),
                forbidden: Arc::new(
                    crate::capabilities_import_forbidden_checker::ArchImportForbiddenChecker::new(),
                ),
                unused: Arc::new(
                    crate::capabilities_import_unused_checker::UnusedImportRuleChecker::new(),
                ),
                cycle: Arc::new(
                    crate::capabilities_cycle_import_analyzer::DependencyCycleAnalyzer::new(),
                ),
                dummy: Arc::new(
                    crate::capabilities_dummy_import_checker::DummyImportChecker::new(),
                ),
                filesystem: self.filesystem.clone(),
                filesystem_io: self.filesystem_io.clone(),
                filesystem_workspace: self.filesystem_workspace.clone(),
                filesystem_parser: self.filesystem_parser.clone(),
            },
            self.config.clone(),
            ignored_paths,
        ))
    }
}
