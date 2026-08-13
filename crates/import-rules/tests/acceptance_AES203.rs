// PURPOSE: Acceptance tests — AES203 unused import detection.
use import_rules_lint_arwaky::capabilities_import_unused_checker::UnusedImportRuleChecker;
use shared::common::taxonomy_path_vo::FilePath;
use shared::filesystem::taxonomy_filesystem_vo::{ImportEntry, ImportType, Language};
use shared::import_rules::IUnusedImportProtocol;
use std::collections::HashMap;
use std::path::PathBuf;

fn checker() -> UnusedImportRuleChecker {
    UnusedImportRuleChecker::new()
}

/// Empty trait map for tests that don't test cross-file trait detection.
fn no_traits() -> HashMap<String, Vec<String>> {
    HashMap::new()
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
        .check_unused_imports("/tmp/test/src/app.rs", content, &imports, &[], &no_traits())
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
        .check_unused_imports(
            "/tmp/test/src/app.rs",
            content,
            &imports,
            &["HashMap".to_string()],
            &no_traits(),
        )
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
        .check_unused_imports(
            "/tmp/test/src/multi.rs",
            content,
            &imports,
            &[],
            &no_traits(),
        )
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
        .check_unused_imports("/tmp/test/src/lib.rs", content, &imports, &[], &no_traits())
        .unwrap();
    assert!(results.is_empty(), "lib.rs should be skipped");
}

#[test]
fn aes203_mod_rs_skipped() {
    let content = "use something::unused;\n";
    let imports = vec![rust_use("something::unused")];
    let results = checker()
        .check_unused_imports("/tmp/test/src/mod.rs", content, &imports, &[], &no_traits())
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
        .check_unused_imports(
            "/tmp/test/src/index.ts",
            content,
            &imports,
            &[],
            &no_traits(),
        )
        .unwrap();
    assert!(results.is_empty(), "index.ts should be skipped");
}

// ─── AES203: empty content ────────────────────────────────

#[test]
fn aes203_empty_content_no_violation() {
    let results = checker()
        .check_unused_imports("/tmp/test/src/file.rs", "", &[], &[], &no_traits())
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
        .check_unused_imports("/tmp/test/src/main.rs", content, &[], &[], &no_traits())
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
        .check_unused_imports("/tmp/test/src/app.py", content, &imports, &[], &no_traits())
        .unwrap();
    assert!(
        results.is_empty(),
        "Future imports should not be flagged, got {}",
        results.len()
    );
}

// ─── AES203: alias binding regression (import ... as <alias>) ──
// Regression: aliased imports must resolve usage against the alias
// binding name, not the original imported name.

#[test]
fn aes203_python_aliased_import_used_via_alias_not_flagged() {
    // `from x import y as z` + usage of `z` — zero AES203 findings.
    let content = "from os import getcwd as _gc\n\n\ndef show() -> str:\n    return _gc()\n";
    let imports = vec![ImportEntry {
        source_file: PathBuf::new(),
        raw_path: "os".to_string(),
        resolved_path: None,
        import_type: ImportType::ImportFrom,
        language: Language::Python,
        is_dynamic: false,
        is_resolved: false,
        symbols: vec!["_gc".to_string()],
        is_reexport: false,
        is_wildcard: false,
    }];
    let results = checker()
        .check_unused_imports(
            "/tmp/test/src/app.py",
            content,
            &imports,
            &["_gc".to_string()],
            &no_traits(),
        )
        .unwrap();
    assert!(
        results.is_empty(),
        "Aliased import used via alias must not be flagged, got {}",
        results.len()
    );
}

#[test]
fn aes203_python_aliased_import_truly_unused_still_flagged() {
    // Aliased import that is never used must still produce AES203.
    let content = "from os import getcwd as _gc\n\n\ndef show() -> str:\n    return \"static\"\n";
    let imports = vec![ImportEntry {
        source_file: PathBuf::new(),
        raw_path: "os".to_string(),
        resolved_path: None,
        import_type: ImportType::ImportFrom,
        language: Language::Python,
        is_dynamic: false,
        is_resolved: false,
        symbols: vec!["_gc".to_string()],
        is_reexport: false,
        is_wildcard: false,
    }];
    let results = checker()
        .check_unused_imports("/tmp/test/src/app.py", content, &imports, &[], &no_traits())
        .unwrap();
    assert!(
        !results.is_empty(),
        "Truly unused aliased import must still be flagged"
    );
    assert!(
        results[0].message.value.contains("_gc"),
        "Message should mention the alias binding, got: {}",
        results[0].message
    );
}

#[test]
fn aes203_python_plain_import_statement_uses_first_segment_binding() {
    // `import os.path` binds `os` in module scope; usage via `os` is not unused.
    let content = "import os.path\n\n\ndef show() -> str:\n    return os.path.join(\"a\", \"b\")\n";
    let imports = vec![ImportEntry {
        source_file: PathBuf::new(),
        raw_path: "os.path".to_string(),
        resolved_path: None,
        import_type: ImportType::Import,
        language: Language::Python,
        is_dynamic: false,
        is_resolved: false,
        symbols: vec!["os".to_string()],
        is_reexport: false,
        is_wildcard: false,
    }];
    let results = checker()
        .check_unused_imports(
            "/tmp/test/src/app.py",
            content,
            &imports,
            &["os".to_string(), "path".to_string()],
            &no_traits(),
        )
        .unwrap();
    assert!(
        results.is_empty(),
        "Dotted import bound to first segment must not be flagged, got {}",
        results.len()
    );
}

#[test]
fn aes203_rust_aliased_use_import_used_via_alias_not_flagged() {
    // `use foo::bar as baz` + usage of `baz` — zero AES203 findings.
    let content = "use std::collections::HashMap as Map;\n\nfn main() {\n    let _m: Map<u8, u8> = Map::new();\n}\n";
    let imports = vec![ImportEntry {
        source_file: PathBuf::new(),
        raw_path: "std::collections::HashMap".to_string(),
        resolved_path: None,
        import_type: ImportType::Use,
        language: Language::Rust,
        is_dynamic: false,
        is_resolved: false,
        symbols: vec!["Map".to_string()],
        is_reexport: false,
        is_wildcard: false,
    }];
    let results = checker()
        .check_unused_imports(
            "/tmp/test/src/app.rs",
            content,
            &imports,
            &["Map".to_string()],
            &no_traits(),
        )
        .unwrap();
    assert!(
        results.is_empty(),
        "Aliased Rust use import must not be flagged, got {}",
        results.len()
    );
}

#[test]
fn aes203_typescript_aliased_named_import_used_via_alias_not_flagged() {
    // `import { Foo as Bar } from './mod'` + usage of `Bar` — zero findings.
    let content = "import { Foo as Bar } from './mod';\n\nconst x: Bar = make();\n";
    let imports = vec![ImportEntry {
        source_file: PathBuf::new(),
        raw_path: "./mod".to_string(),
        resolved_path: None,
        import_type: ImportType::ImportFrom,
        language: Language::TypeScript,
        is_dynamic: false,
        is_resolved: false,
        symbols: vec!["Bar".to_string()],
        is_reexport: false,
        is_wildcard: false,
    }];
    let results = checker()
        .check_unused_imports(
            "/tmp/test/src/app.ts",
            content,
            &imports,
            &["Bar".to_string()],
            &no_traits(),
        )
        .unwrap();
    assert!(
        results.is_empty(),
        "Aliased TS import must not be flagged, got {}",
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
            &no_traits(),
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
    // CalculatorProtocol imported for method dispatch scope —
    // the trait is never named explicitly at call sites,
    // but the compiler needs it in scope to resolve methods.
    // Cross-file analysis: Calculator implements CalculatorProtocol (from trait map).
    let content = "use calculator_shared::contract_calculator_protocol::CalculatorProtocol;\n\n"
        .to_string()
        + "struct AdditionAnalyzer;\n\n"
        + "impl AdditionAnalyzer {\n"
        + "    fn evaluate(&self, x: i32) -> i32 { x }\n"
        + "}\n";
    let imports = vec![rust_use(
        "calculator_shared::contract_calculator_protocol::CalculatorProtocol",
    )];
    let used_ids = vec!["AdditionAnalyzer".to_string()];

    let mut traits = std::collections::HashMap::new();
    traits.insert(
        "CalculatorProtocol".to_string(),
        vec!["AdditionAnalyzer".to_string()],
    );

    let results = checker()
        .check_unused_imports(
            "/tmp/test/src/app.rs",
            &content,
            &imports,
            &used_ids,
            &traits,
        )
        .unwrap();
    assert!(
        results.is_empty(),
        "Trait import used for method dispatch should not be flagged, got {} violations",
        results.len()
    );
}

#[test]
fn aes203_trait_suffix_not_flagged_when_used() {
    let content = "use some_crate::MyTrait;\n\nfn main() {\n    println!(\"hi\");\n}\n";
    let imports = vec![rust_use("some_crate::MyTrait")];
    let used_ids = vec!["main".to_string()];

    let mut traits = std::collections::HashMap::new();
    traits.insert("MyTrait".to_string(), vec!["SomeType".to_string()]);

    // MyTrait not implemented for any type used in this file → should flag
    let results = checker()
        .check_unused_imports(
            "/tmp/test/src/app.rs",
            content,
            &imports,
            &used_ids,
            &traits,
        )
        .unwrap();
    assert!(
        !results.is_empty(),
        "Import with no matching trait impl should be flagged"
    );
}

#[test]
fn aes203_actual_unused_still_flagged() {
    let content = "use std::collections::HashMap;\n\nfn main() {\n    println!(\"hello\");\n}\n";
    let imports = vec![rust_use("std::collections::HashMap")];
    let results = checker()
        .check_unused_imports("/tmp/test/src/app.rs", content, &imports, &[], &no_traits())
        .unwrap();
    assert!(
        !results.is_empty(),
        "Actual unused import must still be flagged"
    );
}

// ─── AES203: cross-file trait map detection ───────────────

#[test]
fn aes203_trait_map_detects_implicit_usage() {
    // Import a trait that is NOT caught by naming heuristic but IS
    // implemented for a type used in this file via the trait map.
    let content = "use my_crate::MyCustomTrait;\n\nfn main() {\n    let _ = MyStruct;\n}\n";
    let imports = vec![rust_use("my_crate::MyCustomTrait")];
    let used_ids = vec!["MyStruct".to_string(), "main".to_string()];

    let mut traits = HashMap::new();
    traits.insert("MyCustomTrait".to_string(), vec!["MyStruct".to_string()]);

    let results = checker()
        .check_unused_imports(
            "/tmp/test/src/app.rs",
            content,
            &imports,
            &used_ids,
            &traits,
        )
        .unwrap();
    assert!(
        results.is_empty(),
        "Trait import should be allowed when cross-file map shows impl for used type, got {}",
        results.len()
    );
}

#[test]
fn aes203_trait_map_no_match_still_flags() {
    // Import a trait that is implemented for a DIFFERENT type than what's used here.
    let content = "use my_crate::MyCustomTrait;\n\nfn main() {\n    let _ = OtherStruct;\n}\n";
    let imports = vec![rust_use("my_crate::MyCustomTrait")];
    let used_ids = vec!["OtherStruct".to_string(), "main".to_string()];

    let mut traits = HashMap::new();
    traits.insert(
        "MyCustomTrait".to_string(),
        vec!["UnrelatedStruct".to_string()],
    );

    let results = checker()
        .check_unused_imports(
            "/tmp/test/src/app.rs",
            content,
            &imports,
            &used_ids,
            &traits,
        )
        .unwrap();
    assert!(
        !results.is_empty(),
        "Trait import should be flagged when no type in this file implements it"
    );
}

#[test]
fn aes203_trait_map_last_segment_match() {
    // Import full path "crate::traits::MyTrait" — the map has "MyTrait"
    let content = "use crate::traits::MyTrait;\n\nfn main() {\n    let _ = Foo;\n}\n";
    let imports = vec![rust_use("crate::traits::MyTrait")];
    let used_ids = vec!["Foo".to_string()];

    let mut traits = HashMap::new();
    traits.insert("MyTrait".to_string(), vec!["Foo".to_string()]);

    let results = checker()
        .check_unused_imports(
            "/tmp/test/src/main.rs",
            content,
            &imports,
            &used_ids,
            &traits,
        )
        .unwrap();
    assert!(
        results.is_empty(),
        "Should match trait by last segment of import path"
    );
}
