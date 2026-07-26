// PURPOSE: Unit tests for DummyImportChecker (AES204)
// Tests dummy function detection, dummy impl detection, taxonomy intent, surface logic.

use import_rules_lint_arwaky::capabilities_dummy_import_checker::DummyImportChecker;
use shared::common::{ContentString, FilePath};

use shared::common::LayerMapVO;
use shared::import_rules::IDummyImportCheckerProtocol;

fn sut() -> DummyImportChecker {
    DummyImportChecker::new()
}

fn empty_layer_map() -> LayerMapVO {
    LayerMapVO::new(std::collections::HashMap::new())
}

fn root_dir() -> FilePath {
    FilePath::new(".").unwrap()
}

// ─── check_dummy_functions ────────────────────────────────

#[test]
fn detects_rust_dummy_function() {
    let content = r#"
use shared::common::FilePath;

fn _use_mandatory_imports() {
    let _ = FilePath::new("x");
}

fn real_logic() {
    println!("doing work");
}
"#;
    let file = FilePath::new("capabilities_test_checker.rs").unwrap();
    let cs = ContentString::new(content.to_string());
    let violations = sut()
        .check_dummy_functions(&file, &cs, &root_dir(), &empty_layer_map())
        .unwrap();

    assert!(
        !violations.is_empty(),
        "Dummy function _use_mandatory_imports should be detected"
    );
    assert!(violations.iter().all(|v| v.code.code() == "AES204"));
}

#[test]
fn no_dummy_function_no_violation() {
    let content = r#"
fn real_function() {
    let x = 42;
    println!("{}", x);
}
"#;
    let file = FilePath::new("capabilities_real.rs").unwrap();
    let cs = ContentString::new(content.to_string());
    let violations = sut()
        .check_dummy_functions(&file, &cs, &root_dir(), &empty_layer_map())
        .unwrap();

    assert!(
        violations.is_empty(),
        "No dummy functions means no violations"
    );
}

#[test]
fn detects_python_dummy_function() {
    let content = r#"
from shared.taxonomy_path_vo import FilePath

def _use_mandatory_imports():
    _ = FilePath("x")

def real_logic():
    print("working")
"#;
    let file = FilePath::new("capabilities_test_checker.py").unwrap();
    let cs = ContentString::new(content.to_string());
    let violations = sut()
        .check_dummy_functions(&file, &cs, &root_dir(), &empty_layer_map())
        .unwrap();

    assert!(
        !violations.is_empty(),
        "Python dummy function should be detected"
    );
}

// ─── check_dummy_impls ────────────────────────────────────

#[test]
fn detects_empty_trait_impl() {
    let content = r#"
use shared::import_rules::IUnusedImportProtocol;

struct MyChecker;

impl IUnusedImportProtocol for MyChecker {
    fn find_unused_imports(&self, _path: &FilePath) -> Vec<LintMessage> {
        todo!()
    }
    fn check_unused_imports(&self, _file: &str, _content: &str, _v: &mut Vec<LintResult>) {
        todo!()
    }
}
"#;
    let file = FilePath::new("capabilities_stub.rs").unwrap();
    let cs = ContentString::new(content.to_string());
    let violations = sut()
        .check_dummy_impls(&file, &cs, &root_dir(), &empty_layer_map())
        .unwrap();

    assert!(
        !violations.is_empty(),
        "Trait impl with all todo!() bodies should be flagged"
    );
}

#[test]
fn real_trait_impl_not_flagged() {
    let content = r#"
use shared::import_rules::IUnusedImportProtocol;

struct MyChecker;

impl IUnusedImportProtocol for MyChecker {
    fn find_unused_imports(&self, _path: &FilePath) -> Vec<LintMessage> {
        let result = do_real_work();
        result.into_iter().map(LintMessage::new).collect()
    }
    fn check_unused_imports(&self, file: &str, content: &str, v: &mut Vec<LintResult>) {
        let aliases = extract_imports(content);
        for alias in aliases {
            v.push(make_result(file, alias));
        }
    }
}
"#;
    let file = FilePath::new("capabilities_real_impl.rs").unwrap();
    let cs = ContentString::new(content.to_string());
    let violations = sut()
        .check_dummy_impls(&file, &cs, &root_dir(), &empty_layer_map())
        .unwrap();

    assert!(
        violations.is_empty(),
        "Real trait impl should not be flagged"
    );
}

// ─── check_dummy_imports ──────────────────────────────────

#[test]
fn import_only_used_in_dummy_function_flagged() {
    let content = r#"
use shared::common::FilePath;

fn _use_mandatory_imports() {
    let _ = FilePath::new("x");
}

fn real_logic() {
    println!("no FilePath here");
}
"#;
    let file = FilePath::new("capabilities_dummy_import.rs").unwrap();
    let cs = ContentString::new(content.to_string());
    let violations = sut()
        .check_dummy_imports(&file, &cs, &root_dir(), &empty_layer_map())
        .unwrap();

    assert!(
        !violations.is_empty(),
        "Import used only in dummy function should be flagged"
    );
}

#[test]
fn import_used_in_real_logic_not_flagged() {
    let content = r#"
use shared::common::FilePath;

fn real_logic() {
    let path = FilePath::new("real/path.rs").unwrap();
    println!("{}", path.value());
}
"#;
    let file = FilePath::new("capabilities_real_usage.rs").unwrap();
    let cs = ContentString::new(content.to_string());
    let violations = sut()
        .check_dummy_imports(&file, &cs, &root_dir(), &empty_layer_map())
        .unwrap();

    assert!(
        violations.is_empty(),
        "Import used in real logic should not be flagged"
    );
}

// ─── check_surface_logic ──────────────────────────────────

#[test]
fn surface_calling_lint_path_directly_flagged() {
    let content = r#"
fn handle_command() {
    let result = lint_path("/some/file.rs");
    println!("{:?}", result);
}
"#;
    let file = FilePath::new("surface_command_handler.rs").unwrap();
    let cs = ContentString::new(content.to_string());
    let violations = sut()
        .check_surface_logic(&file, &cs, &root_dir(), &empty_layer_map())
        .unwrap();

    assert!(
        !violations.is_empty(),
        "Surface calling lint_path() directly should be flagged"
    );
}

#[test]
fn surface_delegating_to_aggregate_not_flagged() {
    let content = r#"
fn handle_command() {
    let result = aggregate.run_audit(&target).await;
    println!("{:?}", result);
}
"#;
    let file = FilePath::new("surface_command_handler.rs").unwrap();
    let cs = ContentString::new(content.to_string());
    let violations = sut()
        .check_surface_logic(&file, &cs, &root_dir(), &empty_layer_map())
        .unwrap();

    assert!(
        violations.is_empty(),
        "Delegating to aggregate is correct surface behavior"
    );
}

// ─── check_taxonomy_intent ────────────────────────────────

#[test]
fn taxonomy_import_only_in_dummy_flagged() {
    let content = r#"
use shared::common::taxonomy_severity_vo::Severity;

fn _use_mandatory_imports() {
    let _ = Severity::MEDIUM;
}
"#;
    let file = FilePath::new("surface_view.rs").unwrap();
    let cs = ContentString::new(content.to_string());
    let violations = sut()
        .check_taxonomy_intent(&file, &cs, &root_dir(), &empty_layer_map())
        .unwrap();

    assert!(
        !violations.is_empty(),
        "Taxonomy import used only in dummy function should be flagged"
    );
}

// ─── Severity & Code ──────────────────────────────────────

#[test]
fn dummy_violations_use_aes204_code() {
    let content = r#"
fn _use_mandatory_imports() {
    let _ = 42;
}
"#;
    let file = FilePath::new("capabilities_x.rs").unwrap();
    let cs = ContentString::new(content.to_string());
    let violations = sut()
        .check_dummy_functions(&file, &cs, &root_dir(), &empty_layer_map())
        .unwrap();

    for v in &violations {
        assert_eq!(v.code.code(), "AES204");
    }
}

// ─── Edge: Empty Content ──────────────────────────────────

#[test]
fn empty_content_no_violations() {
    let file = FilePath::new("capabilities_empty.rs").unwrap();
    let cs = ContentString::new(String::new());
    let mut violations = Vec::new();
    violations.extend(
        sut()
            .check_dummy_functions(&file, &cs, &root_dir(), &empty_layer_map())
            .unwrap(),
    );
    violations.extend(
        sut()
            .check_dummy_impls(&file, &cs, &root_dir(), &empty_layer_map())
            .unwrap(),
    );
    violations.extend(
        sut()
            .check_dummy_imports(&file, &cs, &root_dir(), &empty_layer_map())
            .unwrap(),
    );
    violations.extend(
        sut()
            .check_surface_logic(&file, &cs, &root_dir(), &empty_layer_map())
            .unwrap(),
    );

    assert!(violations.is_empty());
}
