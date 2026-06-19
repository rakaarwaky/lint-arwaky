// PURPOSE: contract_vscode_bridge_port — defines the interface port for building VS Code graphs

use crate::source_parsing::taxonomy_path_vo::DirectoryPath;
use crate::vscode_extension::taxonomy_vscode_graph_vo::VsCodeGraph;

pub trait IVsCodeBridgePort: Send + Sync {
    fn generate_graph(&self, root_dir: &DirectoryPath) -> Result<VsCodeGraph, String>;
}
