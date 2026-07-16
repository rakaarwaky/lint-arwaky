use crate::common::taxonomy_message_vo::LintMessage;
use std::fmt;

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
        target_layer: String,
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
                write!(f, "AES501 TAXONOMY_ORPHAN: '{}' is not imported.\nWHY? {}\nFIX: Import '{}' in {}.", stem, why, stem, target_hint)
            }
            AesOrphanViolation::ContractOrphan {
                suffix,
                trait_name,
                target_layer,
                reason,
            } => {
                let why = match reason.as_ref() {
                    Some(r) => r.to_string(),
                    None => match suffix.as_str() {
                        "port" => format!(
                            "Contract port '{}' failed 3 modular checks:\n\
                             1. IMPLEMENT: Not implemented by any infrastructure_* file \
                             (no adapter provides the outbound interface via `impl Trait for Type`)\n\
                             2. CALL: Not called by any orchestrator, container, capabilities, or surface \
                             (no file uses `TraitName::method()` or imports the trait)\n\
                             3. WIRE: Not wired in any DI container \
                             (no `Arc::new(Type::new())` or constructor injection)\n\
                             Without implement + call + wire, the port is dead code.",
                            trait_name
                        ),
                        "protocol" => format!(
                            "Contract protocol '{}' failed 3 modular checks:\n\
                             1. IMPLEMENT: Not implemented by any capabilities_* file \
                             (no checker/analyzer provides the inbound interface via `impl Trait for Type`)\n\
                             2. CALL: Not called by any orchestrator, container, capabilities, or surface \
                             (no file uses `TraitName::method()` or imports the trait)\n\
                             3. WIRE: Not wired in any DI container \
                             (no `Arc::new(Type::new())` or constructor injection)\n\
                             Without implement + call + wire, the protocol is dead code.",
                            trait_name
                        ),
                        "aggregate" => format!(
                            "Contract aggregate '{}' failed 3 modular checks:\n\
                             1. IMPLEMENT: Not implemented by any agent_*_orchestrator.rs file \
                             (no orchestrator coordinates this workflow via `impl Trait for Type`)\n\
                             2. CALL: Not called by any surface_* file \
                             (no surface invokes this aggregate's methods)\n\
                             3. WIRE: Not wired in any root_*_container.rs \
                             (no `Arc::new(Type::new())` or constructor injection)\n\
                             Without implement + call + wire, the orchestration logic is dead code.",
                            trait_name
                        ),
                        _ => format!(
                            "Contract '{}' is not implemented by any {} file.",
                            trait_name, target_layer
                        ),
                    },
                };
                let fix = match suffix.as_str() {
                    "port" => format!(
                        "1. IMPLEMENT: Create infrastructure_<name>_<suffix>.rs with `impl {} for Type` \
                         — the adapter uses the port's interface to interact with external systems.\n\
                         2. CALL: Import and use the trait in agent_*_orchestrator.rs \
                         — call `trait_name::method()` or reference the trait.\n\
                         3. WIRE: Pass `Arc::new(Type::new())` to the orchestrator constructor \
                         in root_*_container.rs.",
                        trait_name
                    ),
                    "protocol" => format!(
                        "1. IMPLEMENT: Create capabilities_<name>_<suffix>.rs with `impl {} for Type` \
                         — the capability uses the protocol's interface to process domain logic.\n\
                         2. CALL: Import and use the trait in agent_*_orchestrator.rs \
                         — call `trait_name::method()` or reference the trait.\n\
                         3. WIRE: Pass `Arc::new(Type::new())` to the orchestrator constructor \
                         in root_*_container.rs.",
                        trait_name
                    ),
                    "aggregate" => format!(
                        "1. IMPLEMENT: Add `impl {} for Type` in agent_*_orchestrator.rs \
                         — the orchestrator coordinates the workflow.\n\
                         2. CALL: Import and use the aggregate in a surface_* file \
                         — call `aggregate::execute()` or reference the trait.\n\
                         3. WIRE: Pass `Arc::new(Type::new())` to the surface constructor \
                         in root_*_container.rs.",
                        trait_name
                    ),
                    _ => format!(
                        "Implement '{}' in the appropriate layer.",
                        trait_name
                    ),
                };
                write!(
                    f,
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
                write!(f, "AES503 CAPABILITIES_ORPHAN: '{}' is not wired.\nWHY? {}\nFIX: Register '{}' in root_*_entry.rs or root_*_container.rs via `use {}::...;` and wire it into the container's constructor. If this file is obsolete, delete it and remove its module declaration from lib.rs.", stem, why, stem, stem)
            }
            AesOrphanViolation::InfrastructureOrphan { stem, reason } => {
                let why = match reason.as_ref() {
                    Some(r) => r.to_string(),
                    None => format!("Infrastructure file '{}' is not wired in any container and unreachable from any entry point.", stem),
                };
                write!(f, "AES504 INFRASTRUCTURE_ORPHAN: '{}' is not wired.\nWHY? {}\nFIX: Register '{}' in the corresponding agent_*_orchestrator.rs or root_*_container.rs by passing it as a dependency. If this adapter is unused, delete it and remove its module declaration.", stem, why, stem)
            }
            AesOrphanViolation::AgentOrphan { agg_name, reason } => {
                let why = match reason.as_ref() {
                    Some(r) => r.to_string(),
                    None => format!(
                        "Agent aggregate '{}' is not called by any surface or container.",
                        agg_name
                    ),
                };
                write!(f, "AES505 AGENT_ORPHAN: Aggregate '{}' is unreachable from any surface.\nWHY? {}\nFIX: Import and use '{}' in a surface_* file or root_*_container.rs via `Arc<dyn {}>`. If the orchestrator is unused, delete it and remove its module declaration.", agg_name, why, agg_name, agg_name)
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
                let why = match reason.as_ref() {
                    Some(r) => r.to_string(),
                    None => format!(
                        "{} surface '{}' is not imported by any {}.",
                        category, stem, where_hint
                    ),
                };
                write!(f, "AES506 SURFACE_ORPHAN: {} surface '{}' is orphaned.\nWHY? {}\nFIX: Import '{}' in {}. If this surface is dead code, remove the file and its module declaration from lib.rs.", category, stem, why, stem, fix_hint)
            }
        }
    }
}

impl From<AesOrphanViolation> for String {
    fn from(v: AesOrphanViolation) -> String {
        v.to_string()
    }
}
