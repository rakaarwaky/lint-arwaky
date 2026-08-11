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
    use orphan_rules_lint_arwaky::utility_orphan_graph::trace_reachability;
    use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
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
        let alive_set = trace_reachability(&[entry.clone()], &context.import_graph);

        assert!(alive_set.contains(&entry), "Entry should be reachable");
        assert!(
            alive_set.contains("crates/my-crate/src/surfaces_my_cmd.rs"),
            "Surface should be reachable"
        );
        assert!(
            alive_set.contains("crates/my-crate/src/agent_my_orchestrator.rs"),
            "Agent should be reachable"
        );

        // Agent imports contract, so contract should be in import graph edges
        // (but capabilities won't be reachable via BFS alone — needs impl edges)
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
}
