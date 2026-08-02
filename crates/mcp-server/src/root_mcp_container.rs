// PURPOSE: McpContainer — DI wiring for MCP server
// Delegates to binary entry point for all container wiring.
// MCP server only imports shared + dispatcher.
use crate::agent_mcp_server_orchestrator::{McpServerDependencies, McpServerOrchestrator};
use std::sync::Arc;

pub struct McpContainer {
    deps: McpServerDependencies,
}

impl McpContainer {
    pub fn new(deps: McpServerDependencies) -> Self {
        Self { deps }
    }

    pub fn orchestrator(&self) -> McpServerOrchestrator {
        McpServerOrchestrator::new(McpServerDependencies {
            code_analysis_linter: self.deps.code_analysis_linter.clone(),
            fix_orchestrator: self.deps.fix_orchestrator.clone(),
            orphan_orchestrator: self.deps.orphan_orchestrator.clone(),
            maintenance_orchestrator: self.deps.maintenance_orchestrator.clone(),
            git_hooks_aggregate: self.deps.git_hooks_aggregate.clone(),
            setup_orchestrator: self.deps.setup_orchestrator.clone(),
            config_orchestrator: self.deps.config_orchestrator.clone(),
            external_lint: self.deps.external_lint.clone(),
            import_orchestrator: self.deps.import_orchestrator.clone(),
            naming_orchestrator: self.deps.naming_orchestrator.clone(),
            role_orchestrator: self.deps.role_orchestrator.clone(),
            filesystem: self.deps.filesystem.clone(),
        })
    }
}
