// PURPOSE: Acceptance test — FRD: AES Fix: Unused Import Correction
// Requirement: Automatically remove import lines that are not referenced in the file.
// Scope: Rust, Python, JavaScript, and TypeScript.

use auto_fix_lint_arwaky::capabilities_fix_processor::LintFixProcessor;
use shared::auto_fix::contract_fix_protocol::IFixProtocol;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO;
use shared::common::taxonomy_common_vo::{LineNumber, Score};
use shared::common::taxonomy_path_vo::FilePath;
use std::io::Write;
use std::sync::Arc;
use tempfile::NamedTempFile;

struct MockLinter {
    results: Vec<LintResult>,
}
impl ICodeAnalysisAggregate for MockLinter {
    fn run_code_analysis(&self, _: &FilePath) -> LintResultList {
        LintResultList::new(self.results.clone())
    }
    fn run_code_analysis_dir(&self, _: &FilePath) -> LintResultList {
        LintResultList::new(self.results.clone())
    }
    fn run_code_analysis_path(&self, _: &FilePath) -> Vec<LintResult> {
        self.results.clone()
    }
    fn calc_score(&self, _: &[LintResult]) -> Score {
        Score::new(100.0)
    }
    fn check_critical(&self, _: &[LintResult]) -> bool {
        false
    }
    fn format_report(&self, _: &LintResultList, _: &FilePath) -> String {
        String::new()
    }
    fn active_rules(&self) -> Vec<CodeAnalysisRuleVO> {
        vec![]
    }
}

/// FRD-UNUSED-IMPORT-01: Rust `use` statement removed when flagged as AES203.
#[test]
fn frd_rust_unused_use_statement_removed() {
    let mut tmp = NamedTempFile::new().unwrap();
    writeln!(tmp, "use std::collections::HashMap;").unwrap();
    writeln!(tmp, "use std::io;").unwrap();
    writeln!(tmp, "").unwrap();
    writeln!(tmp, "fn main() {{").unwrap();
    writeln!(tmp, "    let _map: HashMap<String, i32> = HashMap::new();").unwrap();
    writeln!(tmp, "}}").unwrap();
    tmp.flush().unwrap();

    let file_path = tmp.path().to_str().unwrap().to_string();
    let sut = LintFixProcessor::new(Arc::new(MockLinter { results: vec![] }));

    let result = sut.fix_unused_import(&file_path, LineNumber::new(2));
    assert!(
        result,
        "AES203 fix should remove the unused `use std::io;` line"
    );

    let content = std::fs::read_to_string(&file_path).unwrap();
    assert!(!content.contains("use std::io;"));
    assert!(content.contains("use std::collections::HashMap;"));
}

/// FRD-UNUSED-IMPORT-02: Python `import` statement removed when flagged as AES203.
#[test]
fn frd_python_unused_import_removed() {
    let mut tmp = NamedTempFile::new().unwrap();
    writeln!(tmp, "import os").unwrap();
    writeln!(tmp, "import json").unwrap();
    writeln!(tmp, "").unwrap();
    writeln!(tmp, "data = json.loads('{{}}')").unwrap();
    tmp.flush().unwrap();

    let file_path = tmp.path().to_str().unwrap().to_string();
    let sut = LintFixProcessor::new(Arc::new(MockLinter { results: vec![] }));

    let result = sut.fix_unused_import(&file_path, LineNumber::new(1));
    assert!(
        result,
        "AES203 fix should remove the unused `import os` line"
    );

    let content = std::fs::read_to_string(&file_path).unwrap();
    assert!(!content.contains("import os"));
    assert!(content.contains("import json"));
}

/// FRD-UNUSED-IMPORT-03: JavaScript/TypeScript `import` removed when flagged.
#[test]
fn frd_typescript_unused_import_removed() {
    let mut tmp = NamedTempFile::new().unwrap();
    writeln!(tmp, "import {{ readFile }} from 'fs';").unwrap();
    writeln!(tmp, "import {{ join }} from 'path';").unwrap();
    writeln!(tmp, "").unwrap();
    writeln!(tmp, "const p = join('/tmp', 'x');").unwrap();
    tmp.flush().unwrap();

    let file_path = tmp.path().to_str().unwrap().to_string();
    let sut = LintFixProcessor::new(Arc::new(MockLinter { results: vec![] }));

    let result = sut.fix_unused_import(&file_path, LineNumber::new(1));
    assert!(result);

    let content = std::fs::read_to_string(&file_path).unwrap();
    assert!(!content.contains("readFile"));
    assert!(content.contains("join"));
}

/// FRD-UNUSED-IMPORT-04: Non-import lines are never removed.
#[test]
fn frd_non_import_line_never_removed() {
    let mut tmp = NamedTempFile::new().unwrap();
    writeln!(tmp, "fn main() {{}}").unwrap();
    writeln!(tmp, "use std::io;").unwrap();
    tmp.flush().unwrap();

    let file_path = tmp.path().to_str().unwrap().to_string();
    let sut = LintFixProcessor::new(Arc::new(MockLinter { results: vec![] }));

    // Line 1 is `fn main() {}` — not an import
    let result = sut.fix_unused_import(&file_path, LineNumber::new(1));
    assert!(!result, "Non-import line must not be removed");

    let content = std::fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("fn main()"));
}

/// FRD-UNUSED-IMPORT-05: Fix does not break remaining code structure.
#[test]
fn frd_fix_preserves_code_functionality() {
    let mut tmp = NamedTempFile::new().unwrap();
    writeln!(tmp, "use std::io;").unwrap();
    writeln!(tmp, "use std::fs;").unwrap();
    writeln!(tmp, "").unwrap();
    writeln!(tmp, "fn main() {{").unwrap();
    writeln!(tmp, "    let _ = std::fs::metadata(\".\");").unwrap();
    writeln!(tmp, "}}").unwrap();
    tmp.flush().unwrap();

    let file_path = tmp.path().to_str().unwrap().to_string();
    let sut = LintFixProcessor::new(Arc::new(MockLinter { results: vec![] }));

    let result = sut.fix_unused_import(&file_path, LineNumber::new(1));
    assert!(result);

    let content = std::fs::read_to_string(&file_path).unwrap();
    // Remaining code must still be structurally valid
    assert!(content.contains("use std::fs;"));
    assert!(content.contains("fn main()"));
    assert!(content.contains("std::fs::metadata"));
}
