// PURPOSE: CapabilitiesRoleChecker — AES403: enforce capability type composition.
//
// ALGORITHM (uses ParseMetadata when available, falls back to line scanning):
//   1. Collect all type declarations (struct/class/enum/interface).
//   2. Rule 1 — Max 3 type declarations per file. If exceeded -> flag CapabilityTooManyTypes.
//      Checked first; if violated, skip implementor check.
//   3. Rule 2 — Must have >= 1 implementor:
//        Rust:   impl <Trait> for <Struct>
//        Python: class <Name>(<Parent>):
//        TS:     class <Name> implements <IProtocol>
//      If none -> flag CapabilityNoImplementor.
//   4. Internal helper types (without implementor pattern) are ALLOWED and not flagged.
//
// NOTE: Import checking is handled by import-rules crate, not role-rules.

use shared::common::LintResult;
use shared::common::Severity;
use shared::common::taxonomy_message_vo::LintMessage;
use shared::filesystem::taxonomy_filesystem_vo::{FileEntry, ParseMetadata};
use shared::role_rules::{AesRoleViolation, ICapabilitiesRoleChecker};

// ─── Block 1: Struct Definition ───────────────────────────
pub struct CapabilitiesRoleChecker {}

// ─── Block 2: Protocol Trait Implementation ───────────────
impl ICapabilitiesRoleChecker for CapabilitiesRoleChecker {
    fn check_capability_routing(
        &self,
        file: &FileEntry,
        layer: &str,
        violations: &mut Vec<LintResult>,
    ) {
        if layer != "capabilities" && !layer.starts_with("capabilities(") {
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
impl Default for CapabilitiesRoleChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl CapabilitiesRoleChecker {
    pub fn new() -> Self {
        Self {}
    }

    /// Check using structured ParseMetadata.
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

                // Rule 1: max 3 types
                if type_count > 3 {
                    violations.push(LintResult::new_arch(
                        &path_str,
                        0,
                        "AES403",
                        Severity::HIGH,
                        AesRoleViolation::CapabilityTooManyTypes {
                            count: type_count,
                            reason: Some(LintMessage::new(format!(
                                "Found {} types (struct + enum), max 3 allowed",
                                type_count
                            ))),
                        },
                    ));
                    return;
                }

                // Rule 2: must have >= 1 struct implementor
                let has_implementor = rust_meta.impl_blocks.iter().any(|imp| {
                    imp.trait_name.is_some()
                        && struct_names.contains(&imp.implementor_type.as_str())
                });
                if !has_implementor {
                    violations.push(LintResult::new_arch(
                        &path_str, 0, "AES403", Severity::MEDIUM,
                        AesRoleViolation::CapabilityNoImplementor {
                            reason: Some(LintMessage::new(format!(
                                "No impl Trait for struct pattern found in {}. At least one struct must implement a _protocol trait.",
                                path_str
                            ))),
                        },
                    ));
                }
            }
            ParseMetadata::Python(py_meta) => {
                let class_count = py_meta.class_declarations.len();
                let implementor_found = py_meta
                    .class_declarations
                    .iter()
                    .any(|c| !c.bases.is_empty());

                if class_count > 3 {
                    violations.push(LintResult::new_arch(
                        &path_str,
                        0,
                        "AES403",
                        Severity::HIGH,
                        AesRoleViolation::CapabilityTooManyTypes {
                            count: class_count,
                            reason: Some(LintMessage::new(format!(
                                "Found {} classes, max 3 allowed",
                                class_count
                            ))),
                        },
                    ));
                    return;
                }
                if !implementor_found {
                    violations.push(LintResult::new_arch(
                        &path_str, 0, "AES403", Severity::MEDIUM,
                        AesRoleViolation::CapabilityNoImplementor {
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
                    violations.push(LintResult::new_arch(
                        &path_str,
                        0,
                        "AES403",
                        Severity::HIGH,
                        AesRoleViolation::CapabilityTooManyTypes {
                            count: type_count,
                            reason: Some(LintMessage::new(format!(
                                "Found {} types (class/interface/enum), max 3 allowed",
                                type_count
                            ))),
                        },
                    ));
                    return;
                }
                if !implementor_found {
                    violations.push(LintResult::new_arch(
                        &path_str, 0, "AES403", Severity::MEDIUM,
                        AesRoleViolation::CapabilityNoImplementor {
                            reason: Some(LintMessage::new(format!(
                                "No class with 'implements' keyword found in {}. At least one class must implement an interface/protocol.",
                                path_str
                            ))),
                        },
                    ));
                }
            }
            _ => {} // ParseMetadata::Unknown — skip
        }
    }

    /// Fallback: line-based scanning when no parse metadata.
    fn _check_fallback(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        let path_str = file.path.to_string_lossy();
        let content = &file.content;
        let lines: Vec<&str> = content.lines().collect();
        let mut type_count: usize = 0;
        let mut implementor_found = false;
        let mut struct_names: Vec<&str> = Vec::new();

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
                            type_count += 1;
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
                            type_count += 1;
                        }
                    }
                }
                if type_count > 3 {
                    violations.push(LintResult::new_arch(
                        &path_str,
                        0,
                        "AES403",
                        Severity::HIGH,
                        AesRoleViolation::CapabilityTooManyTypes {
                            count: type_count,
                            reason: Some(LintMessage::new(format!(
                                "Found {} types (struct + enum), max 3 allowed",
                                type_count
                            ))),
                        },
                    ));
                    return;
                }
                let has_implementor = struct_names.iter().any(|s| {
                    lines.iter().any(|l| {
                        let t = l.trim();
                        t.starts_with("impl ")
                            && (t.contains(&format!("for {} ", s))
                                || t.contains(&format!("for {}{{", s))
                                || t.contains(&format!("for {} {{", s)))
                    })
                });
                if !has_implementor {
                    violations.push(LintResult::new_arch(
                        &path_str, 0, "AES403", Severity::MEDIUM,
                        AesRoleViolation::CapabilityNoImplementor {
                            reason: Some(LintMessage::new(format!(
                                "No impl Trait for struct pattern found in {}. At least one struct must implement a _protocol trait.", path_str
                            ))),
                        },
                    ));
                }
            }
            shared::filesystem::taxonomy_filesystem_vo::Language::Python => {
                for l in &lines {
                    let t = l.trim();
                    if t.starts_with("class ") {
                        type_count += 1;
                        if let Some(start) = t.find('(') {
                            let after_paren = &t[start + 1..];
                            if let Some(end) = after_paren.find(')') {
                                let parents = after_paren[..end].trim();
                                if !parents.is_empty() {
                                    implementor_found = true;
                                }
                            }
                        }
                    }
                }
                if type_count > 3 {
                    violations.push(LintResult::new_arch(
                        &path_str,
                        0,
                        "AES403",
                        Severity::HIGH,
                        AesRoleViolation::CapabilityTooManyTypes {
                            count: type_count,
                            reason: Some(LintMessage::new(format!(
                                "Found {} classes, max 3 allowed",
                                type_count
                            ))),
                        },
                    ));
                    return;
                }
                if !implementor_found {
                    violations.push(LintResult::new_arch(
                        &path_str, 0, "AES403", Severity::MEDIUM,
                        AesRoleViolation::CapabilityNoImplementor {
                            reason: Some(LintMessage::new(format!("No class with parent/inheritance found in {}. At least one class must inherit from a parent class.", path_str))),
                        },
                    ));
                }
            }
            _ => {
                for l in &lines {
                    let t = l.trim();
                    let class_body = t
                        .strip_prefix("export class ")
                        .or_else(|| t.strip_prefix("class "));
                    if let Some(rest) = class_body {
                        type_count += 1;
                        if rest.contains("implements ") {
                            implementor_found = true;
                        }
                        continue;
                    }
                    let iface_body = t
                        .strip_prefix("export interface ")
                        .or_else(|| t.strip_prefix("interface "));
                    if iface_body.is_some() {
                        type_count += 1;
                        continue;
                    }
                    let enum_body = t
                        .strip_prefix("export enum ")
                        .or_else(|| t.strip_prefix("enum "));
                    if enum_body.is_some() {
                        type_count += 1;
                    }
                }
                if type_count > 3 {
                    violations.push(LintResult::new_arch(
                        &path_str,
                        0,
                        "AES403",
                        Severity::HIGH,
                        AesRoleViolation::CapabilityTooManyTypes {
                            count: type_count,
                            reason: Some(LintMessage::new(format!(
                                "Found {} types, max 3 allowed",
                                type_count
                            ))),
                        },
                    ));
                    return;
                }
                if !implementor_found {
                    violations.push(LintResult::new_arch(
                        &path_str,
                        0,
                        "AES403",
                        Severity::MEDIUM,
                        AesRoleViolation::CapabilityNoImplementor {
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

    fn checker() -> CapabilitiesRoleChecker {
        CapabilitiesRoleChecker::new()
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
        let content = "pub struct Foo {}\nimpl IFoo for Foo {}\n";
        let f = make_file("src/capabilities_something.rs", Language::Rust, content);
        let mut v = Vec::new();
        checker().check_capability_routing(&f, "capabilities", &mut v);
        assert!(v.is_empty(), "valid capability composition should pass");
    }

    #[test]
    fn fallback_python_no_parent_flagged() {
        let content = "class Foo:\n    pass\n";
        let f = make_file("src/capabilities_something.py", Language::Python, content);
        let mut v = Vec::new();
        checker().check_capability_routing(&f, "capabilities", &mut v);
        assert!(
            !v.is_empty(),
            "python class without parent should be flagged"
        );
        assert_eq!(v[0].code.code(), "AES403");
    }

    #[test]
    fn fallback_python_with_parent_no_violation() {
        let content = "class Foo(Protocol):\n    pass\n";
        let f = make_file("src/capabilities_something.py", Language::Python, content);
        let mut v = Vec::new();
        checker().check_capability_routing(&f, "capabilities", &mut v);
        assert!(v.is_empty(), "python class with parent should pass");
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
            impl_blocks: vec![shared::filesystem::taxonomy_filesystem_vo::RustImplItem {
                trait_name: Some("IFoo".into()),
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
}
