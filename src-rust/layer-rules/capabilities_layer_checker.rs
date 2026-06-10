// PURPOSE: AES0303 — Detect capability routing bottlenecks (single bottleneck, missing dispatch).
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::output_report::taxonomy_severity_vo::Severity;
use crate::shared_common::taxonomy_violation_rs_constant::{
    aes0303_capability_routing,
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
        // Skip if file has bypass-capability-routing annotation
        let first_lines: Vec<&str> = content.lines().take(30).collect();
        if first_lines
            .iter()
            .any(|l| l.trim() == "// aes: bypass-capability-routing")
        {
            return;
        }
        let structs: Vec<&str> = content
            .lines()
            .filter_map(|l| {
                let t = l.trim();
                let words: Vec<&str> = t.split_whitespace().collect();
                if (t.starts_with("pub struct ") || t.starts_with("struct ")) && words.len() >= 2 {
                    // Find the word "struct" and take the NEXT word as the name
                    let struct_idx = words.iter().position(|w| *w == "struct").unwrap_or(0);
                    Some(
                        words
                            .get(struct_idx + 1)
                            .unwrap_or(&"")
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
                || content.contains(&format!("for {} ", s))
                || content.contains(&format!("for {}{{", s))
                || content.contains(&format!("for {} {{", s))
                || content.contains(&format!("impl {} ", s))
                || content.contains(&format!("impl {}{{", s));
            if !hi && structs.len() <= 3 {
                violations.push(LintResult::new_arch(
                    file,
                    0,
                    "AES0303",
                    Severity::MEDIUM,
                    &aes0303_capability_routing(s),
                ));
            }
        }
    }
}
