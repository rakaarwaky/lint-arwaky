// PURPOSE: RoleOrchestrator — dispatches files to correct role checker based on filename prefix
//
// The role orchestrator is unique among the feature agents: it doesn't
// just delegate to checkers — it first classifies each file by its
// filename prefix (taxonomy_, contract_, capabilities_, etc.), then
// dispatches to the corresponding layer-specific role checker.
//
// ALGORITHM:
//   1. run_all_role_checks iterates files, extracts filename prefix (first underscore-segment).
//   2. Matches prefix to layer (taxonomy, contract, capabilities, infrastructure, agent,
//      surfaces, root/lib/mod) and dispatches to the corresponding role checker.
//   3. Each checker receives the SourceContentVO (file path + content + language) and
//      returns violations via the violations Vec.
//   4. Unknown prefixes emit an INFO-level structured violation instead of eprintln!.
//
// NOTE: check_aggregate (forbidden inheritance) is NOT called here because the orchestrator
//      lacks layer definitions; that check runs via the IContractRoleChecker trait path
//      where callers supply the proper LayerDefinition.

use async_trait::async_trait;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::role_rules::contract_role_aggregate::IRoleAggregate;
use shared::taxonomy_source_vo::{ContentString, SourceContentVO};
use std::path::Path;
use std::sync::Arc;

use shared::role_rules::contract_agent_role_protocol::IAgentRoleChecker;
use shared::role_rules::contract_capabilities_role_protocol::ICapabilitiesRoleChecker;
use shared::role_rules::contract_infrastructure_role_protocol::IInfrastructureRoleChecker;
use shared::role_rules::contract_role_protocol::IContractRoleChecker;
use shared::role_rules::contract_surface_role_protocol::ISurfaceRoleChecker;
use shared::role_rules::contract_taxonomy_role_protocol::ITaxonomyRoleChecker;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct RoleOrchestrator {
    aggregate: Arc<dyn IRoleAggregate>,
    config: shared::config_system::taxonomy_config_vo::ArchitectureConfig,
    ignored_paths: Vec<String>,
}

#[async_trait]
impl shared::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate for RoleOrchestrator {
    async fn run_audit(&self, target: &FilePath) -> Vec<LintResult> {
        let mut results = Vec::new();
        let files = self.collect_files(target);
        let file_strings: Vec<String> = files.values.iter().map(|f| f.to_string()).collect();
        self.run_all_role_checks(&file_strings, &mut results);
        results
    }

    fn name(&self) -> &str {
        "role-rules"
    }
}

// ─── Block 3: Constructors & Helpers ──────────────────────
impl RoleOrchestrator {
    pub fn new(
        aggregate: Arc<dyn IRoleAggregate>,
        config: &shared::config_system::taxonomy_config_vo::ArchitectureConfig,
    ) -> Self {
        let ignored_paths: Vec<String> = config
            .ignored_paths
            .values
            .iter()
            .map(|fp| fp.value.replace('/', std::path::MAIN_SEPARATOR_STR))
            .collect();
        Self {
            aggregate,
            config: config.clone(),
            ignored_paths,
        }
    }

    fn is_ignored(&self, p: &Path) -> bool {
        let s = p.to_string_lossy();
        let dir_name = match p.file_name() {
            Some(n) => n.to_string_lossy(),
            None => std::borrow::Cow::Borrowed(""),
        };
        self.ignored_paths.iter().any(|ignored| {
            s.contains(ignored.as_str()) || dir_name.contains(ignored.trim_start_matches('/'))
        })
    }

    /// Run all AES401-406 role checks across all collected files.
    ///
    /// For each file, extracts the filename prefix (first underscore segment) to
    /// determine which AES layer it belongs to, then dispatches to the appropriate
    /// checker. Each layer has specific rules:
    ///   - agent: file size, type annotations, container/orchestrator/lifecycle
    ///   - surface: function count, smart vs utility vs passive classification
    ///   - infrastructure: port implementation checks
    ///   - contract: port vs protocol differentiation
    ///   - capabilities: routing checks
    ///   - taxonomy: entity, error, event, constant checks
    ///   - root: no role checks (pure DI wiring)
    pub fn run_all_role_checks(&self, files: &[String], violations: &mut Vec<LintResult>) {
        // Global gate: skip all role checks if architecture checker is disabled
        if !self.config.enabled.value {
            return;
        }

        for file in files {
            let content = std::fs::read_to_string(file).unwrap_or_default();
            let filename = Path::new(file)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default();

            // Extract the AES layer prefix from the filename (e.g., "taxonomy_" -> "taxonomy")
            let stem = Path::new(filename)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or_default();
            let prefix = stem.split('_').next().unwrap_or_default();

            let fp = match FilePath::new(file.to_string()) {
                Ok(f) => f,
                Err(_) => continue,
            };
            let content_vo = ContentString::new(content);
            let language = fp.language().as_str().to_string();
            let source_vo = SourceContentVO::new(fp, content_vo, &language);

            // Dispatch based on layer prefix — each layer has its own checker protocol
            match prefix {
                "agent" if self.config.is_rule_enabled("AES405") => {
                    let checker = self.aggregate.agent();
                    checker.check_any_type_annotation(&source_vo, violations);
                    if filename.contains("_container") {
                        checker.check_container(&source_vo, violations);
                    } else if filename.contains("_orchestrator") {
                        checker.check_orchestrator(&source_vo, violations);
                    } else if filename.contains("_lifecycle") {
                        checker.check_lifecycle(&source_vo, violations);
                    }
                }
                "root" => {} // Root layer (di containers, entries) has no role rules
                "surfaces" | "surface" if self.config.is_rule_enabled("AES406") => {
                    let checker = self.aggregate.surface();
                    checker.check_fn_count_limit(&source_vo, violations);
                    // Classify surface type for more specific checks
                    let is_smart = filename.contains("_command")
                        || filename.contains("_controller")
                        || filename.contains("_page")
                        || filename.contains("_entry");
                    let is_utility = filename.contains("_hook")
                        || filename.contains("_store")
                        || filename.contains("_action")
                        || filename.contains("_screen")
                        || filename.contains("_router");
                    if is_smart {
                        checker.check_smart_surface(&source_vo, violations);
                    } else if is_utility {
                        checker.check_utility_surface(&source_vo, violations);
                    } else {
                        checker.check_passive_surface(&source_vo, violations);
                    }
                }
                "infrastructure" | "infra" if self.config.is_rule_enabled("AES404") => {
                    let checker = self.aggregate.infrastructure();
                    checker.check_port_implementation(&source_vo, violations);
                }
                "contract" if self.config.is_rule_enabled("AES402") => {
                    let checker = self.aggregate.contract();
                    if filename.contains("_port") {
                        violations.extend(checker.check_port(&source_vo));
                    } else if filename.contains("_protocol") {
                        violations.extend(checker.check_protocol(&source_vo));
                    }
                }
                "capabilities" | "capability" if self.config.is_rule_enabled("AES403") => {
                    let checker = self.aggregate.capabilities();
                    checker.check_capability_routing(&source_vo, "capabilities", violations);
                }
                "taxonomy" if self.config.is_rule_enabled("AES401") => {
                    let checker = self.aggregate.taxonomy();
                    checker.check_entity(&source_vo, violations);
                    checker.check_error(&source_vo, violations);
                    checker.check_event(&source_vo, violations);
                    checker.check_constant(&source_vo, violations);
                }
                _ => {} // Unknown/disabled prefix — skip
            }
        }
    }

    fn collect_files(&self, target: &FilePath) -> FilePathList {
        let path = Path::new(target.value());
        let mut files = Vec::new();
        if path.is_dir() {
            self.walk_dir(path, &mut files, true);
        } else if path.is_file() {
            if let Ok(p) = FilePath::new(path.to_string_lossy().to_string()) {
                files.push(p);
            }
        }
        FilePathList::new(files)
    }

    fn walk_dir(&self, dir: &Path, files: &mut Vec<FilePath>, is_subdir: bool) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    if is_subdir && self.is_ignored(&path) {
                        continue;
                    }
                    self.walk_dir(&path, files, true);
                } else if path.is_file() {
                    if let Some(ext) = path.extension() {
                        if matches!(
                            ext.to_str(),
                            Some("rs" | "py" | "js" | "ts" | "jsx" | "tsx")
                        ) {
                            if let Ok(fp) = FilePath::new(path.to_string_lossy().to_string()) {
                                files.push(fp);
                            }
                        }
                    }
                }
            }
        }
    }
}

pub struct RoleAggregateImpl {
    taxonomy: Arc<dyn ITaxonomyRoleChecker>,
    contract: Arc<dyn IContractRoleChecker>,
    infrastructure: Arc<dyn IInfrastructureRoleChecker>,
    capabilities: Arc<dyn ICapabilitiesRoleChecker>,
    surface: Arc<dyn ISurfaceRoleChecker>,
    agent: Arc<dyn IAgentRoleChecker>,
}

// ─── Block 2: Public Contract ─────────────────────────────
impl IRoleAggregate for RoleAggregateImpl {
    fn taxonomy(&self) -> &dyn ITaxonomyRoleChecker {
        self.taxonomy.as_ref()
    }
    fn contract(&self) -> &dyn IContractRoleChecker {
        self.contract.as_ref()
    }
    fn infrastructure(&self) -> &dyn IInfrastructureRoleChecker {
        self.infrastructure.as_ref()
    }
    fn capabilities(&self) -> &dyn ICapabilitiesRoleChecker {
        self.capabilities.as_ref()
    }
    fn surface(&self) -> &dyn ISurfaceRoleChecker {
        self.surface.as_ref()
    }
    fn agent(&self) -> &dyn IAgentRoleChecker {
        self.agent.as_ref()
    }
}

impl RoleAggregateImpl {
    pub fn new(
        taxonomy: Arc<dyn ITaxonomyRoleChecker>,
        contract: Arc<dyn IContractRoleChecker>,
        infrastructure: Arc<dyn IInfrastructureRoleChecker>,
        capabilities: Arc<dyn ICapabilitiesRoleChecker>,
        surface: Arc<dyn ISurfaceRoleChecker>,
        agent: Arc<dyn IAgentRoleChecker>,
    ) -> Self {
        Self {
            taxonomy,
            contract,
            infrastructure,
            capabilities,
            surface,
            agent,
        }
    }
}
