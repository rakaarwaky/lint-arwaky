// PURPOSE: Utility layer — dependency graph using petgraph + dashmap
// Build import graph, query dependents/dependencies, detect cycles.

use shared::filesystem::{FileEntry, ImportEntry, Language};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

/// Dependency graph — directed graph of file-to-file import relationships.
pub struct DependencyGraph {
    graph: petgraph::graph::DiGraph<FileNode, ImportEdge>,
    /// Map from file path to node index.
    node_map: HashMap<PathBuf, petgraph::graph::NodeIndex>,
}

#[derive(Debug, Clone)]
pub struct FileNode {
    pub path: PathBuf,
    pub language: Language,
    pub is_external: bool,
}

#[derive(Debug, Clone)]
pub struct ImportEdge {
    pub import_type: ImportType,
    pub raw_path: String,
    pub resolved: bool,
}

use shared::filesystem::ImportType;

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            graph: petgraph::graph::DiGraph::new(),
            node_map: HashMap::new(),
        }
    }

    /// Build graph from imports and file list.
    pub fn build(&mut self, imports: &[ImportEntry], files: &[FileEntry]) {
        let mut graph = petgraph::graph::DiGraph::new();
        let mut node_map: HashMap<PathBuf, petgraph::graph::NodeIndex> = HashMap::new();

        // Add all files as nodes
        for file in files {
            let idx = graph.add_node(FileNode {
                path: file.path.clone(),
                language: file.language,
                is_external: false,
            });
            node_map.insert(file.path.clone(), idx);
        }

        // Add edges from imports
        for import in imports {
            let source_idx = match node_map.get(&import.source_file) {
                Some(idx) => *idx,
                None => continue,
            };

            let target_idx = if let Some(resolved) = &import.resolved_path {
                if let Some(idx) = node_map.get(resolved) {
                    *idx
                } else {
                    // External or unresolved — add as external node
                    let idx = graph.add_node(FileNode {
                        path: resolved.clone(),
                        language: import.language,
                        is_external: true,
                    });
                    node_map.insert(resolved.clone(), idx);
                    idx
                }
            } else {
                // Unresolved — create external node from raw_path
                let path = PathBuf::from(&import.raw_path);
                if let Some(idx) = node_map.get(&path) {
                    *idx
                } else {
                    let idx = graph.add_node(FileNode {
                        path: path.clone(),
                        language: import.language,
                        is_external: true,
                    });
                    node_map.insert(path, idx);
                    idx
                }
            };

            // Avoid duplicate edges
            if !graph.contains_edge(source_idx, target_idx) {
                graph.add_edge(
                    source_idx,
                    target_idx,
                    ImportEdge {
                        import_type: import.import_type,
                        raw_path: import.raw_path.clone(),
                        resolved: import.is_resolved,
                    },
                );
            }
        }

        self.graph = graph;
        self.node_map = node_map;
    }

    /// Get files that import the given file (who depends on me).
    pub fn dependents(&self, path: &PathBuf) -> Vec<PathBuf> {
        let idx = match self.node_map.get(path) {
            Some(idx) => *idx,
            None => return Vec::new(),
        };
        self.graph.neighbors_directed(idx, petgraph::Direction::Incoming)
            .map(|n| self.graph[n].path.clone())
            .collect()
    }

    /// Get files imported by the given file (what do I depend on).
    pub fn dependencies(&self, path: &PathBuf) -> Vec<PathBuf> {
        let idx = match self.node_map.get(path) {
            Some(idx) => *idx,
            None => return Vec::new(),
        };
        self.graph.neighbors_directed(idx, petgraph::Direction::Outgoing)
            .map(|n| self.graph[n].path.clone())
            .collect()
    }

    /// Find circular dependencies using Tarjan's algorithm.
    pub fn cycles(&self) -> Vec<Vec<PathBuf>> {
        let sccs = petgraph::algo::kosaraju_scc(&self.graph);
        sccs.into_iter()
            .filter(|scc| scc.len() > 1)
            .map(|scc| scc.into_iter().map(|n| self.graph[n].path.clone()).collect())
            .collect()
    }

    /// Check if there's a path from `from` to `to`.
    pub fn reachable(&self, from: &PathBuf, to: &PathBuf) -> bool {
        let from_idx = match self.node_map.get(from) {
            Some(idx) => *idx,
            None => return false,
        };
        let to_idx = match self.node_map.get(to) {
            Some(idx) => *idx,
            None => return false,
        };
        petgraph::algo::has_path_connecting(&self.graph, from_idx, to_idx, None)
    }

    /// Find files with no dependents (orphan candidates).
    pub fn orphan_files(&self) -> Vec<PathBuf> {
        self.graph
            .node_indices()
            .filter(|idx| {
                self.graph
                    .edges_directed(*idx, petgraph::Direction::Incoming)
                    .count()
                    == 0
            })
            .map(|idx| self.graph[idx].path.clone())
            .collect()
    }

    /// All files in the graph.
    pub fn all_files(&self) -> HashSet<PathBuf> {
        self.node_map.keys().cloned().collect()
    }

    /// Total nodes and edges count.
    pub fn stats(&self) -> (usize, usize) {
        (self.graph.node_count(), self.graph.edge_count())
    }
}

impl Default for DependencyGraph {
    fn default() -> Self {
        Self::new()
    }
}
