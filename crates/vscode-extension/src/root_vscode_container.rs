// PURPOSE: root_vscode_container — handles dependency injection composition for vscode-extension features

use crate::contract_vscode_bridge_port::IVsCodeBridgePort;
use crate::infrastructure_vscode_graph_generator::VsCodeGraphGenerator;
use std::sync::Arc;

pub struct VsCodeExtensionContainer {
    graph_generator: Arc<dyn IVsCodeBridgePort>,
}

impl VsCodeExtensionContainer {
    pub fn new() -> Self {
        Self {
            graph_generator: Arc::new(VsCodeGraphGenerator::new()),
        }
    }

    pub fn graph_generator(&self) -> Arc<dyn IVsCodeBridgePort> {
        self.graph_generator.clone()
    }
}

impl Default for VsCodeExtensionContainer {
    fn default() -> Self {
        Self::new()
    }
}
