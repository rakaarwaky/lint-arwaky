use crate::role_rules::contract_role_aggregate::IRoleAggregate;
use crate::source_parsing::taxonomy_paths_vo::FilePathList;
use crate::output_report::taxonomy_result_vo::LintResultList;

pub struct RoleOrchestrator {
    aggregate: Box<dyn IRoleAggregate>,
}

impl RoleOrchestrator {
    pub fn new(aggregate: Box<dyn IRoleAggregate>) -> Self {
        Self { aggregate }
    }

    pub async fn run_all_role_checks(
        &self,
        _files: &FilePathList,
        _violations: &mut LintResultList,
    ) {
        self.aggregate.taxonomy();
        self.aggregate.contract();
        self.aggregate.surface();
        self.aggregate.agent();
    }
}
