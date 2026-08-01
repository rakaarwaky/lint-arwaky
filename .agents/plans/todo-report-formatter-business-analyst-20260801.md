# Review Plan: report-formatter — Business Analyst

## Summary

A comprehensive business analyst review of the `report-formatter` crate (v1.1.0) was conducted against `crates/report-formatter/FRD.md`, `PRD.md`, `ARCHITECTURE.md`, and `.agents/rules/RULES_AES.md`. The analysis revealed critical logic gaps and architectural violations: `TextFormatter` violates FR-001's self-containment mandate by taking a dependency on `code_analysis` (`ICodeAnalysisAggregate`); `JsonFormatter` only serializes `report.results` instead of producing the full structured JSON report (with `external_results`, `diagnostics`, and `summary`); `SarifFormatter` and `JunitFormatter` omit `external_results` and `diagnostics` (`PARSE_WARN`) required by FR-003 and FR-004; and no test suite exists to verify the 29 QA checklist scenarios specified in FRD v1.1.0.

## Findings by Category

### Requirements Clarity
| # | Severity | Issue | Location (File:Line) | Recommendation |
|---|----------|-------|----------------------|----------------|
| 1 | 🔴**CRITICAL** | `TextFormatter` delegates text formatting to `code_analysis` aggregate instead of being self-contained as mandated by FR-001 | [capabilities_text_formatter.rs:15-17](file:///home/raka/mcp-arwaky/lint-arwaky/.worktree/crates/report-formatter/src/capabilities_text_formatter.rs#L15-L17) | Remove `ICodeAnalysisAggregate` dependency and implement self-contained text report formatting operating directly on `ScanReport`. |
| 2 | 🔴**CRITICAL** | `JsonFormatter` serializes raw `results` array (`report.results`) instead of producing the FR-002 JSON schema (`violations`, `external_results`, `diagnostics`, `summary`) | [capabilities_json_formatter.rs:20](file:///home/raka/mcp-arwaky/lint-arwaky/.worktree/crates/report-formatter/src/capabilities_json_formatter.rs#L20) | Construct dedicated JSON serialization DTO/structure matching FR-002 requirements. |
| 3 | 🟡**WARNING** | `SarifFormatter` omits `external_results`, `diagnostics` (`PARSE_WARN`), and driver `rules` array mandated by FR-003 | [capabilities_sarif_formatter.rs:19](file:///home/raka/mcp-arwaky/lint-arwaky/.worktree/crates/report-formatter/src/capabilities_sarif_formatter.rs#L19) | Update SARIF serialization to include `rules` array in tool metadata and include external results & diagnostics. |
| 4 | 🟡**WARNING** | `JunitFormatter` omits `external_results` and `diagnostics` (`<skipped>` element) specified in FR-004 | [capabilities_junit_formatter.rs:19](file:///home/raka/mcp-arwaky/lint-arwaky/.worktree/crates/report-formatter/src/capabilities_junit_formatter.rs#L19) | Expand `JunitFormatter` to output test cases for external results and skipped test cases for `PARSE_WARN` diagnostics. |

### Business Flow
| # | Severity | Issue | Location (File:Line) | Recommendation |
|---|----------|-------|----------------------|----------------|
| 1 | 🔴**CRITICAL** | Formatted outputs (JSON, SARIF, JUnit) swallow `report.external_results` and `report.diagnostics`, losing audit findings in CI/CD pipelines | [capabilities_json_formatter.rs:20](file:///home/raka/mcp-arwaky/lint-arwaky/.worktree/crates/report-formatter/src/capabilities_json_formatter.rs#L20), [capabilities_sarif_formatter.rs:19](file:///home/raka/mcp-arwaky/lint-arwaky/.worktree/crates/report-formatter/src/capabilities_sarif_formatter.rs#L19), [capabilities_junit_formatter.rs:19](file:///home/raka/mcp-arwaky/lint-arwaky/.worktree/crates/report-formatter/src/capabilities_junit_formatter.rs#L19) | Process all three finding categories (`results`, `external_results`, `diagnostics`) in every formatter implementation. |
| 2 | 🟡**WARNING** | `TextFormatter::format_text` constructs dummy `FilePath::default()` to pass to `code_analysis_linter.format_report` | [capabilities_text_formatter.rs:54-56](file:///home/raka/mcp-arwaky/lint-arwaky/.worktree/crates/report-formatter/src/capabilities_text_formatter.rs#L54-L56) | Eliminate dummy `FilePath` construction by building native text output directly from `ScanReport`. |

### Logic Implementation
| # | Severity | Issue | Location (File:Line) | Recommendation |
|---|----------|-------|----------------------|----------------|
| 1 | 🔴**CRITICAL** | Cross-crate dependency on `code_analysis` aggregate breaks AES layer independence and `report-formatter` self-containment | [capabilities_text_formatter.rs:15](file:///home/raka/mcp-arwaky/lint-arwaky/.worktree/crates/report-formatter/src/capabilities_text_formatter.rs#L15) | Make `TextFormatter` zero-dependency on other rule crates; construct output string directly. |
| 2 | 🟡**WARNING** | `xml_escape` function is private helper inside `capabilities_junit_formatter.rs` instead of re-usable utility function per FR-007 | [capabilities_junit_formatter.rs:83](file:///home/raka/mcp-arwaky/lint-arwaky/.worktree/crates/report-formatter/src/capabilities_junit_formatter.rs#L83) | Expose `xml_escape` or keep as internal helper cleanly exposed according to FR-007 spec. |

### Testability & Acceptance Criteria
| # | Severity | Issue | Location (File:Line) | Recommendation |
|---|----------|-------|----------------------|----------------|
| 1 | 🟡**WARNING** | Zero unit tests exist in `crates/report-formatter` for verifying FR-001 through FR-007 test scenarios | `crates/report-formatter/tests/` | Create unit test suite covering text, JSON, SARIF, JUnit, orchestrator, fallback, and XML escape scenarios. |

### Traceability (FRD → Code)
| # | Severity | Issue | Location (File:Line) | Recommendation |
|---|----------|-------|----------------------|----------------|
| 1 | 🔴**CRITICAL** | FR-001 (Text Format Output) implementation incomplete & non-self-contained | [capabilities_text_formatter.rs](file:///home/raka/mcp-arwaky/lint-arwaky/.worktree/crates/report-formatter/src/capabilities_text_formatter.rs) | Replace implementation with self-contained text formatter. |
| 2 | 🔴**CRITICAL** | FR-002 (JSON Format Output) implementation incomplete (missing fields) | [capabilities_json_formatter.rs](file:///home/raka/mcp-arwaky/lint-arwaky/.worktree/crates/report-formatter/src/capabilities_json_formatter.rs) | Implement full JSON schema matching FR-002. |
| 3 | 🟡**WARNING** | FR-003 (SARIF Format Output) missing `rules` array & diagnostic notes | [capabilities_sarif_formatter.rs](file:///home/raka/mcp-arwaky/lint-arwaky/.worktree/crates/report-formatter/src/capabilities_sarif_formatter.rs) | Add SARIF rules metadata and include diagnostics/external results. |
| 4 | 🟡**WARNING** | FR-004 (JUnit Format Output) missing skipped testcases for `PARSE_WARN` | [capabilities_junit_formatter.rs](file:///home/raka/mcp-arwaky/lint-arwaky/.worktree/crates/report-formatter/src/capabilities_junit_formatter.rs) | Add `<skipped>` testcases for diagnostics. |

## Violations

- **AES201 / Self-Containment**: `capabilities_text_formatter.rs` depends on `shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate`. FRD v1.1.0 explicitly requires `report-formatter` to be self-contained with no dependencies on other rule crates.

## Action Items

- [ ] 🔴 **P1**: Remove `ICodeAnalysisAggregate` dependency from `TextFormatter` and implement self-contained text formatting logic in `capabilities_text_formatter.rs`.
- [ ] 🔴 **P1**: Update `ReportFormatterDeps` and `ReportFormatterOrchestrator` in `agent_report_formatter_orchestrator.rs` to reflect parameterless formatter constructors.
- [ ] 🔴 **P1**: Implement full FR-002 JSON schema in `capabilities_json_formatter.rs` (`violations`, `external_results`, `diagnostics`, `summary`).
- [ ] 🟡 **P2**: Update `capabilities_sarif_formatter.rs` and `taxonomy_sarif_vo.rs` to include `rules` array, `external_results`, and `diagnostics` with SARIF level `note`.
- [ ] 🟡 **P2**: Update `capabilities_junit_formatter.rs` to include `external_results` test cases and `PARSE_WARN` diagnostics with `<skipped>` elements.
- [ ] 🟢 **P3**: Add comprehensive unit tests in `crates/report-formatter/tests/test_report_formatter.rs` verifying all FRD QA scenarios.

## Fixed Code

### `crates/report-formatter/src/capabilities_text_formatter.rs`

```rust
// PURPOSE: TextFormatter — implements IReportFormatterProtocol for self-contained text output
use shared::cli_commands::{Format, ScanReport};
use shared::common::DisplayContent;
use shared::report_formatter::{format_report_default, IReportFormatterProtocol};
use std::collections::BTreeMap;

// ─── Block 1: Struct Definition ───────────────────────────
/// TextFormatter — produces self-contained human-readable text output from ScanReport.
pub struct TextFormatter;

// ─── Block 2: Protocol Trait Implementation ───────────────
#[async_trait::async_trait]
impl IReportFormatterProtocol for TextFormatter {
    fn format(&self, report: &ScanReport, format: Format) -> DisplayContent {
        if format == Format::Text {
            self.format_text(report)
        } else {
            DisplayContent::new(format_report_default(report))
        }
    }

    fn supported_format(&self) -> Format {
        Format::Text
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────
impl TextFormatter {
    pub fn new() -> Self {
        Self
    }

    pub fn format_text(&self, report: &ScanReport) -> DisplayContent {
        let mut out = String::with_capacity(1024);

        out.push_str("==================================================\n");
        out.push_str("              LINT ARWAKY SCAN REPORT             \n");
        out.push_str("==================================================\n\n");

        if report.results.is_empty()
            && report.external_results.is_empty()
            && report.diagnostics.is_empty()
        {
            out.push_str("Clean report: 0 violations found.\n");
            return DisplayContent::new(out);
        }

        // Violations Section
        if !report.results.is_empty() {
            out.push_str("--- AES VIOLATIONS ---\n");
            for r in &report.results {
                out.push_str(&format!(
                    "[{}] {} at {}:{} — {}\n",
                    r.severity, r.code, r.file.value, r.line.value(), r.message.value
                ));
            }
            out.push('\n');
        }

        // External Results Section
        if !report.external_results.is_empty() {
            out.push_str("--- EXTERNAL LINT RESULTS ---\n");
            for r in &report.external_results {
                out.push_str(&format!(
                    "[{}] {} at {}:{} — {}\n",
                    r.severity, r.code, r.file.value, r.line.value(), r.message.value
                ));
            }
            out.push('\n');
        }

        // Diagnostics Section
        if !report.diagnostics.is_empty() {
            out.push_str("--- DIAGNOSTICS (PARSE_WARN) ---\n");
            for d in &report.diagnostics {
                out.push_str(&format!(
                    "[WARNING] {} (source: {}) — {}\n",
                    d.code, d.source, d.message
                ));
            }
            out.push('\n');
        }

        // Severity breakdown & violation counts by code
        let mut counts: BTreeMap<String, usize> = BTreeMap::new();
        let mut crit = 0;
        let mut high = 0;
        let mut med = 0;
        let mut low = 0;

        for r in &report.results {
            *counts.entry(r.code.to_string()).or_default() += 1;
            match r.severity {
                shared::common::taxonomy_severity_vo::Severity::CRITICAL => crit += 1,
                shared::common::taxonomy_severity_vo::Severity::HIGH => high += 1,
                shared::common::taxonomy_severity_vo::Severity::MEDIUM => med += 1,
                shared::common::taxonomy_severity_vo::Severity::LOW => low += 1,
                _ => {}
            }
        }

        out.push_str("--- SUMMARY ---\n");
        out.push_str(&format!(
            "Total Violations: {} (Critical: {}, High: {}, Medium: {}, Low: {})\n",
            report.results.len(),
            crit,
            high,
            med,
            low
        ));

        if let Some(score) = report.score {
            out.push_str(&format!("Compliance Score: {:.1}%\n", score));
        }

        DisplayContent::new(out)
    }
}

impl Default for TextFormatter {
    fn default() -> Self {
        Self::new()
    }
}
```

### `crates/report-formatter/src/capabilities_json_formatter.rs`

```rust
// PURPOSE: JsonFormatter — implements IReportFormatterProtocol for full JSON output per FR-002
use serde::Serialize;
use shared::cli_commands::{Format, ScanReport};
use shared::common::DisplayContent;
use shared::report_formatter::{format_report_default, IReportFormatterProtocol};

#[derive(Serialize)]
struct JsonViolation<'a> {
    file: &'a str,
    line: i64,
    code: &'a str,
    severity: String,
    message: &'a str,
}

#[derive(Serialize)]
struct JsonDiagnostic<'a> {
    code: &'a str,
    severity: &'static str,
    message: &'a str,
}

#[derive(Serialize)]
struct JsonSummary {
    total_violations: usize,
    critical: usize,
    high: usize,
    medium: usize,
    low: usize,
    score: Option<f64>,
}

#[derive(Serialize)]
struct JsonReportDto<'a> {
    violations: Vec<JsonViolation<'a>>,
    external_results: Vec<JsonViolation<'a>>,
    diagnostics: Vec<JsonDiagnostic<'a>>,
    summary: JsonSummary,
}

pub struct JsonFormatter;

#[async_trait::async_trait]
impl IReportFormatterProtocol for JsonFormatter {
    fn format(&self, report: &ScanReport, format: Format) -> DisplayContent {
        if format == Format::Json {
            self.format_json(report)
        } else {
            DisplayContent::new(format_report_default(report))
        }
    }

    fn supported_format(&self) -> Format {
        Format::Json
    }
}

impl JsonFormatter {
    pub fn new() -> Self {
        Self
    }

    pub fn format_json(&self, report: &ScanReport) -> DisplayContent {
        let mut crit = 0;
        let mut high = 0;
        let mut med = 0;
        let mut low = 0;

        let violations: Vec<JsonViolation> = report
            .results
            .iter()
            .map(|r| {
                match r.severity {
                    shared::common::taxonomy_severity_vo::Severity::CRITICAL => crit += 1,
                    shared::common::taxonomy_severity_vo::Severity::HIGH => high += 1,
                    shared::common::taxonomy_severity_vo::Severity::MEDIUM => med += 1,
                    shared::common::taxonomy_severity_vo::Severity::LOW => low += 1,
                    _ => {}
                }
                JsonViolation {
                    file: &r.file.value,
                    line: r.line.value(),
                    code: r.code.as_str(),
                    severity: r.severity.to_string(),
                    message: &r.message.value,
                }
            })
            .collect();

        let external_results: Vec<JsonViolation> = report
            .external_results
            .iter()
            .map(|r| JsonViolation {
                file: &r.file.value,
                line: r.line.value(),
                code: r.code.as_str(),
                severity: r.severity.to_string(),
                message: &r.message.value,
            })
            .collect();

        let diagnostics: Vec<JsonDiagnostic> = report
            .diagnostics
            .iter()
            .map(|d| JsonDiagnostic {
                code: &d.code,
                severity: "WARNING",
                message: &d.message,
            })
            .collect();

        let summary = JsonSummary {
            total_violations: report.results.len(),
            critical: crit,
            high,
            medium: med,
            low,
            score: report.score,
        };

        let dto = JsonReportDto {
            violations,
            external_results,
            diagnostics,
            summary,
        };

        DisplayContent::new(serde_json::to_string_pretty(&dto).unwrap_or_else(|_| "{}".to_string()))
    }
}

impl Default for JsonFormatter {
    fn default() -> Self {
        Self::new()
    }
}
```

### `crates/report-formatter/src/agent_report_formatter_orchestrator.rs`

```rust
// PURPOSE: ReportFormatterOrchestrator — implements IReportFormatterAggregate
use shared::cli_commands::{Format, ScanReport};
use shared::common::DisplayContent;
use shared::report_formatter::{IReportFormatterAggregate, IReportFormatterProtocol};
use std::sync::Arc;

pub struct ReportFormatterDeps {
    pub text: Arc<dyn IReportFormatterProtocol>,
    pub json: Arc<dyn IReportFormatterProtocol>,
    pub sarif: Arc<dyn IReportFormatterProtocol>,
    pub junit: Arc<dyn IReportFormatterProtocol>,
}

pub struct ReportFormatterOrchestrator {
    deps: ReportFormatterDeps,
}

impl IReportFormatterAggregate for ReportFormatterOrchestrator {
    fn format(&self, report: &ScanReport, format: Format) -> DisplayContent {
        let formatter: &dyn IReportFormatterProtocol = match format {
            Format::Text => self.deps.text.as_ref(),
            Format::Json => self.deps.json.as_ref(),
            Format::Sarif => self.deps.sarif.as_ref(),
            Format::Junit => self.deps.junit.as_ref(),
        };
        formatter.format(report, format)
    }
}

impl ReportFormatterOrchestrator {
    pub fn new(deps: ReportFormatterDeps) -> Self {
        Self { deps }
    }
}
```
