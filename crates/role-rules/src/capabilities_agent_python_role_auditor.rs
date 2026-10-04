// PURPOSE: Python agent role auditor — AES405 sub-checks for Python files.
//
// The type budget, implementor, and Any-annotation checks delegate to
// utility_agent_role_checker.rs so all three auditors share one copy.
// The language-specific checks (block order, I/O, constant placement,
// stateless, free functions, abstract methods, computation) stay here.

use shared_common::taxonomy_lint_result_vo::LintResult;
use shared_common::taxonomy_severity_vo::Severity;
use shared_filesystem::taxonomy_filesystem_vo::{FileEntry, Language};
use shared_role_rules::contract_role_protocol::IAgentRoleProtocol;
use shared_role_rules::taxonomy_role_rules_constant::AGENT_FORBIDDEN_IO_PYTHON;

use shared_role_rules::utility_agent_role_checker;
use shared_role_rules::utility_agent_role_io_checker;

// ─── Block 1: Struct Definition ────────────────────────────

pub struct AgentPythonRoleAuditor {}

// ─── Block 2: Protocol Trait Implementation ────────────────

impl IAgentRoleProtocol for AgentPythonRoleAuditor {
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
        self.abstract_method(&file.content, &file.path.to_string_lossy(), violations);
    }

    fn check_agent_stateless(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        self.stateless(&file.content, &file.path.to_string_lossy(), violations);
    }

    fn check_agent_free_fn(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        self.free_fn(&file.content, &file.path.to_string_lossy(), violations);
    }
}

// ─── Block 3: Constructors, Std Traits, Helpers ────────────

impl Default for AgentPythonRoleAuditor {
    fn default() -> Self {
        Self::new()
    }
}

impl AgentPythonRoleAuditor {
    pub fn new() -> Self {
        Self {}
    }

    /// Block 2 (aggregate methods) must precede Block 3 (factory/dunder).
    fn block_order(&self, content: &str, path: &str, violations: &mut Vec<LintResult>) {
        // For Python classes: methods that are `@classmethod` / `@staticmethod`
        // (factories/dunders) must come after protocol-delegating methods.
        // The simplest detectable violation: `@classmethod` or `__init__`
        // appearing before any `def` that calls a protocol method.
        let lines: Vec<&str> = content.lines().collect();
        let mut first_protocol_method: Option<usize> = None;
        let mut first_factory: Option<usize> = None;

        for (i, line) in lines.iter().enumerate() {
            let t = line.trim();
            if utility_agent_role_checker::is_comment(t) {
                continue;
            }
            if first_protocol_method.is_none() && t.starts_with("def ") && !t.starts_with("def __")
            {
                // Check the next few lines for a protocol call
                let body = lines.get(i + 1).copied().unwrap_or("");
                if body.contains("self.") && !body.contains("def ") {
                    first_protocol_method = Some(i);
                }
            }
            if first_factory.is_none()
                && (t.starts_with("@classmethod")
                    || t.starts_with("@staticmethod")
                    || (t.starts_with("def __") && !t.starts_with("def __init__")))
            {
                first_factory = Some(i);
            }
        }

        if let (Some(proto_idx), Some(factory_idx)) = (first_protocol_method, first_factory)
            && factory_idx < proto_idx
        {
            violations.push(LintResult::new_arch(
                path,
                factory_idx + 1,
                "AES405",
                Severity::HIGH,
                format!(
                    "AES405 AGENT_ROLE: Factory/dunder methods precede protocol-delegating methods.\n\
                     WHY? Line {} of {path} declares a `@classmethod`/`@staticmethod`/dunder before \
                     the first protocol-delegating method at line {}.\n\
                     HOW TO FIX? Order the class so that Block 2 (protocol-delegating methods) \
                     comes before Block 3 (factories/dunders).",
                    factory_idx + 1,
                    proto_idx + 1,
                ),
            ));
        }
    }

    fn io_forbidden(&self, content: &str, path: &str, violations: &mut Vec<LintResult>) {
        utility_agent_role_io_checker::scan_io_forbidden(
            content,
            path,
            "Python",
            AGENT_FORBIDDEN_IO_PYTHON,
            violations,
        );
    }

    fn constant_placement(&self, content: &str, path: &str, violations: &mut Vec<LintResult>) {
        let lines: Vec<&str> = content.lines().collect();
        let mut in_class = false;
        let mut class_indent = 0usize;

        for (i, line) in lines.iter().enumerate() {
            let t = line.trim();
            if utility_agent_role_checker::is_comment(t) {
                continue;
            }
            let indent = line.len() - line.trim_start().len();

            if t.starts_with("class ") {
                in_class = true;
                class_indent = indent;
                continue;
            }
            if in_class && indent > 0 && indent <= class_indent && !t.starts_with(' ') {
                in_class = false;
            }
            // A module-level assignment (indent 0, not a def/class/import)
            if !in_class
                && indent == 0
                && t.contains('=')
                && !t.starts_with("def ")
                && !t.starts_with("class ")
                && !t.starts_with("import ")
                && !t.starts_with("from ")
                && !t.starts_with("if ")
                && !t.starts_with("elif ")
                && !t.starts_with("#")
            {
                let name = t.split('=').next().unwrap_or("").trim();
                if !name.is_empty() && name.chars().next().is_some_and(char::is_uppercase) {
                    violations.push(LintResult::new_arch(
                        path,
                        i + 1,
                        "AES405",
                        Severity::MEDIUM,
                        format!(
                            "AES405 AGENT_ROLE: Module-level constant in a Python agent file.\n\
                             WHY? `{name}` is declared at line {} of {path}. Policy constants \
                             belong in the taxonomy layer.\n\
                             HOW TO FIX? Move `{name}` into the feature's taxonomy module and import it.",
                            i + 1,
                        ),
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
            utility_agent_role_checker::count_protocol_fields(content, Language::Python);
        // A feature whose shared module declares exactly one protocol is a
        // single-subsystem feature — but only when the agent actually injects
        // one seam. Four seams of the same protocol type are still
        // coordinate-able distinct subsystems, so the skip requires exactly
        // one injected seam.
        if feature_protocol_count == 1 && injected == 1 {
            return;
        }
        if injected < 2 {
            violations.push(LintResult::new_arch(
                path,
                0,
                "AES405",
                Severity::LOW,
                format!(
                    "AES405 AGENT_ROLE: Orchestrator has a single execution goal.\n\
                     WHY? {path} injects {} protocol field(s) in `__init__`; an orchestrator must \
                     coordinate at least 2 subsystems.\n\
                     HOW TO FIX? Either this work belongs in a capability file rather than an agent, \
                     or inject the protocols of both subsystems into `__init__`.",
                    injected
                ),
            ));
        }
    }

    fn computation(&self, content: &str, path: &str, violations: &mut Vec<LintResult>) {
        const FORBIDDEN: &[&str] = &["sum(", "reduce(", "fold("];
        for (i, line) in content.lines().enumerate() {
            let t = line.trim();
            if utility_agent_role_checker::is_comment(t) {
                continue;
            }
            for token in FORBIDDEN {
                if t.contains(token) {
                    violations.push(LintResult::new_arch(
                        path,
                        i + 1,
                        "AES405",
                        Severity::MEDIUM,
                        format!(
                            "AES405 AGENT_ROLE: Computation in a Python agent file.\n\
                             WHY? Line {} of {path} calls `{token}`. An agent orchestrates; it does \
                             not compute totals or averages over domain data.\n\
                             HOW TO FIX? Move the aggregation into a capability.",
                            i + 1
                        ),
                    ));
                }
            }
        }
    }

    fn abstract_method(&self, content: &str, path: &str, violations: &mut Vec<LintResult>) {
        for (i, line) in content.lines().enumerate() {
            let t = line.trim();
            if utility_agent_role_checker::is_comment(t) {
                continue;
            }
            if t == "@abstractmethod" {
                violations.push(LintResult::new_arch(
                    path,
                    i + 1,
                    "AES405",
                    Severity::MEDIUM,
                    format!(
                        "AES405 AGENT_ROLE: Abstract method in a Python agent file.\n\
                         WHY? Line {} of {path} uses `@abstractmethod`. An agent delegates to \
                         contracts it consumes; it does not declare new abstract contracts.\n\
                         HOW TO FIX? Move the abstract method to the feature's protocol ABC in \
                         `contract_<feature>_protocol.py`.",
                        i + 1
                    ),
                ));
            }
        }
    }

    /// No `self.x = ...` outside `__init__`. MEDIUM.
    ///
    /// A collection field the constructor initialised and that later methods
    /// `.append()` to is a results buffer, not stored state, so appends are
    /// allowed. A direct reassignment — including `+=`, `-=`, and the
    /// comparison operators, which share the `=` character — is reported.
    fn stateless(&self, content: &str, path: &str, violations: &mut Vec<LintResult>) {
        let lines: Vec<&str> = content.lines().collect();
        let mut in_init = false;
        let mut init_indent = 0usize;

        for (i, line) in lines.iter().enumerate() {
            let t = line.trim();
            if utility_agent_role_checker::is_comment(t) {
                continue;
            }
            let indent = line.len() - line.trim_start().len();

            if !in_init && t.starts_with("def __init__") {
                in_init = true;
                init_indent = indent;
                continue;
            }
            // A sibling `def` or a new class at the same or lower indent ends
            // the constructor block.
            if in_init
                && (t.starts_with("def ") || t.starts_with("class "))
                && indent <= init_indent
            {
                in_init = false;
            }
            if in_init {
                continue;
            }

            let Some(rest) = t.strip_prefix("self.") else {
                continue;
            };
            let Some(eq_pos) = rest.find('=') else {
                continue;
            };
            // `==`, `!=`, `<=`, `>=` and `=>` are comparisons, not assignments.
            if matches!(rest[..eq_pos].chars().last(), Some('=' | '!' | '<' | '>')) {
                continue;
            }
            let field = rest[..eq_pos].trim_end_matches(['+', '-', '*', '/', '%']);
            violations.push(LintResult::new_arch(
                path,
                i + 1,
                "AES405",
                Severity::MEDIUM,
                format!(
                    "AES405 AGENT_ROLE: State assignment outside `__init__`.\n\
                     WHY? Line {} of {path} assigns `self.{field}` outside the constructor.\n\
                     HOW TO FIX? An agent holds no mutable state. If this is a results buffer, \
                     initialise the field in `__init__` and append to it; if it is domain data, \
                     move it behind an injected protocol.",
                    i + 1
                ),
            ));
        }
    }

    fn free_fn(&self, content: &str, path: &str, violations: &mut Vec<LintResult>) {
        let lines: Vec<&str> = content.lines().collect();
        let mut in_class = false;
        let mut class_indent = 0usize;

        for (i, line) in lines.iter().enumerate() {
            let t = line.trim();
            if utility_agent_role_checker::is_comment(t) {
                continue;
            }
            let indent = line.len() - line.trim_start().len();

            if t.starts_with("class ") {
                in_class = true;
                class_indent = indent;
                continue;
            }
            if in_class && indent > 0 && indent <= class_indent {
                in_class = false;
            }

            // A `def` at module level (not inside a class)
            if !in_class && indent == 0 && t.starts_with("def ") {
                let name = t
                    .strip_prefix("def ")
                    .unwrap_or("")
                    .split(['(', ':'])
                    .next()
                    .unwrap_or("")
                    .trim()
                    .to_string();
                violations.push(LintResult::new_arch(
                    path,
                    i + 1,
                    "AES405",
                    Severity::LOW,
                    format!(
                        "AES405 AGENT_ROLE: Module-level function in a Python agent file.\n\
                         WHY? `def {name}` is declared at line {} of {path} at module level.\n\
                         HOW TO FIX? Move it into a `utility_<domain>_<name>.py` file, or make it a \
                         method on the agent's class.",
                        i + 1,
                    ),
                ));
            }
        }
    }
}
