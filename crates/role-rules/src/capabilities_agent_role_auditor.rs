// PURPOSE: AgentRoleChecker — AES405: enforce agent type composition
//
// ALGORITHM (uses ParseMetadata when available):
//   1. Collect all type declarations (struct/enum/class/interface).
//   2. Rule 1 (FRD) — Must have >= 1 aggregate implementor.
//   3. Rule 2 (FRD) — Max 3 type declarations per file.
//   Internal helper types without implementor pattern are ALLOWED.

use shared::common::LintResult;
use shared::common::Severity;
use shared::common::taxonomy_message_vo::LintMessage;
use shared::common::taxonomy_name_vo::SymbolName;
use shared::filesystem::taxonomy_filesystem_vo::{FileEntry, ParseMetadata};
use shared::role_rules::{AesRoleViolation, IAgentRoleChecker};

// ─── Block 1: Struct Definition ───────────────────────────
pub struct AgentRoleChecker {}

// ─── Block 2: Protocol Trait Implementation ───────────────
impl IAgentRoleChecker for AgentRoleChecker {
    fn check_agent_routing(&self, file: &FileEntry, layer: &str, violations: &mut Vec<LintResult>) {
        if layer != "agent" && !layer.starts_with("agent(") {
            return;
        }

        if let Some(meta) = &file.parse_metadata {
            self._check_with_metadata(file, meta, violations);
        } else {
            self._check_fallback(file, violations);
        }
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────
impl Default for AgentRoleChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl AgentRoleChecker {
    pub fn new() -> Self {
        Self {}
    }

    fn _check_with_metadata(
        &self,
        file: &FileEntry,
        meta: &ParseMetadata,
        violations: &mut Vec<LintResult>,
    ) {
        let path_str = file.path.to_string_lossy();
        match meta {
            ParseMetadata::Rust(rust_meta) => {
                let type_count =
                    rust_meta.struct_definitions.len() + rust_meta.enum_definitions.len();
                let struct_names: Vec<&str> = rust_meta
                    .struct_definitions
                    .iter()
                    .map(|s| s.as_str())
                    .collect();

                // Rule 2: max 3 types (checked first per FRD)
                if type_count > 3 {
                    let all_names: Vec<String> = rust_meta
                        .struct_definitions
                        .iter()
                        .chain(rust_meta.enum_definitions.iter())
                        .cloned()
                        .collect();
                    let names_str = all_names.join(", ");
                    violations.push(LintResult::new_arch(
                        &path_str,
                        0,
                        "AES405",
                        Severity::HIGH,
                        AesRoleViolation::AgentTooManyTypes {
                            count: type_count,
                            names: all_names.iter().map(SymbolName::new).collect(),
                            reason: Some(LintMessage::new(format!(
                                "Found {} types (struct/enum) in {}, max 3 allowed: [{}]",
                                type_count, path_str, names_str
                            ))),
                        },
                    ));
                    return;
                }

                // Rule 1: at least 1 aggregate implementor
                let has_implementor = rust_meta.impl_blocks.iter().any(|imp| {
                    imp.trait_name.is_some()
                        && struct_names.contains(&imp.implementor_type.as_str())
                });
                if !has_implementor {
                    violations.push(LintResult::new_arch(
                        &path_str, 0, "AES405", Severity::MEDIUM,
                        AesRoleViolation::AgentNoImplementor {
                            reason: Some(LintMessage::new(format!(
                                "No impl Trait for struct pattern found in {}. At least one struct must implement an aggregate trait.",
                                path_str
                            ))),
                        },
                    ));
                }
            }
            ParseMetadata::Python(py_meta) => {
                let type_count = py_meta.class_declarations.len();
                let implementor_found = py_meta
                    .class_declarations
                    .iter()
                    .any(|c| !c.bases.is_empty());

                if type_count > 3 {
                    let names: Vec<String> = py_meta
                        .class_declarations
                        .iter()
                        .map(|c| c.name.clone())
                        .collect();
                    let names_str = names.join(", ");
                    violations.push(LintResult::new_arch(
                        &path_str,
                        0,
                        "AES405",
                        Severity::HIGH,
                        AesRoleViolation::AgentTooManyTypes {
                            count: type_count,
                            names: names.iter().map(SymbolName::new).collect(),
                            reason: Some(LintMessage::new(format!(
                                "Found {} classes in {}, max 3 allowed: [{}]",
                                type_count, path_str, names_str
                            ))),
                        },
                    ));
                    return;
                }
                if !implementor_found {
                    violations.push(LintResult::new_arch(
                        &path_str, 0, "AES405", Severity::MEDIUM,
                        AesRoleViolation::AgentNoImplementor {
                            reason: Some(LintMessage::new(format!(
                                "No class with parent/inheritance found in {}. At least one class must inherit from a parent class.",
                                path_str
                            ))),
                        },
                    ));
                }
            }
            ParseMetadata::TypeScript(ts_meta) | ParseMetadata::JavaScript(ts_meta) => {
                let type_count = ts_meta.class_declarations.len()
                    + ts_meta.interface_declarations.len()
                    + ts_meta.type_alias_declarations.len();
                let implementor_found = ts_meta
                    .class_declarations
                    .iter()
                    .any(|c| !c.implements.is_empty());

                if type_count > 3 {
                    let mut all_names: Vec<String> = ts_meta
                        .class_declarations
                        .iter()
                        .map(|c| c.name.clone())
                        .collect();
                    all_names.extend(ts_meta.interface_declarations.iter().cloned());
                    all_names.extend(ts_meta.type_alias_declarations.iter().cloned());
                    let names_str = all_names.join(", ");
                    violations.push(LintResult::new_arch(
                        &path_str,
                        0,
                        "AES405",
                        Severity::HIGH,
                        AesRoleViolation::AgentTooManyTypes {
                            count: type_count,
                            names: all_names.iter().map(SymbolName::new).collect(),
                            reason: Some(LintMessage::new(format!(
                                "Found {} types (class/interface/enum) in {}, max 3 allowed: [{}]",
                                type_count, path_str, names_str
                            ))),
                        },
                    ));
                    return;
                }
                if !implementor_found {
                    violations.push(LintResult::new_arch(
                        &path_str, 0, "AES405", Severity::MEDIUM,
                        AesRoleViolation::AgentNoImplementor {
                            reason: Some(LintMessage::new(format!(
                                "No class with 'implements' keyword found in {}. At least one class must implement an aggregate interface.",
                                path_str
                            ))),
                        },
                    ));
                }
            }
            _ => {} // ParseMetadata::Unknown — skip
        }
    }

    fn _check_fallback(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        let path_str = file.path.to_string_lossy();
        let content = &file.content;
        let lines: Vec<&str> = content.lines().collect();
        let mut type_names: Vec<&str> = Vec::new();
        let mut struct_names: Vec<&str> = Vec::new();
        let mut implementor_found = false;

        match file.language {
            shared::filesystem::taxonomy_filesystem_vo::Language::Rust => {
                let mut in_cfg_test = false;
                for l in &lines {
                    let t = l.trim();
                    if t.starts_with("#[cfg(test)]") {
                        in_cfg_test = true;
                        continue;
                    }
                    if in_cfg_test {
                        if t.starts_with('}') {
                            in_cfg_test = false;
                        }
                        continue;
                    }
                    let words: Vec<&str> = t.split_whitespace().collect();
                    if (t.starts_with("pub struct ") || t.starts_with("struct "))
                        && words.len() >= 2
                        && let Some(idx) = words.iter().position(|w| *w == "struct")
                        && let Some(name) = words.get(idx + 1)
                    {
                        let name = name.trim_end_matches(';').trim_end_matches('{');
                        if !name.is_empty() && !name.starts_with('_') {
                            type_names.push(name);
                            struct_names.push(name);
                        }
                    }
                    if (t.starts_with("pub enum ") || t.starts_with("enum "))
                        && words.len() >= 2
                        && let Some(idx) = words.iter().position(|w| *w == "enum")
                        && let Some(name) = words.get(idx + 1)
                    {
                        let name = name.trim_end_matches(';').trim_end_matches('{');
                        if !name.is_empty() && !name.starts_with('_') {
                            type_names.push(name);
                        }
                    }
                }
                if type_names.len() > 3 {
                    let names_str = type_names.join(", ");
                    violations.push(LintResult::new_arch(
                        &path_str,
                        0,
                        "AES405",
                        Severity::HIGH,
                        AesRoleViolation::AgentTooManyTypes {
                            count: type_names.len(),
                            names: type_names.iter().map(|s| SymbolName::new(*s)).collect(),
                            reason: Some(LintMessage::new(format!(
                                "Found {} types in {}, max 3 allowed: [{}]",
                                type_names.len(),
                                path_str,
                                names_str
                            ))),
                        },
                    ));
                    return;
                }
                let has_implementor = struct_names.iter().any(|s| {
                    content.contains("impl ")
                        && (content.contains(&format!("for {} ", s))
                            || content.contains(&format!("for {}{{", s))
                            || content.contains(&format!("for {} {{", s)))
                });
                if !has_implementor {
                    violations.push(LintResult::new_arch(
                        &path_str, 0, "AES405", Severity::MEDIUM,
                        AesRoleViolation::AgentNoImplementor {
                            reason: Some(LintMessage::new(format!("No impl Trait for struct pattern found in {}. At least one struct must implement an aggregate trait.", path_str))),
                        },
                    ));
                }
            }
            shared::filesystem::taxonomy_filesystem_vo::Language::Python => {
                for l in &lines {
                    let t = l.trim();
                    if let Some(after_class) = t.strip_prefix("class ") {
                        let name = after_class
                            .split(['(', ':', ' '])
                            .next()
                            .unwrap_or("")
                            .trim();
                        if !name.is_empty() && !name.starts_with('_') {
                            type_names.push(name);
                        }
                        if let Some(start) = t.find('(') {
                            let after_paren = &t[start + 1..];
                            if let Some(end) = after_paren.find(')')
                                && !after_paren[..end].trim().is_empty()
                            {
                                implementor_found = true;
                            }
                        }
                    }
                }
                if type_names.len() > 3 {
                    let names_str = type_names.join(", ");
                    violations.push(LintResult::new_arch(
                        &path_str,
                        0,
                        "AES405",
                        Severity::HIGH,
                        AesRoleViolation::AgentTooManyTypes {
                            count: type_names.len(),
                            names: type_names.iter().map(|s| SymbolName::new(*s)).collect(),
                            reason: Some(LintMessage::new(format!(
                                "Found {} classes in {}, max 3 allowed: [{}]",
                                type_names.len(),
                                path_str,
                                names_str
                            ))),
                        },
                    ));
                    return;
                }
                if !implementor_found {
                    violations.push(LintResult::new_arch(
                        &path_str,
                        0,
                        "AES405",
                        Severity::MEDIUM,
                        AesRoleViolation::AgentNoImplementor {
                            reason: Some(LintMessage::new(format!(
                                "No class with parent/inheritance found in {}.",
                                path_str
                            ))),
                        },
                    ));
                }
            }
            _ => {
                for l in &lines {
                    let t = l.trim();
                    if let Some(rest) = t
                        .strip_prefix("export class ")
                        .or_else(|| t.strip_prefix("class "))
                    {
                        let name = rest.split([' ', '(', '{']).next().unwrap_or("").trim();
                        if !name.is_empty() && !name.starts_with('_') {
                            type_names.push(name);
                        }
                        if rest.contains("implements ") {
                            implementor_found = true;
                        }
                    } else if let Some(rest) = t
                        .strip_prefix("export interface ")
                        .or_else(|| t.strip_prefix("interface "))
                    {
                        let name = rest.split([' ', '{', '<']).next().unwrap_or("").trim();
                        if !name.is_empty() && !name.starts_with('_') {
                            type_names.push(name);
                        }
                    } else if let Some(rest) = t
                        .strip_prefix("export enum ")
                        .or_else(|| t.strip_prefix("enum "))
                    {
                        let name = rest.split([' ', '{']).next().unwrap_or("").trim();
                        if !name.is_empty() && !name.starts_with('_') {
                            type_names.push(name);
                        }
                    }
                }
                if type_names.len() > 3 {
                    let names_str = type_names.join(", ");
                    violations.push(LintResult::new_arch(
                        &path_str,
                        0,
                        "AES405",
                        Severity::HIGH,
                        AesRoleViolation::AgentTooManyTypes {
                            count: type_names.len(),
                            names: type_names.iter().map(|s| SymbolName::new(*s)).collect(),
                            reason: Some(LintMessage::new(format!(
                                "Found {} types in {}, max 3 allowed: [{}]",
                                type_names.len(),
                                path_str,
                                names_str
                            ))),
                        },
                    ));
                    return;
                }
                if !implementor_found {
                    violations.push(LintResult::new_arch(
                        &path_str,
                        0,
                        "AES405",
                        Severity::MEDIUM,
                        AesRoleViolation::AgentNoImplementor {
                            reason: Some(LintMessage::new(format!(
                                "No class with 'implements' found in {}.",
                                path_str
                            ))),
                        },
                    ));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::filesystem::taxonomy_filesystem_vo::{Language, ParseMetadata, RustMetadata};
    use std::path::PathBuf;

    fn checker() -> AgentRoleChecker {
        AgentRoleChecker::new()
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

    fn make_file_with_rust_meta(path: &str, meta: RustMetadata) -> FileEntry {
        FileEntry {
            path: PathBuf::from(path),
            extension: "rs".to_string(),
            language: Language::Rust,
            size: 100,
            content: String::new(),
            parse_ok: true,
            parse_metadata: Some(ParseMetadata::Rust(meta)),
        }
    }

    #[test]
    fn construction_succeeds() {
        let _ = checker();
    }

    #[test]
    fn non_agent_layer_skipped() {
        let f = make_file(
            "src/capabilities_foo.rs",
            Language::Rust,
            "pub struct Foo {}",
        );
        let mut v = Vec::new();
        checker().check_agent_routing(&f, "capabilities", &mut v);
        assert!(v.is_empty(), "non-agent layer must not produce violations");
    }

    #[test]
    fn agent_layer_with_agent_prefix_routes() {
        // "agent(capabilities)" starts with "agent(" so it IS routed to the agent checker
        let f = make_file(
            "src/capabilities_foo.rs",
            Language::Rust,
            "pub struct Foo {}",
        );
        let mut v = Vec::new();
        checker().check_agent_routing(&f, "agent(capabilities)", &mut v);
        // Single struct without impl — should be flagged
        assert!(
            !v.is_empty(),
            "agent(capabilities) should route to agent checker"
        );
    }

    #[test]
    fn fallback_rust_no_implementor_flagged() {
        let content = "pub struct Foo {}\npub struct Bar {}\n";
        let f = make_file("src/agent_something.rs", Language::Rust, content);
        let mut v = Vec::new();
        checker().check_agent_routing(&f, "agent", &mut v);
        assert!(!v.is_empty(), "should flag missing implementor");
        assert_eq!(v[0].code.code(), "AES405");
    }

    #[test]
    fn fallback_rust_too_many_types_flagged() {
        let content = "pub struct A {}\npub struct B {}\npub struct C {}\npub struct D {}\n";
        let f = make_file("src/agent_something.rs", Language::Rust, content);
        let mut v = Vec::new();
        checker().check_agent_routing(&f, "agent", &mut v);
        assert!(!v.is_empty(), "should flag too many types");
        assert_eq!(v[0].code.code(), "AES405");
        // Should be HIGH severity for too-many-types
        assert_eq!(v[0].severity, Severity::HIGH);
    }

    #[test]
    fn fallback_rust_valid_composition_no_violation() {
        let content = "pub struct Foo {}\nimpl IFoo for Foo {}\n";
        let f = make_file("src/agent_something.rs", Language::Rust, content);
        let mut v = Vec::new();
        checker().check_agent_routing(&f, "agent", &mut v);
        assert!(v.is_empty(), "valid agent composition should pass");
    }

    #[test]
    fn metadata_rust_no_implementor_flagged() {
        let meta = RustMetadata {
            struct_definitions: vec!["Foo".into(), "Bar".into()],
            ..Default::default()
        };
        let f = make_file_with_rust_meta("src/agent_something.rs", meta);
        let mut v = Vec::new();
        checker().check_agent_routing(&f, "agent", &mut v);
        assert!(
            !v.is_empty(),
            "should flag missing implementor via metadata"
        );
        assert_eq!(v[0].code.code(), "AES405");
    }

    #[test]
    fn metadata_rust_too_many_types_flagged() {
        let meta = RustMetadata {
            struct_definitions: vec!["A".into(), "B".into()],
            enum_definitions: vec!["C".into(), "D".into()],
            ..Default::default()
        };
        let f = make_file_with_rust_meta("src/agent_something.rs", meta);
        let mut v = Vec::new();
        checker().check_agent_routing(&f, "agent", &mut v);
        assert!(!v.is_empty(), "should flag too many types via metadata");
        assert_eq!(v[0].severity, Severity::HIGH);
    }

    #[test]
    fn metadata_rust_valid_composition_no_violation() {
        let meta = RustMetadata {
            struct_definitions: vec!["Foo".into()],
            impl_blocks: vec![shared::filesystem::taxonomy_filesystem_vo::RustImplItem {
                trait_name: Some("IFoo".into()),
                trait_path: None,
                implementor_type: "Foo".into(),
                has_generics: false,
            }],
            ..Default::default()
        };
        let f = make_file_with_rust_meta("src/agent_something.rs", meta);
        let mut v = Vec::new();
        checker().check_agent_routing(&f, "agent", &mut v);
        assert!(v.is_empty(), "valid metadata composition should pass");
    }
}
