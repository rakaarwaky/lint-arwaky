use shared_cli_commands::LintResult;
use shared_quality_rules::contract_quality_protocol::IDeadInheritanceProtocol;

use shared_common::taxonomy_severity_vo::Severity;
use shared_quality_rules::utility_bypass_detector::skip_cfg_test_block;

// PURPOSE: DeadInheritanceChecker — IDeadInheritanceProtocol for AES303 dead inheritance sub-check
// Detects empty/placeholder definitions: unit structs without impl, empty Python classes,
// empty JS/TS classes or interfaces.
// ALGORITHM:
//   1. Iterate lines; skip #[cfg(test)] blocks
//   2. For each `struct Foo;` (unit struct) → flag unless followed by impl block or has derive
//   3. For each `class Foo: pass` (Python empty class) → flag
//   4. For each `class Foo {}` / `interface Foo {}` (JS/TS empty) → flag

// ─── Block 1: Struct Definition ───────────────────────────

pub struct DeadInheritanceChecker {}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IDeadInheritanceProtocol for DeadInheritanceChecker {
    fn check_dead_inheritance(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let lines: Vec<&str> = content.lines().collect();
        let mut i = 0;
        while i < lines.len() {
            let t = lines[i].trim();
            // Skip #[cfg(test)] modules correctly — advance past the entire block
            if t.starts_with("#[cfg(test)]") {
                i = skip_cfg_test_block(&lines, i);
                continue;
            }
            // Rust: unit struct `struct Foo;` or `pub struct Foo;` (tuple structs excluded)
            let stripped = Self::strip_visibility(t);
            if stripped.starts_with("struct ") && stripped.ends_with(';') && !stripped.contains('(')
            {
                // Skip unit structs with derive attribute — #[derive(...)] provides impl
                let has_derive = i > 0 && lines[i - 1].trim().starts_with("#[derive(");
                if has_derive {
                    i += 1;
                    continue;
                }
                // Skip if followed by impl block (intentional placeholder)
                let mut next_idx = i + 1;
                while next_idx < lines.len() {
                    let next_t = lines[next_idx].trim();
                    if next_t.is_empty() || next_t.starts_with('#') || next_t.starts_with("//") {
                        next_idx += 1;
                    } else {
                        break;
                    }
                }
                let next_is_impl = match lines.get(next_idx) {
                    Some(l) => l.trim().starts_with("impl "),
                    None => false,
                };
                if !next_is_impl {
                    violations.push(LintResult::new_arch(
                        file,
                        i + 1,
                        "AES303",
                        Severity::MEDIUM,
                        format!(
                            "AES303 DEAD_INHERITANCE: Empty struct, class, or interface implementation block detected.\nWHY? Unit struct declared on line {} in {} without impl or derive\nFIX: Implement the necessary methods/fields or remove the empty definition block.",
                            i + 1,
                            file
                        ),
                    ));
                }
                i += 1;
                continue;
            }
            // Python: empty class `class Foo: pass` (single line or multi-line)
            // Skip abstract classes: class Foo(ABC):, class Foo(Protocol):, or with metaclass=ABCMeta
            if t.starts_with("class ") || t.starts_with("class\t") {
                let is_abstract = t.contains("(ABC)")
                    || t.contains("(Protocol)")
                    || t.contains("metaclass=ABCMeta")
                    || t.contains("ABCMeta");
                if is_abstract {
                    i += 1;
                    continue;
                }
                if t.ends_with(": pass") || t.ends_with(":pass") {
                    violations.push(LintResult::new_arch(
                        file,
                        i + 1,
                        "AES303",
                        Severity::MEDIUM,
                        format!(
                            "AES303 DEAD_INHERITANCE: Empty struct, class, or interface implementation block detected.\nWHY? Empty Python class on line {} in {} (': pass')\nFIX: Implement the necessary methods/fields or remove the empty definition block.",
                            i + 1,
                            file
                        ),
                    ));
                } else if t.ends_with(':') && i + 1 < lines.len() {
                    let next = lines[i + 1].trim();
                    if next == "pass" || next == "..." || next == "Ellipsis" {
                        violations.push(LintResult::new_arch(
                            file,
                            i + 1,
                            "AES303",
                            Severity::MEDIUM,
                            format!(
                                "AES303 DEAD_INHERITANCE: Empty struct, class, or interface implementation block detected.\nWHY? Empty Python class on line {} in {} (body is '{}')\nFIX: Implement the necessary methods/fields or remove the empty definition block.",
                                i + 1,
                                file,
                                next
                            ),
                        ));
                    }
                }
            }
            // JS/TS: empty class/interface `class Foo {}`, `export class Foo {}`, `interface Bar {}`
            if Self::is_empty_js_declaration(t) {
                violations.push(LintResult::new_arch(
                    file,
                    i + 1,
                    "AES303",
                    Severity::MEDIUM,
                    format!(
                        "AES303 DEAD_INHERITANCE: Empty struct, class, or interface implementation block detected.\nWHY? Empty JS/TS class/interface on line {} in {}\nFIX: Implement the necessary methods/fields or remove the empty definition block.",
                        i + 1,
                        file
                    ),
                ));
            }
            i += 1;
        }
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for DeadInheritanceChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl DeadInheritanceChecker {
    pub fn new() -> Self {
        Self {}
    }

    /// Strip Rust visibility modifiers from the beginning of a line.
    /// Handles `pub`, `pub(crate)`, `pub(crate)`, `pub(super)`, etc.
    fn strip_visibility(line: &str) -> &str {
        let trimmed = line.trim();
        if trimmed.starts_with("pub ") || trimmed.starts_with("pub(") {
            // Skip past the visibility modifier
            if let Some(rest) = trimmed.strip_prefix("pub ") {
                rest
            } else if let Some(rest) = trimmed.strip_prefix("pub(") {
                // Find closing paren for pub(crate), pub(super), etc.
                if let Some(end_paren) = rest.find(')') {
                    let after = &rest[end_paren + 1..];
                    // Skip any whitespace after the closing paren
                    after.trim_start()
                } else {
                    trimmed
                }
            } else {
                trimmed
            }
        } else {
            trimmed
        }
    }

    /// Detect JS/TS empty class or interface declarations.
    /// Handles `class Foo {}`, `export class Foo {}`, `export default class Foo {}`.
    fn is_empty_js_declaration(line: &str) -> bool {
        let code = line
            .split_once("//")
            .map(|(code, _comment)| code)
            .unwrap_or(line);

        let compact: String = code.split_whitespace().collect();

        compact.ends_with("{}") && Self::js_ts_declares_primary_symbol(code)
    }

    /// Detect JS/TS primary symbols: class or interface.
    fn js_ts_declares_primary_symbol(line: &str) -> bool {
        let code = line
            .split_once("//")
            .map(|(code, _comment)| code)
            .unwrap_or(line);

        let tokens: Vec<&str> = code.split_whitespace().collect();

        if let Some(pos) = tokens
            .iter()
            .position(|tok| *tok == "class" || *tok == "interface")
        {
            if pos == 0 {
                return true;
            }

            return matches!(
                tokens[pos - 1],
                "export" | "default" | "abstract" | "declare"
            );
        }

        false
    }
}
