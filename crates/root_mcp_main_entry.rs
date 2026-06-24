// PURPOSE: main entry point for lint-arwaky-mcp — MCP server using rmcp official SDK
use rmcp::ServiceExt;
use std::sync::Arc;

use mcp_server::agent_mcp_server_orchestrator::LintArwakyMcpServer;

pub struct McpMainEntry {}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    eprintln!("Lint Arwaky MCP Server v{}", env!("CARGO_PKG_VERSION"));
    eprintln!("Listening on stdin/stdout (JSON-RPC 2.0 via rmcp)");

    // Inline DI composition
    let source_parsing_container =
        source_parsing::root_source_parsing_container::SourceParsingContainer::new();
    let source_parser = source_parsing_container.source_parser();
    let import_container =
        import_rules::root_import_rules_container::ImportContainer::new(source_parser);
    let analyzer = import_container.analyzer();
    let checker_container =
        code_analysis::root_code_analysis_container::CodeAnalysisCheckerContainer::new(analyzer);
    code_analysis::agent_code_analysis_orchestrator::init_global_checker(Arc::new(
        checker_container,
    ));

    let arch_linter = code_analysis::root_code_analysis_container::CodeAnalysisContainer::new()
        .code_analysis_linter();

    let server = LintArwakyMcpServer::new(arch_linter);
    let service = server.serve(rmcp::transport::stdio()).await?;
    service.waiting().await?;

    Ok(())
}
