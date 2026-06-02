// metric_analyzer_processor — Capability for analyzing code metrics (complexity, size, trends).
// Implements IMetricAnalyzerProtocol: analyze_complexity, analyze_file_size, analyze_quality_trend.

use std::fs;
use crate::taxonomy::{
    AdapterName, ColumnNumber, Count, ErrorCode, FilePath, LineNumber,
    LintMessage, LintResult, LintResultList, Score, Severity,
    ScopeRef, LocationList,
};

pub struct MetricAnalyzerProcessor;

impl MetricAnalyzerProcessor {
    pub fn new() -> Self {
        Self
    }

    fn make_result(file: &str, line: i64, code: &str, msg: &str, source: &str, sev: Severity) -> LintResult {
        LintResult {
            file: FilePath::new(file.to_string()),
            line: LineNumber::new(line),
            column: ColumnNumber::new(0),
            code: ErrorCode::new(code),
            message: LintMessage::new(msg),
            source: AdapterName::new(source),
            severity: sev,
            enclosing_scope: ScopeRef {
                name: "".to_string(),
                kind: "".to_string(),
                file: FilePath::new(""),
                start_line: LineNumber::new(0),
                end_line: LineNumber::new(0),
            },
            related_locations: LocationList::new(Vec::new()),
        }
    }

    fn complexity_severity(complexity: i64) -> Severity {
        if complexity > 20 {
            Severity::HIGH
        } else if complexity > 10 {
            Severity::MEDIUM
        } else {
            Severity::LOW
        }
    }

    /// Interprets raw complexity data into severity-rated results.
    /// `raw_data`: list of (filename, lineno, name, complexity) tuples.
    pub fn analyze_complexity(
        &self,
        raw_data: &[(String, i64, String, i64)],
        threshold: &Count,
    ) -> Vec<LintResult> {
        let limit = threshold.value;
        raw_data.iter()
            .filter(|(_, _, _, complexity)| *complexity > limit)
            .map(|(filename, lineno, name, complexity)| {
                let sev = Self::complexity_severity(*complexity);
                Self::make_result(
                    filename,
                    *lineno,
                    "complexity",
                    &format!("High complexity ({}) in {}", complexity, name),
                    "radon-processor",
                    sev,
                )
            })
            .collect()
    }

    /// Checks if a file exceeds size limits (SRP violation rule).
    pub fn analyze_file_size(
        &self,
        file_path: &str,
        line_count: i64,
        limit: i64,
    ) -> Vec<LintResult> {
        if line_count > limit {
            vec![Self::make_result(
                file_path,
                1,
                "SIZE001",
                &format!(
                    "File exceeds {} lines ({}); potential duplication or SRP violation.",
                    limit, line_count
                ),
                "metric-processor",
                Severity::LOW,
            )]
        } else {
            vec![]
        }
    }

    /// Analyzes if code quality is degrading over time.
    pub fn analyze_quality_trend(
        &self,
        current_score: f64,
        previous_score: f64,
    ) -> Vec<LintResult> {
        if current_score < previous_score {
            vec![Self::make_result(
                "project",
                1,
                "TREND001",
                &format!(
                    "Quality trend is negative: {:.1} -> {:.1}",
                    previous_score, current_score
                ),
                "trend-processor",
                Severity::MEDIUM,
            )]
        } else {
            vec![]
        }
    }
}
