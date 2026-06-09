use crate::output_report::taxonomy_result_vo::LintResult;
use crate::output_report::taxonomy_severity_vo::Severity;

pub struct ArchLayerChecker {}

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
        for line in content.lines() {
            let t = line.trim();
            if t.starts_with("use ")
                && (t.contains("::capabilities::")
                    || t.contains("::infrastructure::")
                    || t.contains("::agent::"))
            {
                violations.push(LintResult::new_arch(
                    file,
                    0,
                    "AES023",
                    Severity::HIGH,
                    "AES023 SURFACE_DEPENDENCY: Surface imports from forbidden layer.",
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
                    Some(t.split_whitespace().nth(1).unwrap_or("").trim_end_matches(';'))
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
                    &format!("AES030 CAPABILITY_ROUTING: Struct '{}' no trait impl.", s),
                ));
            }
        }
    }
}
