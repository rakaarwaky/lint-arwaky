use orphan_detector_lint_arwaky::capabilities_orphan_contract_analyzer as m;

#[test]
fn has_rust_impl_basic() {
    assert!(m::has_rust_impl(
        "impl IFooPort for FooAdapter {}",
        "impl IFooPort for",
        &m::word_boundary_re("IFooPort")
    ));
}

#[test]
fn has_rust_impl_no_match() {
    assert!(!m::has_rust_impl(
        "use IFooPort;",
        "impl IFooPort for",
        &m::word_boundary_re("IFooPort")
    ));
}

#[test]
fn has_rust_call_specific() {
    assert!(m::has_rust_call(
        "use shared::import_rules::IFooPort;",
        &m::word_boundary_re("IFooPort")
    ));
}

#[test]
fn has_rust_call_not_just_double_colon() {
    // #4: "use std::collections::HashMap" should NOT match IFooPort
    assert!(!m::has_rust_call(
        "use std::collections::HashMap;\n// TODO: implement IFooPort",
        &m::word_boundary_re("IFooPort")
    ));
}

#[test]
fn has_rust_wire_basic() {
    assert!(m::has_rust_wire(
        "let adapter = Arc::new(FooAdapter::new());",
        &m::word_boundary_re("FooAdapter")
    ));
}

#[test]
fn has_py_impl_inheritance() {
    assert!(m::has_py_impl("class FooAdapter(BaseFoo):", "BaseFoo"));
}

#[test]
fn has_py_impl_abc() {
    assert!(m::has_py_impl(
        "class IFoo(ABC):\n    @abstractmethod\n    def method(self): pass",
        "IFoo"
    ));
}

#[test]
fn has_py_call_basic() {
    assert!(m::has_py_call(
        "from module import IFoo",
        &m::word_boundary_re("IFoo")
    ));
}

#[test]
fn has_ts_impl_basic() {
    assert!(m::has_ts_impl(
        "class FooAdapter implements IFoo {}",
        "IFoo"
    ));
}

#[test]
fn has_ts_impl_word_boundary() {
    // #8: "implements IFooBar" should NOT match "IFoo"
    assert!(!m::has_ts_impl(
        "class FooAdapter implements IFooBar {}",
        "IFoo"
    ));
}

#[test]
fn has_ts_call_basic() {
    assert!(m::has_ts_call(
        "import { IFoo } from './foo';",
        &m::word_boundary_re("IFoo")
    ));
}

#[test]
fn has_ts_wire_basic() {
    assert!(m::has_ts_wire(
        "const adapter = new FooAdapter();",
        &m::word_boundary_re("FooAdapter")
    ));
}

#[test]
fn has_ts_wire_no_arc() {
    assert!(!m::has_ts_wire(
        "const x = Arc<dyn IFoo>;",
        &m::word_boundary_re("IFoo")
    ));
}

#[test]
fn word_boundary_no_false_positive() {
    let re = m::word_boundary_re("Port");
    assert!(re.is_match("impl Port for"));
    assert!(!re.is_match("Portability"));
    assert!(!re.is_match("Transport"));
}

#[test]
fn strip_comments_rust_no_hash() {
    // #3: Rust # should NOT be stripped
    let code = "use IFoo;\n// trait FakeTrait\n#[derive(Debug)]\nlet x = 5; // inline\n";
    let stripped = m::strip_comments(code, "rs");
    assert!(stripped.contains("use IFoo;"));
    assert!(!stripped.contains("FakeTrait"));
    assert!(stripped.contains("#[derive(Debug)]"));
    assert!(!stripped.contains("inline"));
}

#[test]
fn strip_comments_python() {
    let code = "import IFoo\n# class FakeTrait:\nx = 5  # inline\n";
    let stripped = m::strip_comments(code, "py");
    assert!(stripped.contains("import IFoo"));
    assert!(!stripped.contains("FakeTrait"));
}

#[test]
fn strip_comments_typescript() {
    let code = "import { IFoo } from './foo';\n// interface FakeTrait\nconst x = 5; // inline\n";
    let stripped = m::strip_comments(code, "ts");
    assert!(stripped.contains("import { IFoo }"));
    assert!(!stripped.contains("FakeTrait"));
}

#[test]
fn extract_trait_name_rust() {
    let content = "pub trait IFooPort: Send + Sync { fn method(&self); }";
    assert_eq!(
        m::extract_contract_trait_name(content, "contract_foo_port.rs"),
        Some("IFooPort".to_string())
    );
}

#[test]
fn extract_trait_name_python_abc() {
    let content = "from abc import ABC, abstractmethod\nclass IFoo(ABC):\n    @abstractmethod\n    def method(self): pass";
    assert_eq!(
        m::extract_contract_trait_name(content, "contract_foo.py"),
        Some("IFoo".to_string())
    );
}

#[test]
fn extract_trait_name_typescript() {
    let content = "export interface IFoo { method(): void; }";
    assert_eq!(
        m::extract_contract_trait_name(content, "contract_foo.ts"),
        Some("IFoo".to_string())
    );
}
