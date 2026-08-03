// PURPOSE: AesOrphanViolation — data container for orphan rule violations (AES501-506)
// Messages are formatted by `format_orphan_violation()`, not by Display.
use crate::common::taxonomy_message_vo::LintMessage;

#[derive(Debug, Clone)]
pub enum AesOrphanViolation {
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
    UtilityOrphan {
        stem: String,
        reason: Option<LintMessage>,
    },
    UtilityDeadCode {
        stem: String,
        imported_by: Vec<String>,
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

pub fn format_orphan_violation(v: &AesOrphanViolation) -> String {
    match v {
        AesOrphanViolation::TaxonomyOrphan {
            stem,
            category,
            reason,
        } => {
            let target_hint = match *category {
                "utility" | "helper" => "any file that needs its functionality".to_string(),
                _ => "a contract_* file (contract_port, contract_protocol, or contract_aggregate)".to_string(),
            };
            let why = match reason.as_ref() {
                Some(r) => r.to_string(),
                None => {
                    format!("Taxonomy file '{}' is not imported by any file.", stem)
                }
            };
            format!(
                "AES501 TAXONOMY_ORPHAN: '{}' is not imported.\nWHY? {}\nFIX: Import '{}' in {}.",
                stem, why, stem, target_hint
            )
        }
        AesOrphanViolation::ContractOrphan {
            suffix,
            trait_name,
            target_layer,
            reason,
        } => {
            let why = match reason.as_ref() {
                Some(r) => r.to_string(),
                None => format!(
                    "Contract {} '{}' is not implemented by any {} file.",
                    suffix, trait_name, target_layer
                ),
            };
            let fix = match suffix.as_str() {
                "protocol" => format!(
                    "Implement '{}' in a capabilities_* file, or wire it in agent_*_orchestrator.rs if already implemented.",
                    trait_name
                ),
                "aggregate" => format!(
                    "Import and use '{}' in a surface_* file or root_*_container.rs.",
                    trait_name
                ),
                _ => format!("Implement '{}' in the appropriate layer.", trait_name),
            };
            format!(
                "AES502 CONTRACT_ORPHAN: Contract {} '{}' is orphaned.\nWHY? {}\nFIX: {}",
                suffix, trait_name, why, fix
            )
        }
        AesOrphanViolation::CapabilitiesOrphan { stem, reason } => {
            let why = match reason.as_ref() {
                Some(r) => r.to_string(),
                None => format!(
                    "Capabilities file '{}' is not wired in any container.",
                    stem
                ),
            };
            format!(
                "AES503 CAPABILITIES_ORPHAN: '{}' is not wired.\nWHY? {}\nFIX: Register '{}' in root_*_entry.rs or root_*_container.rs via `use {}::...;` and wire it into the container's constructor. If this file is obsolete, delete it and remove its module declaration from lib.rs.",
                stem, why, stem, stem
            )
        }
        AesOrphanViolation::UtilityOrphan { stem, reason } => {
            let why = match reason.as_ref() {
                Some(r) => r.to_string(),
                None => format!(
                    "Utility file '{}' is not imported by any capabilities or other layer file.",
                    stem
                ),
            };
            format!(
                "AES504 UTILITY_ORPHAN: '{}' is not imported.\nWHY? {}\nFIX: Import '{}' in a capabilities_* file that needs its functionality. Utility files must be consumed by other layers. If this file is obsolete, delete it and remove its module declaration from lib.rs.",
                stem, why, stem
            )
        }
        AesOrphanViolation::UtilityDeadCode {
            stem,
            imported_by,
            reason,
        } => {
            let why = match reason.as_ref() {
                Some(r) => r.to_string(),
                None => {
                    let importers = imported_by.join(", ");
                    format!(
                        "Utility file '{}' is only imported by other utility files ({}), not by capability, agent, or surfaces layers.",
                        stem, importers
                    )
                }
            };
            format!(
                "AES504 UTILITY_DEAD_CODE: '{}' has no consumers in capability/agent/surfaces layers.\nWHY? {}\nFIX: Import '{}' in a capabilities_* file that needs its functionality, or delete it if unused. Utility files must be consumed by higher layers, not just other utilities.",
                stem, why, stem
            )
        }
        AesOrphanViolation::AgentOrphan { agg_name, reason } => {
            let why = match reason.as_ref() {
                Some(r) => r.to_string(),
                None => format!(
                    "Agent aggregate '{}' is not called by any surface or container.",
                    agg_name
                ),
            };
            format!(
                "AES505 AGENT_ORPHAN: Aggregate '{}' is unreachable from any surface.\nWHY? {}\nFIX: Import and use '{}' in a surface_* file or root_*_container.rs via `Arc<dyn {}>`. If the orchestrator is unused, delete it and remove its module declaration.",
                agg_name, why, agg_name, agg_name
            )
        }
        AesOrphanViolation::SurfaceOrphan {
            category,
            stem,
            reason,
        } => {
            let (why_line, fix_line) = match *category {
                "smart" => {
                    let why = match reason.as_ref() {
                        Some(r) => r.to_string(),
                        None => format!(
                            "the {} surface '{}' is not imported by any entry point or container such as root_*_entry.py/rs/ts.",
                            category, stem
                        ),
                    };
                    let fix = format!(
                        "Import '{}' at the entry point. If this surface is dead code, delete the file and its module declaration. Consider moving it to utility surface (_hook/_store/_action/_screen) or passive (surface _component/_view/_layout) if it is in the wrong role.",
                        stem
                    );
                    (why, fix)
                }
                "utility" => {
                    let why = match reason.as_ref() {
                        Some(r) => r.to_string(),
                        None => format!(
                            "the {} surface '{}' is not imported by any smart surface (command, controller, page, router).",
                            category, stem
                        ),
                    };
                    let fix = format!(
                        "Import '{}' by a smart surface (command, controller, page, router) or an entry point. If this surface is dead code, delete the file and its module declaration. Consider moving it to passive (surface _component/_view/_layout) if it is in the wrong role.",
                        stem
                    );
                    (why, fix)
                }
                "passive" => {
                    let why = match reason.as_ref() {
                        Some(r) => r.to_string(),
                        None => format!(
                            "the passive surface '{}' is not imported by any smart or utility surface.",
                            stem
                        ),
                    };
                    let fix = format!(
                        "Import '{}' by a smart or utility surface. If this surface is dead code, delete the file and its module declaration.",
                        stem
                    );
                    (why, fix)
                }
                _ => {
                    let why = match reason.as_ref() {
                        Some(r) => r.to_string(),
                        None => format!(
                            "the unknown surface '{}' is not imported by any appropriate importer.",
                            stem
                        ),
                    };
                    let fix = format!(
                        "Import '{}' in an appropriate importer file. If this surface is dead code, delete the file and its module declaration.",
                        stem
                    );
                    (why, fix)
                }
            };
            format!(
                "AES506 SURFACE_ORPHAN: {} surface '{}' is orphaned.\nWHY? {}\nFIX: {}",
                category, stem, why_line, fix_line
            )
        }
    }
}
