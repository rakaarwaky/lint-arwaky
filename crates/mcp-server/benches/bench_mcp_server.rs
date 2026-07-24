// PURPOSE: Benchmark tests — performance regression for MCP server operations
// Best practices: significance_level(0.05), sample_size(30+), reuse runtime across iterations

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
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
    let mut group = c.benchmark_group("execute_command");
    group.sample_size(30);

    group.bench_function("version", |b| {
        b.iter(|| {
            let args = Parameters(ExecuteCommandArgs {
                action: "version".to_string(),
                args: None,
            });
            black_box(rt.block_on(async { surface.execute_command(args).await }))
        })
    });
}

fn bench_list_commands(c: &mut Criterion) {
    let surface = build_surface();
    let rt = tokio::runtime::Runtime::new().unwrap();

    let mut group = c.benchmark_group("list_commands");
    group.significance_level(0.05).confidence_level(0.95);

    for domain in [None, Some("check"), Some("hook")] {
        let label = domain.unwrap_or("all");
        group.bench_with_input(BenchmarkId::new("filter", label), &domain, |b, d| {
            b.iter(|| {
                let args = Parameters(ListCommandsArgs {
                    domain: d.map(String::from),
                });
                black_box(rt.block_on(async { surface.list_commands(args).await }))
            })
        });
    }
    group.finish();
}

fn bench_health_check(c: &mut Criterion) {
    let surface = build_surface();
    let rt = tokio::runtime::Runtime::new().unwrap();
    let mut group = c.benchmark_group("health_check");
    group.sample_size(30);

    group.bench_function("check", |b| {
        b.iter(|| black_box(rt.block_on(async { surface.health_check().await })))
    });
}

fn bench_doctor_command(c: &mut Criterion) {
    let surface = build_surface();
    let rt = tokio::runtime::Runtime::new().unwrap();
    let mut group = c.benchmark_group("execute_command");
    group.sample_size(30);

    group.bench_function("doctor", |b| {
        b.iter(|| {
            let args = Parameters(ExecuteCommandArgs {
                action: "doctor".to_string(),
                args: None,
            });
            black_box(rt.block_on(async { surface.execute_command(args).await }))
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
