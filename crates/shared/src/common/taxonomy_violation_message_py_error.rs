// PURPOSE: AesViolationPy — AES violation messages for Python analysis (enum with Display)
use crate::common::taxonomy_layer_vo::LayerNameVO;
use crate::common::taxonomy_message_vo::LintMessage;
use crate::common::taxonomy_name_vo::SymbolName;
use std::fmt;

pub enum AesViolationPy {
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
    SuffixForbidden {
        reason: Option<LintMessage>,
    },
    // AES013 — Forbidden inheritance
    ForbiddenInheritance {
        reason: Option<LintMessage>,
    },
    // AES020 — File size
    FileTooLarge {
        reason: Option<LintMessage>,
    },
    FileTooShort {
        reason: Option<LintMessage>,
    },
    // AES024 — Class definition
    MandatoryClassDefinition {
        reason: Option<LintMessage>,
    },
    // AES0301 — Taxonomy role
    ConstantPurity {
        reason: Option<LintMessage>,
    },
    PrimitiveUsage {
        primitive: SymbolName,
        reason: Option<LintMessage>,
    },
    // AES0302 — Contract primitive
    ContractPrimitive {
        reason: Option<LintMessage>,
    },
    // AES0303 — Capability role
    CapabilityRouting {
        struct_name: SymbolName,
        reason: Option<LintMessage>,
    },
    // AES0305 — Agent role
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
}

impl fmt::Display for AesViolationPy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // AES001
            Self::ForbiddenImport {
                source_layer,
                forbidden_layer,
                allowed,
                reason,
            } => {
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
                    format!(
                        "Layer '{}' must not depend on '{}' to maintain architectural boundaries.",
                        source_layer, forbidden_layer
                    )
                });
                write!(
                    f,
                    "AES001 FORBIDDEN_IMPORT: Layer '{}' is importing from forbidden layer '{}'.\n\
                    WHY? {}\n\
                    FIX: Remove the import or refactor to use one of the allowed layers: [{}].",
                    source_layer, forbidden_layer, why, allowed_str
                )
            }
            // AES002
            Self::MissingImport {
                source_layer,
                required,
                reason,
            } => {
                let default_why = format!(
                    "Layer '{}' must import '{}' to satisfy architectural contract requirements.",
                    source_layer, required
                );
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(
                    f,
                    "AES002 MANDATORY_IMPORT: Layer '{}' is missing required import '{}'.\n\
                    WHY? {}\n\
                    FIX: Add the required import statement for '{}' in this file.",
                    source_layer, required, why, required
                )
            }
            // AES012
            Self::SuffixForbidden { reason } => {
                let default_why = "Forbidden suffixes prevent technical concepts from leaking into domain layers.".to_string();
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(
                    f,
                    "AES012 SUFFIX_FORBIDDEN: File uses a forbidden suffix for this layer.\n\
                    WHY? {}\n\
                    FIX: Rename the file to use an allowed suffix or move it to the correct layer.",
                    why
                )
            }
            // AES013
            Self::ForbiddenInheritance { reason } => {
                let default_why =
                    "Class implements or inherits from a forbidden contract or component."
                        .to_string();
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(f, "AES013 FORBIDDEN_INHERITANCE: Implementation/inheritance from forbidden source.\n\
                    WHY? {}\n\
                    FIX: Use composition (fields) instead of direct inheritance/implementation.", why)
            }
            // AES020
            Self::FileTooLarge { reason } => {
                let default_why =
                    "Large files violate the Single Responsibility Principle.".to_string();
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(
                    f,
                    "AES020 FILE_TOO_LARGE: File exceeds the maximum allowed line count.\n\
                    WHY? {}\n\
                    FIX: Split the module into smaller, more focused files.",
                    why
                )
            }
            Self::FileTooShort { reason } => {
                let default_why =
                    "Excessively small files clutter the project structure.".to_string();
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(
                    f,
                    "AES021 FILE_TOO_SHORT: File contains fewer than the required minimum lines.\n\
                    WHY? {}\n\
                    FIX: Expand the component or merge this logic into a related module.",
                    why
                )
            }
            // AES024
            Self::MandatoryClassDefinition { reason } => {
                let default_why = "Encapsulation in classes/Protocols is required for proper modularization and contract adherence.".to_string();
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(f, "AES024 MANDATORY_DEFINITION: File is missing a class, dataclass, or Protocol definition.\n\
                    WHY? {}\n\
                    FIX: Group functions into a class or implement a Protocol that defines the module interface.", why)
            }
            // AES0301
            Self::ConstantPurity { reason } => {
                let default_why =
                    "Constant taxonomy modules must only contain pure constant or static values."
                        .to_string();
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(f, "AES0301 TAXONOMY_ROLE: Constant file contains non-constant declaration.\n\
                    WHY? {}\n\
                    FIX: Move the non-constant code to the appropriate layer, or convert it to a constant/static declaration.", why)
            }
            Self::PrimitiveUsage { primitive, reason } => {
                let default_why = format!(
                    "Direct primitive types (like '{}') are forbidden in taxonomy entities, errors, and events to maintain strict value object boundaries and avoid primitive obsession.",
                    primitive
                );
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(
                    f,
                    "AES0301 TAXONOMY_ROLE: Direct primitive '{}' in taxonomy entity, error, or event.\n\
                    WHY? {}\n\
                    FIX: Replace the primitive type with a domain Value Object (VO) or constant from the taxonomy layer.",
                    primitive, why
                )
            }
            // AES0302
            Self::ContractPrimitive { reason } => {
                let default_why = "Contract signatures must enforce Value Object (VO) boundaries to prevent primitive obsession.".to_string();
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(f, "AES0302 CONTRACT_PRIMITIVE: Contract Protocol or method signature uses primitive types instead of taxonomy VO or constant.\n\
                    WHY? {}\n\
                    FIX: Replace primitive types with appropriate Value Objects (VO) or constants from the taxonomy layer.", why)
            }
            // AES0303
            Self::CapabilityRouting {
                struct_name,
                reason,
            } => {
                let default_why = "Capability classes must implement their corresponding Protocols/ABCs to ensure clean interface boundaries.".to_string();
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(
                    f,
                    "AES0303 CAPABILITY_ROLE: Class '{}' has no Protocol implementation.\n\
                    WHY? {}\n\
                    FIX: Implement the capability Protocol for '{}'.",
                    struct_name, why, struct_name
                )
            }
            // AES0305
            Self::StatelessExecution { reason } => {
                let default_why = "Agent execution components must be stateless to guarantee reentrancy and prevent side effects.".to_string();
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(f, "AES0305 AGENT_ROLE: Non-stateless behavior detected.\n\
                    WHY? {}\n\
                    FIX: Remove mutable class state assignments or move initialization logic to the constructor.", why)
            }
            Self::HighLevelPolicy { reason } => {
                let default_why = "Agents must focus on high-level orchestration policies and not import infrastructure adapters directly.".to_string();
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(f, "AES0305 AGENT_ROLE: Low-level implementation details imported.\n\
                    WHY? {}\n\
                    FIX: Reference components using their contract interfaces instead of concrete infrastructure types.", why)
            }
            Self::CoordinatesMultiple { reason } => {
                let default_why = "Orchestrator agents exist to coordinate multiple subsystems; simple single-component logic belongs elsewhere.".to_string();
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(f, "AES0305 AGENT_ROLE: Orchestrator coordinates too few subsystems.\n\
                    WHY? {}\n\
                    FIX: Merge this simple flow into its caller or delegate at least two subsystems to this orchestrator.", why)
            }
            Self::NoDomainLogic { reason } => {
                let default_why =
                    "Complex domain logic detected in a passive agent role or surface wrapper."
                        .to_string();
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(f, "AES0305 AGENT_ROLE: Complex domain logic detected in a passive role.\n\
                    WHY? {}\n\
                    FIX: Move the complex domain/control logic into capabilities or orchestrator components.", why)
            }
            Self::LazyEagerInit { reason } => {
                let default_why = "Agent containers must only declare and wire dependencies, avoiding complex logic in constructors.".to_string();
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(f, "AES0305 AGENT_ROLE: Complex initialization logic found in container module.\n\
                    WHY? {}\n\
                    FIX: Move the initialization/conditional logic out of the constructor or container setup.", why)
            }
            Self::MustImplementContract { reason } => {
                let default_why = "Agent containers must implement the 'ServiceContainerAggregate' Protocol/ABC to satisfy dependency injection protocols.".to_string();
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(f, "AES0305 AGENT_ROLE: Class is missing required contract implementation.\n\
                    WHY? {}\n\
                    FIX: Add the 'ServiceContainerAggregate' implementation for the container class.", why)
            }
            Self::AnyType { reason } => {
                let default_why = "Using 'any' or 'Any' type annotations bypasses type safety and violates agent-level domain-driven design.".to_string();
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(f, "AES0305 AGENT_ROLE: Forbidden 'any' or 'Any' type annotation found.\n\
                    WHY? {}\n\
                    FIX: Replace 'any' annotations with strongly-typed objects, structures, or domain Value Objects (VO).", why)
            }
        }
    }
}

impl From<AesViolationPy> for String {
    fn from(v: AesViolationPy) -> String {
        v.to_string()
    }
}
