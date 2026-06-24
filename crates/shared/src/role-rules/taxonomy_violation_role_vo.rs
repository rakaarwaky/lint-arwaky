// PURPOSE: AesRoleViolation — violation messages for role rules (AES401-406)
use crate::code_analysis::taxonomy_violation_code_analysis_vo::Language;
use crate::common::taxonomy_message_vo::LintMessage;
use crate::common::taxonomy_name_vo::SymbolName;
use std::fmt;

pub struct LabeledRoleViolation {
    violation: AesRoleViolation,
    lang: Language,
}

/// Resolve `reason` to the user-facing "why" string. Falls back to a
/// language-aware default message when no reason was supplied by the auditor.
fn resolve_why<S: Into<String>>(reason: &Option<LintMessage>, default: S) -> String {
    match reason.as_ref() {
        Some(r) => r.to_string(),
        None => default.into(),
    }
}

/// Write the violation body for `v` using `lang` for language-aware wording.
/// Both `Display` impls (`AesRoleViolation` and `LabeledRoleViolation`) route
/// through here so the message templates live in exactly one place per variant.
fn write_violation(
    f: &mut fmt::Formatter<'_>,
    v: &AesRoleViolation,
    lang: Language,
) -> fmt::Result {
    match v {
        AesRoleViolation::ConstantPurity { reason } => {
            let why = resolve_why(
                reason,
                "Constant taxonomy modules must only contain pure constant or static values \
                 to maintain value-level immutability.",
            );
            write!(
                f,
                "AES401 TAXONOMY_ROLE: Constant file contains non-constant declaration.\n\
                        WHY? {why}\n\
                        FIX: Move the non-constant code to the appropriate layer, or convert it \
                        to a constant/static declaration."
            )
        }
        AesRoleViolation::PrimitiveUsage { primitive, reason } => {
            let why = resolve_why(
                reason,
                format!(
                    "Direct primitive types (like '{primitive}') are forbidden in taxonomy \
                     entities, errors, and events to maintain strict value object boundaries \
                     and avoid primitive obsession."
                ),
            );
            write!(
                f,
                "AES401 TAXONOMY_ROLE: Direct primitive '{primitive}' in taxonomy entity, \
                        error, or event.\n\
                        WHY? {why}\n\
                        FIX: Replace the primitive type with a domain Value Object (VO) or \
                        constant from the taxonomy layer."
            )
        }
        AesRoleViolation::ContractPrimitive { reason } => {
            let default = format!(
                "Contracts must enforce value object boundaries to prevent primitive obsession. \
                 Use {} instead of primitives.",
                lang.type_kw()
            );
            let why = resolve_why(reason, default);
            write!(
                f,
                "AES402 CONTRACT_PRIMITIVE: Contract {} or method signature uses primitive \
                        types instead of taxonomy VO or constant.\n\
                        WHY? {why}\n\
                        FIX: Replace primitive types with appropriate Value Objects (VO) or \
                        constants from the taxonomy layer.",
                lang.interface_kw()
            )
        }
        AesRoleViolation::CapabilityRouting {
            struct_name,
            reason,
        } => {
            let default = format!(
                "Capability {}s must implement their corresponding {} traits/interfaces to \
                 ensure clean interface boundaries.",
                lang.struct_keyword(),
                lang.interface_kw()
            );
            let why = resolve_why(reason, default);
            write!(
                f,
                "AES403 CAPABILITY_ROLE: {} '{struct_name}' has no {} implementation.\n\
                        WHY? {why}\n\
                        FIX: Implement the capability protocol {} for '{struct_name}'.",
                lang.struct_keyword(),
                lang.interface_kw(),
                lang.interface_kw()
            )
        }
        AesRoleViolation::SingleBottleneck { reason } => {
            let why = resolve_why(
                reason,
                "Routing all commands to a single capability violates high-level decomposition \
                 and creates a single bottleneck.",
            );
            write!(
                f,
                "AES403 CAPABILITY_ROLE: All orchestrator dispatch routes route to a single \
                        capability.\n\
                        WHY? {why}\n\
                        FIX: Distribute logic or route commands to multiple distinct capabilities."
            )
        }
        AesRoleViolation::InfrastructureNoPort { reason } => {
            let why = resolve_why(
                reason,
                "Infrastructure adapters must implement their corresponding port interfaces.",
            );
            write!(
                f,
                "AES404 INFRASTRUCTURE_ROLE: Infrastructure file has no port trait/protocol \
                        implementation.\n\
                        WHY? {why}\n\
                        FIX: Implement the corresponding port or protocol interface in this \
                        infrastructure adapter."
            )
        }
        AesRoleViolation::StatelessExecution { reason } => {
            let why = resolve_why(
                reason,
                "Agent execution components must be stateless to guarantee reentrancy and \
                 prevent side effects.",
            );
            write!(
                f,
                "AES405 AGENT_ROLE: Non-stateless behavior detected.\n\
                        WHY? {why}\n\
                        FIX: Remove mutable class state assignments or move initialization \
                        logic to the constructor."
            )
        }
        AesRoleViolation::HighLevelPolicy { reason } => {
            let why = resolve_why(
                reason,
                "Agents must focus on high-level orchestration policies and not import \
                 infrastructure adapters directly.",
            );
            write!(
                f,
                "AES405 AGENT_ROLE: Low-level implementation details imported.\n\
                        WHY? {why}\n\
                        FIX: Reference components using their contract interfaces instead of \
                        concrete infrastructure types."
            )
        }
        AesRoleViolation::CoordinatesMultiple { reason } => {
            let why = resolve_why(
                reason,
                "Orchestrator agents exist to coordinate multiple subsystems; simple \
                 single-component logic belongs elsewhere.",
            );
            write!(
                f,
                "AES405 AGENT_ROLE: Orchestrator coordinates too few subsystems.\n\
                        WHY? {why}\n\
                        FIX: Merge this simple flow into its caller or delegate at least two \
                        subsystems to this orchestrator."
            )
        }
        AesRoleViolation::NoDomainLogic { reason } => {
            let why = resolve_why(
                reason,
                "Complex domain logic detected in a passive agent role or surface wrapper.",
            );
            write!(
                f,
                "AES405 AGENT_ROLE: Complex domain logic detected in a passive role.\n\
                        WHY? {why}\n\
                        FIX: Move the complex domain/control logic into capabilities or \
                        orchestrator components."
            )
        }
        AesRoleViolation::LazyEagerInit { reason } => {
            let why = resolve_why(
                reason,
                "Agent containers must only declare and wire dependencies, avoiding complex \
                 logic in constructors.",
            );
            write!(
                f,
                "AES405 AGENT_ROLE: Complex initialization logic found in container module.\n\
                        WHY? {why}\n\
                        FIX: Move the initialization/conditional logic out of the constructor \
                        or container setup."
            )
        }
        AesRoleViolation::MustImplementContract { reason } => {
            let default = format!(
                "Agent containers must implement the 'ServiceContainerAggregate' {} to satisfy \
                 dependency injection protocols.",
                lang.interface_kw()
            );
            let why = resolve_why(reason, default);
            write!(
                f,
                "AES405 AGENT_ROLE: Class is missing required contract implementation.\n\
                        WHY? {why}\n\
                        FIX: Add the 'ServiceContainerAggregate' implementation for the \
                        container class."
            )
        }
        AesRoleViolation::AnyType { reason } => {
            let why = resolve_why(
                reason,
                "Using 'any' or 'Any' type annotations bypasses type safety and violates \
                 agent-level domain-driven design.",
            );
            write!(
                f,
                "AES405 AGENT_ROLE: Forbidden 'any' type annotation found.\n\
                        WHY? {why}\n\
                        FIX: Replace 'any' annotations with strongly-typed objects, \
                        structures, or domain Value Objects (VO)."
            )
        }
        AesRoleViolation::AgentFileSizeLimit { max_lines } => write!(
            f,
            "AES405 AGENT_ROLE: Agent file exceeds {max_lines} lines.\n\
                    WHY? Agent files must remain compact to preserve role clarity.\n\
                    FIX: Split the orchestrator/container into smaller focused modules."
        ),
        AesRoleViolation::PassiveViolation { reason } => {
            let why = resolve_why(
                reason,
                "Passive surfaces must not contain logic that should be in capabilities or \
                 agents.",
            );
            write!(
                f,
                "AES406 SURFACE_ROLE: Passive surface contains business logic.\n\
                        WHY? {why}\n\
                        FIX: Move logic to appropriate capability or agent."
            )
        }
        AesRoleViolation::SurfaceRoleViolation { reason } => {
            let why = resolve_why(
                reason,
                "Surface role violation - surfaces must adhere to their designated role \
                 (command, controller, component, hook, etc.).",
            );
            write!(
                f,
                "AES406 SURFACE_ROLE: Surface role boundary violation.\n\
                        WHY? {why}\n\
                        FIX: Ensure surface only performs its designated responsibilities."
            )
        }
    }
}

impl AesRoleViolation {
    pub fn with_language(self, lang: Language) -> LabeledRoleViolation {
        LabeledRoleViolation {
            violation: self,
            lang,
        }
    }
}

#[derive(Debug, Clone)]
pub enum AesRoleViolation {
    // AES401 — Taxonomy role
    ConstantPurity {
        reason: Option<LintMessage>,
    },
    PrimitiveUsage {
        primitive: SymbolName,
        reason: Option<LintMessage>,
    },
    // AES402 — Contract primitive
    ContractPrimitive {
        reason: Option<LintMessage>,
    },
    // AES403 — Capability role
    CapabilityRouting {
        struct_name: SymbolName,
        reason: Option<LintMessage>,
    },
    SingleBottleneck {
        reason: Option<LintMessage>,
    },
    // AES404 — Infrastructure role
    InfrastructureNoPort {
        reason: Option<LintMessage>,
    },
    // AES405 — Agent role
    StatelessExecution {
        reason: Option<LintMessage>,
    },
    HighLevelPolicy {
        reason: Option<LintMessage>,
    },
    CoordinatesMultiple {
        reason: Option<LintMessage>,
    },
    NoDomainLogic {
        reason: Option<LintMessage>,
    },
    LazyEagerInit {
        reason: Option<LintMessage>,
    },
    MustImplementContract {
        reason: Option<LintMessage>,
    },
    AnyType {
        reason: Option<LintMessage>,
    },
    AgentFileSizeLimit {
        max_lines: usize,
    },
    // AES406 — Surface role
    PassiveViolation {
        reason: Option<LintMessage>,
    },
    SurfaceRoleViolation {
        reason: Option<LintMessage>,
    },
}

impl fmt::Display for AesRoleViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_violation(f, self, Language::Rust)
    }
}

impl fmt::Display for LabeledRoleViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_violation(f, &self.violation, self.lang)
    }
}

impl From<AesRoleViolation> for String {
    fn from(v: AesRoleViolation) -> String {
        v.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn labeled(v: AesRoleViolation, lang: Language) -> LabeledRoleViolation {
        v.with_language(lang)
    }

    /// `LabeledRoleViolation::Display` must produce the same output as the
    /// underlying `AesRoleViolation::Display` when the language is Rust,
    /// because Rust is the implicit default for the bare `AesRoleViolation`
    /// Display impl. Guards against accidental drift between the two impls.
    #[test]
    fn labeled_rust_matches_bare_display() {
        let v = AesRoleViolation::CoordinatesMultiple {
            reason: Some(LintMessage::new("custom".to_string())),
        };
        assert_eq!(
            v.to_string(),
            labeled(v.clone(), Language::Rust).to_string()
        );
    }

    /// `LabeledRoleViolation` with a non-Rust language must swap language-
    /// sensitive tokens (e.g. Python uses "Protocol", JS uses "interface").
    /// Guards against hard-coded Rust tokens leaking into non-Rust labels.
    #[test]
    fn labeled_python_uses_protocol_token() {
        let v = AesRoleViolation::ContractPrimitive {
            reason: Some(LintMessage::new("custom".to_string())),
        };
        let out = labeled(v, Language::Python).to_string();
        assert!(
            out.contains("Protocol"),
            "expected Protocol token in: {out}"
        );
        assert!(
            !out.contains("trait"),
            "must not contain Rust trait in: {out}"
        );
    }

    /// When `reason` is `None`, the per-variant default WHY message must be
    /// used. Confirms `resolve_why` does not emit `None` / `Some(...)`.
    #[test]
    fn missing_reason_uses_default_why() {
        let v = AesRoleViolation::SingleBottleneck { reason: None };
        let out = v.to_string();
        assert!(
            out.contains("single bottleneck"),
            "default WHY missing in: {out}"
        );
        assert!(!out.contains("None"), "leaked None in: {out}");
    }

    /// When `reason` is `Some`, the auditor-supplied WHY must override the
    /// per-variant default.
    #[test]
    fn present_reason_overrides_default_why() {
        let v = AesRoleViolation::SingleBottleneck {
            reason: Some(LintMessage::new("auditor-custom".to_string())),
        };
        let out = v.to_string();
        assert!(
            out.contains("auditor-custom"),
            "custom WHY missing in: {out}"
        );
    }

    /// `AgentFileSizeLimit` carries no `reason`; its display must embed the
    /// numeric `max_lines`.
    #[test]
    fn agent_file_size_limit_includes_max_lines() {
        let v = AesRoleViolation::AgentFileSizeLimit { max_lines: 250 };
        let out = v.to_string();
        assert!(out.contains("250"), "max_lines not in output: {out}");
    }
}
