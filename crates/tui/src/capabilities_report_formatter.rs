use shared::common::taxonomy_display_content_vo::DisplayContent;
use shared::config_system::taxonomy_source_vo::ConfigResult;
use shared::project_setup::taxonomy_doctor_vo::{DependencyReport, ToolchainDiagnostics};
use shared::tui::contract_report_formatter_protocol::IReportFormatterProtocol;
use shared::tui::taxonomy_lint_result_vo::LintExecutionResult;

// (No protocol implementation found in this file)

// PURPOSE: Taxonomy-layer report formatter helper — provides formatting functions for scan results, toolchain diagnostics, dependencies, and active configurations.

use shared::cli_commands::taxonomy_result_vo::LintResultList;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ReportFormatterHelper;

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IReportFormatterProtocol for ReportFormatterHelper {
    fn format_results(&self, results: &LintResultList) -> DisplayContent {
        if results.is_empty() {
            return DisplayContent::new("No violations found.");
        }
        let mut output = format!("Found {} violation(s):\n\n", results.len());
        for (i, r) in results.iter().enumerate() {
            let src = r
                .source
                .as_ref()
                .map(|s| s.to_string())
                .unwrap_or_else(|| "unknown".into());
            output.push_str(&format!(
                "{}. [{}] {}:{} — {}\n   Code: {} | Severity: {}\n\n",
                i + 1,
                src,
                r.file,
                r.line.value,
                r.message,
                r.code,
                r.severity
            ));
        }
        DisplayContent::new(output)
    }

    fn format_doctor_report(&self, diagnostics: &ToolchainDiagnostics) -> LintExecutionResult {
        let mut output = format!(
            "Environment Diagnostics\nBinary: {}\n\n",
            diagnostics.binary_path
        );
        let mut fail_count = 0;
        for (name, tools) in [
            ("Rust Tools", &diagnostics.rust_tools),
            ("Python Tools", &diagnostics.python_tools),
            ("JS/TS Tools", &diagnostics.js_tools),
            ("VCS Tools", &diagnostics.vcs_tools),
        ] {
            output.push_str(&format!("== {} ==\n", name));
            for tool in tools {
                let icon = match tool.status.as_str() {
                    "OK" => "\u{2713}",
                    "WARN" => "\u{26A0}",
                    "FAIL" => {
                        fail_count += 1;
                        "\u{2717}"
                    }
                    _ => "?",
                };
                let note = match tool.status.as_str() {
                    "WARN" => " (optional)",
                    "FAIL" => " (required)",
                    _ => "",
                };
                output.push_str(&format!(
                    "  {} {} {}{}\n",
                    icon, tool.name, tool.version, note
                ));
            }
            output.push('\n');
        }
        if fail_count == 0 {
            output.push_str("All required tools OK.\n");
        } else {
            output.push_str(&format!("{} required tool(s) missing!\n", fail_count));
        }
        LintExecutionResult::success(output, fail_count)
    }

    fn format_dependency_report(
        &self,
        path: &str,
        report: &DependencyReport,
    ) -> LintExecutionResult {
        let count = report.dependencies.len();
        let mut output = format!(
            "Dependency scan for {}\nLanguage: {}\nTotal dependencies: {}\n",
            path, report.language, count
        );
        for dep_type in ["direct", "transitive"] {
            let list: Vec<_> = report
                .dependencies
                .iter()
                .filter(|d| d.dep_type == dep_type)
                .collect();
            if !list.is_empty() {
                output.push_str(&format!(
                    "\n{} ({}) [top 30]:\n",
                    if dep_type == "direct" {
                        "Direct"
                    } else {
                        "Transitive"
                    },
                    list.len()
                ));
                for dep in list.iter().take(30) {
                    output.push_str(&format!("  {} {}\n", dep.name, dep.version));
                }
                if list.len() > 30 {
                    output.push_str(&format!("  ... and {} more\n", list.len() - 30));
                }
            }
        }
        LintExecutionResult::success(output, count)
    }

    fn format_config_result(&self, result: &ConfigResult) -> LintExecutionResult {
        let mut output = String::from("Active Configuration\n");
        output.push_str(&format!(
            "Source: {} ({})\n",
            result.source.path.value, result.source.language
        ));
        if !result.warnings.is_empty() {
            output.push_str("\nWarnings:\n");
            for w in &result.warnings {
                output.push_str(&format!("  - {}\n", w));
            }
        }
        let config = &result.config;
        output.push_str(&format!("\nEnabled: {}\n", config.enabled.value));
        output.push_str(&format!("Layers: {}\n", config.layers.len()));
        output.push_str(&format!("Rules: {}\n", config.rules.len()));
        output.push_str(&format!(
            "Ignored paths: {}\n",
            config.ignored_paths.values.len()
        ));
        output.push_str(&format!(
            "Mandatory class definition: {}\n",
            config.mandatory_class_definition.value
        ));
        output.push_str(&format!(
            "Naming word count: {}\n",
            config.naming.word_count.value
        ));
        if !config.layers.is_empty() {
            output.push_str("\nArchitecture Layers:\n");
            for (name, def) in config.layers.iter() {
                let policy = if def.naming.suffix_policy.value.is_empty() {
                    String::new()
                } else {
                    format!(" (policy: {})", def.naming.suffix_policy.value)
                };
                output.push_str(&format!("  - {}{}\n", name.value, policy));
            }
        }
        if !config.rules.is_empty() {
            output.push_str(&format!("\nRules ({}):\n", config.rules.len()));
            for (i, rule) in config.rules.iter().enumerate() {
                let desc = if rule.description.value.is_empty() {
                    String::new()
                } else if rule.description.value.len() > 60 {
                    format!(" — {}…", &rule.description.value[..60])
                } else {
                    format!(" — {}", rule.description.value)
                };
                output.push_str(&format!(
                    "  {}. {} [{}]{}\n",
                    i + 1,
                    rule.name.value,
                    rule.scope.value,
                    desc
                ));
            }
        }
        LintExecutionResult::success(output, 0)
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for ReportFormatterHelper {
    fn default() -> Self {
        Self::new()
    }
}

impl ReportFormatterHelper {
    pub fn new() -> Self {
        Self
    }
}

