// Verify that all concrete types implement their declared contract traits.
use filesystem_lint_arwaky::agent_filesystem_orchestrator::FilesystemOrchestrator;
use filesystem_lint_arwaky::capabilities_ast_parser::ASTParser;
use filesystem_lint_arwaky::capabilities_dependency_graph::DependencyGraph;
use filesystem_lint_arwaky::capabilities_filesystem_io::CapabilitiesFileSystemIO;
use filesystem_lint_arwaky::capabilities_tool_resolution::CapabilitiesToolResolution;
use filesystem_lint_arwaky::capabilities_workspace_root_finder::CapabilitiesWorkspace;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::filesystem::contract_filesystem_io_protocol::IFileSystemIOProtocol;
use shared::filesystem::contract_graph_protocol::IGraphProtocol;
use shared::filesystem::contract_parser_protocol::IParserProtocol;
use shared::filesystem::contract_tool_resolution_protocol::IToolResolutionProtocol;
use shared::filesystem::contract_workspace_protocol::IWorkspaceProtocol;

#[test]
fn ast_parser_implements_parser_protocol() {
    fn assert_trait<T: IParserProtocol>() {}
    assert_trait::<ASTParser>();
}

#[test]
fn dependency_graph_implements_graph_protocol() {
    fn assert_trait<T: IGraphProtocol>() {}
    assert_trait::<DependencyGraph>();
}

#[test]
fn capabilities_filesystem_io_implements_io_protocol() {
    fn assert_trait<T: IFileSystemIOProtocol>() {}
    assert_trait::<CapabilitiesFileSystemIO>();
}

#[test]
fn capabilities_tool_resolution_implements_tool_resolution_protocol() {
    fn assert_trait<T: IToolResolutionProtocol>() {}
    assert_trait::<CapabilitiesToolResolution>();
}

#[test]
fn capabilities_workspace_root_finder_implements_workspace_protocol() {
    fn assert_trait<T: IWorkspaceProtocol>() {}
    assert_trait::<CapabilitiesWorkspace>();
}

#[test]
fn orchestrator_implements_aggregate() {
    fn assert_trait<T: IFilesystemAggregate>() {}
    assert_trait::<FilesystemOrchestrator>();
}

#[test]
fn orchestrator_implements_all_protocol_traits() {
    fn assert_trait<
        T: IParserProtocol
            + IGraphProtocol
            + IWorkspaceProtocol
            + IToolResolutionProtocol
            + IFileSystemIOProtocol,
    >() {
    }
    assert_trait::<FilesystemOrchestrator>();
}

#[test]
fn all_capabilities_are_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<ASTParser>();
    assert_send_sync::<DependencyGraph>();
    assert_send_sync::<CapabilitiesFileSystemIO>();
    assert_send_sync::<CapabilitiesToolResolution>();
    assert_send_sync::<CapabilitiesWorkspace>();
    assert_send_sync::<FilesystemOrchestrator>();
}

#[test]
fn orchestrator_can_be_boxed_as_trait_object() {
    fn assert_object_safe<T: IFilesystemAggregate>() {}
    assert_object_safe::<FilesystemOrchestrator>();
}
