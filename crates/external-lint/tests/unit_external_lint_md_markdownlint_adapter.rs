// Unit tests for MarkdownLintAdapter — markdownlint output parsing.
use external_lint_lint_arwaky::capabilities_md_markdownlint_adapter::MarkdownLintAdapter;

#[allow(dead_code, unused_imports)]
#[path = "../../shared/tests/common/mock_filesystem.rs"]
mod mock_filesystem;

use mock_filesystem::MockFilesystem;
use shared_common::taxonomy_adapter_name_vo::AdapterName;
use shared_common::taxonomy_common_vo::PatternList;
use shared_common::taxonomy_compliance_vo::ComplianceStatus;
use shared_common::taxonomy_operation_error::LinterOperationError;
use shared_common::taxonomy_path_vo::FilePath;
use shared_common::taxonomy_response_data_vo::ResponseData;
use shared_common::taxonomy_severity_vo::Severity;
use shared_common::taxonomy_tool_name_vo::ToolName;
use shared_external_lint::ICommandExecutorProtocol;
use shared_external_lint::IJsToolResolutionProtocol;
use shared_external_lint::contract_external_lint_protocol::ILinterAdapterProtocol;
use shared_external_lint::taxonomy_duration_vo::Timeout;
use std::sync::{Arc, Mutex};

/// Mock command executor that returns canned stdout for each candidate binary.
struct MockMdExecutor {
    outputs: Mutex<Vec<String>>,
    calls: Mutex<Vec<(String, Vec<String>)>>,
}

impl MockMdExecutor {
    /// One canned stdout per candidate binary, in the order the adapter tries
    /// them: `markdownlint-cli` then `markdownlint-cli2`.
    fn new(outputs: Vec<&str>) -> Self {
        Self {
            outputs: Mutex::new(outputs.iter().map(|s| s.to_string()).collect()),
            calls: Mutex::new(Vec::new()),
        }
    }
}

impl ICommandExecutorProtocol for MockMdExecutor {
    fn execute_command(
        &self,
        command: PatternList,
        _: FilePath,
        _: Option<Timeout>,
    ) -> anyhow::Result<ResponseData> {
        // `execute_command` carries the whole argv; the binary is argv[0].
        let argv: Vec<String> = command.iter().map(|p| p.to_string()).collect();
        let binary = argv.first().cloned().unwrap_or_default();
        self.calls
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .push((binary, argv));
        let mut outputs = self.outputs.lock().unwrap_or_else(|e| e.into_inner());
        let stdout = if outputs.is_empty() {
            String::new()
        } else {
            outputs.remove(0)
        };
        let mut response = ResponseData::new();
        response.stdout = stdout;
        Ok(response)
    }
    fn health_check(&self) -> anyhow::Result<ResponseData> {
        Ok(ResponseData::new())
    }
    fn exec_cmd_scan(
        &self,
        cmd: Vec<String>,
        _: FilePath,
        _: f64,
        _: Option<AdapterName>,
        _: &FilePath,
    ) -> Result<ResponseData, LinterOperationError> {
        self.execute_command(PatternList::new(cmd), FilePath::default(), None)
            .map_err(|e| {
                LinterOperationError::Adapter(shared_common::AdapterError::new(
                    AdapterName::raw("markdownlint"),
                    shared_common::taxonomy_common_vo::ErrorMessage::new(e.to_string()),
                ))
            })
    }
    fn exec_cmd_adapter(
        &self,
        cmd: Vec<String>,
        wd: FilePath,
        timeout_secs: f64,
        adapter_name: AdapterName,
    ) -> Result<ResponseData, LinterOperationError> {
        self.exec_cmd_scan(
            cmd,
            wd,
            timeout_secs,
            Some(adapter_name),
            &FilePath::default(),
        )
    }
}

/// Mock JS tool resolution. Records every `(binary, args)` pair it is asked for
/// and resolves the binaries named in `available`.
struct MockJsResolution {
    available: Vec<String>,
    seen: Mutex<Vec<(String, Vec<String>)>>,
}

impl MockJsResolution {
    fn with(available: &[&str]) -> Self {
        Self {
            available: available.iter().map(|s| s.to_string()).collect(),
            seen: Mutex::new(Vec::new()),
        }
    }
}

impl IJsToolResolutionProtocol for MockJsResolution {
    fn resolve_js_cmd(
        &self,
        tool: &ToolName,
        args: Vec<String>,
        _: &FilePath,
    ) -> Option<Vec<String>> {
        self.seen
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .push((tool.value().to_string(), args.clone()));
        self.available
            .iter()
            .find(|b| b.as_str() == tool.value())
            .map(|b| {
                let mut cmd = vec![b.clone()];
                cmd.extend(args);
                cmd
            })
    }

    fn resolve_js_working_dir(&self, path: &FilePath) -> FilePath {
        path.clone()
    }

    fn js_apply_fix(
        &self,
        _: &FilePath,
        _: &ToolName,
        _: &str,
    ) -> Result<ComplianceStatus, LinterOperationError> {
        Ok(ComplianceStatus::new(false))
    }
}

fn make_adapter(
    outputs: Vec<&str>,
    available: &[&str],
) -> (
    MarkdownLintAdapter,
    Arc<MockMdExecutor>,
    Arc<MockJsResolution>,
) {
    let executor = Arc::new(MockMdExecutor::new(outputs));
    let js = Arc::new(MockJsResolution::with(available));
    let io: Arc<dyn shared_filesystem::IFileSystemIOProtocol> = Arc::new(MockFilesystem::new());
    let tr: Arc<dyn shared_filesystem::IToolResolutionProtocol> = Arc::new(MockFilesystem::new());
    let lint_exec: Arc<dyn ICommandExecutorProtocol> = executor.clone();
    let js_res: Arc<dyn IJsToolResolutionProtocol> = js.clone();
    let adapter = MarkdownLintAdapter::new(lint_exec, js_res, io, tr);
    (adapter, executor, js)
}

fn readme() -> FilePath {
    FilePath::new("/tmp/readme.md".to_string()).unwrap_or_default()
}

/// Two issues as `markdownlint-cli --json` emits them.
const JSON_ISSUES: &str = r#"[
  {
    "fileName": "/tmp/readme.md",
    "lineNumber": 1,
    "ruleNames": ["MD041", "first-line-heading", "first-line-h1"],
    "ruleDescription": "First line in a file should be a top-level heading",
    "errorDetail": null,
    "errorContext": "Title",
    "errorRange": null,
    "fixInfo": null
  },
  {
    "fileName": "/tmp/readme.md",
    "lineNumber": 5,
    "ruleNames": ["MD018", "no-missing-space-atx"],
    "ruleDescription": "No space after hash on atx style heading",
    "errorDetail": null,
    "errorContext": "Bad",
    "errorRange": [2, 3],
    "fixInfo": { "editColumn": 2, "insertText": " " }
  }
]"#;

/// Two issues as `markdownlint-cli2` writes them, one with a column and one
/// without, plus the banner and summary lines that must be ignored.
const TEXT_ISSUES: &str = "\
markdownlint-cli2 v0.23.3 (markdownlint v0.41.1)
Finding: /tmp/readme.md
Linting: 1 file
Summary: 3 issues in 1 file
/tmp/readme.md:1:1 error MD018/no-missing-space-atx No space after hash on atx style heading [Context: \"#Title\"]
/tmp/readme.md:1 error MD041/first-line-heading/first-line-h1 First line in a file should be a top-level heading [Context: \"#Title\"]
/tmp/readme.md:3:33 error MD009/no-trailing-spaces Trailing spaces [Expected: 0 or 2; Actual: 3]
";

// ─── Adapter identity ───

#[test]
fn adapter_name_is_markdownlint() {
    let (adapter, _, _) = make_adapter(vec![], &[]);
    assert_eq!(adapter.name().value(), "markdownlint");
}

// ─── JSON format (markdownlint-cli) ───

#[test]
fn json_output_is_parsed_into_lint_results() {
    let (adapter, _, _) = make_adapter(vec![JSON_ISSUES], &["markdownlint-cli"]);
    let results = adapter.scan(&readme()).unwrap_or_default();

    assert_eq!(results.values.len(), 2, "expected 2 findings");
    assert_eq!(results.values[0].line.value(), 1);
    assert_eq!(results.values[0].code.code(), "markdownlint::MD041");
    assert!(
        results.values[0]
            .message
            .value()
            .contains("First line in a file should be a top-level heading")
    );
    assert_eq!(results.values[1].line.value(), 5);
    assert_eq!(results.values[1].code.code(), "markdownlint::MD018");
}

#[test]
fn json_context_is_appended_to_the_message() {
    let (adapter, _, _) = make_adapter(vec![JSON_ISSUES], &["markdownlint-cli"]);
    let results = adapter.scan(&readme()).unwrap_or_default();
    assert!(
        results.values[0].message.value().contains("[Context:"),
        "got {}",
        results.values[0].message.value()
    );
}

#[test]
fn empty_json_array_yields_empty_results() {
    let (adapter, _, _) = make_adapter(vec!["[]"], &["markdownlint-cli"]);
    let results = adapter.scan(&readme()).unwrap_or_default();
    assert!(results.values.is_empty());
}

#[test]
fn malformed_json_falls_through_to_text_and_stays_empty() {
    let (adapter, _, _) = make_adapter(
        vec!["not valid json", "not valid json"],
        &["markdownlint-cli", "markdownlint-cli2"],
    );
    let results = adapter.scan(&readme()).unwrap_or_default();
    assert!(results.values.is_empty());
}

#[test]
fn entry_without_rule_names_falls_back_to_unknown_code() {
    let json = r#"[{"fileName": "/tmp/readme.md", "lineNumber": 3}]"#;
    let (adapter, _, _) = make_adapter(vec![json], &["markdownlint-cli"]);
    let results = adapter.scan(&readme()).unwrap_or_default();
    assert_eq!(results.values.len(), 1);
    assert_eq!(results.values[0].code.code(), "markdownlint::unknown");
}

// ─── Text format (markdownlint-cli2) ───

#[test]
fn text_output_is_parsed_into_lint_results() {
    let (adapter, _, _) = make_adapter(vec![TEXT_ISSUES], &["markdownlint-cli2"]);
    let results = adapter.scan(&readme()).unwrap_or_default();

    assert_eq!(results.values.len(), 3, "expected 3 findings");
    assert_eq!(results.values[0].code.code(), "markdownlint::MD018");
    assert_eq!(results.values[0].column.value(), 1);
    assert_eq!(results.values[1].code.code(), "markdownlint::MD041");
    assert_eq!(
        results.values[1].column.value(),
        1,
        "missing column defaults to 1"
    );
    assert_eq!(results.values[2].code.code(), "markdownlint::MD009");
    assert_eq!(results.values[2].line.value(), 3);
    assert_eq!(results.values[2].column.value(), 33);
}

#[test]
fn text_banner_and_summary_lines_are_ignored() {
    let (adapter, _, _) = make_adapter(vec![TEXT_ISSUES], &["markdownlint-cli2"]);
    let results = adapter.scan(&readme()).unwrap_or_default();
    for finding in &results.values {
        assert!(
            !finding.message.value().contains("Linting:"),
            "banner leaked into a message: {}",
            finding.message.value()
        );
    }
}

#[test]
fn text_context_suffix_is_stripped_from_the_message() {
    let (adapter, _, _) = make_adapter(vec![TEXT_ISSUES], &["markdownlint-cli2"]);
    let results = adapter.scan(&readme()).unwrap_or_default();
    let message = results.values[0].message.value();
    assert_eq!(message, "No space after hash on atx style heading");
}

#[test]
fn text_without_error_keywords_yields_empty_results() {
    let (adapter, _, _) = make_adapter(
        vec!["markdownlint-cli2 v0.23.3\nSummary: 0 issues in 0 files\n"],
        &["markdownlint-cli2"],
    );
    let results = adapter.scan(&readme()).unwrap_or_default();
    assert!(results.values.is_empty());
}

// ─── Path canonicalization ───

/// markdownlint-cli2 prints paths relative to its working directory; the
/// adapter must re-anchor them onto the scan root so findings land on the
/// same absolute paths the rest of the report uses.
#[test]
fn text_relative_paths_are_canonicalized_onto_the_root() {
    let relative = "\
bad.md:1:1 error MD018/no-missing-space-atx No space after hash [Context: \"#Title\"]\n";
    let (adapter, _, _) = make_adapter(vec![relative], &["markdownlint-cli2"]);
    let results = adapter.scan(&readme()).unwrap_or_default();

    assert_eq!(results.values.len(), 1);
    assert_eq!(
        results.values[0].file.value(),
        "/tmp/readme.md/bad.md",
        "relative file path must be joined onto the scan root"
    );
}

#[test]
fn text_absolute_paths_are_kept_verbatim() {
    let (adapter, _, _) = make_adapter(vec![TEXT_ISSUES], &["markdownlint-cli2"]);
    let results = adapter.scan(&readme()).unwrap_or_default();
    for finding in &results.values {
        assert!(
            std::path::Path::new(finding.file.value()).is_absolute(),
            "got {}",
            finding.file.value()
        );
    }
}

#[test]
fn json_relative_paths_are_canonicalized_onto_the_root() {
    let json = r#"[
      {
        "fileName": "notes/overview.md",
        "lineNumber": 2,
        "ruleNames": ["MD033", "no-inline-html"],
        "ruleDescription": "Inline HTML",
        "errorDetail": null,
        "errorContext": "<div>",
        "errorRange": null,
        "fixInfo": null
      }
    ]"#;
    let (adapter, _, _) = make_adapter(vec![json], &["markdownlint-cli"]);
    let results = adapter.scan(&readme()).unwrap_or_default();

    assert_eq!(results.values.len(), 1);
    assert_eq!(
        results.values[0].file.value(),
        "/tmp/readme.md/notes/overview.md"
    );
}

#[test]
fn json_absolute_paths_are_kept_verbatim() {
    let (adapter, _, _) = make_adapter(vec![JSON_ISSUES], &["markdownlint-cli"]);
    let results = adapter.scan(&readme()).unwrap_or_default();
    for finding in &results.values {
        assert!(
            std::path::Path::new(finding.file.value()).is_absolute(),
            "got {}",
            finding.file.value()
        );
    }
}

// ─── Findings shape ───

#[test]
fn every_finding_carries_the_markdownlint_source_and_medium_severity() {
    for output in [JSON_ISSUES, TEXT_ISSUES] {
        let (adapter, _, _) =
            make_adapter(vec![output], &["markdownlint-cli", "markdownlint-cli2"]);
        let results = adapter.scan(&readme()).unwrap_or_default();
        assert!(
            !results.values.is_empty(),
            "expected findings for {output:.20}"
        );
        for finding in &results.values {
            let source = finding.source.as_ref().expect("source is set");
            assert_eq!(source.value(), "markdownlint");
            assert_eq!(finding.severity, Severity::MEDIUM);
        }
    }
}

// ─── Tool resolution ───

#[test]
fn json_capable_binary_is_tried_before_the_text_only_binary() {
    let (adapter, _, js) = make_adapter(
        vec![JSON_ISSUES],
        &["markdownlint-cli", "markdownlint-cli2"],
    );
    let _ = adapter.scan(&readme()).unwrap_or_default();

    let seen = js.seen.lock().unwrap_or_else(|e| e.into_inner());
    assert_eq!(
        seen.len(),
        1,
        "stopped at the first binary that reported findings"
    );
    assert_eq!(seen[0].0, "markdownlint-cli");
    assert!(
        seen[0].1.contains(&"--json".to_string()),
        "got {:?}",
        seen[0].1
    );
}

#[test]
fn text_only_binary_is_used_when_json_binary_reports_nothing() {
    let (adapter, _, js) = make_adapter(
        vec!["[]", TEXT_ISSUES],
        &["markdownlint-cli", "markdownlint-cli2"],
    );
    let results = adapter.scan(&readme()).unwrap_or_default();

    assert_eq!(results.values.len(), 3, "fell through to the text binary");
    let seen = js.seen.lock().unwrap_or_else(|e| e.into_inner());
    assert_eq!(seen.len(), 2);
    assert_eq!(seen[0].0, "markdownlint-cli");
    assert_eq!(seen[1].0, "markdownlint-cli2");
}

#[test]
fn no_markdown_tool_available_yields_empty_results() {
    let (adapter, _, _) = make_adapter(vec![], &[]);
    let results = adapter.scan(&readme()).unwrap_or_default();
    assert!(results.values.is_empty());
}

#[test]
fn command_passes_the_json_flag_and_the_target_path() {
    let (adapter, executor, _) = make_adapter(vec![JSON_ISSUES], &["markdownlint-cli"]);
    let _ = adapter.scan(&readme()).unwrap_or_default();

    let calls = executor.calls.lock().unwrap_or_else(|e| e.into_inner());
    let argv = &calls[0].1;
    assert!(argv.contains(&"--json".to_string()), "got {argv:?}");
    assert!(argv.contains(&"/tmp/readme.md".to_string()), "got {argv:?}");
}

// ─── Graceful degradation ───

#[test]
fn empty_stdout_yields_empty_results() {
    let (adapter, _, _) = make_adapter(vec![""], &["markdownlint-cli"]);
    let results = adapter.scan(&readme()).unwrap_or_default();
    assert!(results.values.is_empty());
}

#[test]
fn non_markdown_target_does_not_panic() {
    let (adapter, _, _) = make_adapter(vec![JSON_ISSUES], &["markdownlint-cli"]);
    let path = FilePath::new("/tmp/main.rs".to_string()).unwrap_or_default();
    let results = adapter.scan(&path).unwrap_or_default();
    // The mock filesystem reports every path as a directory, so the extension
    // guard cannot fire; the scan still returns a well-formed result.
    assert!(results.values.len() <= 2);
}

#[test]
fn fix_returns_a_status_without_error() {
    let (adapter, _, _) = make_adapter(vec![JSON_ISSUES], &["markdownlint-cli"]);
    let status = adapter.fix(&readme());
    assert!(status.is_ok(), "fix must return a status, not an error");
}

#[test]
fn fix_falls_back_to_cli2_when_only_that_variant_is_installed() {
    // The common host has only `markdownlint-cli2` installed globally. Probing
    // `markdownlint-cli` first must not stop the fix from running.
    let (adapter, executor, _) = make_adapter(vec![""], &["markdownlint-cli2"]);
    adapter.fix(&readme()).expect("fix must not error");

    let calls = executor.calls.lock().unwrap_or_else(|e| e.into_inner());
    assert_eq!(calls.len(), 1, "exactly one CLI invocation: {calls:?}");
    let (binary, argv) = &calls[0];
    assert_eq!(binary, "markdownlint-cli2");
    assert!(argv.contains(&"--fix".to_string()), "got {argv:?}");
    assert!(argv.contains(&"/tmp/readme.md".to_string()), "got {argv:?}");
}

#[test]
fn fix_prefers_cli_when_both_variants_are_installed() {
    let (adapter, executor, _) = make_adapter(vec![""], &["markdownlint-cli", "markdownlint-cli2"]);
    adapter.fix(&readme()).expect("fix must not error");

    let calls = executor.calls.lock().unwrap_or_else(|e| e.into_inner());
    assert_eq!(
        calls.len(),
        1,
        "the first resolvable binary wins: {calls:?}"
    );
    assert_eq!(calls[0].0, "markdownlint-cli");
}

#[test]
fn fix_is_a_noop_status_when_no_cli_is_installed() {
    let (adapter, executor, _) = make_adapter(vec![""], &[]);
    let status = adapter.fix(&readme()).expect("fix must not error");

    assert!(
        !status.value(),
        "an unresolvable CLI reports no fix applied"
    );
    assert!(
        executor
            .calls
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .is_empty(),
        "no binary may be spawned when none resolves"
    );
}

// ─── OutputNormalizer mapping for the new tool ───

#[test]
fn normalizer_prefixes_markdownlint_codes() {
    use shared_external_lint::contract_external_lint_protocol::INormalizeProtocol;
    let normalizer = external_lint_lint_arwaky::OutputNormalizer;
    let root = FilePath::new("/tmp".to_string()).unwrap_or_default();
    let (results, warnings) = normalizer.normalize(
        &ToolName::new("markdownlint"),
        r#"[{"file":"readme.md","line":3,"column":1,"code":"MD041","message":"first line"}]"#,
        &root,
    );
    assert_eq!(results.values.len(), 1);
    assert_eq!(results.values[0].code.code(), "markdownlint::MD041");
    assert!(warnings.is_empty());
}

#[test]
fn normalizer_maps_unknown_markdownlint_severity_to_medium() {
    use shared_external_lint::contract_external_lint_protocol::INormalizeProtocol;
    let normalizer = external_lint_lint_arwaky::OutputNormalizer;
    let tool = ToolName::new("markdownlint");
    let code = shared_common::taxonomy_error_vo::ErrorCode::raw("markdownlint::MD041");
    assert_eq!(
        normalizer.map_severity(&tool, &code, &Severity::INFO),
        Severity::MEDIUM
    );
}
