// PURPOSE: Pure graph traversal utility for orphan detection
// Stateless, domain-agnostic BFS over ImportGraph — reusable across modules.

use shared::quality_rules::taxonomy_analysis_vo::ImportGraph;
use std::collections::{HashSet, VecDeque};
use tracing::debug;

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
    debug!(
        entry_points = entry_points.len(),
        graph_keys = graph.mapping.len(),
        edges_from_first_3 = edges_found,
        "BFS reachability start"
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
    debug!(
        reachable = reachable.len(),
        not_reachable_sample = ?graph
            .mapping
            .keys()
            .filter(|k| !reachable.contains(*k))
            .take(5)
            .collect::<Vec<_>>(),
        "BFS reachability complete"
    );
    reachable
}
