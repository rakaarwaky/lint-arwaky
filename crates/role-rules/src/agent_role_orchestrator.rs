// PURPOSE: RoleOrchestrator — dispatches files to correct role checker based on filename prefix
//
// FRD-compliant: accepts pre-parsed FileEntry from the filesystem crate.
// No file I/O or AST parsing is performed internally.

use shared::common::taxonomy_lint_result_vo::LintResult;
use shared::filesystem::taxonomy_filesystem_vo::FileEntry;
use shared::role_rules::contract_agent_role_protocol::IAgentRoleChecker;
use shared::role_rules::contract_capabilities_role_protocol::ICapabilitiesRoleChecker;
use shared::role_rules::contract_role_contract_protocol::IContractRoleChecker;
use shared::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate;
use shared::role_rules::contract_surface_role_protocol::ISurfaceRoleChecker;
use shared::role_rules::contract_taxonomy_role_protocol::ITaxonomyRoleChecker;
use shared::role_rules::contract_utility_role_protocol::IUtilityRoleChecker;
use std::path::Path;
use std::sync::Arc;

use shared::config_system::taxonomy_config_vo::ArchitectureConfig;

// ─── Block 1: Struct Definitions ──────────────────────────

pub struct RoleCheckerDeps {
    pub taxonomy: Arc<dyn ITaxonomyRoleChecker>,
    pub contract: Arc<dyn IContractRoleChecker>,
    pub capabilities: Arc<dyn ICapabilitiesRoleChecker>,
    pub surface: Arc<dyn ISurfaceRoleChecker>,
    pub agent: Arc<dyn IAgentRoleChecker>,
    pub utility: Arc<dyn IUtilityRoleChecker>,
}

pub struct RoleOrchestrator {
    deps: RoleCheckerDeps,
    config: ArchitectureConfig,
    ignored_paths: Vec<String>,
}

// ─── Block 2: Aggregate Trait Implementation ──────────────
impl IRoleRunnerAggregate for RoleOrchestrator {
    fn run_audit_with_entries(&self, files: &[FileEntry]) -> Vec<LintResult> {
        let mut results = Vec::new();
        self.run_all_role_checks(files, &mut results);
        results
    }

    fn name(&self) -> &str {
        "role-rules"
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────
impl RoleOrchestrator {
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

        for file in files {
            if !file.parse_ok || file.content.is_empty() {
                continue;
            }

            let path_str = file.path.to_string_lossy();
            let filename = file
                .path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default();
            let stem = Path::new(filename)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or_default();
            let prefix = stem.split('_').next().unwrap_or_default();

            // Skip barrel files (single source: shared::common::DEFAULT_RULE_EXCEPTIONS)
            if shared::common::DEFAULT_RULE_EXCEPTIONS.contains(&filename) || filename == "main.rs"
            {
                continue;
            }

            // Skip files in rule-specific exceptions from config
            if self.is_exception(filename) {
                continue;
            }

            if self.is_ignored(&path_str) {
                continue;
            }

            match prefix {
                "agent" => {
                    self.deps
                        .agent
                        .check_agent_routing(file, "agent", violations);
                }
                "root" => {}
                "surfaces" | "surface" => {
                    self.deps.surface.check_fn_count_limit(file, violations);
                    let is_smart = filename.contains("_command")
                        || filename.contains("_controller")
                        || filename.contains("_page")
                        || filename.contains("_entry")
                        || filename.contains("_router");
                    let is_utility = filename.contains("_hook")
                        || filename.contains("_store")
                        || filename.contains("_action")
                        || filename.contains("_screen");
                    if is_smart {
                        self.deps.surface.check_smart_surface(file, violations);
                    } else if is_utility {
                        self.deps.surface.check_utility_surface(file, violations);
                    } else {
                        self.deps.surface.check_passive_surface(file, violations);
                    }
                }
                "contract" => {
                    if filename.contains("_protocol") {
                        violations.extend(self.deps.contract.check_protocol(file));
                    } else if filename.contains("_aggregate") {
                        violations.extend(self.deps.contract.check_aggregate(file));
                    }
                }
                "capabilities" | "capability" => {
                    self.deps.capabilities.check_capability_routing(
                        file,
                        "capabilities",
                        violations,
                    );
                }
                "utility" => {
                    self.deps.utility.check_utility_convention(file, violations);
                }
                "taxonomy" => {
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

    fn is_exception(&self, filename: &str) -> bool {
        // Check if filename is in any rule's exceptions list
        for rule in &self.config.rules {
            if rule.exceptions.values.iter().any(|e| e == filename) {
                return true;
            }
        }
        false
    }
}
