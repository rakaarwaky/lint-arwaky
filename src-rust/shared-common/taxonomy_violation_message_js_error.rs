// PURPOSE: AesViolationJs — AES violation messages for JavaScript/TypeScript analysis (enum with Display)
use crate::shared_common::taxonomy_layer_vo::LayerNameVO;
use crate::shared_common::taxonomy_name_vo::SymbolName;
use crate::shared_common::taxonomy_message_vo::LintMessage;
use std::fmt;

pub enum AesViolationJs {
    // AES001 — Import rules
    ForbiddenImport {
        source_layer: LayerNameVO,
        forbidden_layer: LayerNameVO,
        allowed: Vec<LayerNameVO>,
        reason: Option<LintMessage>,
    },
    // AES002 — Mandatory import
    MissingImport {
        source_layer: LayerNameVO,
        required: SymbolName,
        reason: Option<LintMessage>,
    },
    // AES012 — Suffix rules
    SuffixForbidden { reason: Option<LintMessage> },
    // AES013 — Forbidden inheritance
    ForbiddenInheritance { reason: Option<LintMessage> },
    // AES020 — File size
    FileTooLarge { reason: Option<LintMessage> },
    FileTooShort { reason: Option<LintMessage> },
    // AES024 — Class/interface definition
    MandatoryClassDefinition { reason: Option<LintMessage> },
    // AES0301 — Taxonomy role
    PrimitiveUsage { reason: Option<LintMessage> },
    ConstantPurity { reason: Option<LintMessage> },
    // AES0302 — Contract primitive
    ContractPrimitive { reason: Option<LintMessage> },
    // AES0303 — Capability role
    CapabilityRouting {
        struct_name: SymbolName,
        reason: Option<LintMessage>,
    },
    // AES0305 — Agent role
    StatelessExecution { reason: Option<LintMessage> },
    HighLevelPolicy { reason: Option<LintMessage> },
    CoordinatesMultiple { reason: Option<LintMessage> },
    NoDomainLogic { reason: Option<LintMessage> },
    LazyEagerInit { reason: Option<LintMessage> },
    MustImplementContract { reason: Option<LintMessage> },
    AnyType { reason: Option<LintMessage> },
}

impl fmt::Display for AesViolationJs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // AES001
            Self::ForbiddenImport { source_layer, forbidden_layer, allowed, reason } => {
                let allowed_str = if allowed.is_empty() {
                    "none".to_string()
                } else {
                    allowed
                        .iter()
                        .map(|v| v.value().to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                };
                let why = reason.as_ref().map(|r| r.to_string()).unwrap_or_else(|| {
                    format!("Layer '{}' must not depend on '{}' to maintain architectural boundaries.", source_layer, forbidden_layer)
                });
                write!(f, "AES001 FORBIDDEN_IMPORT: Layer '{}' is importing from forbidden layer '{}'.\n\
                    WHY? {}\n\
                    FIX: Remove the import or refactor to use one of the allowed layers: [{}].",
                    source_layer, forbidden_layer, why, allowed_str)
            }
            // AES002
            Self::MissingImport { source_layer, required, reason } => {
                let default_why = format!("Layer '{}' must import '{}' to satisfy architectural contract requirements.", source_layer, required);
                let why = reason.as_ref().map(|r| r.to_string()).unwrap_or(default_why);
                write!(f, "AES002 MANDATORY_IMPORT: Layer '{}' is missing required import '{}'.\n\
                    WHY? {}\n\
                    FIX: Add the required import statement for '{}' in this file.",
                    source_layer, required, why, required)
            }
            // AES012
            Self::SuffixForbidden { reason } => {
                let default_why = "Forbidden suffixes prevent technical concepts from leaking into domain layers.".to_string();
                let why = reason.as_ref().map(|r| r.to_string()).unwrap_or(default_why);
                write!(f, "AES012 SUFFIX_FORBIDDEN: File uses a forbidden suffix for this layer.\n\
                    WHY? {}\n\
                    FIX: Rename the file to use an allowed suffix or move it to the correct layer.", why)
            }
            // AES013
            Self::ForbiddenInheritance { reason } => {
                let default_why = "Class/interface implements or extends a forbidden contract/interface.".to_string();
                let why = reason.as_ref().map(|r| r.to_string()).unwrap_or(default_why);
                write!(f, "AES013 FORBIDDEN_INHERITANCE: Implementation/inheritance from forbidden source.\n\
                    WHY? {}\n\
                    FIX: Use composition (fields) instead of direct inheritance/implementation.", why)
            }
            // AES020
            Self::FileTooLarge { reason } => {
                let default_why = "Large files violate the Single Responsibility Principle.".to_string();
                let why = reason.as_ref().map(|r| r.to_string()).unwrap_or(default_why);
                write!(f, "AES020 FILE_TOO_LARGE: File exceeds the maximum allowed line count.\n\
                    WHY? {}\n\
                    FIX: Split the module into smaller, more focused files.", why)
            }
            Self::FileTooShort { reason } => {
                let default_why = "Excessively small files clutter the project structure.".to_string();
                let why = reason.as_ref().map(|r| r.to_string()).unwrap_or(default_why);
                write!(f, "AES021 FILE_TOO_SHORT: File contains fewer than the required minimum lines.\n\
                    WHY? {}\n\
                    FIX: Expand the component or merge this logic into a related module.", why)
            }
            // AES024
            Self::MandatoryClassDefinition { reason } => {
                let default_why = "Encapsulation in classes/interfaces is required for proper modularization and contract adherence.".to_string();
                let why = reason.as_ref().map(|r| r.to_string()).unwrap_or(default_why);
                write!(f, "AES024 MANDATORY_DEFINITION: File is missing a class, interface, or type definition.\n\
                    WHY? {}\n\
                    FIX: Group functions into a class or implement an interface that defines the module boundary.", why)
            }
            // AES0301
            Self::PrimitiveUsage { reason } => {
                let default_why = "Direct primitive types must not be used within the taxonomy layer to avoid primitive obsession.".to_string();
                let why = reason.as_ref().map(|r| r.to_string()).unwrap_or(default_why);
                write!(f, "AES0301 PRIMITIVE_USAGE: Direct primitive usage in taxonomy.\n\
                    WHY? {}\n\
                    FIX: Define a specific Value Object (VO) or constant to wrap this primitive type.", why)
            }
            Self::ConstantPurity { reason } => {
                let default_why = "Constant taxonomy modules must only contain pure constant or static values.".to_string();
                let why = reason.as_ref().map(|r| r.to_string()).unwrap_or(default_why);
                write!(f, "AES0301 TAXONOMY_ROLE: Constant file contains non-constant declaration.\n\
                    WHY? {}\n\
                    FIX: Move the non-constant code to the appropriate layer, or convert it to a constant/static declaration.", why)
            }
            // AES0302
            Self::ContractPrimitive { reason } => {
                let default_why = "Contract signatures must enforce Value Object (VO) boundaries to prevent primitive obsession.".to_string();
                let why = reason.as_ref().map(|r| r.to_string()).unwrap_or(default_why);
                write!(f, "AES0302 CONTRACT_PRIMITIVE: Contract trait or method signature uses primitive types instead of taxonomy VO or constant.\n\
                    WHY? {}\n\
                    FIX: Replace primitive types with appropriate Value Objects (VO) or constants from the taxonomy layer.", why)
            }
            // AES0303
            Self::CapabilityRouting { struct_name, reason } => {
                let default_why = "Capability classes/interfaces must implement their corresponding protocol traits to ensure clean interface boundaries.".to_string();
                let why = reason.as_ref().map(|r| r.to_string()).unwrap_or(default_why);
                write!(f, "AES0303 CAPABILITY_ROLE: Struct/class '{}' has no interface/protocol implementation.\n\
                    WHY? {}\n\
                    FIX: Implement the capability protocol interface for '{}'.", struct_name, why, struct_name)
            }
            // AES0305
            Self::StatelessExecution { reason } => {
                let default_why = "Agent execution components must be stateless to guarantee reentrancy and prevent side effects.".to_string();
                let why = reason.as_ref().map(|r| r.to_string()).unwrap_or(default_why);
                write!(f, "AES0305 AGENT_ROLE: Non-stateless behavior detected.\n\
                    WHY? {}\n\
                    FIX: Remove mutable class state assignments or move initialization logic to the constructor.", why)
            }
            Self::HighLevelPolicy { reason } => {
                let default_why = "Agents must focus on high-level orchestration policies and not import infrastructure adapters directly.".to_string();
                let why = reason.as_ref().map(|r| r.to_string()).unwrap_or(default_why);
                write!(f, "AES0305 AGENT_ROLE: Low-level implementation details imported.\n\
                    WHY? {}\n\
                    FIX: Reference components using their contract interfaces instead of concrete infrastructure types.", why)
            }
            Self::CoordinatesMultiple { reason } => {
                let default_why = "Orchestrator agents exist to coordinate multiple subsystems; simple single-component logic belongs elsewhere.".to_string();
                let why = reason.as_ref().map(|r| r.to_string()).unwrap_or(default_why);
                write!(f, "AES0305 AGENT_ROLE: Orchestrator coordinates too few subsystems.\n\
                    WHY? {}\n\
                    FIX: Merge this simple flow into its caller or delegate at least two subsystems to this orchestrator.", why)
            }
            Self::NoDomainLogic { reason } => {
                let default_why = "Complex domain logic detected in a passive agent role or surface wrapper.".to_string();
                let why = reason.as_ref().map(|r| r.to_string()).unwrap_or(default_why);
                write!(f, "AES0305 AGENT_ROLE: Complex domain logic detected in a passive role.\n\
                    WHY? {}\n\
                    FIX: Move the complex domain/control logic into capabilities or orchestrator components.", why)
            }
            Self::LazyEagerInit { reason } => {
                let default_why = "Agent containers must only declare and wire dependencies, avoiding complex logic in constructors.".to_string();
                let why = reason.as_ref().map(|r| r.to_string()).unwrap_or(default_why);
                write!(f, "AES0305 AGENT_ROLE: Complex initialization logic found in container module.\n\
                    WHY? {}\n\
                    FIX: Move the initialization/conditional logic out of the constructor or container setup.", why)
            }
            Self::MustImplementContract { reason } => {
                let default_why = "Agent containers must implement the 'ServiceContainerAggregate' trait to satisfy dependency injection protocols.".to_string();
                let why = reason.as_ref().map(|r| r.to_string()).unwrap_or(default_why);
                write!(f, "AES0305 AGENT_ROLE: Class is missing required contract implementation.\n\
                    WHY? {}\n\
                    FIX: Add the 'ServiceContainerAggregate' implementation for the container struct.", why)
            }
            Self::AnyType { reason } => {
                let default_why = "Using 'any' type annotations bypasses type safety and violates agent-level domain-driven design.".to_string();
                let why = reason.as_ref().map(|r| r.to_string()).unwrap_or(default_why);
                write!(f, "AES0305 AGENT_ROLE: Forbidden 'any' type annotation found.\n\
                    WHY? {}\n\
                    FIX: Replace 'any' annotations with strongly-typed objects, structures, or domain Value Objects (VO).", why)
            }
        }
    }
}

impl From<AesViolationJs> for String {
    fn from(v: AesViolationJs) -> String {
        v.to_string()
    }
}
