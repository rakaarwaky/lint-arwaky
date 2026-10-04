// PURPOSE: Shared AES403 sub-checks for capability role auditors.
//          Type budget and implementor checks use ParseMetadata when available
//          and fall back to line scanning. Per-language files delegate here
//          so one implementation is shared across Rust / Python / TypeScript.

use std::collections::BTreeSet;

use shared_common::taxonomy_lint_result_vo::LintResult;
use shared_common::taxonomy_severity_vo::Severity;
use shared_filesystem::taxonomy_filesystem_vo::{FileEntry, ParseMetadata};

/// True when `layer` names the capabilities layer, as a bare name or a
/// parameterised variant such as `capabilities(feature_x)`.
pub fn is_capabilities_layer(layer: &str) -> bool {
    layer == "capabilities" || layer.starts_with("capabilities(")
}

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
                     FIX: Keep at most 3 types in a capability file. \
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
                 FIX: Keep at most 3 types in a capability file. \
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
             FIX: Create the contract with the `aes-contract` skill, \
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
             FIX: Split this file into separate capability files, one per protocol \
             (e.g. `capabilities_foo_handler.rs`, `capabilities_bar_handler.rs`). \
             If the capabilities share common helper functions, move those shared functions \
             to a `utility_<shared>_resolver.*` file and import it from both capabilities.",
            names = trait_names.join(", ")
        ),
    ));
}

/// Rule 4 — the capability declares all three blocks, in order, and no more.
/// MEDIUM.
///
/// The 3-block structure (Block 1 struct / class, Block 2 protocol
/// implementation, Block 3 constructors, std traits, helpers) is the shape the
/// capability HOW-TO documents, and the `// Block 1:` … `// Block 3:` banners
/// are how a reader sees it. `check_block_order` only compares two `impl` lines
/// — it cannot tell a file that documents its three blocks from one that
/// happens to declare them in a lucky order — so a file with no banners at all
/// passed. This check reads the banners themselves and requires all three.
///
/// Three findings, one per defect, so a file missing two blocks is told about
/// both rather than once:
///
/// - no banner at all — the file never declares the structure;
/// - a subset of 1/2/3 — the file declares some blocks and skips others;
/// - out of order or a block above 3 — the sequence itself is wrong.
///
/// The banner is a whole-word `Block <digits>:` inside a comment. Requiring the
/// colon keeps prose such as `Block 1 (types) -> Block 2` from reading as a
/// banner, and the word boundary keeps `Sub-Block 4:` out.
///
/// The marker parser is duplicated from `utility_agent_role_checker` rather than
/// shared: AES201 forbids a utility importing another utility, so the two
/// layers cannot share one copy without moving the parser into a layer both may
/// import. They are kept identical deliberately — the same banner means the
/// same thing to both rules.
pub fn check_block_markers(file: &FileEntry, violations: &mut Vec<LintResult>) {
    let path = file.path.to_string_lossy().to_string();
    let markers = block_marker_numbers(&file.content);

    if markers.is_empty() {
        violations.push(LintResult::new_arch(
            &path,
            0,
            "AES403",
            Severity::MEDIUM,
            format!(
                "AES403 CAPABILITY_ROLE: Capability file declares no block markers.\n\
                 WHY? {path} carries no `Block 1:` / `Block 2:` / `Block 3:` banner \
                 comment, so the reader is given no map of the file. The 3-block shape \
                 is Block 1 (struct definition) -> Block 2 (protocol trait \
                 implementation) -> Block 3 (constructors, std traits, helpers).\n\
                 FIX: Add the three banner comments above their blocks:\n  \
                 // Block 1: Struct Definition\n  \
                 // Block 2: Protocol Trait Implementation\n  \
                 // Block 3: Constructors, Std Traits, Helpers"
            ),
        ));
        return;
    }

    let declared: BTreeSet<usize> = markers.iter().copied().collect();
    let missing: Vec<usize> = [1usize, 2, 3]
        .into_iter()
        .filter(|n| !declared.contains(n))
        .collect();
    if !missing.is_empty() {
        violations.push(LintResult::new_arch(
            &path,
            0,
            "AES403",
            Severity::MEDIUM,
            format!(
                "AES403 CAPABILITY_ROLE: Capability file is missing block marker(s).\n\
                 WHY? {path} declares {} but not {}. A capability is three blocks — \
                 Block 1 (struct definition), Block 2 (protocol trait implementation), \
                 Block 3 (constructors, std traits, helpers) — and each needs its banner \
                 so the reader can find the seam.\n\
                 FIX: Add the missing banner comment(s) above the block they head.",
                block_list(&declared),
                missing
                    .iter()
                    .map(|n| format!("Block {n}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        ));
    }

    // A banner above 3 means the file outgrew the shape, and a sequence that is
    // not 1 -> 2 -> 3 means the map lies about the order.
    let above_three: Vec<usize> = markers.iter().copied().filter(|n| *n > 3).collect();
    if !above_three.is_empty() {
        violations.push(LintResult::new_arch(
            &path,
            0,
            "AES403",
            Severity::MEDIUM,
            format!(
                "AES403 CAPABILITY_ROLE: Capability file carries block markers beyond Block 3.\n\
                 WHY? {path} declares {}. The 3-block structure is Block 1 (struct \
                 definition) -> Block 2 (protocol trait implementation) -> Block 3 \
                 (constructors, std traits, helpers); a Block 4 means the file has \
                 outgrown it.\n\
                 FIX: Fold the extra blocks back into Block 3, or move the \
                 behaviour they hold into a capability or utility file.",
                above_three
                    .iter()
                    .map(|n| format!("Block {n}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        ));
    }

    // Order is checked over the distinct numbers in the order they first appear,
    // so a banner repeated in a doc comment cannot reorder the sequence.
    let mut sequence: Vec<usize> = Vec::new();
    for n in &markers {
        if !sequence.contains(n) {
            sequence.push(*n);
        }
    }
    let mut expected = sequence.clone();
    expected.sort_unstable();
    if sequence != expected {
        violations.push(LintResult::new_arch(
            &path,
            0,
            "AES403",
            Severity::MEDIUM,
            format!(
                "AES403 CAPABILITY_ROLE: Capability block markers are out of order.\n\
                 WHY? {path} declares them as {} but the structure is fixed: Block 1 \
                 (struct definition) -> Block 2 (protocol trait implementation) -> \
                 Block 3 (constructors, std traits, helpers).\n\
                 FIX: Move the banner comments so they head their blocks in \
                 1 -> 2 -> 3 order.",
                sequence
                    .iter()
                    .map(|n| format!("Block {n}"))
                    .collect::<Vec<_>>()
                    .join(" -> ")
            ),
        ));
    }
}

/// The declared block numbers as a readable `Block 1, Block 2` list.
fn block_list(declared: &BTreeSet<usize>) -> String {
    declared
        .iter()
        .map(|n| format!("Block {n}"))
        .collect::<Vec<_>>()
        .join(", ")
}

/// The `N` of every `Block N:` banner comment, in declaration order.
///
/// A banner is `Block <digits>:` standing as its own word inside a comment. Both
/// halves matter. The colon separates a marker from prose — the HOW-TO and rule
/// messages describe the structure as `Block 1 (type + injected deps) -> Block
/// 2`, and requiring the colon keeps those sentences from reading as markers.
/// The word boundary keeps a longer word that merely contains "Block" — such as
/// `Sub-Block 4:` — from reading as a banner either.
fn block_marker_numbers(content: &str) -> Vec<usize> {
    let mut out = Vec::new();
    for line in content.lines() {
        let t = line.trim();
        if !is_comment(t) {
            continue;
        }
        let Some((idx, rest)) = standalone_word(t, "Block") else {
            continue;
        };
        let rest = &t[idx + rest..];
        let digits: String = rest.chars().take_while(char::is_ascii_digit).collect();
        if digits.is_empty() || !rest[digits.len()..].starts_with(':') {
            continue;
        }
        if let Ok(n) = digits.parse::<usize>() {
            out.push(n);
        }
    }
    out
}

/// True when a line is a comment in any of the three languages.
fn is_comment(trimmed: &str) -> bool {
    trimmed.starts_with("//")
        || trimmed.starts_with("/*")
        || trimmed.starts_with('*')
        || trimmed.starts_with('#')
        || trimmed.starts_with("///")
}

/// The byte offset of *word* in `t` when it stands as a standalone token, plus
/// the length of the characters that follow it.
///
/// `Sub-Block 4:` contains the substring "Block" but the character before it is
/// part of a longer word, so it is not a standalone occurrence. Rust identifiers
/// treat `-` as a separator, but a banner comment is prose: a hyphenated
/// `Sub-Block` reads as one word to a human, so a hyphen counts as part of the
/// preceding token here.
fn standalone_word(t: &str, word: &str) -> Option<(usize, usize)> {
    let mut from = 0;
    while let Some(rel) = t[from..].find(word) {
        let idx = from + rel;
        let before_ok = t[..idx]
            .chars()
            .next_back()
            .is_none_or(|c| !c.is_alphanumeric() && c != '_' && c != '-');
        let after = &t[idx + word.len()..];
        let after_ok = after
            .chars()
            .next()
            .is_none_or(|c| !c.is_alphanumeric() && c != '_');
        if before_ok && after_ok {
            // Return the offset *after* the word and the character that follows
            // it, measured from the character that follows — never from the
            // word's own first byte, which is always one byte and would leave
            // the slice inside a multi-byte character when the next character is
            // not ASCII (a fullwidth colon in a comment, say).
            let skip = after.chars().next().map_or(0, char::len_utf8);
            return Some((idx, word.len() + skip));
        }
        from = idx + word.len();
    }
    None
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
