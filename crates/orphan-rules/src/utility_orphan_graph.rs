// PURPOSE: Pure graph traversal utility for orphan detection
// Stateless, domain-agnostic BFS over ImportGraph — reusable across modules.

use shared::quality_rules::taxonomy_analysis_vo::ImportGraph;
use std::collections::{HashSet, VecDeque};

/// BFS reachability from entry points through the import graph.
/// Returns the set of all files reachable from any entry point.
/// Pure function — no state, no I/O.
pub fn trace_reachability(entry_points: &[String], graph: &ImportGraph) -> HashSet<String> {
    let mut reachable: HashSet<String> = entry_points.iter().cloned().collect();
    let mut queue: VecDeque<String> = entry_points.iter().cloned().collect();
    let mut edges_found = 0u32;
    for ep in entry_points.iter().take(3) {
        if let Some(n) = graph.mapping.get(ep) {
            edges_found += n.len() as u32;
        }
    }
    eprintln!(
        "[debug bfs] eps={} graph_keys={} edges_from_first_3={}",
        entry_points.len(),
        graph.mapping.len(),
        edges_found
    );

    while let Some(current) = queue.pop_front() {
        if let Some(neighbors) = graph.mapping.get(&current) {
            for neighbor in neighbors {
                if reachable.insert(neighbor.clone()) {
                    queue.push_back(neighbor.clone());
                }
            }
        }
    }
    eprintln!(
        "[debug bfs] reachable={} not_reachable_sample={:?}",
        reachable.len(),
        graph
            .mapping
            .keys()
            .filter(|k| !reachable.contains(*k))
            .take(5)
            .collect::<Vec<_>>()
    );
    // Debug: check if shared taxonomy files have inbound links
    let sample_target = "crates/shared/src/common/taxonomy_definition_vo.rs";
    eprintln!(
        "[debug inbound] {} has_key={}",
        sample_target,
        graph.mapping.contains_key(sample_target)
    );
    reachable
}
