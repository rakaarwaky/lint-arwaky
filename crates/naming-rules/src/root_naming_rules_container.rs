// PURPOSE: NamingContainer — wiring for naming-rules feature (root layer, wiring only)
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::naming_rules::contract_naming_analyzer_protocol::INamingAnalyzerProtocol;
use shared::naming_rules::contract_naming_checker_protocol::INamingCheckerProtocol;
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use shared::taxonomy_definition_vo::LayerMapVO;
use shared::taxonomy_layer_vo::LayerNameVO;
use std::sync::Arc;

pub struct NamingContainer {
    naming_convention_checker: Arc<dyn INamingCheckerProtocol>,
    suffix_prefix_checker: Arc<dyn INamingCheckerProtocol>,
    analyzer: Arc<dyn INamingAnalyzerProtocol>,
}

impl NamingContainer {
    pub fn new(analyzer: Arc<dyn INamingAnalyzerProtocol>) -> Self {
        let naming_convention_checker: Arc<dyn INamingCheckerProtocol> =
            Arc::new(crate::capabilities_naming_convention_checker::NamingConventionChecker::new());
        let suffix_prefix_checker: Arc<dyn INamingCheckerProtocol> =
            Arc::new(crate::capabilities_suffix_prefix_checker::SuffixPrefixChecker::new());
        Self {
            naming_convention_checker,
            suffix_prefix_checker,
            analyzer,
        }
    }

    pub fn new_default() -> Self {
        Self::new(Arc::new(DefaultNamingAnalyzer))
    }

    pub fn naming_convention_checker(&self) -> &Arc<dyn INamingCheckerProtocol> {
        &self.naming_convention_checker
    }

    pub fn suffix_prefix_checker(&self) -> &Arc<dyn INamingCheckerProtocol> {
        &self.suffix_prefix_checker
    }

    pub fn analyzer(&self) -> Arc<dyn INamingAnalyzerProtocol> {
        self.analyzer.clone()
    }

    pub fn orchestrator(&self) -> Arc<dyn INamingRunnerAggregate> {
        Arc::new(crate::agent_naming_orchestrator::NamingOrchestrator::new(
            self.naming_convention_checker.clone(),
            self.suffix_prefix_checker.clone(),
            self.analyzer.clone(),
        ))
    }
}

struct DefaultNamingAnalyzer;
impl INamingAnalyzerProtocol for DefaultNamingAnalyzer {
    fn config(&self) -> &ArchitectureConfig {
        static CONFIG: std::sync::OnceLock<ArchitectureConfig> = std::sync::OnceLock::new();
        CONFIG.get_or_init(ArchitectureConfig::default)
    }
    fn layer_map(&self) -> &LayerMapVO {
        static MAP: std::sync::OnceLock<LayerMapVO> = std::sync::OnceLock::new();
        MAP.get_or_init(|| LayerMapVO::new(std::collections::HashMap::new()))
    }
    fn detect_layer(&self, _f: &FilePath, _root_dir: &FilePath) -> Option<LayerNameVO> {
        None
    }
}
