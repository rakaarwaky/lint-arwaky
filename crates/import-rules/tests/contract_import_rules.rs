// PURPOSE: Verify all trait implementations exist and compile.
// Contract tests — trait bound assertions only. Zero runtime logic.

use import_rules_lint_arwaky::agent_import_orchestrator::ImportOrchestrator;
use import_rules_lint_arwaky::capabilities_cycle_import_analyzer::DependencyCycleAnalyzer;
use import_rules_lint_arwaky::capabilities_dummy_import_checker::DummyImportChecker;
use import_rules_lint_arwaky::capabilities_import_forbidden_checker::ArchImportForbiddenChecker;
use import_rules_lint_arwaky::capabilities_import_mandatory_checker::ArchImportMandatoryChecker;
use import_rules_lint_arwaky::capabilities_import_unused_checker::UnusedImportRuleChecker;
use import_rules_lint_arwaky::root_import_rules_container::ImportContainer;

use shared::import_rules::contract_cycle_import_protocol::ICycleImportProtocol;
use shared::import_rules::contract_dummy_import_protocol::IDummyImportCheckerProtocol;
use shared::import_rules::contract_import_forbidden_protocol::IImportForbiddenProtocol;
use shared::import_rules::contract_import_mandatory_protocol::IImportMandatoryProtocol;
use shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use shared::import_rules::contract_unused_import_protocol::IUnusedImportProtocol;

// ─── AES201: Forbidden Import Protocol ────────────────────

#[test]
fn arch_import_forbidden_checker_implements_i_import_forbidden_protocol() {
    fn assert_trait<T: IImportForbiddenProtocol>() {}
    assert_trait::<ArchImportForbiddenChecker>();
}

#[test]
fn forbidden_checker_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<ArchImportForbiddenChecker>();
}

// ─── AES202: Mandatory Import Protocol ────────────────────

#[test]
fn arch_import_mandatory_checker_implements_i_import_mandatory_protocol() {
    fn assert_trait<T: IImportMandatoryProtocol>() {}
    assert_trait::<ArchImportMandatoryChecker>();
}

#[test]
fn mandatory_checker_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<ArchImportMandatoryChecker>();
}

// ─── AES203: Unused Import Protocol ───────────────────────

#[test]
fn unused_import_rule_checker_implements_i_unused_import_protocol() {
    fn assert_trait<T: IUnusedImportProtocol>() {}
    assert_trait::<UnusedImportRuleChecker>();
}

#[test]
fn unused_checker_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<UnusedImportRuleChecker>();
}

// ─── AES204: Dummy Import Protocol ────────────────────────

#[test]
fn dummy_import_checker_implements_i_dummy_import_checker_protocol() {
    fn assert_trait<T: IDummyImportCheckerProtocol>() {}
    assert_trait::<DummyImportChecker>();
}

#[test]
fn dummy_checker_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<DummyImportChecker>();
}

// ─── AES205: Cycle Import Protocol ────────────────────────

#[test]
fn dependency_cycle_analyzer_implements_i_cycle_import_protocol() {
    fn assert_trait<T: ICycleImportProtocol>() {}
    assert_trait::<DependencyCycleAnalyzer>();
}

#[test]
fn cycle_analyzer_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<DependencyCycleAnalyzer>();
}

// ─── Aggregate: Import Runner ─────────────────────────────

#[test]
fn import_orchestrator_implements_i_import_runner_aggregate() {
    fn assert_trait<T: IImportRunnerAggregate>() {}
    assert_trait::<ImportOrchestrator>();
}

#[test]
fn import_orchestrator_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<ImportOrchestrator>();
}

// ─── Root: Container Wiring ───────────────────────────────

#[test]
fn import_container_produces_aggregate_arc() {
    let config = shared::config_system::taxonomy_config_vo::ArchitectureConfig::default();
    let container = ImportContainer::new_with_config(config);

    let _orchestrator: std::sync::Arc<dyn IImportRunnerAggregate> = container.orchestrator();
}

// ─── Rule Name Identity ───────────────────────────────────

#[test]
fn forbidden_checker_rule_name_is_aes201() {
    let checker = ArchImportForbiddenChecker::new();
    assert_eq!(checker.rule_name().value(), "AES201");
}

#[test]
fn mandatory_checker_rule_name_is_aes202() {
    let checker = ArchImportMandatoryChecker::new();
    assert_eq!(checker.rule_name().value(), "AES202");
}

#[test]
fn dummy_checker_rule_name_is_aes204() {
    let checker = DummyImportChecker::new();
    assert_eq!(checker.rule_name().value(), "AES204");
}

// ─── Orchestrator Name ────────────────────────────────────

#[test]
fn orchestrator_name_is_import_rules() {
    let config = shared::config_system::taxonomy_config_vo::ArchitectureConfig::default();
    let container = ImportContainer::new_with_config(config);
    let orch = container.orchestrator();
    assert_eq!(orch.name(), "import-rules");
}
