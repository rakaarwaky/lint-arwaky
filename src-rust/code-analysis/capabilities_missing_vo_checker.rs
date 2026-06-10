// PURPOSE: MissingVoChecker — IMissingVoProtocol for AES0303/AES0304: detect raw literals in capabilities/infrastructure
use crate::code_analysis::contract_missing_vo_protocol::IMissingVoProtocol;
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::output_report::taxonomy_severity_vo::Severity;
use crate::shared_common::taxonomy_violation_message_rs_error::AesViolation;

pub struct MissingVoChecker {}

impl Default for MissingVoChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl MissingVoChecker {
    pub fn new() -> Self {
        Self {}
    }
}

impl IMissingVoProtocol for MissingVoChecker {
    fn check_missing_vo(
        &self,
        file: &str,
        content: &str,
        layer: &str,
        violations: &mut Vec<LintResult>,
    ) {
        let is_cap = layer == "capabilities" || layer.starts_with("capabilities(");
        let is_infra = layer == "infrastructure" || layer.starts_with("infrastructure(");
        if !is_cap && !is_infra {
            return;
        }
        for (i, line) in content.lines().enumerate() {
            let t = line.trim();
            // Rust: `let x = "literal"` or `let x = 42`
            // JS: `const x = "literal"` or `let x = "literal"` or `var x = "literal"`
            // Python: `x = "literal"` or `x = 42`
            let is_assignment = t.starts_with("let ")
                || t.starts_with("const ")
                || t.starts_with("var ")
                || (t.contains(" = ")
                    && !t.starts_with("//")
                    && !t.starts_with("#")
                    && !t.starts_with("if")
                    && !t.starts_with("else")
                    && !t.starts_with("return")
                    && !t.starts_with("import")
                    && !t.starts_with("from")
                    && !t.starts_with("def ")
                    && !t.starts_with("class ")
                    && !t.starts_with("fn ")
                    && !t.starts_with("pub ")
                    && !t.starts_with("self."));
            if is_assignment && t.contains(" = ") {
                let rhs = t
                    .split(" = ")
                    .nth(1)
                    .unwrap_or("")
                    .trim_end_matches(';')
                    .trim_end_matches(',');
                if (rhs.starts_with('"') && rhs.ends_with('"') && !rhs.contains("::"))
                    || (rhs.starts_with('\'') && rhs.ends_with('\'') && rhs.len() <= 3)
                    || rhs.parse::<i64>().is_ok()
                    || rhs.parse::<f64>().is_ok()
                {
                    let (code, msg) = if is_cap {
                        ("AES0303", AesViolation::MissingVo)
                    } else {
                        ("AES0304", AesViolation::InfrastructureMissingVo)
                    };
                    violations.push(LintResult::new_arch(
                        file,
                        i + 1,
                        code,
                        Severity::MEDIUM,
                        msg,
                    ));
                }
            }
        }
    }
}
