// PURPOSE: Route files to the correct role checker based on filename prefix (taxonomy/contract/agent/surface).
// Dispatches to ITaxonomyRoleChecker/IContractRoleChecker for actual enforcement.
// aes: wired-by-dispatch
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
                _ => {}
            }
        }
    }
}
