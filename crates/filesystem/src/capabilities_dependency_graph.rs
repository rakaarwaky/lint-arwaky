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
use std::sync::OnceLock;

// ─── Block 1: Struct Definition ───────────────────────────

static EMPTY_PATH_MAP: std::sync::LazyLock<HashMap<PathBuf, Vec<PathBuf>>> =
    std::sync::LazyLock::new(HashMap::new);
static EMPTY_STRING_MAP: std::sync::LazyLock<HashMap<String, Vec<PathBuf>>> =
    std::sync::LazyLock::new(HashMap::new);

pub struct DependencyGraph {
    graph: OnceLock<petgraph::graph::DiGraph<FileNodeVO, ImportEdgeVO>>,
    node_map: OnceLock<HashMap<PathBuf, petgraph::graph::NodeIndex>>,
    reverse_links: OnceLock<HashMap<PathBuf, Vec<PathBuf>>>,
    definitions: OnceLock<HashMap<String, Vec<PathBuf>>>,
    implementations: OnceLock<HashMap<String, Vec<PathBuf>>>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            graph: OnceLock::new(),
            node_map: OnceLock::new(),
            reverse_links: OnceLock::new(),
            definitions: OnceLock::new(),
            implementations: OnceLock::new(),
        }
    }
}

// ─── Block 2: Public Contract (domain protocol ONLY) ──────

impl IGraphProtocol for DependencyGraph {
    fn build_graph(
        &self,
        imports: &[ImportEntry],
        files: &[FileEntry],
        definitions: &[DefinitionEntry],
        implementations: &[ImplEntry],
    ) {
        // Delegate to the inherent method
        DependencyGraph::build_graph(self, imports, files, definitions, implementations);
    }

    fn symbol_definitions(&self) -> &HashMap<String, Vec<PathBuf>> {
        self.definitions.get().unwrap_or(&EMPTY_STRING_MAP)
    }

    fn implementations(&self) -> &HashMap<String, Vec<PathBuf>> {
        self.implementations.get().unwrap_or(&EMPTY_STRING_MAP)
    }

    fn dependents(&self, path: &Path) -> Vec<PathBuf> {
        self.reverse_links
            .get()
            .and_then(|m| m.get(path))
            .cloned()
            .unwrap_or_default()
    }

    fn dependencies(&self, path: &Path) -> Vec<PathBuf> {
        let graph = match self.graph.get() {
            Some(g) => g,
            None => return Vec::new(),
        };
        let node_map = match self.node_map.get() {
            Some(m) => m,
            None => return Vec::new(),
        };
        let idx = match node_map.get(path) {
            Some(idx) => *idx,
            None => return Vec::new(),
        };
        graph
            .neighbors_directed(idx, petgraph::Direction::Outgoing)
            .map(|n| graph[n].path.clone())
            .collect()
    }

    fn reachable(&self, from: &Path, to: &Path) -> bool {
        let graph = match self.graph.get() {
            Some(g) => g,
            None => return false,
        };
        let node_map = match self.node_map.get() {
            Some(m) => m,
            None => return false,
        };
        let from_idx = match node_map.get(from) {
            Some(idx) => *idx,
            None => return false,
        };
        let to_idx = match node_map.get(to) {
            Some(idx) => *idx,
            None => return false,
        };
        petgraph::algo::has_path_connecting(graph, from_idx, to_idx, None)
    }

    fn reverse_links(&self) -> &HashMap<PathBuf, Vec<PathBuf>> {
        self.reverse_links.get().unwrap_or(&EMPTY_PATH_MAP)
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
    /// Uses OnceLock — safe to call once; subsequent calls are no-ops.
    pub fn build_graph(
        &self,
        imports: &[ImportEntry],
        files: &[FileEntry],
        definitions: &[DefinitionEntry],
        implementations: &[ImplEntry],
    ) {
        let mut graph = petgraph::graph::DiGraph::new();
        let mut node_map: HashMap<PathBuf, petgraph::graph::NodeIndex> = HashMap::new();

        for file in files {
            let idx = graph.add_node(FileNodeVO {
                path: file.path.clone(),
                language: file.language,
                is_external: false,
            });
            node_map.insert(file.path.clone(), idx);
        }

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

        // Build ReverseLinkIndex
        let mut reverse_links: HashMap<PathBuf, Vec<PathBuf>> = HashMap::new();
        for edge in graph.edge_indices() {
            if let Some((source, target)) = graph.edge_endpoints(edge) {
                let target_path = graph[target].path.clone();
                let source_path = graph[source].path.clone();
                reverse_links
                    .entry(target_path)
                    .or_default()
                    .push(source_path);
            }
        }

        // Build DefinitionMap
        let mut def_map: HashMap<String, Vec<PathBuf>> = HashMap::new();
        for def in definitions {
            def_map
                .entry(def.name.clone())
                .or_default()
                .push(def.file_path.clone());
        }

        // Build ImplMap
        let mut impl_map: HashMap<String, Vec<PathBuf>> = HashMap::new();
        for imp in implementations {
            impl_map
                .entry(imp.trait_name.clone())
                .or_default()
                .push(imp.file_path.clone());
        }

        let _ = self.graph.set(graph);
        let _ = self.node_map.set(node_map);
        let _ = self.reverse_links.set(reverse_links);
        let _ = self.definitions.set(def_map);
        let _ = self.implementations.set(impl_map);
    }

    pub fn cycles(&self) -> Vec<Vec<PathBuf>> {
        let graph = match self.graph.get() {
            Some(g) => g,
            None => return Vec::new(),
        };
        let sccs = petgraph::algo::kosaraju_scc(graph);
        sccs.into_iter()
            .filter(|scc| scc.len() > 1)
            .map(|scc| scc.into_iter().map(|n| graph[n].path.clone()).collect())
            .collect()
    }

    pub fn orphan_files(&self) -> Vec<PathBuf> {
        let graph = match self.graph.get() {
            Some(g) => g,
            None => return Vec::new(),
        };
        graph
            .node_indices()
            .filter(|idx| {
                graph
                    .edges_directed(*idx, petgraph::Direction::Incoming)
                    .count()
                    == 0
            })
            .map(|idx| graph[idx].path.clone())
            .collect()
    }

    pub fn all_files(&self) -> HashSet<PathBuf> {
        self.node_map
            .get()
            .map(|m| m.keys().cloned().collect())
            .unwrap_or_default()
    }

    pub fn stats(&self) -> (usize, usize) {
        match self.graph.get() {
            Some(g) => (g.node_count(), g.edge_count()),
            None => (0, 0),
        }
    }
}
