use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::common::taxonomy_language_vo::Language;
use shared::common::taxonomy_severity_vo::Severity;
use shared::common::utility_language_detector::detect_language_info_from_source;
use shared::common::utility_signature_parser::{
    extract_python_method_signatures, extract_trait_method_signatures,
    extract_typescript_method_signatures, python_signature_uses_forbidden_primitive,
    signature_uses_forbidden_primitive, typescript_signature_uses_forbidden_primitive,
};
use shared::role_rules::contract_role_contract_protocol::IContractRoleChecker;
use shared::role_rules::taxonomy_violation_role_vo::AesRoleViolation;
use shared::taxonomy_source_vo::SourceContentVO;

// PURPOSE: ContractRoleChecker — IContractRoleChecker for AES402: contract primitive type audits
//
// ALGORITHM:
//   check_contract_primitive (protocol dispatch) — Detects primitive type employment
//   in contract method signatures. Uses LanguageDetector to determine language, then
//   delegates signature parsing to shared utility functions.

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ContractRoleChecker {}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IContractRoleChecker for ContractRoleChecker {
    fn check_protocol(&self, source: &SourceContentVO) -> Vec<LintResult> {
        let mut violations = Vec::new();
        self.check_contract_primitive(source, &mut violations);
        violations
    }

    fn check_aggregate(&self, source: &SourceContentVO) -> Vec<LintResult> {
        let mut violations = Vec::new();
        self.check_contract_primitive(source, &mut violations);
        violations
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
}
