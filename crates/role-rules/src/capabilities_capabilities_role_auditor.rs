// PURPOSE: CapabilitiesRoleChecker — AES403: enforce capability type composition.
//
// ALGORITHM (applied uniformly across Rust, Python, TypeScript):
//   1. Collect all type declarations (struct/class/enum/interface).
//      Skip #[cfg(test)] blocks (Rust only).
//   2. Rule 1 — Max 3 type declarations per file. If exceeded → flag CapabilityTooManyTypes.
//      Checked first; if violated, skip implementor check entirely.
//   3. Rule 2 — Must have ≥ 1 implementor:
//        Rust:   impl <Trait> for <Struct>
//        Python: class <Name>(<Parent>):
//        TS:     class <Name> implements <IProtocol>
//      If none → flag CapabilityNoImplementor.
//   4. Internal helper types (without implementor pattern) are ALLOWED and not flagged.
//
// NOTE: Import checking is handled by import-rules crate, not role-rules.

use shared::cli_commands::LintResult;
use shared::common::Severity;
use shared::common::taxonomy_message_vo::LintMessage;
use shared::common::utility_language_detector::detect_language_info_from_source;
use shared::role_rules::{AesRoleViolation, ICapabilitiesRoleChecker};

use shared::common::SourceContentVO;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct CapabilitiesRoleChecker {}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl ICapabilitiesRoleChecker for CapabilitiesRoleChecker {
    fn check_capability_routing(
        &self,
        source: &SourceContentVO,
        layer: &str,
        violations: &mut Vec<LintResult>,
    ) {
        if layer != "capabilities" && !layer.starts_with("capabilities(") {
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
            // is_js = true for both JavaScript AND TypeScript
            self._check_ts_routing(file, content, violations);
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

    fn _check_rust_routing(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        // ── Collect all type declarations (struct/enum), skip #[cfg(test)] ──
        let mut in_cfg_test = false;
        let mut type_count: usize = 0;
        let mut struct_names: Vec<&str> = Vec::new(); // structs only (for implementor check)

        for l in content.lines() {
            let t = l.trim();

            // skip #[cfg(test)] block
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
            if (t.starts_with("pub struct ") || t.starts_with("struct ")) && words.len() >= 2
                && let Some(idx) = words.iter().position(|w| *w == "struct")
                    && let Some(name) = words.get(idx + 1) {
                        let name = name.trim_end_matches(';').trim_end_matches('{');
                        if !name.is_empty() && !name.starts_with('_') {
                            type_count += 1;
                            struct_names.push(name);
                        }
                    }

            // detect enum
            if (t.starts_with("pub enum ") || t.starts_with("enum ")) && words.len() >= 2
                && let Some(idx) = words.iter().position(|w| *w == "enum")
                    && let Some(name) = words.get(idx + 1) {
                        let name = name.trim_end_matches(';').trim_end_matches('{');
                        if !name.is_empty() && !name.starts_with('_') {
                            type_count += 1;
                        }
                    }
        }

        // ── RULE 1: max 3 types (struct + enum) ──────────────
        if type_count > 3 {
            violations.push(LintResult::new_arch(
                file,
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
            return; // no further check needed
        }

        // ── RULE 2: must have ≥ 1 struct implementor ──
        //    Implementor = has "impl <Trait> for <StructName>"
        //    (not merely "for item in collection")
        let has_implementor = struct_names.iter().any(|s| {
            content.lines().any(|l| {
                let t = l.trim();
                t.starts_with("impl ")
                    && (t.contains(&format!("for {} ", s))
                        || t.contains(&format!("for {}{{", s))
                        || t.contains(&format!("for {} {{", s)))
            })
        });

        if !has_implementor {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES403",
                Severity::MEDIUM,
                AesRoleViolation::CapabilityNoImplementor {
                    reason: Some(LintMessage::new(format!(
                        "No impl Trait for struct pattern found in {}. At least one struct must implement a _protocol trait.",
                        file
                    ))),
                },
            ));
        }
    }

    fn _check_ts_routing(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let lines: Vec<&str> = content.lines().collect();

        // ── Collect all type declarations (class/interface/enum) ────
        let mut type_count: usize = 0;
        let mut implementor_found = false;

        for l in &lines {
            let t = l.trim();

            // ── class ──
            let class_body = t
                .strip_prefix("export class ")
                .or_else(|| t.strip_prefix("class "));
            if let Some(rest) = class_body {
                let name = rest
                    .split(|c: char| [' ', '(', '{'].contains(&c))
                    .next()
                    .unwrap_or("")
                    .trim();

                if name.is_empty() || name.starts_with('_') {
                    continue;
                }
                type_count += 1;

                // check for "implements" keyword (any implements = implementor)
                if rest.contains("implements ") {
                    implementor_found = true;
                }
                continue;
            }

            // ── interface ──
            let iface_body = t
                .strip_prefix("export interface ")
                .or_else(|| t.strip_prefix("interface "));
            if let Some(rest) = iface_body {
                let name = rest
                    .split(|c: char| [' ', '{', '<'].contains(&c))
                    .next()
                    .unwrap_or("")
                    .trim();
                if !name.is_empty() && !name.starts_with('_') {
                    type_count += 1;
                }
                continue;
            }

            // ── enum ──
            let enum_body = t
                .strip_prefix("export enum ")
                .or_else(|| t.strip_prefix("enum "));
            if let Some(rest) = enum_body {
                let name = rest
                    .split(|c: char| [' ', '{'].contains(&c))
                    .next()
                    .unwrap_or("")
                    .trim();
                if !name.is_empty() && !name.starts_with('_') {
                    type_count += 1;
                }
            }
        }

        // ── RULE 1: max 3 types ──────────────────────────────
        if type_count > 3 {
            violations.push(LintResult::new_arch(
                file,
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

        // ── RULE 2: must have ≥ 1 implementor ──────────────────
        if !implementor_found {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES403",
                Severity::MEDIUM,
                AesRoleViolation::CapabilityNoImplementor {
                    reason: Some(LintMessage::new(format!(
                        "No class with 'implements' keyword found in {}. At least one class must implement an interface/protocol.",
                        file
                    ))),
                },
            ));
        }
    }

    fn _check_python_routing(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let lines: Vec<&str> = content.lines().collect();

        // ── Collect all classes with parents (inheritance) ──────
        let mut class_count: usize = 0;
        let mut implementor_found = false;
        let mut i = 0;

        while i < lines.len() {
            let t = lines[i].trim();
            i += 1;

            if !t.starts_with("class ") {
                continue;
            }

            // parse: class Name(Parent1, Parent2):  or multi-line version
            let after_class = &t[6..]; // skip "class "
            let name = after_class
                .split(|c: char| ['(', ':', ' '].contains(&c))
                .next()
                .unwrap_or("")
                .trim();

            if name.is_empty() || name.starts_with('_') {
                continue;
            }

            class_count += 1;

            // check for inheritance — handle single-line and multi-line class declarations
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

        // ── RULE 1: max 3 types ──────────────────────────────
        if class_count > 3 {
            violations.push(LintResult::new_arch(
                file,
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

        // ── RULE 2: must have ≥ 1 implementor ────────────────
        if !implementor_found {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES403",
                Severity::MEDIUM,
                AesRoleViolation::CapabilityNoImplementor {
                    reason: Some(LintMessage::new(format!(
                        "No class with parent/inheritance found in {}. At least one class must inherit from a parent class.",
                        file
                    ))),
                },
            ));
        }
    }
}
