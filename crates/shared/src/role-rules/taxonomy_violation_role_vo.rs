// PURPOSE: AesRoleViolation — violation messages for role rules (AES401-406)
use crate::common::taxonomy_language_vo::Language;
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
        #[allow(deprecated)]
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
        AesRoleViolation::CapabilityNoProtocol { reason } => {
            let why = resolve_why(
                reason,
                "file has 'capabilities_' prefix but no _protocol import — this file is \
                 broken/useless. Either it is not a real capability (rename or delete), or \
                 a proper contract protocol requirement has not been created yet (create the \
                 protocol first, then implement it here)",
            );
            write!(
                f,
                "AES403 CAPABILITY_ROLE: Capabilities file has no _protocol implementation.\n\
                        WHY? {why}\n\
                        FIX: Rename the file if it is not a capability, delete if obsolete, \
                        or create the required contract protocol first then implement it here."
            )
        }
        AesRoleViolation::CapabilityNoImplementor { reason } => {
            let why = resolve_why(
                reason,
                "At least one struct must implement a _protocol trait (impl Trait for Struct). \
                 Internal helper structs are allowed.",
            );
            write!(
                f,
                "AES403 CAPABILITY_ROLE: No struct implements a _protocol trait.\n\
                        WHY? {why}\n\
                        FIX: At least one struct in this file must implement the capability \
                        _protocol. Convert an existing struct or keep only internal helpers."
            )
        }
        AesRoleViolation::CapabilityTooManyTypes { count, reason } => {
            let why = resolve_why(
                reason,
                "Max 3 types (struct/enum) allowed in capabilities. \
                 Refactor excess types to taxonomy layer.",
            );
            write!(
                f,
                "AES403 CAPABILITY_ROLE: Too many types ({count} struct/enum) in capabilities file.\n\
                        WHY? {why}\n\
                        FIX: Keep at most 3 types. Move excess structs/enums to the taxonomy layer."
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
        AesRoleViolation::UtilityRole { reason } => {
            let why = resolve_why(
                reason,
                "file has 'utility_' prefix but does not contain stateless standalone functions — \
                 this file may be misplaced. Utility files must contain only pure, stateless \
                 functions that depend only on taxonomy.",
            );
            write!(
                f,
                "AES404 UTILITY_ROLE: Utility file does not follow utility layer conventions.\n\
                        WHY? {why}\n\
                        FIX: Ensure the file contains only stateless standalone functions. \
                        If this is not a utility file, rename it to use the correct layer prefix. \
                        If obsolete, delete the file and remove its module declaration."
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
        AesRoleViolation::AgentNoAggregate { reason } => {
            let why = resolve_why(
                reason,
                "file has 'agent_' prefix but no _aggregate import — this file is \
                 broken/useless. Either it is not a real agent (rename or delete), or \
                 a proper aggregate contract has not been created yet (create the \
                 aggregate first, then implement it here)",
            );
            write!(
                f,
                "AES405 AGENT_ROLE: Agent file has no _aggregate implementation.\n\
                        WHY? {why}\n\
                        FIX: Rename the file if it is not an agent, delete if obsolete, \
                        or create the required aggregate contract first then implement it here."
            )
        }
        AesRoleViolation::AgentNoImplementor { reason } => {
            let why = resolve_why(
                reason,
                "At least one struct must implement an _aggregate trait (impl Trait for Struct). \
                 Internal helper structs are allowed.",
            );
            write!(
                f,
                "AES405 AGENT_ROLE: No struct implements an _aggregate trait.\n\
                        WHY? {why}\n\
                        FIX: At least one struct in this file must implement the agent \
                        _aggregate. Convert an existing struct or keep only internal helpers."
            )
        }
        AesRoleViolation::AgentTooManyTypes { count, names, reason } => {
            let names_str: Vec<String> = names.iter().map(|n| n.to_string()).collect();
            let names_list = names_str.join(", ");
            let why = resolve_why(
                reason,
                "Max 3 types (struct/enum) allowed in agent files. \
                 Refactor excess types to taxonomy layer.",
            );
            write!(
                f,
                "AES405 AGENT_ROLE: Too many types ({count} struct/enum) in agent file: [{names_list}].\n\
                        WHY? {why}\n\
                        FIX: Keep at most 3 types. Move excess structs/enums to the taxonomy layer."
            )
        }
        AesRoleViolation::HighLevelPolicy { reason } => {
            let why = resolve_why(
                reason,
                "Agents must focus on high-level orchestration policies and not import \
                 concrete implementations directly.",
            );
            write!(
                f,
                "AES405 AGENT_ROLE: Low-level implementation details imported.\n\
                        WHY? {why}\n\
                        FIX: Reference components using their contract interfaces instead of \
                        concrete types."
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
    /// Deprecated: superseded by `CapabilityNoProtocol`, `CapabilityNoImplementor`, and
    /// `CapabilityTooManyTypes`. Kept for backward compatibility with old reports.
    #[deprecated(since = "1.10.106", note = "Use CapabilityNoProtocol, CapabilityNoImplementor, or CapabilityTooManyTypes")]
    CapabilityRouting {
        struct_name: SymbolName,
        reason: Option<LintMessage>,
    },
    CapabilityNoProtocol {
        reason: Option<LintMessage>,
    },
    /// No struct implementing the protocol/port trait
    CapabilityNoImplementor {
        reason: Option<LintMessage>,
    },
    /// Number of structs + enums exceeds 3
    CapabilityTooManyTypes {
        count: usize,
        reason: Option<LintMessage>,
    },
    SingleBottleneck {
        reason: Option<LintMessage>,
    },
    // AES404 — Utility role
    UtilityRole {
        reason: Option<LintMessage>,
    },
    // AES405 — Agent role
    AgentNoAggregate {
        reason: Option<LintMessage>,
    },
    AgentNoImplementor {
        reason: Option<LintMessage>,
    },
    AgentTooManyTypes {
        count: usize,
        names: Vec<SymbolName>,
        reason: Option<LintMessage>,
    },
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
