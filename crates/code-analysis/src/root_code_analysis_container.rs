use crate::agent_code_analysis_orchestrator::{CodeAnalysisDeps, CodeAnalysisOrchestrator};
use crate::capabilities_check_bypass_checker::BypassChecker;
use crate::capabilities_code_duplication_analyzer::CodeDuplicationAnalyzer;
use crate::capabilities_line_checker::ArchLineChecker;
use crate::capabilities_mandatory_definition_checker::MandatoryDefinitionChecker;
use shared::code_analysis::contract_bypass_checker_protocol::IBypassCheckerProtocol;
use shared::code_analysis::contract_class_protocol::IMandatoryClassProtocol;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::code_analysis::contract_code_metric_analyzer_protocol::ICodeMetricAnalyzerProtocol;
use shared::code_analysis::contract_dead_inheritance_protocol::IDeadInheritanceProtocol;
use shared::code_analysis::contract_line_protocol::ILineCheckerProtocol;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::taxonomy_definition_vo::LayerMapVO;
use std::sync::Arc;

pub struct CodeAnalysisContainer {
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
}

impl CodeAnalysisContainer {
    pub fn new() -> Self {
        let config = ArchitectureConfig::default();
        let layer_map = LayerMapVO::new(std::collections::HashMap::new());
        let mandatory = Arc::new(MandatoryDefinitionChecker::new());
        let deps = CodeAnalysisDeps {
            bypass_checker: Arc::new(BypassChecker::default()) as Arc<dyn IBypassCheckerProtocol>,
            dead_inheritance_checker: mandatory.clone() as Arc<dyn IDeadInheritanceProtocol>,
            line_checker: Arc::new(ArchLineChecker {}) as Arc<dyn ILineCheckerProtocol>,
            class_checker: mandatory as Arc<dyn IMandatoryClassProtocol>,
            duplication_checker: Arc::new(CodeDuplicationAnalyzer::new())
                as Arc<dyn ICodeMetricAnalyzerProtocol>,
        };
        Self {
            code_analysis_linter: Arc::new(CodeAnalysisOrchestrator::new(deps, config, layer_map)),
        }
    }

    pub fn new_with_config(config: ArchitectureConfig, layer_map: LayerMapVO) -> Self {
        let mandatory = Arc::new(MandatoryDefinitionChecker::new());
        let bypass = config
            .rules
            .iter()
            .find(|r| r.name.value == "AES304")
            .map(|r| BypassChecker::from_patterns(&r.code_analysis.forbidden_bypass))
            .unwrap_or_default();
        let dup_checker = Arc::new(CodeDuplicationAnalyzer::from_config(Arc::new(
            config.clone(),
        )));
        let deps = CodeAnalysisDeps {
            bypass_checker: Arc::new(bypass) as Arc<dyn IBypassCheckerProtocol>,
            dead_inheritance_checker: mandatory.clone() as Arc<dyn IDeadInheritanceProtocol>,
            line_checker: Arc::new(ArchLineChecker {}) as Arc<dyn ILineCheckerProtocol>,
            class_checker: mandatory as Arc<dyn IMandatoryClassProtocol>,
            duplication_checker: dup_checker as Arc<dyn ICodeMetricAnalyzerProtocol>,
        };
        Self {
            code_analysis_linter: Arc::new(CodeAnalysisOrchestrator::new(deps, config, layer_map)),
        }
    }

    pub fn from_orchestrator(
        orchestrator: &Arc<dyn IConfigOrchestratorAggregate>,
        project_root: &str,
    ) -> Self {
        let fp = FilePath::new(project_root.to_string()).unwrap_or_default();
        let config = orchestrator.load_config_sync(&fp);
        let layer_map = LayerMapVO::new(config.layers.clone());
        Self::new_with_config(config, layer_map)
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
