use shared_role_rules::utility_signature_parser::{
    extract_python_method_signatures, extract_trait_method_signatures,
    extract_typescript_method_signatures, python_signature_uses_forbidden_primitive,
    signature_uses_forbidden_primitive, typescript_signature_uses_forbidden_primitive,
};

#[test]
fn extract_trait_method_signatures_rust() {
    let content = "pub trait IReaderProtocol {\n    fn read(&self) -> Result<String>;\n    fn other() {}\n}\nfn free() {}\n";
    let sigs = extract_trait_method_signatures(content);
    assert_eq!(sigs.len(), 1);
    assert_eq!(sigs[0].0, 2);
    assert!(sigs[0].1.contains("fn read"));
}
#[test]
fn extract_python_method_signatures_with_primitives() {
    let content = "class Foo:\n    def run(self) -> str:\n        pass\n    def safe(self, value) -> str:\n        return value\n";
    let sigs = extract_python_method_signatures(content);
    assert_eq!(sigs.len(), 2);
    assert!(sigs[0].1.contains("def run"));
    assert!(sigs[1].1.contains("def safe"));
}
#[test]
fn extract_typescript_method_signatures_test() {
    let content = "interface IFoo {\n  getName(): string;\n  safeName(): unknown;\n}\n";
    let sigs = extract_typescript_method_signatures(content);
    assert_eq!(sigs.len(), 1);
}
#[test]
fn forbidden_primitive_detection_python() {
    let found = python_signature_uses_forbidden_primitive("def run(self, x: str) -> int:");
    assert!(found.contains(&"str"));
    assert!(found.contains(&"int"));
    let clean =
        python_signature_uses_forbidden_primitive("def run(self, x: ValueObject) -> ValueObject:");
    assert!(clean.is_empty());
}
#[test]
fn python_generic_brackets_with_space_not_flagged_as_bare_list_dict() {
    // `list [ResultVO]` / `dict [KeyVO, ValueVO]` are parameterized and must
    // not be reported as bare `list` / `dict`.
    let found = python_signature_uses_forbidden_primitive("def run(self) -> list [ResultVO]:");
    assert!(
        !found.contains(&"list"),
        "list [ResultVO] should not be bare list"
    );

    let found =
        python_signature_uses_forbidden_primitive("def run(self) -> dict [KeyVO, ValueVO]:");
    assert!(
        !found.contains(&"dict"),
        "dict [K, V] should not be bare dict"
    );

    // Parameter-side spaced generic annotations are also not bare.
    let found =
        python_signature_uses_forbidden_primitive("def run(self, items: list [ResultVO]) -> bool:");
    assert!(
        !found.contains(&"list"),
        "param list [ResultVO] should not be bare list"
    );

    let found = python_signature_uses_forbidden_primitive(
        "def run(self, mapping: dict [KeyVO, ValueVO]) -> bool:",
    );
    assert!(
        !found.contains(&"dict"),
        "param dict [K, V] should not be bare dict"
    );

    // Bare list/dict without brackets are still flagged.
    let found = python_signature_uses_forbidden_primitive("def run(self) -> list:");
    assert!(found.contains(&"list"));
}
#[test]
fn forbidden_primitive_detection_typescript() {
    let found = typescript_signature_uses_forbidden_primitive("getName(x: string): any");
    assert!(found.contains(&"string"));
    assert!(found.contains(&"any"));
}
#[test]
fn forbidden_primitive_detection_rust() {
    let found = signature_uses_forbidden_primitive("fn read(&self, x: i32) -> String;");
    assert!(found.contains(&"i32"));
    assert!(found.contains(&"String"));

    // `Result<String, …>` at a contract boundary is forbidden in its own right,
    // so this signature is NOT clean — assert the specific finding it produces.
    let result_sig = signature_uses_forbidden_primitive(
        "fn read(&self, x: &FilePath) -> Result<String, Error>;",
    );
    assert!(
        result_sig.contains(&"Result<String, _>"),
        "expected Result<String, _>, found {result_sig:?}"
    );

    // Neither a reference parameter nor `bool` return is a primitive.
    let clean = signature_uses_forbidden_primitive("fn read(&self, path: &FilePath) -> bool;");
    assert!(
        clean.is_empty(),
        "expected no forbidden primitives in a clean signature, found {clean:?}"
    );
}
