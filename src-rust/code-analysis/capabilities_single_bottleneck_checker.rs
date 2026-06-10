// PURPOSE: SingleBottleneckChecker — ISingleBottleneckProtocol for AES0303: detect oversized capability files
use crate::code_analysis::contract_single_bottleneck_protocol::ISingleBottleneckProtocol;
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::output_report::taxonomy_severity_vo::Severity;
use crate::shared_common::taxonomy_violation_message_rs_error::AesViolation;

pub struct SingleBottleneckChecker {}

impl Default for SingleBottleneckChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl SingleBottleneckChecker {
    pub fn new() -> Self {
        Self {}
    }
}

impl ISingleBottleneckProtocol for SingleBottleneckChecker {
    fn check_single_bottleneck(
        &self,
        file: &str,
        content: &str,
        layer: &str,
        violations: &mut Vec<LintResult>,
    ) {
        if layer != "capabilities" && !layer.starts_with("capabilities(") {
            return;
        }
        // Count functions across languages: Rust `fn`, Python `def`, JS `function`
        let fc = content.matches("fn ").count()
            + content.matches("def ").count()
            + content.matches("function ").count();
        // Count impl/class blocks: Rust `impl`, Python/JS `class`
        let ic = content.matches("impl ").count() + content.matches("class ").count();
        if fc > 30 {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES0303",
                Severity::MEDIUM,
                &format!("{} Found {} functions.", AesViolation::SingleBottleneck, fc),
            ));
        }
        if ic > 5 {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES0303",
                Severity::MEDIUM,
                &format!("{} Found {} impl/class blocks.", AesViolation::SingleBottleneck, ic),
            ));
        }
    }
}
