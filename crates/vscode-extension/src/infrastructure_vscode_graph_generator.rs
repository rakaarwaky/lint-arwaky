// PURPOSE: capabilities_vscode_graph_generator — implements IVsCodeBridgePort to scan files and generate a node/edge graph

use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::sync::Arc;

use crate::contract_vscode_bridge_port::IVsCodeBridgePort;
use crate::taxonomy_vscode_graph_vo::{
    VsCodeEdge, VsCodeEdgeRelation, VsCodeGraph, VsCodeNode, VsCodeNodeKind,
};
use code_analysis::agent_code_analysis_orchestrator::collect_source_files;
use shared::config_system::taxonomy_config_vo::default_aes_config;
use shared::source_parsing::contract_parser_port::ISourceParserPort;
use shared::source_parsing::taxonomy_path_vo::DirectoryPath;
use source_parsing::root_source_parsing_container::SourceParsingContainer;

pub struct VsCodeGraphGenerator {
    source_parser: Arc<dyn ISourceParserPort>,
}

impl VsCodeGraphGenerator {
    pub fn new() -> Self {
        let parser_container = SourceParsingContainer::new();
        Self {
            source_parser: parser_container.source_parser(),
        }
    }

    /// Normalizes a path relative to the root directory for short, readable node IDs
    fn get_relative_path(&self, path: &str, root: &str) -> String {
        let clean_path = path.replace('\\', "/");
        let clean_root = root.replace('\\', "/");
        let root_dir = if clean_root.ends_with('/') {
            clean_root
        } else {
            format!("{}/", clean_root)
        };

        if clean_path.starts_with(&root_dir) {
            clean_path[root_dir.len()..].to_string()
        } else {
            // Settle for basename if not within root
            Path::new(path)
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| clean_path.clone())
        }
    }

    /// Helper to detect if a relationship is lateral based on AES file naming conventions
    fn is_lateral_layer(&self, src: &str, target: &str) -> bool {
        let prefixes = [
            "taxonomy_",
            "contract_",
            "capabilities_",
            "infrastructure_",
            "agent_",
            "surface_",
            "root_",
        ];

        let src_basename = Path::new(src)
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        let target_basename = Path::new(target)
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();

        for prefix in &prefixes {
            if src_basename.starts_with(prefix) && target_basename.starts_with(prefix) {
                return true;
            }
        }
        false
    }
}

impl Default for VsCodeGraphGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl IVsCodeBridgePort for VsCodeGraphGenerator {
    fn generate_graph(&self, root_dir: &DirectoryPath) -> Result<VsCodeGraph, String> {
        let config = default_aes_config();
        let ignored: Vec<String> = config
            .ignored_paths
            .values
            .iter()
            .map(|fp| fp.value.replace('/', std::path::MAIN_SEPARATOR_STR))
            .collect();
        let files = collect_source_files(Path::new(&root_dir.value), root_dir, &ignored);
        let mut graph = VsCodeGraph::new();

        // 1. First Pass: Collect all files and define their symbols (Classes, Functions)
        // Maps symbol name -> (defining file rel path, NodeKind, line_number)
        let mut symbol_definitions: HashMap<String, (String, VsCodeNodeKind, Option<i64>)> =
            HashMap::new();
        // Maps normalized file path -> list of symbol node IDs
        let mut file_symbols: HashMap<String, Vec<String>> = HashMap::new();

        for fp in &files {
            let rel_path = self.get_relative_path(&fp.value, &root_dir.value);

            // Add File Node
            graph.nodes.push(VsCodeNode {
                id: rel_path.clone(),
                label: fp.basename(),
                kind: VsCodeNodeKind::File,
                file: rel_path.clone(),
                line: None,
            });

            // Extract Classes/Structures/Traits
            if let Ok(class_meta) = self.source_parser.get_class_definitions(fp) {
                if let Some(classes_val) = class_meta.value.get("classes") {
                    if let Some(classes_arr) = classes_val.as_array() {
                        for c in classes_arr {
                            if let Some(name) = c.get("name").and_then(|n| n.as_str()) {
                                let line = c.get("line").and_then(|l| l.as_i64());

                                // Determine node kind (Class, Interface, or Trait)
                                let kind = if name.contains("Trait") || name.ends_with("Protocol") {
                                    VsCodeNodeKind::Trait
                                } else if name.contains("Interface") || name.ends_with("Port") {
                                    VsCodeNodeKind::Interface
                                } else {
                                    VsCodeNodeKind::Class
                                };

                                let symbol_id = format!("{}#{}", rel_path, name);
                                symbol_definitions.insert(
                                    name.to_string(),
                                    (rel_path.clone(), kind.clone(), line),
                                );

                                graph.nodes.push(VsCodeNode {
                                    id: symbol_id.clone(),
                                    label: name.to_string(),
                                    kind,
                                    file: rel_path.clone(),
                                    line,
                                });

                                file_symbols
                                    .entry(rel_path.clone())
                                    .or_default()
                                    .push(symbol_id);
                            }
                        }
                    }
                }
            }

            // Extract Functions
            let fn_meta = self.source_parser.get_function_definitions(fp);
            if let Some(funcs_val) = fn_meta.value.get("functions") {
                if let Some(funcs_arr) = funcs_val.as_array() {
                    for f in funcs_arr {
                        if let Some(name) = f.get("name").and_then(|n| n.as_str()) {
                            let line = f.get("line").and_then(|l| l.as_i64());
                            let symbol_id = format!("{}#{}", rel_path, name);

                            symbol_definitions.insert(
                                name.to_string(),
                                (rel_path.clone(), VsCodeNodeKind::Function, line),
                            );

                            graph.nodes.push(VsCodeNode {
                                id: symbol_id.clone(),
                                label: name.to_string(),
                                kind: VsCodeNodeKind::Function,
                                file: rel_path.clone(),
                                line,
                            });

                            file_symbols
                                .entry(rel_path.clone())
                                .or_default()
                                .push(symbol_id);
                        }
                    }
                }
            }
        }

        // 2. Second Pass: Resolve edges and relationships
        let mut edge_counter = 0;
        let mut added_edges: HashSet<String> = HashSet::new();

        let mut add_edge_safe = |graph: &mut VsCodeGraph,
                                 source: String,
                                 target: String,
                                 relation: VsCodeEdgeRelation,
                                 counter: &mut i32| {
            let edge_key = format!("{}->{}:{:?}", source, target, relation);
            if added_edges.insert(edge_key) {
                *counter += 1;
                graph.edges.push(VsCodeEdge {
                    id: format!("edge_{}", counter),
                    source,
                    target,
                    relation,
                });
            }
        };

        for fp in &files {
            let rel_path = self.get_relative_path(&fp.value, &root_dir.value);

            // Connect file to its defined symbols (structural parent relations)
            if let Some(syms) = file_symbols.get(&rel_path) {
                for sym_id in syms {
                    add_edge_safe(
                        &mut graph,
                        rel_path.clone(),
                        sym_id.clone(),
                        VsCodeEdgeRelation::Uses,
                        &mut edge_counter,
                    );
                }
            }

            // Extract raw symbols to check call graphs / usage
            if let Ok(raw_meta) = self.source_parser.get_raw_symbols(fp) {
                if let Some(value_map) = raw_meta.value {
                    // 2.a. Imports / File-Level Uses
                    if let Some(imports_val) = value_map.get("aliases") {
                        if let Some(aliases_map) = imports_val.as_object() {
                            for (_alias, full_import_path) in aliases_map {
                                if let Some(imported_module) = full_import_path.as_str() {
                                    // Try to find the file matching this import module
                                    let clean_import = imported_module.replace('.', "/");
                                    for other_fp in &files {
                                        let other_rel = self
                                            .get_relative_path(&other_fp.value, &root_dir.value);
                                        if other_rel != rel_path
                                            && (other_rel.contains(&clean_import)
                                                || clean_import.contains(
                                                    &other_rel
                                                        .replace(".rs", "")
                                                        .replace(".py", "")
                                                        .replace(".ts", ""),
                                                ))
                                        {
                                            // Determine if lateral (same prefix layer)
                                            let is_lateral =
                                                self.is_lateral_layer(&rel_path, &other_rel);
                                            let relation = if is_lateral {
                                                VsCodeEdgeRelation::Lateral
                                            } else {
                                                VsCodeEdgeRelation::Uses
                                            };
                                            add_edge_safe(
                                                &mut graph,
                                                rel_path.clone(),
                                                other_rel,
                                                relation,
                                                &mut edge_counter,
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // 2.b. Inheritance and Traits
                    if let Some(bases_val) = value_map.get("class_bases") {
                        if let Some(bases_map) = bases_val.as_object() {
                            for (class_name, bases_arr) in bases_map {
                                if let Some(bases) = bases_arr.as_array() {
                                    for base in bases {
                                        if let Some(base_name) = base.as_str() {
                                            // Check if we have the base definition
                                            if let Some((def_file, kind, _)) =
                                                symbol_definitions.get(base_name)
                                            {
                                                let source_id =
                                                    format!("{}#{}", rel_path, class_name);
                                                let target_id =
                                                    format!("{}#{}", def_file, base_name);

                                                let relation = match kind {
                                                    VsCodeNodeKind::Trait
                                                    | VsCodeNodeKind::Interface => {
                                                        VsCodeEdgeRelation::Implement
                                                    }
                                                    _ => VsCodeEdgeRelation::Inherit,
                                                };

                                                add_edge_safe(
                                                    &mut graph,
                                                    source_id,
                                                    target_id,
                                                    relation,
                                                    &mut edge_counter,
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // 2.c. Outgoing & Incoming Calls (based on used symbols)
                    if let Some(used_val) = value_map.get("used") {
                        if let Some(used_arr) = used_val.as_array() {
                            for u in used_arr {
                                if let Some(used_sym) = u.as_str() {
                                    // Check if this symbol is defined elsewhere in the codebase
                                    if let Some((def_file, def_kind, _)) =
                                        symbol_definitions.get(used_sym)
                                    {
                                        if *def_file != rel_path {
                                            let target_id = format!("{}#{}", def_file, used_sym);

                                            // Connect the current file/module to the external symbol
                                            // Let's check if there is a specific defined function in this file that calls it,
                                            // or just connect from the file itself.
                                            let source_id = rel_path.clone();

                                            let is_fn_or_class = matches!(
                                                def_kind,
                                                VsCodeNodeKind::Function | VsCodeNodeKind::Class
                                            );
                                            let relation = if is_fn_or_class {
                                                VsCodeEdgeRelation::OutgoingCall
                                            } else {
                                                VsCodeEdgeRelation::Uses
                                            };

                                            add_edge_safe(
                                                &mut graph,
                                                source_id,
                                                target_id,
                                                relation,
                                                &mut edge_counter,
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(graph)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_vscode_graph_generator_basic() {
        let test_dir_str = "target/test_vscode_graph";
        let mock_file_path = format!("{}/src/capabilities_mock_checker.rs", test_dir_str);

        fs::create_dir_all(format!("{}/src", test_dir_str)).unwrap();

        let code = r#"
use shared::contract_port::IPort;

pub struct MockChecker {
    pub name: String,
}

pub fn check_something() {}
"#;
        fs::write(&mock_file_path, code).unwrap();

        let root_dir = DirectoryPath::new(test_dir_str.to_string()).unwrap();
        let generator = VsCodeGraphGenerator::new();
        let graph = generator.generate_graph(&root_dir).unwrap();

        // Check node counts and contents
        assert!(!graph.nodes.is_empty(), "Nodes should not be empty");

        let file_nodes: Vec<_> = graph
            .nodes
            .iter()
            .filter(|n| matches!(n.kind, VsCodeNodeKind::File))
            .collect();
        assert_eq!(file_nodes.len(), 1);
        assert_eq!(file_nodes[0].label, "capabilities_mock_checker.rs");

        let class_nodes: Vec<_> = graph
            .nodes
            .iter()
            .filter(|n| matches!(n.kind, VsCodeNodeKind::Class))
            .collect();
        assert_eq!(class_nodes.len(), 1);
        assert_eq!(class_nodes[0].label, "MockChecker");

        let func_nodes: Vec<_> = graph
            .nodes
            .iter()
            .filter(|n| matches!(n.kind, VsCodeNodeKind::Function))
            .collect();
        assert_eq!(func_nodes.len(), 1);
        assert_eq!(func_nodes[0].label, "check_something");

        // Clean up
        let _ = fs::remove_dir_all(test_dir_str);
    }
}
