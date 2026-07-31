// PURPOSE: RoleOrchestrator — dispatches files to correct role checker based on filename prefix
//
// The role orchestrator is unique among the feature agents: it doesn't
// just delegate to checkers — it first classifies each file by its
// filename prefix (taxonomy_, contract_, capabilities_, etc.), then
// dispatches to the corresponding layer-specific role checker.
//
// ALGORITHM:
//   1. run_all_role_checks iterates files, extracts filename prefix (first underscore-segment).
//   2. Matches prefix to layer (taxonomy, contract, utility, capabilities, agent,
//      surfaces, root/lib/mod) and dispatches to the corresponding role checker.
//   3. Each checker receives the SourceContentVO (file path + content + language) and
//      returns violations via the violations Vec.
//   4. Unknown prefixes are silently skipped (handled by other crates).

use async_trait::async_trait;
use shared::cli_commands::LintResult;
use shared::common::{FilePath, FilePathList};

use shared::common::utility_language_detector::detect_language;
use shared::role_rules::{
    IAgentRoleChecker, ICapabilitiesRoleChecker, IContractRoleChecker, IRoleRunnerAggregate,
    ISurfaceRoleChecker, ITaxonomyRoleChecker, IUtilityRoleChecker,
};

use filesystem::utility_filesystem_io::{read_file, walk_source_files};
use shared::common::{ContentString, SourceContentVO};
use std::path::Path;
use std::sync::Arc;

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
    config: shared::config_system::ArchitectureConfig,
    ignored_paths: Vec<String>,
}

// ─── Block 2: Aggregate Trait Implementation ──────────────

#[async_trait]
impl IRoleRunnerAggregate for RoleOrchestrator {
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

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl RoleOrchestrator {
    pub fn new(deps: RoleCheckerDeps, config: &shared::config_system::ArchitectureConfig) -> Self {
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

    pub fn run_all_role_checks(&self, files: &[String], violations: &mut Vec<LintResult>) {
        if !self.config.enabled.value {
            return;
        }

        for file in files {
            let content = read_file(file).unwrap_or_default();
            let filename = Path::new(file)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default();

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
            let language = detect_language(&fp).as_str().to_string();
            let source_vo = SourceContentVO::new(fp, content_vo, &language);

            match prefix {
                "agent" => {
                    self.deps
                        .agent
                        .check_agent_routing(&source_vo, "agent", violations);
                }
                "root" => {}
                "surfaces" | "surface" => {
                    self.deps
                        .surface
                        .check_fn_count_limit(&source_vo, violations);
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
                        self.deps
                            .surface
                            .check_smart_surface(&source_vo, violations);
                    } else if is_utility {
                        self.deps
                            .surface
                            .check_utility_surface(&source_vo, violations);
                    } else {
                        self.deps
                            .surface
                            .check_passive_surface(&source_vo, violations);
                    }
                }
                "contract" => {
                    if filename.contains("_protocol") {
                        violations.extend(self.deps.contract.check_protocol(&source_vo));
                    } else if filename.contains("_aggregate") {
                        violations.extend(self.deps.contract.check_aggregate(&source_vo));
                    }
                }
                "capabilities" | "capability" => {
                    self.deps.capabilities.check_capability_routing(
                        &source_vo,
                        "capabilities",
                        violations,
                    );
                }
                "utility" => {
                    self.deps
                        .utility
                        .check_utility_convention(&source_vo, violations);
                }
                "taxonomy" => {
                    self.deps.taxonomy.check_entity(&source_vo, violations);
                    self.deps.taxonomy.check_error(&source_vo, violations);
                    self.deps.taxonomy.check_event(&source_vo, violations);
                    self.deps.taxonomy.check_constant(&source_vo, violations);
                }
                _ => {}
            }
        }
    }

    fn collect_files(&self, target: &FilePath) -> FilePathList {
        let path = Path::new(target.value());
        let mut files = Vec::new();
        if path.is_dir() {
            walk_source_files(path, &mut files, &self.ignored_paths);
        } else if path.is_file()
            && let Ok(p) = FilePath::new(path.to_string_lossy().to_string())
        {
            files.push(p);
        }
        FilePathList::new(files)
    }
}
