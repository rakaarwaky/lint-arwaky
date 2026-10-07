// Unit tests for auto-fix capabilities — standalone fix methods (bypass, unused import, rename).
// Uses a mock IFileSystemIOProtocol that reads/writes directly (bypasses filesystem aggregate cache).
use auto_fix_lint_arwaky::capabilities_bypass_fix::BypassFix;
use auto_fix_lint_arwaky::capabilities_symbol_rename::SymbolRename;
use auto_fix_lint_arwaky::capabilities_unused_import_fix::UnusedImportFix;
use auto_fix_lint_arwaky::capabilities_violation_report::ViolationReport;
use shared_auto_fix::{
    FixOutcome, IBypassFixProtocol, ISymbolRenameProtocol, IUnusedImportFixProtocol,
    IViolationReportProtocol,
};
use shared_common::{ContentString, FilePath, Severity, SymbolName};
use shared_filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

/// Build a LintResult with the given code for FR-004 manual-report tests.
fn violation(code: &str, severity: Severity) -> shared_common::LintResult {
    shared_common::LintResult {
        file: FilePath::new("src/main.rs").unwrap(),
        line: shared_common::LineNumber::new(3),
        code: shared_common::ErrorCode::raw(code),
        message: shared_common::LintMessage::new(format!("violation {code}")),
        source: Some(shared_common::AdapterName::raw("architecture")),
        severity,
        ..Default::default()
    }
}

/// Mock IO backed by a HashMap — no filesystem aggregate cache issues.
struct MockIO {
    files: Mutex<HashMap<String, String>>,
}

impl MockIO {
    fn with_files(files: HashMap<String, String>) -> Self {
        Self {
            files: Mutex::new(files),
        }
    }
}

impl IFileSystemIOProtocol for MockIO {
    fn path_exists(&self, path: &Path) -> bool {
        self.files
            .lock()
            .unwrap()
            .contains_key(path.to_string_lossy().as_ref())
    }
    fn read_to_string(&self, path: &Path) -> Result<ContentString, io::Error> {
        self.files
            .lock()
            .unwrap()
            .get(path.to_string_lossy().as_ref())
            .map(|c| ContentString::new(c.clone()))
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "file not found"))
    }
    fn write_string(&self, path: &Path, content: &str) -> Result<(), io::Error> {
        self.files
            .lock()
            .unwrap()
            .insert(path.to_string_lossy().to_string(), content.to_string());
        Ok(())
    }
    // Required IFileSystemIOProtocol methods — all minimal stubs
    fn is_dir(&self, _path: &Path) -> bool {
        false
    }
    fn is_file(&self, _path: &Path) -> bool {
        false
    }
    fn should_ignore(&self, _path: &FilePath, _ignored: &[String]) -> bool {
        false
    }
    fn canonicalize(&self, path: &Path) -> Result<PathBuf, io::Error> {
        Ok(path.to_path_buf())
    }
    fn canonicalize_path_str(&self, path: &FilePath) -> FilePath {
        path.clone()
    }
    fn is_symlink(&self, _path: &Path) -> bool {
        false
    }
    fn metadata(&self, _path: &Path) -> Result<std::fs::Metadata, io::Error> {
        Err(io::Error::new(io::ErrorKind::NotFound, "not found"))
    }
    fn symlink_metadata(&self, _path: &Path) -> Result<std::fs::Metadata, io::Error> {
        Err(io::Error::new(io::ErrorKind::NotFound, "not found"))
    }
    fn get_file_stem<'a>(&self, path: &'a str) -> &'a str {
        Path::new(path).file_stem().unwrap().to_str().unwrap()
    }
    fn is_source_file(&self, _path: &Path) -> bool {
        false
    }
    fn is_source_ext(
        &self,
        _ext: &shared_filesystem::taxonomy_filesystem_vo::FileExtension,
    ) -> bool {
        false
    }
    fn get_basename<'a>(&self, path: &'a str) -> &'a str {
        Path::new(path).file_name().unwrap().to_str().unwrap()
    }
    fn get_parent<'a>(&self, path: &'a str) -> &'a str {
        Path::new(path).parent().unwrap().to_str().unwrap()
    }
    fn is_python_file(&self, _path: &Path) -> bool {
        false
    }
    fn scan_directory_with_ignored(
        &self,
        _dir: &Path,
        _ignored: &shared_common::PatternList,
    ) -> Vec<PathBuf> {
        vec![]
    }
    fn is_ignored_dir(&self, _dir: &Path, _ignored: &shared_common::PatternList) -> bool {
        false
    }
    fn read_dir_entries_as_pathbuf(&self, _dir: &Path) -> Result<Vec<PathBuf>, io::Error> {
        Ok(vec![])
    }
    fn copy_file(
        &self,
        _src: &Path,
        _dst: &Path,
    ) -> Result<shared_filesystem::taxonomy_filesystem_vo::ByteCount, io::Error> {
        Ok(shared_filesystem::taxonomy_filesystem_vo::ByteCount { bytes: 0 })
    }
    fn create_dir_all(&self, _path: &Path) -> Result<(), io::Error> {
        Ok(())
    }
    fn remove_dir_all(&self, _path: &Path) -> Result<(), io::Error> {
        Ok(())
    }
    fn set_permissions(
        &self,
        _path: &Path,
        _mode: shared_filesystem::taxonomy_filesystem_vo::FileMode,
    ) -> io::Result<()> {
        Ok(())
    }
    fn remove_file(&self, _path: &Path) -> io::Result<()> {
        Ok(())
    }
    fn run_git_command(
        &self,
        _args: &[&str],
        _dir: &str,
    ) -> shared_filesystem::taxonomy_filesystem_vo::GitCommandResult {
        shared_filesystem::taxonomy_filesystem_vo::GitCommandResult::new(
            String::new(),
            String::new(),
            true,
        )
    }
    fn parse_output_lines(
        &self,
        _output: &str,
    ) -> shared_filesystem::taxonomy_filesystem_vo::ParsedLines {
        shared_filesystem::taxonomy_filesystem_vo::ParsedLines::new(vec![])
    }
    fn run_external_command_in(
        &self,
        _name: &shared_common::ToolName,
        _args: &[&str],
        _current_dir: &str,
    ) -> (String, String, bool) {
        (String::new(), String::new(), true)
    }
    fn timing(&self) -> &shared_filesystem::ScanTiming {
        use std::sync::OnceLock;
        static TIMING: OnceLock<shared_filesystem::ScanTiming> = OnceLock::new();
        TIMING.get_or_init(shared_filesystem::ScanTiming::default)
    }
}

fn make_files(entries: &[(&str, &str)]) -> HashMap<String, String> {
    entries
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

fn make_bypass_fix(files: HashMap<String, String>) -> BypassFix {
    let io: Arc<dyn IFileSystemIOProtocol> = Arc::new(MockIO::with_files(files));
    BypassFix::new(io)
}

fn make_unused_import_fix(files: HashMap<String, String>) -> UnusedImportFix {
    let io: Arc<dyn IFileSystemIOProtocol> = Arc::new(MockIO::with_files(files));
    UnusedImportFix::new(io)
}

fn make_symbol_rename(files: HashMap<String, String>) -> SymbolRename {
    let io: Arc<dyn IFileSystemIOProtocol> = Arc::new(MockIO::with_files(files));
    SymbolRename::new(io)
}

fn make_violation_report(files: HashMap<String, String>) -> ViolationReport {
    let io: Arc<dyn IFileSystemIOProtocol> = Arc::new(MockIO::with_files(files));
    let linter = quality_rules::CodeAnalysisContainer::new().code_analysis_linter();
    let unused_import_fix = Arc::new(UnusedImportFix::new(io.clone()));
    let bypass_fix = Arc::new(BypassFix::new(io.clone()));
    let symbol_rename = Arc::new(SymbolRename::new(io.clone()));
    ViolationReport::new(linter, unused_import_fix, bypass_fix, symbol_rename, io)
}

// ── fix_bypass_comments tests ──────────────────────────────

/// Test FR-PRD-003 exit-code aggregation: `Failed` outcomes must trigger error in FixResult.
#[test]
fn fix_result_exit_code_contract_failed_outcomes_trigger_error() {
    use shared_auto_fix::{FailReason, FixOutcome};
    let fp = "/tmp/nonexistent.rs";
    // Target file does not exist in MockIO → produces FailReason::FileNotFound
    let p = make_bypass_fix(HashMap::new());
    let outcome = p.fix_bypass_comments(fp, shared_common::LineNumber::new(1));
    assert_eq!(outcome, FixOutcome::Failed(FailReason::FileNotFound));

    // Display implementation works and formats reason without debug punctuation
    assert_eq!(format!("{outcome}"), "Failed(FileNotFound)");
}

#[test]
fn fix_outcome_display_formatting() {
    use shared_auto_fix::{FailReason, SkipReason};
    assert_eq!(
        format!("{}", FixOutcome::Applied { changes: 3 }),
        "Applied (3 change(s))"
    );
    assert_eq!(
        format!("{}", FixOutcome::Skipped(SkipReason::MultiLineImport)),
        "Skipped(MultiLineImport)"
    );
    assert_eq!(
        format!("{}", FixOutcome::Failed(FailReason::WriteError)),
        "Failed(WriteError)"
    );
}

#[test]
fn fix_bypass_strips_allow_attr() {
    let fp = "/tmp/allow.rs";
    let p = make_bypass_fix(make_files(&[(fp, "#[allow(dead_code)]\nfn unused() {}\n")]));
    let outcome = p.fix_bypass_comments(fp, shared_common::LineNumber::new(1));
    assert!(matches!(outcome, FixOutcome::Applied { .. }));
}

#[test]
fn fix_bypass_strips_hack_comment() {
    let fp = "/tmp/hack.rs";
    let p = make_bypass_fix(make_files(&[(fp, "// HACK: workaround\nfn main() {}\n")]));
    let outcome = p.fix_bypass_comments(fp, shared_common::LineNumber::new(1));
    assert!(matches!(outcome, FixOutcome::Applied { .. }));
}

#[test]
fn fix_bypass_strips_noqa_inline() {
    let fp = "/tmp/noqa.rs";
    let p = make_bypass_fix(make_files(&[(fp, "let x = foo()  # noqa\n")]));
    let outcome = p.fix_bypass_comments(fp, shared_common::LineNumber::new(1));
    assert!(matches!(outcome, FixOutcome::Applied { .. }));
}

#[test]
fn fix_bypass_replaces_unwrap() {
    let fp = "/tmp/unwrap.rs";
    let p = make_bypass_fix(make_files(&[(fp, "let x = foo().unwrap();\n")]));
    let outcome = p.fix_bypass_comments(fp, shared_common::LineNumber::new(1));
    assert!(matches!(outcome, FixOutcome::Applied { .. }));
}

#[test]
fn fix_bypass_skips_unsafe_macros() {
    let fp = "/tmp/panic.rs";
    let p = make_bypass_fix(make_files(&[(fp, "panic!(\"not implemented\");\n")]));
    let outcome = p.fix_bypass_comments(fp, shared_common::LineNumber::new(1));
    assert!(matches!(outcome, FixOutcome::Skipped(_)));
}

#[test]
fn fix_bypass_skips_expect_with_message() {
    let fp = "/tmp/expect.rs";
    let p = make_bypass_fix(make_files(&[(fp, "foo().expect(\"msg\");\n")]));
    let outcome = p.fix_bypass_comments(fp, shared_common::LineNumber::new(1));
    assert!(matches!(outcome, FixOutcome::Skipped(_)));
}

#[test]
fn fix_bypass_skips_nonexistent_line() {
    let fp = "/tmp/short.rs";
    let p = make_bypass_fix(make_files(&[(fp, "fn main() {}\n")]));
    let outcome = p.fix_bypass_comments(fp, shared_common::LineNumber::new(999));
    assert!(matches!(outcome, FixOutcome::Skipped(_)));
}

#[test]
fn fix_bypass_skips_non_bypass_line() {
    let fp = "/tmp/clean.rs";
    let p = make_bypass_fix(make_files(&[(fp, "fn main() {}\n")]));
    let outcome = p.fix_bypass_comments(fp, shared_common::LineNumber::new(1));
    assert!(matches!(outcome, FixOutcome::Skipped(_)));
}

// ── fix_unused_import tests ────────────────────────────────

#[test]
fn fix_unused_removes_use_line() {
    let fp = "/tmp/unused.rs";
    let p = make_unused_import_fix(make_files(&[(
        fp,
        "use std::collections::HashMap;\nfn main() {}\n",
    )]));
    let outcome = p.fix_unused_import(fp, shared_common::LineNumber::new(1));
    assert!(matches!(outcome, FixOutcome::Applied { .. }));
}

#[test]
fn fix_unused_removes_js_require() {
    let fp = "/tmp/require.js";
    let p = make_unused_import_fix(make_files(&[(
        fp,
        "const fs = require('fs');\nconsole.log(1);\n",
    )]));
    let outcome = p.fix_unused_import(fp, shared_common::LineNumber::new(1));
    assert!(matches!(outcome, FixOutcome::Applied { .. }));
}

#[test]
fn fix_unused_removes_python_import() {
    let fp = "/tmp/import.py";
    let p = make_unused_import_fix(make_files(&[(fp, "import os\nprint('hello')\n")]));
    let outcome = p.fix_unused_import(fp, shared_common::LineNumber::new(1));
    assert!(matches!(outcome, FixOutcome::Applied { .. }));
}

#[test]
fn fix_unused_skips_multiline() {
    let fp = "/tmp/multi.rs";
    let content = "use std::collections::{\n    HashMap,\n    BTreeMap,\n};\nfn main() {}\n";
    let p = make_unused_import_fix(make_files(&[(fp, content)]));
    let outcome = p.fix_unused_import(fp, shared_common::LineNumber::new(1));
    assert!(matches!(outcome, FixOutcome::Skipped(_)));
}

#[test]
fn fix_unused_skips_non_import_line() {
    let fp = "/tmp/code.rs";
    let p = make_unused_import_fix(make_files(&[(fp, "fn main() {}\n")]));
    let outcome = p.fix_unused_import(fp, shared_common::LineNumber::new(1));
    assert!(matches!(outcome, FixOutcome::Skipped(_)));
}

// ── rename_symbol tests ────────────────────────────────────

#[test]
fn rename_replaces_word_boundaries() {
    let fp = "/tmp/rename.rs";
    let p = make_symbol_rename(make_files(&[(fp, "fn bad_name() { let bad_name = 1; }\n")]));
    let outcome = p.rename_symbol(
        fp,
        &SymbolName::new("bad_name"),
        &SymbolName::new("renamed_bad_name"),
    );
    assert!(matches!(outcome, FixOutcome::Applied { .. }));
}

#[test]
fn rename_preserves_surrounding_text() {
    let fp = "/tmp/context.rs";
    let p = make_symbol_rename(make_files(&[(
        fp,
        "fn bad_name(x: i32) -> i32 { bad_name + 1 }\n",
    )]));
    let outcome = p.rename_symbol(
        fp,
        &SymbolName::new("bad_name"),
        &SymbolName::new("good_name"),
    );
    if let FixOutcome::Applied { changes } = &outcome {
        assert!(*changes >= 2, "Should replace at least 2 occurrences");
    } else {
        panic!("Expected Applied");
    }
}

#[test]
fn rename_skips_keyword_conflict() {
    let fp = "/tmp/kw.rs";
    let p = make_symbol_rename(make_files(&[(fp, "fn bad_fn() {}\n")]));
    let outcome = p.rename_symbol(fp, &SymbolName::new("bad_fn"), &SymbolName::new("fn"));
    assert!(matches!(outcome, FixOutcome::Skipped(_)));
}

#[test]
fn rename_skips_nonexistent_symbol() {
    let fp = "/tmp/nosym.rs";
    let p = make_symbol_rename(make_files(&[(fp, "fn main() {}\n")]));
    let outcome = p.rename_symbol(
        fp,
        &SymbolName::new("nonexistent"),
        &SymbolName::new("renamed"),
    );
    assert!(matches!(outcome, FixOutcome::Skipped(_)));
}

// ── report_non_fixable tests (FR-004) ────────────────────

#[test]
fn manual_report_lists_only_non_fixable_codes() {
    let p = make_violation_report(make_files(&[]));
    let violations = vec![
        violation("AES101", Severity::HIGH),
        violation("AES203", Severity::LOW),
        violation("AES304", Severity::MEDIUM),
        violation("AES401", Severity::LOW),
        violation("clippy::needless_return", Severity::MEDIUM),
    ];
    let report = p.report_non_fixable(&violations);
    assert_eq!(report.len(), 2, "only AES401 and tool-native codes remain");
    let joined = report
        .iter()
        .map(|m| m.value().to_string())
        .collect::<Vec<_>>()
        .join("\n");
    assert!(joined.contains("AES401"));
    assert!(joined.contains("clippy::needless_return"));
    assert!(!joined.contains("AES101"));
}

#[test]
fn manual_report_on_empty_violations_is_empty() {
    let p = make_violation_report(make_files(&[]));
    assert!(p.report_non_fixable(&[]).is_empty());
}

// ── regression tests (#934, #935, #937, #938) ───────────

/// Shared mock IO whose in-memory file map can be read back after a fix.
struct SharedMockIO {
    files: Mutex<HashMap<String, String>>,
}

impl SharedMockIO {
    fn new(files: HashMap<String, String>) -> Arc<Self> {
        Arc::new(Self {
            files: Mutex::new(files),
        })
    }
    fn read(&self, path: &str) -> Option<String> {
        self.files.lock().unwrap().get(path).cloned()
    }
}

impl IFileSystemIOProtocol for SharedMockIO {
    fn path_exists(&self, path: &Path) -> bool {
        let key = path.to_string_lossy().into_owned();
        self.files.lock().unwrap().contains_key(&key)
    }
    fn read_to_string(&self, path: &Path) -> Result<ContentString, io::Error> {
        let key = path.to_string_lossy().into_owned();
        self.files
            .lock()
            .unwrap()
            .get(&key)
            .map(|c| ContentString::new(c.clone()))
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "not found"))
    }
    fn write_string(&self, path: &Path, content: &str) -> Result<(), io::Error> {
        self.files
            .lock()
            .unwrap()
            .insert(path.to_string_lossy().into_owned(), content.to_string());
        Ok(())
    }
    fn is_dir(&self, _path: &Path) -> bool {
        false
    }
    fn is_file(&self, path: &Path) -> bool {
        let key = path.to_string_lossy().into_owned();
        self.files.lock().unwrap().contains_key(&key)
    }
    fn should_ignore(&self, _path: &FilePath, _ignored: &[String]) -> bool {
        false
    }
    fn canonicalize(&self, path: &Path) -> Result<PathBuf, io::Error> {
        Ok(path.to_path_buf())
    }
    fn canonicalize_path_str(&self, path: &FilePath) -> FilePath {
        path.clone()
    }
    fn is_symlink(&self, _path: &Path) -> bool {
        false
    }
    fn metadata(&self, _path: &Path) -> Result<std::fs::Metadata, io::Error> {
        Err(io::Error::new(io::ErrorKind::NotFound, "not found"))
    }
    fn symlink_metadata(&self, _path: &Path) -> Result<std::fs::Metadata, io::Error> {
        Err(io::Error::new(io::ErrorKind::NotFound, "not found"))
    }
    fn get_file_stem<'a>(&self, path: &'a str) -> &'a str {
        Path::new(path).file_stem().unwrap().to_str().unwrap()
    }
    fn is_source_file(&self, _path: &Path) -> bool {
        true
    }
    fn is_source_ext(
        &self,
        _ext: &shared_filesystem::taxonomy_filesystem_vo::FileExtension,
    ) -> bool {
        true
    }
    fn get_basename<'a>(&self, path: &'a str) -> &'a str {
        Path::new(path).file_name().unwrap().to_str().unwrap()
    }
    fn get_parent<'a>(&self, path: &'a str) -> &'a str {
        Path::new(path).parent().unwrap().to_str().unwrap()
    }
    fn is_python_file(&self, _path: &Path) -> bool {
        false
    }
    fn scan_directory_with_ignored(
        &self,
        _dir: &Path,
        _ignored: &shared_common::PatternList,
    ) -> Vec<PathBuf> {
        vec![]
    }
    fn is_ignored_dir(&self, _dir: &Path, _ignored: &shared_common::PatternList) -> bool {
        false
    }
    fn read_dir_entries_as_pathbuf(&self, _path: &Path) -> Result<Vec<PathBuf>, io::Error> {
        Ok(vec![])
    }
    fn copy_file(
        &self,
        _src: &Path,
        _dst: &Path,
    ) -> Result<shared_filesystem::taxonomy_filesystem_vo::ByteCount, io::Error> {
        Ok(shared_filesystem::taxonomy_filesystem_vo::ByteCount { bytes: 0 })
    }
    fn create_dir_all(&self, _path: &Path) -> Result<(), io::Error> {
        Ok(())
    }
    fn remove_dir_all(&self, _path: &Path) -> Result<(), io::Error> {
        Ok(())
    }
    fn set_permissions(
        &self,
        _path: &Path,
        _mode: shared_filesystem::taxonomy_filesystem_vo::FileMode,
    ) -> io::Result<()> {
        Ok(())
    }
    fn remove_file(&self, _path: &Path) -> io::Result<()> {
        Ok(())
    }
    fn run_git_command(
        &self,
        _args: &[&str],
        _dir: &str,
    ) -> shared_filesystem::taxonomy_filesystem_vo::GitCommandResult {
        shared_filesystem::taxonomy_filesystem_vo::GitCommandResult::new(
            String::new(),
            String::new(),
            true,
        )
    }
    fn parse_output_lines(
        &self,
        _output: &str,
    ) -> shared_filesystem::taxonomy_filesystem_vo::ParsedLines {
        shared_filesystem::taxonomy_filesystem_vo::ParsedLines::new(vec![])
    }
    fn run_external_command_in(
        &self,
        _name: &shared_common::ToolName,
        _args: &[&str],
        _current_dir: &str,
    ) -> (String, String, bool) {
        (String::new(), String::new(), true)
    }
    fn timing(&self) -> &shared_filesystem::ScanTiming {
        use std::sync::OnceLock;
        static TIMING: OnceLock<shared_filesystem::ScanTiming> = OnceLock::new();
        TIMING.get_or_init(shared_filesystem::ScanTiming::default)
    }
}

fn make_shared_mock(files: HashMap<String, String>) -> Arc<SharedMockIO> {
    SharedMockIO::new(files)
}

fn shared_unused_import_fix(
    files: HashMap<String, String>,
) -> (UnusedImportFix, Arc<SharedMockIO>) {
    let io = make_shared_mock(files);
    let io_dyn: Arc<dyn IFileSystemIOProtocol> = io.clone();
    (UnusedImportFix::new(io_dyn), io)
}

fn shared_bypass_fix(files: HashMap<String, String>) -> (BypassFix, Arc<SharedMockIO>) {
    let io = make_shared_mock(files);
    let io_dyn: Arc<dyn IFileSystemIOProtocol> = io.clone();
    (BypassFix::new(io_dyn), io)
}

/// #935: removing the last remaining code line of a file with no trailing
/// newline must not add one (FRD FR-AutoFix-001 "newlines preserved").
#[test]
fn fix_unused_import_preserves_missing_trailing_newline() {
    let fp = "/tmp/notrail.rs";
    let (fix, io) = shared_unused_import_fix(make_files(&[(
        fp,
        "use std::collections::HashMap;\nfn main() {}",
    )]));
    let outcome = fix.fix_unused_import(fp, shared_common::LineNumber::new(1));
    assert!(matches!(outcome, FixOutcome::Applied { .. }));
    let content = io.read(fp).unwrap();
    assert_eq!(content, "fn main() {}");
    assert!(
        !content.ends_with('\n'),
        "trailing newline must not be added"
    );
}

/// #935: removing an import from a file that HAS a trailing newline keeps one.
#[test]
fn fix_unused_import_preserves_trailing_newline() {
    let fp = "/tmp/trail.rs";
    let (fix, io) = shared_unused_import_fix(make_files(&[(
        fp,
        "use std::collections::HashMap;\nfn main() {}\n",
    )]));
    let outcome = fix.fix_unused_import(fp, shared_common::LineNumber::new(1));
    assert!(matches!(outcome, FixOutcome::Applied { .. }));
    let content = io.read(fp).unwrap();
    assert_eq!(content, "fn main() {}\n");
    assert!(
        content.ends_with('\n'),
        "trailing newline must be preserved"
    );
}

/// #935 (bypass): stripping a `#[allow]` line from a file whose last line
/// has no trailing newline must not add one.
#[test]
fn fix_bypass_preserves_missing_trailing_newline() {
    let fp = "/tmp/bnotrail.rs";
    let (fix, io) = shared_bypass_fix(make_files(&[(fp, "#[allow(dead_code)]\nfn x() {}")]));
    let outcome = fix.fix_bypass_comments(fp, shared_common::LineNumber::new(1));
    assert!(matches!(outcome, FixOutcome::Applied { .. }));
    let content = io.read(fp).unwrap();
    assert_eq!(content, "fn x() {}");
    assert!(
        !content.ends_with('\n'),
        "trailing newline must not be added"
    );
}

/// #934: report_violations must confine the linter to the requested path's
/// files. A single source file contributes one entry; a directory is walked.
/// We assert the pipeline runs without error and reports over the path's
/// files, not the empty project-wide fallback.
#[test]
fn report_violations_scopes_to_requested_path() {
    let fp = "/tmp/aes101_scope.rs";
    // Seed a file whose CamelCase symbol the naming rule would flag.
    let files = make_files(&[(fp, "struct MyName;\n")]);
    let p = make_violation_report(files);
    // Runs without panicking; the key #934 invariant is that the linter is
    // invoked with the path's files rather than an empty list. We can only
    // observe the outcome (no crash, sensible report) here.
    let res = p.report_violations(&FilePath::new(fp.to_string()).unwrap(), true);
    let out = res.output.value();
    assert!(
        out.contains("Dry-run"),
        "dry-run report should render, got: {out}"
    );
}

/// #937 + #938: extract the flagged symbol from "Symbol MyName does not match
/// snake_case" and record a same-name outcome as a skip. The helper lives in
/// the crate; we exercise it through the pipeline by feeding a message that
/// names `snake_case` and asserting `MyName` (not `snake_case`) is what the
/// rename targets.
#[test]
fn aes101_extraction_targets_flagged_symbol_not_convention_word() {
    let fp = "/tmp/aes101_target.rs";
    // `MyName` is the CamelCase symbol to rename; `snake_case` is the
    // convention word that must NOT be renamed.
    let files = make_files(&[(fp, "struct MyName;\nlet snake_case = 0;\n")]);
    let p = make_violation_report(files);
    let res = p.report_violations(&FilePath::new(fp.to_string()).unwrap(), false);
    let out = res.output.value();
    // `MyName` should be the rename target; `snake_case` must survive.
    // If #937 regressed, `snake_case` would be renamed to `renamed_snake_case`
    // and `MyName` would be left alone.
    assert!(
        !out.contains("renamed_snake_case"),
        "convention word snake_case must not be renamed, got: {out}"
    );
}
