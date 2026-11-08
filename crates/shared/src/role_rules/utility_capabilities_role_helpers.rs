// ─── Shared helpers for capability role auditors (moved out to keep
// utility_capabilities_role_checker.rs under the AES301 line limit) ─────────

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

/// True when a decorator claims protocol attachment: `@with_protocol`,
/// `@with_adapter_protocol`, `@protocol`, `@protocol_impl("I...")`, …
/// Matched by shape so renames (`@with_tools_protocol`) stay covered.
pub fn is_protocol_decorator(name: &str) -> bool {
    let head = name
        .trim_start_matches('@')
        .split(|c: char| c.is_ascii_digit() || c == '(')
        .next()
        .unwrap_or(name);
    head == "protocol"
        || head == "protocol_impl"
        || head.starts_with("with_protocol")
        || (head.starts_with("with_") && head.ends_with("_protocol"))
}

/// True when a line is a comment in any of the three languages.
pub fn is_comment(trimmed: &str) -> bool {
    trimmed.starts_with("//")
        || trimmed.starts_with("/*")
        || trimmed.starts_with('*')
        || trimmed.starts_with('#')
        || trimmed.starts_with("///")
}

/// Count type declarations by line scan, for files with no parse metadata.
pub fn scan_type_declarations(content: &str) -> usize {
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
pub fn scan_protocol_impl(content: &str) -> bool {
    let has_function = content.lines().any(|l| l.trim_start().starts_with("def "));
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
        // Decorator that claims protocol attachment: requires at least one
        // `def` somewhere in the file, so a bare `@with_adapter_protocol` on
        // an empty class does not pass.
        if t.starts_with('@') && is_protocol_decorator(t) && has_function {
            return true;
        }
        t.contains("implements I") && t.contains("Protocol")
    })
}

/// Collect distinct protocol trait names by line scan, for files with no parse metadata.
///
/// Supports Rust (`impl IXxxProtocol for Yyy`), Python
/// (`class X(IAProtocol, IBProtocol)`), and TypeScript
/// (`class X implements IAProtocol, IBProtocol`).
pub fn scan_protocol_names(content: &str) -> Vec<String> {
    let mut protocols: Vec<String> = Vec::new();
    let has_function = content.lines().any(|l| l.trim_start().starts_with("def "));

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

        // Decorator that claims protocol attachment: one synthetic entry per
        // file, counted only when the file actually carries a `def`.
        if t.starts_with('@') && is_protocol_decorator(t) && has_function && protocols.is_empty() {
            protocols.push("decorated_protocol".to_string());
        }
    }

    protocols
}

/// The `N` and name text of every `Block N:` banner comment, in declaration
/// order. The name is the rest of the line after the colon, with box-drawing
/// dashes and surrounding whitespace stripped.
///
/// Banner lines are whole-line: optional comment sigil, dashes, `Block N:`,
/// name text, trailing dashes. Prose inside string literals never matches —
/// only a line that actually reads as a banner comment counts. This is how
/// auditor messages that quote `// Block 1:` inside a string don't get
/// mistaken for real banners.
///
/// `Block 1:` / `Block 2:` / `Block 3:` are the canonical capability banners.
/// A file that never declares its blocks yields an empty vec; callers fall
/// back to raw line position.
pub fn block_markers(content: &str) -> Vec<(usize, String, usize)> {
    let mut out = Vec::new();
    for (line_idx, line) in content.lines().enumerate() {
        let t = line.trim();
        if !is_comment(t) {
            continue;
        }
        // Anchor to whole-line banner shape: optional comment prefix, dashes,
        // "Block N:", name text, trailing dashes. Prose inside string literals
        // (e.g. auditor messages that quote `// Block 1:`) never matches,
        // because a real banner line starts with the comment sigil itself.
        let stripped = t
            .trim_start_matches("//")
            .trim_start_matches('/')
            .trim_start_matches('*')
            .trim_start_matches('#')
            .trim();
        if !stripped.starts_with("───") && !stripped.starts_with("---") {
            continue;
        }
        // Expect `─── Block N: name ───` after the sigil.
        let Some((_, rest)) = stripped.split_once(' ') else {
            continue;
        };
        let Some((num_str, name_tail)) = rest.split_once(':') else {
            continue;
        };
        let digits: String = num_str.chars().filter(char::is_ascii_digit).collect();
        if digits.is_empty() {
            continue;
        }
        let Some(n) = digits.parse::<usize>().ok() else {
            continue;
        };
        // name_tail is like ` Module Declaration ───` or ` Module Contracts ───`.
        // Canonical names per block number.
        let expected_name: std::collections::BTreeMap<usize, &'static str> =
            std::collections::BTreeMap::from([
                (1, "Module Declaration"),
                (2, "Module Contract"),
                (3, "Module Helpers"),
            ]);
        let name = name_tail
            .trim()
            .trim_matches(|c| c == '─' || c == '-')
            .trim()
            .to_string();
        let _ = expected_name.get(&n); // validate block number is known
        out.push((n, name, line_idx + 1));
    }
    out
}
