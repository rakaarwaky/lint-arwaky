// PURPOSE: Verify all trait implementations exist for auto-fix crate types.
// Layer: Contract verification — compile-time trait bound checks.

use auto_fix_lint_arwaky::agent_fix_orchestrator::FixOrchestrator;
use auto_fix_lint_arwaky::capabilities_file_adapter::FileAdapter;
use auto_fix_lint_arwaky::capabilities_fix_processor::LintFixProcessor;

use shared::auto_fix::contract_file_adapter_protocol::IFileAdapterProtocol;
use shared::auto_fix::contract_fix_aggregate::LintFixOrchestratorAggregate;
use shared::auto_fix::contract_fix_protocol::IFixProtocol;

// ─── IFileAdapterProtocol ─────────────────────────────────

#[test]
fn file_adapter_implements_i_file_adapter_protocol() {
    fn assert_trait<T: IFileAdapterProtocol>() {}
    assert_trait::<FileAdapter>();
}

#[test]
fn file_adapter_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<FileAdapter>();
}

// ─── IFixProtocol ─────────────────────────────────────────

#[test]
fn lint_fix_processor_implements_i_fix_protocol() {
    fn assert_trait<T: IFixProtocol>() {}
    assert_trait::<LintFixProcessor>();
}

#[test]
fn lint_fix_processor_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<LintFixProcessor>();
}

// ─── LintFixOrchestratorAggregate ─────────────────────────

#[test]
fn fix_orchestrator_implements_aggregate() {
    fn assert_trait<T: LintFixOrchestratorAggregate>() {}
    assert_trait::<FixOrchestrator>();
}

#[test]
fn fix_orchestrator_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<FixOrchestrator>();
}

// ─── Default / Clone ──────────────────────────────────────

#[test]
fn file_adapter_implements_default() {
    fn assert_default<T: Default>() {}
    assert_default::<FileAdapter>();
}
