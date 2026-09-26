// Acceptance tests — AES503 DI-aware orphan detection (P1+P3).
// Tests that capabilities wired via dependency injection (constructor injection)
// are NOT flagged as false-positive orphans when they implement contracts
// imported by agent layer files.

#[cfg(test)]
mod di_aware_orphan_tests {
    use filesystem::agent_filesystem_orchestrator::{
        FilesystemOrchestrator, FilesystemOrchestratorDeps,
    };
    use filesystem::capabilities_ast_parser::ASTParser;
    use filesystem::capabilities_dependency_graph::DependencyGraph;
    use filesystem::capabilities_filesystem_io::CapabilitiesFileSystemIO;
    use filesystem::capabilities_tool_resolution::CapabilitiesToolResolution;
    use filesystem::capabilities_workspace_root_finder::CapabilitiesWorkspace;
    use orphan_rules_lint_arwaky::capabilities_orphan_contract_analyzer::ContractOrphanAnalyzer;
    use orphan_rules_lint_arwaky::utility_orphan_graph::trace_reachability;
    use shared::common::taxonomy_path_vo::FilePath;
    use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
    use shared::orphan_rules::contract_orphan_protocol::IContractOrphanProtocol;
    use shared::quality_rules::taxonomy_analysis_vo::ReachabilityResult;
    use std::collections::HashMap;
    use std::sync::Arc;

    fn build_orchestrator() -> FilesystemOrchestrator {
        FilesystemOrchestrator::new(FilesystemOrchestratorDeps {
            io: Arc::new(CapabilitiesFileSystemIO::with_default_timing()),
            workspace: Arc::new(CapabilitiesWorkspace::new()),
            tool_resolution: Arc::new(CapabilitiesToolResolution::new()),
            parser: Arc::new(ASTParser::new()),
            graph: Arc::new(DependencyGraph::new()),
        })
    }

    /// P1+P3: Rust DI pattern — Agent imports contract trait, Capabilities implements it.
    /// The capabilities file should be reachable through implementation edges,
    /// NOT flagged as AES503 orphan.
    #[test]
    fn aes503_rust_di_pattern_implementation_edges() {
        let tmp = tempfile::TempDir::new().unwrap();
        let member_src = tmp.path().join("crates").join("my-crate").join("src");
        std::fs::create_dir_all(&member_src).unwrap();

        // 1. Root entry file (entry point)
        std::fs::write(
            member_src.join("root_my_entry.rs"),
            r#"use crate::surfaces_my_cmd::MyCommand;
fn main() {
    let cmd = MyCommand::new();
    cmd.run();
}
"#,
        )
        .unwrap();

        // 2. Surface file imports agent
        std::fs::write(
            member_src.join("surfaces_my_cmd.rs"),
            r#"use crate::agent_my_orchestrator::MyOrchestrator;

pub struct MyCommand { orch: MyOrchestrator }
impl MyCommand {
    pub fn new() -> Self { Self { orch: MyOrchestrator::new() } }
    pub fn run(&self) { self.orch.process(); }
}
"#,
        )
        .unwrap();

        // 3. Agent layer imports CONTRACT (trait/protocol) — NOT capabilities directly
        std::fs::write(
            member_src.join("agent_my_orchestrator.rs"),
            r#"use crate::contract_my_protocol::IMyProtocol;

pub struct MyOrchestrator { proto: Box<dyn IMyProtocol> }
impl MyOrchestrator {
    pub fn new() -> Self {
        Self { proto: Box::new(crate::capabilities_my_processor::MyProcessor::new()) }
    }
    pub fn process(&self) { self.proto.execute(); }
}
"#,
        )
        .unwrap();

        // 4. Contract layer — defines the trait
        std::fs::write(
            member_src.join("contract_my_protocol.rs"),
            r#"pub trait IMyProtocol {
    fn execute(&self);
}
"#,
        )
        .unwrap();

        // 5. Capabilities layer — implements the contract (via `impl`)
        std::fs::write(
            member_src.join("capabilities_my_processor.rs"),
            r#"use crate::contract_my_protocol::IMyProtocol;

pub struct MyProcessor;
impl IMyProtocol for MyProcessor {
    fn execute(&self) { /* real work */ }
}
"#,
        )
        .unwrap();

        // 6. Root container — wires capabilities to agent (constructor injection)
        std::fs::write(
            member_src.join("root_my_container.rs"),
            r#"use crate::agent_my_orchestrator::MyOrchestrator;
use crate::capabilities_my_processor::MyProcessor;

pub fn create_app() -> MyOrchestrator {
    MyOrchestrator::new()  // DI: orchestrator creates processor internally
}
"#,
        )
        .unwrap();

        let orch = build_orchestrator();
        let context = orch.build_orphan_graph_context(tmp.path(), &[]);

        // Verify the inheritance map has the impl edge: contract → capabilities
        let inheritance = &context.inheritance_map;
        assert!(
            inheritance.mapping.contains_key("IMyProtocol"),
            "InheritanceMap should contain IMyProtocol trait"
        );
        let impl_files = inheritance.mapping.get("IMyProtocol").unwrap();
        assert!(
            impl_files
                .iter()
                .any(|f| f.to_string().contains("capabilities_my_processor")),
            "IMyProtocol should have capabilities_my_processor as implementor"
        );

        // Verify BFS reachability traces through entry → surface → agent
        let entry = "crates/my-crate/src/root_my_entry.rs".to_string();
        let alive_set = trace_reachability(std::slice::from_ref(&entry), &context.import_graph);

        assert!(alive_set.contains(&entry), "Entry should be reachable");
        assert!(
            alive_set.contains("crates/my-crate/src/surfaces_my_cmd.rs"),
            "Surface should be reachable"
        );
        assert!(
            alive_set.contains("crates/my-crate/src/agent_my_orchestrator.rs"),
            "Agent should be reachable"
        );
        assert!(
            alive_set.contains("crates/my-crate/src/contract_my_protocol.rs"),
            "Contract should be reachable (imported by agent)"
        );
        // The capability implements the contract (impl bridge), so it must be
        // reachable even though the agent never statically imports it.
        assert!(
            alive_set.contains("crates/my-crate/src/capabilities_my_processor.rs"),
            "Capability implementing a reachable contract must be alive via impl bridge"
        );
    }

    /// P1+P3: The extend_reachability_through_impls method should add capabilities
    /// files to the alive set when they implement contracts that are reachable.
    #[test]
    fn aes503_di_reachability_extended_through_impls() {
        let tmp = tempfile::TempDir::new().unwrap();
        let member_src = tmp.path().join("crates").join("my-crate").join("src");
        std::fs::create_dir_all(&member_src).unwrap();

        // Minimal DI pattern: entry → surface → agent → contract
        // (capabilities not imported by agent, only implements contract)
        std::fs::write(
            member_src.join("root_entry.rs"),
            "use surfaces_cmd::MyCmd;\n",
        )
        .unwrap();
        std::fs::write(
            member_src.join("surfaces_cmd.rs"),
            "use agent_orch::Orchestrator;\n",
        )
        .unwrap();
        // Agent imports a contract trait (not capabilities directly)
        std::fs::write(
            member_src.join("agent_orch.rs"),
            "use contract_proto::IProto;\n",
        )
        .unwrap();
        // Contract defines the trait
        std::fs::write(
            member_src.join("contract_proto.rs"),
            "pub trait IProto { fn run(&self); }\n",
        )
        .unwrap();
        // Capabilities implements the contract (via impl block)
        std::fs::write(
            member_src.join("capabilities_impl.rs"),
            "use contract_proto::IProto;\npub struct MyImpl;\nimpl IProto for MyImpl { fn run(&self) {} }\n",
        )
        .unwrap();

        let orch = build_orchestrator();
        let context = orch.build_orphan_graph_context(tmp.path(), &[]);

        // Verify inheritance map has the impl edge
        assert!(
            context.inheritance_map.mapping.contains_key("IProto"),
            "InheritanceMap should contain IProto"
        );

        // The capability implements the reachable contract, so it must be alive
        // via the impl bridge even though nothing statically imports it.
        let entry = "crates/my-crate/src/root_entry.rs".to_string();
        let alive_set = trace_reachability(std::slice::from_ref(&entry), &context.import_graph);
        assert!(
            alive_set.contains("crates/my-crate/src/capabilities_impl.rs"),
            "Capability implementing a reachable contract must be alive via impl bridge"
        );
    }

    /// P1+P3: Contract that is only reachable through alive implementors should NOT be orphan.
    /// When a contract has an alive implementor (capability), the contract itself is wired.
    #[test]
    fn aes502_contract_wired_through_implementation() {
        let tmp = tempfile::TempDir::new().unwrap();
        let member_src = tmp.path().join("crates").join("my-crate").join("src");
        std::fs::create_dir_all(&member_src).unwrap();

        // Entry → Surface → Agent (all reachable)
        std::fs::write(
            member_src.join("root_entry.rs"),
            "use surfaces_cmd::MyCmd;\n",
        )
        .unwrap();
        std::fs::write(
            member_src.join("surfaces_cmd.rs"),
            "use agent_orch::Orchestrator;\n",
        )
        .unwrap();
        // Agent imports contract
        std::fs::write(
            member_src.join("agent_orch.rs"),
            "use contract_io::IFileSystemIO;\n",
        )
        .unwrap();
        // Contract defines trait
        std::fs::write(
            member_src.join("contract_io.rs"),
            "pub trait IFileSystemIO { fn read(&self) -> String; }\n",
        )
        .unwrap();
        // Capabilities implements contract
        std::fs::write(
            member_src.join("capabilities_io.rs"),
            "use contract_io::IFileSystemIO;\npub struct FileSystemIOImpl;\nimpl IFileSystemIO for FileSystemIOImpl { fn read(&self) -> String { String::new() } }\n",
        )
        .unwrap();

        let orch = build_orchestrator();
        let context = orch.build_orphan_graph_context(tmp.path(), &[]);

        // Verify contract has an implementor in inheritance map
        assert!(
            context
                .inheritance_map
                .mapping
                .contains_key("IFileSystemIO"),
            "Contract trait IFileSystemIO should have implementation"
        );

        // The contract must NOT be flagged as orphan: its implementor is alive.
        let contract_path = "crates/my-crate/src/contract_io.rs".to_string();
        let alive_set = trace_reachability(
            &["crates/my-crate/src/root_entry.rs".to_string()],
            &context.import_graph,
        );
        let alive = ReachabilityResult::new(
            alive_set
                .iter()
                .filter_map(|p| FilePath::new(p.clone()).ok())
                .collect(),
        );
        let mut content_map = HashMap::new();
        content_map.insert(
            contract_path.clone(),
            "pub trait IFileSystemIO { fn read(&self) -> String; }\n".to_string(),
        );
        content_map.insert(
            "crates/my-crate/src/capabilities_io.rs".to_string(),
            "use contract_io::IFileSystemIO;\npub struct FileSystemIOImpl;\nimpl IFileSystemIO for FileSystemIOImpl { fn read(&self) -> String { String::new() } }\n"
                .to_string(),
        );
        let inheritance = context.inheritance_map.clone();
        let result = ContractOrphanAnalyzer::new().is_contract_orphan(
            &FilePath::new(contract_path.clone()).unwrap(),
            &FilePath::new(".".to_string()).unwrap(),
            &inheritance,
            &[
                contract_path.clone(),
                "crates/my-crate/src/capabilities_io.rs".to_string(),
            ],
            &content_map,
            &alive,
        );
        assert!(
            !result.is_orphan,
            "Contract with an alive implementor must not be orphan: {}",
            result.reason
        );
    }

    /// P1+P3: Multiple capabilities implementing the same contract.
    /// All implementing files should be added to the alive set through impl edges.
    #[test]
    fn aes503_multiple_impls_same_contract() {
        let tmp = tempfile::TempDir::new().unwrap();
        let member_src = tmp.path().join("crates").join("my-crate").join("src");
        std::fs::create_dir_all(&member_src).unwrap();

        // Entry → Agent (reachable)
        std::fs::write(
            member_src.join("root_entry.rs"),
            "use agent_orch::Orchestrator;\n",
        )
        .unwrap();
        std::fs::write(
            member_src.join("agent_orch.rs"),
            "use contract_proto::IProto;\n",
        )
        .unwrap();
        // Contract
        std::fs::write(
            member_src.join("contract_proto.rs"),
            "pub trait IProto { fn run(&self); }\n",
        )
        .unwrap();
        // Multiple capabilities implementing the same contract
        std::fs::write(
            member_src.join("capabilities_impl_a.rs"),
            "use contract_proto::IProto;\npub struct ImplA;\nimpl IProto for ImplA { fn run(&self) {} }\n",
        )
        .unwrap();
        std::fs::write(
            member_src.join("capabilities_impl_b.rs"),
            "use contract_proto::IProto;\npub struct ImplB;\nimpl IProto for ImplB { fn run(&self) {} }\n",
        )
        .unwrap();

        let orch = build_orchestrator();
        let context = orch.build_orphan_graph_context(tmp.path(), &[]);

        // Both implementations should be in the inheritance map
        let impls = context.inheritance_map.mapping.get("IProto").unwrap();
        assert!(
            impls
                .iter()
                .any(|f| f.to_string().contains("capabilities_impl_a")),
            "ImplA should be registered"
        );
        assert!(
            impls
                .iter()
                .any(|f| f.to_string().contains("capabilities_impl_b")),
            "ImplB should be registered"
        );
    }

    /// P4: a Python class with multiple bases (`class Foo(ProtocolA, ProtocolB)`)
    /// must register an implementation bridge for EACH base, not just the first.
    #[test]
    fn aes503_python_class_with_multiple_bases_registers_all_bridges() {
        let tmp = tempfile::TempDir::new().unwrap();
        let member_src = tmp.path().join("modules").join("calc").join("src");
        std::fs::create_dir_all(&member_src).unwrap();

        std::fs::write(
            member_src.join("contract_a_protocol.py"),
            "class ProtocolA:\n    pass\n",
        )
        .unwrap();
        std::fs::write(
            member_src.join("contract_b_protocol.py"),
            "class ProtocolB:\n    pass\n",
        )
        .unwrap();
        std::fs::write(
            member_src.join("capabilities_multi_base.py"),
            "from contract_a_protocol import ProtocolA\nfrom contract_b_protocol import ProtocolB\n\n\nclass MultiImpl(ProtocolA, ProtocolB):\n    pass\n",
        )
        .unwrap();

        let orch = build_orchestrator();
        let context = orch.build_orphan_graph_context(tmp.path(), &[]);

        let impls_a = context
            .inheritance_map
            .mapping
            .get("ProtocolA")
            .expect("ProtocolA should have registered implementors");
        assert!(
            impls_a
                .iter()
                .any(|f| f.contains("capabilities_multi_base")),
            "MultiImpl should bridge from ProtocolA: {:?}",
            impls_a
        );

        let impls_b = context
            .inheritance_map
            .mapping
            .get("ProtocolB")
            .expect("ProtocolB should have registered implementors");
        assert!(
            impls_b
                .iter()
                .any(|f| f.contains("capabilities_multi_base")),
            "MultiImpl should also bridge from ProtocolB: {:?}",
            impls_b
        );
    }

    /// P4: a Python class with no bases (`class Foo:`) must not register any
    /// spurious implementation bridge (regression for the empty-base filter).
    #[test]
    fn aes503_python_class_without_bases_registers_no_bridge() {
        let tmp = tempfile::TempDir::new().unwrap();
        let member_src = tmp.path().join("modules").join("calc").join("src");
        std::fs::create_dir_all(&member_src).unwrap();

        std::fs::write(
            member_src.join("capabilities_standalone.py"),
            "class Standalone:\n    pass\n",
        )
        .unwrap();

        let orch = build_orchestrator();
        let context = orch.build_orphan_graph_context(tmp.path(), &[]);

        assert!(
            !context
                .inheritance_map
                .mapping
                .values()
                .any(|impls| impls.iter().any(|f| f.contains("capabilities_standalone"))),
            "A class without bases should not register any implementation bridge"
        );
    }

    /// P4: a TypeScript class implementing multiple interfaces
    /// (`class Foo implements IA, IB`) must register a bridge for each.
    #[test]
    fn aes503_typescript_class_with_multiple_implements_registers_all_bridges() {
        let tmp = tempfile::TempDir::new().unwrap();
        let member_src = tmp.path().join("packages").join("calc");
        std::fs::create_dir_all(&member_src).unwrap();

        std::fs::write(
            member_src.join("contract_a.ts"),
            "export interface IProtoA {}\n",
        )
        .unwrap();
        std::fs::write(
            member_src.join("contract_b.ts"),
            "export interface IProtoB {}\n",
        )
        .unwrap();
        std::fs::write(
            member_src.join("capabilities_multi_impl.ts"),
            "import { IProtoA } from './contract_a';\nimport { IProtoB } from './contract_b';\n\nexport class MultiImpl implements IProtoA, IProtoB {}\n",
        )
        .unwrap();

        let orch = build_orchestrator();
        let context = orch.build_orphan_graph_context(tmp.path(), &[]);

        let impls_a = context
            .inheritance_map
            .mapping
            .get("IProtoA")
            .expect("IProtoA should have registered implementors");
        assert!(
            impls_a
                .iter()
                .any(|f| f.contains("capabilities_multi_impl")),
            "MultiImpl should bridge from IProtoA: {:?}",
            impls_a
        );

        let impls_b = context
            .inheritance_map
            .mapping
            .get("IProtoB")
            .expect("IProtoB should have registered implementors");
        assert!(
            impls_b
                .iter()
                .any(|f| f.contains("capabilities_multi_impl")),
            "MultiImpl should also bridge from IProtoB: {:?}",
            impls_b
        );
    }
}
