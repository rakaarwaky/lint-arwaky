// PURPOSE: taxonomy_vscode_graph_vo — contains VS Code graph value objects (nodes, edges, graph)

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum VsCodeNodeKind {
    File,
    Class,
    Function,
    Interface,
    Trait,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VsCodeNode {
    pub id: String,
    pub label: String,
    pub kind: VsCodeNodeKind,
    pub file: String,
    pub line: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum VsCodeEdgeRelation {
    IncomingCall,
    OutgoingCall,
    Lateral,
    Inherit,
    Implement,
    Uses,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VsCodeEdge {
    pub id: String,
    pub source: String,
    pub target: String,
    pub relation: VsCodeEdgeRelation,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VsCodeGraph {
    pub nodes: Vec<VsCodeNode>,
    pub edges: Vec<VsCodeEdge>,
}

impl VsCodeGraph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
}
