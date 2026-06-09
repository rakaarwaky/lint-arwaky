//! Inline bypass and agent-wildcard checks.

use std::path::Path;

use crate::code_analysis::agent_checker_helpers::mk_result;
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::output_report::taxonomy_severity_vo::Severity;
use crate::shared_common::taxonomy_violationrs_constant::{
    AES022_BYPASS_COMMENT, AES022_PANIC, AES022_UNWRAP_EXPECT,
};

/// Check for bypass comments and forbidden patterns (unwrap, panic, #[allow]).
pub fn check_bypass_comments(file: &str, content: &str, violations: &mut Vec<LintResult>) {
    let markers = [
        ("H", "noqa"),
        ("H", "type: ignore"),
        ("H", "pylint: disable"),
        ("S", "eslint-disable"),
        ("A", "ts-ignore"),
        ("A", "ts-expect-error"),
        ("S", "NOLINT"),
    ];
    let mkc = |p, k| match p {
        "H" => format!("#{}", k),
        "S" => format!("//{}", k),
        "A" => format!("//@{}", k),
        _ => String::new(),
    };
    let patterns: Vec<String> = markers.iter().map(|&(p, k)| mkc(p, k)).collect();
    let unwrap_pat = [".", "unwrap()"].concat();
    let expect_pat = [".", "expect("].concat();
    let panic_pat = ["panic", "!("].concat();
    for (i, line) in content.lines().enumerate() {
        let t = line.trim();
        if t.starts_with("#[allow(") || t.starts_with("#[expect(") {
            violations.push(mk_result(file, i + 1, "AES022", Severity::CRITICAL, AES022_BYPASS_COMMENT));
            continue;
        }
        for p in &patterns {
            if t.to_lowercase().contains(p.as_str()) {
                violations.push(mk_result(file, i + 1, "AES022", Severity::CRITICAL, AES022_BYPASS_COMMENT));
                break;
            }
        }
        if t.contains(&unwrap_pat) || t.contains(&expect_pat) {
            violations.push(mk_result(file, i + 1, "AES022", Severity::CRITICAL, AES022_UNWRAP_EXPECT));
            continue;
        }
        if t.contains(&panic_pat) {
            violations.push(mk_result(file, i + 1, "AES022", Severity::CRITICAL, AES022_PANIC));
            continue;
        }
    }
}

/// Check for wildcard/bypass patterns in agent-layer files.
pub fn check_agent_any_bypass(file: &str, content: &str, violations: &mut Vec<LintResult>) {
    let filename = Path::new(file)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");
    if !filename.starts_with("agent_") {
        return;
    }
    for (i, line) in content.lines().enumerate() {
        let wc1 = format!("{}*{}", ":", ":");
        let wc2 = format!("{}* {}", "::", "}");
        if line.trim().contains(&wc1) || line.trim().contains(&wc2) {
            violations.push(mk_result(
                file,
                i + 1,
                "AES001",
                Severity::HIGH,
                "AES001 FORBIDDEN_IMPORT: Wildcard import in agent layer.",
            ));
        }
    }
}
