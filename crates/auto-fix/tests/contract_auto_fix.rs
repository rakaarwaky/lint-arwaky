// Contract tests — verify all concrete types implement their declared contract traits.
use auto_fix_lint_arwaky::agent_fix_orchestrator::FixOrchestrator;
use auto_fix_lint_arwaky::capabilities_file_adapter::FileAdapter;
use auto_fix_lint_arwaky::capabilities_fix_processor::LintFixProcessor;
use shared::auto_fix::IFileAdapterProtocol;
use shared::auto_fix::IFixProtocol;
use shared::auto_fix::LintFixOrchestratorAggregate;

#[test]
fn file_adapter_implements_file_adapter_protocol() {
    fn assert_trait<T: IFileAdapterProtocol>() {}
    assert_trait::<FileAdapter>();
}

#[test]
fn lint_fix_processor_implements_fix_protocol() {
    fn assert_trait<T: IFixProtocol>() {}
    assert_trait::<LintFixProcessor>();
}

#[test]
fn fix_orchestrator_implements_fix_orchestrator_aggregate() {
    fn assert_trait<T: LintFixOrchestratorAggregate>() {}
    assert_trait::<FixOrchestrator>();
}

#[test]
fn all_capabilities_are_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<FileAdapter>();
    assert_send_sync::<LintFixProcessor>();
    assert_send_sync::<FixOrchestrator>();
}

#[test]
fn orchestrator_can_be_boxed_as_trait_object() {
    fn assert_object_safe<T: LintFixOrchestratorAggregate>() {}
    assert_object_safe::<FixOrchestrator>();
}

#[test]
fn fix_protocol_can_be_arc_trait_object() {
    fn assert_object_safe<T: IFixProtocol>() {}
    assert_object_safe::<LintFixProcessor>();
}

#[test]
fn file_adapter_can_be_arc_trait_object() {
    fn assert_object_safe<T: IFileAdapterProtocol>() {}
    assert_object_safe::<FileAdapter>();
}
