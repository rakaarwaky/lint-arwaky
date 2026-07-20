// PURPOSE: Root container for code-analysis — defines CodeAnalysisCheckerContainer and CodeAnalysisContainer
// Wiring: ICodeMetricAnalyzerProtocol → CodeDuplicationAnalyzer (capabilities layer)
// ALGORITHM:
//   CodeAnalysisCheckerContainer: injects checkers (BypassChecker, ArchLineChecker,
//     MandatoryDefinitionChecker, CodeDuplicationAnalyzer) and exposes them via typed accessors.
//   CodeAnalysisContainer: wraps CodeAnalysisOrchestrator as IArchLintProtocol for surface consumption.

use crate::capabilities_check_bypass_checker::BypassChecker;
use crate::capabilities_code_duplication_analyzer::CodeDuplicationAnalyzer;
use crate::capabilities_line_checker::ArchLineChecker;
use crate::capabilities_mandatory_definition_checker::MandatoryDefinitionChecker;
use shared::code_analysis::contract_bypass_checker_protocol::IBypassCheckerProtocol;
use shared::code_analysis::contract_class_protocol::IMandatoryClassProtocol;
use shared::code_analysis::contract_dead_inheritance_protocol::IDeadInheritanceProtocol;
use shared::code_analysis::contract_line_protocol::ILineCheckerProtocol;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::taxonomy_definition_vo::LayerMapVO;
use std::sync::Arc;

/// CodeAnalysisCheckerContainer holds only code-analysis protocol implementations.
/// Other crates (import-rules, naming-rules, role-rules, orphan-detector)
/// have their own containers and orchestrators.
#[derive(Clone)]
pub struct CodeAnalysisCheckerContainer {
    config: ArchitectureConfig,
    layer_map: LayerMapVO,
    bypass_checker: Arc<dyn IBypassCheckerProtocol>,
    mandatory_definition_checker: Arc<MandatoryDefinitionChecker>,
    line_checker: Arc<dyn ILineCheckerProtocol>,
    code_duplication_analyzer: Arc<CodeDuplicationAnalyzer>,
}

impl CodeAnalysisCheckerContainer {
    pub fn new(config: ArchitectureConfig, layer_map: LayerMapVO) -> Self {
        let mandatory = Arc::new(MandatoryDefinitionChecker::new());
        let bypass = config
            .rules
            .iter()
            .find(|r| r.name.value == "AES304")
            .map(|r| BypassChecker::from_patterns(&r.code_analysis.forbidden_bypass))
            .unwrap_or_default();
        Self {
            config,
            layer_map,
            bypass_checker: Arc::new(bypass),
            mandatory_definition_checker: mandatory,
            line_checker: Arc::new(ArchLineChecker {}),
            code_duplication_analyzer: Arc::new(CodeDuplicationAnalyzer::new()),
        }
    }

    pub fn config(&self) -> &ArchitectureConfig {
        &self.config
    }

    pub fn bypass_checker(&self) -> &Arc<dyn IBypassCheckerProtocol> {
        &self.bypass_checker
    }

    pub fn dead_inheritance_checker(&self) -> Arc<dyn IDeadInheritanceProtocol> {
        self.mandatory_definition_checker.clone()
    }

    pub fn line_checker(&self) -> &Arc<dyn ILineCheckerProtocol> {
        &self.line_checker
    }

    pub fn class_checker(&self) -> Arc<dyn IMandatoryClassProtocol> {
        self.mandatory_definition_checker.clone()
    }

    pub fn detect_layer(
        &self,
        file: &str,
        _root_dir: &str,
    ) -> Option<shared::taxonomy_layer_vo::LayerNameVO> {
        let filename = shared::common::utility_layer_detector::extract_filename(file);
        let layer = shared::common::utility_layer_detector::detect_layer_from_prefix(filename)?;
        let keys = shared::common::utility_layer_detector::collect_layer_keys(&self.layer_map);
        Some(shared::taxonomy_layer_vo::LayerNameVO::new(
            shared::common::utility_layer_detector::resolve_specialized_layer(&layer, file, &keys),
        ))
    }

    pub fn get_layer_def(
        &self,
        layer: &shared::taxonomy_layer_vo::LayerNameVO,
    ) -> Option<&shared::common::taxonomy_definition_vo::LayerDefinition> {
        shared::common::utility_layer_detector::get_layer_def(&layer.value, &self.config.layers)
    }

    pub fn duplication_checker(&self) -> &Arc<CodeDuplicationAnalyzer> {
        &self.code_duplication_analyzer
    }

    pub fn as_checker_ref(&self) -> &dyn CodeAnalysisCheckerContainerRef {
        self
    }
}

/// Trait for dynamic dispatch of CodeAnalysisCheckerContainer
pub trait CodeAnalysisCheckerContainerRef {
    fn detect_layer(
        &self,
        file: &str,
        root_dir: &str,
    ) -> Option<shared::taxonomy_layer_vo::LayerNameVO>;
    fn get_layer_def(
        &self,
        layer: &shared::taxonomy_layer_vo::LayerNameVO,
    ) -> Option<&shared::common::taxonomy_definition_vo::LayerDefinition>;
}

impl CodeAnalysisCheckerContainerRef for CodeAnalysisCheckerContainer {
    fn detect_layer(
        &self,
        file: &str,
        root_dir: &str,
    ) -> Option<shared::taxonomy_layer_vo::LayerNameVO> {
        self.detect_layer(file, root_dir)
    }
    fn get_layer_def(
        &self,
        layer: &shared::taxonomy_layer_vo::LayerNameVO,
    ) -> Option<&shared::common::taxonomy_definition_vo::LayerDefinition> {
        self.get_layer_def(layer)
    }
}

impl Default for CodeAnalysisCheckerContainer {
    fn default() -> Self {
        let config = ArchitectureConfig::default();
        let layer_map = LayerMapVO::new(std::collections::HashMap::new());
        Self::new(config, layer_map)
    }
}

// CodeAnalysisContainer — wiring for code-analysis feature
use crate::CodeAnalysisOrchestrator;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;

pub struct CodeAnalysisContainer {
    code_analysis_linter: Arc<CodeAnalysisOrchestrator>,
}

impl CodeAnalysisContainer {
    pub fn new() -> Self {
        Self {
            code_analysis_linter: Arc::new(CodeAnalysisOrchestrator::new()),
        }
    }

    pub fn new_with_config(config: ArchitectureConfig, layer_map: LayerMapVO) -> Self {
        let checker_container = CodeAnalysisCheckerContainer::new(config, layer_map);
        Self {
            code_analysis_linter: Arc::new(CodeAnalysisOrchestrator::new_with_container(Arc::new(
                checker_container,
            ))),
        }
    }

    pub fn code_analysis_linter(&self) -> Arc<dyn ICodeAnalysisAggregate> {
        self.code_analysis_linter.clone()
    }
}

impl Default for CodeAnalysisContainer {
    fn default() -> Self {
        Self::new()
    }
}
