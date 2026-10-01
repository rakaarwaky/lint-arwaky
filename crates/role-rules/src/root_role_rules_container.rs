// PURPOSE: RoleContainer — DI wiring for role-rules feature (root layer, wiring only)
//
// Wires all capabilities into RoleCheckerDeps, then exposes the orchestrator
// via the IRoleRunnerAggregate contract. No business logic lives here.

use crate::agent_role_orchestrator::{RoleCheckerDeps, RoleOrchestrator};
use crate::capabilities_agent_python_role_auditor::AgentPythonRoleAuditor;
use crate::capabilities_agent_rust_role_auditor::AgentRustRoleAuditor;
use crate::capabilities_agent_ts_role_auditor::AgentTsRoleAuditor;
use crate::capabilities_capabilities_python_role_auditor::CapabilitiesPythonRoleAuditor;
use crate::capabilities_capabilities_rust_role_auditor::CapabilitiesRustRoleAuditor;
use crate::capabilities_capabilities_ts_role_auditor::CapabilitiesTypeScriptRoleAuditor;
use crate::capabilities_contract_python_role_auditor::ContractPythonRoleAuditor;
use crate::capabilities_contract_rust_role_auditor::ContractRustRoleAuditor;
use crate::capabilities_contract_ts_role_auditor::ContractTypeScriptRoleAuditor;
use crate::capabilities_surface_role_auditor::SurfaceRoleChecker;
use crate::capabilities_taxonomy_role_auditor::TaxonomyRoleChecker;
use crate::capabilities_utility_python_role_auditor::UtilityPythonRoleAuditor;
use crate::capabilities_utility_rust_role_auditor::UtilityRustRoleAuditor;
use crate::capabilities_utility_ts_role_auditor::UtilityTypeScriptRoleAuditor;
use shared_config_system::taxonomy_config_system_vo::ArchitectureConfig;
use shared_role_rules::IRoleRunnerAggregate;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct RoleContainer {
    deps: RoleCheckerDeps,
    config: ArchitectureConfig,
}

// ─── Block 2: Constructors, Helpers, Private Methods ──────

impl RoleContainer {
    pub fn new_with_config(config: ArchitectureConfig) -> Self {
        let rust_auditor = Arc::new(CapabilitiesRustRoleAuditor::new());
        let python_auditor = Arc::new(CapabilitiesPythonRoleAuditor::new());
        let ts_auditor = Arc::new(CapabilitiesTypeScriptRoleAuditor::new());
        let agent_rust = Arc::new(AgentRustRoleAuditor::new());
        let agent_python = Arc::new(AgentPythonRoleAuditor::new());
        let agent_ts = Arc::new(AgentTsRoleAuditor::new());
        let deps = RoleCheckerDeps {
            taxonomy: Arc::new(TaxonomyRoleChecker::new()),
            contract_rust: Arc::new(ContractRustRoleAuditor::new()),
            contract_python: Arc::new(ContractPythonRoleAuditor::new()),
            contract_typescript: Arc::new(ContractTypeScriptRoleAuditor::new()),
            capabilities_rust: rust_auditor.clone(),
            capabilities_python: python_auditor.clone(),
            capabilities_typescript: ts_auditor.clone(),
            capabilities: rust_auditor,
            surface: Arc::new(SurfaceRoleChecker::new()),
            agent_rust,
            agent_python,
            agent_ts,
            utility_rust: Arc::new(UtilityRustRoleAuditor::new()),
            utility_python: Arc::new(UtilityPythonRoleAuditor::new()),
            utility_typescript: Arc::new(UtilityTypeScriptRoleAuditor::new()),
        };
        Self { deps, config }
    }

    pub fn orchestrator(&self) -> Arc<dyn IRoleRunnerAggregate> {
        let deps = RoleCheckerDeps {
            taxonomy: Arc::clone(&self.deps.taxonomy),
            contract_rust: Arc::clone(&self.deps.contract_rust),
            contract_python: Arc::clone(&self.deps.contract_python),
            contract_typescript: Arc::clone(&self.deps.contract_typescript),
            capabilities_rust: Arc::clone(&self.deps.capabilities_rust),
            capabilities_python: Arc::clone(&self.deps.capabilities_python),
            capabilities_typescript: Arc::clone(&self.deps.capabilities_typescript),
            capabilities: Arc::clone(&self.deps.capabilities),
            surface: Arc::clone(&self.deps.surface),
            agent_rust: Arc::clone(&self.deps.agent_rust),
            agent_python: Arc::clone(&self.deps.agent_python),
            agent_ts: Arc::clone(&self.deps.agent_ts),
            utility_rust: Arc::clone(&self.deps.utility_rust),
            utility_python: Arc::clone(&self.deps.utility_python),
            utility_typescript: Arc::clone(&self.deps.utility_typescript),
        };
        Arc::new(RoleOrchestrator::new(deps, &self.config))
    }
}
