// PURPOSE: Acceptance tests — AES203 unused import detection.
use import_rules_lint_arwaky::capabilities_import_unused_checker::UnusedImportRuleChecker;
use shared::common::taxonomy_path_vo::FilePath;
use shared::filesystem::taxonomy_filesystem_vo::{ImportEntry, ImportType, Language};
use shared::import_rules::IUnusedImportProtocol;
use std::path::PathBuf;

fn checker() -> UnusedImportRuleChecker {
    UnusedImportRuleChecker::new()
}

/// Build a simple ImportEntry for a Rust `use` statement.
fn rust_use(path: &str) -> ImportEntry {
    ImportEntry {
        source_file: PathBuf::new(),
        raw_path: path.to_string(),
        resolved_path: None,
        import_type: ImportType::Use,
        language: Language::Rust,
        is_dynamic: false,
        is_resolved: false,
        symbols: Vec::new(),
        is_reexport: false,
        is_wildcard: false,
    }
}

/// Build a Python import entry.
fn python_use(path: &str) -> ImportEntry {
    ImportEntry {
        source_file: PathBuf::new(),
        raw_path: path.to_string(),
        resolved_path: None,
        import_type: ImportType::Use,
        language: Language::Python,
        is_dynamic: false,
        is_resolved: false,
        symbols: Vec::new(),
        is_reexport: false,
        is_wildcard: false,
    }
}

// ─── AES203: unused Rust imports ──────────────────────────

#[test]
fn aes203_unused_std_import_detected() {
    let content = "use std::collections::HashMap;\n\nfn main() {\n    println!(\"hello\");\n}\n";
    let imports = vec![rust_use("std::collections::HashMap")];
    let results = checker()
        .check_unused_imports("/tmp/test/src/app.rs", content, &imports, &[])
        .unwrap();
    assert!(
        !results.is_empty(),
        "Unused HashMap import should be detected"
    );
    assert_eq!(results[0].code.code(), "AES203");
}

#[test]
fn aes203_used_import_no_violation() {
    let content =
        "use std::collections::HashMap;\n\nfn main() {\n    let _map = HashMap::new();\n}\n";
    let imports = vec![rust_use("std::collections::HashMap")];
    let results = checker()
        .check_unused_imports("/tmp/test/src/main.rs", content, &imports, &[])
        .unwrap();
    assert!(
        results.is_empty(),
        "Used import should not produce violation, got {}",
        results.len()
    );
}

#[test]
fn aes203_multiple_unused_imports() {
    let content = r#"use std::collections::HashMap;
use std::collections::BTreeMap;

fn main() {
    println!("no imports used");
}
"#;
    let imports = vec![
        rust_use("std::collections::HashMap"),
        rust_use("std::collections::BTreeMap"),
    ];
    let results = checker()
        .check_unused_imports("/tmp/test/src/multi.rs", content, &imports, &[])
        .unwrap();
    assert!(
        results.len() >= 2,
        "Should detect at least 2 unused imports, got {}",
        results.len()
    );
}

// ─── AES203: barrel file exemption ────────────────────────

#[test]
fn aes203_lib_rs_skipped() {
    let content = "use something::unused;\n";
    let imports = vec![rust_use("something::unused")];
    let results = checker()
        .check_unused_imports("/tmp/test/src/lib.rs", content, &imports, &[])
        .unwrap();
    assert!(results.is_empty(), "lib.rs should be skipped");
}

#[test]
fn aes203_mod_rs_skipped() {
    let content = "use something::unused;\n";
    let imports = vec![rust_use("something::unused")];
    let results = checker()
        .check_unused_imports("/tmp/test/src/mod.rs", content, &imports, &[])
        .unwrap();
    assert!(results.is_empty(), "mod.rs should be skipped");
}

#[test]
fn aes203_index_ts_skipped() {
    let content = "import { something } from './module';\n";
    let imports = vec![ImportEntry {
        source_file: PathBuf::new(),
        raw_path: "something".to_string(),
        resolved_path: None,
        import_type: ImportType::Use,
        language: Language::TypeScript,
        is_dynamic: false,
        is_resolved: false,
        symbols: Vec::new(),
        is_reexport: false,
        is_wildcard: false,
    }];
    let results = checker()
        .check_unused_imports("/tmp/test/src/index.ts", content, &imports, &[])
        .unwrap();
    assert!(results.is_empty(), "index.ts should be skipped");
}

// ─── AES203: empty content ────────────────────────────────

#[test]
fn aes203_empty_content_no_violation() {
    let results = checker()
        .check_unused_imports("/tmp/test/src/file.rs", "", &[], &[])
        .unwrap();
    assert!(
        results.is_empty(),
        "Empty content should produce no violations"
    );
}

#[test]
fn aes203_no_imports_no_violation() {
    let content = "fn main() {\n    println!(\"hello\");\n}\n";
    let results = checker()
        .check_unused_imports("/tmp/test/src/main.rs", content, &[], &[])
        .unwrap();
    assert!(results.is_empty(), "No imports means no violations");
}

// ─── AES203: protocol trait compliance ────────────────────

#[test]
fn aes203_find_unused_imports_returns_lint_messages() {
    let content = "use std::io::Read;\n\nfn main() {\n    println!(\"hello\");\n}\n";
    let imports = vec![rust_use("std::io::Read")];
    let fp = FilePath::new("/tmp/test/src/file.rs".to_string()).unwrap();
    let results = checker()
        .find_unused_imports(&fp, content, &imports, &[])
        .unwrap();
    // Should return at least one lint message mentioning Read
    assert!(
        !results.is_empty(),
        "find_unused_imports should detect unused Read"
    );
    assert!(
        results[0].value.contains("Read"),
        "Message should mention Read"
    );
}

// ─── AES203: future import exemption ──────────────────────

#[test]
fn aes203_future_import_not_flagged() {
    let content = "from __future__ import annotations\n\nx: int = 1\n";
    let imports = vec![python_use("annotations")];
    let results = checker()
        .check_unused_imports("/tmp/test/src/app.py", content, &imports, &[])
        .unwrap();
    assert!(
        results.is_empty(),
        "Future imports should not be flagged, got {}",
        results.len()
    );
}

// ─── AES203: used identifiers from AST ────────────────────

#[test]
fn aes203_used_identifiers_prevents_false_positive() {
    let content = "use std::collections::HashMap;\n\nfn main() {\n    let _ = HashMap::new();\n}\n";
    let imports = vec![rust_use("std::collections::HashMap")];
    // Providing "HashMap" as a used identifier from tree-sitter
    let results = checker()
        .check_unused_imports(
            "/tmp/test/src/main.rs",
            content,
            &imports,
            &["HashMap".to_string()],
        )
        .unwrap();
    assert!(
        results.is_empty(),
        "Pre-extracted used identifiers should prevent false positives"
    );
}

// ─── AES203: Rust trait method dispatch ───────────────────

#[test]
fn aes203_trait_protocol_import_not_flagged() {
    // ContractProtocol imported for method dispatch scope —
    // the trait is never named explicitly at call sites,
    // but the compiler needs it in scope to resolve methods.
    let content = "use calculator_shared::contract_calculator_protocol::CalculatorProtocol;\n\n"
        .to_string()
        + "struct AdditionAnalyzer;\n\n"
        + "impl AdditionAnalyzer {\n"
        + "    fn evaluate(&self, x: i32) -> i32 { x }\n"
        + "}\n";
    let imports = vec![rust_use(
        "calculator_shared::contract_calculator_protocol::CalculatorProtocol",
    )];
    let results = checker()
        .check_unused_imports("/tmp/test/src/main.rs", &content, &imports, &[])
        .unwrap();
    assert!(
        results.is_empty(),
        "Trait import used for method dispatch should not be flagged, got {} violations",
        results.len()
    );
}

#[test]
fn aes203_trait_suffix_protocol_not_flagged() {
    let content = "use some_crate::MyTrait;\n\nfn main() {\n    println!(\"hi\");\n}\n";
    let imports = vec![rust_use("some_crate::MyTrait")];
    let results = checker()
        .check_unused_imports("/tmp/test/src/main.rs", content, &imports, &[])
        .unwrap();
    assert!(
        results.is_empty(),
        "Import with 'Trait' suffix should not be flagged"
    );
}

#[test]
fn aes203_actual_unused_still_flagged() {
    let content = "use std::collections::HashMap;\n\nfn main() {\n    println!(\"hello\");\n}\n";
    let imports = vec![rust_use("std::collections::HashMap")];
    let results = checker()
        .check_unused_imports("/tmp/test/src/app.rs", content, &imports, &[])
        .unwrap();
    assert!(
        !results.is_empty(),
        "Actual unused import must still be flagged"
    );
}
