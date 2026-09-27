// Contract tests — verify all concrete types implement their declared contract traits.
use auto_fix_lint_arwaky::agent_fix_orchestrator::FixOrchestrator;
use auto_fix_lint_arwaky::capabilities_file_adapter::FileAdapter;
use auto_fix_lint_arwaky::capabilities_fix_processor::LintFixProcessor;
use shared::auto_fix::IFileAdapterProtocol;
use shared::auto_fix::IFixAggregate;
use shared::auto_fix::{
    IBypassFixProtocol, IFixPipelineProtocol, IManualReportProtocol, ISymbolRenameProtocol,
    IUnusedImportFixProtocol,
};

#[test]
fn file_adapter_implements_file_adapter_protocol() {
    fn assert_trait<T: IFileAdapterProtocol>() {}
    assert_trait::<FileAdapter>();
}

#[test]
fn lint_fix_processor_implements_pipeline_protocol() {
    fn assert_trait<T: IFixPipelineProtocol>() {}
    assert_trait::<LintFixProcessor>();
}

#[test]
fn lint_fix_processor_implements_bypass_fix_protocol() {
    fn assert_trait<T: IBypassFixProtocol>() {}
    assert_trait::<LintFixProcessor>();
}

#[test]
fn lint_fix_processor_implements_unused_import_fix_protocol() {
    fn assert_trait<T: IUnusedImportFixProtocol>() {}
    assert_trait::<LintFixProcessor>();
}

#[test]
fn lint_fix_processor_implements_symbol_rename_protocol() {
    fn assert_trait<T: ISymbolRenameProtocol>() {}
    assert_trait::<LintFixProcessor>();
}

#[test]
fn lint_fix_processor_implements_manual_report_protocol() {
    fn assert_trait<T: IManualReportProtocol>() {}
    assert_trait::<LintFixProcessor>();
}

#[test]
fn fix_orchestrator_implements_fix_orchestrator_aggregate() {
    fn assert_trait<T: IFixAggregate>() {}
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
    fn assert_object_safe<T: IFixAggregate>() {}
    assert_object_safe::<FixOrchestrator>();
}

#[test]
fn pipeline_protocol_can_be_arc_trait_object() {
    fn assert_object_safe<T: IFixPipelineProtocol>() {}
    assert_object_safe::<LintFixProcessor>();
}

#[test]
fn file_adapter_can_be_arc_trait_object() {
    fn assert_object_safe<T: IFileAdapterProtocol>() {}
    assert_object_safe::<FileAdapter>();
}
