// PURPOSE: TypeScript agent role auditor — AES405 sub-checks for TS/JS files.
//
// The type budget, implementor, and Any-annotation checks delegate to
// utility_agent_role_checker.rs so all three auditors share one copy.
// The language-specific checks stay here.

use shared_common::taxonomy_lint_vo::LintResult;
use shared_common::taxonomy_severity_vo::Severity;
use shared_filesystem::taxonomy_filesystem_vo::{FileEntry, Language};
use shared_role_rules::contract_role_protocol::IAgentRoleProtocol;
use shared_role_rules::taxonomy_role_rules_constant::AGENT_FORBIDDEN_IO_TYPESCRIPT;

use shared_role_rules::utility_agent_role_checker;
use shared_role_rules::utility_agent_role_io_checker;

// ─── Block 1: Module Declaration ────────────────────────────

pub struct AgentTsRoleAuditor {}

impl AgentTsRoleAuditor {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for AgentTsRoleAuditor {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Block 2: Module Contract ────────────────

impl IAgentRoleProtocol for AgentTsRoleAuditor {
    fn check_agent_routing(&self, file: &FileEntry, layer: &str, violations: &mut Vec<LintResult>) {
        if !utility_agent_role_checker::is_agent_layer(layer) {
            return;
        }
        // Composition sub-checks only; the orchestrator calls the remaining
        // rules individually so a caller can target a specific rule.
        utility_agent_role_checker::check_type_budget(file, violations);
        utility_agent_role_checker::check_implementor(file, violations);
        utility_agent_role_checker::check_any_annotation(file, violations);
    }

    fn check_agent_implementor(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        utility_agent_role_checker::check_implementor(file, violations);
    }

    fn check_agent_type_budget(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        utility_agent_role_checker::check_type_budget(file, violations);
    }

    fn check_agent_any_annotation(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        utility_agent_role_checker::check_any_annotation(file, violations);
    }

    fn check_agent_block_order(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        self.block_order(&file.content, &file.path.to_string_lossy(), violations);
    }

    fn check_agent_protocol_forbidden(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        utility_agent_role_checker::check_agent_protocol_forbidden(file, violations);
    }

    fn check_agent_block_markers(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        utility_agent_role_checker::check_block_markers(file, violations);
    }

    fn check_agent_constructor_block1(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        utility_agent_role_checker::check_constructor_in_block1(file, violations);
    }

    fn check_agent_io_forbidden(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        self.io_forbidden(&file.content, &file.path.to_string_lossy(), violations);
    }

    fn check_agent_constant_placement(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        self.constant_placement(&file.content, &file.path.to_string_lossy(), violations);
    }

    fn check_agent_subsystem_count(
        &self,
        file: &FileEntry,
        feature_protocol_count: usize,
        violations: &mut Vec<LintResult>,
    ) {
        self.subsystem_count(
            &file.content,
            &file.path.to_string_lossy(),
            feature_protocol_count,
            violations,
        );
    }

    fn check_agent_computation(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        self.computation(&file.content, &file.path.to_string_lossy(), violations);
    }

    fn check_agent_abstract_method(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        // TS has no decorator equivalent; an interface declaring a method with
        // no body, or an abstract class, is the TS equivalent.
        let path = file.path.to_string_lossy().to_string();
        for (i, line) in file.content.lines().enumerate() {
            let t = line.trim();
            if utility_agent_role_checker::is_comment(t) {
                continue;
            }
            if t.starts_with("abstract class ") || t.starts_with("abstract class") {
                violations.push(LintResult::new_arch_with_name(
                    &path,
                    i + 1,
                    "AES405",
                    Severity::MEDIUM,
                    format!("Abstract class at line {} in a TypeScript agent file; agents delegate to contracts", i + 1),
                    "AGENT_ROLE_ABSTRACT_METHOD",
                    "An agent delegates to protocols it consumes; it does not declare new abstract contracts.",
                    "Move the interface to the feature's protocol file in the shared layer.",
                ));
            }
        }
    }

    fn check_agent_stateless(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        self.stateless(&file.content, &file.path.to_string_lossy(), violations);
    }

    fn check_agent_free_fn(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        self.free_fn(&file.content, &file.path.to_string_lossy(), violations);
    }
}

// ─── Block 3: Module Helpers ────────────

impl AgentTsRoleAuditor {
    /// Block 2 (protocol-delegating methods) must precede Block 3
    /// (constructor/`toString`/`toJSON`). The violation is detected when a
    /// `constructor` appears before the first method that calls a protocol.
    fn block_order(&self, content: &str, path: &str, violations: &mut Vec<LintResult>) {
        let lines: Vec<&str> = content.lines().collect();
        let mut first_protocol_method: Option<usize> = None;
        let mut constructor_line: Option<usize> = None;

        for (i, line) in lines.iter().enumerate() {
            let t = line.trim();
            if utility_agent_role_checker::is_comment(t) {
                continue;
            }
            if first_protocol_method.is_none() && t.starts_with("public ") && t.contains("(") {
                let body = lines.get(i + 1).copied().unwrap_or("");
                if body.contains("this.") || body.contains("protocol") {
                    first_protocol_method = Some(i);
                }
            }
            if constructor_line.is_none() && t.starts_with("constructor(") {
                constructor_line = Some(i);
            }
        }

        if let (Some(proto_idx), Some(ctor_idx)) = (first_protocol_method, constructor_line)
            && ctor_idx < proto_idx
        {
            violations.push(LintResult::new_arch_with_name(
                path,
                ctor_idx + 1,
                "AES405",
                Severity::HIGH,
                "Constructor precedes protocol-delegating methods in an agent class".to_string(),
                "AGENT_ROLE_BLOCK_ORDER",
                "Block 2 (protocol-delegating methods) must come before Block 3 (constructor) so the reader meets the orchestration contract first.",
                "Order the class so protocol-delegating methods come before the constructor.",
            ));
        }
    }

    fn io_forbidden(&self, content: &str, path: &str, violations: &mut Vec<LintResult>) {
        utility_agent_role_io_checker::scan_io_forbidden(
            content,
            path,
            "TypeScript",
            AGENT_FORBIDDEN_IO_TYPESCRIPT,
            violations,
        );
    }

    fn constant_placement(&self, content: &str, path: &str, violations: &mut Vec<LintResult>) {
        for (i, line) in content.lines().enumerate() {
            let t = line.trim();
            if utility_agent_role_checker::is_comment(t) {
                continue;
            }
            // A top-level `const X: Type = ...` (not inside a class, not `export const`
            // which is a module export, not a function parameter).
            let indent = line.len() - line.trim_start().len();
            if indent == 0
                && t.starts_with("const ")
                && !t.starts_with("export const ")
                && t.contains(":")
                && t.contains("=")
            {
                let name = t
                    .strip_prefix("const ")
                    .unwrap_or("")
                    .split([':', '='])
                    .next()
                    .unwrap_or("")
                    .trim();
                if !name.is_empty() {
                    violations.push(LintResult::new_arch_with_name(
                        path,
                        i + 1,
                        "AES405",
                        Severity::MEDIUM,
                        format!("Top-level constant `{name}` in a TypeScript agent file; policy constants belong in taxonomy"),
                        "AGENT_ROLE_CONSTANT_PLACEMENT",
                        "Policy constants belong to the shared taxonomy so every layer reads one value.",
                        format!("Move `{name}` into the feature's taxonomy module and import it."),
                    ));
                }
            }
        }
    }

    fn subsystem_count(
        &self,
        content: &str,
        path: &str,
        feature_protocol_count: usize,
        violations: &mut Vec<LintResult>,
    ) {
        let (injected, _distinct) =
            utility_agent_role_checker::count_protocol_fields(content, Language::TypeScript);
        // A feature whose shared module declares exactly one protocol is a
        // single-subsystem feature — but only when the agent actually injects
        // one seam. Four seams of the same protocol type are still
        // coordinate-able distinct subsystems, so the skip requires exactly
        // one injected seam.
        if feature_protocol_count == 1 && injected == 1 {
            return;
        }
        if injected < 2 {
            violations.push(LintResult::new_arch_with_name(
                path,
                crate::WHOLE_FILE,
                "AES405",
                Severity::LOW,
                "Agent injects fewer than two capability protocols, so it coordinates one subsystem".to_string(),
                "AGENT_ROLE_SUBSYSTEM_COUNT",
                "A feature whose shared module declares exactly one protocol is a genuine single-subsystem feature; forcing a second seam adds one that does no distinct job. When nothing injects a second protocol seam, this work belongs in a capability rather than an agent.",
                "Either this work belongs in a capability file rather than an agent, or inject the protocols of both subsystems into the agent.",
            ));
        }
    }

    fn computation(&self, content: &str, path: &str, violations: &mut Vec<LintResult>) {
        const FORBIDDEN: &[&str] = &[".reduce(", ".reduceRight(", "Math."];
        for (i, line) in content.lines().enumerate() {
            let t = line.trim();
            if utility_agent_role_checker::is_comment(t) {
                continue;
            }
            for token in FORBIDDEN {
                if t.contains(token) {
                    violations.push(LintResult::new_arch_with_name(
                        path,
                        i + 1,
                        "AES405",
                        Severity::MEDIUM,
                        format!("Computation in a TypeScript agent file at line {}; an agent orchestrates, it does not compute", i + 1),
                        "AGENT_ROLE_COMPUTATION",
                        "An agent orchestrates requests between subsystems; it does not aggregate domain totals.",
                        "Move the aggregation into a capability.",
                    ));
                }
            }
        }
    }

    fn stateless(&self, content: &str, path: &str, violations: &mut Vec<LintResult>) {
        let lines: Vec<&str> = content.lines().collect();
        let mut in_ctor = false;
        let mut ctor_depth: i32 = 0;

        for (i, line) in lines.iter().enumerate() {
            let t = line.trim();
            if utility_agent_role_checker::is_comment(t) {
                continue;
            }

            if !in_ctor && t.starts_with("constructor(") {
                in_ctor = true;
                ctor_depth = 0;
                ctor_depth += i32::from(t.contains('{')) - i32::from(t.contains('}'));
                continue;
            }
            if in_ctor {
                ctor_depth += i32::from(t.contains('{')) - i32::from(t.contains('}'));
                if ctor_depth <= 0 {
                    in_ctor = false;
                }
                continue;
            }

            // Flag `this.x = ...` outside the constructor.
            let Some(rest) = t.strip_prefix("this.") else {
                continue;
            };
            let Some(eq_pos) = rest.find('=') else {
                continue;
            };
            if matches!(rest[..eq_pos].chars().last(), Some('=' | '!' | '<' | '>')) {
                continue;
            }
            let _field = rest[..eq_pos].trim_end_matches(['+', '-', '*', '/', '%']);
            violations.push(LintResult::new_arch_with_name(
                path,
                i + 1,
                "AES405",
                Severity::MEDIUM,
                format!("State assignment outside constructor at line {}; an agent holds no mutable state", i + 1),
                "AGENT_ROLE_STATELESS",
                "An agent holds no mutable state; assignments outside the constructor make behaviour depend on call order.",
                "Initialize the field in the constructor and append to it; move domain data behind a protocol.",
            ));
        }
    }

    fn free_fn(&self, content: &str, path: &str, violations: &mut Vec<LintResult>) {
        let mut in_class = false;
        let mut class_depth: i32 = 0;

        for (i, line) in content.lines().enumerate() {
            let t = line.trim();
            if utility_agent_role_checker::is_comment(t) {
                continue;
            }
            let indent = line.len() - line.trim_start().len();

            if t.starts_with("class ") || t.starts_with("export class ") {
                in_class = true;
                class_depth = 0;
                continue;
            }
            if in_class {
                class_depth += i32::from(t.contains('{')) - i32::from(t.contains('}'));
                if class_depth <= 0 && t.contains('}') {
                    in_class = false;
                }
                continue;
            }

            // Module-level `function` or `const fn`
            if indent == 0 {
                let is_fn = t.starts_with("function ")
                    || t.starts_with("export function ")
                    || t.starts_with("async function ")
                    || (t.starts_with("const ") && t.contains("=>"));
                if is_fn {
                    let name = t
                        .trim_start_matches("export ")
                        .trim_start_matches("async ")
                        .trim_start_matches("const ")
                        .trim_start_matches("function ")
                        .split(['(', ':', ' ', '='])
                        .next()
                        .unwrap_or("")
                        .trim();
                    if !name.is_empty() {
                        violations.push(LintResult::new_arch_with_name(
                            path,
                            i + 1,
                            "AES405",
                            Severity::LOW,
                            format!("Module-level function `{name}` in a TypeScript agent file; it belongs in a utility file"),
                            "AGENT_ROLE_FREE_FUNCTION",
                            "An agent file holds one type and its methods; a free function is a utility that belongs elsewhere.",
                            "Move it into a `utility_<domain>.ts` file, or make it a method on the agent's class.".to_string(),
                        ));
                    }
                }
            }
        }
    }
}
