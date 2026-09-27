// Unit tests for LintFixProcessor — standalone fix methods (bypass, unused import, rename).
// Uses a mock FileAdapter that reads/writes directly (bypasses filesystem aggregate cache).
use auto_fix_lint_arwaky::capabilities_fix_processor::LintFixProcessor;
use shared::auto_fix::{
    FixOutcome, IBypassFixProtocol, IFileAdapterProtocol, IManualReportProtocol,
    ISymbolRenameProtocol, IUnusedImportFixProtocol,
};
use shared::common::{ContentString, FilePath, Severity};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Build a LintResult with the given code for FR-005 manual-report tests.
fn violation(code: &str, severity: Severity) -> shared::common::LintResult {
    shared::common::LintResult {
        file: FilePath::new("src/main.rs").unwrap(),
        line: shared::common::LineNumber::new(3),
        code: shared::common::ErrorCode::raw(code),
        message: shared::common::LintMessage::new(format!("violation {code}")),
        source: Some(shared::common::AdapterName::raw("architecture")),
        severity,
        ..Default::default()
    }
}

/// Mock adapter backed by a HashMap — no filesystem aggregate cache issues.
struct MockAdapter {
    files: Mutex<HashMap<String, String>>,
}

impl MockAdapter {
    fn with_files(files: HashMap<String, String>) -> Self {
        Self {
            files: Mutex::new(files),
        }
    }
}

impl IFileAdapterProtocol for MockAdapter {
    fn read_file(&self, path: &FilePath) -> Option<ContentString> {
        self.files
            .lock()
            .unwrap()
            .get(path.value())
            .map(|c| ContentString::new(c.clone()))
    }
    fn write_file(&self, path: &FilePath, content: &ContentString) -> bool {
        self.files
            .lock()
            .unwrap()
            .insert(path.value().to_string(), content.value().to_string());
        true
    }
    fn path_exists(&self, path: &FilePath) -> bool {
        self.files.lock().unwrap().contains_key(path.value())
    }
}

fn make_files(entries: &[(&str, &str)]) -> HashMap<String, String> {
    entries
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

fn make_processor(files: HashMap<String, String>) -> LintFixProcessor {
    let linter = quality_rules::CodeAnalysisContainer::new().code_analysis_linter();
    let adapter: Arc<dyn IFileAdapterProtocol> = Arc::new(MockAdapter::with_files(files));
    LintFixProcessor::new(linter, adapter)
}

// ── fix_bypass_comments tests ──────────────────────────────

#[test]
fn fix_bypass_strips_allow_attr() {
    let fp = "/tmp/allow.rs";
    let p = make_processor(make_files(&[(fp, "#[allow(dead_code)]\nfn unused() {}\n")]));
    let outcome = p.fix_bypass_comments(fp, shared::common::LineNumber::new(1));
    assert!(matches!(outcome, FixOutcome::Applied { .. }));
}

#[test]
fn fix_bypass_strips_hack_comment() {
    let fp = "/tmp/hack.rs";
    let p = make_processor(make_files(&[(fp, "// HACK: workaround\nfn main() {}\n")]));
    let outcome = p.fix_bypass_comments(fp, shared::common::LineNumber::new(1));
    assert!(matches!(outcome, FixOutcome::Applied { .. }));
}

#[test]
fn fix_bypass_strips_noqa_inline() {
    let fp = "/tmp/noqa.rs";
    let p = make_processor(make_files(&[(fp, "let x = foo()  # noqa\n")]));
    let outcome = p.fix_bypass_comments(fp, shared::common::LineNumber::new(1));
    assert!(matches!(outcome, FixOutcome::Applied { .. }));
}

#[test]
fn fix_bypass_replaces_unwrap() {
    let fp = "/tmp/unwrap.rs";
    let p = make_processor(make_files(&[(fp, "let x = foo().unwrap();\n")]));
    let outcome = p.fix_bypass_comments(fp, shared::common::LineNumber::new(1));
    assert!(matches!(outcome, FixOutcome::Applied { .. }));
}

#[test]
fn fix_bypass_skips_unsafe_macros() {
    let fp = "/tmp/panic.rs";
    let p = make_processor(make_files(&[(fp, "panic!(\"not implemented\");\n")]));
    let outcome = p.fix_bypass_comments(fp, shared::common::LineNumber::new(1));
    assert!(matches!(outcome, FixOutcome::Skipped(_)));
}

#[test]
fn fix_bypass_skips_expect_with_message() {
    let fp = "/tmp/expect.rs";
    let p = make_processor(make_files(&[(fp, "foo().expect(\"msg\");\n")]));
    let outcome = p.fix_bypass_comments(fp, shared::common::LineNumber::new(1));
    assert!(matches!(outcome, FixOutcome::Skipped(_)));
}

#[test]
fn fix_bypass_skips_nonexistent_line() {
    let fp = "/tmp/short.rs";
    let p = make_processor(make_files(&[(fp, "fn main() {}\n")]));
    let outcome = p.fix_bypass_comments(fp, shared::common::LineNumber::new(999));
    assert!(matches!(outcome, FixOutcome::Skipped(_)));
}

#[test]
fn fix_bypass_skips_non_bypass_line() {
    let fp = "/tmp/clean.rs";
    let p = make_processor(make_files(&[(fp, "fn main() {}\n")]));
    let outcome = p.fix_bypass_comments(fp, shared::common::LineNumber::new(1));
    assert!(matches!(outcome, FixOutcome::Skipped(_)));
}

// ── fix_unused_import tests ────────────────────────────────

#[test]
fn fix_unused_removes_use_line() {
    let fp = "/tmp/unused.rs";
    let p = make_processor(make_files(&[(
        fp,
        "use std::collections::HashMap;\nfn main() {}\n",
    )]));
    let outcome = p.fix_unused_import(fp, shared::common::LineNumber::new(1));
    assert!(matches!(outcome, FixOutcome::Applied { .. }));
}

#[test]
fn fix_unused_removes_js_require() {
    let fp = "/tmp/require.js";
    let p = make_processor(make_files(&[(
        fp,
        "const fs = require('fs');\nconsole.log(1);\n",
    )]));
    let outcome = p.fix_unused_import(fp, shared::common::LineNumber::new(1));
    assert!(matches!(outcome, FixOutcome::Applied { .. }));
}

#[test]
fn fix_unused_removes_python_import() {
    let fp = "/tmp/import.py";
    let p = make_processor(make_files(&[(fp, "import os\nprint('hello')\n")]));
    let outcome = p.fix_unused_import(fp, shared::common::LineNumber::new(1));
    assert!(matches!(outcome, FixOutcome::Applied { .. }));
}

#[test]
fn fix_unused_skips_multiline() {
    let fp = "/tmp/multi.rs";
    let content = "use std::collections::{\n    HashMap,\n    BTreeMap,\n};\nfn main() {}\n";
    let p = make_processor(make_files(&[(fp, content)]));
    let outcome = p.fix_unused_import(fp, shared::common::LineNumber::new(1));
    assert!(matches!(outcome, FixOutcome::Skipped(_)));
}

#[test]
fn fix_unused_skips_non_import_line() {
    let fp = "/tmp/code.rs";
    let p = make_processor(make_files(&[(fp, "fn main() {}\n")]));
    let outcome = p.fix_unused_import(fp, shared::common::LineNumber::new(1));
    assert!(matches!(outcome, FixOutcome::Skipped(_)));
}

// ── rename_symbol tests ────────────────────────────────────

#[test]
fn rename_replaces_word_boundaries() {
    let fp = "/tmp/rename.rs";
    let p = make_processor(make_files(&[(fp, "fn bad_name() { let bad_name = 1; }\n")]));
    let outcome = p.rename_symbol(fp, "bad_name", "renamed_bad_name");
    assert!(matches!(outcome, FixOutcome::Applied { .. }));
}

#[test]
fn rename_preserves_surrounding_text() {
    let fp = "/tmp/context.rs";
    let p = make_processor(make_files(&[(
        fp,
        "fn bad_name(x: i32) -> i32 { bad_name + 1 }\n",
    )]));
    let outcome = p.rename_symbol(fp, "bad_name", "good_name");
    if let FixOutcome::Applied { changes } = &outcome {
        assert!(*changes >= 2, "Should replace at least 2 occurrences");
    } else {
        panic!("Expected Applied");
    }
}

#[test]
fn rename_skips_keyword_conflict() {
    let fp = "/tmp/kw.rs";
    let p = make_processor(make_files(&[(fp, "fn bad_fn() {}\n")]));
    let outcome = p.rename_symbol(fp, "bad_fn", "fn");
    assert!(matches!(outcome, FixOutcome::Skipped(_)));
}

#[test]
fn rename_skips_nonexistent_symbol() {
    let fp = "/tmp/nosym.rs";
    let p = make_processor(make_files(&[(fp, "fn main() {}\n")]));
    let outcome = p.rename_symbol(fp, "nonexistent", "renamed");
    assert!(matches!(outcome, FixOutcome::Skipped(_)));
}

// ── report_non_fixable tests (FR-005) ────────────────────

#[test]
fn manual_report_lists_only_non_fixable_codes() {
    let p = make_processor(make_files(&[]));
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
    let p = make_processor(make_files(&[]));
    assert!(p.report_non_fixable(&[]).is_empty());
}
