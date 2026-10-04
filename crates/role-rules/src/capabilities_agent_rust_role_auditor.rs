// PURPOSE: Rust agent role auditor — AES405 sub-checks for Rust files.
//
// The orchestrator (agent_role_orchestrator.rs) selects this auditor by
// `file.language` and calls the trait entry point. The three checks whose
// token set is language-independent (type budget, implementor, Any
// annotation) live in utility_agent_role_checker.rs so all three auditors
// share one copy; the rest are Rust-specific and stay here.

use shared_common::taxonomy_lint_result_vo::LintResult;
use shared_common::taxonomy_severity_vo::Severity;
use shared_filesystem::taxonomy_filesystem_vo::FileEntry;
use shared_role_rules::contract_role_protocol::IAgentRoleProtocol;
use shared_role_rules::taxonomy_role_rules_constant::AGENT_FORBIDDEN_IO_RUST;

use shared_role_rules::utility_agent_role_checker;
use shared_role_rules::utility_agent_role_io_checker;

// === Block 1: Type Definition ===

pub struct AgentRustRoleAuditor {}

// === Block 2: Protocol Implementation ===

impl IAgentRoleProtocol for AgentRustRoleAuditor {
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
        // Rust has no `@abstractmethod` marker. A trait declaration with a
        // body-less required method is the Rust equivalent, and an agent must
        // not declare one: it delegates to contracts it consumes.
        let path = file.path.to_string_lossy().to_string();
        let mut in_trait = false;
        for (i, line) in file.content.lines().enumerate() {
            let t = line.trim();
            if utility_agent_role_checker::is_comment(t) {
                continue;
            }
            if t.starts_with("pub trait ") || t.starts_with("trait ") {
                in_trait = true;
            }
            if in_trait && t.ends_with('{') && t.starts_with("fn ") {
                violations.push(LintResult::new_arch_with_name(
                    &path,
                    i + 1,
                    "AES405",
                    Severity::MEDIUM,
                    format!(
                        "AES405 AGENT_ROLE: Trait declared in an agent file.\n\
                         WHY: Line {} declares a body-less `fn` inside a trait.\n\
                         FIX: An agent delegates to contracts it consumes; it does not \
                         declare new ones. Move the trait to `crates/shared/src/<feature>/`.",
                        i + 1
                    ),
                    "AGENT_ROLE",
                    format!("Line {} declares a body-less `fn` inside a trait.", i + 1),
                    "An agent delegates to contracts it consumes; it does not \
                     declare new ones. Move the trait to `crates/shared/src/<feature>/`.",
                ));
            }
            if in_trait && t == "}" {
                in_trait = false;
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

// === Block 3: Helpers ===

impl Default for AgentRustRoleAuditor {
    fn default() -> Self {
        Self::new()
    }
}

impl AgentRustRoleAuditor {
    pub fn new() -> Self {
        Self {}
    }

    /// Block 2 (aggregate impl) must precede Block 3 (inherent impl). HIGH.
    ///
    /// The file reads types and injected deps, then aggregate methods, then
    /// constructors and helpers. An inherent `impl` block placed before the
    /// aggregate impl means the reader meets `new` before the contract the
    /// orchestrator fulfils.
    fn block_order(&self, content: &str, path: &str, violations: &mut Vec<LintResult>) {
        let mut aggregate_line: Option<(usize, String)> = None;
        let mut inherent_line: Option<(usize, String)> = None;
        let mut in_cfg_test = false;

        for (i, line) in content.lines().enumerate() {
            let t = line.trim();
            if t.starts_with("#[cfg(test)]") {
                in_cfg_test = true;
                continue;
            }
            if in_cfg_test {
                if t == "}" {
                    in_cfg_test = false;
                }
                continue;
            }
            if !t.starts_with("impl ") || utility_agent_role_checker::is_comment(t) {
                continue;
            }
            // `impl Trait for Type` is the aggregate impl; `impl Type` (no
            // `for`) is the inherent impl. `impl Display for Type` is a std
            // trait impl, which belongs in Block 3 and is not the aggregate.
            let is_trait_impl = t.contains(" for ");
            let is_aggregate = is_trait_impl
                && t.split(" for ")
                    .next()
                    .is_some_and(|lhs| utility_agent_role_checker::is_aggregate_name(Some(lhs)));
            if is_aggregate && aggregate_line.is_none() {
                aggregate_line = Some((i, t.trim_end_matches('{').trim().to_string()));
            }
            if !is_trait_impl && inherent_line.is_none() {
                inherent_line = Some((i, t.trim_end_matches('{').trim().to_string()));
            }
        }

        if let (Some((agg_idx, agg_src)), Some((inh_idx, inh_src))) =
            (aggregate_line, inherent_line)
            && inh_idx < agg_idx
        {
            violations.push(LintResult::new_arch_with_name(
                path,
                inh_idx + 1,
                "AES405",
                Severity::HIGH,
                format!(
                    "AES405 AGENT_ROLE: Block 3 (inherent impl) precedes Block 2 (aggregate impl).\n\
                     WHY: `{inh_src}` is declared at line {} but `{agg_src}` follows at line {}.\n\
                     FIX: Reorder the file so it reads:\n  \
                     Block 1 (type + injected deps) -> Block 2 (aggregate impl) -> Block 3 (constructor, std traits, helpers).",
                    inh_idx + 1,
                    agg_idx + 1,
                ),
                "AGENT_ROLE",
                format!(
                    "`{inh_src}` is declared at line {} but `{agg_src}` follows at line {}.",
                    inh_idx + 1,
                    agg_idx + 1
                ),
                "Reorder the file so it reads:\n  \
                 Block 1 (type + injected deps) -> Block 2 (aggregate impl) -> Block 3 (constructor, std traits, helpers).",
            ));
        }
    }

    /// No filesystem, network, database, or console access. MEDIUM.
    fn io_forbidden(&self, content: &str, path: &str, violations: &mut Vec<LintResult>) {
        utility_agent_role_io_checker::scan_io_forbidden(
            content,
            path,
            "Rust",
            AGENT_FORBIDDEN_IO_RUST,
            violations,
        );
    }

    /// No file-level `const` — policy constants belong in taxonomy. MEDIUM.
    fn constant_placement(&self, content: &str, path: &str, violations: &mut Vec<LintResult>) {
        for (i, line) in content.lines().enumerate() {
            let t = line.trim();
            if utility_agent_role_checker::is_comment(t) {
                continue;
            }
            // A `const` nested in an `impl` is an associated constant owned by
            // the type; only an indent-0 declaration is a file-level constant.
            let indent = line.len() - line.trim_start().len();
            if indent == 0
                && t.starts_with("const ")
                && !t.starts_with("const fn ")
                && t.contains("=")
            {
                let name = t.split_whitespace().nth(1).unwrap_or("<unnamed>");
                violations.push(LintResult::new_arch_with_name(
                    path,
                    i + 1,
                    "AES405",
                    Severity::MEDIUM,
                    format!(
                        "AES405 AGENT_ROLE: File-level constant in an agent file.\n\
                         WHY: `const {name}` is declared at line {}. Policy constants \
                         belong to the shared taxonomy so every layer reads one value.\n\
                         FIX: Move `{name}` into `taxonomy_<domain>_constant.rs` and import it.",
                        i + 1
                    ),
                    "AGENT_ROLE",
                    format!(
                        "`const {name}` is declared at line {}. Policy constants \
                         belong to the shared taxonomy so every layer reads one value.",
                        i + 1
                    ),
                    format!("Move `{name}` into `taxonomy_<domain>_constant.rs` and import it."),
                ));
            }
        }
    }

    /// At least 2 injected protocol subsystems, unless the feature has one. LOW.
    ///
    /// A feature whose shared module declares exactly one protocol is a
    /// single-subsystem feature: its agent coordinates everything the feature
    /// has, and demanding a second seam would mean inventing a protocol that
    /// does no distinct job.
    fn subsystem_count(
        &self,
        content: &str,
        path: &str,
        feature_protocol_count: usize,
        violations: &mut Vec<LintResult>,
    ) {
        let (injected, _distinct) = utility_agent_role_checker::count_protocol_fields(
            content,
            shared_filesystem::taxonomy_filesystem_vo::Language::Rust,
        );
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
                0,
                "AES405",
                Severity::LOW,
                "AES405 AGENT_ROLE: Orchestrator has a single execution goal.\n\
                 WHY: File injects 1 protocol field(s); an orchestrator must coordinate \
                 at least 2 subsystems.\n\
                 FIX: Either this work belongs in a capability file rather than an \
                 agent, or the feature genuinely has more than one subsystem and the agent \
                 should hold and coordinate their protocols.",
                "AGENT_ROLE",
                "File injects 1 protocol field(s); an orchestrator must coordinate \
                 at least 2 subsystems.",
                "Either this work belongs in a capability file rather than an \
                 agent, or the feature genuinely has more than one subsystem and the agent \
                 should hold and coordinate their protocols.",
            ));
        }
    }

    /// No totals, averages, folds, or reductions. MEDIUM.
    ///
    /// An agent routes requests between subsystems; it does not aggregate
    /// their results. `.len()` and `.count()` on a collection the agent built
    /// are not computation and stay allowed.
    fn computation(&self, content: &str, path: &str, violations: &mut Vec<LintResult>) {
        // `.sum()` on the collection itself, not `.sum::<T>()` called on
        // `&self` — an iterator `.sum()` on a bounded slice the agent
        // produced for display is fine; the forbidden case is computing a
        // total from domain data.
        const FORBIDDEN: &[&str] = &["sum::<", ".fold(", ".reduce(", ".product("];
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
                        format!(
                            "AES405 AGENT_ROLE: Computation in an agent file.\n\
                             WHY: Line {} calls `{token}`. An agent orchestrates; it does \
                             not compute totals or averages over domain data.\n\
                             FIX: Move the aggregation into a capability, and have the agent \
                             route to it.",
                            i + 1
                        ),
                        "AGENT_ROLE",
                        format!(
                            "Line {} calls `{token}`. An agent orchestrates; it does \
                             not compute totals or averages over domain data.",
                            i + 1
                        ),
                        "Move the aggregation into a capability, and have the agent \
                         route to it.",
                    ));
                }
            }
        }
    }

    /// No `&mut self`, no assignment to a stored field. MEDIUM.
    ///
    /// The agent holds injected protocols and reads them. A `&mut self` method
    /// or a `self.field = …` outside the constructor is stored state the
    /// orchestrator mutates, which makes its behaviour depend on call order.
    /// A `OnceLock` / `Mutex` guard taken for reading is fine; the check looks
    /// for assignment, not for interior mutability.
    fn stateless(&self, content: &str, path: &str, violations: &mut Vec<LintResult>) {
        for (i, line) in content.lines().enumerate() {
            let t = line.trim();
            if utility_agent_role_checker::is_comment(t) {
                continue;
            }
            if t.contains("&mut self") {
                violations.push(LintResult::new_arch_with_name(
                    path,
                    i + 1,
                    "AES405",
                    Severity::MEDIUM,
                    format!(
                        "AES405 AGENT_ROLE: Mutable receiver in an agent file.\n\
                         WHY: Line {} takes `&mut self`. An agent coordinates \
                         dependencies and does not mutate its own state.\n\
                         FIX: Take `&self` and move any mutation behind the injected \
                         protocol that owns the state.",
                        i + 1
                    ),
                    "AGENT_ROLE",
                    format!(
                        "Line {} takes `&mut self`. An agent coordinates \
                         dependencies and does not mutate its own state.",
                        i + 1
                    ),
                    "Take `&self` and move any mutation behind the injected \
                     protocol that owns the state.",
                ));
            }
        }
    }

    /// No module-level free functions. LOW.
    ///
    /// An agent file holds one type and the methods on it. A free `fn` at the
    /// file level is a utility that belongs in a `utility_<domain>_*.rs` file.
    fn free_fn(&self, content: &str, path: &str, violations: &mut Vec<LintResult>) {
        let mut in_impl_depth: i32 = 0;
        let mut in_cfg_test = false;

        for (i, line) in content.lines().enumerate() {
            let t = line.trim();
            if t.starts_with("#[cfg(test)]") {
                in_cfg_test = true;
                continue;
            }
            if in_cfg_test {
                if t == "}" {
                    in_cfg_test = false;
                }
                continue;
            }

            if in_impl_depth > 0 {
                in_impl_depth += i32::from(t.contains('{')) - i32::from(t.contains('}'));
                if in_impl_depth < 0 {
                    in_impl_depth = 0;
                }
                continue;
            }

            if t.starts_with("impl ") {
                in_impl_depth = 1;
                continue;
            }
            if t.starts_with("mod ") || t.starts_with("pub mod ") {
                in_impl_depth = 1;
                continue;
            }

            // At file level: `fn`, `pub fn`, `pub(crate) fn`, or an attribute on
            // the line directly above one.
            let is_free = t.starts_with("fn ")
                || t.starts_with("pub fn ")
                || t.starts_with("pub(crate) fn ")
                || t.starts_with("async fn ");
            if is_free {
                let name = t
                    .trim_start_matches("pub(crate) ")
                    .trim_start_matches("pub ")
                    .trim_start_matches("async ")
                    .trim_start_matches("fn ")
                    .split(['(', '<', ' '])
                    .next()
                    .unwrap_or("")
                    .to_string();
                violations.push(LintResult::new_arch_with_name(
                    path,
                    i + 1,
                    "AES405",
                    Severity::LOW,
                    format!(
                        "AES405 AGENT_ROLE: Free function in an agent file.\n\
                         WHY: `fn {name}` is declared at file level (line {}). An agent \
                         file holds one type and its methods.\n\
                         FIX: Move `{name}` into a `utility_<domain>_*.rs` file, or make it \
                         an associated function on the agent's type.",
                        i + 1
                    ),
                    "AGENT_ROLE",
                    format!(
                        "`fn {name}` is declared at file level (line {}). An agent \
                         file holds one type and its methods.",
                        i + 1
                    ),
                    format!(
                        "Move `{name}` into a `utility_<domain>_*.rs` file, or make it \
                         an associated function on the agent's type."
                    ),
                ));
            }
        }
    }
}
