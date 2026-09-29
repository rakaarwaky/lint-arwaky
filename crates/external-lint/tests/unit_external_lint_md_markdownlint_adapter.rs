// Unit tests for MarkdownLintAdapter — markdownlint output parsing.
use external_lint_lint_arwaky::capabilities_md_markdownlint_adapter::MarkdownLintAdapter;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_severity_vo::Severity;
use shared::common::taxonomy_tool_name_vo::ToolName;
use shared::external_lint::contract_external_lint_protocol::ILinterAdapterProtocol;
use shared::filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use shared::filesystem::contract_filesystem_protocol::IToolResolutionProtocol;

#[allow(dead_code, unused_imports)]
#[path = "../../shared/tests/common/mock_filesystem.rs"]
mod mock_filesystem;

use mock_filesystem::MockFilesystem;
use std::sync::Arc;

fn make_adapter(
    io: Arc<dyn IFileSystemIOProtocol>,
    tr: Arc<dyn IToolResolutionProtocol>,
) -> MarkdownLintAdapter {
    MarkdownLintAdapter::new(io, tr)
}

fn readme() -> FilePath {
    FilePath::new("/tmp/readme.md".to_string()).unwrap_or_default()
}

// ─── Adapter identity ───

#[test]
fn adapter_name_is_markdownlint() {
    let adapter = make_adapter(
        Arc::new(MockFilesystem::new()),
        Arc::new(MockFilesystem::new()),
    );
    assert_eq!(adapter.name().value(), "markdownlint");
}

// ─── Non-markdown files are rejected early ───

#[test]
fn non_markdown_target_returns_empty() {
    let io = Arc::new(MockFilesystem::new());
    let tr = Arc::new(MockFilesystem::new());
    let adapter = make_adapter(io, tr);
    let path = FilePath::new("/tmp/main.rs".to_string()).unwrap_or_default();
    let results = adapter.scan(&path).unwrap_or_default();
    assert!(results.values.is_empty());
}

// ─── Tool discovery ───

/// With MockFilesystem both `resolve_js_cmd` and `is_executable_in_path`
/// return `None`/`false`, so no binary is ever discovered and scan yields
/// empty results without spawning any subprocesses.
#[test]
fn no_tool_available_yields_empty_results() {
    let adapter = make_adapter(
        Arc::new(MockFilesystem::new()),
        Arc::new(MockFilesystem::new()),
    );
    let results = adapter.scan(&readme()).unwrap_or_default();
    assert!(results.values.is_empty());
}

// ─── Command construction ───

/// Even when no tool is available, the adapter calls `resolve_js_cmd` for
/// each candidate binary in `tool_invocations`.
#[test]
fn resolve_js_cmd_is_called_for_each_candidate() {
    use shared::common::taxonomy_path_vo::FilePath;

    #[derive(Default)]
    struct CapturingResolution {
        seen: std::sync::Mutex<Vec<String>>,
        inner: MockFilesystem,
    }

    impl IToolResolutionProtocol for CapturingResolution {
        fn is_executable_in_path(&self, tool: &ToolName) -> bool {
            self.seen.lock().unwrap().push(tool.value().to_string());
            self.inner.is_executable_in_path(tool)
        }
        fn is_binary_available(&self, tool: &ToolName) -> bool {
            self.inner.is_binary_available(tool)
        }
        fn has_local_bin(&self, working_dir: &std::path::Path, tool: &ToolName) -> bool {
            self.inner.has_local_bin(working_dir, tool)
        }
        fn resolve_js_cmd(
            &self,
            tool: &ToolName,
            args: Vec<String>,
            working_dir: &FilePath,
        ) -> Option<Vec<String>> {
            self.seen
                .lock()
                .unwrap()
                .push(format!("{}:{}", tool.value(), args.join(",")));
            self.inner.resolve_js_cmd(tool, args, working_dir)
        }
        fn resolve_js_working_dir(&self, path: &FilePath) -> FilePath {
            self.inner.resolve_js_working_dir(path)
        }
        fn resolve_cargo_working_dir(&self, path: &FilePath) -> FilePath {
            self.inner.resolve_cargo_working_dir(path)
        }
        fn resolve_cargo_lock_working_dir(&self, path: &FilePath) -> FilePath {
            self.inner.resolve_cargo_lock_working_dir(path)
        }
        fn has_config_file(&self, dir: &std::path::Path) -> bool {
            self.inner.has_config_file(dir)
        }
        fn has_cargo_toml(&self, path: &FilePath) -> Option<FilePath> {
            self.inner.has_cargo_toml(path)
        }
        fn has_cargo_lock(&self, path: &FilePath) -> Option<FilePath> {
            self.inner.has_cargo_lock(path)
        }
        fn is_python_file_recursive(&self, path: &FilePath) -> bool {
            self.inner.is_python_file_recursive(path)
        }
        fn default_working_dir(&self, path: &FilePath) -> FilePath {
            self.inner.default_working_dir(path)
        }
    }

    let cap = Arc::new(CapturingResolution::default());
    let adapter = make_adapter(Arc::new(MockFilesystem::new()), cap.clone());
    let _ = adapter.scan(&readme());
    let seen = cap.seen.lock().unwrap();
    // Both markdownlint-cli and markdownlint-cli2 should have been queried
    assert!(
        seen.iter().any(|s| s.starts_with("markdownlint-cli")),
        "expected markdownlint-cli query, got {:?}",
        seen
    );
    assert!(
        seen.iter().any(|s| s.starts_with("markdownlint-cli2")),
        "expected markdownlint-cli2 query, got {:?}",
        seen
    );
}

// ─── Fix returns status without error ───

#[test]
fn fix_returns_a_status_without_error() {
    let adapter = make_adapter(
        Arc::new(MockFilesystem::new()),
        Arc::new(MockFilesystem::new()),
    );
    let status = adapter.fix(&readme());
    assert!(status.is_ok(), "fix must return a status, not an error");
    if let Ok(cs) = status {
        assert!(!cs.value(), "no tool available means non-compliant status");
    }
}

// ─── OutputNormalizer mapping for the new tool ───

#[test]
fn normalizer_prefixes_markdownlint_codes() {
    use shared::external_lint::INormalizeProtocol;
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
    use shared::external_lint::INormalizeProtocol;
    let normalizer = external_lint_lint_arwaky::OutputNormalizer;
    let tool = ToolName::new("markdownlint");
    let code = shared::common::taxonomy_error_vo::ErrorCode::raw("markdownlint::MD041");
    assert_eq!(
        normalizer.map_severity(&tool, &code, &Severity::INFO),
        Severity::MEDIUM
    );
}
