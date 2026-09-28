// Acceptance test AES402 — Contract primitives.
// Contract trait/aggregate files must not use primitive types in method signatures.
use role_rules_lint_arwaky::root_role_rules_container::RoleContainer;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::filesystem::taxonomy_filesystem_vo::{FileEntry, Language};
use shared::role_rules::taxonomy_role_request::RoleRequest;
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

// ── Rust: protocol trait with String return type → AES402 ──

#[test]
fn aes402_rust_protocol_with_string_return_detected() {
    let file = make_file(
        "src/contract_user_repository_protocol.rs",
        Language::Rust,
        "pub trait IUserRepository {\n    fn find_by_id(&self, id: u64) -> String;\n}\n",
    );
    let results = run_audit(vec![file]);
    let aes402: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES402")
        .collect();
    assert!(
        !aes402.is_empty(),
        "contract protocol with String return type should trigger AES402"
    );
}

// ── Rust: aggregate trait with i32 param → AES402 ──

#[test]
fn aes402_rust_aggregate_with_primitive_param_detected() {
    let file = make_file(
        "src/contract_config_aggregate.rs",
        Language::Rust,
        "pub trait IConfigAggregate {\n    fn set_value(&self, key: i32);\n}\n",
    );
    let results = run_audit(vec![file]);
    let aes402: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES402")
        .collect();
    assert!(
        !aes402.is_empty(),
        "contract aggregate with i32 param should trigger AES402"
    );
}

// ── Python: protocol with str type → AES402 ──

#[test]
fn aes402_python_protocol_with_str_type_detected() {
    let file = make_file(
        "src/contract_user_protocol.py",
        Language::Python,
        "class IUserRepository(Protocol):\n    def find_by_id(self, id: int) -> str: ...\n",
    );
    let results = run_audit(vec![file]);
    let aes402: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES402")
        .collect();
    assert!(
        !aes402.is_empty(),
        "python contract protocol with str type should trigger AES402"
    );
}

// ── TypeScript: protocol with string type → AES402 ──

#[test]
fn aes402_typescript_protocol_with_string_type_detected() {
    let file = make_file(
        "src/contract_user_repository_protocol.ts",
        Language::TypeScript,
        "export interface IUserRepository {\n    findById(id: number): string;\n}\n",
    );
    let results = run_audit(vec![file]);
    let aes402: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES402")
        .collect();
    assert!(
        !aes402.is_empty(),
        "typescript contract protocol with string type should trigger AES402"
    );
}

// ── Clean protocol with no primitive types → no AES402 ──

#[test]
fn aes402_clean_protocol_no_violation() {
    // A protocol with no trait methods (empty trait) should pass
    let file = make_file(
        "src/contract_clean_protocol.rs",
        Language::Rust,
        "pub trait IClean {}\n",
    );
    let results = run_audit(vec![file]);
    let aes402: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES402")
        .collect();
    assert!(
        aes402.is_empty(),
        "clean protocol should not trigger AES402"
    );
}

// ── Non-contract file is not checked by contract rules ──

#[test]
fn aes402_non_contract_file_ignored() {
    let file = make_file(
        "src/taxonomy_my_entity.rs",
        Language::Rust,
        "pub fn process(data: String) -> String { data }\n",
    );
    let results = run_audit(vec![file]);
    let aes402: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES402")
        .collect();
    assert!(
        aes402.is_empty(),
        "non-contract file should not trigger AES402"
    );
}

// ── Default body ──

#[test]
fn aes402_rust_contract_default_body_detected() {
    let file = make_file(
        "src/contract_bad_protocol.rs",
        Language::Rust,
        "pub trait IBadProtocol {\n    fn broken(&self) { unreachable!() }\n}\n",
    );
    let results = run_audit(vec![file]);
    let v = results
        .iter()
        .filter(|r| r.code.code() == "AES402")
        .collect::<Vec<_>>();
    assert!(
        v.iter().any(|r| r.message.value().contains("default body")),
        "default body in Rust trait should trigger AES402"
    );
}

#[test]
fn aes402_python_contract_default_body_detected() {
    let file = make_file(
        "src/contract_bad_protocol.py",
        Language::Python,
        "class IBadProtocol:\n    def broken(self):\n        raise NotImplementedError()\n",
    );
    let results = run_audit(vec![file]);
    let v = results
        .iter()
        .filter(|r| r.code.code() == "AES402")
        .collect::<Vec<_>>();
    assert!(
        v.iter().any(|r| r.message.value().contains("default body")),
        "default body in Python contract should trigger AES402"
    );
}

#[test]
fn aes402_ts_contract_default_body_detected() {
    let file = make_file(
        "src/contract_bad_protocol.ts",
        Language::TypeScript,
        "export interface IBadProtocol {\n    broken(): void { return; }\n}\n",
    );
    let results = run_audit(vec![file]);
    let v = results
        .iter()
        .filter(|r| r.code.code() == "AES402")
        .collect::<Vec<_>>();
    assert!(
        v.iter().any(|r| r.message.value().contains("inline body")),
        "default body in TS interface should trigger AES402"
    );
}

// ── Aggregate method count ──

#[test]
fn aes402_rust_aggregate_multiple_methods_detected() {
    let file = make_file(
        "src/contract_bad_aggregate.rs",
        Language::Rust,
        "pub trait IBadAggregate {\n    fn execute(&self);\n    fn extra(&self);\n}\n",
    );
    let results = run_audit(vec![file]);
    let v = results
        .iter()
        .filter(|r| r.code.code() == "AES402")
        .collect::<Vec<_>>();
    assert!(
        v.iter()
            .any(|r| r.message.value().contains("more than one method")),
        "aggregate with >1 method should trigger AES402"
    );
}

#[test]
fn aes402_python_aggregate_multiple_methods_detected() {
    let file = make_file(
        "src/contract_bad_aggregate.py",
        Language::Python,
        "class IBadAggregate:\n    def execute(self): ...\n    def extra(self): ...\n",
    );
    let results = run_audit(vec![file]);
    let v = results
        .iter()
        .filter(|r| r.code.code() == "AES402")
        .collect::<Vec<_>>();
    assert!(
        v.iter()
            .any(|r| r.message.value().contains("more than one method")),
        "aggregate with >1 method should trigger AES402"
    );
}

#[test]
fn aes402_ts_aggregate_multiple_methods_detected() {
    let file = make_file(
        "src/contract_bad_aggregate.ts",
        Language::TypeScript,
        "export interface IBadAggregate {\n    execute(): void;\n    extra(): void;\n}\n",
    );
    let results = run_audit(vec![file]);
    let v = results
        .iter()
        .filter(|r| r.code.code() == "AES402")
        .collect::<Vec<_>>();
    assert!(
        v.iter()
            .any(|r| r.message.value().contains("more than one method")),
        "aggregate with >1 method should trigger AES402"
    );
}

// ── Dispatch bag ──

#[test]
fn aes402_rust_dispatch_bag_detected() {
    let file = make_file(
        "src/contract_dispatch_protocol.rs",
        Language::Rust,
        "pub trait IDispatchProtocol {\n    fn execute(&self, op: &str, target: u64);\n}\n",
    );
    let results = run_audit(vec![file]);
    let v = results
        .iter()
        .filter(|r| r.code.code() == "AES402")
        .collect::<Vec<_>>();
    assert!(
        v.iter()
            .any(|r| r.message.value().contains("dispatch parameter")
                || r.message.value().contains("operation name")),
        "dispatch bag should trigger AES402"
    );
}

#[test]
fn aes402_python_dispatch_bag_detected() {
    let file = make_file(
        "src/contract_dispatch_protocol.py",
        Language::Python,
        "class IDispatchProtocol:\n    def execute(self, op: str, target: int) -> None: ...\n",
    );
    let results = run_audit(vec![file]);
    let v = results
        .iter()
        .filter(|r| r.code.code() == "AES402")
        .collect::<Vec<_>>();
    assert!(
        v.iter()
            .any(|r| r.message.value().contains("dispatch parameter")
                || r.message.value().contains("operation name")),
        "dispatch bag should trigger AES402"
    );
}

#[test]
fn aes402_ts_dispatch_bag_detected() {
    let file = make_file(
        "src/contract_dispatch_protocol.ts",
        Language::TypeScript,
        "export interface IDispatchProtocol {\n    execute(op: string, target: number): void;\n}\n",
    );
    let results = run_audit(vec![file]);
    let v = results
        .iter()
        .filter(|r| r.code.code() == "AES402")
        .collect::<Vec<_>>();
    assert!(
        v.iter()
            .any(|r| r.message.value().contains("dispatch parameter")
                || r.message.value().contains("operation name")),
        "dispatch bag should trigger AES402"
    );
}

// ── Docstring stub is a declaration, not a body ──

#[test]
fn aes402_python_docstring_then_stub_is_clean() {
    let file = make_file(
        "src/contract_docstring_protocol.py",
        Language::Python,
        "class IDocProtocol(ABC):\n    def run(self, req: RequestVO) -> ResultVO:\n        \
         \"\"\"Run the request.\"\"\"\n        ...\n",
    );
    let results = run_audit(vec![file]);
    let aes402: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES402")
        .collect();
    assert!(
        aes402.is_empty(),
        "a documented `...` stub is a declaration, not a default body"
    );
}

#[test]
fn aes402_python_multiline_docstring_then_stub_is_clean() {
    let file = make_file(
        "src/contract_docstring_multi_protocol.py",
        Language::Python,
        "class IDocProtocol(ABC):\n    def run(self, req: RequestVO) -> ResultVO:\n        \
         \"\"\"Run the request.\n\n        Returns the matching response.\n        \"\"\"\n        ...\n",
    );
    let results = run_audit(vec![file]);
    let aes402: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES402")
        .collect();
    assert!(
        aes402.is_empty(),
        "a multi-line documented `...` stub is a declaration, not a default body"
    );
}

#[test]
fn aes402_python_statement_after_docstring_still_a_body() {
    let file = make_file(
        "src/contract_docstring_body_protocol.py",
        Language::Python,
        "class IDocProtocol(ABC):\n    def run(self, req: RequestVO) -> ResultVO:\n        \
         \"\"\"Run the request.\"\"\"\n        raise NotImplementedError()\n",
    );
    let results = run_audit(vec![file]);
    let v = results
        .iter()
        .filter(|r| r.code.code() == "AES402")
        .collect::<Vec<_>>();
    assert!(
        v.iter().any(|r| r.message.value().contains("default body")),
        "a docstring followed by a real statement is still a default body"
    );
}

// ── Untyped return ──

#[test]
fn aes402_rust_untyped_return_enum_detected() {
    let file = make_file(
        "src/contract_response_protocol.rs",
        Language::Rust,
        "pub enum Response {\n    Ok(String),\n    Err(i32),\n}\n",
    );
    let results = run_audit(vec![file]);
    let v = results
        .iter()
        .filter(|r| r.code.code() == "AES402")
        .collect::<Vec<_>>();
    assert!(
        v.iter()
            .any(|r| r.message.value().contains("primitive fields")
                || r.message.value().contains("untyped return")),
        "enum of primitives should trigger AES402"
    );
}

#[test]
fn aes402_python_untyped_return_enum_detected() {
    let file = make_file(
        "src/contract_response_protocol.py",
        Language::Python,
        "from enum import Enum\n\nclass Response(Enum):\n    OK = 'ok'\n    FAIL = 1\n",
    );
    let results = run_audit(vec![file]);
    let v = results
        .iter()
        .filter(|r| r.code.code() == "AES402")
        .collect::<Vec<_>>();
    assert!(
        v.iter()
            .any(|r| r.message.value().contains("primitive fields")
                || r.message.value().contains("untyped return")),
        "enum of primitives should trigger AES402"
    );
}

#[test]
fn aes402_ts_untyped_return_enum_detected() {
    let file = make_file(
        "src/contract_response_protocol.ts",
        Language::TypeScript,
        "export enum Response {\n  Ok = 'ok',\n  Fail = 1,\n}\n",
    );
    let results = run_audit(vec![file]);
    let v = results
        .iter()
        .filter(|r| r.code.code() == "AES402")
        .collect::<Vec<_>>();
    assert!(
        v.iter()
            .any(|r| r.message.value().contains("primitive fields")
                || r.message.value().contains("untyped return")),
        "enum of primitives should trigger AES402"
    );
}
