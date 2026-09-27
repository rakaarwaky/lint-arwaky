// PURPOSE: SetupContainer — wiring for project-setup feature (root layer, wiring only)

use shared::filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use shared::project_setup::{
    IAdapterInstallationProtocol, IConfigTemplateProtocol, IConfigWritingProtocol,
    IEnvGenerationProtocol, IFilePathExistenceProtocol, ILanguageDetectionProtocol,
    IMcpConfigGenerationProtocol, IPreFlightProtocol, ISetupAggregate,
};

use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct SetupContainer {
    aggregate: Arc<dyn ISetupAggregate>,
    mcp_config: Arc<dyn IMcpConfigGenerationProtocol>,
    env_generation: Arc<dyn IEnvGenerationProtocol>,
    language_detection: Arc<dyn ILanguageDetectionProtocol>,
    adapter_installation: Arc<dyn IAdapterInstallationProtocol>,
    config_template: Arc<dyn IConfigTemplateProtocol>,
    config_writing: Arc<dyn IConfigWritingProtocol>,
    pre_flight: Arc<dyn IPreFlightProtocol>,
    path_existence: Arc<dyn IFilePathExistenceProtocol>,
}

// ─── Block 2: Container Construction ──────────────────────

impl SetupContainer {
    pub fn new(io: Arc<dyn IFileSystemIOProtocol>) -> Self {
        let installer =
            Arc::new(crate::capabilities_setup_installer_adapter::SetupInstallerAdapter::new());
        let processor =
            Arc::new(crate::capabilities_setup_processor::SetupManagementProcessor::new(io));
        let protocols = crate::agent_setup_orchestrator::SetupProtocols {
            mcp_config: processor.clone(),
            env_generation: processor.clone(),
            language_detection: processor.clone(),
            adapter_installation: installer.clone(),
            config_template: processor.clone(),
            config_writing: processor.clone(),
            pre_flight: processor.clone(),
            path_existence: processor.clone(),
        };
        let aggregate =
            Arc::new(crate::agent_setup_orchestrator::SetupManagementOrchestrator::new(protocols));
        Self {
            aggregate,
            mcp_config: processor.clone(),
            env_generation: processor.clone(),
            language_detection: processor.clone(),
            adapter_installation: installer.clone(),
            config_template: processor.clone(),
            config_writing: processor.clone(),
            pre_flight: processor.clone(),
            path_existence: processor,
        }
    }

    pub fn aggregate(&self) -> Arc<dyn ISetupAggregate> {
        self.aggregate.clone()
    }

    pub fn mcp_config(&self) -> &Arc<dyn IMcpConfigGenerationProtocol> {
        &self.mcp_config
    }

    pub fn env_generation(&self) -> &Arc<dyn IEnvGenerationProtocol> {
        &self.env_generation
    }

    pub fn language_detection(&self) -> &Arc<dyn ILanguageDetectionProtocol> {
        &self.language_detection
    }

    pub fn adapter_installation(&self) -> &Arc<dyn IAdapterInstallationProtocol> {
        &self.adapter_installation
    }

    pub fn config_template(&self) -> &Arc<dyn IConfigTemplateProtocol> {
        &self.config_template
    }

    pub fn config_writing(&self) -> &Arc<dyn IConfigWritingProtocol> {
        &self.config_writing
    }

    pub fn pre_flight(&self) -> &Arc<dyn IPreFlightProtocol> {
        &self.pre_flight
    }

    pub fn path_existence(&self) -> &Arc<dyn IFilePathExistenceProtocol> {
        &self.path_existence
    }
}

// ─── Note: No Default impl ────────────────────────────────
// SetupContainer requires a filesystem instance — construction is only
// possible via SetupContainer::new(filesystem). Providing a Default impl
// would be a bypass violation (AES304) since there is no valid zero-arg
// construction path.
