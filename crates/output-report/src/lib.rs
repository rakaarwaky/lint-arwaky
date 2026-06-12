// PURPOSE: Module declarations for output-report (orchestrators, formatters)
pub use shared::output_report::contract_client_aggregate::OutputClientAggregate;
pub use shared::output_report::contract_output_aggregate::IReportFormatterProtocol;
pub use shared::output_report::contract_report_aggregate::ReportCommandsAggregate;
pub use shared::output_report::taxonomy_position_vo::Position;
pub use shared::output_report::taxonomy_result_vo::{LintResult, LintResultList};
pub use shared::output_report::taxonomy_score_constant::{
    FORMAT_JSON, FORMAT_JUNIT, FORMAT_SARIF, FORMAT_TEXT,
};
pub use shared::output_report::taxonomy_score_vo::{compute_score, FileFormat};
pub use shared::output_report::taxonomy_severity_vo::Severity;
pub mod agent_output_orchestrator;
pub use agent_output_orchestrator::OutputClientOrchestrator;
pub mod agent_commands_orchestrator;
pub use agent_commands_orchestrator::ReportCommandsOrchestrator;
pub mod capabilities_reporting_formatter;
pub use capabilities_reporting_formatter::ReportFormatterProcessor;
pub mod root_output_report_container;
