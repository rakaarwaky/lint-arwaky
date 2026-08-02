// FR-004: Graph Data Construction
// Produces: DiGraph + ReverseLinkIndex + DefinitionMap + ImplMap
// Consumer: orphan-detector, FR-003 (also uses FR-002 output)
//
// Capabilities: struct DependencyGraph — implements IGraphProtocol
// 3-block structure per AES skill

use shared::filesystem::contract_graph_protocol::IGraphProtocol;
use shared::filesystem::taxonomy_filesystem_vo::{
    DefinitionEntry, FileEntry, FileNodeVO, ImplEntry, ImportEdgeVO, ImportEntry,
};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

// ─── Block 1: Struct Definition ───────────────────────────

pub struct DependencyGraph {
    graph: petgraph::graph::DiGraph<FileNodeVO, ImportEdgeVO>,
    node_map: HashMap<PathBuf, petgraph::graph::NodeIndex>,
    reverse_links: HashMap<PathBuf, Vec<PathBuf>>,
    definitions: HashMap<String, Vec<PathBuf>>,
    implementations: HashMap<String, Vec<PathBuf>>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            graph: petgraph::graph::DiGraph::new(),
            node_map: HashMap::new(),
            reverse_links: HashMap::new(),
            definitions: HashMap::new(),
            implementations: HashMap::new(),
        }
    }
}

// ─── Block 2: Public Contract (domain protocol ONLY) ──────

impl IGraphProtocol for DependencyGraph {
    fn symbol_definitions(&self) -> &HashMap<String, Vec<PathBuf>> {
        &self.definitions
    }

    fn implementations(&self) -> &HashMap<String, Vec<PathBuf>> {
        &self.implementations
    }

    fn dependents(&self, path: &Path) -> Vec<PathBuf> {
        self.reverse_links.get(path).cloned().unwrap_or_default()
    }

    fn dependencies(&self, path: &Path) -> Vec<PathBuf> {
        let idx = match self.node_map.get(path) {
            Some(idx) => *idx,
            None => return Vec::new(),
        };
        self.graph
            .neighbors_directed(idx, petgraph::Direction::Outgoing)
            .map(|n| self.graph[n].path.clone())
            .collect()
    }

    fn reachable(&self, from: &Path, to: &Path) -> bool {
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

    fn reverse_links(&self) -> &HashMap<PathBuf, Vec<PathBuf>> {
        &self.reverse_links
    }
}

// ─── Block 3: Constructors, Std Traits & Helpers ─────────

impl Default for DependencyGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl DependencyGraph {
    /// Build graph from imports, file list, definitions, and implementations.
    pub fn build_graph(
        &mut self,
        imports: &[ImportEntry],
        files: &[FileEntry],
        definitions: &[DefinitionEntry],
        implementations: &[ImplEntry],
    ) {
        let mut graph = petgraph::graph::DiGraph::new();
        let mut node_map: HashMap<PathBuf, petgraph::graph::NodeIndex> = HashMap::new();

        // Add all files as nodes
        for file in files {
            let idx = graph.add_node(FileNodeVO {
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
                    let idx = graph.add_node(FileNodeVO {
                        path: resolved.clone(),
                        language: import.language,
                        is_external: true,
                    });
                    node_map.insert(resolved.clone(), idx);
                    idx
                }
            } else {
                let path = PathBuf::from(&import.raw_path);
                if let Some(idx) = node_map.get(&path) {
                    *idx
                } else {
                    let idx = graph.add_node(FileNodeVO {
                        path: path.clone(),
                        language: import.language,
                        is_external: true,
                    });
                    node_map.insert(path, idx);
                    idx
                }
            };

            if !graph.contains_edge(source_idx, target_idx) {
                graph.add_edge(
                    source_idx,
                    target_idx,
                    ImportEdgeVO {
                        import_type: import.import_type,
                        raw_path: import.raw_path.clone(),
                        resolved: import.is_resolved,
                        is_reexport: import.is_reexport,
                        is_wildcard: import.is_wildcard,
                    },
                );
            }
        }

        self.graph = graph;
        self.node_map = node_map;

        // Build ReverseLinkIndex
        self.reverse_links.clear();
        for edge in self.graph.edge_indices() {
            if let Some((source, target)) = self.graph.edge_endpoints(edge) {
                let target_path = self.graph[target].path.clone();
                let source_path = self.graph[source].path.clone();
                self.reverse_links
                    .entry(target_path)
                    .or_default()
                    .push(source_path);
            }
        }

        // Build DefinitionMap
        self.definitions.clear();
        for def in definitions {
            self.definitions
                .entry(def.name.clone())
                .or_default()
                .push(def.file_path.clone());
        }

        // Build ImplMap
        self.implementations.clear();
        for imp in implementations {
            self.implementations
                .entry(imp.trait_name.clone())
                .or_default()
                .push(imp.file_path.clone());
        }
    }

    pub fn cycles(&self) -> Vec<Vec<PathBuf>> {
        let sccs = petgraph::algo::kosaraju_scc(&self.graph);
        sccs.into_iter()
            .filter(|scc| scc.len() > 1)
            .map(|scc| {
                scc.into_iter()
                    .map(|n| self.graph[n].path.clone())
                    .collect()
            })
            .collect()
    }

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

    pub fn all_files(&self) -> HashSet<PathBuf> {
        self.node_map.keys().cloned().collect()
    }

    pub fn stats(&self) -> (usize, usize) {
        (self.graph.node_count(), self.graph.edge_count())
    }
}
