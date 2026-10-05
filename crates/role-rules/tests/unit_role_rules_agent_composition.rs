// Unit tests for the two AES405 sub-checks added by this change:
// `check_agent_protocol_forbidden` (no contract protocol in an agent) and
// `check_agent_block_markers` (no block marker above 3).
//
// The two run in different ways: when the filesystem has parsed the file the
// check reads the AST metadata, and when it has not it falls back to scanning
// the raw lines. Both paths are covered here, because either one alone would
// leave a real blind spot — a line-scanner that only understands `class X` and
// `impl IFoo for Y` misses the `export class` and `impl<T>` spellings that real
// code uses.
use role_rules_lint_arwaky::IAgentRoleProtocol;
use role_rules_lint_arwaky::capabilities_agent_rust_role_auditor::AgentRustRoleAuditor;
use role_rules_lint_arwaky::capabilities_agent_ts_role_auditor::AgentTsRoleAuditor;

use shared_filesystem::taxonomy_filesystem_vo::{
    FileEntry, Language, ParseMetadata, PythonClassItem, PythonMetadata, RustImplItem,
    RustMetadata, TSClassItem, TypeScriptMetadata,
};
use std::path::PathBuf;

fn rust_file(content: &str, meta: Option<ParseMetadata>) -> FileEntry {
    FileEntry {
        path: PathBuf::from("src/agent_thing.rs"),
        extension: "rs".to_string(),
        language: Language::Rust,
        size: content.len() as u64,
        content: content.to_string(),
        parse_ok: meta.is_some(),
        parse_metadata: meta,
    }
}

fn ts_file(content: &str, meta: Option<ParseMetadata>) -> FileEntry {
    FileEntry {
        path: PathBuf::from("src/agent_thing.ts"),
        extension: "ts".to_string(),
        language: Language::TypeScript,
        size: content.len() as u64,
        content: content.to_string(),
        parse_ok: meta.is_some(),
        parse_metadata: meta,
    }
}

fn protocol_messages(content: &str, meta: Option<ParseMetadata>, lang: &str) -> Vec<String> {
    let file = if lang == "ts" {
        ts_file(content, meta)
    } else {
        rust_file(content, meta)
    };
    let auditor = AgentRustRoleAuditor::new();
    let mut v = Vec::new();
    auditor.check_agent_protocol_forbidden(&file, &mut v);
    v.into_iter().map(|r| r.message.value.to_string()).collect()
}

fn block_messages(content: &str) -> Vec<String> {
    let auditor = AgentRustRoleAuditor::new();
    let mut v = Vec::new();
    auditor.check_agent_block_markers(&rust_file(content, None), &mut v);
    v.into_iter().map(|r| r.message.value.to_string()).collect()
}

// ── Parsed-metadata path ──────────────────────────────────
//
// The AST parser populates `parse_metadata` in production, so this is the path
// a real audit takes. Asserting only the line-scan fallback would let the
// metadata arms rot unnoticed.

#[test]
fn rust_metadata_protocol_impl_flagged() {
    let content = "pub struct Agent {}\n";
    let mut meta = RustMetadata::default();
    meta.impl_blocks = vec![RustImplItem {
        trait_name: Some("IScannerProtocol".to_string()),
        trait_path: None,
        implementor_type: "Agent".to_string(),
        has_generics: false,
    }];
    let messages = protocol_messages(content, Some(ParseMetadata::Rust(meta)), "rs");
    assert!(
        messages
            .iter()
            .any(|m| m.contains("implements a contract protocol")),
        "rust metadata naming IScannerProtocol must flag: {messages:?}"
    );
}

#[test]
fn rust_metadata_aggregate_impl_not_flagged() {
    let content = "pub struct Agent {}\n";
    let mut meta = RustMetadata::default();
    meta.impl_blocks = vec![RustImplItem {
        trait_name: Some("IThingAggregate".to_string()),
        trait_path: None,
        implementor_type: "Agent".to_string(),
        has_generics: false,
    }];
    let messages = protocol_messages(content, Some(ParseMetadata::Rust(meta)), "rs");
    assert!(
        !messages
            .iter()
            .any(|m| m.contains("implements a contract protocol")),
        "the aggregate trait is not a protocol: {messages:?}"
    );
}

#[test]
fn rust_metadata_generic_impl_flagged() {
    // The AST parser records `impl<T>` via `has_generics`; the trait name is
    // still recorded, so the check reads it the same way.
    let content = "pub struct Agent {}\n";
    let mut meta = RustMetadata::default();
    meta.impl_blocks = vec![RustImplItem {
        trait_name: Some("IScannerProtocol".to_string()),
        trait_path: None,
        implementor_type: "Agent".to_string(),
        has_generics: true,
    }];
    let messages = protocol_messages(content, Some(ParseMetadata::Rust(meta)), "rs");
    assert!(
        messages
            .iter()
            .any(|m| m.contains("implements a contract protocol")),
        "a generic impl in metadata must flag: {messages:?}"
    );
}

#[test]
fn rust_metadata_impl_without_trait_not_flagged() {
    // An inherent `impl Agent { … }` has no trait — nothing to flag.
    let content = "pub struct Agent {}\n";
    let mut meta = RustMetadata::default();
    meta.impl_blocks = vec![RustImplItem {
        trait_name: None,
        trait_path: None,
        implementor_type: "Agent".to_string(),
        has_generics: false,
    }];
    let messages = protocol_messages(content, Some(ParseMetadata::Rust(meta)), "rs");
    assert!(
        !messages
            .iter()
            .any(|m| m.contains("implements a contract protocol")),
        "an inherent impl is not a protocol impl: {messages:?}"
    );
}

#[test]
fn ts_metadata_protocol_impl_flagged() {
    let content = "export class Agent {}\n";
    let mut meta = TypeScriptMetadata::default();
    meta.class_declarations = vec![TSClassItem {
        name: "Agent".to_string(),
        implements: vec!["IScannerProtocol".to_string()],
    }];
    let messages = protocol_messages(content, Some(ParseMetadata::TypeScript(meta)), "ts");
    assert!(
        messages
            .iter()
            .any(|m| m.contains("implements a contract protocol")),
        "ts metadata naming IScannerProtocol must flag: {messages:?}"
    );
}

#[test]
fn ts_metadata_aggregate_impl_not_flagged() {
    let content = "export class Agent {}\n";
    let mut meta = TypeScriptMetadata::default();
    meta.class_declarations = vec![TSClassItem {
        name: "Agent".to_string(),
        implements: vec!["IThingAggregate".to_string()],
    }];
    let messages = protocol_messages(content, Some(ParseMetadata::TypeScript(meta)), "ts");
    assert!(
        !messages
            .iter()
            .any(|m| m.contains("implements a contract protocol")),
        "the aggregate interface is not a protocol: {messages:?}"
    );
}

#[test]
fn python_metadata_protocol_base_flagged() {
    // Python records base classes, so a protocol base shows up there.
    let content = "class Agent(IScannerProtocol):\n    pass\n";
    let mut meta = PythonMetadata::default();
    meta.class_declarations = vec![PythonClassItem {
        name: "Agent".to_string(),
        bases: vec!["IScannerProtocol".to_string()],
        decorators: vec![],
        body_fn_count: 0,
        module_fn_names: vec![],
    }];
    let file = FileEntry {
        path: PathBuf::from("src/agent_thing.py"),
        extension: "py".to_string(),
        language: Language::Python,
        size: content.len() as u64,
        content: content.to_string(),
        parse_ok: true,
        parse_metadata: Some(ParseMetadata::Python(meta)),
    };
    let mut v = Vec::new();
    AgentRustRoleAuditor::new().check_agent_protocol_forbidden(&file, &mut v);
    assert!(
        v.iter()
            .any(|r| r.message.value.contains("implements a contract protocol")),
        "python base naming IScannerProtocol must flag"
    );
}

#[test]
fn python_metadata_parameterized_protocol_base_flagged() {
    // `class Scanner(IScannerProtocol[T])` — the base carries type parameters.
    let content = "class Scanner(IScannerProtocol[T]):\n    pass\n";
    let mut meta = PythonMetadata::default();
    meta.class_declarations = vec![PythonClassItem {
        name: "Scanner".to_string(),
        bases: vec!["IScannerProtocol[T]".to_string()],
        decorators: vec![],
        body_fn_count: 0,
        module_fn_names: vec![],
    }];
    let file = FileEntry {
        path: PathBuf::from("src/agent_thing.py"),
        extension: "py".to_string(),
        language: Language::Python,
        size: content.len() as u64,
        content: content.to_string(),
        parse_ok: true,
        parse_metadata: Some(ParseMetadata::Python(meta)),
    };
    let mut v = Vec::new();
    AgentRustRoleAuditor::new().check_agent_protocol_forbidden(&file, &mut v);
    assert!(
        v.iter()
            .any(|r| r.message.value.contains("implements a contract protocol")),
        "a parameterized python base must still be recognised"
    );
}

// ── Line-scan fallback ───────────────────────────────────
//
// Reached whenever the file was not parsed (parse failed, or a caller builds a
// FileEntry by hand). The spellings below are the ones real code uses.

#[test]
fn fallback_rust_plain_impl_flagged() {
    let messages = protocol_messages(
        "impl IScannerProtocol for Agent {\n    fn scan(&self) {}\n}\n",
        None,
        "rs",
    );
    assert!(
        messages
            .iter()
            .any(|m| m.contains("implements a contract protocol")),
        "plain impl must flag: {messages:?}"
    );
}

#[test]
fn fallback_rust_generic_impl_flagged() {
    // `impl<T> IFooProtocol for X` — the generic parameter sits between the
    // `impl` keyword and the trait name.
    let messages = protocol_messages(
        "impl<T> IScannerProtocol for Agent {\n    fn scan(&self) {}\n}\n",
        None,
        "rs",
    );
    assert!(
        messages
            .iter()
            .any(|m| m.contains("implements a contract protocol")),
        "a generic impl head must still be recognised: {messages:?}"
    );
}

#[test]
fn fallback_rust_parameterized_protocol_flagged() {
    // `impl IFooProtocol<u32> for X` — the trait carries type parameters.
    let messages = protocol_messages(
        "impl IScannerProtocol<u32> for Agent {\n    fn scan(&self) {}\n}\n",
        None,
        "rs",
    );
    assert!(
        messages
            .iter()
            .any(|m| m.contains("implements a contract protocol")),
        "a parameterized protocol must still be recognised: {messages:?}"
    );
}

#[test]
fn fallback_ts_export_class_impl_flagged() {
    // `export class X implements IFoo` is the standard TypeScript declaration
    // form; a scanner that only accepts a bare `class ` keyword misses it.
    let messages = protocol_messages(
        "export class Agent implements IScannerProtocol {\n  scan() {}\n}\n",
        None,
        "ts",
    );
    assert!(
        messages
            .iter()
            .any(|m| m.contains("implements a contract protocol")),
        "the `export class` form must be recognised: {messages:?}"
    );
}

#[test]
fn fallback_ts_bare_class_impl_flagged() {
    let messages = protocol_messages(
        "class Agent implements IScannerProtocol {\n  scan() {}\n}\n",
        None,
        "ts",
    );
    assert!(
        messages
            .iter()
            .any(|m| m.contains("implements a contract protocol")),
        "the bare `class` form must be recognised: {messages:?}"
    );
}

#[test]
fn fallback_std_and_aggregate_impls_not_flagged() {
    let messages = protocol_messages(
        "impl Default for Agent {}\nimpl IThingAggregate for Agent {}\n",
        None,
        "rs",
    );
    assert!(
        !messages
            .iter()
            .any(|m| m.contains("implements a contract protocol")),
        "std traits and aggregates are not protocols: {messages:?}"
    );
}

// ── Block markers ────────────────────────────────────────
//
// A marker is a `─── Block <n>:` banner. The check is a ceiling on the number,
// not a validation of the sequence, and prose that merely contains the word
// "Block" must not read as a banner.

#[test]
fn block_four_flagged() {
    let messages = block_messages(
        "\
// ─── Block 1: Types ───
// ─── Block 2: Aggregate ───
// ─── Block 3: Helpers ───
// ─── Block 4: Extra ───
",
    );
    assert!(
        messages
            .iter()
            .any(|m| m.contains("block markers beyond Block 3")),
        "a Block 4 banner must flag: {messages:?}"
    );
}

#[test]
fn block_three_not_flagged() {
    let messages = block_messages(
        "\
// ─── Block 1: Types ───
// ─── Block 2: Aggregate ───
// ─── Block 3: Helpers ───
",
    );
    assert!(
        !messages
            .iter()
            .any(|m| m.contains("block markers beyond Block 3")),
        "three blocks is the ceiling: {messages:?}"
    );
}

#[test]
fn block_prose_without_colon_not_flagged() {
    let messages =
        block_messages("// Block 1 (types) -> Block 2 (aggregate) -> Block 3 (helpers).\n");
    assert!(
        !messages
            .iter()
            .any(|m| m.contains("block markers beyond Block 3")),
        "prose describing the structure is not a banner: {messages:?}"
    );
}

#[test]
fn block_sub_block_word_not_flagged() {
    // `Sub-Block 4:` contains "Block 4:" as a substring but is not a banner
    // heading — the word must start the token, not merely appear inside one.
    let messages = block_messages(
        "\
// ─── Block 1: Types ───
// ─── Block 2: Aggregate ───
// ─── Block 3: Helpers ───
// Sub-Block 4: prose about a sub-section
",
    );
    assert!(
        !messages
            .iter()
            .any(|m| m.contains("block markers beyond Block 3")),
        "`Sub-Block 4:` is prose, not a banner: {messages:?}"
    );
}

#[test]
fn ts_block_four_flagged() {
    let auditor = AgentTsRoleAuditor::new();
    let mut v = Vec::new();
    auditor.check_agent_block_markers(
        &ts_file(
            "// ─── Block 1: Types ───\n// ─── Block 2: Aggregate ───\n// ─── Block 4: Extra ───\n",
            None,
        ),
        &mut v,
    );
    assert!(!v.is_empty(), "a Block 4 banner in TS must flag");
}

// ── Fallback false positives ─────────────────────────────
//
// The line scan is a heuristic, so it only runs on lines that are declarations.
// A `class …` mentioned inside a string literal or a trailing comment is prose,
// and treating it as a declaration would report a protocol that was never
// implemented.

#[test]
fn fallback_ignores_class_inside_string_literal() {
    let messages = protocol_messages(
        "let doc = \"class Foo implements IScannerProtocol\";\n",
        None,
        "ts",
    );
    assert!(
        !messages
            .iter()
            .any(|m| m.contains("implements a contract protocol")),
        "a `class` in a string literal is not a declaration: {messages:?}"
    );
}

#[test]
fn fallback_ignores_class_after_trailing_comment() {
    let messages = protocol_messages(
        "// see also: class Foo implements IScannerProtocol\n",
        None,
        "ts",
    );
    assert!(
        !messages
            .iter()
            .any(|m| m.contains("implements a contract protocol")),
        "a `class` inside a comment is not a declaration: {messages:?}"
    );
}

#[test]
fn fallback_accepts_real_declaration_prefixes() {
    // The prefixes TypeScript actually writes must keep working after the
    // declaration-prefix restriction.
    for decl in [
        "class Agent implements IScannerProtocol {",
        "export class Agent implements IScannerProtocol {",
        "export default class Agent implements IScannerProtocol {",
        "export abstract class Agent implements IScannerProtocol {",
        "abstract class Agent implements IScannerProtocol {",
    ] {
        let messages = protocol_messages(&format!("{decl}\n  scan() {{}}\n}}\n"), None, "ts");
        assert!(
            messages
                .iter()
                .any(|m| m.contains("implements a contract protocol")),
            "`{decl}` is a declaration and must flag: {messages:?}"
        );
    }
}

#[test]
fn block_marker_scan_survives_multibyte_punctuation() {
    // The banner scan must not slice mid-character: a comment using fullwidth
    // punctuation after `Block` is legal text, and reading its length in bytes
    // would panic on the character boundary.
    let messages = block_messages("// ─── Block：4 ───\n");
    assert!(
        messages.is_empty(),
        "no banner here (no colon after the number), and no panic: {messages:?}"
    );
    let ascii = block_messages("// ─── Block 4: ───\n");
    assert!(
        ascii
            .iter()
            .any(|m| m.contains("block markers beyond Block 3")),
        "the ASCII banner still flags: {ascii:?}"
    );
}
