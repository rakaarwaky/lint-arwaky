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

/// Count distinct protocol traits implemented in a capability file.
///
/// Returns `(count, Vec<trait_name>)` — `count == 1` means single-protocol
/// (no violation); `count == 0` is handled by `check_implementor`;
/// `count > 1` triggers `CapabilityMultiProtocol` (AES403 MEDIUM).
pub fn count_protocol_traits(file: &FileEntry) -> (usize, Vec<String>) {
    if let Some(meta) = &file.parse_metadata {
        return match meta {
            ParseMetadata::Rust(r) => {
                let structs: Vec<&str> = r.struct_definitions.iter().map(|s| s.as_str()).collect();
                let mut protocols: Vec<String> = Vec::new();
                for imp in &r.impl_blocks {
                    if let Some(trait_name) = &imp.trait_name {
                        if is_protocol_trait(trait_name)
                            && structs.contains(&imp.implementor_type.as_str())
                            && !protocols.contains(&trait_name.to_lowercase())
                        {
                            protocols.push(trait_name.clone());
                        }
                    }
                }
                (protocols.len(), protocols)
            }
            ParseMetadata::Python(py) => {
                let mut protocols: Vec<String> = Vec::new();
                for c in &py.class_declarations {
                    for base in &c.bases {
                        if is_protocol_base(base) && !protocols.contains(&base.to_lowercase()) {
                            protocols.push(base.clone());
                        }
                    }
                }
                (protocols.len(), protocols)
            }
            ParseMetadata::TypeScript(ts) | ParseMetadata::JavaScript(ts) => {
                let mut protocols: Vec<String> = Vec::new();
                for c in &ts.class_declarations {
                    for iface in &c.implements {
                        if is_protocol_trait(iface) && !protocols.contains(&iface.to_lowercase()) {
                            protocols.push(iface.clone());
                        }
                    }
                }
                (protocols.len(), protocols)
            }
            _ => (0, Vec::new()),
        };
    }
    // Fallback line-scan when parse metadata is unavailable.
    let protocols = scan_protocol_names(&file.content);
    (protocols.len(), protocols)
}

/// Rule 3 — exactly one contract protocol per capability file. MEDIUM.
pub fn check_single_protocol(file: &FileEntry, violations: &mut Vec<LintResult>) {
    let path = file.path.to_string_lossy().to_string();
    let (count, protocols) = count_protocol_traits(file);
    if count <= 1 {
        return;
    }
    let trait_names: Vec<&str> = protocols.iter().map(|s| s.as_str()).collect();
    violations.push(LintResult::new_arch(
        &path,
        0,
        "AES403",
        Severity::MEDIUM,
        format!(
            "AES403 CAPABILITY_ROLE: Capability file implements multiple protocols.\n\
             WHY? {count} protocol traits found: {names}.\n\
             HOW TO FIX? Split this file into separate capability files, one per protocol \
             (e.g. `capabilities_foo_handler.rs`, `capabilities_bar_handler.rs`). \
             If the capabilities share common helper functions, move those shared functions \
             to a `utility_<shared>_resolver.*` file and import it from both capabilities.",
            names = trait_names.join(", ")
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

/// Collect distinct protocol trait names by line scan, for files with no parse metadata.
///
/// Supports Rust (`impl IXxxProtocol for Yyy`), Python
/// (`class X(IAProtocol, IBProtocol)`), and TypeScript
/// (`class X implements IAProtocol, IBProtocol`).
fn scan_protocol_names(content: &str) -> Vec<String> {
    let mut protocols: Vec<String> = Vec::new();

    for line in content.lines() {
        let t = line.trim();

        // Rust: `impl IXxxProtocol for Yyy`
        if let Some(rest) = t.strip_prefix("impl ") {
            if let Some(trait_part) = rest.split(" for ").next() {
                let name = trait_part.trim();
                if is_protocol_trait(name) && !protocols.iter().any(|p| p == name) {
                    protocols.push(name.to_string());
                }
            }
        }

        // Python: `class X(IAProtocol, IBProtocol):`
        // TypeScript: `class X implements IAProtocol, IBProtocol {`
        // Both may be prefixed with `export `.
        let class_body = t
            .strip_prefix("export class ")
            .or_else(|| t.strip_prefix("class "))
            .unwrap_or("");
        if !class_body.is_empty() {
            // `class DualService(IFoo, IBar):` → split at '(' to get name and bases.
            // `class DualService implements IFoo, IBar {` → split at '(' gives no '(',
            // so fall through to the `implements` path.
            let (type_part, rest) = class_body.split_once('(').unwrap_or((class_body, ""));
            let type_part = type_part.trim();

            if !rest.is_empty() {
                // Python-style bases: everything inside `(` ... `)`.
                // Strip trailing `)` and `:` in order; `trim_end_matches`
                // consumes chars from the right one at a time.
                let bases_str = rest.trim_end_matches(&[')', ':', ' '] as &[char]).trim();
                for base in bases_str.split(',') {
                    let b = base.trim();
                    if is_protocol_base(b) && !protocols.iter().any(|p| p == b) {
                        protocols.push(b.to_string());
                    }
                }
            } else if let Some((_, after)) = type_part.split_once(" implements ") {
                // TypeScript-style: `class Foo implements IFoo, IBar {`
                // `after` is `IFoo, IBar {` or `IFoo, IBar`; strip braces from both sides.
                let iface_list =
                    after.trim_matches(|c: char| c == '{' || c == '}' || c.is_whitespace());
                for iface in iface_list.split(',') {
                    let i = iface.trim();
                    if is_protocol_trait(i) && !protocols.iter().any(|p| p == i) {
                        protocols.push(i.to_string());
                    }
                }
            }
        }
    }

    protocols
}
