use crate::common::taxonomy_message_vo::LintMessage;
use std::fmt;

#[derive(Debug, Clone)]
pub enum AesOrphanViolation {
    OrphanCode {
        stem: String,
        reason: Option<LintMessage>,
    },
    TaxonomyOrphan {
        stem: String,
        category: &'static str,
        reason: Option<LintMessage>,
    },
    ContractOrphan {
        suffix: String,
        trait_name: String,
        target_layer: &'static str,
        reason: Option<LintMessage>,
    },
    CapabilitiesOrphan {
        stem: String,
        reason: Option<LintMessage>,
    },
    InfrastructureOrphan {
        stem: String,
        reason: Option<LintMessage>,
    },
    AgentOrphan {
        agg_name: String,
        reason: Option<LintMessage>,
    },
    SurfaceOrphan {
        category: &'static str,
        stem: String,
        reason: Option<LintMessage>,
    },
}

impl fmt::Display for AesOrphanViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AesOrphanViolation::OrphanCode { stem, reason } => {
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or_else(|| format!("File '{}' matches no known layer prefix and is not referenced by any other file.", stem));
                write!(f, "AES500 ORPHAN_CODE: '{}' is unreachable.\nWHY? {}\nFIX: Rename the file with a valid layer prefix (taxonomy_, contract_, capabilities_, infrastructure_, agent_, surface_, root_) or import it from another file.", stem, why)
            }
            AesOrphanViolation::TaxonomyOrphan {
                stem,
                category,
                reason,
            } => {
                let target_hint = match *category {
                    "utility" | "helper" => "any file that needs its functionality".to_string(),
                    _ => "a contract_* file (contract_port, contract_protocol, or contract_aggregate)".to_string(),
                };
                let why = reason.as_ref().map(|r| r.to_string()).unwrap_or_else(|| {
                    format!("Taxonomy file '{}' is not imported by any file.", stem)
                });
                write!(f, "AES501 TAXONOMY_ORPHAN: '{}' is not imported.\nWHY? {}\nFIX: Import '{}' in {}.", stem, why, stem, target_hint)
            }
            AesOrphanViolation::ContractOrphan {
                suffix,
                trait_name,
                target_layer,
                reason,
            } => {
                let why = reason.as_ref().map(|r| r.to_string()).unwrap_or_else(|| {
                    format!(
                        "Contract {} '{}' is not implemented by any {} file.",
                        suffix, trait_name, target_layer
                    )
                });
                let fix = match suffix.as_str() {
                    "port" => format!("Implement '{}' in an infrastructure_* file, or wire it in agent_*_orchestrator.rs if already implemented.", trait_name),
                    "protocol" => format!("Implement '{}' in a capabilities_* file, or wire it in agent_*_orchestrator.rs if already implemented.", trait_name),
                    "aggregate" => format!("Import and use '{}' in a surface_* file or root_*_container.rs.", trait_name),
                    _ => format!("Implement '{}' in the appropriate layer.", trait_name),
                };
                write!(
                    f,
                    "AES502 CONTRACT_ORPHAN: Contract {} '{}' is orphaned.\nWHY? {}\nFIX: {}",
                    suffix, trait_name, why, fix
                )
            }
            AesOrphanViolation::CapabilitiesOrphan { stem, reason } => {
                let why = reason.as_ref().map(|r| r.to_string()).unwrap_or_else(|| {
                    format!(
                        "Capabilities file '{}' is not wired in any container.",
                        stem
                    )
                });
                write!(f, "AES503 CAPABILITIES_ORPHAN: '{}' is not wired.\nWHY? {}\nFIX: Register '{}' in root_*_entry.rs or root_*_container.rs, or remove the file if it is obsolete.", stem, why, stem)
            }
            AesOrphanViolation::InfrastructureOrphan { stem, reason } => {
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or_else(|| format!("Infrastructure file '{}' is not wired in any container and unreachable from any entry point.", stem));
                write!(f, "AES504 INFRASTRUCTURE_ORPHAN: '{}' is not wired.\nWHY? {}\nFIX: Register '{}' in root_*_entry.rs or root_*_container.rs, or remove the file if it is obsolete.", stem, why, stem)
            }
            AesOrphanViolation::AgentOrphan { agg_name, reason } => {
                let why = reason.as_ref().map(|r| r.to_string()).unwrap_or_else(|| {
                    format!(
                        "Agent aggregate '{}' is not called by any surface or container.",
                        agg_name
                    )
                });
                write!(f, "AES505 AGENT_ORPHAN: Aggregate '{}' is unreachable from any surface.\nWHY? {}\nFIX: Import and use '{}' in a surface_* file or root_*_container.rs, or remove the file if obsolete.", agg_name, why, agg_name)
            }
            AesOrphanViolation::SurfaceOrphan {
                category,
                stem,
                reason,
            } => {
                let (where_hint, fix_hint) = match *category {
                    "smart" => ("entry point or router", "an entry point (root_*_entry.rs, cli_*, mcp_*) or router file"),
                    "utility" => ("smart surface", "a smart surface (command, controller, page)"),
                    "passive" => ("smart or utility surface", "a smart surface (command, controller, page) or utility surface (hook, store, action, screen, router)"),
                    _ => ("the appropriate importer", "an appropriate importer file"),
                };
                let why = reason.as_ref().map(|r| r.to_string()).unwrap_or_else(|| {
                    format!(
                        "{} surface '{}' is not imported by any {}.",
                        category, stem, where_hint
                    )
                });
                write!(f, "AES506 SURFACE_ORPHAN: {} surface '{}' is orphaned.\nWHY? {}\nFIX: Import '{}' in {}.", category, stem, why, stem, fix_hint)
            }
        }
    }
}

impl From<AesOrphanViolation> for String {
    fn from(v: AesOrphanViolation) -> String {
        v.to_string()
    }
}
