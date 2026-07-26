use crate::agent_code_analysis_orchestrator::{CodeAnalysisDeps, CodeAnalysisOrchestrator};
use crate::capabilities_check_bypass_checker::BypassChecker;
use crate::capabilities_code_duplication_analyzer::CodeDuplicationAnalyzer;
use crate::capabilities_line_checker::ArchLineChecker;
use crate::capabilities_mandatory_definition_checker::MandatoryDefinitionChecker;
use shared::code_analysis::{
    IBypassCheckerProtocol, ICodeAnalysisAggregate, ICodeMetricAnalyzerProtocol,
    IDeadInheritanceProtocol, ILineCheckerProtocol, IMandatoryClassProtocol,
};

use shared::common::FilePath;
use shared::config_system::{ArchitectureConfig, IConfigOrchestratorAggregate};

use shared::common::LayerMapVO;
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
