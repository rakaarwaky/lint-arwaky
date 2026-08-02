// PURPOSE: RoleContainer — DI wiring for role-rules feature (root layer, wiring only)
//
// Wires all capabilities into RoleCheckerDeps, then exposes the orchestrator
// via the IRoleRunnerAggregate contract. No business logic lives here.

use crate::agent_role_orchestrator::{RoleCheckerDeps, RoleOrchestrator};
use crate::capabilities_agent_role_auditor::AgentRoleChecker;
use crate::capabilities_capabilities_role_auditor::CapabilitiesRoleChecker;
use crate::capabilities_contract_role_auditor::ContractRoleChecker;
use crate::capabilities_surface_role_auditor::SurfaceRoleChecker;
use crate::capabilities_taxonomy_role_auditor::TaxonomyRoleChecker;
use crate::capabilities_utility_role_auditor::UtilityRoleChecker;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::role_rules::IRoleRunnerAggregate;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct RoleContainer {
    deps: RoleCheckerDeps,
    config: ArchitectureConfig,
}

// ─── Block 2: Constructors, Helpers, Private Methods ──────

impl RoleContainer {
    pub fn new_with_config(config: ArchitectureConfig) -> Self {
        let deps = RoleCheckerDeps {
            taxonomy: Arc::new(TaxonomyRoleChecker::new()),
            contract: Arc::new(ContractRoleChecker::new()),
            capabilities: Arc::new(CapabilitiesRoleChecker::new()),
            surface: Arc::new(SurfaceRoleChecker::new()),
            agent: Arc::new(AgentRoleChecker::new()),
            utility: Arc::new(UtilityRoleChecker::new()),
        };
        Self { deps, config }
    }

    pub fn orchestrator(&self) -> Arc<dyn IRoleRunnerAggregate> {
        let deps = RoleCheckerDeps {
            taxonomy: Arc::clone(&self.deps.taxonomy),
            contract: Arc::clone(&self.deps.contract),
            capabilities: Arc::clone(&self.deps.capabilities),
            surface: Arc::clone(&self.deps.surface),
            agent: Arc::clone(&self.deps.agent),
            utility: Arc::clone(&self.deps.utility),
        };
        Arc::new(RoleOrchestrator::new(deps, &self.config))
    }
}
