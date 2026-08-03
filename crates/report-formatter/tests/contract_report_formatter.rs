// Verify that all concrete types implement their declared contract traits.
use report_formatter_lint_arwaky::agent_report_formatter_orchestrator::ReportFormatterOrchestrator;
use report_formatter_lint_arwaky::capabilities_json_formatter::JsonFormatter;
use report_formatter_lint_arwaky::capabilities_junit_formatter::JunitFormatter;
use report_formatter_lint_arwaky::capabilities_sarif_formatter::SarifFormatter;
use report_formatter_lint_arwaky::capabilities_text_formatter::TextFormatter;
use shared::report_formatter::{IReportFormatterAggregate, IReportFormatterProtocol};

#[test]
fn text_formatter_implements_protocol() {
    fn assert_trait<T: IReportFormatterProtocol>() {}
    assert_trait::<TextFormatter>();
}

#[test]
fn json_formatter_implements_protocol() {
    fn assert_trait<T: IReportFormatterProtocol>() {}
    assert_trait::<JsonFormatter>();
}

#[test]
fn sarif_formatter_implements_protocol() {
    fn assert_trait<T: IReportFormatterProtocol>() {}
    assert_trait::<SarifFormatter>();
}

#[test]
fn junit_formatter_implements_protocol() {
    fn assert_trait<T: IReportFormatterProtocol>() {}
    assert_trait::<JunitFormatter>();
}

#[test]
fn orchestrator_implements_aggregate() {
    fn assert_trait<T: IReportFormatterAggregate>() {}
    assert_trait::<ReportFormatterOrchestrator>();
}

#[test]
fn all_contracts_are_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<TextFormatter>();
    assert_send_sync::<JsonFormatter>();
    assert_send_sync::<SarifFormatter>();
    assert_send_sync::<JunitFormatter>();
    assert_send_sync::<ReportFormatterOrchestrator>();
}
