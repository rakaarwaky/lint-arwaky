// PURPOSE: RoleOrchestrator — dispatches files to correct role checker based on filename prefix

use crate::output_report::taxonomy_result_vo::LintResult;
use crate::role_rules::contract_role_aggregate::IRoleAggregate;
use crate::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate;
use crate::shared_common::taxonomy_source_vo::{ContentString, SourceContentVO};
use crate::source_parsing::taxonomy_path_vo::FilePath;
use std::path::Path;

pub struct RoleOrchestrator {
    aggregate: Box<dyn IRoleAggregate>,
}

impl RoleOrchestrator {
    pub fn new(aggregate: Box<dyn IRoleAggregate>) -> Self {
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
                _ => {}
            }
        }
    }
}

#[async_trait::async_trait]
impl IRoleRunnerAggregate for RoleOrchestrator {
    async fn run_audit(&self, target: &FilePath) -> Vec<LintResult> {
        let mut results = Vec::new();
        let path_str = target.value();
        self.run_all_role_checks(&[path_str.to_string()], 500, &mut results);
        results
    }

    fn name(&self) -> &str { "role-rules" }
}
