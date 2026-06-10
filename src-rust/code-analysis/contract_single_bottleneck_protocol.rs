// PURPOSE: ISingleBottleneckProtocol — port trait for AES0303: detect oversized capability files (fn > 30, impl > 5)
use crate::output_report::taxonomy_result_vo::LintResult;

pub trait ISingleBottleneckProtocol: Send + Sync {
    fn check_single_bottleneck(
        &self,
        file: &str,
        content: &str,
        layer: &str,
        violations: &mut Vec<LintResult>,
    );
}
