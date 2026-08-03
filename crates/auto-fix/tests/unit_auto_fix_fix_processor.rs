// Unit tests for LintFixProcessor — fix bypass comments, unused imports, and rename symbols.
// Uses real quality-rules container + real filesystem for integration-style unit tests.
use auto_fix_lint_arwaky::capabilities_fix_processor::LintFixProcessor;
use auto_fix_lint_arwaky::capabilities_file_adapter::FileAdapter;
use shared::auto_fix::{FixOutcome, IFixProtocol};
use shared::common::FilePath;
use std::sync::Arc;
use tempfile::TempDir;

fn make_processor() -> (LintFixProcessor, tempfile::TempDir) {
    let linter = quality_rules::CodeAnalysisContainer::new().code_analysis_linter();
    let filesystem = filesystem::root_filesystem_container::FilesystemContainer::new()
        .orchestrator();
    let file_adapter: Arc<dyn shared::auto_fix::IFileAdapterProtocol> =
        Arc::new(FileAdapter::new(filesystem));
    let tmp = TempDir::new().unwrap();
    (LintFixProcessor::new(linter, file_adapter), tmp)
}

#[test]
fn fix_bypass_comments_strips_allow_attr() {
    let (processor, tmp) = make_processor();
    let file = tmp.path().join("bypass.rs");
    let content = "#[allow(dead_code)]\nfn unused() {}\n";
    std::fs::write(&file, content).unwrap();
    let fp = FilePath::new(file.to_string_lossy().to_string()).unwrap();

    let outcome = processor.fix_bypass_comments(&fp.value(), shared::common::LineNumber::new(1));
    assert!(matches!(outcome, FixOutcome::Applied { .. }));
}

#[test]
fn fix_unused_import_removes_use_line() {
    let (processor, tmp) = make_processor();
    let file = tmp.path().join("unused_import.rs");
    let content = "use std::collections::HashMap;\nfn main() {}\n";
    std::fs::write(&file, content).unwrap();
    let fp = FilePath::new(file.to_string_lossy().to_string()).unwrap();

    let outcome =
        processor.fix_unused_import(&fp.value(), shared::common::LineNumber::new(1));
    assert!(matches!(outcome, FixOutcome::Applied { .. }));
}

#[test]
fn rename_symbol_replaces_word_boundaries() {
    let (processor, tmp) = make_processor();
    let file = tmp.path().join("rename.rs");
    let content = "fn bad_name() { let bad_name = 1; }\n";
    std::fs::write(&file, content).unwrap();
    let fp = FilePath::new(file.to_string_lossy().to_string()).unwrap();

    let outcome = processor.rename_symbol(&fp.value(), "bad_name", "renamed_bad_name");
    assert!(matches!(outcome, FixOutcome::Applied { .. }));
}

#[test]
fn fix_bypass_skips_unsafe_macros() {
    let (processor, tmp) = make_processor();
    let file = tmp.path().join("unsafe_macro.rs");
    let content = "panic!(\"not implemented\");\n";
    std::fs::write(&file, content).unwrap();
    let fp = FilePath::new(file.to_string_lossy().to_string()).unwrap();

    let outcome = processor.fix_bypass_comments(&fp.value(), shared::common::LineNumber::new(1));
    assert!(matches!(outcome, FixOutcome::Skipped(_)));
}

#[test]
fn fix_unused_import_skips_multiline() {
    let (processor, tmp) = make_processor();
    let file = tmp.path().join("multiline_import.rs");
    let content = "use std::collections::{\n    HashMap,\n    BTreeMap,\n};\nfn main() {}\n";
    std::fs::write(&file, content).unwrap();
    let fp = FilePath::new(file.to_string_lossy().to_string()).unwrap();

    let outcome =
        processor.fix_unused_import(&fp.value(), shared::common::LineNumber::new(1));
    assert!(matches!(outcome, FixOutcome::Skipped(_)));
}

#[test]
fn rename_skips_keyword_conflict() {
    let (processor, tmp) = make_processor();
    let file = tmp.path().join("keyword.rs");
    let content = "fn bad_fn() {}\n";
    std::fs::write(&file, content).unwrap();
    let fp = FilePath::new(file.to_string_lossy().to_string()).unwrap();

    let outcome = processor.rename_symbol(&fp.value(), "bad_fn", "fn");
    assert!(matches!(outcome, FixOutcome::Skipped(_)));
}

#[test]
fn fix_bypass_skips_nonexistent_line() {
    let (processor, tmp) = make_processor();
    let file = tmp.path().join("nonexist.rs");
    std::fs::write(&file, "fn main() {}\n").unwrap();
    let fp = FilePath::new(file.to_string_lossy().to_string()).unwrap();

    let outcome = processor.fix_bypass_comments(&fp.value(), shared::common::LineNumber::new(999));
    assert!(matches!(outcome, FixOutcome::Skipped(_)));
}

#[test]
fn fix_unused_import_skips_non_import_line() {
    let (processor, tmp) = make_processor();
    let file = tmp.path().join("no_import.rs");
    std::fs::write(&file, "fn main() {}\n").unwrap();
    let fp = FilePath::new(file.to_string_lossy().to_string()).unwrap();

    let outcome =
        processor.fix_unused_import(&fp.value(), shared::common::LineNumber::new(1));
    assert!(matches!(outcome, FixOutcome::Skipped(_)));
}
