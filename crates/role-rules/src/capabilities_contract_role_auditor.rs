// PURPOSE: ContractRoleChecker — IContractRoleChecker for AES402: contract primitive type audits
//
// ALGORITHM:
//   1. check_aggregate — Scans import lines for blocked trait patterns (layer + suffix)
//      defined in LayerDefinition.role.forbidden_inheritance. Flags any `impl Trait for X`
//      or equivalent that uses a disallowed trait by name.
//   2. scan_contract_primitive (port/protocol dispatch) — Detects primitive type employment
//      in contract interfaces (port/protocol files). Uses LanguageDetector to determine
//      language, then checks for known primitive keywords per language.
//
// NOTE: check_contract_primitive is called for all files (not just test projects)
//      since AES402 applies universally — removed test-project guard per DX audit.
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::taxonomy_violation_code_analysis_vo::Language;
use shared::role_rules::contract_role_protocol::IContractRoleChecker;
use shared::role_rules::taxonomy_signature_utils::{
    extract_python_method_signatures, extract_trait_method_signatures,
    extract_typescript_method_signatures, python_signature_uses_forbidden_primitive,
    regex_lite_match_whole_token, signature_uses_forbidden_primitive,
    typescript_signature_uses_forbidden_primitive,
};
use shared::role_rules::taxonomy_violation_role_vo::AesRoleViolation;
use shared::taxonomy_definition_vo::LayerDefinition;
use shared::taxonomy_name_vo::SymbolName;
use shared::taxonomy_source_vo::SourceContentVO;

pub struct ContractRoleChecker {}

#[async_trait::async_trait]
impl IContractRoleChecker for ContractRoleChecker {
    fn check_port(
        &self,
        source: &SourceContentVO,
    ) -> Vec<shared::cli_commands::taxonomy_result_vo::LintResult> {
        self.check_port(source)
    }
    fn check_protocol(
        &self,
        source: &SourceContentVO,
    ) -> Vec<shared::cli_commands::taxonomy_result_vo::LintResult> {
        self.check_protocol(source)
    }
    fn check_aggregate(
        &self,
        source: &SourceContentVO,
        def: &shared::taxonomy_definition_vo::LayerDefinition,
        violations: &mut Vec<shared::cli_commands::taxonomy_result_vo::LintResult>,
    ) {
        self.check_aggregate(source, def, violations);
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
                violations.push(LintResult::new_arch(
                    file,
                    0,
                    "AES013",
                    Severity::HIGH,
                    AesRoleViolation::ForbiddenInheritance {
                        trait_name: SymbolName::new(trait_name.clone()),
                        reason: None,
                    }
                    .to_string(),
                ));
            }
        }
    }

    /// Detect primitive type usage in contract method signatures (AES402).
    ///
    /// Scans ONLY method signatures inside `pub trait Name { ... }` blocks, NOT
    /// the whole file. This prevents false positives from:
    ///   * doc-comments mentioning "String" or "str" in prose
    ///   * identifier names that contain primitive type names
    ///     (e.g. `StringBuilder`, `MyFloat`)
    ///   * language words in English comments ("Float values are rounded")
    ///
    /// Rules:
    ///   * `&str` (borrowed string slice) is allowed — borrow lifetimes preclude
    ///     replacement with a taxonomy VO without major API changes. It is the
    ///     idiomatic Rust type for file paths, error messages, and other borrowed
    ///     string data passed into trait methods.
    ///   * `bool` is allowed — represents a semantic toggle that is not meaningfully
    ///     expressible as a VO without ceremony.
    ///   * `String` (owned) is FORBIDDEN — must be replaced with a taxonomy VO
    ///     (`LintMessage`, `ErrorMessage`, `SymbolName`, `JobId`, etc.).
    ///   * `Result<String, _>` / `Result<&str, _>` are FORBIDDEN — error variants
    ///     must use a defined `taxonomy_*_error` type, not a raw `String`.
    ///   * Numeric primitives `i32/i64/u32/u64/f32/f64` and `char` are FORBIDDEN —
    ///     must be wrapped in domain VOs (`Count`, `LineNumber`, `ColumnNumber`,
    ///     `Duration`, etc.) or new domain-specific VOs.
    ///
    /// Only the parameter types and return type of each trait method signature
    /// are inspected — implementation bodies are out of scope (the contract
    /// layer is the public interface; internal representations are an adapter
    /// concern).
    fn check_contract_primitive(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>) {
        let file = source.file_path.value();
        let content = source.content.value();
        let li = crate::taxonomy_language_info_vo::LanguageInfo::new(source);
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
            // Python-specific: look for class methods with primitive type annotations
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
            // JS/TS-specific: look for interface/class methods with primitive type annotations
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

        // Rust: use trait-method-signature parser
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

impl Default for ContractRoleChecker {
    fn default() -> Self {
        Self::new()
    }
}
