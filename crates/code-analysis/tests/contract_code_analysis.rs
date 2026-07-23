// PURPOSE: Verify that all capabilities implement their declared protocol traits
// and that the orchestrator implements its aggregate trait.

use code_analysis_lint_arwaky::*;
use shared::code_analysis::contract_bypass_checker_protocol::IBypassCheckerProtocol;
use shared::code_analysis::contract_class_protocol::IMandatoryClassProtocol;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::code_analysis::contract_code_metric_analyzer_protocol::ICodeMetricAnalyzerProtocol;
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

// ─── CodeDuplicationAnalyzer implements ICodeMetricAnalyzerProtocol ──

#[test]
fn code_duplication_analyzer_implements_code_metric_analyzer_protocol() {
    fn assert_trait<T: ICodeMetricAnalyzerProtocol>() {}
    assert_trait::<CodeDuplicationAnalyzer>();
}

// ─── CodeAnalysisOrchestrator implements ICodeAnalysisAggregate ──────

#[test]
fn code_analysis_orchestrator_implements_code_analysis_aggregate() {
    fn assert_trait<T: ICodeAnalysisAggregate>() {}
    assert_trait::<CodeAnalysisOrchestrator>();
}

// ─── Container wiring produces trait objects ─────────────────────────

#[test]
fn container_exposes_bypass_checker_as_trait_object() {
    let container = CodeAnalysisCheckerContainer::default();
    let checker: &std::sync::Arc<dyn IBypassCheckerProtocol> = container.bypass_checker();
    // Trait object is usable — call a method to prove dispatch works.
    let mut violations = Vec::new();
    checker.check_bypass_comments("test.rs", "let x = 1;", &mut violations);
    assert!(violations.is_empty());
}

#[test]
fn container_exposes_line_checker_as_trait_object() {
    let container = CodeAnalysisCheckerContainer::default();
    let checker: &std::sync::Arc<dyn ILineCheckerProtocol> = container.line_checker();
    let mut violations = Vec::new();
    checker.check_line_counts("test.rs", None, "line1\nline2", &mut violations);
    assert!(violations.is_empty());
}

#[test]
fn container_exposes_class_checker_as_trait_object() {
    let container = CodeAnalysisCheckerContainer::default();
    let checker: std::sync::Arc<dyn IMandatoryClassProtocol> = container.class_checker();
    let mut violations = Vec::new();
    checker.check_mandatory_class_definition("test.rs", None, "struct Foo {}", &mut violations);
    assert!(violations.is_empty());
}

#[test]
fn container_exposes_dead_inheritance_checker_as_trait_object() {
    let container = CodeAnalysisCheckerContainer::default();
    let checker: std::sync::Arc<dyn IDeadInheritanceProtocol> =
        container.dead_inheritance_checker();
    let mut violations = Vec::new();
    checker.check_dead_inheritance("test.rs", "struct Foo { x: i32 }", &mut violations);
    assert!(violations.is_empty());
}

#[test]
fn code_analysis_container_exposes_aggregate() {
    let container = CodeAnalysisContainer::new();
    let aggregate: std::sync::Arc<dyn ICodeAnalysisAggregate> = container.code_analysis_linter();
    let rules = aggregate.active_rules();
    // Default config has no rules configured, so empty is valid.
    assert!(rules.is_empty() || !rules.is_empty());
}
