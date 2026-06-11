// PURPOSE: OutputContainer — wiring for output-report feature (root layer, wiring only)
pub struct OutputContainer {}

impl OutputContainer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn report_formatter(&self) -> Box<dyn crate::output_report::contract_output_aggregate::IReportFormatterProtocol> {
        Box::new(
            crate::output_report::capabilities_reporting_formatter::ReportFormatterProcessor::new(),
        )
    }
}
impl Default for OutputContainer {
    fn default() -> Self {
        Self::new()
    }
}

