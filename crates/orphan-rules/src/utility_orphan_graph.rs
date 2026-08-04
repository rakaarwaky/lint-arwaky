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

    while let Some(current) = queue.pop_front() {
        if let Some(neighbors) = graph.mapping.get(&current) {
            for neighbor in neighbors {
                if reachable.insert(neighbor.clone()) {
                    queue.push_back(neighbor.clone());
                }
            }
        }
    }

    reachable
}
