use crate::output_report::taxonomy_result_vo::LintResult;
use crate::output_report::taxonomy_severity_vo::Severity;
use crate::shared_common::taxonomy_violationrs_constant::{
    aes030_capability_routing, AES023_SURFACE_DEPENDENCY,
};

pub struct ArchLayerChecker {}

impl Default for ArchLayerChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl ArchLayerChecker {
    pub fn new() -> Self {
        Self {}
    }

    pub fn check_surface_imports(
        &self,
        file: &str,
        content: &str,
        layer: &str,
        violations: &mut Vec<LintResult>,
    ) {
        if layer != "surfaces" && !layer.starts_with("surfaces(") {
            return;
        }
        for (i, line) in content.lines().enumerate() {
            let t = line.trim();
            if t.starts_with("use ")
                && (t.contains("::capabilities::")
                    || t.contains("::infrastructure::")
                    || t.contains("::agent::"))
            {
                violations.push(LintResult::new_arch(
                    file,
                    i + 1,
                    "AES023",
                    Severity::CRITICAL,
                    AES023_SURFACE_DEPENDENCY,
                ));
                break;
            }
        }
    }

    pub fn check_capability_routing(
        &self,
        file: &str,
        content: &str,
        layer: &str,
        violations: &mut Vec<LintResult>,
    ) {
        if layer != "capabilities" && !layer.starts_with("capabilities(") {
            return;
        }
        let structs: Vec<&str> = content
            .lines()
            .filter_map(|l| {
                let t = l.trim();
                if t.starts_with("pub struct ") || t.starts_with("struct ") {
                    Some(
                        t.split_whitespace()
                            .nth(1)
                            .unwrap_or("")
                            .trim_end_matches(';'),
                    )
                } else {
                    None
                }
            })
            .filter(|n| !n.is_empty() && !n.starts_with('_'))
            .collect();
        for s in &structs {
            let hi = content.contains(&format!("impl I{}", s))
                || content.contains(&format!(" for {} ", s));
            if !hi && structs.len() <= 3 {
                violations.push(LintResult::new_arch(
                    file,
                    0,
                    "AES030",
                    Severity::MEDIUM,
                    &aes030_capability_routing(s),
                ));
            }
        }
    }
}
