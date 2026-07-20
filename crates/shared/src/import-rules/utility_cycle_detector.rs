use crate::common::taxonomy_name_vo::SymbolName;
use crate::import_rules::taxonomy_dependency_edge_vo::DependencyEdge;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, PartialEq)]
enum Color {
    White,
    Gray,
    Black,
}

pub fn normalize_to_layer(name: &str) -> String {
    let layer_prefixes = [
        "taxonomy_",
        "contract_",
        "capabilities_",
        "utility_",
        "agent_",
        "surface_",
    ];
    let base = match name.rsplit('/').next() {
        Some(b) => b,
        None => name,
    };
    for prefix in &layer_prefixes {
        if base.starts_with(prefix) {
            return prefix.trim_end_matches('_').to_string();
        }
    }
    name.to_string()
}

pub fn detect_cycle_edges(edges: &[DependencyEdge]) -> Vec<SymbolName> {
    let normalized_edges: Vec<DependencyEdge> = edges
        .iter()
        .map(|e| DependencyEdge::new(normalize_to_layer(&e.source), normalize_to_layer(&e.target)))
        .collect();

    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for e in &normalized_edges {
        graph
            .entry(e.source.clone())
            .or_default()
            .push(e.target.clone());
        graph.entry(e.target.clone()).or_default();
    }

    let mut color: HashMap<String, Color> = HashMap::new();
    let mut parent: HashMap<String, String> = HashMap::new();
    let mut cycle_edges_set: HashSet<(String, String)> = HashSet::new();

    for node in graph.keys() {
        color.entry(node.clone()).or_insert(Color::White);
    }

    for node in graph.keys().cloned().collect::<Vec<_>>() {
        if color[&node] == Color::White {
            dfs_3color(&node, &graph, &mut color, &mut parent, &mut cycle_edges_set);
        }
    }

    let mut unique_cycles: Vec<String> = Vec::new();
    let mut reported: HashSet<String> = HashSet::new();

    for (src, tgt) in &cycle_edges_set {
        let cycle_nodes = extract_cycle_nodes(src, tgt, &parent);
        if let Some(cycle) = cycle_nodes {
            let mut sorted_cycle = cycle.clone();
            sorted_cycle.sort();
            let dedup_key = sorted_cycle.join("->");
            if reported.insert(dedup_key) {
                for i in 0..cycle.len() {
                    let next = cycle[(i + 1) % cycle.len()].clone();
                    unique_cycles.push(format!("{}->{}", cycle[i], next));
                }
            }
        }
    }

    unique_cycles.into_iter().map(SymbolName::new).collect()
}

fn dfs_3color(
    node: &str,
    graph: &HashMap<String, Vec<String>>,
    color: &mut HashMap<String, Color>,
    parent: &mut HashMap<String, String>,
    cycle_edges: &mut HashSet<(String, String)>,
) {
    color.insert(node.to_string(), Color::Gray);

    if let Some(neighbors) = graph.get(node) {
        for neighbor in neighbors {
            if *color.get(neighbor).unwrap_or(&Color::White) == Color::Gray {
                cycle_edges.insert((node.to_string(), neighbor.clone()));
            } else if *color.get(neighbor).unwrap_or(&Color::White) == Color::White {
                parent.insert(neighbor.clone(), node.to_string());
                dfs_3color(neighbor, graph, color, parent, cycle_edges);
            }
        }
    }

    color.insert(node.to_string(), Color::Black);
}

fn extract_cycle_nodes(
    src: &str,
    tgt: &str,
    parent: &HashMap<String, String>,
) -> Option<Vec<String>> {
    let mut path = Vec::new();
    let mut cur = src;
    path.push(cur.to_string());

    while cur != tgt {
        match parent.get(cur) {
            Some(p) => {
                cur = p;
                path.push(cur.to_string());
            }
            None => return None,
        }
    }

    path.reverse();
    Some(path)
}
