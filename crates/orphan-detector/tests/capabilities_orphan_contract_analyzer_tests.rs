use orphan_detector_lint_arwaky::capabilities_orphan_contract_analyzer::*;

#[test]
fn has_rust_impl_basic() {
    assert!(has_rust_impl(
        "impl IFooPort for FooAdapter {}",
        "impl IFooPort for",
        &word_boundary_re("IFooPort")
    ));
}

#[test]
fn has_rust_impl_no_match() {
    assert!(!has_rust_impl(
        "use IFooPort;",
        "impl IFooPort for",
        &word_boundary_re("IFooPort")
    ));
}

#[test]
fn has_rust_call_basic() {
    assert!(has_rust_call(
        "use shared::import_rules::IFooPort;",
        &word_boundary_re("IFooPort")
    ));
}

#[test]
fn has_rust_wire_basic() {
    assert!(has_rust_wire(
        "let adapter = Arc::new(FooAdapter::new());",
        &word_boundary_re("FooAdapter")
    ));
}

#[test]
fn has_py_impl_inheritance() {
    assert!(has_py_impl("class FooAdapter(BaseFoo):", "BaseFoo"));
}

#[test]
fn has_py_impl_abc() {
    assert!(has_py_impl(
        "from abc import ABC, abstractmethod\nclass IFoo(ABC):\n    @abstractmethod\n    def method(self): pass",
        "IFoo"
    ));
}

#[test]
fn has_py_call_basic() {
    assert!(has_py_call(
        "from module import IFoo",
        &word_boundary_re("IFoo")
    ));
}

#[test]
fn has_ts_impl_basic() {
    assert!(has_ts_impl("class FooAdapter implements IFoo {}", "IFoo"));
}

#[test]
fn has_ts_call_basic() {
    assert!(has_ts_call(
        "import { IFoo } from './foo';",
        &word_boundary_re("IFoo")
    ));
}

#[test]
fn has_ts_wire_basic() {
    assert!(has_ts_wire(
        "const adapter = new FooAdapter();",
        &word_boundary_re("FooAdapter")
    ));
}

#[test]
fn has_ts_wire_no_arc() {
    assert!(!has_ts_wire(
        "const x = Arc<dyn IFoo>;",
        &word_boundary_re("IFoo")
    ));
}

#[test]
fn word_boundary_no_false_positive() {
    let re = word_boundary_re("Port");
    assert!(re.is_match("impl Port for"));
    assert!(!re.is_match("Portability"));
    assert!(!re.is_match("Transport"));
}

#[test]
fn strip_comments_rust() {
    let code = "use IFoo;\n// trait FakeTrait\nlet x = 5; // inline comment\n";
    let stripped = strip_comments(code);
    assert!(stripped.contains("use IFoo;"));
    assert!(!stripped.contains("FakeTrait"));
    assert!(!stripped.contains("inline comment"));
}

#[test]
fn strip_comments_python() {
    let code = "import IFoo\n# class FakeTrait:\nx = 5  # inline\n";
    let stripped = strip_comments(code);
    assert!(stripped.contains("import IFoo"));
    assert!(!stripped.contains("FakeTrait"));
}

#[test]
fn extract_trait_name_rust() {
    let content = "pub trait IFooPort: Send + Sync { fn method(&self); }";
    assert_eq!(
        extract_contract_trait_name(content, "contract_foo_port.rs"),
        Some("IFooPort".to_string())
    );
}

#[test]
fn extract_trait_name_python() {
    let content = "class IFoo(ABC):\n    @abstractmethod\n    def method(self): pass";
    assert_eq!(
        extract_contract_trait_name(content, "contract_foo.py"),
        Some("IFoo".to_string())
    );
}

#[test]
fn extract_trait_name_typescript() {
    let content = "export interface IFoo { method(): void; }";
    assert_eq!(
        extract_contract_trait_name(content, "contract_foo.ts"),
        Some("IFoo".to_string())
    );
}
