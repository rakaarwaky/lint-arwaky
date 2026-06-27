use role_rules_lint_arwaky::capabilities_capabilities_role_auditor::CapabilitiesRoleChecker;
use shared::common::taxonomy_path_vo::FilePath;
use shared::taxonomy_source_vo::{ContentString, SourceContentVO};

fn make_source(file: &str, content: &str, language: &str) -> SourceContentVO {
    let fp = FilePath::new(file.to_string()).unwrap_or_default();
    SourceContentVO::new(fp, ContentString::new(content.to_string()), language)
}

// ─── Non-capabilities layer is skipped ──────────────────────────────────────

#[test]
fn non_capabilities_layer_is_skipped() {
    let checker = CapabilitiesRoleChecker::new();
    let mut violations = Vec::new();
    let src = make_source("infrastructure_adapter.rs", "pub struct MyAdapter;", "rust");
    checker.check_capability_routing(&src, "infrastructure", &mut violations);
    assert!(violations.is_empty());
}

// ─── Rust: has protocol import ──────────────────────────────────────────────

#[test]
fn rust_with_protocol_import_and_impl_no_violation() {
    let checker = CapabilitiesRoleChecker::new();
    let mut violations = Vec::new();
    let src = make_source(
        "capabilities_file_checker.rs",
        "use shared::capabilities::contract_file_protocol::IFileChecker;\npub struct FileChecker;\nimpl IFileChecker for FileChecker {}",
        "rust",
    );
    checker.check_capability_routing(&src, "capabilities", &mut violations);
    assert!(violations.is_empty());
}

#[test]
fn rust_without_protocol_import_emits_violation() {
    let checker = CapabilitiesRoleChecker::new();
    let mut violations = Vec::new();
    let src = make_source(
        "capabilities_random.rs",
        "pub struct RandomStuff;\nimpl RandomStuff {}",
        "rust",
    );
    checker.check_capability_routing(&src, "capabilities", &mut violations);
    assert!(!violations.is_empty());
    assert!(violations[0].code.to_string().contains("AES403"));
}

#[test]
fn rust_with_protocol_but_no_impl_emits_routing_violation() {
    let checker = CapabilitiesRoleChecker::new();
    let mut violations = Vec::new();
    let src = make_source(
        "capabilities_file_checker.rs",
        "use shared::capabilities::contract_file_protocol::IFileChecker;\npub struct FileChecker;\n// Missing: struct has no trait impl",
        "rust",
    );
    checker.check_capability_routing(&src, "capabilities", &mut violations);
    assert!(!violations.is_empty());
}

#[test]
fn rust_struct_has_impl_blocks() {
    let checker = CapabilitiesRoleChecker::new();
    let mut violations = Vec::new();
    let src = make_source(
        "capabilities_user_checker.rs",
        "use shared::capabilities::contract_user_protocol::IUserChecker;\npub struct UserChecker;\npub struct OtherStruct;\nimpl IUserChecker for UserChecker {}\nimpl OtherStruct {}",
        "rust",
    );
    checker.check_capability_routing(&src, "capabilities", &mut violations);
    assert!(violations.is_empty());
}

// ─── Python ─────────────────────────────────────────────────────────────────

#[test]
fn python_with_protocol_import_and_class_no_violation() {
    let checker = CapabilitiesRoleChecker::new();
    let mut violations = Vec::new();
    let src = make_source(
        "capabilities_user_checker.py",
        "from somewhere import contract_user_protocol\nclass UserChecker:\n    def check(self): pass",
        "python",
    );
    checker.check_capability_routing(&src, "capabilities", &mut violations);
    assert!(violations.is_empty());
}

#[test]
fn python_without_protocol_import_emits_violation() {
    let checker = CapabilitiesRoleChecker::new();
    let mut violations = Vec::new();
    let src = make_source(
        "capabilities_random.py",
        "class RandomStuff:\n    pass",
        "python",
    );
    checker.check_capability_routing(&src, "capabilities", &mut violations);
    assert!(!violations.is_empty());
}

// ─── JavaScript ─────────────────────────────────────────────────────────────

#[test]
fn js_with_protocol_import_and_class_no_violation() {
    let checker = CapabilitiesRoleChecker::new();
    let mut violations = Vec::new();
    let src = make_source(
        "capabilities_user_checker.ts",
        "import { IUserChecker } from './_protocol'\nclass UserChecker implements IUserChecker {\n    public check(): void {}\n}",
        "javascript",
    );
    checker.check_capability_routing(&src, "capabilities", &mut violations);
    assert!(violations.is_empty());
}

#[test]
fn js_without_protocol_import_emits_violation() {
    let checker = CapabilitiesRoleChecker::new();
    let mut violations = Vec::new();
    let src = make_source(
        "capabilities_random.ts",
        "class RandomStuff {\n    helper(): void {}\n}",
        "javascript",
    );
    checker.check_capability_routing(&src, "capabilities", &mut violations);
    assert!(!violations.is_empty());
}
