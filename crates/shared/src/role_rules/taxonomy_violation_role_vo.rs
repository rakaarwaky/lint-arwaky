// PURPOSE: AesRoleViolation — data container for role rule violations (AES401-406)
// Messages are written inline in each checker, not here.
use crate::common::taxonomy_language_vo::Language;
use crate::common::taxonomy_message_vo::LintMessage;
use crate::common::taxonomy_name_vo::SymbolName;

fn resolve_why(reason: &Option<LintMessage>, default: &str) -> String {
    match reason {
        Some(r) => r.to_string(),
        None => default.to_string(),
    }
}

#[derive(Debug, Clone)]
pub enum AesRoleViolation {
    ConstantPurity { reason: Option<LintMessage> },
    PrimitiveUsage { primitive: SymbolName, reason: Option<LintMessage> },
    ContractPrimitive { reason: Option<LintMessage> },
    CapabilityNoProtocol { reason: Option<LintMessage> },
    CapabilityNoImplementor { reason: Option<LintMessage> },
    CapabilityTooManyTypes { count: usize, reason: Option<LintMessage> },
    SingleBottleneck { reason: Option<LintMessage> },
    UtilityRole { reason: Option<LintMessage> },
    AgentNoImplementor { reason: Option<LintMessage> },
    AgentTooManyTypes { count: usize, names: Vec<SymbolName>, reason: Option<LintMessage> },
    StatelessExecution { reason: Option<LintMessage> },
    HighLevelPolicy { reason: Option<LintMessage> },
    CoordinatesMultiple { reason: Option<LintMessage> },
    NoDomainLogic { reason: Option<LintMessage> },
    LazyEagerInit { reason: Option<LintMessage> },
    MustImplementContract { reason: Option<LintMessage> },
    AgentFileSizeLimit { max_lines: usize },
    PassiveViolation { reason: Option<LintMessage> },
    SurfaceRoleViolation { reason: Option<LintMessage> },
}

/// Returns only the WHY string for the given role violation.
/// Full message assembly happens at the consumer via RoleMessage / SurfaceMessage.
pub fn format_role_violation(v: &AesRoleViolation, lang: Language) -> String {
    match v {
        AesRoleViolation::ConstantPurity { reason } => {
            resolve_why(reason, "Constant taxonomy modules must only contain pure constant or static values to maintain value-level immutability.")
        }
        AesRoleViolation::PrimitiveUsage { primitive, reason } => {
            resolve_why(reason, &format!("Direct primitive types (like '{primitive}') are forbidden in taxonomy entities, errors, and events to maintain strict value object boundaries and avoid primitive obsession."))
        }
        AesRoleViolation::ContractPrimitive { reason } => {
            resolve_why(reason, &format!("Contracts must enforce value object boundaries to prevent primitive obsession. Use {} instead of primitives.", lang.type_kw()))
        }
        AesRoleViolation::CapabilityNoProtocol { reason } => {
            resolve_why(reason, "file has 'capabilities_' prefix but no _protocol import — this file is broken/useless. Either it is not a real capability (rename or delete), or a proper contract protocol requirement has not been created yet (create the protocol first, then implement it here)")
        }
        AesRoleViolation::CapabilityNoImplementor { reason } => {
            resolve_why(reason, "At least one struct must implement a _protocol trait (impl Trait for Struct). Internal helper structs are allowed.")
        }
        AesRoleViolation::CapabilityTooManyTypes { count, reason } => {
            resolve_why(reason, &format!("Max 3 types (struct/enum) allowed in capabilities. Found {count}. Refactor excess types to taxonomy layer."))
        }
        AesRoleViolation::SingleBottleneck { reason } => {
            resolve_why(reason, "Routing all commands to a single capability violates high-level decomposition and creates a single bottleneck.")
        }
        AesRoleViolation::UtilityRole { reason } => {
            resolve_why(reason, "file has 'utility_' prefix but does not contain stateless standalone functions — this file may be misplaced. Utility files must contain only pure, stateless functions that depend only on taxonomy.")
        }
        AesRoleViolation::AgentNoImplementor { reason } => {
            resolve_why(reason, "At least one struct must implement an _aggregate trait (impl Trait for Struct). Internal helper structs are allowed.")
        }
        AesRoleViolation::AgentTooManyTypes { count, names, reason } => {
            let names_str: Vec<String> = names.iter().map(|n| n.to_string()).collect();
            resolve_why(reason, &format!("Max 3 types (struct/enum) allowed in agent files. Found {count}: [{}]. Refactor excess types to taxonomy layer.", names_str.join(", ")))
        }
        AesRoleViolation::StatelessExecution { reason } => {
            resolve_why(reason, "Agent execution components must be stateless to guarantee reentrancy and prevent side effects.")
        }
        AesRoleViolation::HighLevelPolicy { reason } => {
            resolve_why(reason, "Agents must focus on high-level orchestration policies and not import concrete implementations directly.")
        }
        AesRoleViolation::CoordinatesMultiple { reason } => {
            resolve_why(reason, "Orchestrator agents exist to coordinate multiple subsystems; simple single-component logic belongs elsewhere.")
        }
        AesRoleViolation::NoDomainLogic { reason } => {
            resolve_why(reason, "Complex domain logic detected in a passive agent role or surface wrapper.")
        }
        AesRoleViolation::LazyEagerInit { reason } => {
            resolve_why(reason, "Agent containers must only declare and wire dependencies, avoiding complex logic in constructors.")
        }
        AesRoleViolation::MustImplementContract { reason } => {
            resolve_why(reason, &format!("Agent containers must implement the 'ServiceContainerAggregate' {} to satisfy dependency injection protocols.", lang.interface_kw()))
        }
        AesRoleViolation::AgentFileSizeLimit { max_lines } => {
            format!("Agent file exceeds {max_lines} lines.")
        }
        AesRoleViolation::PassiveViolation { reason } => {
            resolve_why(reason, "Passive surfaces must not contain logic that should be in capabilities or agents.")
        }
        AesRoleViolation::SurfaceRoleViolation { reason } => {
            resolve_why(reason, "Surface role violation - surfaces must adhere to their designated role (command, controller, component, hook, etc.).")
        }
    }
}
