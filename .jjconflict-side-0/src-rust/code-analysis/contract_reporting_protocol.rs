// lint_reporting_protocol — Protocols for report formatting and output.
// Defines interfaces for transforming ArchitectureGovernanceEntitys into various output formats.
use crate::output_report::taxonomy_score_vo::FileFormat;
use crate::pipeline_jobs::taxonomy_job_vo::ResponseData;
use crate::shared_common::taxonomy_governance_entity::ArchitectureGovernanceEntity;
use crate::shared_common::taxonomy_suggestion_vo::LogOutput;
use async_trait::async_trait;

#[async_trait]
pub trait ILintReportingProtocol: Send + Sync {
    /// Format the report as a plain text representation.
    async fn format(&self, report: &ArchitectureGovernanceEntity) -> LogOutput;

    /// Return formatted data for surface consumption.
    async fn get_formatted_payload(
        &self,
        report: &ArchitectureGovernanceEntity,
        output_format: FileFormat,
    ) -> ResponseData;

    /// Convert report to SARIF format.
    async fn to_sarif(&self, report: &ArchitectureGovernanceEntity) -> LogOutput;

    /// Convert report to JUnit XML format.
    async fn to_junit(&self, report: &ArchitectureGovernanceEntity) -> LogOutput;
}
