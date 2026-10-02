// PURPOSE: `ignored_rules:` filtering — a project silences individual linter
// rules (AES or external-tool native) by code substring, with no per-tool
// companion config file.
//
// The filter runs in the orchestrator's post-processing, after every adapter's
// findings are merged, so one list covers all of them. These tests drive the
// orchestrator with a stub adapter that emits a fixed set of codes, which is
// what proves the filter is adapter-agnostic.
#[allow(dead_code, unused_imports)]
#[path = "../../shared/tests/common/mock_filesystem.rs"]
mod mock_filesystem;

use std::collections::HashMap;
use std::sync::Arc;

use external_lint_lint_arwaky::agent_external_lint_orchestrator::{
    ExternalLintDeps, ExternalLintOrchestrator,
};
use external_lint_lint_arwaky::capabilities_external_lint_selector::CapabilitiesExternalLintSelector;
use mock_filesystem::MockFilesystem;
use shared_cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared_common::taxonomy_adapter_name_vo::AdapterName;
use shared_common::taxonomy_common_vo::{ColumnNumber, LineNumber};
use shared_common::taxonomy_error_vo::ErrorCode;
use shared_common::taxonomy_lint_vo::LocationList;
use shared_common::taxonomy_message_vo::{ComplianceStatus, LintMessage};
use shared_common::taxonomy_operation_error::LinterOperationError;
use shared_common::taxonomy_path_vo::FilePath;
use shared_common::taxonomy_severity_vo::Severity;
use shared_external_lint::ExternalLintRequest;
use shared_external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
use shared_external_lint::contract_external_lint_protocol::ILinterAdapterProtocol;
use shared_external_lint::taxonomy_external_lint_vo::ExternalLintContext;

/// Adapter that always reports the same codes, so a test can assert exactly
/// which ones survived suppression.
struct StubAdapter {
    name: &'static str,
    codes: Vec<&'static str>,
}

impl ILinterAdapterProtocol for StubAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::raw(self.name)
    }

    fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        Ok(LintResultList::new(
            self.codes
                .iter()
                .map(|code| LintResult {
                    file: path.clone(),
                    line: LineNumber::new(1),
                    column: ColumnNumber::new(1),
                    code: ErrorCode::raw(*code),
                    message: LintMessage::new("stub finding"),
                    source: Some(AdapterName::raw(self.name)),
                    severity: Severity::MEDIUM,
                    enclosing_scope: Default::default(),
                    related_locations: LocationList::new(),
                })
                .collect(),
        ))
    }

    fn fix(&self, _path: &FilePath) -> Result<ComplianceStatus, LinterOperationError> {
        Ok(ComplianceStatus::new(false))
    }
}

/// Run the orchestrator over a single stub adapter and return the codes that
/// survived `ignored_rules`.
fn surviving_codes(
    adapter_name: &'static str,
    codes: &[&'static str],
    ignored: Vec<String>,
) -> Vec<String> {
    let mut adapters: HashMap<String, Arc<dyn ILinterAdapterProtocol>> = HashMap::new();
    adapters.insert(
        adapter_name.to_string(),
        Arc::new(StubAdapter {
            name: adapter_name,
            codes: codes.to_vec(),
        }),
    );
    let fs = Arc::new(MockFilesystem::new());
    let orchestrator = ExternalLintOrchestrator::new(ExternalLintDeps {
        adapters,
        filesystem: fs.clone(),
        filesystem_io: fs,
        // Every language flag is true, so the default selector picks this
        // adapter regardless of its name; the filter is what is under test.
        selector: Arc::new(CapabilitiesExternalLintSelector::with_defaults()),
    });
    let context = ExternalLintContext {
        has_rust: true,
        has_python: true,
        has_js: true,
        has_markdown: true,
        ignored_rules: ignored,
        config_entries: Vec::new(),
        ignored_paths: Vec::new(),
    };
    orchestrator
        .execute(ExternalLintRequest::scan_all_with_context(
            &FilePath::new("/tmp".to_string()).expect("valid path"),
            &context,
        ))
        .into_violations()
        .values
        .iter()
        .map(|v| v.code.code().to_string())
        .collect()
}

const MD_CODES: [&str; 3] = [
    "markdownlint::MD013",
    "markdownlint::MD024",
    "markdownlint::MD041",
];

#[test]
fn an_empty_list_suppresses_nothing() {
    let codes = surviving_codes("markdownlint", &MD_CODES, Vec::new());
    assert_eq!(codes.len(), 3, "all findings survive: {codes:?}");
}

#[test]
fn a_full_code_suppresses_only_that_finding() {
    let codes = surviving_codes(
        "markdownlint",
        &MD_CODES,
        vec!["markdownlint::MD013".to_string()],
    );
    assert_eq!(
        codes,
        vec![
            "markdownlint::MD024".to_string(),
            "markdownlint::MD041".to_string()
        ]
    );
}

#[test]
fn a_bare_rule_id_suppresses_it_under_any_tool_prefix() {
    // `MD013` rather than `markdownlint::MD013` — the same convenience
    // `scan --filter` offers, so a single-tool project needs one short entry.
    let codes = surviving_codes("markdownlint", &MD_CODES, vec!["MD013".to_string()]);
    assert!(!codes.iter().any(|c| c.contains("MD013")), "{codes:?}");
    assert_eq!(codes.len(), 2, "{codes:?}");
}

#[test]
fn multiple_entries_accumulate() {
    let codes = surviving_codes(
        "markdownlint",
        &MD_CODES,
        vec![
            "markdownlint::MD013".to_string(),
            "markdownlint::MD024".to_string(),
        ],
    );
    assert_eq!(codes, vec!["markdownlint::MD041".to_string()]);
}

#[test]
fn suppression_is_case_insensitive() {
    let codes = surviving_codes(
        "markdownlint",
        &MD_CODES,
        vec!["MARKDOWNLINT::md013".to_string()],
    );
    assert!(!codes.iter().any(|c| c.contains("MD013")), "{codes:?}");
}

#[test]
fn an_entry_matching_nothing_suppresses_nothing() {
    // A stale entry must not degrade into "suppress everything".
    let codes = surviving_codes("markdownlint", &MD_CODES, vec!["MD999".to_string()]);
    assert_eq!(codes.len(), 3, "{codes:?}");
}

#[test]
fn aes_codes_go_through_the_same_list() {
    // The list is code-agnostic on purpose: one switch for every linter, not a
    // second mechanism for the internal rules.
    let codes = surviving_codes(
        "clippy",
        &["AES101", "AES205", "AES403"],
        vec!["AES205".to_string()],
    );
    assert_eq!(codes, vec!["AES101".to_string(), "AES403".to_string()]);
}

#[test]
fn one_entry_can_silence_a_whole_rule_range() {
    let codes = surviving_codes(
        "clippy",
        &["AES101", "AES205", "AES403"],
        vec!["AES2".to_string()],
    );
    assert_eq!(codes, vec!["AES101".to_string(), "AES403".to_string()]);
}

#[test]
fn suppressing_everything_yields_an_empty_report() {
    let codes = surviving_codes(
        "markdownlint",
        &MD_CODES,
        MD_CODES.iter().map(|c| c.to_string()).collect(),
    );
    assert!(codes.is_empty(), "a fully suppressed project reports 0");
}
