// Unit tests for the three language-specific capabilities auditors — AES403 sub-checks.
use role_rules_lint_arwaky::capabilities_capabilities_python_role_auditor::CapabilitiesPythonRoleAuditor;
use role_rules_lint_arwaky::capabilities_capabilities_rust_role_auditor::CapabilitiesRustRoleAuditor;
use role_rules_lint_arwaky::capabilities_capabilities_ts_role_auditor::CapabilitiesTypeScriptRoleAuditor;
use shared_common::Severity;
use shared_filesystem::taxonomy_filesystem_vo::FileEntry;
use shared_role_rules::ICapabilitiesRoleProtocol;

use shared_filesystem::taxonomy_filesystem_vo::{
    ExternalReferenceMap, Language, ParseMetadata, RustMetadata,
};
use std::path::PathBuf;

fn checker() -> Box<dyn ICapabilitiesRoleProtocol> {
    Box::new(CapabilitiesRustRoleAuditor::new())
}

/// The three block banners, in the comment syntax of each language.
///
/// `check_capability_routing` runs every AES403 sub-check including the
/// block-marker one, so a fixture that asserts a *different* sub-check is clean
/// must carry the banners too — otherwise it reports the missing map and the
/// test fails for the wrong reason. The parser keys off the comment sigil, so
/// each language needs its own spelling.
fn banners(lang: Language) -> &'static str {
    match lang {
        Language::Python => {
            "# ─── Block 1: Class Definition ───\n\
             # ─── Block 2: Protocol Method Implementation ───\n\
             # ─── Block 3: Dunder Methods, Factories, Helpers ───\n"
        }
        _ => {
            "// ─── Block 1: Struct Definition ───\n\
             // ─── Block 2: Protocol Trait Implementation ───\n\
             // ─── Block 3: Constructors, Std Traits, Helpers ───\n"
        }
    }
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

/// A `FileEntry` whose shape comes from parse metadata rather than from its
/// text.
///
/// The content carries the three block banners so a test that supplies valid
/// metadata and asserts the file is clean does not trip the block-marker
/// sub-check, which reads the text. `size` is a fixed placeholder — nothing
/// under test reads it.
fn make_file_with_rust_meta(path: &str, meta: RustMetadata) -> FileEntry {
    let content = banners(Language::Rust).to_string();
    FileEntry {
        path: PathBuf::from(path),
        extension: "rs".to_string(),
        language: Language::Rust,
        size: content.len() as u64,
        content,
        parse_ok: true,
        parse_metadata: Some(ParseMetadata::Rust(meta)),
    }
}

#[test]
fn construction_succeeds() {
    let _ = checker();
}

#[test]
fn non_capabilities_layer_skipped() {
    let f = make_file("src/agent_foo.rs", Language::Rust, "pub struct Foo {}");
    let mut v = Vec::new();
    checker().check_capability_routing(&f, "agent", &mut v);
    assert!(
        v.is_empty(),
        "non-capabilities layer must not produce violations"
    );
}

#[test]
fn capabilities_layer_with_parens_routes() {
    // "capabilities(agent)" starts with "capabilities(" so it IS routed
    let f = make_file("src/agent_foo.rs", Language::Rust, "pub struct Foo {}");
    let mut v = Vec::new();
    checker().check_capability_routing(&f, "capabilities(agent)", &mut v);
    // Single struct without impl — should be flagged
    assert!(
        !v.is_empty(),
        "capabilities(agent) should route to capabilities checker"
    );
}

#[test]
fn fallback_rust_no_implementor_flagged() {
    let content = "pub struct Foo {}\n";
    let f = make_file("src/capabilities_something.rs", Language::Rust, content);
    let mut v = Vec::new();
    checker().check_capability_routing(&f, "capabilities", &mut v);
    assert!(!v.is_empty(), "should flag missing implementor");
    assert_eq!(v[0].code.code(), "AES403");
}

#[test]
fn fallback_rust_too_many_types_flagged() {
    let content = "pub struct A {}\npub struct B {}\npub struct C {}\npub struct D {}\n";
    let f = make_file("src/capabilities_something.rs", Language::Rust, content);
    let mut v = Vec::new();
    checker().check_capability_routing(&f, "capabilities", &mut v);
    assert!(!v.is_empty(), "should flag too many types");
    assert_eq!(v[0].code.code(), "AES403");
    assert_eq!(v[0].severity, Severity::HIGH);
}

#[test]
fn fallback_rust_valid_composition_no_violation() {
    let content = format!(
        "{}pub struct Foo {{}}\nimpl IFooProtocol for Foo {{}}\n",
        banners(Language::Rust)
    );
    let f = make_file("src/capabilities_something.rs", Language::Rust, &content);
    let mut v = Vec::new();
    checker().check_capability_routing(&f, "capabilities", &mut v);
    assert!(v.is_empty(), "valid capability composition should pass");
}

#[test]
fn fallback_python_no_parent_flagged() {
    let content = "class Foo:\n    pass\n";
    let f = make_file("src/capabilities_something.py", Language::Python, content);
    let mut v = Vec::new();
    checker_for(f.language).check_capability_routing(&f, "capabilities", &mut v);
    assert!(
        !v.is_empty(),
        "python class without parent should be flagged"
    );
    assert_eq!(v[0].code.code(), "AES403");
}

#[test]
fn fallback_python_with_parent_no_violation() {
    // `typing.Protocol` is not a contract protocol. Use a real protocol base.
    let content = format!(
        "{}class Foo(IFooProtocol):\n    pass\n",
        banners(Language::Python)
    );
    let f = make_file("src/capabilities_something.py", Language::Python, &content);
    let mut v = Vec::new();
    checker_for(f.language).check_capability_routing(&f, "capabilities", &mut v);
    assert!(
        v.is_empty(),
        "python class with protocol parent should pass"
    );
}

#[test]
fn metadata_rust_no_implementor_flagged() {
    let meta = RustMetadata {
        struct_definitions: vec!["Foo".into()],
        ..Default::default()
    };
    let f = make_file_with_rust_meta("src/capabilities_something.rs", meta);
    let mut v = Vec::new();
    checker().check_capability_routing(&f, "capabilities", &mut v);
    assert!(
        !v.is_empty(),
        "should flag missing implementor via metadata"
    );
    assert_eq!(v[0].code.code(), "AES403");
}

#[test]
fn metadata_rust_valid_composition_no_violation() {
    let meta = RustMetadata {
        struct_definitions: vec!["Foo".into()],
        impl_blocks: vec![shared_filesystem::taxonomy_filesystem_vo::RustImplItem {
            trait_name: Some("IFooProtocol".into()),
            trait_path: None,
            implementor_type: "Foo".into(),
            has_generics: false,
        }],
        ..Default::default()
    };
    let f = make_file_with_rust_meta("src/capabilities_something.rs", meta);
    let mut v = Vec::new();
    checker().check_capability_routing(&f, "capabilities", &mut v);
    assert!(v.is_empty(), "valid metadata composition should pass");
}

// ──────────────────────────────────────────────────────────
// check_capability_block_order
// ──────────────────────────────────────────────────────────

#[test]
fn block_order_inherent_before_protocol_flagged() {
    // Inherent impl (line 1 of body) comes before the protocol impl.
    let content = "pub struct Foo {}\nimpl Foo {\n    pub fn new() -> Self { Self }\n}\nimpl IFooProtocol for Foo {}\n";
    let f = make_file("src/capabilities_foo.rs", Language::Rust, content);
    let mut v = Vec::new();
    checker().check_capability_block_order(&f, &mut v);
    assert_eq!(
        v.len(),
        1,
        "inherent impl before protocol impl should be flagged"
    );
    assert_eq!(v[0].code.code(), "AES403");
    assert_eq!(v[0].severity, Severity::HIGH);
}

#[test]
fn block_order_protocol_before_inherent_no_violation() {
    // Protocol impl first, then inherent impl — correct 1→2→3 order.
    let content = "pub struct Foo {}\nimpl IFooProtocol for Foo {}\nimpl Foo {\n    pub fn new() -> Self { Self }\n}\n";
    let f = make_file("src/capabilities_foo.rs", Language::Rust, content);
    let mut v = Vec::new();
    checker().check_capability_block_order(&f, &mut v);
    assert!(
        v.is_empty(),
        "protocol impl before inherent impl is the correct order"
    );
}

#[test]
fn block_order_no_inherent_impl_no_violation() {
    // Only a protocol impl — no inherent block to misorder.
    let content = "pub struct Foo {}\nimpl IFooProtocol for Foo {}\n";
    let f = make_file("src/capabilities_foo.rs", Language::Rust, content);
    let mut v = Vec::new();
    checker().check_capability_block_order(&f, &mut v);
    assert!(
        v.is_empty(),
        "no inherent impl should not trigger block-order check"
    );
}

#[test]
fn block_order_default_impl_before_protocol_flagged() {
    // `impl Default for Foo` is an inherent impl block.
    let content = "pub struct Foo {}\nimpl Default for Foo {\n    fn default() -> Self { Self }\n}\nimpl IFooProtocol for Foo {}\n";
    let f = make_file("src/capabilities_foo.rs", Language::Rust, content);
    let mut v = Vec::new();
    checker().check_capability_block_order(&f, &mut v);
    assert_eq!(
        v.len(),
        1,
        "`impl Default` before protocol impl should be flagged"
    );
}

#[test]
fn block_order_python_helper_before_protocol_flagged() {
    let content = "class Foo(IFooProtocol):\n    def _helper(self):\n        return 1\n\n    def execute(self):\n        return 2\n";
    let f = make_file("src/capabilities_foo.py", Language::Python, content);
    let mut v = Vec::new();
    checker_for(f.language).check_capability_block_order(&f, &mut v);
    assert_eq!(
        v.len(),
        1,
        "a python private helper before the public protocol method should be flagged"
    );
    assert_eq!(v[0].severity, Severity::HIGH);
}

#[test]
fn block_order_python_protocol_before_helper_no_violation() {
    let content = "class Foo(IFooProtocol):\n    def execute(self):\n        return 2\n\n    def _helper(self):\n        return 1\n";
    let f = make_file("src/capabilities_foo.py", Language::Python, content);
    let mut v = Vec::new();
    checker_for(f.language).check_capability_block_order(&f, &mut v);
    assert!(
        v.is_empty(),
        "python protocol method before helper is correct"
    );
}

#[test]
fn block_order_python_constructor_not_treated_as_helper() {
    // `__init__` is Block 1 and must not count as a Block 3 helper.
    let content = "class Foo(IFooProtocol):\n    def __init__(self):\n        self.x = 1\n\n    def execute(self):\n        return self.x\n";
    let f = make_file("src/capabilities_foo.py", Language::Python, content);
    let mut v = Vec::new();
    checker_for(f.language).check_capability_block_order(&f, &mut v);
    assert!(
        v.is_empty(),
        "`__init__` is Block 1 and must not be flagged"
    );
}

#[test]
fn block_order_ts_helper_before_protocol_flagged() {
    let content = "export class Foo implements IFooProtocol {\n    private _helper(): number { return 1; }\n\n    execute(): number { return 2; }\n}\n";
    let f = make_file("src/capabilities_foo.ts", Language::TypeScript, content);
    let mut v = Vec::new();
    checker_for(f.language).check_capability_block_order(&f, &mut v);
    assert_eq!(
        v.len(),
        1,
        "a TS private helper before the public protocol method should be flagged"
    );
    assert_eq!(v[0].severity, Severity::HIGH);
}

#[test]
fn block_order_ts_protocol_before_helper_no_violation() {
    let content = "export class Foo implements IFooProtocol {\n    execute(): number { return 2; }\n\n    private _helper(): number { return 1; }\n}\n";
    let f = make_file("src/capabilities_foo.ts", Language::TypeScript, content);
    let mut v = Vec::new();
    checker_for(f.language).check_capability_block_order(&f, &mut v);
    assert!(v.is_empty(), "TS protocol method before helper is correct");
}

#[test]
fn block_order_ts_constructor_is_block_one() {
    let content = "export class Foo implements IFooProtocol {\n    constructor(private dep: Dep) {}\n\n    execute(): number { return 1; }\n}\n";
    let f = make_file("src/capabilities_foo.ts", Language::TypeScript, content);
    let mut v = Vec::new();
    checker_for(f.language).check_capability_block_order(&f, &mut v);
    assert!(
        v.is_empty(),
        "the constructor is Block 1 and must not be flagged"
    );
}

// ──────────────────────────────────────────────────────────
// check_capability_constant_placement
// ──────────────────────────────────────────────────────────

#[test]
fn constant_placement_file_level_const_flagged() {
    let content = "const MAX_WORDS: usize = 3;\npub struct Foo {}\nimpl IFooProtocol for Foo {}\n";
    let f = make_file("src/capabilities_foo.rs", Language::Rust, content);
    let mut v = Vec::new();
    checker().check_capability_constant_placement(&f, &mut v);
    assert_eq!(
        v.len(),
        1,
        "file-level const in capabilities should be flagged"
    );
    assert_eq!(v[0].code.code(), "AES403");
    assert_eq!(v[0].severity, Severity::MEDIUM);
    assert!(v[0].message.to_string().contains("MAX_WORDS"));
}

#[test]
fn constant_placement_no_local_const_no_violation() {
    let content = "pub struct Foo {}\nimpl IFooProtocol for Foo {}\n";
    let f = make_file("src/capabilities_foo.rs", Language::Rust, content);
    let mut v = Vec::new();
    checker().check_capability_constant_placement(&f, &mut v);
    assert!(
        v.is_empty(),
        "no file-level const should not trigger constant placement check"
    );
}

#[test]
fn constant_placement_two_consts_flagged() {
    let content = "const A: usize = 1;\nconst B: &str = \"x\";\npub struct Foo {}\nimpl IFooProtocol for Foo {}\n";
    let f = make_file("src/capabilities_foo.rs", Language::Rust, content);
    let mut v = Vec::new();
    checker().check_capability_constant_placement(&f, &mut v);
    assert_eq!(v.len(), 2, "two file-level consts should each be flagged");
}

#[test]
fn constant_placement_python_module_const_flagged() {
    let content =
        "MAX_WORDS = 3\n\nclass Foo(IFooProtocol):\n    def execute(self):\n        return 1\n";
    let f = make_file("src/capabilities_foo.py", Language::Python, content);
    let mut v = Vec::new();
    checker_for(f.language).check_capability_constant_placement(&f, &mut v);
    assert_eq!(
        v.len(),
        1,
        "a module-level python constant should be flagged"
    );
    assert_eq!(v[0].severity, Severity::MEDIUM);
    assert!(v[0].message.to_string().contains("MAX_WORDS"));
}

#[test]
fn constant_placement_python_lowercase_name_ignored() {
    // Only SHOUTING_CASE names are treated as policy constants.
    let content =
        "counter = 3\n\nclass Foo(IFooProtocol):\n    def execute(self):\n        return 1\n";
    let f = make_file("src/capabilities_foo.py", Language::Python, content);
    let mut v = Vec::new();
    checker_for(f.language).check_capability_constant_placement(&f, &mut v);
    assert!(
        v.is_empty(),
        "a lowercase module binding is not a policy constant"
    );
}

#[test]
fn constant_placement_python_class_attribute_ignored() {
    // An indented assignment is a class attribute, not a module constant.
    let content =
        "class Foo(IFooProtocol):\n    MAX_WORDS = 3\n\n    def execute(self):\n        return 1\n";
    let f = make_file("src/capabilities_foo.py", Language::Python, content);
    let mut v = Vec::new();
    checker_for(f.language).check_capability_constant_placement(&f, &mut v);
    assert!(
        v.is_empty(),
        "a class attribute is not a module-level constant"
    );
}

#[test]
fn constant_placement_ts_module_const_flagged() {
    let content = "const MAX_WORDS = 3;\n\nexport class Foo implements IFooProtocol {\n    execute(): number { return 1; }\n}\n";
    let f = make_file("src/capabilities_foo.ts", Language::TypeScript, content);
    let mut v = Vec::new();
    checker_for(f.language).check_capability_constant_placement(&f, &mut v);
    assert_eq!(v.len(), 1, "a module-level TS const should be flagged");
    assert_eq!(v[0].severity, Severity::MEDIUM);
    assert!(v[0].message.to_string().contains("MAX_WORDS"));
}

#[test]
fn constant_placement_ts_class_member_ignored() {
    // An indented `const` inside a class is a class member, not a module constant.
    let content = "export class Foo implements IFooProtocol {\n    private readonly MAX_WORDS = 3;\n\n    execute(): number { return 1; }\n}\n";
    let f = make_file("src/capabilities_foo.ts", Language::TypeScript, content);
    let mut v = Vec::new();
    checker_for(f.language).check_capability_constant_placement(&f, &mut v);
    assert!(
        v.is_empty(),
        "a class member is not a module-level constant"
    );
}

// ──────────────────────────────────────────────────────────
// check_capability_test_placement
// ──────────────────────────────────────────────────────────

#[test]
fn test_placement_cfg_test_flagged() {
    let content = "pub struct Foo {}\nimpl IFooProtocol for Foo {}\n\n#[cfg(test)]\nmod tests {\n    #[test]\n    fn it_works() {}\n}\n";
    let f = make_file("src/capabilities_foo.rs", Language::Rust, content);
    let mut v = Vec::new();
    checker().check_capability_test_placement(&f, &mut v);
    assert_eq!(
        v.len(),
        1,
        "`#[cfg(test)] mod tests` should produce one finding, not two"
    );
    assert_eq!(v[0].severity, Severity::LOW);
}

#[test]
fn test_placement_no_test_code_no_violation() {
    let content = "pub struct Foo {}\nimpl IFooProtocol for Foo {}\n";
    let f = make_file("src/capabilities_foo.rs", Language::Rust, content);
    let mut v = Vec::new();
    checker().check_capability_test_placement(&f, &mut v);
    assert!(
        v.is_empty(),
        "no inline test code should not trigger test placement check"
    );
}

#[test]
fn test_placement_mod_tests_alone_flagged() {
    // `mod tests` without `#[cfg(test)]` is still inline test code.
    let content = "pub struct Foo {}\nimpl IFooProtocol for Foo {}\n\nmod tests {\n    #[test]\n    fn it_works() {}\n}\n";
    let f = make_file("src/capabilities_foo.rs", Language::Rust, content);
    let mut v = Vec::new();
    checker().check_capability_test_placement(&f, &mut v);
    assert_eq!(v.len(), 1, "`mod tests` alone should be flagged once");
}

#[test]
fn test_placement_python_test_function_flagged() {
    let content = "class Foo(IFooProtocol):\n    def execute(self):\n        return 1\n\n\ndef test_execute():\n    assert Foo().execute() == 1\n";
    let f = make_file("src/capabilities_foo.py", Language::Python, content);
    let mut v = Vec::new();
    checker_for(f.language).check_capability_test_placement(&f, &mut v);
    assert_eq!(v.len(), 1, "a python `def test_*` should be flagged once");
    assert_eq!(v[0].severity, Severity::LOW);
}

#[test]
fn test_placement_python_test_class_one_finding() {
    // A whole test class is one finding, not one per method.
    let content = "class Foo(IFooProtocol):\n    def execute(self):\n        return 1\n\n\nclass TestFoo:\n    def test_a(self):\n        pass\n\n    def test_b(self):\n        pass\n";
    let f = make_file("src/capabilities_foo.py", Language::Python, content);
    let mut v = Vec::new();
    checker_for(f.language).check_capability_test_placement(&f, &mut v);
    assert_eq!(v.len(), 1, "a test class should produce one finding");
}

#[test]
fn test_placement_python_no_test_code_no_violation() {
    let content = "class Foo(IFooProtocol):\n    def execute(self):\n        return 1\n";
    let f = make_file("src/capabilities_foo.py", Language::Python, content);
    let mut v = Vec::new();
    checker_for(f.language).check_capability_test_placement(&f, &mut v);
    assert!(v.is_empty(), "no test code should not trigger the check");
}

#[test]
fn test_placement_ts_describe_block_flagged() {
    let content = "export class Foo implements IFooProtocol {\n    execute(): number { return 1; }\n}\n\ndescribe('Foo', () => {\n    it('runs', () => { });\n    it('stops', () => { });\n});\n";
    let f = make_file("src/capabilities_foo.ts", Language::TypeScript, content);
    let mut v = Vec::new();
    checker_for(f.language).check_capability_test_placement(&f, &mut v);
    assert_eq!(v.len(), 1, "a describe block should produce one finding");
    assert_eq!(v[0].severity, Severity::LOW);
}

#[test]
fn test_placement_ts_no_test_code_no_violation() {
    let content =
        "export class Foo implements IFooProtocol {\n    execute(): number { return 1; }\n}\n";
    let f = make_file("src/capabilities_foo.ts", Language::TypeScript, content);
    let mut v = Vec::new();
    checker_for(f.language).check_capability_test_placement(&f, &mut v);
    assert!(v.is_empty(), "no test code should not trigger the check");
}

// ──────────────────────────────────────────────────────────
// check_capability_helper_visibility
// ──────────────────────────────────────────────────────────

#[test]
fn helper_visibility_pub_fn_no_caller_flagged() {
    let content = "pub struct Foo {}\nimpl IFooProtocol for Foo {}\nimpl Foo {\n    pub fn new() -> Self { Self }\n    pub fn internal_helper() -> bool { true }\n}\n";
    let f = make_file("src/capabilities_foo.rs", Language::Rust, content);
    let refs = ExternalReferenceMap::default();
    let mut v = Vec::new();
    checker().check_capability_helper_visibility(&f, &refs, &mut v);
    assert_eq!(
        v.len(),
        1,
        "pub helper with no external caller should be flagged MEDIUM"
    );
    assert_eq!(v[0].severity, Severity::MEDIUM);
    assert!(v[0].message.to_string().contains("internal_helper"));
}

#[test]
fn helper_visibility_prod_caller_not_flagged() {
    let content = "pub struct Foo {}\nimpl IFooProtocol for Foo {}\nimpl Foo {\n    pub fn new() -> Self { Self }\n    pub fn build_graph(data: &str) {}\n}\n";
    let f = make_file("src/capabilities_foo.rs", Language::Rust, content);
    let mut refs = ExternalReferenceMap::default();
    refs.by_file.insert(
        "src/agent_foo_orchestrator.rs".to_string(),
        vec!["build_graph".to_string()],
    );
    let mut v = Vec::new();
    checker().check_capability_helper_visibility(&f, &refs, &mut v);
    assert!(
        v.is_empty(),
        "a helper called from another production module should not be flagged"
    );
}

#[test]
fn helper_visibility_test_only_caller_not_flagged() {
    let content = "pub struct Foo {}\nimpl IFooProtocol for Foo {}\nimpl Foo {\n    pub fn new() -> Self { Self }\n    pub fn cycles() {}\n}\n";
    let f = make_file("src/capabilities_foo.rs", Language::Rust, content);
    let mut refs = ExternalReferenceMap::default();
    refs.by_file.insert(
        "tests/acceptance_foo.rs".to_string(),
        vec!["cycles".to_string()],
    );
    refs.has_test_references = true;
    let mut v = Vec::new();
    checker().check_capability_helper_visibility(&f, &refs, &mut v);
    assert!(
        v.is_empty(),
        "a helper covered only by an integration test must stay `pub`"
    );
}

#[test]
fn helper_visibility_constructor_not_flagged() {
    let content = "pub struct Foo {}\nimpl IFooProtocol for Foo {}\nimpl Foo {\n    pub fn new() -> Self { Self }\n}\n";
    let f = make_file("src/capabilities_foo.rs", Language::Rust, content);
    let refs = ExternalReferenceMap::default();
    let mut v = Vec::new();
    checker().check_capability_helper_visibility(&f, &refs, &mut v);
    assert!(
        v.is_empty(),
        "`pub fn new` is a constructor and should not be flagged"
    );
}

#[test]
fn helper_visibility_pub_crate_fn_not_flagged() {
    let content = "pub struct Foo {}\nimpl IFooProtocol for Foo {}\nimpl Foo {\n    pub fn new() -> Self { Self }\n    pub(crate) fn helper() -> bool { true }\n}\n";
    let f = make_file("src/capabilities_foo.rs", Language::Rust, content);
    let refs = ExternalReferenceMap::default();
    let mut v = Vec::new();
    checker().check_capability_helper_visibility(&f, &refs, &mut v);
    assert!(v.is_empty(), "`pub(crate)` helpers should not be flagged");
}

#[test]
fn helper_visibility_private_fn_not_flagged() {
    let content = "pub struct Foo {}\nimpl IFooProtocol for Foo {}\nimpl Foo {\n    pub fn new() -> Self { Self }\n    fn helper() -> bool { true }\n}\n";
    let f = make_file("src/capabilities_foo.rs", Language::Rust, content);
    let refs = ExternalReferenceMap::default();
    let mut v = Vec::new();
    checker().check_capability_helper_visibility(&f, &refs, &mut v);
    assert!(v.is_empty(), "private `fn` helpers should not be flagged");
}

#[test]
fn helper_visibility_python_public_method_no_caller_flagged() {
    // The reference map holds every parsed file, so the contract protocol
    // declaration of `execute` appears as an external reference and the
    // protocol method is exempt. `internal_helper` has no reference anywhere.
    let content = "class Foo(IFooProtocol):\n    def _private(self):\n        return 0\n\n    def execute(self):\n        return 1\n\n    def internal_helper(self):\n        return 2\n";
    let f = make_file("src/capabilities_foo.py", Language::Python, content);
    let mut refs = ExternalReferenceMap::default();
    refs.by_file.insert(
        "src/contract_foo_protocol.py".to_string(),
        vec!["execute".to_string()],
    );
    let mut v = Vec::new();
    checker_for(f.language).check_capability_helper_visibility(&f, &refs, &mut v);
    assert_eq!(
        v.len(),
        1,
        "a public python method with no caller should be flagged"
    );
    assert_eq!(v[0].severity, Severity::MEDIUM);
    assert!(v[0].message.to_string().contains("internal_helper"));
}

#[test]
fn helper_visibility_python_underscore_method_not_flagged() {
    // A leading underscore marks the method private by convention.
    let content = "class Foo(IFooProtocol):\n    def execute(self):\n        return 1\n\n    def _internal(self):\n        return 2\n";
    let f = make_file("src/capabilities_foo.py", Language::Python, content);
    let mut refs = ExternalReferenceMap::default();
    refs.by_file.insert(
        "src/contract_foo_protocol.py".to_string(),
        vec!["execute".to_string()],
    );
    let mut v = Vec::new();
    checker_for(f.language).check_capability_helper_visibility(&f, &refs, &mut v);
    assert!(v.is_empty(), "an underscore-prefixed method is private");
}

#[test]
fn helper_visibility_python_prod_caller_not_flagged() {
    let content = "class Foo(IFooProtocol):\n    def execute(self):\n        return 1\n\n    def build_graph(self):\n        return 2\n";
    let f = make_file("src/capabilities_foo.py", Language::Python, content);
    let mut refs = ExternalReferenceMap::default();
    refs.by_file.insert(
        "src/contract_foo_protocol.py".to_string(),
        vec!["execute".to_string()],
    );
    refs.by_file.insert(
        "src/agent_foo_orchestrator.py".to_string(),
        vec!["build_graph".to_string()],
    );
    let mut v = Vec::new();
    checker_for(f.language).check_capability_helper_visibility(&f, &refs, &mut v);
    assert!(
        v.is_empty(),
        "a method called from a production module stays public"
    );
}

#[test]
fn helper_visibility_python_test_only_caller_not_flagged() {
    let content = "class Foo(IFooProtocol):\n    def execute(self):\n        return 1\n\n    def cycles(self):\n        return 2\n";
    let f = make_file("src/capabilities_foo.py", Language::Python, content);
    let mut refs = ExternalReferenceMap::default();
    refs.by_file.insert(
        "src/contract_foo_protocol.py".to_string(),
        vec!["execute".to_string()],
    );
    refs.by_file
        .insert("tests/test_foo.py".to_string(), vec!["cycles".to_string()]);
    refs.has_test_references = true;
    let mut v = Vec::new();
    checker_for(f.language).check_capability_helper_visibility(&f, &refs, &mut v);
    assert!(
        v.is_empty(),
        "a method covered only by a test must stay public"
    );
}

#[test]
fn helper_visibility_ts_public_method_no_caller_flagged() {
    let content = "export class Foo implements IFooProtocol {\n    execute(): number { return 1; }\n\n    public internalHelper(): number { return 2; }\n}\n";
    let f = make_file("src/capabilities_foo.ts", Language::TypeScript, content);
    let refs = ExternalReferenceMap::default();
    let mut v = Vec::new();
    checker_for(f.language).check_capability_helper_visibility(&f, &refs, &mut v);
    assert_eq!(
        v.len(),
        1,
        "an explicit `public` helper with no caller should be flagged"
    );
    assert_eq!(v[0].severity, Severity::MEDIUM);
    assert!(v[0].message.to_string().contains("internalHelper"));
}

#[test]
fn helper_visibility_ts_private_method_not_flagged() {
    let content = "export class Foo implements IFooProtocol {\n    execute(): number { return 1; }\n\n    private internalHelper(): number { return 2; }\n}\n";
    let f = make_file("src/capabilities_foo.ts", Language::TypeScript, content);
    let refs = ExternalReferenceMap::default();
    let mut v = Vec::new();
    checker_for(f.language).check_capability_helper_visibility(&f, &refs, &mut v);
    assert!(v.is_empty(), "a `private` method is not a leak");
}

#[test]
fn helper_visibility_ts_constructor_not_flagged() {
    let content = "export class Foo implements IFooProtocol {\n    public constructor(private dep: Dep) {}\n\n    execute(): number { return 1; }\n}\n";
    let f = make_file("src/capabilities_foo.ts", Language::TypeScript, content);
    let refs = ExternalReferenceMap::default();
    let mut v = Vec::new();
    checker_for(f.language).check_capability_helper_visibility(&f, &refs, &mut v);
    assert!(v.is_empty(), "the constructor is exempt");
}

#[test]
fn helper_visibility_ts_prod_caller_not_flagged() {
    let content = "export class Foo implements IFooProtocol {\n    execute(): number { return 1; }\n\n    public buildGraph(): number { return 2; }\n}\n";
    let f = make_file("src/capabilities_foo.ts", Language::TypeScript, content);
    let mut refs = ExternalReferenceMap::default();
    refs.by_file.insert(
        "src/agent_foo_orchestrator.ts".to_string(),
        vec!["buildGraph".to_string()],
    );
    let mut v = Vec::new();
    checker_for(f.language).check_capability_helper_visibility(&f, &refs, &mut v);
    assert!(
        v.is_empty(),
        "a method called from a production module stays public"
    );
}

// ──────────────────────────────────────────────────────────
// #930: referenced_only_from_tests determinism
// ──────────────────────────────────────────────────────────

#[test]
fn referenced_only_from_tests_prod_and_test_ref_returns_false() {
    // A method referenced by both a test file and a prod file is NOT
    // "referenced only from tests" — must return false regardless of
    // HashMap iteration order.
    let mut refs = ExternalReferenceMap::default();
    refs.by_file.insert(
        "tests/acceptance_foo.rs".to_string(),
        vec!["cycles".to_string()],
    );
    refs.by_file.insert(
        "src/agent_foo_orchestrator.rs".to_string(),
        vec!["cycles".to_string()],
    );
    assert!(!refs.referenced_only_from_tests("src/capabilities_foo.rs", "cycles"));
}

#[test]
fn referenced_only_from_tests_test_only_returns_true() {
    // Path must sit under a `/tests/` directory (is_test_or_bench_path).
    let mut refs = ExternalReferenceMap::default();
    refs.by_file.insert(
        "crates/foo/tests/acceptance_foo.rs".to_string(),
        vec!["cycles".to_string()],
    );
    assert!(refs.referenced_only_from_tests("src/capabilities_foo.rs", "cycles"));
}

#[test]
fn referenced_only_from_tests_no_external_refs_returns_false() {
    let refs = ExternalReferenceMap::default();
    assert!(!refs.referenced_only_from_tests("src/capabilities_foo.rs", "cycles"));
}

// ──────────────────────────────────────────────────────────
// Implementor must be a contract PROTOCOL, not an aggregate
// ──────────────────────────────────────────────────────────

#[test]
fn implementor_aggregate_only_flagged() {
    // An aggregate trait is agent-layer work; a capability needs a protocol.
    let content = "pub struct Foo {}\nimpl IFooAggregate for Foo {}\n";
    let f = make_file("src/capabilities_foo.rs", Language::Rust, content);
    let mut v = Vec::new();
    checker().check_capability_implementor(&f, &mut v);
    assert_eq!(
        v.len(),
        1,
        "an aggregate impl is not a valid capability implementor"
    );
    assert_eq!(v[0].severity, Severity::MEDIUM);
    assert!(
        v[0].message.to_string().contains("contract protocol"),
        "message should name the protocol requirement"
    );
}

#[test]
fn implementor_std_trait_only_flagged() {
    // A std trait impl is not a contract protocol.
    let content = "pub struct Foo {}\nimpl Display for Foo {}\n";
    let f = make_file("src/capabilities_foo.rs", Language::Rust, content);
    let mut v = Vec::new();
    checker().check_capability_implementor(&f, &mut v);
    assert_eq!(
        v.len(),
        1,
        "std trait impl alone is not a valid implementor"
    );
}

#[test]
fn implementor_qualified_protocol_path_accepted() {
    // `impl shared::foo::IFooProtocol for Foo` must be recognised.
    let content = "pub struct Foo {}\nimpl shared_role_rules::IFooProtocol for Foo {}\n";
    let f = make_file("src/capabilities_foo.rs", Language::Rust, content);
    let mut v = Vec::new();
    checker().check_capability_implementor(&f, &mut v);
    assert!(
        v.is_empty(),
        "a fully-qualified protocol path should be accepted"
    );
}

#[test]
fn implementor_python_aggregate_base_flagged() {
    let content = "class Foo(IAggregate):\n    pass\n";
    let f = make_file("src/capabilities_foo.py", Language::Python, content);
    let mut v = Vec::new();
    checker_for(f.language).check_capability_implementor(&f, &mut v);
    assert_eq!(
        v.len(),
        1,
        "an aggregate base class is not a valid implementor"
    );
}

#[test]
fn implementor_python_protocol_base_accepted() {
    let content = "class Foo(IFooProtocol):\n    pass\n";
    let f = make_file("src/capabilities_foo.py", Language::Python, content);
    let mut v = Vec::new();
    checker_for(f.language).check_capability_implementor(&f, &mut v);
    assert!(v.is_empty(), "a protocol base class should be accepted");
}

#[test]
fn implementor_python_multiple_bases_one_is_protocol_accepted() {
    let content = "class Foo(Mixin, IFooProtocol):\n    pass\n";
    let f = make_file("src/capabilities_foo.py", Language::Python, content);
    let mut v = Vec::new();
    checker_for(f.language).check_capability_implementor(&f, &mut v);
    assert!(
        v.is_empty(),
        "one protocol base among several should satisfy the rule"
    );
}

#[test]
fn implementor_metadata_aggregate_flagged() {
    let meta = RustMetadata {
        struct_definitions: vec!["Foo".into()],
        impl_blocks: vec![shared_filesystem::taxonomy_filesystem_vo::RustImplItem {
            trait_name: Some("IFooAggregate".into()),
            trait_path: None,
            implementor_type: "Foo".into(),
            has_generics: false,
        }],
        ..Default::default()
    };
    let f = make_file_with_rust_meta("src/capabilities_foo.rs", meta);
    let mut v = Vec::new();
    checker().check_capability_implementor(&f, &mut v);
    assert_eq!(
        v.len(),
        1,
        "metadata path must also reject an aggregate-only implementor"
    );
}

#[test]
fn implementor_metadata_qualified_protocol_accepted() {
    let meta = RustMetadata {
        struct_definitions: vec!["Foo".into()],
        impl_blocks: vec![shared_filesystem::taxonomy_filesystem_vo::RustImplItem {
            trait_name: Some("IFooProtocol".into()),
            trait_path: Some("shared_role_rules::contract_role_protocol".into()),
            implementor_type: "Foo".into(),
            has_generics: false,
        }],
        ..Default::default()
    };
    let f = make_file_with_rust_meta("src/capabilities_foo.rs", meta);
    let mut v = Vec::new();
    checker().check_capability_implementor(&f, &mut v);
    assert!(
        v.is_empty(),
        "metadata path should accept a protocol implementor"
    );
}

#[test]
fn implementor_metadata_protocol_and_aggregate_both_present_accepted() {
    // An aggregate alongside a protocol is fine — the protocol satisfies AES403.
    let meta = RustMetadata {
        struct_definitions: vec!["Foo".into()],
        impl_blocks: vec![
            shared_filesystem::taxonomy_filesystem_vo::RustImplItem {
                trait_name: Some("IFooAggregate".into()),
                trait_path: None,
                implementor_type: "Foo".into(),
                has_generics: false,
            },
            shared_filesystem::taxonomy_filesystem_vo::RustImplItem {
                trait_name: Some("IFooProtocol".into()),
                trait_path: None,
                implementor_type: "Foo".into(),
                has_generics: false,
            },
        ],
        ..Default::default()
    };
    let f = make_file_with_rust_meta("src/capabilities_foo.rs", meta);
    let mut v = Vec::new();
    checker().check_capability_implementor(&f, &mut v);
    assert!(v.is_empty(), "one protocol impl satisfies the rule");
}

// ──────────────────────────────────────────────────────────
// A valid capability in each language produces no violation
// ──────────────────────────────────────────────────────────

#[test]
fn routing_valid_python_capability_no_violation() {
    let content = format!(
        "{}class Foo(IFooProtocol):\n    def execute(self):\n        return 1\n\n    def _helper(self):\n        return 2\n",
        banners(Language::Python)
    );
    let f = make_file("src/capabilities_foo.py", Language::Python, &content);
    let mut v = Vec::new();
    checker_for(f.language).check_capability_routing(&f, "capabilities", &mut v);
    assert!(
        v.is_empty(),
        "a valid python capability should pass every sub-check"
    );
}

#[test]
fn routing_valid_ts_capability_no_violation() {
    let content = format!(
        "{}export class Foo implements IFooProtocol {{\n    private readonly dep: Dep;\n\n    public constructor(dep: Dep) {{ this.dep = dep; }}\n\n    execute(): number {{ return 1; }}\n\n    private _helper(): number {{ return 2; }}\n}}\n",
        banners(Language::TypeScript)
    );
    let f = make_file("src/capabilities_foo.ts", Language::TypeScript, &content);
    let mut v = Vec::new();
    checker_for(f.language).check_capability_routing(&f, "capabilities", &mut v);
    assert!(
        v.is_empty(),
        "a valid TypeScript capability should pass every sub-check"
    );
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
