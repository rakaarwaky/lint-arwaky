// PURPOSE: SetupContainer — wiring for project-setup feature (root layer, wiring only)

use shared::filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use shared::project_setup::{
    IAdapterInstallationProtocol, IEnvGenerationProtocol, ILanguageDetectionProtocol,
    IMcpConfigGenerationProtocol, ISetupAggregate,
};

use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct SetupContainer {
    aggregate: Arc<dyn ISetupAggregate>,
    mcp_config: Arc<dyn IMcpConfigGenerationProtocol>,
    env_generation: Arc<dyn IEnvGenerationProtocol>,
    language_detection: Arc<dyn ILanguageDetectionProtocol>,
    adapter_installation: Arc<dyn IAdapterInstallationProtocol>,
}

// ─── Block 2: Container Construction ──────────────────────

impl SetupContainer {
    pub fn new(io: Arc<dyn IFileSystemIOProtocol>) -> Self {
        let installer =
            Arc::new(crate::capabilities_setup_installer_adapter::SetupInstallerAdapter::new());
        let mcp_config =
            Arc::new(crate::capabilities_mcp_config_generator::SetupMcpConfigGenerator::new());
        let env_generation = Arc::new(crate::capabilities_env_generator::SetupEnvGenerator::new(
            io.clone(),
        ));
        let language_detection =
            Arc::new(crate::capabilities_language_detector::SetupLanguageDetector::new(io));
        let protocols = crate::agent_setup_orchestrator::SetupProtocols {
            mcp_config: mcp_config.clone(),
            env_generation: env_generation.clone(),
            language_detection: language_detection.clone(),
            adapter_installation: installer.clone(),
        };
        let aggregate =
            Arc::new(crate::agent_setup_orchestrator::SetupManagementOrchestrator::new(protocols));
        Self {
            aggregate,
            mcp_config: mcp_config.clone(),
            env_generation: env_generation.clone(),
            language_detection: language_detection.clone(),
            adapter_installation: installer.clone(),
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
}

// ─── Note: No Default impl ────────────────────────────────
// SetupContainer requires a filesystem instance — construction is only
// possible via SetupContainer::new(filesystem). Providing a Default impl
// would be a bypass violation (AES304) since there is no valid zero-arg
// construction path.
