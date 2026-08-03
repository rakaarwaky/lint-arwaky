// PURPOSE: ContractRoleChecker — IContractRoleChecker for AES402: contract primitive type audits
//
// ALGORITHM:
//   Uses FileEntry from the filesystem crate. Detects primitive types in contract
//   method signatures. Uses content line scanning for signature detection since
//   ParseMetadata does not yet expose method parameter/return types.

use shared::common::LintResult;
use shared::common::utility_language_detector::detect_language_info;
use shared::common::utility_signature_parser::{
    extract_python_method_signatures, extract_trait_method_signatures,
    extract_typescript_method_signatures, python_signature_uses_forbidden_primitive,
    signature_uses_forbidden_primitive, typescript_signature_uses_forbidden_primitive,
};
use shared::common::{Language, LintMessage, Severity};
use shared::filesystem::taxonomy_filesystem_vo::FileEntry;
use shared::role_rules::{AesRoleViolation, IContractRoleChecker};

// ─── Block 1: Struct Definition ───────────────────────────
pub struct ContractRoleChecker {}

// ─── Block 2: Protocol Trait Implementation ───────────────
impl IContractRoleChecker for ContractRoleChecker {
    fn check_protocol(&self, file: &FileEntry) -> Vec<LintResult> {
        let mut violations = Vec::new();
        self.check_contract_primitive(file, &mut violations);
        violations
    }

    fn check_aggregate(&self, file: &FileEntry) -> Vec<LintResult> {
        let mut violations = Vec::new();
        self.check_contract_primitive(file, &mut violations);
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

    fn check_contract_primitive(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        let path_str = file.path.to_string_lossy();
        let content = &file.content;
        let fp = match shared::common::FilePath::new(path_str.to_string()) {
            Ok(fp) => fp,
            Err(_) => return,
        };
        let li = detect_language_info(&fp);
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
                let msg = Self::fmt(
                    &AesRoleViolation::ContractPrimitive {
                        reason: Some(LintMessage::new(format!(
                            "Forbidden primitive types in signature: {}",
                            forbidden.join(", ")
                        ))),
                    },
                    lang,
                );
                violations.push(LintResult::new_arch(
                    &path_str,
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
                let msg = Self::fmt(
                    &AesRoleViolation::ContractPrimitive {
                        reason: Some(LintMessage::new(format!(
                            "Forbidden primitive types in signature: {}",
                            forbidden.join(", ")
                        ))),
                    },
                    lang,
                );
                violations.push(LintResult::new_arch(
                    &path_str,
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
            let msg = Self::fmt(
                &AesRoleViolation::ContractPrimitive {
                    reason: Some(LintMessage::new(format!(
                        "Forbidden primitive types in signature: {}",
                        forbidden.join(", ")
                    ))),
                },
                lang,
            );
            violations.push(LintResult::new_arch(
                &path_str,
                line_no,
                "AES402",
                Severity::HIGH,
                msg,
            ));
        }
    }
}
