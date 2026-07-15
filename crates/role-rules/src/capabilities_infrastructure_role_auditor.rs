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
            && (content.contains("_port") || content.contains("_protocol"));
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
