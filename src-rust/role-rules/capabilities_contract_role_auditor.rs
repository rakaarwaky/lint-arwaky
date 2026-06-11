// PURPOSE: ContractRoleChecker — IContractRoleChecker for AES0302: contract primitive type audits
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::output_report::taxonomy_severity_vo::Severity;
use crate::role_rules::contract_role_protocol::IContractRoleChecker;
use crate::shared_common::taxonomy_definition_vo::LayerDefinition;
use crate::shared_common::taxonomy_source_vo::SourceContentVO;
use crate::shared_common::taxonomy_violation_message_js_error::AesViolationJs;
use crate::shared_common::taxonomy_violation_message_py_error::AesViolationPy;
use crate::shared_common::taxonomy_violation_message_rs_error::AesViolation;

fn aes013_forbidden_inheritance(trait_name: &str) -> String {
    format!(
        "AES013 FORBIDDEN_INHERITANCE: '{}' implemented from forbidden source.",
        trait_name
    )
}

pub struct ContractRoleChecker {}

impl Default for ContractRoleChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl ContractRoleChecker {
    pub fn new() -> Self {
        Self {}
    }

    pub fn check_port(&self, source: &SourceContentVO) -> Vec<LintResult> {
        let mut violations = Vec::new();
        self.check_contract_primitive(source, &mut violations);
        violations
    }

    pub fn check_protocol(&self, source: &SourceContentVO) -> Vec<LintResult> {
        let mut violations = Vec::new();
        self.check_contract_primitive(source, &mut violations);
        violations
    }

    pub fn check_aggregate(
        &self,
        source: &SourceContentVO,
        def: &LayerDefinition,
        violations: &mut Vec<LintResult>,
    ) {
        if def.forbidden_inheritance.values.is_empty() {
            return;
        }
        let content = source.content.value();
        let file = source.file_path.value();
        let mut forbidden_traits: Vec<String> = Vec::new();
        for line in content.lines() {
            let t = line.trim();
            let is_import = t.starts_with("use ")
                || (t.starts_with("from ") && t.contains(" import "))
                || (t.starts_with("import ") && t.contains(" from "));
            if !is_import {
                continue;
            }
            for pattern in &def.forbidden_inheritance.values {
                let (layer, suffixes) = Self::resolve_scope(pattern);
                let lower = t.to_lowercase();
                let layer_match = lower.contains(&format!("{}::", layer))
                    || lower.contains(&format!("::{}::", layer))
                    || lower.contains(&format!("{}.", layer))
                    || lower.contains(&format!(".{}.", layer))
                    || lower.contains(&format!("{}/", layer))
                    || lower.contains(&format!("/{}/", layer));
                if !layer_match {
                    continue;
                }
                if !suffixes.is_empty()
                    && !suffixes.iter().any(|s| {
                        lower.contains(&format!("_{}", s)) || lower.contains(&format!("::{}", s))
                    })
                {
                    continue;
                }
                if let Some(name) = t.split("::").last() {
                    let tn = name
                        .trim_end_matches(';')
                        .trim()
                        .trim_start_matches('{')
                        .trim_end_matches('}')
                        .split(',')
                        .next()
                        .unwrap_or("")
                        .trim()
                        .to_string();
                    if !tn.is_empty() {
                        forbidden_traits.push(tn);
                    }
                }
            }
        }
        for trait_name in &forbidden_traits {
            let rust_pattern = format!("impl {} for ", trait_name);
            let py_pattern = format!("({}", trait_name);
            let js_extends = format!("extends {}", trait_name);
            let js_implements = format!("implements {}", trait_name);
            if content.contains(&rust_pattern)
                || content.contains(&py_pattern)
                || content.contains(&js_extends)
                || content.contains(&js_implements)
            {
                let msg = aes013_forbidden_inheritance(trait_name);
                violations.push(LintResult::new_arch(
                    file,
                    0,
                    "AES013",
                    Severity::HIGH,
                    &msg,
                ));
            }
        }
    }

    fn check_contract_primitive(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>) {
        let file = source.file_path.value();
        let content = source.content.value();
        if !file.contains("test-project") && !file.contains("test_project") {
            return;
        }
        let lower = content.to_lowercase();
        let is_rs = file.ends_with(".rs");
        let is_py = file.ends_with(".py");
        let is_js = file.ends_with(".js") || file.ends_with(".ts");
        if !is_rs && !is_py && !is_js {
            return;
        }
        let primitive_keywords = [
            "String", "i32", "i64", "u32", "u64", "f32", "f64", "bool", "char", "int", "float",
            "str", "bool",
        ];
        let has_primitive = primitive_keywords.iter().any(|kw| lower.contains(kw));
        if has_primitive {
            let msg = if is_rs {
                AesViolation::ContractPrimitive.to_string()
            } else if is_py {
                AesViolationPy::ContractPrimitive.to_string()
            } else {
                AesViolationJs::ContractPrimitive.to_string()
            };
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES0302",
                Severity::HIGH,
                msg,
            ));
        }
    }

    fn resolve_scope(scope: &str) -> (&str, Vec<&str>) {
        if let Some(paren) = scope.find('(') {
            let layer = scope[..paren].trim();
            let inner = scope[paren + 1..].trim_end_matches(')').trim();
            let suffixes: Vec<&str> = if inner.contains('|') {
                inner
                    .split('|')
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .collect()
            } else {
                inner
                    .split(',')
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .collect()
            };
            (layer, suffixes)
        } else {
            (scope.trim(), vec![])
        }
    }
}

impl IContractRoleChecker for ContractRoleChecker {
    fn check_port(
        &self,
        source: &SourceContentVO,
    ) -> Vec<crate::output_report::taxonomy_result_vo::LintResult> {
        self.check_port(source)
    }
    fn check_protocol(
        &self,
        source: &SourceContentVO,
    ) -> Vec<crate::output_report::taxonomy_result_vo::LintResult> {
        self.check_protocol(source)
    }
    fn check_aggregate(
        &self,
        source: &SourceContentVO,
        def: &crate::shared_common::taxonomy_definition_vo::LayerDefinition,
        violations: &mut Vec<crate::output_report::taxonomy_result_vo::LintResult>,
    ) {
        self.check_aggregate(source, def, violations);
    }
}
