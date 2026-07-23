// PURPOSE: Benchmark tests — performance regression for MCP server operations

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use mcp_server_lint_arwaky::agent_mcp_server_orchestrator::{
    McpServerDependencies, McpServerOrchestrator,
};
use mcp_server_lint_arwaky::root_mcp_container::McpContainer;
use mcp_server_lint_arwaky::surface_mcp_command::LintArwakyMcpServer;
use rmcp::handler::server::wrapper::Parameters;
use shared::mcp_server::taxonomy_mcp_tool_args_vo::{ExecuteCommandArgs, ListCommandsArgs};
use std::sync::Arc;

fn build_surface() -> LintArwakyMcpServer {
    let container = McpContainer::new_default();
    let deps = McpServerDependencies {
        analysis_pipeline: container.analysis_pipeline.clone(),
        external_lint: container.external_lint.clone(),
    };
    LintArwakyMcpServer::new(Arc::new(McpServerOrchestrator::new(deps)))
}

fn bench_version_command(c: &mut Criterion) {
    let surface = build_surface();
    let rt = tokio::runtime::Runtime::new().unwrap();

    c.bench_function("execute_command_version", |b| {
        b.iter(|| {
            rt.block_on(async {
                let args = Parameters(ExecuteCommandArgs {
                    action: "version".to_string(),
                    args: None,
                });
                surface.execute_command(args).await
            })
        })
    });
}

fn bench_list_commands(c: &mut Criterion) {
    let surface = build_surface();
    let rt = tokio::runtime::Runtime::new().unwrap();

    let mut group = c.benchmark_group("list_commands");

    for domain in [None, Some("check"), Some("hook")] {
        let label = domain.unwrap_or("all");
        group.bench_with_input(BenchmarkId::new("filter", label), &domain, |b, d| {
            b.iter(|| {
                rt.block_on(async {
                    let args = Parameters(ListCommandsArgs {
                        domain: d.map(String::from),
                    });
                    surface.list_commands(args).await
                })
            })
        });
    }
    group.finish();
}

fn bench_health_check(c: &mut Criterion) {
    let surface = build_surface();
    let rt = tokio::runtime::Runtime::new().unwrap();

    c.bench_function("health_check", |b| {
        b.iter(|| rt.block_on(async { surface.health_check().await }))
    });
}

fn bench_doctor_command(c: &mut Criterion) {
    let surface = build_surface();
    let rt = tokio::runtime::Runtime::new().unwrap();

    c.bench_function("execute_command_doctor", |b| {
        b.iter(|| {
            rt.block_on(async {
                let args = Parameters(ExecuteCommandArgs {
                    action: "doctor".to_string(),
                    args: None,
                });
                surface.execute_command(args).await
            })
        })
    });
}

criterion_group!(
    benches,
    bench_version_command,
    bench_list_commands,
    bench_health_check,
    bench_doctor_command,
);
criterion_main!(benches);
