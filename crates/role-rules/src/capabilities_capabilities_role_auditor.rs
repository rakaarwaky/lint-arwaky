// PURPOSE: CapabilitiesRoleChecker — AES403: enforce capability type composition.
//
// ALGORITHM (per language):
//   1. Guard: file must import from a _protocol module.
//      If missing → flag CapabilityNoProtocol.
//   2. Collect all type declarations (struct/enum/class/interface).
//      Skip #[cfg(test)] blocks (Rust).
//   3. Rule 3 — Max 3 types. If exceeded → flag CapabilityTooManyTypes.
//   4. Rule 2 — At least 1 implementor required:
//        Rust:   impl <Trait> for <Struct>
//        Python: class <Name>(<ABCProtocol>):
//        TS:     class <Name> implements <IProtocol>
//      If none → flag CapabilityNoImplementor.
//   5. Rule 1 — Internal helper types without trait impl are ALLOWED.
//
// NOTE: The layer guard is redundant with the caller but kept for defensive programming.

use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::common::taxonomy_severity_vo::Severity;
use shared::common::utility_language_detector::detect_language_info_from_source;
use shared::role_rules::contract_capabilities_role_protocol::ICapabilitiesRoleChecker;
use shared::role_rules::taxonomy_violation_role_vo::AesRoleViolation;
use shared::taxonomy_source_vo::SourceContentVO;

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
        // ── Guard: must have _protocol import ──────────────────
        let has_proto_import = content.contains("use ") && content.contains("_protocol::");
        if !has_proto_import {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES403",
                Severity::MEDIUM,
                AesRoleViolation::CapabilityNoProtocol { reason: None },
            ));
            return;
        }

        // ── Collect all structs & enums (skip #[cfg(test)]) ──
        let mut in_cfg_test = false;
        let mut type_names: Vec<&str> = Vec::new(); // all structs + enums
        let mut struct_names: Vec<&str> = Vec::new(); // only structs

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

        // ── RULE 3: max 3 types (struct + enum) ──────────────
        if type_names.len() > 3 {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES403",
                Severity::HIGH,
                AesRoleViolation::CapabilityTooManyTypes {
                    count: type_names.len(),
                    reason: None,
                },
            ));
            return; // no further check needed
        }

        // ── RULE 2: must have ≥ 1 struct implementor protocol ──
        //    Implementor = has "impl <Trait> for <StructName>"
        //    (not merely "for item in collection")
        let has_implementor = struct_names.iter().any(|s| {
            content.lines().any(|l| {
                let t = l.trim();
                t.starts_with("impl ") && t.contains(&format!("for {s}"))
            })
        });

        if !has_implementor {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES403",
                Severity::MEDIUM,
                AesRoleViolation::CapabilityNoImplementor { reason: None },
            ));
        }

        // ── RULE 1: internal struct without trait impl → NOT flagged ──
        // (no loop that flags individual structs anymore)
    }

    fn _check_ts_routing(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        // ── Guard: must have _protocol import ──────────────────
        let has_proto_import = content.contains("import ") && content.contains("_protocol");
        if !has_proto_import {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES403",
                Severity::MEDIUM,
                AesRoleViolation::CapabilityNoProtocol { reason: None },
            ));
            return;
        }

        // ── Collect names imported from _protocol ────────────────
        //    example: import { IPaymentService, ILogger } from '../payment_protocol'
        //    → proto_names = ["IPaymentService", "ILogger"]
        let proto_names: Vec<&str> = content
            .lines()
            .filter(|l| {
                let t = l.trim();
                t.starts_with("import ") && t.contains("_protocol")
            })
            .flat_map(|l| {
                // extract content inside { ... }
                if let Some(start) = l.find('{') {
                    if let Some(end) = l.find('}') {
                        l[start + 1..end]
                            .split(',')
                            .map(|s| {
                                // handle "IPaymentService as IPay"
                                s.trim().split(" as ").next().unwrap_or("").trim()
                            })
                            .filter(|s| !s.is_empty())
                            .collect::<Vec<&str>>()
                    } else {
                        vec![]
                    }
                } else {
                    vec![]
                }
            })
            .collect();

        let lines: Vec<&str> = content.lines().collect();

        // ── Collect all type declarations (class/interface/enum) ────
        //    type alias → not counted; inline parsing, no struct needed
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

                // parse implements
                if let Some(pos) = rest.find("implements ") {
                    let after = &rest[pos + 11..];
                    let before_brace = after.split('{').next().unwrap_or(after);
                    if before_brace
                        .split(',')
                        .any(|imp| proto_names.contains(&imp.trim()))
                    {
                        implementor_found = true;
                    }
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

        // ── RULE 3: max 3 types ──────────────────────────────
        if type_count > 3 {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES403",
                Severity::HIGH,
                AesRoleViolation::CapabilityTooManyTypes {
                    count: type_count,
                    reason: None,
                },
            ));
            return;
        }

        // ── RULE 2: must have ≥ 1 implementor ──────────────────
        //    Implementor = class that has "implements X"
        //    where X is in proto_names (imported from _protocol)
        if !implementor_found {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES403",
                Severity::MEDIUM,
                AesRoleViolation::CapabilityNoImplementor { reason: None },
            ));
        }

        // ── RULE 1: internal class without implements → NOT flagged ──
    }

    fn _check_python_routing(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        // ── Guard: must have _protocol import ──────────────────
        let has_proto_import = (content.contains("import ") || content.contains("from "))
            && content.contains("_protocol");
        if !has_proto_import {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES403",
                Severity::MEDIUM,
                AesRoleViolation::CapabilityNoProtocol { reason: None },
            ));
            return;
        }

        // ── Collect names imported from _protocol ────────────────
        //    example: from shared.some_protocol import ISomeService, IOther
        //    → proto_names = {"ISomeService", "IOther"}
        let proto_names: Vec<&str> = content
            .lines()
            .filter(|l| {
                let t = l.trim();
                (t.starts_with("from ") || t.starts_with("import ")) && t.contains("_protocol")
            })
            .flat_map(|l| {
                // extract the part after "import"
                if let Some(pos) = l.find("import ") {
                    let after = &l[pos + 7..]; // len("import ") = 7
                    after
                        .split(',')
                        .map(|s| s.trim().split(" as ").next().unwrap_or("").trim())
                        .filter(|s| !s.is_empty())
                        .collect::<Vec<&str>>()
                } else {
                    vec![]
                }
            })
            .collect();

        let lines: Vec<&str> = content.lines().collect();

        // ── Collect all classes (parents only) ──────────────────
        struct PyClass<'a> {
            parents: Vec<&'a str>,
        }

        let mut classes: Vec<PyClass> = Vec::new();

        for l in &lines {
            let t = l.trim();
            if !t.starts_with("class ") {
                continue;
            }

            // parse: class Name(Parent1, Parent2):
            let after_class = &t[6..]; // skip "class "
            let name = after_class
                .split(|c: char| ['(', ':', ' '].contains(&c))
                .next()
                .unwrap_or("")
                .trim();

            if name.is_empty() || name.starts_with('_') {
                continue;
            }

            // parse parents from inside (...)
            let parents: Vec<&str> = if let Some(start) = t.find('(') {
                if let Some(end) = t.find(')') {
                    t[start + 1..end]
                        .split(',')
                        .map(|s| s.trim())
                        .filter(|s| !s.is_empty())
                        .collect()
                } else {
                    vec![]
                }
            } else {
                vec![]
            };

            classes.push(PyClass { parents });
        }

        let total_types = classes.len();

        // ── RULE 3: max 3 types ──────────────────────────────
        if total_types > 3 {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES403",
                Severity::HIGH,
                AesRoleViolation::CapabilityTooManyTypes {
                    count: total_types,
                    reason: None,
                },
            ));
            return;
        }

        // ── RULE 2: must have ≥ 1 ABC implementor ─────────────
        //    Implementor = class that inherits from a name
        //    imported from _protocol
        //    example: class PaymentCap(ISomeService):  ← implementor
        let has_implementor = classes
            .iter()
            .any(|c| c.parents.iter().any(|p| proto_names.contains(p)));

        if !has_implementor {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES403",
                Severity::MEDIUM,
                AesRoleViolation::CapabilityNoImplementor { reason: None },
            ));
        }

        // ── RULE 1: internal class without ABC → NOT flagged ──
    }
}
