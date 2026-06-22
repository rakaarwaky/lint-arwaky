// PURPOSE: taxonomy_cycle_helper — pure utility functions for cycle and layer path normalization
use crate::import_rules::taxonomy_dependency_edge_vo::DependencyEdge;
use crate::taxonomy_name_vo::SymbolName;
use std::collections::{HashMap, HashSet};

pub fn normalize_to_layer(name: &str) -> String {
    let layer_prefixes = [
        "taxonomy_",
        "contract_",
        "capabilities_",
        "infrastructure_",
        "agent_",
        "surface_",
    ];
    let base = name.rsplit('/').next().unwrap_or(name);
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

    let mut graph: HashMap<String, HashSet<String>> = HashMap::new();
    for e in &normalized_edges {
        graph
            .entry(e.source.clone())
            .or_default()
            .insert(e.target.clone());
    }

    let mut unique_cycles: Vec<String> = Vec::new();
    let mut reported: HashSet<String> = HashSet::new();

    let nodes: Vec<String> = graph.keys().cloned().collect();

    for node in &nodes {
        let mut local_visited: HashSet<String> = HashSet::new();
        let mut path_stack: Vec<String> = Vec::new();
        let mut cycles: Vec<Vec<String>> = Vec::new();
        dfs_collect_paths(
            node,
            &graph,
            &mut local_visited,
            &mut path_stack,
            &mut cycles,
        );

        for cycle in cycles {
            let mut sorted_cycle = cycle.clone();
            sorted_cycle.sort();
            let dedup_key = sorted_cycle.join("->");
            if reported.insert(dedup_key) {
                for i in 0..cycle.len() {
                    let next = cycle[(i + 1) % cycle.len()].clone();
                    unique_cycles.push(format!("{}->{}", cycle[i].clone(), next));
                }
            }
        }
    }

    unique_cycles.into_iter().map(SymbolName::new).collect()
}

// ─── Private Helpers ───

fn dfs_collect_paths(
    node: &str,
    graph: &HashMap<String, HashSet<String>>,
    visited: &mut HashSet<String>,
    path_stack: &mut Vec<String>,
    cycles: &mut Vec<Vec<String>>,
) {
    if path_stack.contains(&node.to_string()) {
        if let Some(pos) = path_stack.iter().position(|n| n == node) {
            let cycle: Vec<String> = path_stack[pos..].to_vec();
            cycles.push(cycle);
        }
        return;
    }
    if visited.contains(node) {
        return;
    }
    visited.insert(node.to_string());
    path_stack.push(node.to_string());

    if let Some(neighbors) = graph.get(node) {
        for neighbor in neighbors {
            dfs_collect_paths(neighbor, graph, visited, path_stack, cycles);
        }
    }

    path_stack.pop();
}
