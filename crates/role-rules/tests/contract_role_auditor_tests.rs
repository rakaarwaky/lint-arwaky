use role_rules_lint_arwaky::ContractRoleChecker;

#[test]
fn extracts_single_line_method_signatures() {
    let src = "pub trait IFoo {\n    fn a(&self) -> bool;\n    fn b(&self, x: &str) -> usize;\n    fn c(&self) -> Result<String, ErrorCode>;\n}\n";
    let sigs = ContractRoleChecker::extract_trait_method_signatures(src);
    assert_eq!(sigs.len(), 3);
    assert!(sigs[0].1.contains("fn a"));
    assert!(sigs[1].1.contains("fn b"));
    assert!(sigs[2].1.contains("fn c"));
}

#[test]
fn ignores_free_functions_and_impls() {
    let src = "fn helper() -> String { ... }\nimpl Foo {\n    pub fn method(&self) -> String { ... }\n}\npub trait IFoo {\n    fn only(&self) -> usize;\n}\n";
    let sigs = ContractRoleChecker::extract_trait_method_signatures(src);
    assert_eq!(sigs.len(), 1);
    assert!(sigs[0].1.contains("fn only"));
}

#[test]
fn detects_string_param() {
    assert_eq!(
        ContractRoleChecker::signature_uses_forbidden_primitive("fn f(&self, msg: String);"),
        vec!["String"]
    );
}

#[test]
fn detects_result_string() {
    let v = ContractRoleChecker::signature_uses_forbidden_primitive(
        "fn f(&self, p: &Path) -> Result<String, ErrorCode>;",
    );
    assert!(v.contains(&"String"));
    assert!(v.contains(&"Result<String, _>"));
}

#[test]
fn allows_result_borrowed_str() {
    let v = ContractRoleChecker::signature_uses_forbidden_primitive(
        "fn f(&self, p: &Path) -> Result<&str, ErrorCode>;",
    );
    assert!(v.is_empty());
}

#[test]
fn detects_numeric_primitives() {
    assert!(
        ContractRoleChecker::signature_uses_forbidden_primitive("fn f(&self, n: i32) -> i64;")
            .contains(&"i32")
    );
    assert!(ContractRoleChecker::signature_uses_forbidden_primitive(
        "fn f(&self, n: usize) -> bool;"
    )
    .contains(&"usize"));
    assert!(
        ContractRoleChecker::signature_uses_forbidden_primitive("fn f(&self) -> f64;")
            .contains(&"f64")
    );
}

#[test]
fn allows_borrowed_str() {
    assert!(ContractRoleChecker::signature_uses_forbidden_primitive(
        "fn f(&self, file: &str, content: &str) -> bool;"
    )
    .is_empty());
}

#[test]
fn allows_bool() {
    assert!(
        ContractRoleChecker::signature_uses_forbidden_primitive("fn f(&self) -> bool;").is_empty()
    );
    assert!(ContractRoleChecker::signature_uses_forbidden_primitive(
        "fn f(&self, flag: bool) -> bool;"
    )
    .is_empty());
}

#[test]
fn does_not_match_substring_of_identifier() {
    assert!(ContractRoleChecker::signature_uses_forbidden_primitive(
        "fn f(&self, s: StringBuilder);"
    )
    .is_empty());
    assert!(
        ContractRoleChecker::signature_uses_forbidden_primitive("fn f(&self, x: MyFloat);")
            .is_empty()
    );
}

#[test]
fn empty_signature_is_clean() {
    assert!(ContractRoleChecker::signature_uses_forbidden_primitive("").is_empty());
    assert!(ContractRoleChecker::signature_uses_forbidden_primitive("   ").is_empty());
}
