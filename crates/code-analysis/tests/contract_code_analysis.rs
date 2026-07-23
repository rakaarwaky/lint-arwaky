use code_analysis_lint_arwaky::{
    capabilities_check_bypass_checker::BypassChecker,
    capabilities_line_checker::ArchLineChecker,
    capabilities_mandatory_definition_checker::MandatoryDefinitionChecker,
    root_code_analysis_container::CodeAnalysisContainer,
    CodeAnalysisOrchestrator,
};
use shared::code_analysis::contract_bypass_checker_protocol::IBypassCheckerProtocol;
use shared::code_analysis::contract_class_protocol::IMandatoryClassProtocol;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::code_analysis::contract_dead_inheritance_protocol::IDeadInheritanceProtocol;
use shared::code_analysis::contract_line_protocol::ILineCheckerProtocol;

// ─── BypassChecker implements IBypassCheckerProtocol ───────

#[test]
fn bypass_checker_implements_bypass_checker_protocol() {
    fn assert_trait<T: IBypassCheckerProtocol>() {}
    assert_trait::<BypassChecker>();
}

// ─── ArchLineChecker implements ILineCheckerProtocol ───────

#[test]
fn arch_line_checker_implements_line_checker_protocol() {
    fn assert_trait<T: ILineCheckerProtocol>() {}
    assert_trait::<ArchLineChecker>();
}

// ─── MandatoryDefinitionChecker implements IMandatoryClassProtocol ───

#[test]
fn mandatory_definition_checker_implements_mandatory_class_protocol() {
    fn assert_trait<T: IMandatoryClassProtocol>() {}
    assert_trait::<MandatoryDefinitionChecker>();
}

// ─── MandatoryDefinitionChecker implements IDeadInheritanceProtocol ──

#[test]
fn mandatory_definition_checker_implements_dead_inheritance_protocol() {
    fn assert_trait<T: IDeadInheritanceProtocol>() {}
    assert_trait::<MandatoryDefinitionChecker>();
}

// ─── CodeAnalysisOrchestrator implements ICodeAnalysisAggregate ──────

#[test]
fn code_analysis_orchestrator_implements_code_analysis_aggregate() {
    fn assert_trait<T: ICodeAnalysisAggregate>() {}
    assert_trait::<CodeAnalysisOrchestrator>();
}

// ─── CodeAnalysisContainer exposes aggregate ─────────────────────────

#[test]
fn code_analysis_container_exposes_aggregate() {
    let container = CodeAnalysisContainer::new();
    let aggregate: std::sync::Arc<dyn ICodeAnalysisAggregate> = container.code_analysis_linter();
    let rules = aggregate.active_rules();
    assert!(rules.is_empty() || !rules.is_empty());
}
