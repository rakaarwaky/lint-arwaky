use crate::agent_config_orchestrator::{ConfigOrchestrator, ConfigOrchestratorDeps};
use crate::capabilities_parser_provider::ConfigParserProvider;
use crate::capabilities_workspace_detector::WorkspaceDetector;
use crate::capabilities_yaml_reader::ConfigYamlReader;
// Utility module wired into entry for orphan reachability (AES504)
use shared_config_system::utility_config_parser;
use shared_config_system::{
    IConfigMergeProtocol, IConfigOrchestratorAggregate, IConfigReadProtocol,
    IWorkspaceMembersProtocol,
};
use shared_filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared_filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;

use std::sync::Arc;

pub struct ConfigContainer {
    orchestrator: Arc<dyn IConfigOrchestratorAggregate>,
    reader: Arc<dyn IConfigReadProtocol>,
    parser: Arc<dyn IConfigMergeProtocol>,
    detector: Arc<dyn IWorkspaceMembersProtocol>,
}

impl ConfigContainer {
    /// Create a new config container, wiring all capabilities to the filesystem aggregate.
    pub fn new(
        filesystem: Arc<dyn IFilesystemAggregate>,
        io: Arc<dyn IFileSystemIOProtocol>,
    ) -> Self {
        let workspace_detector = Arc::new(WorkspaceDetector::new(io.clone()));
        let yaml_reader = Arc::new(ConfigYamlReader::new(io.clone()));
        let parser = Arc::new(ConfigParserProvider::new(io));

        Self {
            orchestrator: Arc::new(ConfigOrchestrator::new(ConfigOrchestratorDeps {
                workspace_detector: workspace_detector.clone(),
                config_reader: yaml_reader.clone(),
                parser: parser.clone(),
                filesystem,
            })),
            reader: yaml_reader,
            parser,
            detector: workspace_detector,
        }
    }

    pub fn orchestrator(&self) -> Arc<dyn IConfigOrchestratorAggregate> {
        self.orchestrator.clone()
    }

    pub fn reader(&self) -> Arc<dyn IConfigReadProtocol> {
        self.reader.clone()
    }

    pub fn parser(&self) -> Arc<dyn IConfigMergeProtocol> {
        self.parser.clone()
    }

    pub fn lister(&self) -> Arc<dyn IConfigReadProtocol> {
        self.reader.clone()
    }

    pub fn detector(&self) -> Arc<dyn IWorkspaceMembersProtocol> {
        self.detector.clone()
    }

    /// Get default AES configuration (from shared parser).
    pub fn default_config(
        &self,
    ) -> shared_config_system::taxonomy_config_system_vo::ArchitectureConfig {
        shared_config_system::utility_config_parser::default_aes_config()
    }

    /// Parse score threshold from YAML (uses utility_config_parser).
    pub fn parse_score_threshold(&self, yaml_str: &str) -> Option<f64> {
        utility_config_parser::parse_score_threshold(yaml_str)
    }
}
