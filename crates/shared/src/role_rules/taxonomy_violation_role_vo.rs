// PURPOSE: AesRoleViolation — data container for role rule violations (AES401-406)
// Messages are written inline in each checker, not here.
use crate::common::taxonomy_message_vo::LintMessage;
use crate::common::taxonomy_name_vo::SymbolName;

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
    CapabilityNoProtocol {
        reason: Option<LintMessage>,
    },
    CapabilityNoImplementor {
        reason: Option<LintMessage>,
    },
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
