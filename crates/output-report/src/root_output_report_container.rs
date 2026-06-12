// PURPOSE: OutputContainer — wiring for output-report feature (root layer, wiring only)
pub struct OutputContainer {}

impl OutputContainer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn report_formatter(&self) -> Box<dyn crate::IReportFormatterProtocol> {
        Box::new(crate::capabilities_reporting_formatter::ReportFormatterProcessor::new())
    }
}
impl Default for OutputContainer {
    fn default() -> Self {
        Self::new()
    }
}
