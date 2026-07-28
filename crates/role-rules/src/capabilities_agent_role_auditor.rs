// PURPOSE: AgentRoleChecker — AES405: enforce agent type composition
//          and forbid any-type annotations.
//
// ALGORITHM:
//   check_agent_routing (per language):
//     1. Collect all type declarations (struct/enum/class/interface).
//        Skip #[cfg(test)] blocks (Rust).
//     2. Rule 1 — Max 3 types. If exceeded → flag AgentTooManyTypes.
//     3. Rule 2 — At least 1 implementor required:
//          Rust:   impl <Trait> for <Struct>
//          Python: class <Name>(<Parent>):  (any parent = implementor)
//          TS:     class <Name> implements <IProtocol> (any implements = implementor)
//        If none → flag AgentNoImplementor.
//     4. Internal helper types without implementor pattern are ALLOWED.
//
//   Note: aggregate import is enforced by mandatory checker (AES202), not here.
//
//   check_any_type_annotation:
//     Line-by-line scan for `: any`, `: Any`, `-> any`, `-> Any`,
//     `Any<`, `Any[`, `any[` patterns. Flags each as AES405 AnyType.

use shared::cli_commands::LintResult;
use shared::common::utility_language_detector::detect_language_info_from_source;
use shared::common::{LintMessage, Severity};
use shared::role_rules::{AesRoleViolation, IAgentRoleChecker};

use shared::common::{SourceContentVO, SymbolName};

// ─── Block 1: Struct Definition ───────────────────────────

pub struct AgentRoleChecker {}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IAgentRoleChecker for AgentRoleChecker {
    fn check_agent_routing(
        &self,
        source: &SourceContentVO,
        layer: &str,
        violations: &mut Vec<LintResult>,
    ) {
        if layer != "agent" && !layer.starts_with("agent(") {
            return;
        }
        let file = source.file_path.value();
        let content = source.content.value();
        let li = detect_language_info_from_source(source);

        if li.is_rs {
            self._check_rust_routing(file, content, violations);
        } else if li.is_py {
            self._check_python_routing(file, content, violations);
        } else if li.is_js {
            self._check_ts_routing(file, content, violations);
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
    }        // ─── Rust ──────────────────────────────────────────────

    fn _check_rust_routing(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        // Note: aggregate import is enforced by mandatory checker (AES202), not here.
        // AES405 only checks type composition rules.

        // Collect all structs & enums (skip #[cfg(test)])
        let mut in_cfg_test = false;
        let mut type_names: Vec<&str> = Vec::new();
        let mut struct_names: Vec<&str> = Vec::new();

        for l in content.lines() {
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

            // detect struct
            if (t.starts_with("pub struct ") || t.starts_with("struct ")) && words.len() >= 2 {
                if let Some(idx) = words.iter().position(|w| *w == "struct") {
                    if let Some(name) = words.get(idx + 1) {
                        let name = name.trim_end_matches(';').trim_end_matches('{');
                        if !name.is_empty() && !name.starts_with('_') {
                            type_names.push(name);
                            struct_names.push(name);
                        }
                    }
                }
            }

            // detect enum
            if (t.starts_with("pub enum ") || t.starts_with("enum ")) && words.len() >= 2 {
                if let Some(idx) = words.iter().position(|w| *w == "enum") {
                    if let Some(name) = words.get(idx + 1) {
                        let name = name.trim_end_matches(';').trim_end_matches('{');
                        if !name.is_empty() && !name.starts_with('_') {
                            type_names.push(name);
                        }
                    }
                }
            }
        }

        // Rule 3: max 3 types
        if type_names.len() > 3 {
            let names_str: String = type_names.join(", ");
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES405",
                Severity::HIGH,
                AesRoleViolation::AgentTooManyTypes {
                    count: type_names.len(),
                    names: type_names.iter().map(|s| SymbolName::new(*s)).collect(),
                    reason: Some(LintMessage::new(format!(
                        "Found {} types (struct/enum) in {}, max 3 allowed: [{}]",
                        type_names.len(),
                        file,
                        names_str
                    ))),
                },
            ));
            return;
        }

        // Rule 2: at least 1 implementor of aggregate trait
        let has_implementor = struct_names.iter().any(|s| {
            content.contains("impl ")
                && (content.contains(&format!("for {} ", s))
                    || content.contains(&format!("for {}{{", s))
                    || content.contains(&format!("for {} {{", s)))
        });

        if !has_implementor {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES405",
                Severity::MEDIUM,
                AesRoleViolation::AgentNoImplementor {
                    reason: Some(LintMessage::new(format!(
                        "No impl Trait for struct pattern found in {}. At least one struct must implement an aggregate trait.",
                        file
                    ))),
                },
            ));
        }

        // Rule 1: internal structs without aggregate impl are NOT flagged
    }

    // ─── TypeScript / JavaScript ───────────────────────────

    fn _check_ts_routing(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        // Note: aggregate import is enforced by mandatory checker (AES202), not here.
        // AES405 only checks type composition rules.

        let mut type_names: Vec<&str> = Vec::new();
        let mut implementor_found = false;

        for l in content.lines() {
            let t = l.trim();

            // detect class
            let class_body = t
                .strip_prefix("export class ")
                .or_else(|| t.strip_prefix("class "));
            if let Some(rest) = class_body {
                let name = rest.split([' ', '(', '{']).next().unwrap_or("").trim();
                if name.is_empty() || name.starts_with('_') {
                    continue;
                }
                type_names.push(name);

                // Any implements clause = implementor found (aggregate check is AES202's job)
                if rest.contains("implements ") {
                    implementor_found = true;
                }
                continue;
            }

            // detect interface
            let iface_body = t
                .strip_prefix("export interface ")
                .or_else(|| t.strip_prefix("interface "));
            if let Some(rest) = iface_body {
                let name = rest.split([' ', '{', '<']).next().unwrap_or("").trim();
                if !name.is_empty() && !name.starts_with('_') {
                    type_names.push(name);
                }
                continue;
            }

            // detect enum
            let enum_body = t
                .strip_prefix("export enum ")
                .or_else(|| t.strip_prefix("enum "));
            if let Some(rest) = enum_body {
                let name = rest.split([' ', '{']).next().unwrap_or("").trim();
                if !name.is_empty() && !name.starts_with('_') {
                    type_names.push(name);
                }
            }
        }

        // Rule 3: max 3 types
        if type_names.len() > 3 {
            let names_str: String = type_names.join(", ");
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES405",
                Severity::HIGH,
                AesRoleViolation::AgentTooManyTypes {
                    count: type_names.len(),
                    names: type_names.iter().map(|s| SymbolName::new(*s)).collect(),
                    reason: Some(LintMessage::new(format!(
                        "Found {} types (class/interface/enum) in {}, max 3 allowed: [{}]",
                        type_names.len(),
                        file,
                        names_str
                    ))),
                },
            ));
            return;
        }

        // Rule 2: at least 1 implementor
        if !implementor_found {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES405",
                Severity::MEDIUM,
                AesRoleViolation::AgentNoImplementor {
                    reason: Some(LintMessage::new(format!(
                        "No class with 'implements' keyword found in {}. At least one class must implement an aggregate interface.",
                        file
                    ))),
                },
            ));
        }

        // Rule 1: internal class without implements → NOT flagged
    }

    // ─── Python ────────────────────────────────────────────

    fn _check_python_routing(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        // Note: aggregate import is enforced by mandatory checker (AES202), not here.
        // AES405 only checks type composition rules.

        let lines: Vec<&str> = content.lines().collect();
        let mut type_names: Vec<&str> = Vec::new();
        let mut implementor_found = false;
        let mut i = 0;

        while i < lines.len() {
            let t = lines[i].trim();
            i += 1;

            if !t.starts_with("class ") {
                continue;
            }

            // skip "class " prefix
            let after_class = &t[6..];
            let name = after_class
                .split(['(', ':', ' '])
                .next()
                .unwrap_or("")
                .trim();
            if name.is_empty() || name.starts_with('_') {
                continue;
            }
            type_names.push(name);

            // Any inheritance = implementor found (aggregate check is AES202's job)
            // Handle both single-line and multi-line class declarations
            if let Some(start) = t.find('(') {
                let after_paren = &t[start + 1..];

                if let Some(end) = after_paren.find(')') {
                    // Single-line: class Name(Parent1, Parent2):
                    let parents = after_paren[..end].trim();
                    if !parents.is_empty() {
                        implementor_found = true;
                    }
                } else {
                    // Multi-line: class Name(\n    Parent1,\n    Parent2,\n):
                    // Collect text from subsequent lines until closing ')'
                    let mut paren_text = String::from(after_paren);
                    while i < lines.len() {
                        let next = lines[i];
                        i += 1;
                        if let Some(end) = next.find(')') {
                            paren_text.push_str(&next[..end]);
                            break;
                        }
                        paren_text.push_str(next);
                        if next.trim().starts_with("class ") {
                            break;
                        }
                    }
                    if !paren_text.trim().is_empty() {
                        implementor_found = true;
                    }
                }
            }
        }

        // Rule 3: max 3 types
        if type_names.len() > 3 {
            let names_str: String = type_names.join(", ");
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES405",
                Severity::HIGH,
                AesRoleViolation::AgentTooManyTypes {
                    count: type_names.len(),
                    names: type_names.iter().map(|s| SymbolName::new(*s)).collect(),
                    reason: Some(LintMessage::new(format!(
                        "Found {} classes in {}, max 3 allowed: [{}]",
                        type_names.len(),
                        file,
                        names_str
                    ))),
                },
            ));
            return;
        }

        // Rule 2: at least 1 implementor
        if !implementor_found {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES405",
                Severity::MEDIUM,
                AesRoleViolation::AgentNoImplementor {
                    reason: Some(LintMessage::new(format!(
                        "No class with parent/inheritance found in {}. At least one class must inherit from a parent class.",
                        file
                    ))),
                },
            ));
        }

        // Rule 1: internal class without ABC → NOT flagged
    }
}
