// PURPOSE: Contract tests — verify all capabilities implement their declared protocol traits
use std::sync::Arc;

use quality_rules_lint_arwaky::CodeAnalysisContainer;
use quality_rules_lint_arwaky::agent_quality_orchestrator::CodeAnalysisDeps;
use quality_rules_lint_arwaky::agent_quality_orchestrator::CodeAnalysisOrchestrator;
use quality_rules_lint_arwaky::capabilities_check_bypass_checker::BypassChecker;
use quality_rules_lint_arwaky::capabilities_code_duplication_analyzer::CodeDuplicationAnalyzer;
use quality_rules_lint_arwaky::capabilities_line_checker::ArchLineChecker;
use quality_rules_lint_arwaky::capabilities_mandatory_definition_checker::MandatoryDefinitionChecker;

use shared::quality_rules::IBypassCheckerProtocol;
use shared::quality_rules::ICodeAnalysisAggregate;
use shared::quality_rules::ICodeMetricAnalyzerProtocol;
use shared::quality_rules::IDeadInheritanceProtocol;
use shared::quality_rules::ILineCheckerProtocol;
use shared::quality_rules::IMandatoryClassProtocol;

use shared::common::LayerMapVO;
use shared::config_system::ArchitectureConfig;

// ── ArchLineChecker → ILineCheckerProtocol ──────────────────

fn assert_line_checker(_: &dyn ILineCheckerProtocol) {}

#[test]
fn arch_line_checker_implements_iline_checker_protocol() {
    let checker = ArchLineChecker::new();
    assert_line_checker(&checker);
}

// ── MandatoryDefinitionChecker → IDeadInheritanceProtocol + IMandatoryClassProtocol ──

fn assert_dead_inheritance_checker(_: &dyn IDeadInheritanceProtocol) {}
fn assert_mandatory_class_checker(_: &dyn IMandatoryClassProtocol) {}

#[test]
fn mandatory_definition_checker_implements_idead_inheritance_protocol() {
    let checker = MandatoryDefinitionChecker::new();
    assert_dead_inheritance_checker(&checker);
}

#[test]
fn mandatory_definition_checker_implements_imandatory_class_protocol() {
    let checker = MandatoryDefinitionChecker::new();
    assert_mandatory_class_checker(&checker);
}

// ── BypassChecker → IBypassCheckerProtocol ──────────────────

fn assert_bypass_checker(_: &dyn IBypassCheckerProtocol) {}

#[test]
fn bypass_checker_implements_ibypass_checker_protocol() {
    let checker = BypassChecker::new();
    assert_bypass_checker(&checker);
}

// ── CodeDuplicationAnalyzer → ICodeMetricAnalyzerProtocol ──

fn assert_code_metric_analyzer(_: &dyn ICodeMetricAnalyzerProtocol) {}

#[test]
fn code_duplication_analyzer_implements_icode_metric_analyzer_protocol() {
    let config = Arc::new(ArchitectureConfig::default());
    let analyzer = CodeDuplicationAnalyzer::from_config(config);
    assert_code_metric_analyzer(&analyzer);
}

// ── CodeAnalysisOrchestrator → ICodeAnalysisAggregate ────────

fn assert_code_analysis_aggregate(_: &dyn ICodeAnalysisAggregate) {}

#[test]
fn code_analysis_orchestrator_implements_icode_analysis_aggregate() {
    let config = ArchitectureConfig::default();
    let layer_map = LayerMapVO::new(std::collections::HashMap::new());
    let mandatory = Arc::new(MandatoryDefinitionChecker::new());
    let deps = CodeAnalysisDeps {
        bypass_checker: Arc::new(BypassChecker::new()),
        dead_inheritance_checker: mandatory.clone(),
        line_checker: Arc::new(ArchLineChecker::new()),
        class_checker: mandatory,
        duplication_checker: Arc::new(CodeDuplicationAnalyzer::from_config(Arc::new(
            config.clone(),
        ))),
    };
    let orchestrator = CodeAnalysisOrchestrator::new(deps, config, layer_map);
    assert_code_analysis_aggregate(&orchestrator);
}

// ── Container returns valid aggregate ────────────────────────

#[test]
fn container_returns_valid_aggregate() {
    let container = CodeAnalysisContainer::new();
    let linter = container.code_analysis_linter();
    assert_code_analysis_aggregate(linter.as_ref());
}
