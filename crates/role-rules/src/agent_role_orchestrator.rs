// PURPOSE: RoleOrchestrator — dispatches files to correct role checker based on filename prefix

use async_trait::async_trait;
use shared::output_report::taxonomy_result_vo::LintResult;
use shared::role_rules::contract_role_aggregate::IRoleAggregate;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::source_parsing::taxonomy_paths_vo::FilePathList;
use shared::taxonomy_source_vo::{ContentString, SourceContentVO};
use std::path::Path;
use std::sync::Arc;

use crate::capabilities_agent_role_auditor::AgentRoleChecker;
use crate::capabilities_contract_role_auditor::ContractRoleChecker;
use crate::capabilities_surface_role_auditor::SurfaceRoleChecker;
use crate::capabilities_taxonomy_role_auditor::TaxonomyRoleChecker;
use shared::role_rules::contract_agent_role_protocol::IAgentRoleChecker;
use shared::role_rules::contract_role_protocol::IContractRoleChecker;
use shared::role_rules::contract_surface_role_protocol::ISurfaceRoleChecker;
use shared::role_rules::contract_taxonomy_role_protocol::ITaxonomyRoleChecker;

pub struct RoleOrchestrator {
    aggregate: Arc<dyn IRoleAggregate>,
}

impl RoleOrchestrator {
    pub fn new(aggregate: Arc<dyn IRoleAggregate>) -> Self {
        Self { aggregate }
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

            let prefix = filename.split('_').next().unwrap_or("");

            let fp = FilePath::new(file.to_string()).unwrap_or_default();
            let content_vo = ContentString::new(content);
            let language = if file.ends_with(".rs") {
                "rust"
            } else if file.ends_with(".py") {
                "python"
            } else if file.ends_with(".js")
                || file.ends_with(".ts")
                || file.ends_with(".jsx")
                || file.ends_with(".tsx")
            {
                "javascript"
            } else {
                "unknown"
            };
            let source_vo = SourceContentVO::new(fp, content_vo, language);

            match prefix {
                "agent" => {
                    let checker = self.aggregate.agent();
                    // All agent files get these checks
                    checker.check_file_size_limit(&source_vo, max_lines, violations);
                    checker.check_any_type_annotation(&source_vo, violations);
                    // Suffix-specific checks
                    if filename.contains("_container") {
                        checker.check_container(&source_vo, violations);
                    } else if filename.contains("_orchestrator") {
                        checker.check_orchestrator(&source_vo, violations);
                    } else if filename.contains("_lifecycle") {
                        checker.check_lifecycle(&source_vo, violations);
                    }
                }
                "surfaces" | "surface" => {
                    let checker = self.aggregate.surface();
                    // All surface files get this check
                    checker.check_fn_count_limit(&source_vo, violations);
                    // Type-specific checks
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
                "taxonomy" => {
                    let checker = self.aggregate.taxonomy();
                    checker.check_entity(&source_vo, violations);
                    checker.check_error(&source_vo, violations);
                    checker.check_event(&source_vo, violations);
                    checker.check_constant(&source_vo, violations);
                }
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
            self.walk_dir(path, &mut files);
        } else if path.is_file() {
            files.push(FilePath::new(path.to_string_lossy().to_string()).unwrap_or_default());
        }
        FilePathList::new(files)
    }

    fn walk_dir(&self, dir: &Path, files: &mut Vec<FilePath>) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    self.walk_dir(&path, files);
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
    taxonomy: TaxonomyRoleChecker,
    contract: ContractRoleChecker,
    surface: SurfaceRoleChecker,
    agent: AgentRoleChecker,
}

impl Default for RoleAggregateImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl RoleAggregateImpl {
    pub fn new() -> Self {
        Self {
            taxonomy: TaxonomyRoleChecker::new(),
            contract: ContractRoleChecker::new(),
            surface: SurfaceRoleChecker::new(),
            agent: AgentRoleChecker::new(),
        }
    }
}

impl IRoleAggregate for RoleAggregateImpl {
    fn taxonomy(&self) -> &dyn ITaxonomyRoleChecker {
        &self.taxonomy
    }
    fn contract(&self) -> &dyn IContractRoleChecker {
        &self.contract
    }
    fn surface(&self) -> &dyn ISurfaceRoleChecker {
        &self.surface
    }
    fn agent(&self) -> &dyn IAgentRoleChecker {
        &self.agent
    }
}
