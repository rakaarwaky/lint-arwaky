// Contract tests — verify all concrete types implement their declared contract traits.
use report_formatter_lint_arwaky::agent_report_formatter_orchestrator::ReportFormatterOrchestrator;
use report_formatter_lint_arwaky::capabilities_json_formatter::JsonFormatter;
use report_formatter_lint_arwaky::capabilities_junit_formatter::JunitFormatter;
use report_formatter_lint_arwaky::capabilities_sarif_formatter::SarifFormatter;
use report_formatter_lint_arwaky::capabilities_text_formatter::TextFormatter;
use shared::report_formatter::{
    IDefaultReportFallbackProtocol, IFormatDelegationProtocol, IJUnitFormatProtocol,
    IJsonFormatProtocol, IReportFormatterAggregate, ISarifFormatProtocol, ITextFormatProtocol,
    IXmlEscapeProtocol,
};

#[test]
fn text_formatter_implements_text_format_protocol() {
    fn assert_text_format<T: ITextFormatProtocol>() {}
    assert_text_format::<TextFormatter>();
}

#[test]
fn text_formatter_implements_default_fallback_protocol() {
    fn assert_default_fallback<T: IDefaultReportFallbackProtocol>() {}
    assert_default_fallback::<TextFormatter>();
}

#[test]
fn text_formatter_implements_format_delegation_protocol() {
    fn assert_format_delegation<T: IFormatDelegationProtocol>() {}
    assert_format_delegation::<TextFormatter>();
}

#[test]
fn json_formatter_implements_json_format_protocol() {
    fn assert_json_format<T: IJsonFormatProtocol>() {}
    assert_json_format::<JsonFormatter>();
}

#[test]
fn json_formatter_implements_format_delegation_protocol() {
    fn assert_format_delegation<T: IFormatDelegationProtocol>() {}
    assert_format_delegation::<JsonFormatter>();
}

#[test]
fn sarif_formatter_implements_sarif_format_protocol() {
    fn assert_sarif_format<T: ISarifFormatProtocol>() {}
    assert_sarif_format::<SarifFormatter>();
}

#[test]
fn sarif_formatter_implements_format_delegation_protocol() {
    fn assert_format_delegation<T: IFormatDelegationProtocol>() {}
    assert_format_delegation::<SarifFormatter>();
}

#[test]
fn junit_formatter_implements_junit_format_protocol() {
    fn assert_junit_format<T: IJUnitFormatProtocol>() {}
    assert_junit_format::<JunitFormatter>();
}

#[test]
fn junit_formatter_implements_format_delegation_protocol() {
    fn assert_format_delegation<T: IFormatDelegationProtocol>() {}
    assert_format_delegation::<JunitFormatter>();
}

#[test]
fn junit_formatter_implements_xml_escape_protocol() {
    fn assert_xml_escape<T: IXmlEscapeProtocol>() {}
    assert_xml_escape::<JunitFormatter>();
}

#[test]
fn orchestrator_implements_aggregate() {
    fn assert_aggregate<T: IReportFormatterAggregate>() {}
    assert_aggregate::<ReportFormatterOrchestrator>();
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
