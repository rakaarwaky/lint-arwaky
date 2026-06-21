// PURPOSE: RoleOrchestrator — dispatches files to correct role checker based on filename prefix

use async_trait::async_trait;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::role_rules::contract_role_aggregate::IRoleAggregate;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::source_parsing::taxonomy_paths_vo::FilePathList;
use shared::taxonomy_source_vo::{ContentString, SourceContentVO};
use std::path::Path;
use std::sync::Arc;

use shared::role_rules::contract_agent_role_protocol::IAgentRoleChecker;
use shared::role_rules::contract_capabilities_role_protocol::ICapabilitiesRoleChecker;
use shared::role_rules::contract_infrastructure_role_protocol::IInfrastructureRoleChecker;
use shared::role_rules::contract_role_protocol::IContractRoleChecker;
use shared::role_rules::contract_surface_role_protocol::ISurfaceRoleChecker;
use shared::role_rules::contract_taxonomy_role_protocol::ITaxonomyRoleChecker;

pub struct RoleOrchestrator {
    aggregate: Arc<dyn IRoleAggregate>,
    ignored_paths: Vec<String>,
}

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
            ignored_paths,
        }
    }

    fn is_ignored(&self, p: &Path) -> bool {
        let s = p.to_string_lossy();
        let dir_name = p
            .file_name()
            .map(|n| n.to_string_lossy())
            .unwrap_or_default();
        self.ignored_paths.iter().any(|ignored| {
            s.contains(ignored.as_str()) || dir_name.contains(ignored.trim_start_matches('/'))
        })
    }

    pub fn run_all_role_checks(
        &self,
        files: &[String],
        max_lines: usize,
        violations: &mut Vec<LintResult>,
    ) {
        for file in files {
            let content = std::fs::read_to_string(file).unwrap_or_default();
            let filename = Path::new(file)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("");

            let stem = Path::new(filename)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("");
            let prefix = stem.split('_').next().unwrap_or("");

            let fp = FilePath::new(file.to_string()).unwrap_or_default();
            let content_vo = ContentString::new(content);
            let detector =
                shared::source_parsing::taxonomy_language_detector_helper::LanguageDetector::new();
            let language = detector.detect(&fp).as_str().to_string();
            let source_vo = SourceContentVO::new(fp, content_vo, &language);

            match prefix {
                "agent" => {
                    let checker = self.aggregate.agent();
                    checker.check_file_size_limit(&source_vo, max_lines, violations);
                    checker.check_any_type_annotation(&source_vo, violations);
                    if filename.contains("_container") {
                        checker.check_container(&source_vo, violations);
                    } else if filename.contains("_orchestrator") {
                        checker.check_orchestrator(&source_vo, violations);
                    } else if filename.contains("_lifecycle") {
                        checker.check_lifecycle(&source_vo, violations);
                    }
                }
                "root" => {}
                "surfaces" | "surface" => {
                    let checker = self.aggregate.surface();
                    checker.check_fn_count_limit(&source_vo, violations);
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
                "infrastructure" | "infra" => {
                    let checker = self.aggregate.infrastructure();
                    checker.check_port_implementation(&source_vo, violations);
                }
                "contract" => {
                    let checker = self.aggregate.contract();
                    let mut contract_violations = Vec::new();
                    if filename.contains("_port") {
                        contract_violations.extend(checker.check_port(&source_vo));
                    } else if filename.contains("_protocol") {
                        contract_violations.extend(checker.check_protocol(&source_vo));
                    }
                    checker.check_aggregate(
                        &source_vo,
                        &Default::default(),
                        &mut contract_violations,
                    );
                    violations.extend(contract_violations);
                }
                "capabilities" | "capability" => {
                    let checker = self.aggregate.capabilities();
                    checker.check_capability_routing(&source_vo, "capabilities", violations);
                }
                "taxonomy" => {
                    let checker = self.aggregate.taxonomy();
                    checker.check_entity(&source_vo, violations);
                    checker.check_error(&source_vo, violations);
                    checker.check_event(&source_vo, violations);
                    checker.check_constant(&source_vo, violations);
                }
                "lib" | "mod" => {}
                other => {
                    eprintln!("Warning: unhandled case {:?} in {}", other, module_path!());
                }
            }
        }
    }

    fn collect_files(&self, target: &FilePath) -> FilePathList {
        let path = Path::new(target.value());
        let mut files = Vec::new();
        if path.is_dir() {
            self.walk_dir(path, &mut files, true);
        } else if path.is_file() {
            files.push(FilePath::new(path.to_string_lossy().to_string()).unwrap_or_default());
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
                            files.push(
                                FilePath::new(path.to_string_lossy().to_string())
                                    .unwrap_or_default(),
                            );
                        }
                    }
                }
            }
        }
    }
}

#[async_trait]
impl shared::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate for RoleOrchestrator {
    async fn run_audit(&self, target: &FilePath) -> Vec<LintResult> {
        let mut results = Vec::new();
        let files = self.collect_files(target);
        let file_strings: Vec<String> = files.values.iter().map(|f| f.to_string()).collect();
        self.run_all_role_checks(&file_strings, 500, &mut results);
        results
    }

    fn name(&self) -> &str {
        "role-rules"
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
