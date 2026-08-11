// PURPOSE: Pure graph traversal utility for orphan detection
// Stateless, domain-agnostic BFS over ImportGraph — reusable across modules.

use shared::quality_rules::taxonomy_analysis_vo::{ImportGraph, InheritanceMap};
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

/// Extended BFS that follows the contract→capabilities impl bridge.
/// After standard trace_reachability, adds all capability files that
/// implement reachable contracts via trait impl edges.
///
/// This closes the gap: Agent imports Contract ✅ → but capabilities
/// implement the contract (not imported), so BFS stops at contract.
/// The inheritance_map provides trait_name → [impl_file] edges.
pub fn trace_reachability_with_impl_bridge(
    entry_points: &[String],
    graph: &ImportGraph,
    inheritance_map: &InheritanceMap,
) -> HashSet<String> {
    // Phase 1: standard BFS through import graph
    let mut reachable = trace_reachability(entry_points, graph);

    debug!(
        post_import_reachable = reachable.len(),
        "BFS import phase complete, starting impl bridge"
    );

    // Phase 2: for each reachable file that is a contract (protocol/aggregate suffix),
    // find all files implementing it and add them to alive set
    for contract_file in reachable.iter().cloned().collect::<Vec<_>>() {
        let stem = contract_file.rsplit('/').next().unwrap_or(&contract_file);
        if let Some(pos) = stem.find('_') {
            let suffix = &stem[pos + 1..];
            // Only contracts with protocol or aggregate suffix have impl bridge
            if suffix != "protocol" && suffix != "aggregate" {
                continue;
            }
            // Extract trait name from contract filename: e.g. "contract_io_protocol.rs" → "IIOProtocol"
            // Contract names follow pattern: {layer}_{name}_{suffix} → trait name is PascalCase
            let parts: Vec<&str> = stem[..pos].split('_').collect();
            if parts.len() >= 2 {
                let trait_name: String = parts[1..]
                    .iter()
                    .filter(|s| !s.is_empty())
                    .map(|s| {
                        let mut chars = s.chars();
                        match chars.next() {
                            Some(c) => c.to_uppercase().to_string() + chars.as_str(),
                            None => String::new(),
                        }
                    })
                    .collect::<Vec<String>>()
                    .join("");

                // Look up all implementors of this trait in the inheritance map
                if let Some(impl_files) = inheritance_map.get_trait_implementors(&trait_name) {
                    for impl_file in impl_files {
                        if reachable.insert(impl_file.clone()) {
                            debug!(contract = trait_name, impl = impl_file, "bridge: contract→capabilities");
                        }
                    }
                }
            }
        }
    }

    debug!(
        final_reachable = reachable.len(),
        "BFS reachability with impl bridge complete"
    );

    reachable
}
