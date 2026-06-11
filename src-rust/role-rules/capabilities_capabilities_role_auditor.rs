// PURPOSE: CapabilitiesRoleChecker — AES0303: detect capability routing bottlenecks (single bottleneck, missing dispatch)
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::output_report::taxonomy_severity_vo::Severity;
use crate::role_rules::contract_capabilities_role_protocol::ICapabilitiesRoleChecker;
use crate::shared_common::taxonomy_name_vo::SymbolName;
use crate::shared_common::taxonomy_source_vo::SourceContentVO;
use crate::shared_common::taxonomy_violation_message_rs_error::AesViolation;

pub struct CapabilitiesRoleChecker {}

impl Default for CapabilitiesRoleChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl CapabilitiesRoleChecker {
    pub fn new() -> Self {
        Self {}
    }

    pub fn check_capability_routing(
        &self,
        source: &SourceContentVO,
        layer: &str,
        violations: &mut Vec<LintResult>,
    ) {
        if layer != "capabilities" && !layer.starts_with("capabilities(") {
            return;
        }
        let file = source.file_path.value();
        let content = source.content.value();
        let structs: Vec<&str> = content
            .lines()
            .filter_map(|l| {
                let t = l.trim();
                let words: Vec<&str> = t.split_whitespace().collect();
                if (t.starts_with("pub struct ") || t.starts_with("struct ")) && words.len() >= 2 {
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
                    AesViolation::CapabilityRouting {
                        struct_name: SymbolName::new(*s),
                        reason: None,
                    },
                ));
            }
        }
    }
}

impl ICapabilitiesRoleChecker for CapabilitiesRoleChecker {
    fn check_capability_routing(
        &self,
        source: &SourceContentVO,
        layer: &str,
        violations: &mut Vec<LintResult>,
    ) {
        self.check_capability_routing(source, layer, violations);
    }
}
