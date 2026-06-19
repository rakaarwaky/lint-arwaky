// PURPOSE: AesOrphanViolation — violation messages for orphan detection (AES501-506)
use crate::common::taxonomy_message_vo::LintMessage;
use std::fmt;

#[derive(Debug, Clone)]
pub enum AesOrphanViolation {
    OrphanCode { reason: Option<LintMessage> },
    TaxonomyOrphan { reason: Option<LintMessage> },
    CapabilitiesOrphan { reason: Option<LintMessage> },
    AgentOrphan { reason: Option<LintMessage> },
    SurfaceOrphan { category: &'static str, reason: Option<LintMessage> },
}

impl fmt::Display for AesOrphanViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AesOrphanViolation::OrphanCode { reason } => {
                let default_why = "Orphan code indicates dead, unreachable, or unreferenced logic that should not exist in the active workspace.".to_string();
                let why = reason.as_ref().map(|r| r.to_string()).unwrap_or(default_why);
                write!(f, "AES030 ORPHAN_CODE: Unused or unreachable orphan file detected.\n\
                        WHY? {}\n\
                        FIX: [AI DECISION REQUIRED] Decide whether you should wire this implementation (import/reference it in the appropriate container, orchestrator, or router) or delete this file if it is obsolete.", why)
            }
            AesOrphanViolation::TaxonomyOrphan { reason } => {
                let default_why = "Taxonomy value objects must be referenced by contract layer modules.".to_string();
                let why = reason.as_ref().map(|r| r.to_string()).unwrap_or(default_why);
                write!(f, "AES501 TAXONOMY_ORPHAN: Taxonomy code is unreachable.\n\
                        WHY? {}\n\
                        FIX: Wire the taxonomy file into its contract layer implementation.", why)
            }
            AesOrphanViolation::CapabilitiesOrphan { reason } => {
                let default_why = "Capabilities must be registered in the root container to be discoverable.".to_string();
                let why = reason.as_ref().map(|r| r.to_string()).unwrap_or(default_why);
                write!(f, "AES503 CAPABILITIES_ORPHAN: Capabilities implementation is not wired.\n\
                        WHY? {}\n\
                        FIX: Register this capability in the root container or orchestrator.", why)
            }
            AesOrphanViolation::AgentOrphan { reason } => {
                let default_why = "Agent components must serve as the entry point for capability operations.".to_string();
                let why = reason.as_ref().map(|r| r.to_string()).unwrap_or(default_why);
                write!(f, "AES505 AGENT_ORPHAN: Agent component is unreachable from any surface.\n\
                        WHY? {}\n\
                        FIX: Ensure the agent is invoked by a controller, command, or root entry point within the dependency flow.", why)
            }
            AesOrphanViolation::SurfaceOrphan { category, reason } => {
                let default_why = format!("Surface files in '{}' category must be reachable through entry points or other surfaces.", category);
                let why = reason.as_ref().map(|r| r.to_string()).unwrap_or(default_why);
                write!(f, "AES506 SURFACE_ORPHAN: {} surface is unreachable.\n\
                        WHY? {}\n\
                        FIX: Wire the surface to its appropriate entry point, router, or parent surface.", category, why)
            }
        }
    }
}

impl From<AesOrphanViolation> for String {
    fn from(v: AesOrphanViolation) -> String {
        v.to_string()
    }
}
