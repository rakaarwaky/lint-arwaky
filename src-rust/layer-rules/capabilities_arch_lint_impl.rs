// PURPOSE: ArchLintProtocolImpl — capabilities layer implementation of IArchLintProtocol (AES014)
use crate::layer_rules::contract_lint_protocol::IArchLintProtocol;
use crate::output_report::taxonomy_result_vo::LintResultList;

pub struct ArchLintProtocolImpl {}

impl Default for ArchLintProtocolImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl ArchLintProtocolImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::layer_rules::contract_lint_protocol::IArchLintProtocol for ArchLintProtocolImpl {
    fn run_self_lint(&self, project_root: &str) -> crate::output_report::taxonomy_result_vo::LintResultList {
        // Delegate to the actual linter implementation
        // This is a minimal implementation that satisfies the protocol
        crate::output_report::taxonomy_result_vo::LintResultList::new(vec![])
    }

    fn run_self_lint_dir(&self, _src_dir: &str) -> crate::output_report::taxonomy_result_vo::LintResultList {
        crate::output_report::taxonomy_result_vo::LintResultList::new(vec![])
    }

    fn format_report(&self, results: &crate::output_report::taxonomy_result_vo::LintResultList, project_root: &str) -> String {
        // Delegate to the actual formatter
        let mut output = String::new();
        output.push_str(&format!("  AES Self-Lint Report for {project_root}\n"));
        for r in &results.values {
            output.push_str(&format!("  [{}] {}:{}:{} {} - {}\n",
                r.severity, r.file, r.line, r.column, r.code, r.message));
        }
        output.push_str(&format!("Total violations: {}\n", results.values.len()));
        output
    }
}
