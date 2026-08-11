// Acceptance tests — DI-aware orphan detection (issues #191-192-193).
//
// These tests build a *real* dependency-injection workspace where:
//   entry → surface → agent → contract   (all via static imports)
//   capability *implements* the contract  (trait/impl, class base, or `implements`)
//   container *wires* the capability       (references its type name) but does NOT
//   statically import it, and the agent does NOT import the capability either.
//
// Without DI-aware reachability the capability is unreachable (false AES503),
// because BFS stops at the contract. With the fix the impl-bridge edge
// (contract → capability) and the container-wiring edge (container → capability)
// make the capability reachable, so it is NOT flagged.

use filesystem::agent_filesystem_orchestrator::{
    FilesystemOrchestrator, FilesystemOrchestratorDeps,
};
use filesystem::capabilities_ast_parser::ASTParser;
use filesystem::capabilities_dependency_graph::DependencyGraph;
use filesystem::capabilities_filesystem_io::CapabilitiesFileSystemIO;
use filesystem::capabilities_tool_resolution::CapabilitiesToolResolution;
use filesystem::capabilities_workspace_root_finder::CapabilitiesWorkspace;
use orphan_rules_lint_arwaky::utility_orphan_graph::trace_reachability;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use std::sync::Arc;

/// Creates a filesystem orchestrator with default dependencies.
///
/// # Examples
///
/// ```
/// let _orchestrator = build_orch();
/// ```
///
/// # Returns
///
/// A filesystem orchestrator with default dependencies.
fn build_orch() -> FilesystemOrchestrator {
    FilesystemOrchestrator::new(FilesystemOrchestratorDeps {
        io: Arc::new(CapabilitiesFileSystemIO::with_default_timing()),
        workspace: Arc::new(CapabilitiesWorkspace::new()),
        tool_resolution: Arc::new(CapabilitiesToolResolution::new()),
        parser: Arc::new(ASTParser::new()),
        graph: Arc::new(DependencyGraph::new()),
    })
}

/// Rust DI gap: agent imports only the contract; container references the
/// capability type without importing it; capability `impl`s the contract.
#[test]
fn aes503_rust_di_wiring_reachable() {
    let tmp = tempfile::TempDir::new().unwrap();
    let crate_dir = tmp.path().join("crates").join("calc");
    let src = crate_dir.join("src");
    std::fs::create_dir_all(&src).unwrap();
    std::fs::write(
        crate_dir.join("Cargo.toml"),
        "[package]\nname = \"calc\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[lib]\npath = \"src/lib.rs\"\n",
    )
    .unwrap();
    // lib.rs declares the modules so `use calc::<mod>::...` resolves to files.
    std::fs::write(
        src.join("lib.rs"),
        "pub mod root_calc_entry;\npub mod root_calc_container;\npub mod surface_calc_cli;\npub mod agent_calc_orchestrator;\npub mod contract_calc_protocol;\npub mod capability_calc_addition;\n",
    )
    .unwrap();
    std::fs::write(
        tmp.path().join("Cargo.toml"),
        "[workspace]\nresolver = \"2\"\nmembers = [\"crates/calc\"]\n",
    )
    .unwrap();
    std::fs::write(
        src.join("root_calc_entry.rs"),
        "use calc::root_calc_container::CalculatorContainer;\nuse calc::surface_calc_cli::run;\nfn main(){ let _ = CalculatorContainer::new(); run(); }\n",
    )
    .unwrap();
    std::fs::write(
        src.join("surface_calc_cli.rs"),
        "use calc::agent_calc_orchestrator::CalculatorOrchestrator;\npub fn run(){ let _: Option<CalculatorOrchestrator>; }\n",
    )
    .unwrap();
    std::fs::write(
        src.join("agent_calc_orchestrator.rs"),
        "use calc::contract_calc_protocol::CalculatorProtocol;\npub struct CalculatorOrchestrator;\nimpl CalculatorProtocol for CalculatorOrchestrator {}\n",
    )
    .unwrap();
    std::fs::write(
        src.join("contract_calc_protocol.rs"),
        "pub trait CalculatorProtocol {}\n",
    )
    .unwrap();
    std::fs::write(
        src.join("capability_calc_addition.rs"),
        "use calc::contract_calc_protocol::CalculatorProtocol;\npub struct AdditionAnalyzer;\nimpl CalculatorProtocol for AdditionAnalyzer {}\n",
    )
    .unwrap();
    // Container wires AdditionAnalyzer by type name but does NOT import it.
    std::fs::write(
        src.join("root_calc_container.rs"),
        "use calc::agent_calc_orchestrator::CalculatorOrchestrator;\npub struct CalculatorContainer;\nimpl CalculatorContainer {\n  pub fn new() -> Self { let _a = AdditionAnalyzer; let _o = CalculatorOrchestrator; Self }\n}\n",
    )
    .unwrap();

    let orch = build_orch();
    let ctx = orch.build_orphan_graph_context(tmp.path(), &[]);
    let entry = "crates/calc/src/root_calc_entry.rs".to_string();
    let capability = "crates/calc/src/capability_calc_addition.rs".to_string();
    let contract = "crates/calc/src/contract_calc_protocol.rs".to_string();

    let alive = trace_reachability(std::slice::from_ref(&entry), &ctx.import_graph);
    assert!(alive.contains(&entry), "entry must be alive");
    assert!(
        alive.contains(&contract),
        "contract must be alive (imported by agent)"
    );
    assert!(
        alive.contains(&capability),
        "capability must be alive via DI wiring/impl bridge (got {:#?})",
        alive
    );
}

/// Python DI gap: agent imports only the contract; container references the
/// capability class without importing it; capability subclasses the contract.
#[test]
fn aes503_python_di_wiring_reachable() {
    let tmp = tempfile::TempDir::new().unwrap();
    let src = tmp.path().join("modules").join("calc").join("src");
    std::fs::create_dir_all(&src).unwrap();
    std::fs::write(
        src.join("root_calc_entry.py"),
        "from root_calc_container import CalculatorContainer\nfrom surface_calc_cli import run\n\n\ndef main():\n    run(CalculatorContainer().build())\n",
    )
    .unwrap();
    std::fs::write(
        src.join("surface_calc_cli.py"),
        "from agent_calc_orchestrator import CalculatorOrchestrator\n\n\ndef run(o: CalculatorOrchestrator):\n    pass\n",
    )
    .unwrap();
    std::fs::write(
        src.join("agent_calc_orchestrator.py"),
        "from contract_calc_protocol import CalculatorProtocol\n\n\nclass CalculatorOrchestrator(CalculatorProtocol):\n    pass\n",
    )
    .unwrap();
    std::fs::write(
        src.join("contract_calc_protocol.py"),
        "from abc import ABC\n\n\nclass CalculatorProtocol(ABC):\n    pass\n",
    )
    .unwrap();
    std::fs::write(
        src.join("capability_calc_addition.py"),
        "from contract_calc_protocol import CalculatorProtocol\n\n\nclass AdditionAnalyzer(CalculatorProtocol):\n    pass\n",
    )
    .unwrap();
    // Container references AdditionAnalyzer by type name but does NOT import it.
    std::fs::write(
        src.join("root_calc_container.py"),
        "from agent_calc_orchestrator import CalculatorOrchestrator\n\n\nclass CalculatorContainer:\n    def build(self):\n        addition = AdditionAnalyzer()\n        return CalculatorOrchestrator()\n",
    )
    .unwrap();

    let orch = build_orch();
    let ctx = orch.build_orphan_graph_context(tmp.path(), &[]);
    let entry = "modules/calc/src/root_calc_entry.py".to_string();
    let capability = "modules/calc/src/capability_calc_addition.py".to_string();
    let contract = "modules/calc/src/contract_calc_protocol.py".to_string();

    let alive = trace_reachability(std::slice::from_ref(&entry), &ctx.import_graph);
    assert!(alive.contains(&entry), "entry must be alive");
    assert!(
        alive.contains(&contract),
        "contract must be alive (imported by agent)"
    );
    assert!(
        alive.contains(&capability),
        "capability must be alive via DI wiring/impl bridge (got {:#?})",
        alive
    );
}

/// TypeScript DI gap: agent imports only the interface; container references the
/// capability class without importing it; capability `implements` the interface.
#[test]
fn aes503_typescript_di_wiring_reachable() {
    let tmp = tempfile::TempDir::new().unwrap();
    let src = tmp.path().join("packages").join("calc");
    std::fs::create_dir_all(&src).unwrap();
    std::fs::write(
        src.join("root_calc_entry.ts"),
        "import { CalculatorContainer } from './root_calc_container';\nimport { run } from './surface_calc_cli';\n\nrun(new CalculatorContainer().build());\n",
    )
    .unwrap();
    std::fs::write(
        src.join("surface_calc_cli.ts"),
        "import { CalculatorOrchestrator } from './agent_calc_orchestrator';\n\nexport function run(o: CalculatorOrchestrator): void {}\n",
    )
    .unwrap();
    std::fs::write(
        src.join("agent_calc_orchestrator.ts"),
        "import { CalculatorProtocol } from './contract_calc_protocol';\n\nexport class CalculatorOrchestrator implements CalculatorProtocol {}\n",
    )
    .unwrap();
    std::fs::write(
        src.join("contract_calc_protocol.ts"),
        "export interface CalculatorProtocol {}\n",
    )
    .unwrap();
    std::fs::write(
        src.join("capability_calc_addition.ts"),
        "import { CalculatorProtocol } from './contract_calc_protocol';\n\nexport class AdditionAnalyzer implements CalculatorProtocol {}\n",
    )
    .unwrap();
    // Container references AdditionAnalyzer by type name but does NOT import it.
    std::fs::write(
        src.join("root_calc_container.ts"),
        "import { CalculatorOrchestrator } from './agent_calc_orchestrator';\n\nexport class CalculatorContainer {\n  build(): CalculatorOrchestrator {\n    const addition = AdditionAnalyzer;\n    return new CalculatorOrchestrator();\n  }\n}\n",
    )
    .unwrap();

    let orch = build_orch();
    let ctx = orch.build_orphan_graph_context(tmp.path(), &[]);
    let entry = "packages/calc/root_calc_entry.ts".to_string();
    let capability = "packages/calc/capability_calc_addition.ts".to_string();
    let contract = "packages/calc/contract_calc_protocol.ts".to_string();

    let alive = trace_reachability(std::slice::from_ref(&entry), &ctx.import_graph);
    assert!(alive.contains(&entry), "entry must be alive");
    assert!(
        alive.contains(&contract),
        "contract must be alive (imported by agent)"
    );
    assert!(
        alive.contains(&capability),
        "capability must be alive via DI wiring/impl bridge (got {:#?})",
        alive
    );
}
