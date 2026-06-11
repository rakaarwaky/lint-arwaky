// PURPOSE: CommandOrchestrator — orchestrates output formatting commands (plain, json, junit, sarif)

use crate::code_analysis::taxonomy_governance_entity::ArchitectureGovernanceEntity;
use crate::output_report::contract_report_aggregate::ReportCommandsAggregate;
use crate::output_report::taxonomy_score_vo::FileFormat;
use crate::source_parsing::taxonomy_path_vo::FilePath;

use async_trait::async_trait;

pub struct ReportCommandsOrchestrator {}

#[async_trait]
impl ReportCommandsAggregate for ReportCommandsOrchestrator {
    fn root_path(&self) -> Option<&FilePath> {
        None
    }
    async fn report(&self, path: &FilePath, output_format: &FileFormat) {
        let report_data = self.run_analysis(path).await;
        let formatted = self.get_formatted_output(&report_data, output_format);
        println!("{}", formatted);
    }
    async fn security(&self, path: &FilePath) {
        println!("Running security scan for: {:?}", path);
    }
}

impl Default for ReportCommandsOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

impl ReportCommandsOrchestrator {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn run_analysis(&self, _path: &FilePath) -> ArchitectureGovernanceEntity {
        // Orchestrate the analysis run
        ArchitectureGovernanceEntity::default()
    }

    pub fn get_formatted_output(
        &self,
        report_data: &ArchitectureGovernanceEntity,
        output_format: &FileFormat,
    ) -> String {
        // Get formatted output
        match output_format.name.as_ref() {
            "json" => {
                serde_json::to_string_pretty(report_data).unwrap_or_else(|_| "{}".to_string())
            }
            "sarif" | "junit" => {
                // Placeholder for SARIF/JUnit formatting
                String::new()
            }
            _ => String::new(),
        }
    }
}
