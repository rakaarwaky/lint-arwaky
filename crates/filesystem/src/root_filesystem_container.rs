// Root layer — wiring for filesystem feature (composition root)
// Creates concrete capabilities, injects as Arc<dyn ProtocolTrait> into agent
// Consumer calls container.orchestrator() to get Arc<dyn IFilesystemAggregate>

use std::sync::Arc;

use crate::agent_filesystem_orchestrator::{FilesystemOrchestrator, FilesystemOrchestratorDeps};
use crate::capabilities_ast_parser::ASTParser;
use crate::capabilities_dependency_graph::DependencyGraph;
use crate::capabilities_filesystem_io::CapabilitiesFileSystemIO;
use crate::capabilities_tool_resolution::CapabilitiesToolResolution;
use crate::capabilities_workspace_root_finder::CapabilitiesWorkspace;
use shared_filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared_filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use shared_filesystem::contract_filesystem_protocol::IGraphProtocol;
use shared_filesystem::contract_filesystem_protocol::IParserProtocol;
use shared_filesystem::contract_filesystem_protocol::IToolResolutionProtocol;
use shared_filesystem::contract_filesystem_protocol::IWorkspaceProtocol;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct FilesystemContainer {
    io: Arc<dyn IFileSystemIOProtocol>,
    workspace: Arc<dyn IWorkspaceProtocol>,
    tool_resolution: Arc<dyn IToolResolutionProtocol>,
    parser: Arc<dyn IParserProtocol>,
    graph: Arc<dyn IGraphProtocol>,
}

// ─── Block 2: Wiring & Factory ────────────────────────────

impl FilesystemContainer {
    /// Create container with default capabilities.
    pub fn new() -> Self {
        let io: Arc<dyn IFileSystemIOProtocol> =
            Arc::new(CapabilitiesFileSystemIO::with_default_timing());
        let workspace: Arc<dyn IWorkspaceProtocol> = Arc::new(CapabilitiesWorkspace::new());
        let tool_resolution: Arc<dyn IToolResolutionProtocol> =
            Arc::new(CapabilitiesToolResolution::new());
        let parser: Arc<dyn IParserProtocol> = Arc::new(ASTParser::new());
        let graph: Arc<dyn IGraphProtocol> = Arc::new(DependencyGraph::new());

        Self {
            io,
            workspace,
            tool_resolution,
            parser,
            graph,
        }
    }

    /// Create the orchestrator — the single entry point for consumers.
    pub fn orchestrator(&self) -> Arc<dyn IFilesystemAggregate> {
        Arc::new(FilesystemOrchestrator::new(FilesystemOrchestratorDeps {
            io: self.io.clone(),
            workspace: self.workspace.clone(),
            tool_resolution: self.tool_resolution.clone(),
            parser: self.parser.clone(),
            graph: self.graph.clone(),
        }))
    }

    /// Raw filesystem I/O seam — for consumers that need file reads, writes,
    /// path checks, or subprocess runs rather than feature-level operations.
    pub fn io(&self) -> Arc<dyn IFileSystemIOProtocol> {
        self.io.clone()
    }

    /// Workspace structure detection seam.
    pub fn workspace(&self) -> Arc<dyn IWorkspaceProtocol> {
        self.workspace.clone()
    }

    /// External tool lookup seam.
    pub fn tool_resolution(&self) -> Arc<dyn IToolResolutionProtocol> {
        self.tool_resolution.clone()
    }

    /// Source parsing seam.
    pub fn parser(&self) -> Arc<dyn IParserProtocol> {
        self.parser.clone()
    }

    /// Dependency graph seam.
    pub fn graph(&self) -> Arc<dyn IGraphProtocol> {
        self.graph.clone()
    }
}

impl Default for FilesystemContainer {
    fn default() -> Self {
        Self::new()
    }
}
