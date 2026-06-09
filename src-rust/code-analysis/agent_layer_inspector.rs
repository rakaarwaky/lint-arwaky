//! Inline layer-specific checks — agent role, surface role, bottleneck, missing VO.

use crate::output_report::taxonomy_result_vo::LintResult;
use crate::output_report::taxonomy_severity_vo::Severity;
use crate::shared_common::taxonomy_violationrs_constant::{
    AES031_SURFACE_ROLE_VIOLATION, AES036_SINGLE_BOTTLENECK, AES038_MISSING_VO,
};

use crate::code_analysis::agent_checker_helpers::mk_result;

/// Check that agent files don't exceed 300 lines (AES032).
pub fn check_agent_role(file: &str, content: &str, layer: &str, violations: &mut Vec<LintResult>) {
    if layer != "agent" && !layer.starts_with("agent(") {
        return;
    }
    if content.lines().count() > 300 {
        violations.push(mk_result(
            file,
            0,
            "AES032",
            Severity::HIGH,
            "AES032 AGENT_ROLE: Agent file exceeds 300 lines.",
        ));
    }
}

/// Check that surface files don't have too many functions (AES031).
pub fn check_surface_role(
    file: &str,
    content: &str,
    layer: &str,
    violations: &mut Vec<LintResult>,
) {
    if layer != "surfaces" && !layer.starts_with("surfaces(") {
        return;
    }
    if content.matches("fn ").count() > 15 {
        violations.push(mk_result(
            file,
            0,
            "AES031",
            Severity::HIGH,
            AES031_SURFACE_ROLE_VIOLATION,
        ));
    }
}

/// Check that capabilities files don't exceed function/impl block limits (AES036).
pub fn check_single_bottleneck(
    file: &str,
    content: &str,
    layer: &str,
    violations: &mut Vec<LintResult>,
) {
    if layer != "capabilities" && !layer.starts_with("capabilities(") {
        return;
    }
    let fc = content.matches("fn ").count();
    let ic = content.matches("impl ").count();
    if fc > 30 {
        violations.push(mk_result(
            file,
            0,
            "AES036",
            Severity::MEDIUM,
            &format!("{} Found {} functions.", AES036_SINGLE_BOTTLENECK, fc),
        ));
    }
    if ic > 5 {
        violations.push(mk_result(
            file,
            0,
            "AES036",
            Severity::MEDIUM,
            &format!("{} Found {} impl blocks.", AES036_SINGLE_BOTTLENECK, ic),
        ));
    }
}

/// Check for primitive literal assignments that should use Value Objects (AES038).
pub fn check_missing_vo(
    file: &str,
    content: &str,
    layer: &str,
    violations: &mut Vec<LintResult>,
) {
    let is_cap = layer == "capabilities" || layer.starts_with("capabilities(");
    let is_infra = layer == "infrastructure" || layer.starts_with("infrastructure(");
    if !is_cap && !is_infra {
        return;
    }
    for (i, line) in content.lines().enumerate() {
        let t = line.trim();
        if t.starts_with("let ") && t.contains(" = ") {
            let rhs = t.split(" = ").nth(1).unwrap_or("").trim_end_matches(';');
            if (rhs.starts_with('"') && rhs.ends_with('"') && !rhs.contains("::"))
                || rhs.parse::<i64>().is_ok()
                || rhs.parse::<f64>().is_ok()
            {
                violations.push(mk_result(
                    file,
                    i + 1,
                    "AES038",
                    Severity::MEDIUM,
                    AES038_MISSING_VO,
                ));
            }
        }
    }
}
