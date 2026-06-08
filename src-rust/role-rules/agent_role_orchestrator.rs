use crate::role_rules::capabilities_role_checker::ArchRoleChecker;
use crate::layer_rules::contract_rule_protocol::IAnalyzer;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use crate::source_parsing::taxonomy_paths_vo::FilePathList;
use crate::output_report::taxonomy_result_vo::LintResultList;

pub struct RoleOrchestrator {
    checker: ArchRoleChecker,
}

impl RoleOrchestrator {
    pub fn new() -> Self {
        Self {
            checker: ArchRoleChecker::new(),
        }
    }

    pub async fn run_role_checks(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    ) {
        self.checker
            .check_agent_roles(analyzer, files, root_dir, results)
            .await;
        self.checker
            .check_surface_roles(analyzer, files, root_dir, results)
            .await;
    }
}
