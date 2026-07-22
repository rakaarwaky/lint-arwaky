// PURPOSE: Benchmark — measure project-setup throughput.
// Layer: Benchmark (performance validation).

use project_setup_lint_arwaky::agent_setup_orchestrator::SetupManagementOrchestrator;
use project_setup_lint_arwaky::capabilities_setup_installer_adapter::SetupInstallerAdapter;
use project_setup_lint_arwaky::capabilities_setup_processor::SetupManagementProcessor;
use project_setup_lint_arwaky::root_project_setup_container::SetupContainer;
use shared::cli_commands::taxonomy_protocol_vo::{TransportProtocol, TransportUrlVO};
use shared::project_setup::contract_setup_aggregate::SetupManagementAggregate;
use std::sync::Arc;
use std::time::Instant;

fn build_orchestrator() -> SetupManagementOrchestrator {
    let installer = Arc::new(SetupInstallerAdapter::new());
    let protocol = Arc::new(SetupManagementProcessor::new(installer));
    SetupManagementOrchestrator::new(protocol)
}

// ─── Benchmark: Container instantiation throughput ──

#[test]
fn bench_container_instantiation() {
    let start = Instant::now();
    for _ in 0..1000 {
        let _ = SetupContainer::new();
    }
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_millis() < 1000,
        "1000 container instantiations took {}ms",
        elapsed.as_millis()
    );
}

// ─── Benchmark: MCP config generation throughput ──

#[test]
fn bench_mcp_config_generation() {
    let orchestrator = build_orchestrator();
    let transport = TransportProtocol::HTTP;

    let start = Instant::now();
    for _ in 0..1000 {
        let _claude = orchestrator.mcp_config_claude(&transport);
        let _hermes = orchestrator.mcp_config_hermes(&transport);
        let _vscode = orchestrator.mcp_config_vscode(&transport);
    }
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_millis() < 5000,
        "MCP config generation took {}ms",
        elapsed.as_millis()
    );
}

// ─── Benchmark: Language detection throughput ──

#[test]
fn bench_language_detection() {
    let orchestrator = build_orchestrator();

    let start = Instant::now();
    for _ in 0..1000 {
        let _ = orchestrator.detect_language();
        let _ = orchestrator.detect_languages();
    }
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_millis() < 5000,
        "Language detection took {}ms",
        elapsed.as_millis()
    );
}

// ─── Benchmark: HTTP check throughput ──

#[test]
fn bench_http_check() {
    let orchestrator = build_orchestrator();

    let start = Instant::now();
    for _ in 0..1000 {
        let url = TransportUrlVO::new("http://localhost:3001".to_string());
        let _ = orchestrator.check_http(&url);
    }
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_millis() < 5000,
        "HTTP checks took {}ms",
        elapsed.as_millis()
    );
}

// ─── Benchmark: Full pipeline throughput ──

#[test]
fn bench_full_pipeline() {
    let orchestrator = build_orchestrator();
    let transport = TransportProtocol::HTTP;
    let home = shared::common::taxonomy_path_vo::DirectoryPath::new("/tmp".to_string()).unwrap();

    let start = Instant::now();
    for _ in 0..100 {
        let _claude = orchestrator.mcp_config_claude(&transport);
        let _env = orchestrator.generate_env(&transport, &home);
        let _language = orchestrator.detect_language();
    }
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_millis() < 10000,
        "Full pipeline took {}ms",
        elapsed.as_millis()
    );
}
