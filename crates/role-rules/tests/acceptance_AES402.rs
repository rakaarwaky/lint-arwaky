// Acceptance test AES402 — Contract primitives.
// Contract trait/aggregate files must not use primitive types in method signatures.
use role_rules_lint_arwaky::root_role_rules_container::RoleContainer;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::filesystem::taxonomy_filesystem_vo::{FileEntry, Language};
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
    orch.run_audit_with_entries(&files)
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
