// PURPOSE: AgentRoleChecker — AES405: enforce agent type composition
//          and forbid any-type annotations.
//
// ALGORITHM:
//   check_agent_routing (per language):
//     1. Guard: file must import from a _aggregate module.
//        If missing → flag AgentNoAggregate.
//     2. Collect all type declarations (struct/enum/class/interface).
//        Skip #[cfg(test)] blocks (Rust).
//     3. Rule 3 — Max 3 types. If exceeded → flag AgentTooManyTypes.
//     4. Rule 2 — At least 1 implementor required:
//          Rust:   impl <Trait> for <Struct>  (trait from _aggregate)
//          Python: class <Name>(<AggregateABC>):
//          TS:     class <Name> implements <IAggregate>
//        If none → flag AgentNoImplementor.
//     5. Rule 1 — Internal helper types without aggregate impl are ALLOWED.
//
//   check_any_type_annotation:
//     Line-by-line scan for `: any`, `: Any`, `-> any`, `-> Any`,
//     `Any<`, `Any[`, `any[` patterns. Flags each as AES405 AnyType.

use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::common::taxonomy_severity_vo::Severity;
use shared::common::utility_language_detector::detect_language_info_from_source;
use shared::role_rules::contract_agent_role_protocol::IAgentRoleChecker;
use shared::role_rules::taxonomy_violation_role_vo::AesRoleViolation;
use shared::taxonomy_name_vo::SymbolName;
use shared::taxonomy_source_vo::SourceContentVO;

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
    }

    // ─── Rust ──────────────────────────────────────────────

    fn _check_rust_routing(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        // Guard: must have _aggregate import
        let has_aggregate_import =
            content.contains("use ") && content.contains("_aggregate::");
        if !has_aggregate_import {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES405",
                Severity::MEDIUM,
                AesRoleViolation::AgentNoAggregate { reason: None },
            ));
            return;
        }

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
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES405",
                Severity::HIGH,
                AesRoleViolation::AgentTooManyTypes {
                    count: type_names.len(),
                    names: type_names.iter().map(|s| SymbolName::new(*s)).collect(),
                    reason: None,
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
                AesRoleViolation::AgentNoImplementor { reason: None },
            ));
        }

        // Rule 1: internal structs without aggregate impl are NOT flagged
    }

    // ─── TypeScript / JavaScript ───────────────────────────

    fn _check_ts_routing(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        // Guard: must have _aggregate import
        let has_aggregate_import =
            content.contains("import ") && content.contains("_aggregate");
        if !has_aggregate_import {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES405",
                Severity::MEDIUM,
                AesRoleViolation::AgentNoAggregate { reason: None },
            ));
            return;
        }

        // Collect names imported from _aggregate
        let aggregate_names: Vec<&str> = content
            .lines()
            .filter(|l| {
                let t = l.trim();
                t.starts_with("import ") && t.contains("_aggregate")
            })
            .flat_map(|l| {
                if let Some(start) = l.find('{') {
                    if let Some(end) = l.find('}') {
                        l[start + 1..end]
                            .split(',')
                            .map(|s| s.trim().split(" as ").next().unwrap_or("").trim())
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

        let mut type_names: Vec<&str> = Vec::new();
        let mut implementor_found = false;

        for l in content.lines() {
            let t = l.trim();

            // detect class
            let class_body = t
                .strip_prefix("export class ")
                .or_else(|| t.strip_prefix("class "));
            if let Some(rest) = class_body {
                let name = rest
                    .split(|c: char| c == ' ' || c == '(' || c == '{')
                    .next()
                    .unwrap_or("")
                    .trim();
                if name.is_empty() || name.starts_with('_') {
                    continue;
                }
                type_names.push(name);

                if let Some(pos) = rest.find("implements ") {
                    let after = &rest[pos + 11..];
                    let before_brace = after.split('{').next().unwrap_or(after);
                    let implements: Vec<&str> = before_brace
                        .split(',')
                        .map(|s| s.trim())
                        .filter(|s| !s.is_empty())
                        .collect();
                    if implements.iter().any(|imp| aggregate_names.contains(imp)) {
                        implementor_found = true;
                    }
                }
                continue;
            }

            // detect interface
            let iface_body = t
                .strip_prefix("export interface ")
                .or_else(|| t.strip_prefix("interface "));
            if let Some(rest) = iface_body {
                let name = rest
                    .split(|c: char| c == ' ' || c == '{' || c == '<')
                    .next()
                    .unwrap_or("")
                    .trim();
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
                let name = rest
                    .split(|c: char| c == ' ' || c == '{')
                    .next()
                    .unwrap_or("")
                    .trim();
                if !name.is_empty() && !name.starts_with('_') {
                    type_names.push(name);
                }
            }
        }

        // Rule 3: max 3 types
        if type_names.len() > 3 {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES405",
                Severity::HIGH,
                AesRoleViolation::AgentTooManyTypes {
                    count: type_names.len(),
                    names: type_names.iter().map(|s| SymbolName::new(*s)).collect(),
                    reason: None,
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
                AesRoleViolation::AgentNoImplementor { reason: None },
            ));
        }

        // Rule 1: internal class without implements → NOT flagged
    }

    // ─── Python ────────────────────────────────────────────

    fn _check_python_routing(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        // Guard: must have _aggregate import
        let has_aggregate_import =
            (content.contains("import ") || content.contains("from "))
                && content.contains("_aggregate");
        if !has_aggregate_import {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES405",
                Severity::MEDIUM,
                AesRoleViolation::AgentNoAggregate { reason: None },
            ));
            return;
        }

        // Collect names imported from _aggregate
        let aggregate_names: Vec<&str> = content
            .lines()
            .filter(|l| {
                let t = l.trim();
                (t.starts_with("from ") || t.starts_with("import "))
                    && t.contains("_aggregate")
            })
            .flat_map(|l| {
                if let Some(pos) = l.find("import ") {
                    let after = &l[pos + 7..];
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

        let mut type_names: Vec<&str> = Vec::new();
        let mut implementor_found = false;

        for l in content.lines() {
            let t = l.trim();
            if !t.starts_with("class ") {
                continue;
            }

            // skip "class " prefix
            let after_class = &t[6..];
            let name = after_class
                .split(|c: char| c == '(' || c == ':' || c == ' ')
                .next()
                .unwrap_or("")
                .trim();
            if name.is_empty() || name.starts_with('_') {
                continue;
            }
            type_names.push(name);

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

            if parents.iter().any(|p| aggregate_names.contains(p)) {
                implementor_found = true;
            }
        }

        // Rule 3: max 3 types
        if type_names.len() > 3 {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES405",
                Severity::HIGH,
                AesRoleViolation::AgentTooManyTypes {
                    count: type_names.len(),
                    names: type_names.iter().map(|s| SymbolName::new(*s)).collect(),
                    reason: None,
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
                AesRoleViolation::AgentNoImplementor { reason: None },
            ));
        }

        // Rule 1: internal class without ABC → NOT flagged
    }
}
