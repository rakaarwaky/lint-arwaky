// PURPOSE: Module declarations for vscode-extension (graph VO, bridge port, graph generator, container)

pub use shared::vscode_extension::taxonomy_vscode_graph_vo;
pub use shared::vscode_extension::taxonomy_vscode_graph_vo::{VsCodeEdge, VsCodeGraph, VsCodeNode};

pub use shared::vscode_extension::contract_vscode_bridge_port;
pub use shared::vscode_extension::contract_vscode_bridge_port::IVsCodeBridgePort;

pub mod infrastructure_vscode_graph_generator;
pub use infrastructure_vscode_graph_generator::VsCodeGraphGenerator;

pub mod surface_vscode_command;
pub use surface_vscode_command::handle_vscode_graph;

pub mod root_vscode_container;
pub use root_vscode_container::VsCodeExtensionContainer;
