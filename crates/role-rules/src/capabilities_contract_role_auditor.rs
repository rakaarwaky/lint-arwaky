use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::taxonomy_violation_code_analysis_vo::Language;
use shared::common::utility_language_detector::detect_language_info_from_source;
use shared::common::utility_signature_parser::{
    extract_python_method_signatures, extract_trait_method_signatures,
    extract_typescript_method_signatures, python_signature_uses_forbidden_primitive,
    signature_uses_forbidden_primitive, typescript_signature_uses_forbidden_primitive,
};
use shared::role_rules::contract_role_protocol::IContractRoleChecker;
use shared::role_rules::taxonomy_violation_role_vo::AesRoleViolation;
use shared::taxonomy_definition_vo::LayerDefinition;
use shared::taxonomy_source_vo::SourceContentVO;

// PURPOSE: ContractRoleChecker — IContractRoleChecker for AES402: contract primitive type audits
//
// ALGORITHM:
//   1. check_aggregate — Scans import lines for blocked trait patterns (layer + suffix)
//      defined in LayerDefinition.role.forbidden_inheritance. Flags any `impl Trait for X`
//      or equivalent that uses a disallowed trait by name.
//   2. scan_contract_primitive (port/protocol dispatch) — Detects primitive type employment
//      in contract interfaces (port/protocol files). Uses LanguageDetector to determine
//      language, then delegates signature parsing to shared utility functions.

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ContractRoleChecker {}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IContractRoleChecker for ContractRoleChecker {
    fn check_port(
        &self,
        source: &SourceContentVO,
    ) -> Vec<LintResult> {
        let mut violations = Vec::new();
        self.check_contract_primitive(source, &mut violations);
        violations
    }

    fn check_protocol(
        &self,
        source: &SourceContentVO,
    ) -> Vec<LintResult> {
        let mut violations = Vec::new();
        self.check_contract_primitive(source, &mut violations);
        violations
    }

    fn check_aggregate(
        &self,
        source: &SourceContentVO,
        def: &LayerDefinition,
        violations: &mut Vec<LintResult>,
    ) {
        if def.role.forbidden_inheritance.values.is_empty() {
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
            for pattern in &def.role.forbidden_inheritance.values {
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
                    let tn = match name
                        .trim_end_matches(';')
                        .trim()
                        .trim_start_matches('{')
                        .trim_end_matches('}')
                        .split(',')
                        .next()
                    {
                        Some(s) => s.trim().to_string(),
                        None => String::new(),
                    };
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
                let msg = Self::aes013_forbidden_inheritance(trait_name);
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
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for ContractRoleChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl ContractRoleChecker {
    pub fn new() -> Self {
        Self {}
    }

    fn aes013_forbidden_inheritance(trait_name: &str) -> String {
        format!(
            "AES013 FORBIDDEN_INHERITANCE: '{}' implemented from forbidden source.\n\
             WHY? Contracts must not inherit from forbidden source layers.\n\
             FIX: Remove the inheritance or use a valid contract port/protocol instead.",
            trait_name
        )
    }

    /// Detect primitive type usage in contract method signatures (AES402).
    fn check_contract_primitive(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>) {
        let file = source.file_path.value();
        let content = source.content.value();
        let li = detect_language_info_from_source(source);
        let is_rs = li.is_rs;
        let is_py = li.is_py;
        let is_js = li.is_js;
        if !is_rs && !is_py && !is_js {
            return;
        }

        let lang = if is_rs {
            Language::Rust
        } else if is_py {
            Language::Python
        } else {
            Language::JavaScript
        };

        if is_py {
            for (line_no, sig) in extract_python_method_signatures(content) {
                let forbidden = python_signature_uses_forbidden_primitive(&sig);
                if forbidden.is_empty() {
                    continue;
                }
                let msg = AesRoleViolation::ContractPrimitive { reason: None }
                    .with_language(lang)
                    .to_string();
                violations.push(LintResult::new_arch(
                    file,
                    line_no,
                    "AES402",
                    Severity::HIGH,
                    msg,
                ));
            }
            return;
        }

        if is_js {
            for (line_no, sig) in extract_typescript_method_signatures(content) {
                let forbidden = typescript_signature_uses_forbidden_primitive(&sig);
                if forbidden.is_empty() {
                    continue;
                }
                let msg = AesRoleViolation::ContractPrimitive { reason: None }
                    .with_language(lang)
                    .to_string();
                violations.push(LintResult::new_arch(
                    file,
                    line_no,
                    "AES402",
                    Severity::HIGH,
                    msg,
                ));
            }
            return;
        }

        for (line_no, sig) in extract_trait_method_signatures(content) {
            let forbidden = signature_uses_forbidden_primitive(&sig);
            if forbidden.is_empty() {
                continue;
            }
            let msg = AesRoleViolation::ContractPrimitive { reason: None }
                .with_language(lang)
                .to_string();
            violations.push(LintResult::new_arch(
                file,
                line_no,
                "AES402",
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
