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
        let li = crate::taxonomy_language_info_vo::LanguageInfo::new(source);

        if li.is_rs {
            self._check_rust_routing(file, content, violations);
        } else if li.is_py {
            self._check_python_routing(file, content, violations);
        } else if li.is_js {
            self._check_js_routing(file, content, violations);
        }
    }

    fn _check_rust_routing(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let has_proto_import = content.contains("use ")
            && (content.contains("_protocol::") || content.contains("_port::"));
        if !has_proto_import {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES403",
                Severity::MEDIUM,
                AesRoleViolation::CapabilityNoProtocol { reason: None },
            ));
            return;
        }
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
                    Some(match words.get(struct_idx + 1) {
                        Some(w) => w.trim_end_matches(';'),
                        None => "",
                    })
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

    fn _check_js_routing(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let has_proto_import = content.contains("import ")
            && (content.contains("_protocol") || content.contains("_port"));
        if !has_proto_import {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES403",
                Severity::MEDIUM,
                AesRoleViolation::CapabilityNoProtocol { reason: None },
            ));
            return;
        }
        let lines: Vec<&str> = content.lines().collect();
        let mut classes: Vec<(&str, usize)> = Vec::new();
        for (i, l) in lines.iter().enumerate() {
            let t = l.trim();
            if t.starts_with("class ") {
                let name = match t.split_whitespace().nth(1) {
                    Some(n) => match n.split('{').next() {
                        Some(n) => match n.split(':').next() {
                            Some(n) => n.split_whitespace().next().unwrap_or_default(),
                            None => "",
                        },
                        None => "",
                    },
                    None => "",
                };
                if !name.is_empty() && !name.starts_with('_') {
                    classes.push((name, i));
                }
            }
        }
        if classes.len() > 3 {
            return;
        }
        for (name, start_line) in &classes {
            let mut has_method = false;
            for line in lines.iter().skip(start_line + 1).map(|l| l.trim()) {
                if line.starts_with('}') || line.starts_with(';') {
                    break;
                }
                if line.starts_with("function ")
                    || line.starts_with("public ")
                    || line.starts_with("private ")
                    || line.starts_with("protected ")
                    || line.starts_with("static ")
                    || line.starts_with("get ")
                    || line.starts_with("set ")
                    || line.starts_with("async ")
                {
                    has_method = true;
                    break;
                }
            }
            if !has_method {
                violations.push(LintResult::new_arch(
                    file,
                    0,
                    "AES403",
                    Severity::MEDIUM,
                    AesRoleViolation::CapabilityRouting {
                        struct_name: SymbolName::new(*name),
                        reason: None,
                    },
                ));
            }
        }
    }

    fn _check_python_routing(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let has_proto_import = (content.contains("import ") || content.contains("from "))
            && (content.contains("_protocol") || content.contains("_port"));
        if !has_proto_import {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES403",
                Severity::MEDIUM,
                AesRoleViolation::CapabilityNoProtocol { reason: None },
            ));
            return;
        }
        let lines: Vec<&str> = content.lines().collect();
        let mut classes: Vec<(&str, usize)> = Vec::new();
        for (i, l) in lines.iter().enumerate() {
            let t = l.trim();
            if t.starts_with("class ") {
                let name = match t.split_whitespace().nth(1) {
                    Some(n) => n.trim_end_matches(':'),
                    None => "",
                };
                if !name.is_empty() && !name.starts_with('_') {
                    classes.push((name, i));
                }
            }
        }
        if classes.len() > 3 {
            return;
        }
        for (name, start_line) in &classes {
            let mut body_lines = 0;
            let mut has_method = false;
            let mut indent: Option<usize> = None;
            for line in lines.iter().skip(start_line + 1) {
                if line.trim().is_empty() {
                    continue;
                }
                let leading = line.len() - line.trim_start().len();
                if indent.is_none() {
                    if leading == 0 {
                        break;
                    }
                    indent = Some(leading);
                }
                if line.trim_start().starts_with("def ") {
                    has_method = true;
                    break;
                }
                body_lines += 1;
                if body_lines > 20 {
                    break;
                }
            }
            if !has_method {
                violations.push(LintResult::new_arch(
                    file,
                    0,
                    "AES403",
                    Severity::MEDIUM,
                    AesRoleViolation::CapabilityRouting {
                        struct_name: SymbolName::new(*name),
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
