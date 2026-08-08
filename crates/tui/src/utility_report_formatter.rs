// PURPOSE: Stateless report formatting helpers for TUI output
//
// Provides formatting function for toolchain diagnostics.
// Pure utility function — no trait impls.

use shared::maintenance::ToolchainDiagnostics;
use shared::tui::LintExecutionResult;

/// Format toolchain diagnostics into a LintExecutionResult.
pub fn format_doctor_report(diagnostics: &ToolchainDiagnostics) -> LintExecutionResult {
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
