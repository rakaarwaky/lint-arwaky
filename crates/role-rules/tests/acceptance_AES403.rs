// Acceptance test AES403 — Capability implementation.
// Capabilities must have >= 1 implementor and max 3 types per file.
//
// Every inline fixture below is a well-formed 3-block capability carrying its
// `Block 1:` / `Block 2:` / `Block 3:` banners. That is not decoration: the
// block-marker sub-check reports a capability with no banner, so a fixture
// written as a bare `struct` + two `impl` lines would trip a second,
// unrelated rule and make these tests assert the wrong thing. The banner block
// is factored out below so each fixture states only the defect it is about.
use role_rules_lint_arwaky::root_role_rules_container::RoleContainer;
use shared_config_system::taxonomy_config_system_vo::ArchitectureConfig;
use shared_filesystem::taxonomy_filesystem_vo::{FileEntry, Language};
use shared_role_rules::taxonomy_role_rules_request::RoleRequest;
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

fn run_audit(files: Vec<FileEntry>) -> Vec<shared_common::LintResult> {
    let config = ArchitectureConfig::default();
    let container = RoleContainer::new_with_config(config);
    let orch = container.orchestrator();
    orch.execute(RoleRequest::audit(&files)).into_violations()
}

/// The three block banners a well-formed capability carries.
///
/// Prepended to a fixture so the block-marker sub-check stays silent and the
/// test asserts only the defect it is named for.
const BANNERS: &str = "// ─── Block 1: Struct Definition ───\n\
     // ─── Block 2: Protocol Trait Implementation ───\n\
     // ─── Block 3: Constructors, Std Traits, Helpers ───\n";

/// The same three banners in Python's comment syntax. The block-marker parser
/// is language-agnostic and keys off the comment sigil, so a Python fixture
/// needs `#` banners for its markers to be read at all.
const PY_BANNERS: &str = "# ─── Block 1: Class Definition ───\n\
     # ─── Block 2: Protocol Method Implementation ───\n\
     # ─── Block 3: Dunder Methods, Factories, Helpers ───\n";

/// Run the audit and keep only the AES403 findings, which is what every test
/// in this file asserts on.
fn aes403_findings(files: Vec<FileEntry>) -> Vec<shared_common::LintResult> {
    run_audit(files)
        .into_iter()
        .filter(|r| r.code.code() == "AES403")
        .collect()
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
    assert_eq!(aes403[0].severity, shared_common::Severity::HIGH);
}

// ── Valid capability with implementor → no violation ──

#[test]
fn aes403_valid_capability_no_violation() {
    let file = make_file(
        "src/capabilities_user_service.rs",
        Language::Rust,
        &format!(
            "{BANNERS}\
             pub struct UserService {{}}\nimpl IUserServiceProtocol for UserService {{}}\n"
        ),
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
        &format!(
            "{PY_BANNERS}\
             class UserService(IUserServiceProtocol):\n    pass\n"
        ),
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
    assert_eq!(aes403[0].severity, shared_common::Severity::HIGH);
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
        &format!(
            "{BANNERS}\
             pub struct UserService {{}}\n\
             impl IUserServiceProtocol for UserService {{}}\n\
             impl UserService {{\n    pub fn new() -> Self {{ Self }}\n}}\n"
        ),
    );
    let aes403 = aes403_findings(vec![file]);
    assert!(
        aes403.is_empty(),
        "protocol impl before inherent impl is the correct order, got: {:?}",
        aes403
            .iter()
            .map(|r| r.message.to_string())
            .collect::<Vec<_>>()
    );
}

// ── Local constant → CapabilityLocalConstant ──

#[test]
fn aes403_local_constant_detected() {
    let file = make_file(
        "src/capabilities_user_service.rs",
        Language::Rust,
        &format!(
            "{BANNERS}\
             const MAX_RETRIES: usize = 3;\n\
             pub struct UserService {{}}\n\
             impl IUserServiceProtocol for UserService {{}}\n"
        ),
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
    assert_eq!(aes403[0].severity, shared_common::Severity::MEDIUM);
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
        &format!(
            "{BANNERS}\
             pub struct UserService {{}}\n\
             impl IUserServiceProtocol for UserService {{}}\n\
             #[cfg(test)]\nmod tests {{\n    #[test]\n    fn t() {{}}\n}}\n"
        ),
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
    assert_eq!(aes403[0].severity, shared_common::Severity::LOW);
}

// ── Public helper → CapabilityPublicHelper ──

#[test]
fn aes403_public_helper_detected() {
    let file = make_file(
        "src/capabilities_user_service.rs",
        Language::Rust,
        &format!(
            "{BANNERS}\
             pub struct UserService {{}}\n\
             impl IUserServiceProtocol for UserService {{}}\n\
             impl UserService {{\n\
             \x20   pub fn new() -> Self {{ Self }}\n\
             \x20   pub fn internal_helper() -> bool {{ true }}\n\
             }}\n"
        ),
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
    assert_eq!(aes403[0].severity, shared_common::Severity::MEDIUM);
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
        &format!(
            "{BANNERS}\
             export class UserService implements IUserServiceProtocol {{\n    execute(): number {{ return 1; }}\n}}\n"
        ),
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
        &format!(
            "{PY_BANNERS}\
             MAX_RETRIES = 3\n\nclass UserService(IUserServiceProtocol):\n    def execute(self):\n        return 1\n"
        ),
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
        &format!(
            "{BANNERS}\
             const MAX_RETRIES = 3;\n\nexport class UserService implements IUserServiceProtocol {{\n    execute(): number {{ return 1; }}\n}}\n"
        ),
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
        &format!(
            "{PY_BANNERS}\
             class UserService(IUserServiceProtocol):\n    def execute(self):\n        return 1\n\n\ndef test_execute():\n    pass\n"
        ),
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
    assert_eq!(aes403[0].severity, shared_common::Severity::LOW);
}

#[test]
fn aes403_typescript_embedded_test_detected() {
    let file = make_file(
        "src/capabilities_user_service.ts",
        Language::TypeScript,
        &format!(
            "{BANNERS}\
             export class UserService implements IUserServiceProtocol {{\n    execute(): number {{ return 1; }}\n}}\n\ndescribe('UserService', () => {{\n    it('runs', () => {{ }});\n}});\n"
        ),
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
    assert_eq!(aes403[0].severity, shared_common::Severity::LOW);
}

// ──────────────────────────────────────────────────────────
// A valid capability in each language produces no violation
// ──────────────────────────────────────────────────────────

#[test]
fn aes403_valid_python_capability_no_violation() {
    let file = make_file(
        "src/capabilities_user_service.py",
        Language::Python,
        &format!(
            "{PY_BANNERS}\
             class UserService(IUserServiceProtocol):\n    def __init__(self, dep):\n        self.dep = dep\n\n    def execute(self):\n        return 1\n\n    def _helper(self):\n        return 2\n"
        ),
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
        &format!(
            "{BANNERS}\
             export class UserService implements IUserServiceProtocol {{\n    private readonly dep: Dep;\n\n    public constructor(dep: Dep) {{ this.dep = dep; }}\n\n    execute(): number {{ return 1; }}\n\n    private _helper(): number {{ return 2; }}\n}}\n"
        ),
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

// ──────────────────────────────────────────────────────────
// CapabilityMultiProtocol — one protocol per capability file
// ──────────────────────────────────────────────────────────

#[test]
fn aes403_python_decorator_protocol_claim_with_methods_ok() {
    // `@with_adapter_protocol` claims the protocol attachment; module-level
    // functions carry the real work, so the implementor check passes. No
    // protocol-named base class, so the multi-protocol sub-check stays silent.
    let file = make_file(
        "src/capabilities_codegraph_tools.py",
        Language::Python,
        &format!(
            "{PY_BANNERS}\
             @with_adapter_protocol\nclass CodegraphToolsAdapter:\n    _display = 'codegraph'\n    def __init__(self, units=None):\n        self._units = dict(ADAPTER_UNITS) if units is None else units\n\ndef build_adapter_unit(tool_id, cfg):\n    return AdapterUnit(tool_id, cfg)\n"
        ),
    );
    let results = run_audit(vec![file]);
    let aes403: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES403")
        .collect();
    assert!(
        aes403.is_empty(),
        "decorated class backed by module functions should not trigger AES403: {aes403:?}"
    );
}

#[test]
fn aes403_python_decorator_protocol_claim_empty_bypass_flagged() {
    // A decorator that claims protocol attachment on a class that carries no
    // methods and no module-level functions is the linter-cheating bypass.
    let file = make_file(
        "src/capabilities_codegraph_tools.py",
        Language::Python,
        &format!(
            "{PY_BANNERS}\
             @with_adapter_protocol\nclass CodegraphToolsAdapter:\n    _display = 'codegraph'\n"
        ),
    );
    let results = run_audit(vec![file]);
    let aes403: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES403")
        .collect();
    assert_eq!(
        aes403.len(),
        1,
        "decorated empty-class bypass must trigger AES403"
    );
    assert!(
        aes403[0].message.value.contains("no contract protocol"),
        "violation should cite the implementor failure"
    );
}

#[test]
fn aes403_python_decorator_renamed_variant_bypass_flagged() {
    // Shape-based decorator match: `@with_tools_protocol` claims protocol
    // attachment; no bases, no methods, no module functions → bypass stays
    // blocked.
    let file = make_file(
        "src/capabilities_tools_adapter.py",
        Language::Python,
        &format!(
            "{PY_BANNERS}\
             @with_tools_protocol\nclass ToolsAdapter:\n    _display = 'tools'\n"
        ),
    );
    let results = run_audit(vec![file]);
    let aes403: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES403")
        .collect();
    assert_eq!(
        aes403.len(),
        1,
        "renamed with_*_protocol decorator bypass must still trigger AES403"
    );
}

#[test]
fn aes403_multi_protocol_rust_detected() {
    // Two distinct protocol impls on the same struct → MEDIUM violation.
    let file = make_file(
        "src/capabilities_dual_protocol.rs",
        Language::Rust,
        &format!(
            "{BANNERS}\
             pub struct DualService {{}}\n\
             impl IFooProtocol for DualService {{}}\n\
             impl IBarProtocol for DualService {{}}\n"
        ),
    );
    let results = run_audit(vec![file]);
    let aes403: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES403")
        .collect();
    assert!(
        !aes403.is_empty(),
        "capability implementing 2 protocols should trigger AES403"
    );
    assert!(
        aes403
            .iter()
            .any(|r| r.message.to_string().contains("multiple protocols")),
        "message should mention splitting multiple protocols"
    );
    assert_eq!(
        aes403
            .iter()
            .filter(|r| r.severity == shared_common::Severity::MEDIUM)
            .count(),
        1,
        "multi-protocol violation should be MEDIUM"
    );
}

#[test]
fn aes403_multi_protocol_rust_single_no_violation() {
    let file = make_file(
        "src/capabilities_foo_service.rs",
        Language::Rust,
        "pub struct FooService {}\nimpl IFooProtocol for FooService {}\n",
    );
    let results = run_audit(vec![file]);
    let multi: Vec<_> = results
        .iter()
        .filter(|r| {
            r.code.code() == "AES403" && r.message.to_string().contains("multiple protocols")
        })
        .collect();
    assert!(
        multi.is_empty(),
        "single-protocol capability should not trigger multi-protocol"
    );
}

#[test]
fn aes403_multi_protocol_python_detected() {
    let file = make_file(
        "src/capabilities_dual_protocol.py",
        Language::Python,
        "class DualService(IFooProtocol, IBarProtocol):\n    pass\n",
    );
    let results = run_audit(vec![file]);
    let aes403: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES403")
        .collect();
    assert!(
        !aes403.is_empty(),
        "python capability with 2 protocol bases should trigger AES403"
    );
    assert!(
        aes403
            .iter()
            .any(|r| r.message.to_string().contains("multiple protocols")),
        "message should mention splitting multiple protocols"
    );
}

#[test]
fn aes403_multi_protocol_ts_detected() {
    let file = make_file(
        "src/capabilities_dual_protocol.ts",
        Language::TypeScript,
        "export class DualService implements IFooProtocol, IBarProtocol {}",
    );
    let results = run_audit(vec![file]);
    let aes403: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES403")
        .collect();
    assert!(
        !aes403.is_empty(),
        "typescript capability with 2 protocol implements should trigger AES403"
    );
    assert!(
        aes403
            .iter()
            .any(|r| r.message.to_string().contains("multiple protocols")),
        "message should mention splitting multiple protocols"
    );
}

#[test]
fn aes403_multi_protocol_aggregate_and_protocol_accepted() {
    // Aggregate + protocol on the same type is fine — only count *protocols*.
    let file = make_file(
        "src/capabilities_foo_service.rs",
        Language::Rust,
        "pub struct FooService {}\n\
         impl IFooAggregate for FooService {}\n\
         impl IFooProtocol for FooService {}\n",
    );
    let results = run_audit(vec![file]);
    let multi: Vec<_> = results
        .iter()
        .filter(|r| {
            r.code.code() == "AES403" && r.message.to_string().contains("multiple protocols")
        })
        .collect();
    assert!(
        multi.is_empty(),
        "aggregate + protocol is a single protocol; should not trigger multi-protocol"
    );
}

// ── Block markers: no banner at all ──

/// A capability with all three banners, used as the clean baseline.
const THREE_BANNERS: &str = "// ─── Block 1: Struct Definition ───\n\
     pub struct BannerProbe {}\n\
     // ─── Block 2: Protocol Trait Implementation ───\n\
     impl IBannerProtocol for BannerProbe { fn run(&self) -> usize { 1 } }\n\
     // ─── Block 3: Constructors, Std Traits, Helpers ───\n\
     impl BannerProbe { fn helper(&self) -> usize { 2 } }\n";

fn marker_findings(results: &[shared_common::LintResult]) -> Vec<String> {
    results
        .iter()
        .filter(|r| r.code.code() == "AES403" && r.message.to_string().contains("block marker"))
        .map(|r| r.message.to_string())
        .collect()
}

#[test]
fn aes403_all_three_banners_no_violation() {
    let file = make_file(
        "src/capabilities_banner_probe.rs",
        Language::Rust,
        THREE_BANNERS,
    );
    let findings = marker_findings(&run_audit(vec![file]));
    assert!(
        findings.is_empty(),
        "all three banners in order should pass, got: {findings:?}"
    );
}

#[test]
fn aes403_no_banner_detected() {
    // The defect that motivated the check: two `impl` blocks in a valid order,
    // but no banner at all, so the reader gets no map of the file.
    let file = make_file(
        "src/capabilities_banner_probe.rs",
        Language::Rust,
        "pub struct BannerProbe {}\n\
         impl IBannerProtocol for BannerProbe { fn run(&self) -> usize { 1 } }\n\
         impl BannerProbe { fn helper(&self) -> usize { 2 } }\n",
    );
    let findings = marker_findings(&run_audit(vec![file]));
    assert_eq!(
        findings.len(),
        1,
        "a capability with no banner should report exactly one marker finding, got: {findings:?}"
    );
    assert!(
        findings[0].contains("declares no block markers"),
        "the finding should name the missing-map defect, got: {}",
        findings[0]
    );
}

#[test]
fn aes403_missing_middle_banner_detected() {
    let file = make_file(
        "src/capabilities_banner_probe.rs",
        Language::Rust,
        "// ─── Block 1: Struct Definition ───\n\
         pub struct BannerProbe {}\n\
         impl IBannerProtocol for BannerProbe { fn run(&self) -> usize { 1 } }\n\
         // ─── Block 3: Constructors, Std Traits, Helpers ───\n\
         impl BannerProbe { fn helper(&self) -> usize { 2 } }\n",
    );
    let findings = marker_findings(&run_audit(vec![file]));
    assert_eq!(findings.len(), 1, "got: {findings:?}");
    assert!(
        findings[0].contains("missing block marker"),
        "a subset of banners should read as missing, got: {}",
        findings[0]
    );
    assert!(
        findings[0].contains("Block 2"),
        "the finding should name which block is missing, got: {}",
        findings[0]
    );
}

#[test]
fn aes403_out_of_order_banners_detected() {
    let file = make_file(
        "src/capabilities_banner_probe.rs",
        Language::Rust,
        "// ─── Block 1: Struct Definition ───\n\
         pub struct BannerProbe {}\n\
         // ─── Block 3: Constructors, Std Traits, Helpers ───\n\
         impl BannerProbe { fn new() -> Self { Self } }\n\
         // ─── Block 2: Protocol Trait Implementation ───\n\
         impl IBannerProtocol for BannerProbe { fn run(&self) -> usize { 1 } }\n",
    );
    let findings = marker_findings(&run_audit(vec![file]));
    assert_eq!(findings.len(), 1, "got: {findings:?}");
    assert!(
        findings[0].contains("out of order"),
        "a 1 -> 3 -> 2 sequence should read as out of order, got: {}",
        findings[0]
    );
}

#[test]
fn aes403_banner_above_three_detected() {
    let file = make_file(
        "src/capabilities_banner_probe.rs",
        Language::Rust,
        &format!("{THREE_BANNERS}// ─── Block 4: Trailing ───\n"),
    );
    let findings = marker_findings(&run_audit(vec![file]));
    assert_eq!(findings.len(), 1, "got: {findings:?}");
    assert!(
        findings[0].contains("beyond Block 3"),
        "a Block 4 banner should read as beyond the shape, got: {}",
        findings[0]
    );
}

#[test]
fn aes403_prose_mentioning_blocks_is_not_a_marker() {
    // The HOW-TO and rule messages describe the structure as
    // `Block 1 (types) -> Block 2`. No colon after the digits, so this prose
    // must not satisfy the presence check — the file still has no banner.
    let file = make_file(
        "src/capabilities_banner_probe.rs",
        Language::Rust,
        "// Block 1 (types) -> Block 2 (protocol) -> Block 3 (helpers).\n\
         pub struct BannerProbe {}\n\
         impl IBannerProtocol for BannerProbe { fn run(&self) -> usize { 1 } }\n",
    );
    let findings = marker_findings(&run_audit(vec![file]));
    assert_eq!(
        findings.len(),
        1,
        "prose must not count as a banner, so the file still has none, got: {findings:?}"
    );
    assert!(
        findings[0].contains("declares no block markers"),
        "got: {}",
        findings[0]
    );
}

#[test]
fn aes403_sub_block_is_not_a_marker() {
    // `Sub-Block 4:` contains the substring "Block" but the character before it
    // belongs to a longer word, so it is not a banner.
    let file = make_file(
        "src/capabilities_banner_probe.rs",
        Language::Rust,
        "// Sub-Block 4: Trailing helpers.\n\
         pub struct BannerProbe {}\n\
         impl IBannerProtocol for BannerProbe { fn run(&self) -> usize { 1 } }\n",
    );
    let findings = marker_findings(&run_audit(vec![file]));
    assert_eq!(
        findings.len(),
        1,
        "Sub-Block must not read as a banner, so the file has none, got: {findings:?}"
    );
    assert!(
        findings[0].contains("declares no block markers"),
        "got: {}",
        findings[0]
    );
}

// ── Block markers across languages ──
// The marker parser is shared, so the comment syntax is the only thing that
// varies: `//` for Rust and TypeScript, `#` for Python.

#[test]
fn aes403_python_bash_comments_recognised() {
    let file = make_file(
        "src/capabilities_banner_probe.py",
        Language::Python,
        "# ─── Block 1: Class Definition ───\n\
         class BannerProbe:\n\
             def run(self):\n\
                 return 1\n\
         # ─── Block 2: Protocol Method Implementation ───\n\
         # ─── Block 3: Dunder Methods, Factories, Helpers ───\n",
    );
    let findings = marker_findings(&run_audit(vec![file]));
    assert!(
        findings.is_empty(),
        "python `#` banners should be recognised, got: {findings:?}"
    );
}

#[test]
fn aes403_typescript_slash_comments_recognised() {
    let file = make_file(
        "src/capabilities_banner_probe.ts",
        Language::TypeScript,
        "// ─── Block 1: Class Definition ───\n\
         export class BannerProbe {\n\
           run(): number {\n\
             return 1;\n\
           }\n\
         }\n\
         // ─── Block 2: Protocol Method Implementation ───\n\
         // ─── Block 3: Utility Methods, Factories, Helpers ───\n",
    );
    let findings = marker_findings(&run_audit(vec![file]));
    assert!(
        findings.is_empty(),
        "typescript `//` banners should be recognised, got: {findings:?}"
    );
}

#[test]
fn aes403_python_missing_banner_detected() {
    let file = make_file(
        "src/capabilities_banner_probe.py",
        Language::Python,
        "class BannerProbe:\n    def run(self):\n        return 1\n",
    );
    let findings = marker_findings(&run_audit(vec![file]));
    assert_eq!(findings.len(), 1, "got: {findings:?}");
    assert!(
        findings[0].contains("declares no block markers"),
        "got: {}",
        findings[0]
    );
}
