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
use shared::code_analysis::contract_layer_detection_protocol::ILayerDetectionProtocol;
use shared::code_analysis::contract_line_protocol::ILineCheckerProtocol;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use std::sync::Arc;

/// CodeAnalysisCheckerContainer holds only code-analysis protocol implementations.
/// Other crates (import-rules, naming-rules, role-rules, orphan-detector)
/// have their own containers and orchestrators.
#[derive(Clone)]
pub struct CodeAnalysisCheckerContainer {
    analyzer: Arc<dyn ILayerDetectionProtocol>,
    bypass_checker: Arc<dyn IBypassCheckerProtocol>,
    mandatory_definition_checker: Arc<MandatoryDefinitionChecker>,
    line_checker: Arc<dyn ILineCheckerProtocol>,
    code_duplication_analyzer: Arc<CodeDuplicationAnalyzer>,
}

impl CodeAnalysisCheckerContainer {
    pub fn new(analyzer: Arc<dyn ILayerDetectionProtocol>) -> Self {
        let mandatory = Arc::new(MandatoryDefinitionChecker::new());
        // Honor AES304 forbidden_bypass from config when the analyzer exposes one;
        // fall back to the in-code default list otherwise.
        let bypass = analyzer
            .config()
            .rules
            .iter()
            .find(|r| r.name.value == "AES304")
            .map(|r| BypassChecker::from_patterns(&r.code_analysis.forbidden_bypass))
            .unwrap_or_default();
        Self {
            analyzer,
            bypass_checker: Arc::new(bypass),
            mandatory_definition_checker: mandatory,
            line_checker: Arc::new(ArchLineChecker {}),
            code_duplication_analyzer: Arc::new(CodeDuplicationAnalyzer::new()),
        }
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
        file: &FilePath,
        root_dir: &FilePath,
    ) -> Option<shared::taxonomy_layer_vo::LayerNameVO> {
        self.analyzer.detect_layer(file, root_dir)
    }

    pub fn get_layer_def(
        &self,
        layer: &shared::taxonomy_layer_vo::LayerNameVO,
    ) -> Option<shared::common::taxonomy_definition_vo::LayerDefinition> {
        self.analyzer.get_layer_def(layer)
    }

    pub fn analyzer(&self) -> &Arc<dyn ILayerDetectionProtocol> {
        &self.analyzer
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
        file: &FilePath,
        root_dir: &FilePath,
    ) -> Option<shared::taxonomy_layer_vo::LayerNameVO>;
    fn get_layer_def(
        &self,
        layer: &shared::taxonomy_layer_vo::LayerNameVO,
    ) -> Option<shared::common::taxonomy_definition_vo::LayerDefinition>;
}

impl CodeAnalysisCheckerContainerRef for CodeAnalysisCheckerContainer {
    fn detect_layer(
        &self,
        file: &FilePath,
        root_dir: &FilePath,
    ) -> Option<shared::taxonomy_layer_vo::LayerNameVO> {
        self.detect_layer(file, root_dir)
    }
    fn get_layer_def(
        &self,
        layer: &shared::taxonomy_layer_vo::LayerNameVO,
    ) -> Option<shared::common::taxonomy_definition_vo::LayerDefinition> {
        self.get_layer_def(layer)
    }
}

impl Default for CodeAnalysisCheckerContainer {
    fn default() -> Self {
        Self::new(Arc::new(PlaceholderAnalyzer))
    }
}

struct PlaceholderAnalyzer;
impl ILayerDetectionProtocol for PlaceholderAnalyzer {
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

    pub fn new_with_analyzer(analyzer: Arc<dyn ILayerDetectionProtocol>) -> Self {
        let checker_container = CodeAnalysisCheckerContainer::new(analyzer);
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
