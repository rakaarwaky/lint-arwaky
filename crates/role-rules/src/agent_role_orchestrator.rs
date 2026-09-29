// PURPOSE: RoleOrchestrator — dispatches files to correct role checker based on filename prefix
//
// FRD-compliant: accepts pre-parsed FileEntry from the filesystem crate.
// No file I/O or AST parsing is performed internally.

use shared::common::taxonomy_layer_vo::LayerNameVO;
use shared::common::taxonomy_lint_result_vo::LintResult;
use shared::filesystem::taxonomy_filesystem_vo::FileEntry;
use shared::role_rules::contract_role_protocol::IAgentRoleProtocol;
use shared::role_rules::contract_role_protocol::ICapabilitiesRoleProtocol;
use shared::role_rules::contract_role_protocol::IClassificationProtocol;
use shared::role_rules::contract_role_protocol::IContractRoleProtocol;
use shared::role_rules::contract_role_protocol::ISurfaceRoleProtocol;
use shared::role_rules::contract_role_protocol::ITaxonomyRoleProtocol;
use shared::role_rules::contract_role_protocol::IUtilityRoleProtocol;
use shared::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate;
use shared::role_rules::taxonomy_role_request::RoleRequest;
use shared::role_rules::taxonomy_role_response::RoleResponse;
use std::path::Path;
use std::sync::Arc;

use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::filesystem::taxonomy_filesystem_vo::Language;

use shared::role_rules::utility_agent_role_checker::resolve_feature_protocol_count;
use shared::role_rules::utility_role_reference_scanner::build_external_reference_map;

// ─── Block 1: Struct Definitions ──────────────────────────

pub struct RoleCheckerDeps {
    pub taxonomy: Arc<dyn ITaxonomyRoleProtocol>,
    pub contract_rust: Arc<dyn IContractRoleProtocol>,
    pub contract_python: Arc<dyn IContractRoleProtocol>,
    pub contract_typescript: Arc<dyn IContractRoleProtocol>,
    pub capabilities_rust: Arc<dyn ICapabilitiesRoleProtocol>,
    pub capabilities_python: Arc<dyn ICapabilitiesRoleProtocol>,
    pub capabilities_typescript: Arc<dyn ICapabilitiesRoleProtocol>,
    pub capabilities: Arc<dyn ICapabilitiesRoleProtocol>,
    pub surface: Arc<dyn ISurfaceRoleProtocol>,
    pub agent_rust: Arc<dyn IAgentRoleProtocol>,
    pub agent_python: Arc<dyn IAgentRoleProtocol>,
    pub agent_ts: Arc<dyn IAgentRoleProtocol>,
    pub utility_rust: Arc<dyn IUtilityRoleProtocol>,
    pub utility_python: Arc<dyn IUtilityRoleProtocol>,
    pub utility_typescript: Arc<dyn IUtilityRoleProtocol>,
}

pub struct RoleOrchestrator {
    deps: RoleCheckerDeps,
    config: ArchitectureConfig,
    ignored_paths: Vec<String>,
}

// ─── FR-001: File Classification and Dispatch ─────────────

impl IClassificationProtocol for RoleOrchestrator {
    fn classify_layer(&self, file: &FileEntry) -> Option<LayerNameVO> {
        let filename = file
            .path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default();
        let stem = Path::new(filename)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default();
        match stem.split('_').next().unwrap_or_default() {
            "taxonomy" => Some(LayerNameVO::new("taxonomy")),
            "contract" => Some(LayerNameVO::new("contract")),
            "capabilities" | "capability" => Some(LayerNameVO::new("capabilities")),
            "utility" => Some(LayerNameVO::new("utility")),
            "agent" => Some(LayerNameVO::new("agent")),
            "surface" | "surfaces" => Some(LayerNameVO::new("surfaces")),
            // `root` is pure DI wiring; unrecognised prefixes are skipped.
            _ => None,
        }
    }
}

// ─── Block 2: Aggregate Trait Implementation ──────────────
impl IRoleRunnerAggregate for RoleOrchestrator {
    fn execute(&self, request: RoleRequest) -> RoleResponse {
        match request {
            RoleRequest::RunAuditWithEntries { files } => {
                let violations = self.run_audit_with_entries(&files);
                RoleResponse::Audit { violations }
            }
            RoleRequest::Name => RoleResponse::Name {
                name: self.name().to_string(),
            },
        }
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────
impl RoleOrchestrator {
    pub fn run_audit_with_entries(&self, files: &[FileEntry]) -> Vec<LintResult> {
        let mut results = Vec::new();
        self.run_all_role_checks(files, &mut results);
        results
    }

    pub fn name(&self) -> &str {
        "role-rules"
    }

    pub fn new(deps: RoleCheckerDeps, config: &ArchitectureConfig) -> Self {
        let ignored_paths: Vec<String> = config
            .ignored_paths
            .values
            .iter()
            .map(|fp| fp.value.replace('/', std::path::MAIN_SEPARATOR_STR))
            .collect();
        Self {
            deps,
            config: config.clone(),
            ignored_paths,
        }
    }

    /// Run all role checks on pre-parsed FileEntry slices.
    pub fn run_all_role_checks(&self, files: &[FileEntry], violations: &mut Vec<LintResult>) {
        if !self.config.enabled.value {
            return;
        }

        let references = build_external_reference_map(files);

        for file in files {
            if !file.parse_ok || file.content.is_empty() {
                continue;
            }

            let path_str = file.path.to_string_lossy().to_string();
            let filename = file
                .path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default();
            let stem = Path::new(filename)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or_default();
            let basename = stem;
            let prefix = basename.split('_').next().unwrap_or_default();

            // Skip barrel files (single source: shared::common::DEFAULT_RULE_EXCEPTIONS)
            if shared::common::DEFAULT_RULE_EXCEPTIONS.contains(&filename) || filename == "main.rs"
            {
                continue;
            }

            if self.is_ignored(&path_str) {
                continue;
            }

            match prefix {
                "agent"
                    if self.is_rule_enabled("AES405") && !self.is_exception("AES405", filename) =>
                {
                    let deps = &self.deps;
                    let auditor: &dyn IAgentRoleProtocol = match file.language {
                        Language::Rust => &*deps.agent_rust,
                        Language::Python => &*deps.agent_python,
                        Language::TypeScript | Language::JavaScript => &*deps.agent_ts,
                        _ => continue,
                    };
                    // Composition rules (implementor, type budget, Any) are
                    // grouped; the rest are called individually so each stays
                    // independently addressable. P14 takes the feature's
                    // declared protocol count because the auditor skips a
                    // feature that declares exactly one.
                    auditor.check_agent_routing(file, "agent", violations);
                    auditor.check_agent_block_order(file, violations);
                    auditor.check_agent_io_forbidden(file, violations);
                    auditor.check_agent_constant_placement(file, violations);
                    auditor.check_agent_computation(file, violations);
                    auditor.check_agent_abstract_method(file, violations);
                    auditor.check_agent_stateless(file, violations);
                    auditor.check_agent_free_fn(file, violations);
                    auditor.check_agent_subsystem_count(
                        file,
                        resolve_feature_protocol_count(&file.path),
                        violations,
                    );
                }
                "root" => {}
                "surfaces" | "surface"
                    if self.is_rule_enabled("AES406") && !self.is_exception("AES406", filename) =>
                {
                    self.deps.surface.check_fn_count_limit(file, violations);
                    match shared::role_rules::taxonomy_role_vo::classify_surface_tier(basename) {
                        shared::role_rules::taxonomy_role_vo::SurfaceTier::Smart => {
                            self.deps.surface.check_smart_surface(file, violations);
                        }
                        shared::role_rules::taxonomy_role_vo::SurfaceTier::Utility => {
                            self.deps.surface.check_utility_surface(file, violations);
                        }
                        shared::role_rules::taxonomy_role_vo::SurfaceTier::Passive => {
                            self.deps.surface.check_passive_surface(file, violations);
                        }
                    }
                }
                "contract"
                    if self.is_rule_enabled("AES402") && !self.is_exception("AES402", filename) =>
                {
                    let auditor = match file.language {
                        Language::Rust => &*self.deps.contract_rust,
                        Language::Python => &*self.deps.contract_python,
                        Language::TypeScript | Language::JavaScript => {
                            &*self.deps.contract_typescript
                        }
                        _ => continue,
                    };
                    auditor.check_contract_routing(file, "contract", violations);
                }
                "capabilities" | "capability"
                    if self.is_rule_enabled("AES403") && !self.is_exception("AES403", filename) =>
                {
                    let language = file.language;
                    let deps = &self.deps;
                    let auditor: &dyn ICapabilitiesRoleProtocol = match language {
                        Language::Rust => &*deps.capabilities_rust,
                        Language::Python => &*deps.capabilities_python,
                        Language::TypeScript | Language::JavaScript => {
                            &*deps.capabilities_typescript
                        }
                        _ => {
                            continue;
                        }
                    };
                    auditor.check_capability_routing_with_references(
                        file,
                        "capabilities",
                        &references,
                        violations,
                    );
                }
                "utility"
                    if self.is_rule_enabled("AES404") && !self.is_exception("AES404", filename) =>
                {
                    let language = file.language;
                    let deps = &self.deps;
                    let auditor: &dyn IUtilityRoleProtocol = match language {
                        Language::Rust => &*deps.utility_rust,
                        Language::Python => &*deps.utility_python,
                        Language::TypeScript | Language::JavaScript => &*deps.utility_typescript,
                        _ => continue,
                    };
                    auditor.check_utility_convention(file, violations);
                }
                "taxonomy"
                    if self.is_rule_enabled("AES401") && !self.is_exception("AES401", filename) =>
                {
                    self.deps.taxonomy.check_entity(file, violations);
                    self.deps.taxonomy.check_error(file, violations);
                    self.deps.taxonomy.check_event(file, violations);
                    self.deps.taxonomy.check_constant(file, violations);
                }
                _ => {}
            }
        }
    }

    fn is_ignored(&self, path: &str) -> bool {
        let segments: Vec<&str> = path.split('/').collect();
        self.ignored_paths.iter().any(|ignored| {
            let ignored_segments: Vec<&str> = ignored.split('/').collect();
            ignored_segments
                .iter()
                .all(|igs| segments.iter().any(|s| s == igs))
        })
    }

    fn is_exception(&self, code: &str, filename: &str) -> bool {
        let stem = std::path::Path::new(filename)
            .file_stem()
            .and_then(|s| s.to_str());
        self.config.rules.iter().any(|r| {
            r.rule_type.code() == code
                && r.exceptions
                    .values
                    .iter()
                    .any(|e| e == filename || stem.is_some_and(|s| e == s))
        })
    }

    fn is_rule_enabled(&self, code: &str) -> bool {
        self.config
            .rules
            .iter()
            .find(|r| r.rule_type.code() == code)
            .map(|r| r.enabled.value)
            .unwrap_or(true)
    }
}
