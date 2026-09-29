// Acceptance test AES403 — Capability implementation.
// Capabilities must have >= 1 implementor and max 3 types per file.
use role_rules_lint_arwaky::root_role_rules_container::RoleContainer;
use shared::config_system::taxonomy_config_system_vo::ArchitectureConfig;
use shared::filesystem::taxonomy_filesystem_vo::{FileEntry, Language};
use shared::role_rules::taxonomy_role_rules_request::RoleRequest;
use std::path::PathBuf;

fn make_file(path: &str, lang: Language, content: &str) -> FileEntry {
    FileEntry {
        path: PathBuf::from(path),
        extension: match lang {
            Language::Rust => "rs",
            Language::Python => "py",
            Language::TypeScript | Language::JavaScript => "ts",
            _ => "txt",
        }
        .to_string(),
        language: lang,
        size: content.len() as u64,
        content: content.to_string(),
        parse_ok: true,
        parse_metadata: None,
    }
}

fn run_audit(files: Vec<FileEntry>) -> Vec<shared::common::LintResult> {
    let config = ArchitectureConfig::default();
    let container = RoleContainer::new_with_config(config);
    let orch = container.orchestrator();
    orch.execute(RoleRequest::audit(&files)).into_violations()
}

// ── No implementor → CapabilityNoImplementor ──

#[test]
fn aes403_no_implementor_detected() {
    let file = make_file(
        "src/capabilities_user_service.rs",
        Language::Rust,
        "pub struct UserService {}\n",
    );
    let results = run_audit(vec![file]);
    let aes403: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES403")
        .collect();
    assert!(
        !aes403.is_empty(),
        "capability without implementor should trigger AES403"
    );
}

// ── Too many types → CapabilityTooManyTypes ──

#[test]
fn aes403_too_many_types_detected() {
    let file = make_file(
        "src/capabilities_feature.rs",
        Language::Rust,
        "pub struct A {}\npub struct B {}\npub struct C {}\npub struct D {}\n",
    );
    let results = run_audit(vec![file]);
    let aes403: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES403")
        .collect();
    assert!(
        !aes403.is_empty(),
        "capability with 4 types should trigger AES403"
    );
    assert_eq!(aes403[0].severity, shared::common::Severity::HIGH);
}

// ── Valid capability with implementor → no violation ──

#[test]
fn aes403_valid_capability_no_violation() {
    let file = make_file(
        "src/capabilities_user_service.rs",
        Language::Rust,
        "pub struct UserService {}\nimpl IUserServiceProtocol for UserService {}\n",
    );
    let results = run_audit(vec![file]);
    let aes403: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES403")
        .collect();
    assert!(
        aes403.is_empty(),
        "valid capability with implementor should not trigger AES403"
    );
}

// ── Python: no parent class → CapabilityNoImplementor ──

#[test]
fn aes403_python_no_parent_detected() {
    let file = make_file(
        "src/capabilities_user_service.py",
        Language::Python,
        "class UserService:\n    pass\n",
    );
    let results = run_audit(vec![file]);
    let aes403: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES403")
        .collect();
    assert!(
        !aes403.is_empty(),
        "python capability without parent should trigger AES403"
    );
}

// ── Python: with parent → no violation ──

#[test]
fn aes403_python_with_parent_no_violation() {
    let file = make_file(
        "src/capabilities_user_service.py",
        Language::Python,
        "class UserService(IUserServiceProtocol):\n    pass\n",
    );
    let results = run_audit(vec![file]);
    let aes403: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES403")
        .collect();
    assert!(
        aes403.is_empty(),
        "python capability with parent should not trigger AES403"
    );
}

// ── Non-capabilities file is not checked ──

#[test]
fn aes403_non_capability_file_ignored() {
    let file = make_file(
        "src/agent_orchestrator.rs",
        Language::Rust,
        "pub struct Orchestrator {}\n",
    );
    let results = run_audit(vec![file]);
    let aes403: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES403")
        .collect();
    assert!(
        aes403.is_empty(),
        "non-capability file should not trigger AES403"
    );
}

// ── Block order: inherent impl before protocol impl → CapabilityBlockOrder ──

#[test]
fn aes403_block_order_inherent_before_protocol_detected() {
    let file = make_file(
        "src/capabilities_user_service.rs",
        Language::Rust,
        "pub struct UserService {}\n\
         impl UserService {\n    pub fn new() -> Self { Self }\n}\n\
         impl IUserServiceProtocol for UserService {}\n",
    );
    let results = run_audit(vec![file]);
    let aes403: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES403")
        .collect();
    assert!(
        !aes403.is_empty(),
        "inherent impl before protocol impl should trigger AES403"
    );
    assert_eq!(aes403[0].severity, shared::common::Severity::HIGH);
    assert!(
        aes403[0].message.to_string().contains("Block 2"),
        "message should name the block-order rule"
    );
}

#[test]
fn aes403_block_order_correct_order_no_violation() {
    let file = make_file(
        "src/capabilities_user_service.rs",
        Language::Rust,
        "pub struct UserService {}\n\
         impl IUserServiceProtocol for UserService {}\n\
         impl UserService {\n    pub fn new() -> Self { Self }\n}\n",
    );
    let results = run_audit(vec![file]);
    let aes403: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES403")
        .collect();
    assert!(
        aes403.is_empty(),
        "protocol impl before inherent impl is the correct order"
    );
}

// ── Local constant → CapabilityLocalConstant ──

#[test]
fn aes403_local_constant_detected() {
    let file = make_file(
        "src/capabilities_user_service.rs",
        Language::Rust,
        "const MAX_RETRIES: usize = 3;\n\
         pub struct UserService {}\n\
         impl IUserServiceProtocol for UserService {}\n",
    );
    let results = run_audit(vec![file]);
    let aes403: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES403")
        .collect();
    assert!(
        !aes403.is_empty(),
        "file-level const in a capability should trigger AES403"
    );
    assert_eq!(aes403[0].severity, shared::common::Severity::MEDIUM);
    assert!(
        aes403[0].message.to_string().contains("MAX_RETRIES"),
        "message should name the constant"
    );
}

// ── Embedded test → CapabilityEmbeddedTest ──

#[test]
fn aes403_embedded_test_detected() {
    let file = make_file(
        "src/capabilities_user_service.rs",
        Language::Rust,
        "pub struct UserService {}\n\
         impl IUserServiceProtocol for UserService {}\n\
         #[cfg(test)]\nmod tests {\n    #[test]\n    fn t() {}\n}\n",
    );
    let results = run_audit(vec![file]);
    let aes403: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES403")
        .collect();
    assert_eq!(
        aes403.len(),
        1,
        "an inline test module should produce exactly one AES403 finding"
    );
    assert_eq!(aes403[0].severity, shared::common::Severity::LOW);
}

// ── Public helper → CapabilityPublicHelper ──

#[test]
fn aes403_public_helper_detected() {
    let file = make_file(
        "src/capabilities_user_service.rs",
        Language::Rust,
        "pub struct UserService {}\n\
         impl IUserServiceProtocol for UserService {}\n\
         impl UserService {\n\
         \x20   pub fn new() -> Self { Self }\n\
         \x20   pub fn internal_helper() -> bool { true }\n\
         }\n",
    );
    let results = run_audit(vec![file]);
    let aes403: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES403")
        .collect();
    assert_eq!(
        aes403.len(),
        1,
        "a non-constructor pub helper with no external caller should produce exactly one AES403 finding"
    );
    assert_eq!(aes403[0].severity, shared::common::Severity::MEDIUM);
    assert!(
        aes403[0].message.to_string().contains("internal_helper"),
        "message should name the helper"
    );
}

// ──────────────────────────────────────────────────────────
// TypeScript: no `implements` clause → CapabilityNoImplementor
// ──────────────────────────────────────────────────────────

#[test]
fn aes403_typescript_no_implements_detected() {
    let file = make_file(
        "src/capabilities_user_service.ts",
        Language::TypeScript,
        "export class UserService {\n    execute(): number { return 1; }\n}\n",
    );
    let results = run_audit(vec![file]);
    let aes403: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES403")
        .collect();
    assert!(
        !aes403.is_empty(),
        "a TypeScript capability without `implements` should trigger AES403"
    );
}

#[test]
fn aes403_typescript_with_implements_no_violation() {
    let file = make_file(
        "src/capabilities_user_service.ts",
        Language::TypeScript,
        "export class UserService implements IUserServiceProtocol {\n    execute(): number { return 1; }\n}\n",
    );
    let results = run_audit(vec![file]);
    let aes403: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES403")
        .collect();
    assert!(
        aes403.is_empty(),
        "a TypeScript capability with `implements` should not trigger AES403"
    );
}

// ──────────────────────────────────────────────────────────
// Module-level constants in each language
// ──────────────────────────────────────────────────────────

#[test]
fn aes403_python_module_constant_detected() {
    let file = make_file(
        "src/capabilities_user_service.py",
        Language::Python,
        "MAX_RETRIES = 3\n\nclass UserService(IUserServiceProtocol):\n    def execute(self):\n        return 1\n",
    );
    let results = run_audit(vec![file]);
    let aes403: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES403")
        .collect();
    assert!(
        !aes403.is_empty(),
        "a python module-level constant should trigger AES403"
    );
    assert!(
        aes403[0].message.to_string().contains("MAX_RETRIES"),
        "message should name the constant"
    );
}

#[test]
fn aes403_typescript_module_constant_detected() {
    let file = make_file(
        "src/capabilities_user_service.ts",
        Language::TypeScript,
        "const MAX_RETRIES = 3;\n\nexport class UserService implements IUserServiceProtocol {\n    execute(): number { return 1; }\n}\n",
    );
    let results = run_audit(vec![file]);
    let aes403: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES403")
        .collect();
    assert!(
        !aes403.is_empty(),
        "a TypeScript module-level const should trigger AES403"
    );
    assert!(
        aes403[0].message.to_string().contains("MAX_RETRIES"),
        "message should name the constant"
    );
}

// ──────────────────────────────────────────────────────────
// Embedded tests in each language
// ──────────────────────────────────────────────────────────

#[test]
fn aes403_python_embedded_test_detected() {
    let file = make_file(
        "src/capabilities_user_service.py",
        Language::Python,
        "class UserService(IUserServiceProtocol):\n    def execute(self):\n        return 1\n\n\ndef test_execute():\n    pass\n",
    );
    let results = run_audit(vec![file]);
    let aes403: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES403")
        .collect();
    assert_eq!(
        aes403.len(),
        1,
        "a python `def test_*` should produce exactly one AES403 finding"
    );
    assert_eq!(aes403[0].severity, shared::common::Severity::LOW);
}

#[test]
fn aes403_typescript_embedded_test_detected() {
    let file = make_file(
        "src/capabilities_user_service.ts",
        Language::TypeScript,
        "export class UserService implements IUserServiceProtocol {\n    execute(): number { return 1; }\n}\n\ndescribe('UserService', () => {\n    it('runs', () => { });\n});\n",
    );
    let results = run_audit(vec![file]);
    let aes403: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES403")
        .collect();
    assert_eq!(
        aes403.len(),
        1,
        "a TypeScript describe block should produce exactly one AES403 finding"
    );
    assert_eq!(aes403[0].severity, shared::common::Severity::LOW);
}

// ──────────────────────────────────────────────────────────
// A valid capability in each language produces no violation
// ──────────────────────────────────────────────────────────

#[test]
fn aes403_valid_python_capability_no_violation() {
    let file = make_file(
        "src/capabilities_user_service.py",
        Language::Python,
        "class UserService(IUserServiceProtocol):\n    def __init__(self, dep):\n        self.dep = dep\n\n    def execute(self):\n        return 1\n\n    def _helper(self):\n        return 2\n",
    );
    let results = run_audit(vec![file]);
    let aes403: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES403")
        .collect();
    assert!(
        aes403.is_empty(),
        "a valid python capability should not trigger AES403"
    );
}

#[test]
fn aes403_valid_typescript_capability_no_violation() {
    let file = make_file(
        "src/capabilities_user_service.ts",
        Language::TypeScript,
        "export class UserService implements IUserServiceProtocol {\n    private readonly dep: Dep;\n\n    public constructor(dep: Dep) { this.dep = dep; }\n\n    execute(): number { return 1; }\n\n    private _helper(): number { return 2; }\n}\n",
    );
    let results = run_audit(vec![file]);
    let aes403: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES403")
        .collect();
    assert!(
        aes403.is_empty(),
        "a valid TypeScript capability should not trigger AES403"
    );
}
