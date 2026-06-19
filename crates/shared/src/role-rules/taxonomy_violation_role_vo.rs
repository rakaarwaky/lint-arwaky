// PURPOSE: AesRoleViolation — violation messages for role rules (AES401-406)
use crate::code_analysis::taxonomy_violation_code_analysis_vo::Language;
use crate::common::taxonomy_message_vo::LintMessage;
use crate::common::taxonomy_name_vo::SymbolName;
use std::fmt;

pub struct LabeledRoleViolation {
    violation: AesRoleViolation,
    lang: Language,
}

impl fmt::Display for LabeledRoleViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lang = self.lang;
        match &self.violation {
            AesRoleViolation::ConstantPurity { reason } => {
                let default_why = "Constant taxonomy modules must only contain pure constant or static values to maintain value-level immutability.".to_string();
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(f, "AES401 TAXONOMY_ROLE: Constant file contains non-constant declaration.\n\
                        WHY? {}\n\
                        FIX: Move the non-constant code to the appropriate layer, or convert it to a constant/static declaration.", why)
            }
            AesRoleViolation::PrimitiveUsage { primitive, reason } => {
                let default_why = format!("Direct primitive types (like '{}') are forbidden in taxonomy entities, errors, and events to maintain strict value object boundaries and avoid primitive obsession.", primitive);
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(f, "AES401 TAXONOMY_ROLE: Direct primitive '{}' in taxonomy entity, error, or event.\n\
                        WHY? {}\n\
                        FIX: Replace the primitive type with a domain Value Object (VO) or constant from the taxonomy layer.", primitive, why)
            }
            AesRoleViolation::ContractPrimitive { reason } => {
                let default_why = format!("Contracts must enforce value object boundaries to prevent primitive obsession. Use {} instead of primitives.", lang.type_kw());
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(f, "AES402 CONTRACT_PRIMITIVE: Contract {} or method signature uses primitive types instead of taxonomy VO or constant.\n\
                        WHY? {}\n\
                        FIX: Replace primitive types with appropriate Value Objects (VO) or constants from the taxonomy layer.", lang.interface_kw(), why)
            }
            AesRoleViolation::CapabilityRouting {
                struct_name,
                reason,
            } => {
                let default_why = format!("Capability {}s must implement their corresponding {} traits/interfaces to ensure clean interface boundaries.", lang.struct_keyword(), lang.interface_kw());
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(
                    f,
                    "AES403 CAPABILITY_ROLE: {} '{}' has no {} implementation.\n\
                        WHY? {}\n\
                        FIX: Implement the capability protocol {} for '{}'.",
                    lang.struct_keyword(),
                    struct_name,
                    lang.interface_kw(),
                    why,
                    lang.interface_kw(),
                    struct_name
                )
            }
            AesRoleViolation::SingleBottleneck { reason } => {
                let default_why = "Routing all commands to a single capability violates high-level decomposition and creates a single bottleneck.".to_string();
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(f, "AES403 CAPABILITY_ROLE: All orchestrator dispatch routes route to a single capability.\n\
                        WHY? {}\n\
                        FIX: Distribute logic or route commands to multiple distinct capabilities.", why)
            }
            AesRoleViolation::InfrastructureNoPort { reason } => {
                let default_why =
                    "Infrastructure adapters must implement their corresponding port interfaces."
                        .to_string();
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(f, "AES404 INFRASTRUCTURE_ROLE: Infrastructure file has no port trait/protocol implementation.\n\
                        WHY? {}\n\
                        FIX: Implement the corresponding port or protocol interface in this infrastructure adapter.", why)
            }
            AesRoleViolation::StatelessExecution { reason } => {
                let default_why = "Agent execution components must be stateless to guarantee reentrancy and prevent side effects.".to_string();
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(f, "AES405 AGENT_ROLE: Non-stateless behavior detected.\n\
                        WHY? {}\n\
                        FIX: Remove mutable class state assignments or move initialization logic to the constructor.", why)
            }
            AesRoleViolation::HighLevelPolicy { reason } => {
                let default_why = "Agents must focus on high-level orchestration policies and not import infrastructure adapters directly.".to_string();
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(f, "AES405 AGENT_ROLE: Low-level implementation details imported.\n\
                        WHY? {}\n\
                        FIX: Reference components using their contract interfaces instead of concrete infrastructure types.", why)
            }
            AesRoleViolation::CoordinatesMultiple { reason } => {
                let default_why = "Orchestrator agents exist to coordinate multiple subsystems; simple single-component logic belongs elsewhere.".to_string();
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(f, "AES405 AGENT_ROLE: Orchestrator coordinates too few subsystems.\n\
                        WHY? {}\n\
                        FIX: Merge this simple flow into its caller or delegate at least two subsystems to this orchestrator.", why)
            }
            AesRoleViolation::NoDomainLogic { reason } => {
                let default_why =
                    "Complex domain logic detected in a passive agent role or surface wrapper."
                        .to_string();
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(f, "AES405 AGENT_ROLE: Complex domain logic detected in a passive role.\n\
                        WHY? {}\n\
                        FIX: Move the complex domain/control logic into capabilities or orchestrator components.", why)
            }
            AesRoleViolation::LazyEagerInit { reason } => {
                let default_why = "Agent containers must only declare and wire dependencies, avoiding complex logic in constructors.".to_string();
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(f, "AES405 AGENT_ROLE: Complex initialization logic found in container module.\n\
                        WHY? {}\n\
                        FIX: Move the initialization/conditional logic out of the constructor or container setup.", why)
            }
            AesRoleViolation::MustImplementContract { reason } => {
                let default_why = format!("Agent containers must implement the 'ServiceContainerAggregate' {} to satisfy dependency injection protocols.", lang.interface_kw());
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(f, "AES405 AGENT_ROLE: Class is missing required contract implementation.\n\
                        WHY? {}\n\
                        FIX: Add the 'ServiceContainerAggregate' implementation for the container class.", why)
            }
            AesRoleViolation::AnyType { reason } => {
                let default_why = "Using 'any' or 'Any' type annotations bypasses type safety and violates agent-level domain-driven design.".to_string();
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(f, "AES405 AGENT_ROLE: Forbidden 'any' type annotation found.\n\
                        WHY? {}\n\
                        FIX: Replace 'any' annotations with strongly-typed objects, structures, or domain Value Objects (VO).", why)
            }
            AesRoleViolation::AgentFileSizeLimit { max_lines } => {
                write!(f, "AES405 AGENT_ROLE: Agent file exceeds {} lines.\n\
                        WHY? Agent files must remain compact to preserve role clarity.\n\
                        FIX: Split the orchestrator/container into smaller focused modules.", max_lines)
            }
            AesRoleViolation::PassiveViolation { reason } => {
                let default_why =
                    "Passive surfaces must not contain logic that should be in capabilities or agents."
                        .to_string();
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(
                    f,
                    "AES406 SURFACE_ROLE: Passive surface contains business logic.\n\
                        WHY? {}\n\
                        FIX: Move logic to appropriate capability or agent.",
                    why
                )
            }
            AesRoleViolation::SurfaceRoleViolation { reason } => {
                let default_why = "Surface role violation - surfaces must adhere to their designated role (command, controller, component, hook, etc.).".to_string();
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(
                    f,
                    "AES406 SURFACE_ROLE: Surface role boundary violation.\n\
                        WHY? {}\n\
                        FIX: Ensure surface only performs its designated responsibilities.",
                    why
                )
            }
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
        let lang = Language::Rust;
        match self {
            AesRoleViolation::ConstantPurity { reason } => {
                let default_why = "Constant taxonomy modules must only contain pure constant or static values to maintain value-level immutability.".to_string();
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(f, "AES401 TAXONOMY_ROLE: Constant file contains non-constant declaration.\n\
                        WHY? {}\n\
                        FIX: Move the non-constant code to the appropriate layer, or convert it to a constant/static declaration.", why)
            }
            AesRoleViolation::PrimitiveUsage { primitive, reason } => {
                let default_why = format!("Direct primitive types (like '{}') are forbidden in taxonomy entities, errors, and events to maintain strict value object boundaries and avoid primitive obsession.", primitive);
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(f, "AES401 TAXONOMY_ROLE: Direct primitive '{}' in taxonomy entity, error, or event.\n\
                        WHY? {}\n\
                        FIX: Replace the primitive type with a domain Value Object (VO) or constant from the taxonomy layer.", primitive, why)
            }
            AesRoleViolation::ContractPrimitive { reason } => {
                let default_why = format!("Contracts must enforce value object boundaries to prevent primitive obsession. Use {} instead of primitives.", lang.type_kw());
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(f, "AES402 CONTRACT_PRIMITIVE: Contract {} or method signature uses primitive types instead of taxonomy VO or constant.\n\
                        WHY? {}\n\
                        FIX: Replace primitive types with appropriate Value Objects (VO) or constants from the taxonomy layer.", lang.interface_kw(), why)
            }
            AesRoleViolation::CapabilityRouting {
                struct_name,
                reason,
            } => {
                let default_why = format!("Capability {}s must implement their corresponding {} traits/interfaces to ensure clean interface boundaries.", lang.struct_keyword(), lang.interface_kw());
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(
                    f,
                    "AES403 CAPABILITY_ROLE: {} '{}' has no {} implementation.\n\
                        WHY? {}\n\
                        FIX: Implement the capability protocol {} for '{}'.",
                    lang.struct_keyword(),
                    struct_name,
                    lang.interface_kw(),
                    why,
                    lang.interface_kw(),
                    struct_name
                )
            }
            AesRoleViolation::SingleBottleneck { reason } => {
                let default_why = "Routing all commands to a single capability violates high-level decomposition and creates a single bottleneck.".to_string();
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(f, "AES403 CAPABILITY_ROLE: All orchestrator dispatch routes route to a single capability.\n\
                        WHY? {}\n\
                        FIX: Distribute logic or route commands to multiple distinct capabilities.", why)
            }
            AesRoleViolation::InfrastructureNoPort { reason } => {
                let default_why =
                    "Infrastructure adapters must implement their corresponding port interfaces."
                        .to_string();
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(f, "AES404 INFRASTRUCTURE_ROLE: Infrastructure file has no port trait/protocol implementation.\n\
                        WHY? {}\n\
                        FIX: Implement the corresponding port or protocol interface in this infrastructure adapter.", why)
            }
            AesRoleViolation::StatelessExecution { reason } => {
                let default_why = "Agent execution components must be stateless to guarantee reentrancy and prevent side effects.".to_string();
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(f, "AES405 AGENT_ROLE: Non-stateless behavior detected.\n\
                        WHY? {}\n\
                        FIX: Remove mutable class state assignments or move initialization logic to the constructor.", why)
            }
            AesRoleViolation::HighLevelPolicy { reason } => {
                let default_why = "Agents must focus on high-level orchestration policies and not import infrastructure adapters directly.".to_string();
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(f, "AES405 AGENT_ROLE: Low-level implementation details imported.\n\
                        WHY? {}\n\
                        FIX: Reference components using their contract interfaces instead of concrete infrastructure types.", why)
            }
            AesRoleViolation::CoordinatesMultiple { reason } => {
                let default_why = "Orchestrator agents exist to coordinate multiple subsystems; simple single-component logic belongs elsewhere.".to_string();
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(f, "AES405 AGENT_ROLE: Orchestrator coordinates too few subsystems.\n\
                        WHY? {}\n\
                        FIX: Merge this simple flow into its caller or delegate at least two subsystems to this orchestrator.", why)
            }
            AesRoleViolation::NoDomainLogic { reason } => {
                let default_why =
                    "Complex domain logic detected in a passive agent role or surface wrapper."
                        .to_string();
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(f, "AES405 AGENT_ROLE: Complex domain logic detected in a passive role.\n\
                        WHY? {}\n\
                        FIX: Move the complex domain/control logic into capabilities or orchestrator components.", why)
            }
            AesRoleViolation::LazyEagerInit { reason } => {
                let default_why = "Agent containers must only declare and wire dependencies, avoiding complex logic in constructors.".to_string();
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(f, "AES405 AGENT_ROLE: Complex initialization logic found in container module.\n\
                        WHY? {}\n\
                        FIX: Move the initialization/conditional logic out of the constructor or container setup.", why)
            }
            AesRoleViolation::MustImplementContract { reason } => {
                let default_why = format!("Agent containers must implement the 'ServiceContainerAggregate' {} to satisfy dependency injection protocols.", lang.interface_kw());
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(f, "AES405 AGENT_ROLE: Class is missing required contract implementation.\n\
                        WHY? {}\n\
                        FIX: Add the 'ServiceContainerAggregate' implementation for the container class.", why)
            }
            AesRoleViolation::AnyType { reason } => {
                let default_why = "Using 'any' or 'Any' type annotations bypasses type safety and violates agent-level domain-driven design.".to_string();
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(f, "AES405 AGENT_ROLE: Forbidden 'any' type annotation found.\n\
                        WHY? {}\n\
                        FIX: Replace 'any' annotations with strongly-typed objects, structures, or domain Value Objects (VO).", why)
            }
            AesRoleViolation::AgentFileSizeLimit { max_lines } => {
                write!(f, "AES405 AGENT_ROLE: Agent file exceeds {} lines.\n\
                        WHY? Agent files must remain compact to preserve role clarity.\n\
                        FIX: Split the orchestrator/container into smaller focused modules.", max_lines)
            }
            AesRoleViolation::PassiveViolation { reason } => {
                let default_why =
                    "Passive surfaces must not contain logic that should be in capabilities or agents."
                        .to_string();
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(
                    f,
                    "AES406 SURFACE_ROLE: Passive surface contains business logic.\n\
                        WHY? {}\n\
                        FIX: Move logic to appropriate capability or agent.",
                    why
                )
            }
            AesRoleViolation::SurfaceRoleViolation { reason } => {
                let default_why = "Surface role violation - surfaces must adhere to their designated role (command, controller, component, hook, etc.).".to_string();
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(
                    f,
                    "AES406 SURFACE_ROLE: Surface role boundary violation.\n\
                        WHY? {}\n\
                        FIX: Ensure surface only performs its designated responsibilities.",
                    why
                )
            }
        }
    }
}

impl From<AesRoleViolation> for String {
    fn from(v: AesRoleViolation) -> String {
        v.to_string()
    }
}
