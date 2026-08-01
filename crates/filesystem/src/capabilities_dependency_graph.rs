// PURPOSE: Capabilities layer — dependency graph construction (FR-004)
// Build DiGraph (file -> file edges), ReverseLinkIndex, DefinitionMap, ImplMap.
// Uses petgraph for the graph, DashMap for parallel construction.

use shared::filesystem::taxonomy_filesystem_vo::*;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

/// Dependency graph — directed graph of file-to-file import relationships.
/// Includes ReverseLinkIndex, DefinitionMap, and ImplMap per FR-004.
pub struct DependencyGraph {
    graph: petgraph::graph::DiGraph<FileNodeVO, ImportEdgeVO>,
    node_map: HashMap<PathBuf, petgraph::graph::NodeIndex>,
    /// ReverseLinkIndex: file -> list of files that import it.
    reverse_links: HashMap<PathBuf, Vec<PathBuf>>,
    /// DefinitionMap: symbol name -> defining file(s).
    definitions: HashMap<String, Vec<PathBuf>>,
    /// ImplMap: trait/interface name -> implementor file(s).
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

    /// Build graph from imports, file list, definitions, and implementations.
    /// FR-004 business rules:
    /// - Nodes: each source file (keyed by workspace-root-relative path).
    /// - Edges: import relationship (source file -> imported file).
    /// - Duplicate imports: single edge, deduplicated.
    /// - Barrel file re-exports: reverse link points to original source.
    pub fn build(
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
                    // External or unresolved — add as external node
                    let idx = graph.add_node(FileNodeVO {
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
                    let idx = graph.add_node(FileNodeVO {
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

        // Build ReverseLinkIndex: invert all edges
        self.reverse_links.clear();
        for edge in self.graph.edge_indices() {
            let (source, target) = self.graph.edge_endpoints(edge).unwrap();
            let target_path = self.graph[target].path.clone();
            let source_path = self.graph[source].path.clone();
            self.reverse_links
                .entry(target_path)
                .or_default()
                .push(source_path);
        }

        // Build DefinitionMap from parse metadata
        self.definitions.clear();
        for def in definitions {
            self.definitions
                .entry(def.name.clone())
                .or_default()
                .push(def.file_path.clone());
        }

        // Build ImplMap from parse metadata
        self.implementations.clear();
        for imp in implementations {
            self.implementations
                .entry(imp.trait_name.clone())
                .or_default()
                .push(imp.file_path.clone());
        }
    }

    /// Get files that import the given file (who depends on me).
    /// Uses ReverseLinkIndex for O(1) lookup.
    pub fn dependents(&self, path: &PathBuf) -> Vec<PathBuf> {
        self.reverse_links.get(path).cloned().unwrap_or_default()
    }

    /// Get files imported by the given file (what do I depend on).
    pub fn dependencies(&self, path: &PathBuf) -> Vec<PathBuf> {
        let idx = match self.node_map.get(path) {
            Some(idx) => *idx,
            None => return Vec::new(),
        };
        self.graph
            .neighbors_directed(idx, petgraph::Direction::Outgoing)
            .map(|n| self.graph[n].path.clone())
            .collect()
    }

    /// Find circular dependencies using Kosaraju's SCC algorithm.
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

    /// Check if there's a path from `from` to `to` (BFS).
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

    /// Get the reverse link index.
    pub fn reverse_links(&self) -> &HashMap<PathBuf, Vec<PathBuf>> {
        &self.reverse_links
    }

    /// Get the definition map.
    pub fn definitions(&self) -> &HashMap<String, Vec<PathBuf>> {
        &self.definitions
    }

    /// Get the implementation map.
    pub fn implementations(&self) -> &HashMap<String, Vec<PathBuf>> {
        &self.implementations
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
