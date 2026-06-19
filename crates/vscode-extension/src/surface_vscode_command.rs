// PURPOSE: surface_vscode_command — exposes a helper function to export graph JSON

use crate::contract_vscode_bridge_port::IVsCodeBridgePort;
use crate::infrastructure_vscode_graph_generator::VsCodeGraphGenerator;
use shared::source_parsing::taxonomy_path_vo::DirectoryPath;

pub fn handle_vscode_graph(root_path: &str) -> Result<String, String> {
    let dir = DirectoryPath::new(root_path).map_err(|e| format!("Invalid path: {}", e))?;

    let generator = VsCodeGraphGenerator::new();
    let graph = generator.generate_graph(&dir)?;

    serde_json::to_string_pretty(&graph).map_err(|e| format!("JSON serialization failed: {}", e))
}
