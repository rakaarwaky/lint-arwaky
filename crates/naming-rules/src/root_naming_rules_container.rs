// PURPOSE: NamingContainer — wiring for naming-rules feature (root layer, wiring only)
use shared::code_analysis::contract_layer_detection_protocol::ILayerDetectionProtocol;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::naming_rules::contract_naming_checker_protocol::INamingCheckerProtocol;
use shared::naming_rules::contract_naming_filesystem_port::INamingFileSystemPort;
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use std::sync::Arc;

pub struct NamingContainer {
    naming_convention_checker: Arc<dyn INamingCheckerProtocol>,
    suffix_prefix_checker: Arc<dyn INamingCheckerProtocol>,
    analyzer: Arc<dyn ILayerDetectionProtocol>,
    fs: Arc<dyn INamingFileSystemPort>,
}

impl NamingContainer {
    pub fn new(analyzer: Arc<dyn ILayerDetectionProtocol>) -> Self {
        let naming_convention_checker: Arc<dyn INamingCheckerProtocol> =
            Arc::new(crate::capabilities_naming_convention_checker::NamingConventionChecker::new());
        let suffix_prefix_checker: Arc<dyn INamingCheckerProtocol> =
            Arc::new(crate::capabilities_suffix_prefix_checker::SuffixPrefixChecker::new());
        let fs: Arc<dyn INamingFileSystemPort> =
            Arc::new(crate::infrastructure_filesystem_adapter::OSFileSystemAdapter::new());
        Self {
            naming_convention_checker,
            suffix_prefix_checker,
            analyzer,
            fs,
        }
    }

    pub fn new_default() -> Self {
        Self::new(Arc::new(DefaultLayerDetector))
    }

    pub fn naming_convention_checker(&self) -> &Arc<dyn INamingCheckerProtocol> {
        &self.naming_convention_checker
    }

    pub fn suffix_prefix_checker(&self) -> &Arc<dyn INamingCheckerProtocol> {
        &self.suffix_prefix_checker
    }

    pub fn analyzer(&self) -> Arc<dyn ILayerDetectionProtocol> {
        self.analyzer.clone()
    }

    pub fn orchestrator(&self) -> Arc<dyn INamingRunnerAggregate> {
        Arc::new(crate::agent_naming_orchestrator::NamingOrchestrator::new(
            self.naming_convention_checker.clone(),
            self.suffix_prefix_checker.clone(),
            self.analyzer.clone(),
            self.fs.clone(),
        ))
    }
}

struct DefaultLayerDetector;
impl ILayerDetectionProtocol for DefaultLayerDetector {
    fn config(&self) -> &ArchitectureConfig {
        static CONFIG: std::sync::OnceLock<ArchitectureConfig> = std::sync::OnceLock::new();
        CONFIG.get_or_init(ArchitectureConfig::default)
    }
    fn detect_layer(
        &self,
        _file_path: &shared::common::taxonomy_path_vo::FilePath,
        _root_dir: &shared::common::taxonomy_path_vo::FilePath,
    ) -> Option<shared::common::taxonomy_layer_vo::LayerNameVO> {
        None
    }
    fn get_layer_def(
        &self,
        _layer: &shared::common::taxonomy_layer_vo::LayerNameVO,
    ) -> Option<shared::common::taxonomy_definition_vo::LayerDefinition> {
        None
    }
    fn get_orphan_entry_points(&self) -> Vec<shared::common::taxonomy_path_vo::FilePath> {
        Vec::new()
    }
    fn extract_layer_from_prefix(
        &self,
        _filename: &shared::common::taxonomy_path_vo::FilePath,
    ) -> Option<shared::common::taxonomy_layer_vo::LayerNameVO> {
        None
    }
    fn resolve_specialized_layer(
        &self,
        base_layer: &shared::common::taxonomy_layer_vo::LayerNameVO,
        _file_path: &shared::common::taxonomy_path_vo::FilePath,
    ) -> shared::common::taxonomy_layer_vo::LayerNameVO {
        base_layer.clone()
    }
    fn detect_module_layer(
        &self,
        _module: &str,
    ) -> Option<shared::common::taxonomy_layer_vo::LayerNameVO> {
        None
    }
    fn refine_module_layer(
        &self,
        base_name: &shared::common::taxonomy_layer_vo::LayerNameVO,
        _parts: &[&str],
    ) -> shared::common::taxonomy_layer_vo::LayerNameVO {
        base_name.clone()
    }
}
