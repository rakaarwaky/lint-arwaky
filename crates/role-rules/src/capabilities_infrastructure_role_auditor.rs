// PURPOSE: InfrastructureRoleChecker — IInfrastructureRoleChecker for AES404: infrastructure has no port implementation
//
// ALGORITHM:
//   1. check_port_implementation checks if the file imports a port/protocol
//      (contains `_port::` or `_protocol::` after `use `). If the file has an
//      `infrastructure_` prefix but does NOT import any port, emits an
//      InfrastructureNoPort violation — the file is either not real infrastructure
//      or is missing a contract port.
//   2. If the file DOES import a port/protocol but has no `impl ... for ...`
//      block, emits InfrastructureNoPort violation — the port is declared
//      but not implemented.
//
// NOTE: This is a simple keyword-based heuristic. It may miss cases where the
//      implementation is in a different file or uses a different pattern.
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::role_rules::contract_infrastructure_role_protocol::IInfrastructureRoleChecker;
use shared::role_rules::taxonomy_violation_role_vo::AesRoleViolation;
use shared::taxonomy_source_vo::SourceContentVO;

pub struct InfrastructureRoleChecker {}

impl Default for InfrastructureRoleChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl InfrastructureRoleChecker {
    pub fn new() -> Self {
        Self {}
    }
}

impl IInfrastructureRoleChecker for InfrastructureRoleChecker {
    fn check_port_implementation(
        &self,
        source: &SourceContentVO,
        violations: &mut Vec<LintResult>,
    ) {
        let file = source.file_path.value();
        let content = source.content.value();
        let li = crate::taxonomy_language_helper::detect_language(source);

        if li.is_rs {
            self._check_rust(file, content, violations);
        } else if li.is_py {
            self._check_python(file, content, violations);
        } else if li.is_js {
            self._check_js(file, content, violations);
        }
    }
}

impl InfrastructureRoleChecker {
    fn _check_rust(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let has_import = content.contains("use ")
            && (content.contains("_port::") || content.contains("_protocol::"));
        if !has_import {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES404",
                Severity::HIGH,
                AesRoleViolation::InfrastructureNoPort { reason: None }.to_string(),
            ));
            return;
        }
        let has_impl = content.contains("impl ")
            && (content.contains(" for ")
                || content.contains(" for")
                || content.contains("impl<T")
                || content.contains("impl<"));
        if !has_impl {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES404",
                Severity::HIGH,
                AesRoleViolation::InfrastructureNoPort { reason: None }.to_string(),
            ));
        }
    }

    fn _check_python(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let has_import = (content.contains("import ") || content.contains("from "))
            && (content.contains("_port") || content.contains("_protocol"));
        if !has_import {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES404",
                Severity::HIGH,
                AesRoleViolation::InfrastructureNoPort { reason: None }.to_string(),
            ));
            return;
        }
        let has_impl = content.contains("class ")
            && (content.contains("(_port") || content.contains("(_protocol"));
        if !has_impl {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES404",
                Severity::HIGH,
                AesRoleViolation::InfrastructureNoPort { reason: None }.to_string(),
            ));
        }
    }

    fn _check_js(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let has_import = content.contains("import ")
            && (content.contains("_port") || content.contains("_protocol"));
        if !has_import {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES404",
                Severity::HIGH,
                AesRoleViolation::InfrastructureNoPort { reason: None }.to_string(),
            ));
            return;
        }
        let has_impl = content.contains("implements");
        if !has_impl {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES404",
                Severity::HIGH,
                AesRoleViolation::InfrastructureNoPort { reason: None }.to_string(),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::common::taxonomy_path_vo::FilePath;
    use shared::taxonomy_source_vo::{ContentString, SourceContentVO};

    fn make_source(file: &str, content: &str, language: &str) -> SourceContentVO {
        let fp = FilePath::new(file.to_string()).unwrap_or_default();
        SourceContentVO::new(fp, ContentString::new(content.to_string()), language)
    }

    #[test]
    fn rust_with_port_import_no_violation() {
        let checker = InfrastructureRoleChecker::new();
        let mut violations = Vec::new();
        let source = make_source(
            "infrastructure_test_adapter.rs",
            "use shared::contract::some_port::SomeTrait;\nimpl SomeTrait for TestAdapter {}",
            "rust",
        );
        checker.check_port_implementation(&source, &mut violations);
        assert!(violations.is_empty(), "Expected no violation for Rust with port import + impl");
    }

    #[test]
    fn rust_without_port_import_emits_violation() {
        let checker = InfrastructureRoleChecker::new();
        let mut violations = Vec::new();
        let source = make_source(
            "infrastructure_random.rs",
            "fn helper() -> i32 { 42 }",
            "rust",
        );
        checker.check_port_implementation(&source, &mut violations);
        assert_eq!(violations.len(), 1, "Expected 1 violation for missing port import");
        assert!(violations[0].code.to_string().contains("AES404"));
    }

    #[test]
    fn python_with_port_import_no_violation() {
        let checker = InfrastructureRoleChecker::new();
        let mut violations = Vec::new();
        let source = make_source(
            "infrastructure_test_adapter.py",
            "from somewhere import _port_interface\nclass TestAdapter(_port_interface): pass",
            "python",
        );
        checker.check_port_implementation(&source, &mut violations);
        assert!(violations.is_empty(), "Expected no violation for Python with port import");
    }

    #[test]
    fn js_without_port_import_emits_violation() {
        let checker = InfrastructureRoleChecker::new();
        let mut violations = Vec::new();
        let source = make_source(
            "infrastructure_foo.js",
            "function helper() { return 42; }",
            "javascript",
        );
        checker.check_port_implementation(&source, &mut violations);
        assert_eq!(violations.len(), 1, "Expected 1 violation for missing port import");
        assert!(violations[0].code.to_string().contains("AES404"));
    }

    #[test]
    fn python_without_port_import_emits_violation() {
        let checker = InfrastructureRoleChecker::new();
        let mut violations = Vec::new();
        let source = make_source(
            "infrastructure_random.py",
            "x = 1",
            "python",
        );
        checker.check_port_implementation(&source, &mut violations);
        assert_eq!(violations.len(), 1, "Expected 1 violation for missing port import");
        assert!(violations[0].code.to_string().contains("AES404"));
    }
}
