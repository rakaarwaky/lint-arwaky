// PURPOSE: DependencyEdge — representing directed edges in dependency graph

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DependencyEdge {
    pub source: String,
    pub target: String,
}

impl DependencyEdge {
    pub fn new(source: String, target: String) -> Self {
        Self { source, target }
    }
}
