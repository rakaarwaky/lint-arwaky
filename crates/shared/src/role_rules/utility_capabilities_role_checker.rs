// PURPOSE: Shared AES403 sub-checks for capability role auditors.
//          Type budget and implementor checks use ParseMetadata when available
//          and fall back to line scanning. Per-language files delegate here
//          so one implementation is shared across Rust / Python / TypeScript.

use crate::common::taxonomy_lint_result_vo::LintResult;
use crate::common::taxonomy_severity_vo::Severity;
use crate::filesystem::taxonomy_filesystem_vo::{FileEntry, ParseMetadata};

/// True when `trait_name` names a contract protocol trait.
///
/// A capability implements a contract protocol, never an aggregate — aggregate
/// traits belong to the agent layer (AES405).
pub fn is_protocol_trait(trait_name: &str) -> bool {
    let base = trait_name.rsplit("::").next().unwrap_or(trait_name).trim();
    base.starts_with('I') && base.ends_with("Protocol") && !base.ends_with("Aggregate")
}

/// True when a Python base class names a contract protocol.
pub fn is_protocol_base(base: &str) -> bool {
    let base = base.split('[').next().unwrap_or(base).trim();
    is_protocol_trait(base.rsplit('.').next().unwrap_or(base))
}

/// Number of type declarations and whether one implements a protocol.
///
/// Returns `None` when the metadata variant has no capability shape.
fn profile(meta: &ParseMetadata) -> Option<(usize, bool, &'static str, &'static str)> {
    match meta {
        ParseMetadata::Rust(r) => {
            let structs: Vec<&str> = r.struct_definitions.iter().map(|s| s.as_str()).collect();
            let count = r.struct_definitions.len() + r.enum_definitions.len();
            let has_impl = r.impl_blocks.iter().any(|imp| {
                imp.trait_name
                    .as_ref()
                    .is_some_and(|t| is_protocol_trait(t))
                    && structs.contains(&imp.implementor_type.as_str())
            });
            Some((
                count,
                has_impl,
                "struct + enum",
                "No `impl I<Name>Protocol for <Struct>` found in this file.",
            ))
        }
        ParseMetadata::Python(py) => {
            let count = py.class_declarations.len();
            let has_impl = py
                .class_declarations
                .iter()
                .any(|c| c.bases.iter().any(|b| is_protocol_base(b)));
            Some((
                count,
                has_impl,
                "class",
                "No class inherits an `I<Name>Protocol` ABC in this file.",
            ))
        }
        ParseMetadata::TypeScript(ts) | ParseMetadata::JavaScript(ts) => {
            let count = ts.class_declarations.len()
                + ts.interface_declarations.len()
                + ts.type_alias_declarations.len();
            let has_impl = ts
                .class_declarations
                .iter()
                .any(|c| c.implements.iter().any(|i| is_protocol_trait(i)));
            Some((
                count,
                has_impl,
                "class / interface / type",
                "No class implements an `I<Name>Protocol` interface in this file.",
            ))
        }
        _ => None,
    }
}

/// Rule 1 — at most 3 type declarations per capability file. HIGH.
pub fn check_type_budget(file: &FileEntry, violations: &mut Vec<LintResult>) {
    let path = file.path.to_string_lossy().to_string();
    let Some((count, _, kind, _)) = file.parse_metadata.as_ref().and_then(profile) else {
        let count = scan_type_declarations(&file.content);
        if count > 3 {
            violations.push(LintResult::new_arch(
                &path,
                0,
                "AES403",
                Severity::HIGH,
                format!(
                    "AES403 CAPABILITY_ROLE: Capability declares too many types.\n\
                     WHY? Found {count} type declarations, max 3 allowed.\n\
                     HOW TO FIX? Keep at most 3 types in a capability file. \
                     Move the excess to the taxonomy layer or split across capability files."
                ),
            ));
        }
        return;
    };
    if count > 3 {
        violations.push(LintResult::new_arch(
            &path,
            0,
            "AES403",
            Severity::HIGH,
            format!(
                "AES403 CAPABILITY_ROLE: Capability declares too many types.\n\
                 WHY? Found {count} {kind} declarations, max 3 allowed.\n\
                 HOW TO FIX? Keep at most 3 types in a capability file. \
                 Move the excess to the taxonomy layer or split across capability files."
            ),
        ));
    }
}

/// Rule 2 — at least one type implements a contract protocol. MEDIUM.
pub fn check_implementor(file: &FileEntry, violations: &mut Vec<LintResult>) {
    let path = file.path.to_string_lossy().to_string();
    let found = match file.parse_metadata.as_ref().and_then(profile) {
        Some((_, has_impl, _, why)) => {
            if has_impl {
                return;
            }
            why.to_string()
        }
        None => {
            if scan_protocol_impl(&file.content) {
                return;
            }
            "No `impl I<Name>Protocol for <Struct>` pattern found.".to_string()
        }
    };
    violations.push(LintResult::new_arch(
        &path,
        0,
        "AES403",
        Severity::MEDIUM,
        format!(
            "AES403 CAPABILITY_ROLE: Capability implements no contract protocol.\n\
             WHY? {found} A capability must implement a protocol contract; \
             an aggregate trait belongs to the agent layer (AES405).\n\
             HOW TO FIX? Create the contract with the `aes-contract` skill, \
             then add the protocol implementation as Block 2 of this file."
        ),
    ));
}

/// Count type declarations by line scan, for files with no parse metadata.
fn scan_type_declarations(content: &str) -> usize {
    let mut count = 0;
    for line in content.lines() {
        let t = line.trim();
        if t.starts_with("struct ")
            || t.starts_with("pub struct ")
            || t.starts_with("enum ")
            || t.starts_with("pub enum ")
            || t.starts_with("class ")
            || t.starts_with("pub class ")
            || t.starts_with("interface ")
            || t.starts_with("pub interface ")
        {
            count += 1;
        }
    }
    count
}

/// True when a line-scan finds a protocol implementation.
fn scan_protocol_impl(content: &str) -> bool {
    content.lines().any(|line| {
        let t = line.trim();
        if let Some(rest) = t.strip_prefix("impl ") {
            return rest.contains("Protocol for")
                && is_protocol_trait(rest.split(" for ").next().unwrap_or("").trim());
        }
        if t.starts_with("class ") {
            // `class Foo(IProtocol)`, `class Foo(Mixin, IProtocol)`, and
            // `class Foo(Base, IProtocolGeneric[T])` all count. The line ends
            // in `):` for a `class` statement, so trim that before slicing
            // the base-class argument list out.
            let decl = t.trim_end().trim_end_matches(':');
            if let Some(open) = decl.find('(') {
                if decl.ends_with(')') {
                    return decl[open + 1..decl.len() - 1]
                        .split(',')
                        .any(is_protocol_base);
                }
            }
            return false;
        }
        t.contains("implements I") && t.contains("Protocol")
    })
}
