// PURPOSE: Contract tests — verify all capabilities implement declared protocol traits.
use import_rules_lint_arwaky::capabilities_cycle_import_analyzer::DependencyCycleAnalyzer;
use import_rules_lint_arwaky::capabilities_dummy_import_checker::DummyImportChecker;
use import_rules_lint_arwaky::capabilities_import_forbidden_checker::ArchImportForbiddenChecker;
use import_rules_lint_arwaky::capabilities_import_mandatory_checker::ArchImportMandatoryChecker;
use import_rules_lint_arwaky::capabilities_import_unused_checker::UnusedImportRuleChecker;
use shared::import_rules::contract_cycle_import_protocol::ICycleImportProtocol;
use shared::import_rules::contract_dummy_import_protocol::IDummyImportCheckerProtocol;
use shared::import_rules::contract_import_forbidden_protocol::IImportForbiddenProtocol;
use shared::import_rules::contract_import_mandatory_protocol::IImportMandatoryProtocol;
use shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use shared::import_rules::contract_unused_import_protocol::IUnusedImportProtocol;

// ── Compile-time trait bound assertions ────────────────────

fn _assert_import_runner_aggregate<T: IImportRunnerAggregate>() {}
fn _assert_import_forbidden_protocol<T: IImportForbiddenProtocol>() {}
fn _assert_import_mandatory_protocol<T: IImportMandatoryProtocol>() {}
fn _assert_unused_import_protocol<T: IUnusedImportProtocol>() {}
fn _assert_cycle_import_protocol<T: ICycleImportProtocol>() {}
fn _assert_dummy_import_checker_protocol<T: IDummyImportCheckerProtocol>() {}

#[test]
fn orchestrator_implements_import_runner_aggregate() {
    _assert_import_runner_aggregate::<
        import_rules_lint_arwaky::agent_import_orchestrator::ImportOrchestrator,
    >();
}

#[test]
fn forbidden_checker_implements_import_forbidden_protocol() {
    _assert_import_forbidden_protocol::<ArchImportForbiddenChecker>();
}

#[test]
fn mandatory_checker_implements_import_mandatory_protocol() {
    _assert_import_mandatory_protocol::<ArchImportMandatoryChecker>();
}

#[test]
fn unused_checker_implements_unused_import_protocol() {
    _assert_unused_import_protocol::<UnusedImportRuleChecker>();
}

#[test]
fn cycle_analyzer_implements_cycle_import_protocol() {
    _assert_cycle_import_protocol::<DependencyCycleAnalyzer>();
}

#[test]
fn dummy_checker_implements_dummy_import_checker_protocol() {
    _assert_dummy_import_checker_protocol::<DummyImportChecker>();
}
