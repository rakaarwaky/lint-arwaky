// Contract tests — verify all concrete types implement their declared contract traits.
use auto_fix_lint_arwaky::agent_fix_orchestrator::FixOrchestrator;
use auto_fix_lint_arwaky::capabilities_bypass_fix::BypassFix;
use auto_fix_lint_arwaky::capabilities_symbol_rename::SymbolRename;
use auto_fix_lint_arwaky::capabilities_unused_import_fix::UnusedImportFix;
use auto_fix_lint_arwaky::capabilities_violation_report::ViolationReport;
use shared_auto_fix::{
    IBypassFixProtocol, IFixAggregate, ISymbolRenameProtocol, IUnusedImportFixProtocol,
    IViolationReportProtocol,
};

#[test]
fn violation_report_implements_violation_report_protocol() {
    fn assert_trait<T: IViolationReportProtocol>() {}
    assert_trait::<ViolationReport>();
}

#[test]
fn bypass_fix_implements_bypass_fix_protocol() {
    fn assert_trait<T: IBypassFixProtocol>() {}
    assert_trait::<BypassFix>();
}

#[test]
fn unused_import_fix_implements_unused_import_fix_protocol() {
    fn assert_trait<T: IUnusedImportFixProtocol>() {}
    assert_trait::<UnusedImportFix>();
}

#[test]
fn symbol_rename_implements_symbol_rename_protocol() {
    fn assert_trait<T: ISymbolRenameProtocol>() {}
    assert_trait::<SymbolRename>();
}

#[test]
fn fix_orchestrator_implements_fix_aggregate() {
    fn assert_trait<T: IFixAggregate>() {}
    assert_trait::<FixOrchestrator>();
}

#[test]
fn all_capabilities_are_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<BypassFix>();
    assert_send_sync::<UnusedImportFix>();
    assert_send_sync::<SymbolRename>();
    assert_send_sync::<ViolationReport>();
    assert_send_sync::<FixOrchestrator>();
}

#[test]
fn orchestrator_can_be_boxed_as_trait_object() {
    fn assert_object_safe<T: IFixAggregate>() {}
    assert_object_safe::<FixOrchestrator>();
}

#[test]
fn violation_report_protocol_can_be_arc_trait_object() {
    fn assert_object_safe<T: IViolationReportProtocol>() {}
    assert_object_safe::<ViolationReport>();
}
