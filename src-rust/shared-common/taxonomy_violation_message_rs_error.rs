// PURPOSE: AesViolation — AES violation messages for Rust analysis (enum with Display)
use crate::shared_common::taxonomy_layer_vo::LayerNameVO;
use crate::shared_common::taxonomy_message_vo::LintMessage;
use crate::shared_common::taxonomy_name_vo::SymbolName;
use std::fmt;

pub enum AesViolation {
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
    SuffixMismatch {
        reason: Option<LintMessage>,
    },
    // AES013 — Forbidden inheritance
    ForbiddenInheritance {
        reason: Option<LintMessage>,
    },
    // AES014 — Mandatory inheritance
    MandatoryInheritance {
        reason: Option<LintMessage>,
    },
    // AES015 — Circular import
    CircularImport {
        reason: Option<LintMessage>,
    },
    // AES020 — File size
    FileTooLarge {
        reason: Option<LintMessage>,
    },
    FileTooShort {
        reason: Option<LintMessage>,
    },
    // AES022 — Bypass comments
    BypassComment {
        reason: Option<LintMessage>,
    },
    UnwrapExpect {
        reason: Option<LintMessage>,
    },
    Panic {
        reason: Option<LintMessage>,
    },
    // AES023 — Unused imports
    FixUnusedImport {
        reason: Option<LintMessage>,
    },
    // AES024 — Class/struct definition & dead inheritance
    MandatoryClassDefinition {
        reason: Option<LintMessage>,
    },
    DeadInheritance {
        reason: Option<LintMessage>,
    },
    // AES030 — Orphan code
    OrphanCode {
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
    SingleBottleneck {
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
    // AES0306 — Surface role
    HierarchyViolation {
        reason: Option<LintMessage>,
    },
    PassiveViolation {
        reason: Option<LintMessage>,
    },
    SurfaceRoleViolation {
        reason: Option<LintMessage>,
    },
}

impl fmt::Display for AesViolation {
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
                let dynamic_why = match reason {
                    Some(r) => r.to_string(),
                    None => {
                        let src = source_layer.value();
                        if src == "taxonomy(vo)" {
                            "Taxonomy Value Objects (VO) must remain completely pure and cannot import agents, infrastructure, surfaces, contracts, capabilities, or root components.".to_string()
                        } else if src == "taxonomy(entity)"
                            || src == "taxonomy(error)"
                            || src == "taxonomy(event)"
                        {
                            "Taxonomy Entities, Errors, and Events can only import taxonomy VOs/constants and are forbidden from importing agents, infrastructure, surfaces, contracts, or capabilities.".to_string()
                        } else if src == "taxonomy(constant)" {
                            "Taxonomy Constants must remain pure static value declarations and cannot import agents, infrastructure, surfaces, contracts, capabilities, or root components.".to_string()
                        } else if src == "contract(port)" || src == "contract(protocol)" {
                            "Contract Ports and Protocols represent pure interface definitions and are forbidden from importing agents, infrastructure, surfaces, capabilities, aggregates, or root components.".to_string()
                        } else if src == "contract(aggregate)" {
                            "Contract Aggregates represent high-level composition/DI contracts and must not import agents, infrastructure, surfaces, capabilities, or root components.".to_string()
                        } else if src == "capabilities" {
                            "Capabilities implement domain business logic and must never depend on infrastructure adapters, agents, or UI/surfaces.".to_string()
                        } else if src == "infrastructure" {
                            "Infrastructure adapters implement technology-specific protocols and must never import surfaces, capabilities, agents, or root components directly.".to_string()
                        } else if src == "agent(container)" {
                            "Agent Containers handle dependency injection and are forbidden from importing UI/surfaces or root components.".to_string()
                        } else if src == "agent(orchestrator)" {
                            "Agent Orchestrators coordinate flows and are forbidden from importing UI/surfaces, infrastructure adapters, capabilities, or root components.".to_string()
                        } else if src == "agent(lifecycle)" {
                            "Agent Lifecycles manage agent states and are forbidden from importing orchestrators/containers, infrastructure, capabilities, surfaces, or root components.".to_string()
                        } else if src == "surfaces(command)"
                            || src == "surfaces(controller)"
                            || src == "surfaces(page)"
                            || src == "surfaces(entry)"
                        {
                            "Smart Surfaces act as user/CLI entry points and must never import agents, infrastructure, capabilities, or ports/protocols directly (must use ServiceContainerAggregate).".to_string()
                        } else if src == "surfaces(hook)"
                            || src == "surfaces(store)"
                            || src == "surfaces(action)"
                            || src == "surfaces(screen)"
                            || src == "surfaces(router)"
                        {
                            "Surface utility components (hooks, stores, routers) manage local state and must never import agents, infrastructure, capabilities, or ports/protocols.".to_string()
                        } else if src == "surfaces(component)"
                            || src == "surfaces(view)"
                            || src == "surfaces(layout)"
                        {
                            "Passive Surface components (views, layouts) render UI and are forbidden from importing agents, contracts, infrastructure, capabilities, or smart surfaces.".to_string()
                        } else if src.starts_with("taxonomy") {
                            "Taxonomy must remain pure and free from framework/layer dependencies to ensure domain model integrity.".to_string()
                        } else if src.starts_with("contract") {
                            "Contract interfaces represent pure specifications and must not depend on capabilities, infrastructure, or agent implementations.".to_string()
                        } else if src.starts_with("agent") {
                            "Agent orchestrators and containers must never depend on the UI/surface layer to maintain clean separation of concerns.".to_string()
                        } else if src.starts_with("surfaces") {
                            "Surfaces are external I/O boundaries and must never bypass contract aggregates to depend on capabilities, agent internals, or infrastructure.".to_string()
                        } else {
                            format!("Layer '{}' must not depend on '{}' to maintain architectural boundaries.", source_layer, forbidden_layer)
                        }
                    }
                };
                write!(
                    f,
                    "AES001 FORBIDDEN_IMPORT: Layer '{}' is importing from forbidden layer '{}'.\n\
                    WHY? {}\n\
                    FIX: Remove the import or refactor to use one of the allowed layers: [{}].",
                    source_layer, forbidden_layer, dynamic_why, allowed_str
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
            Self::SuffixMismatch { reason } => {
                let default_why = "Contract files must end with '_port', '_protocol', or '_aggregate' to specify their interface type.".to_string();
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(f, "AES012 SUFFIX_MISMATCH: Contract file is missing a valid contract suffix.\n\
                    WHY? {}\n\
                    FIX: Rename the file to include '_port', '_protocol', or '_aggregate' as a suffix.", why)
            }
            // AES013
            Self::ForbiddenInheritance { reason } => {
                let default_why = "Contract aggregates must not inherit from port or protocol contracts to keep composition clean.".to_string();
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(f, "AES013 FORBIDDEN_INHERITANCE: Implementation/inheritance from forbidden source.\n\
                    WHY? {}\n\
                    FIX: Use composition (fields) instead of direct inheritance/implementation.", why)
            }
            // AES014
            Self::MandatoryInheritance { reason } => {
                let default_why = "Files that import contract interfaces are expected to implement them to fulfill architectural roles.".to_string();
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(f, "AES014 MANDATORY_INHERITANCE: File imports contracts but no struct or class implements them.\n\
                    WHY? {}\n\
                    FIX: Define a struct or class that implements the imported contract interface.", why)
            }
            // AES015
            Self::CircularImport { reason } => {
                let default_why = "Circular dependencies couple components together and break unidirectional data/import flow.".to_string();
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(
                    f,
                    "AES015 CIRCULAR_IMPORT: Circular dependency detected.\n\
                    WHY? {}\n\
                    FIX: Refactor imports or extract the shared logic into a lower, common layer.",
                    why
                )
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
            // AES022
            Self::BypassComment { reason } => {
                let default_why =
                    "Bypassing code checks hides issues and risks architectural regressions."
                        .to_string();
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(f, "AES022 BYPASS_COMMENT: Forbidden bypass comment or annotation detected.\n\
                    WHY? {}\n\
                    FIX: Remove the bypass comment (e.g. noqa, eslint-disable, ts-ignore) and resolve the issue properly.", why)
            }
            Self::UnwrapExpect { reason } => {
                let default_why = "Using unwrap or expect results in runtime panics and bypasses proper error propagation.".to_string();
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(f, "AES022 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.\n\
                    WHY? {}\n\
                    FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').", why)
            }
            Self::Panic { reason } => {
                let default_why = "Manual panic calls crash the program unexpectedly instead of using structured error recovery.".to_string();
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(
                    f,
                    "AES022 PANIC: Forbidden panic call detected.\n\
                    WHY? {}\n\
                    FIX: Return a Result or handle the failure case gracefully without panicking.",
                    why
                )
            }
            // AES023
            Self::FixUnusedImport { reason } => {
                let default_why = "Unused imports clutter the codebase and increase compilation/dependency overhead.".to_string();
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(f, "AES023 UNUSED_IMPORT: Unused import detected.\n\
                    WHY? {}\n\
                    FIX: Remove the unused import statement or use the imported symbol in this file.", why)
            }
            // AES024
            Self::MandatoryClassDefinition { reason } => {
                let default_why = "Encapsulation in structs/traits is required for proper modularization and contract adherence.".to_string();
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(f, "AES024 MANDATORY_DEFINITION: File is missing a struct, enum, or trait definition.\n\
                    WHY? {}\n\
                    FIX: Group functions into a struct or implement a Trait that defines the module interface.", why)
            }
            Self::DeadInheritance { reason } => {
                let default_why = "Empty inheritance/implementation blocks do not add behavior and indicate dead or incomplete code.".to_string();
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(f, "AES024 DEAD_INHERITANCE: Empty struct, class, or trait implementation block detected.\n\
                    WHY? {}\n\
                    FIX: Implement the necessary methods/fields or remove the empty definition block.", why)
            }
            // AES030
            Self::OrphanCode { reason } => {
                let default_why = "Orphan code indicates dead, unreachable, or unreferenced logic that should not exist in the active workspace.".to_string();
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(f, "AES030 ORPHAN_CODE: Unused or unreachable orphan file detected.\n\
                    WHY? {}\n\
                    FIX: [AI DECISION REQUIRED] Decide whether you should wire this implementation (import/reference it in the appropriate container, orchestrator, or router) or delete this file if it is obsolete.", why)
            }
            // AES0301
            Self::ConstantPurity { reason } => {
                let default_why = "Constant taxonomy modules must only contain pure constant or static values to maintain value-level immutability.".to_string();
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
                let default_why = "Contracts must enforce value object boundaries to prevent primitive obsession.".to_string();
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(f, "AES0302 CONTRACT_PRIMITIVE: Contract trait or method signature uses primitive types instead of taxonomy VO or constant.\n\
                    WHY? {}\n\
                    FIX: Replace primitive types with appropriate Value Objects (VO) or constants from the taxonomy layer.", why)
            }
            // AES0303
            Self::CapabilityRouting {
                struct_name,
                reason,
            } => {
                let default_why = "Capability structs must implement their corresponding protocol traits to ensure clean interface boundaries.".to_string();
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(
                    f,
                    "AES0303 CAPABILITY_ROLE: Struct '{}' has no trait implementation.\n\
                    WHY? {}\n\
                    FIX: Implement the capability protocol trait for '{}'.",
                    struct_name, why, struct_name
                )
            }
            Self::SingleBottleneck { reason } => {
                let default_why = "Routing all commands to a single capability violates high-level decomposition and creates a single bottleneck.".to_string();
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(f, "AES0303 CAPABILITY_ROLE: All orchestrator dispatch routes route to a single capability.\n\
                    WHY? {}\n\
                    FIX: Distribute logic or route commands to multiple distinct capabilities.", why)
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
                let default_why = "Agent containers must implement the 'ServiceContainerAggregate' trait to satisfy dependency injection protocols.".to_string();
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(f, "AES0305 AGENT_ROLE: Class is missing required contract implementation.\n\
                    WHY? {}\n\
                    FIX: Add the 'ServiceContainerAggregate' implementation for the container struct.", why)
            }
            Self::AnyType { reason } => {
                let default_why = "Using 'any' type annotations bypasses type safety and violates agent-level domain-driven design.".to_string();
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(f, "AES0305 AGENT_ROLE: Forbidden 'any' or 'Any' type annotation found.\n\
                    WHY? {}\n\
                    FIX: Replace 'any' annotations with strongly-typed objects, structures, or domain Value Objects (VO).", why)
            }
            // AES0306
            Self::HierarchyViolation { reason } => {
                let default_why = "Surface components must expose their capabilities cleanly through layer barrels to prevent leaky encapsulation.".to_string();
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(f, "AES0306 SURFACE_ROLE: Surface file is not imported from the layer barrel/router.\n\
                    WHY? {}\n\
                    FIX: Add the file's exports to the module barrel or main entry router.", why)
            }
            Self::PassiveViolation { reason } => {
                let default_why = "Surfaces must act strictly as passive I/O boundaries and not implement core business rules.".to_string();
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(f, "AES0306 SURFACE_ROLE: Surface file contains active domain or business logic.\n\
                    WHY? {}\n\
                    FIX: Refactor the business logic out of the surface component into capabilities or orchestrator layers.", why)
            }
            Self::SurfaceRoleViolation { reason } => {
                let default_why = "Surface files must stay lightweight and focused (e.g. fewer than 15 functions) to prevent complexity accumulation.".to_string();
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(
                    f,
                    "AES0306 SURFACE_ROLE: Surface file exceeds its role mandate.\n\
                    WHY? {}\n\
                    FIX: Divide the surface file into smaller, more specific surface components.",
                    why
                )
            }
        }
    }
}

impl From<AesViolation> for String {
    fn from(v: AesViolation) -> String {
        v.to_string()
    }
}
