// report_formatter_processor — Capability for formatting reports (SARIF, JUnit).
// Implements ILintReportingProtocol: format, get_formatted_payload, to_sarif, to_junit.
use crate::shared_common::taxonomy_governance_entity::ArchitectureGovernanceEntity;
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::output_report::taxonomy_score_vo::FileFormat;
use crate::output_report::taxonomy_severity_vo::Severity;
use /* UNKNOWN: LogOutput */ crate::shared_common::taxonomy_suggestion_vo::LogOutput;
use /* UNKNOWN: ResponseData */ crate::pipeline_jobs::taxonomy_job_vo::ResponseData;
use serde_json::json;
use std::collections::BTreeMap;

/// Business logic for transforming ArchitectureGovernanceEntitys into standard formats.
pub struct ReportFormatterProcessor {}

impl ReportFormatterProcessor {
    pub fn new() -> Self {
        Self {}
    }

    /// Standard entry point for formatting as JSON/Dict.
    pub fn format(&self, report: &ArchitectureGovernanceEntity) -> LogOutput {
        let data = self.report_to_dict(report);
        let json_str = serde_json::to_string_pretty(&data).unwrap_or_else(|_| "{}".to_string());
        LogOutput::new(json_str)
    }

    /// Unified entry point for getting a formatted payload for the Surface layer.
    pub fn get_formatted_payload(
        &self,
        report: &ArchitectureGovernanceEntity,
        output_format: &FileFormat,
    ) -> ResponseData {
        let data = self.report_to_dict(report);

        match output_format.name.as_ref() {
            "sarif" => {
                let sarif = self.to_sarif(report);
                let mut value = ResponseData::new();
                value.value = Some(json!({
                    "format": "sarif",
                    "data": sarif.value
                }));
                value
            }
            "junit" => {
                let junit = self.to_junit(report);
                let mut value = ResponseData::new();
                value.value = Some(json!({
                    "format": "junit",
                    "data": junit.value
                }));
                value
            }
            "json" => {
                let mut value = ResponseData::new();
                value.value = Some(json!({
                    "format": "json",
                    "data": data
                }));
                value
            }
            _ => {
                // Default to text/dict representation
                let mut value = ResponseData::new();
                value.value = Some(json!({
                    "format": "text",
                    "data": data
                }));
                value
            }
        }
    }

    /// Plain-text report format (AES self-lint style).
    pub fn format_text(&self, results: &[LintResult], project_root: &str) -> String {
        let mut lines: Vec<String> = Vec::new();
        lines.push("=".repeat(60));
        lines.push("  AES Architecture Compliance Report (Self-Lint)".to_string());
        lines.push("=".repeat(60));
        lines.push(format!("  Project: {}", project_root));
        lines.push(format!("  Files scanned: {}", results.len()));
        lines.push("=".repeat(60));
        lines.push("".to_string());

        let mut critical = Vec::new();
        let mut high = Vec::new();
        let mut medium = Vec::new();
        let mut low = Vec::new();
        for r in results {
            match r.severity {
                Severity::CRITICAL => critical.push(r),
                Severity::HIGH => high.push(r),
                Severity::MEDIUM => medium.push(r),
                Severity::LOW => low.push(r),
                _ => medium.push(r),
            }
        }
        for (sev, items) in [("CRITICAL", &critical), ("HIGH", &high), ("MEDIUM", &medium), ("LOW", &low)] {
            if items.is_empty() { continue; }
            lines.push(format!("  [{}] {} violations", sev, items.len()));
            lines.push("-".repeat(60));
            for r in items.iter() {
                let code_str = format!("{}", r.code);
                lines.push(format!("  [{}] {}", code_str, r.file.value));
                for msg_line in r.message.value.lines() {
                    lines.push(format!("    {}", msg_line));
                }
            }
            lines.push("".to_string());
        }
        let total = results.len();
        let mut per_code: BTreeMap<String, usize> = BTreeMap::new();
        for r in results {
            *per_code.entry(format!("{}", r.code)).or_insert(0) += 1;
        }
        lines.push("=".repeat(60));
        lines.push(format!("  Total AES Violations: {}", total));
        lines.push(format!("  Total Category AES Violations: {}", per_code.len()));
        if !per_code.is_empty() {
            lines.push("-".repeat(60));
            for (code, count) in &per_code {
                lines.push(format!("  {}: {}", code, count));
            }
        }
        lines.push("".to_string());
        if total == 0 {
            lines.push("  Status: PASS - No AES violations detected".to_string());
        } else {
            lines.push("  Status: FAIL - AES violations detected".to_string());
        }
        lines.push("=".repeat(60));
        lines.join("\n")
    }

    /// Map severity string to SARIF level.
    fn _get_severity(&self, sev: &str) -> &str {
        match sev.to_lowercase().as_ref() {
            "high" => "error",
            "medium" => "warning",
            "low" => "note",
            _ => "warning",
        }
    }

    /// Transform a ArchitectureGovernanceEntity into a SARIF formatted string.
    pub fn to_sarif(&self, report: &ArchitectureGovernanceEntity) -> LogOutput {
        let report_data = self.report_to_dict(report);
        let mut results_list: Vec<serde_json::Value> = Vec::new();

        if let Some(obj) = report_data.as_object() {
            for (adapter_name, adapter_results) in obj {
                if adapter_name == "score"
                    || adapter_name == "is_passing"
                    || adapter_name == "summary"
                {
                    continue;
                }
                if let Some(results_arr) = adapter_results.as_array() {
                    for error in results_arr {
                        let code = error
                            .get("code")
                            .and_then(|v| v.as_str())
                            .unwrap_or("unknown");
                        let severity = error
                            .get("severity")
                            .and_then(|v| v.as_str())
                            .unwrap_or("medium");
                        let message = error.get("message").and_then(|v| v.as_str()).unwrap_or("");
                        let file = error
                            .get("file")
                            .and_then(|v| v.as_str())
                            .unwrap_or("unknown");
                        let line = error.get("line").and_then(|v| v.as_i64()).unwrap_or(1);
                        let column = error.get("column").and_then(|v| v.as_i64()).unwrap_or(1);

                        results_list.push(json!({
                            "ruleId": format!("{}/{}", adapter_name, code),
                            "level": self._get_severity(severity),
                            "message": { "text": message },
                            "locations": [{
                                "physicalLocation": {
                                    "artifactLocation": { "uri": file },
                                    "region": {
                                        "startLine": line,
                                        "startColumn": column
                                    }
                                }
                            }]
                        }));
                    }
                }
            }
        }

        let sarif = json!({
            "$schema": "https://json.schemastore.org/sarif-2.1.0.json",
            "version": "2.1.0",
            "runs": [{
                "tool": { "driver": { "name": "Auto-Linter" } },
                "results": results_list
            }]
        });

        LogOutput::new(serde_json::to_string_pretty(&sarif).unwrap_or_else(|_| "{}".to_string()))
    }

    /// Transform a ArchitectureGovernanceEntity into a JUnit XML formatted string.
    pub fn to_junit(&self, report: &ArchitectureGovernanceEntity) -> LogOutput {
        let report_data = self.report_to_dict(report);
        let mut xml_lines: Vec<String> = Vec::new();
        xml_lines.push(r#"<?xml version="1.0" encoding="UTF-8"?>"#.to_string());

        let mut total_tests: i64 = 0;
        let mut total_failures: i64 = 0;
        let mut testsuites: Vec<String> = Vec::new();

        if let Some(obj) = report_data.as_object() {
            for (adapter_name, adapter_results) in obj {
                if adapter_name == "score"
                    || adapter_name == "is_passing"
                    || adapter_name == "summary"
                {
                    continue;
                }
                if let Some(results_arr) = adapter_results.as_array() {
                    let failure_count = results_arr.len() as i64;
                    let test_count = std::cmp::max(1, failure_count);

                    testsuites.push(format!(
                        r#"  <testsuite name="{}" tests="{}" failures="{}">"#,
                        adapter_name, test_count, failure_count
                    ));

                    if failure_count == 0 {
                        testsuites.push(format!(
                            r#"    <testcase name="lint_{}" classname="{}"/>"#,
                            adapter_name, adapter_name
                        ));
                        total_tests += 1;
                    } else {
                        for (i, error) in results_arr.iter().enumerate() {
                            let msg = xml_escape(
                                error.get("message").and_then(|v| v.as_str()).unwrap_or(""),
                            );
                            testsuites.push(format!(
                                r#"    <testcase name="lint_{}_{}" classname="{}">"#,
                                adapter_name, i, adapter_name
                            ));
                            testsuites.push(format!(
                                r#"      <failure message="Linting failed">{}</failure>"#,
                                msg
                            ));
                            testsuites.push("    </testcase>".to_string());
                            total_tests += 1;
                            total_failures += 1;
                        }
                    }

                    testsuites.push("  </testsuite>".to_string());
                }
            }
        }

        xml_lines.push(format!(
            r#"<testsuites name="Auto-Linter" tests="{}" failures="{}">"#,
            total_tests, total_failures
        ));
        xml_lines.extend(testsuites);
        xml_lines.push("</testsuites>".to_string());

        LogOutput::new(xml_lines.join("\n"))
    }

    /// Converts ArchitectureGovernanceEntity entity to a plain dictionary for formatting.
    pub fn report_to_dict(&self, report: &ArchitectureGovernanceEntity) -> serde_json::Value {
        let violation_count = report.violation_count().value;
        let adapter_count = report
            .results
            .values
            .iter()
            .map(|r| {
                r.source
                    .as_ref()
                    .map(|s| s.value.clone())
                    .unwrap_or_default()
            })
            .collect::<std::collections::HashSet<_>>()
            .len();

        let mut data = serde_json::json!({
            "score": report.score.value,
            "is_passing": report.is_passing.value,
            "summary": {
                "violation_count": violation_count,
                "adapter_count": adapter_count
            }
        });

        // Group results by source adapter
        let mut by_source: std::collections::HashMap<String, Vec<serde_json::Value>> =
            std::collections::HashMap::new();

        for result in &report.results.values {
            let source_name = result
                .source
                .as_ref()
                .map(|s| s.value.clone())
                .unwrap_or_else(|| "unknown".to_string());

            let mut entry = serde_json::json!({
                "file": result.file.to_string(),
                "line": result.line.value,
                "column": result.column.value,
                "code": result.code.to_string(),
                "message": result.message.to_string(),
                "severity": result.severity.to_string(),
                "enclosing_scope": result.enclosing_scope.as_ref().map(|s| s.to_string())
            });

            // Remove null enclosing_scope for cleaner output
            if entry
                .get("enclosing_scope")
                .and_then(|v| v.as_null())
                .is_some()
            {
                entry.as_object_mut().map(|m| m.remove("enclosing_scope"));
            }

            by_source.entry(source_name).or_default().push(entry);
        }

        if let Some(obj) = data.as_object_mut() {
            for (source_name, results) in by_source {
                obj.insert(source_name, serde_json::Value::Array(results));
            }
        }

        data
    }
}

/// Escape special XML characters.
fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use super::{xml_escape, ArchitectureGovernanceEntity, ReportFormatterProcessor};

    #[test]
    fn test_get_severity() {
        let formatter = ReportFormatterProcessor::new();
        assert_eq!(formatter._get_severity("high"), "error");
        assert_eq!(formatter._get_severity("medium"), "warning");
        assert_eq!(formatter._get_severity("low"), "note");
        assert_eq!(formatter._get_severity("unknown"), "warning");
    }

    #[test]
    fn test_xml_escape() {
        assert_eq!(xml_escape("a < b & c > d"), "a &lt; b &amp; c &gt; d");
        assert_eq!(xml_escape(r#"he said "hi""#), "he said &quot;hi&quot;");
    }

    #[test]
    fn test_report_to_dict() {
        let formatter = ReportFormatterProcessor::new();
        let report = ArchitectureGovernanceEntity::new();
        let dict = formatter.report_to_dict(&report);
        assert_eq!(dict["score"], 100.0);
        assert_eq!(dict["is_passing"], true);
    }
}
