// lint_reporting_protocol — Protocols for report formatting and output.
// Defines interfaces for transforming GovernanceReports into various output formats.
use crate::taxonomy::{FileFormat, GovernanceReport, LogOutput, ResponseData};
use async_trait::async_trait;

#[async_trait]
pub trait ILintReportingProtocol: Send + Sync {
    /// Format the report as a plain text representation.
    async fn format(&self, report: &GovernanceReport) -> LogOutput;

    /// Return formatted data for surface consumption.
    async fn get_formatted_payload(
        &self,
        report: &GovernanceReport,
        output_format: FileFormat,
    ) -> ResponseData;

    /// Convert report to SARIF format.
    async fn to_sarif(&self, report: &GovernanceReport) -> LogOutput;

    /// Convert report to JUnit XML format.
    async fn to_junit(&self, report: &GovernanceReport) -> LogOutput;
}
