// PURPOSE: main entry point for lint-arwaky-mcp — MCP server using rmcp official SDK
use rmcp::ServiceExt;
use std::sync::Arc;

use mcp_server::agent_mcp_server_orchestrator::McpServerOrchestrator;
use mcp_server::surface_mcp_command::LintArwakyMcpServer;

pub struct McpMainEntry {}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    eprintln!("Lint Arwaky MCP Server v{}", env!("CARGO_PKG_VERSION"));
    eprintln!("Listening on stdin/stdout (JSON-RPC 2.0 via rmcp)");

    // Import rules container
    let import_container =
        import_rules::root_import_rules_container::ImportContainer::new_default();
    let analyzer = import_container.analyzer();

    // Code analysis container
    let checker_container =
        code_analysis::root_code_analysis_container::CodeAnalysisCheckerContainer::new(analyzer);
    code_analysis::agent_code_analysis_orchestrator::init_global_checker(Arc::new(
        checker_container,
    ));
    let code_analysis_linter =
        code_analysis::root_code_analysis_container::CodeAnalysisContainer::new()
            .code_analysis_linter();

    // Import runner aggregate
    let import_orchestrator = import_container.orchestrator();

    // Naming rules container
    let naming_container =
        naming_rules::root_naming_rules_container::NamingContainer::new_default();
    let naming_orchestrator = naming_container.orchestrator();

    // Orphan detector container (provides both aggregate + layer_detector)
    let orphan_container = orphan_detector::root_orphan_detector_container::OrphanContainer::new();
    let orphan_orchestrator = orphan_container.analyzer();
    let layer_detector = orphan_container.layer_detector();

    // Scanner provider (for orphan detection file collection)
    let scanner_provider: Arc<
        dyn shared::source_parsing::contract_scanner_provider_port::IScannerProviderPort,
    > = Arc::new(
        shared::source_parsing::infrastructure_file_collector_provider::FileCollectorProvider::new(
        ),
    );

    // External lint container
    let ext_container =
        external_lint::root_external_lint_container::ExternalLintContainer::new_default();
    let external_lint = ext_container.aggregate();

    // Role rules container
    let role_container = role_rules::root_role_rules_container::RoleContainer::new();
    let role_orchestrator = role_container.orchestrator();

    // Wire MCP server agent with all 6 aggregates
    let agent = McpServerOrchestrator::new(
        code_analysis_linter,
        import_orchestrator,
        naming_orchestrator,
        orphan_orchestrator,
        layer_detector,
        scanner_provider,
        external_lint,
        role_orchestrator,
    );

    let server = LintArwakyMcpServer::new(Arc::new(agent));
    let service = server.serve(rmcp::transport::stdio()).await?;
    service.waiting().await?;

    Ok(())
}
