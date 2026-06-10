// PURPOSE: RoleOrchestrator — dispatches files to correct role checker based on filename prefix

use crate::output_report::taxonomy_result_vo::LintResult;
use crate::role_rules::contract_role_aggregate::IRoleAggregate;
use std::path::Path;

pub struct RoleOrchestrator {
    aggregate: Box<dyn IRoleAggregate>,
}

impl RoleOrchestrator {
    pub fn new(aggregate: Box<dyn IRoleAggregate>) -> Self {
        Self { aggregate }
    }

    pub fn run_all_role_checks(&self, files: &[String], violations: &mut Vec<LintResult>) {
        for file in files {
            let content = std::fs::read_to_string(file).unwrap_or_default();
            let filename = Path::new(file)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("");

            let prefix = filename.split('_').next().unwrap_or("");

            match prefix {
                "agent" => {
                    let checker = self.aggregate.agent();
                    // All agent files get these checks
                    checker.check_file_size_limit(file, &content, violations);
                    checker.check_any_type_annotation(file, &content, violations);
                    // Suffix-specific checks
                    if filename.contains("_container") {
                        checker.check_container(file, &content, violations);
                    } else if filename.contains("_orchestrator") {
                        checker.check_orchestrator(file, &content, violations);
                    } else if filename.contains("_lifecycle") {
                        checker.check_lifecycle(file, &content, violations);
                    }
                }
                "surfaces" | "surface" => {
                    let checker = self.aggregate.surface();
                    // All surface files get this check
                    checker.check_fn_count_limit(file, &content, violations);
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
                        checker.check_smart_surface(file, &content, violations);
                    } else if is_utility {
                        checker.check_utility_surface(file, &content, violations);
                    } else {
                        checker.check_passive_surface(file, &content, violations);
                    }
                }
                "contract" => {
                    let checker = self.aggregate.contract();
                    let mut contract_violations = Vec::new();
                    if filename.contains("_port") {
                        contract_violations.extend(checker.check_port(file, &content, files));
                    } else if filename.contains("_protocol") {
                        contract_violations.extend(checker.check_protocol(file, &content, files));
                    }
                    checker.check_aggregate(file, &content, &Default::default(), &mut contract_violations);
                    violations.extend(contract_violations);
                }
                "taxonomy" => {
                    let checker = self.aggregate.taxonomy();
                    checker.check_entity(file, &content, violations);
                    checker.check_error(file, &content, violations);
                    checker.check_event(file, &content, violations);
                    checker.check_constant(file, violations);
                }
                _ => {}
            }
        }
    }
}
