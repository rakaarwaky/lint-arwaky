// PURPOSE: CommandOrchestrator — orchestrates output formatting commands (plain, json, junit, sarif)

use shared::code_analysis::taxonomy_governance_entity::ArchitectureGovernanceEntity;
use shared::output_report::contract_report_aggregate::ReportCommandsAggregate;
use shared::output_report::taxonomy_score_vo::FileFormat;
use shared::source_parsing::taxonomy_path_vo::FilePath;

use async_trait::async_trait;

pub struct ReportCommandsOrchestrator {
    root_path: Option<FilePath>,
}

#[async_trait]
impl ReportCommandsAggregate for ReportCommandsOrchestrator {
    fn root_path(&self) -> Option<&FilePath> {
        self.root_path.as_ref()
    }
    async fn report(&self, path: &FilePath, output_format: &FileFormat) {
        let report_data = self.run_analysis(path).await;
        let formatted = self.get_formatted_output(&report_data, output_format);
        println!("{}", formatted);
    }
    async fn security(&self, path: &FilePath) {
        let report_data = self.run_analysis(path).await;
        let sarif_format = FileFormat::new("sarif");
        let formatted = self.get_formatted_output(&report_data, &sarif_format);
        println!("{}", formatted);
    }
}

impl Default for ReportCommandsOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

impl ReportCommandsOrchestrator {
    pub fn new() -> Self {
        Self {
            root_path: None,
        }
    }

    pub fn with_root_path(root_path: FilePath) -> Self {
        Self {
            root_path: Some(root_path),
        }
    }

    pub async fn run_analysis(&self, _path: &FilePath) -> ArchitectureGovernanceEntity {
        ArchitectureGovernanceEntity::default()
    }

    pub fn get_formatted_output(
        &self,
        report_data: &ArchitectureGovernanceEntity,
        output_format: &FileFormat,
    ) -> String {
        match output_format.name.as_ref() {
            "json" => {
                serde_json::to_string_pretty(report_data).unwrap_or_else(|_| "{}".to_string())
            }
            "sarif" => self.format_sarif(report_data),
            "junit" => self.format_junit(report_data),
            _ => String::new(),
        }
    }

    fn format_sarif(&self, report_data: &ArchitectureGovernanceEntity) -> String {
        let results: Vec<serde_json::Value> = report_data
            .results
            .iter()
            .map(|r| {
                serde_json::json!({
                    "ruleId": r.code.code(),
                    "level": match r.severity {
                        shared::output_report::taxonomy_severity_vo::Severity::CRITICAL => "error",
                        shared::output_report::taxonomy_severity_vo::Severity::HIGH => "error",
                        shared::output_report::taxonomy_severity_vo::Severity::MEDIUM => "warning",
                        shared::output_report::taxonomy_severity_vo::Severity::LOW => "note",
                        shared::output_report::taxonomy_severity_vo::Severity::INFO => "note",
                    },
                    "message": { "text": r.message.value },
                    "locations": [{
                        "physicalLocation": {
                            "artifactLocation": { "uri": r.file.value() },
                            "region": {
                                "startLine": r.line.value(),
                                "startColumn": r.column.value()
                            }
                        }
                    }]
                })
            })
            .collect();

        let sarif = serde_json::json!({
            "$schema": "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-2.1.0.json",
            "version": "2.1.0",
            "runs": [{
                "tool": { "driver": { "name": "lint-arwaky" } },
                "results": results
            }]
        });

        serde_json::to_string_pretty(&sarif).unwrap_or_else(|_| "{}".to_string())
    }

    fn format_junit(&self, report_data: &ArchitectureGovernanceEntity) -> String {
        let total = report_data.results.len();
        let failures: usize = report_data
            .results
            .iter()
            .filter(|r| {
                !matches!(
                    r.severity,
                    shared::output_report::taxonomy_severity_vo::Severity::INFO
                )
            })
            .count();

        let mut xml = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<testsuite name="lint-arwaky" tests="{}" failures="{}">"#,
            total, failures
        );

        for result in report_data.results.iter() {
            let severity_attr = format!("{:?}", result.severity).to_lowercase();
            let message = result
                .message
                .value
                .replace('&', "&amp;")
                .replace('<', "&lt;")
                .replace('>', "&gt;");
            let file = result.file.value();

            xml.push_str(&format!(
                r#"
  <testcase name="{}:{}" classname="{}">
    <failure type="{}" message="{}">{}:{}</failure>
  </testcase>"#,
                result.code.code(), severity_attr, file, severity_attr, message, file, result.line.value()
            ));
        }

        xml.push_str("\n</testsuite>");
        xml
    }
}
