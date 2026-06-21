// PURPOSE: CapabilitiesRoleChecker — AES403: detect capability routing (missing interface implementation)
//
// ALGORITHM:
//   1. check_capability_routing — Scans capabilities-layer files for struct definitions.
//      For each struct, checks if the file contains `impl I{StructName}`, `impl ... for {StructName}`,
//      or `impl {StructName}`. If not and the file has <= 3 structs, flags CapabilityRouting.
//      Skips `#[cfg(test)]` blocks.
//
// NOTE: The layer guard is redundant with the caller but kept for defensive programming.
//      This checker assumes Rust syntax; Python/JS support would need additional parsing.
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::role_rules::contract_capabilities_role_protocol::ICapabilitiesRoleChecker;
use shared::role_rules::taxonomy_violation_role_vo::AesRoleViolation;
use shared::taxonomy_name_vo::SymbolName;
use shared::taxonomy_source_vo::SourceContentVO;

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
        let mut in_cfg_test = false;
        let structs: Vec<&str> = content
            .lines()
            .filter_map(|l| {
                let t = l.trim();
                if t.starts_with("#[cfg(test)]") {
                    in_cfg_test = true;
                    return None;
                }
                if in_cfg_test {
                    if t == "}" || t.starts_with("}") {
                        in_cfg_test = false;
                    }
                    return None;
                }
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
                    "AES403",
                    Severity::MEDIUM,
                    AesRoleViolation::CapabilityRouting {
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
