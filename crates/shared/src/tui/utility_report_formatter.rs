// PURPOSE: Stateless report formatting helpers for TUI output
//
// Provides formatting function for toolchain diagnostics.
// Pure utility function — no trait impls.

use crate::LintExecutionResult;
use shared_maintenance::ToolchainDiagnostics;

/// Format toolchain diagnostics into a LintExecutionResult.
pub fn format_doctor_report(diagnostics: &ToolchainDiagnostics) -> LintExecutionResult {
    // Same direct NO_COLOR read as `status_icon` in the CLI doctor formatter;
    // importing utility_tui_theme here would trip AES201 (utility→utility).
    format_doctor_report_with_no_color(diagnostics, std::env::var_os("NO_COLOR").is_some())
}

/// #559: no-color-aware variant, exposed for testability (mirrors
/// `utility_tui_theme::color_with_override`).
pub fn format_doctor_report_with_no_color(
    diagnostics: &ToolchainDiagnostics,
    no_color: bool,
) -> LintExecutionResult {
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
            // #559: fall back to ASCII labels when NO_COLOR is set, matching
            // the glyph_* convention established in #365.
            let icon = if no_color {
                match tool.status.as_str() {
                    "OK" => "OK",
                    "WARN" => "WARN",
                    "FAIL" => {
                        fail_count += 1;
                        "FAIL"
                    }
                    _ => "?",
                }
            } else {
                match tool.status.as_str() {
                    "OK" => "\u{2713}",
                    "WARN" => "\u{26A0}",
                    "FAIL" => {
                        fail_count += 1;
                        "\u{2717}"
                    }
                    _ => "?",
                }
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
