// AES403 sub-checks — one protocol per capability file.
use role_rules_lint_arwaky::capabilities_capabilities_python_role_auditor::CapabilitiesPythonRoleAuditor;
use role_rules_lint_arwaky::capabilities_capabilities_rust_role_auditor::CapabilitiesRustRoleAuditor;
use role_rules_lint_arwaky::capabilities_capabilities_ts_role_auditor::CapabilitiesTypeScriptRoleAuditor;
use shared_common::Severity;
use shared_filesystem::taxonomy_filesystem_vo::FileEntry;
use shared_role_rules::ICapabilitiesRoleProtocol;

use shared_filesystem::taxonomy_filesystem_vo::Language;
use std::path::PathBuf;

fn checker() -> Box<dyn ICapabilitiesRoleProtocol> {
    Box::new(CapabilitiesRustRoleAuditor::new())
}

fn checker_for(lang: Language) -> Box<dyn ICapabilitiesRoleProtocol> {
    match lang {
        Language::Rust => Box::new(CapabilitiesRustRoleAuditor::new()),
        Language::Python => Box::new(CapabilitiesPythonRoleAuditor::new()),
        Language::TypeScript | Language::JavaScript => {
            Box::new(CapabilitiesTypeScriptRoleAuditor::new())
        }
        _ => panic!("unsupported language: {lang:?}"),
    }
}

fn make_file(path: &str, lang: Language, content: &str) -> FileEntry {
    FileEntry {
        path: PathBuf::from(path),
        extension: match lang {
            Language::Rust => "rs",
            Language::Python => "py",
            _ => "ts",
        }
        .to_string(),
        language: lang,
        size: content.len() as u64,
        content: content.to_string(),
        parse_ok: false,
        parse_metadata: None,
    }
}

// ──────────────────────────────────────────────────────────
// check_capability_single_protocol — one protocol per file
// ──────────────────────────────────────────────────────────

#[test]
fn single_protocol_rust_multi_flagged() {
    let content = "pub struct DualService {}\nimpl IFooProtocol for DualService {}\nimpl IBarProtocol for DualService {}\n";
    let f = make_file("src/capabilities_dual.rs", Language::Rust, content);
    let mut v = Vec::new();
    checker().check_capability_single_protocol(&f, &mut v);
    assert_eq!(
        v.len(),
        1,
        "two Rust protocol impls should trigger one AES403"
    );
    assert_eq!(v[0].severity, Severity::MEDIUM);
    assert!(v[0].message.to_string().contains("multiple protocols"));
}

#[test]
fn single_protocol_rust_single_no_violation() {
    let content = "pub struct FooService {}\nimpl IFooProtocol for FooService {}\n";
    let f = make_file("src/capabilities_foo.rs", Language::Rust, content);
    let mut v = Vec::new();
    checker().check_capability_single_protocol(&f, &mut v);
    assert!(v.is_empty(), "single protocol should not trigger");
}

#[test]
fn single_protocol_python_multi_flagged() {
    let content = "class DualService(IFooProtocol, IBarProtocol):\n    pass\n";
    let f = make_file("src/capabilities_dual.py", Language::Python, content);
    let mut v = Vec::new();
    checker_for(f.language).check_capability_single_protocol(&f, &mut v);
    assert_eq!(
        v.len(),
        1,
        "two Python protocol bases should trigger one AES403"
    );
    assert_eq!(v[0].severity, Severity::MEDIUM);
    assert!(v[0].message.to_string().contains("multiple protocols"));
}

#[test]
fn single_protocol_ts_multi_flagged() {
    let content = "export class DualService implements IFooProtocol, IBarProtocol {}";
    let f = make_file("src/capabilities_dual.ts", Language::TypeScript, content);
    let mut v = Vec::new();
    checker_for(f.language).check_capability_single_protocol(&f, &mut v);
    assert_eq!(
        v.len(),
        1,
        "two TS protocol implements should trigger one AES403"
    );
    assert_eq!(v[0].severity, Severity::MEDIUM);
    assert!(v[0].message.to_string().contains("multiple protocols"));
}
