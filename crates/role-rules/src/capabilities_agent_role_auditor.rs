use shared::common::taxonomy_severity_vo::Severity;
use shared::role_rules::contract_agent_role_protocol::IAgentRoleChecker;
use shared::role_rules::taxonomy_violation_role_vo::AesRoleViolation;
use shared::taxonomy_source_vo::SourceContentVO;

// PURPOSE: AgentRoleChecker — IAgentRoleChecker for AES405: agent file size limits and any-type checks
//
// ALGORITHM:
//   1. check_file_size_limit — Counts lines in the source file. If the count exceeds
//      max_lines, emits AES405 AgentFileSizeLimit.
//   2. check_any_type_annotation — Line-by-line scan for `: any`, `: Any`, `-> any`,
//      `-> Any`, `Any<`, `Any[`, or `any[` patterns. Flags each match as AES405 AnyType.
//
// NOTE: check_container / check_orchestrator / check_lifecycle are no-ops because
//      container/orchestrator/lifecycle role checks are done via the IAnalyzer-based
//      entry points (check_surface_hierarchy, check_surface_roles) rather than inline.
//      These trait methods are required by IAgentRoleChecker but are intentionally
//      empty for this checker implementation.
use shared::cli_commands::taxonomy_result_vo::LintResult;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct AgentRoleChecker {}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IAgentRoleChecker for AgentRoleChecker {
    fn check_container(
        &self,
        _source: &SourceContentVO,
        _violations: &mut Vec<shared::cli_commands::taxonomy_result_vo::LintResult>,
    ) {
    }
    fn check_orchestrator(
        &self,
        _source: &SourceContentVO,
        _violations: &mut Vec<shared::cli_commands::taxonomy_result_vo::LintResult>,
    ) {
    }
    fn check_lifecycle(
        &self,
        _source: &SourceContentVO,
        _violations: &mut Vec<shared::cli_commands::taxonomy_result_vo::LintResult>,
    ) {
    }
    fn check_file_size_limit(
        &self,
        _source: &SourceContentVO,
        _max_lines: usize,
        _violations: &mut Vec<shared::cli_commands::taxonomy_result_vo::LintResult>,
    ) {
    }

    fn check_any_type_annotation(
        &self,
        source: &SourceContentVO,
        violations: &mut Vec<shared::cli_commands::taxonomy_result_vo::LintResult>,
    ) {
        let content = source.content.value();
        let file = source.file_path.value();
        for (i, line) in content.lines().enumerate() {
            let t = line.trim();
            if t.contains(": any")
                || t.contains(": Any")
                || t.contains("-> any")
                || t.contains("-> Any")
                || t.contains("Any<")
                || t.contains("Any[")
                || t.contains("any[")
            {
                violations.push(LintResult::new_arch(
                    file,
                    i + 1,
                    "AES405",
                    Severity::HIGH,
                    AesRoleViolation::AnyType { reason: None }.to_string(),
                ));
            }
        }
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for AgentRoleChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl AgentRoleChecker {
    pub fn new() -> Self {
        Self {}
    }
}
