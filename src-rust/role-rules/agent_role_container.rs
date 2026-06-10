// PURPOSE: RoleContainer — assembles concrete role checkers into IRoleAggregate implementation
use crate::role_rules::capabilities_agent_role_auditor::AgentRoleChecker;
use crate::role_rules::capabilities_contract_role_auditor::ContractRoleChecker;
use crate::role_rules::capabilities_surface_role_auditor::SurfaceRoleChecker;
use crate::role_rules::capabilities_taxonomy_role_auditor::TaxonomyRoleChecker;
use crate::role_rules::contract_agent_role_protocol::IAgentRoleChecker;
use crate::role_rules::contract_role_aggregate::IRoleAggregate;
use crate::role_rules::contract_role_protocol::IContractRoleChecker;
use crate::role_rules::contract_surface_role_protocol::ISurfaceRoleChecker;
use crate::role_rules::contract_taxonomy_role_protocol::ITaxonomyRoleChecker;

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

impl ITaxonomyRoleChecker for TaxonomyRoleChecker {
    fn check_vo(&self) -> Vec<crate::output_report::taxonomy_result_vo::LintResult> {
        self.check_vo()
    }
    fn check_entity(
        &self,
        file: &str,
        content: &str,
        violations: &mut Vec<crate::output_report::taxonomy_result_vo::LintResult>,
    ) {
        self.check_entity(file, content, violations);
    }
    fn check_error(
        &self,
        file: &str,
        content: &str,
        violations: &mut Vec<crate::output_report::taxonomy_result_vo::LintResult>,
    ) {
        self.check_error(file, content, violations);
    }
    fn check_event(
        &self,
        file: &str,
        content: &str,
        violations: &mut Vec<crate::output_report::taxonomy_result_vo::LintResult>,
    ) {
        self.check_event(file, content, violations);
    }
    fn check_constant(
        &self,
        file: &str,
        violations: &mut Vec<crate::output_report::taxonomy_result_vo::LintResult>,
    ) {
        self.check_constant(file, violations);
    }
}
impl IContractRoleChecker for ContractRoleChecker {
    fn check_port(&self, file: &str, content: &str, all_files: &[String]) -> Vec<crate::output_report::taxonomy_result_vo::LintResult> {
        self.check_port(file, content, all_files)
    }
    fn check_protocol(&self, file: &str, content: &str, all_files: &[String]) -> Vec<crate::output_report::taxonomy_result_vo::LintResult> {
        self.check_protocol(file, content, all_files)
    }
    fn check_aggregate(
        &self,
        file: &str,
        content: &str,
        def: &crate::shared_common::taxonomy_definition_vo::LayerDefinition,
        violations: &mut Vec<crate::output_report::taxonomy_result_vo::LintResult>,
    ) {
        self.check_aggregate(file, content, def, violations);
    }
}

impl ISurfaceRoleChecker for SurfaceRoleChecker {
    fn check_smart_surface(
        &self,
        _file: &str,
        _content: &str,
        _violations: &mut Vec<crate::output_report::taxonomy_result_vo::LintResult>,
    ) {
    }
    fn check_utility_surface(
        &self,
        _file: &str,
        _content: &str,
        _violations: &mut Vec<crate::output_report::taxonomy_result_vo::LintResult>,
    ) {
    }
    fn check_passive_surface(
        &self,
        _file: &str,
        _content: &str,
        _violations: &mut Vec<crate::output_report::taxonomy_result_vo::LintResult>,
    ) {
    }
    fn check_fn_count_limit(
        &self,
        file: &str,
        content: &str,
        violations: &mut Vec<crate::output_report::taxonomy_result_vo::LintResult>,
    ) {
        self.check_fn_count_limit(file, content, violations);
    }
}

impl IAgentRoleChecker for AgentRoleChecker {
    fn check_container(
        &self,
        _file: &str,
        _content: &str,
        _violations: &mut Vec<crate::output_report::taxonomy_result_vo::LintResult>,
    ) {
    }
    fn check_orchestrator(
        &self,
        _file: &str,
        _content: &str,
        _violations: &mut Vec<crate::output_report::taxonomy_result_vo::LintResult>,
    ) {
    }
    fn check_lifecycle(
        &self,
        _file: &str,
        _content: &str,
        _violations: &mut Vec<crate::output_report::taxonomy_result_vo::LintResult>,
    ) {
    }
    fn check_file_size_limit(
        &self,
        file: &str,
        content: &str,
        violations: &mut Vec<crate::output_report::taxonomy_result_vo::LintResult>,
    ) {
        self.check_file_size_limit(file, content, violations);
    }
    fn check_any_type_annotation(
        &self,
        file: &str,
        content: &str,
        violations: &mut Vec<crate::output_report::taxonomy_result_vo::LintResult>,
    ) {
        self.check_any_type_annotation(file, content, violations);
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
