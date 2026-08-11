// PURPOSE: Container-aware wiring propagation for DI traceability (issues #191-193).
// Stateless, domain-agnostic — reusable across crates.
//
// Two synthetic-edge builders complement the import graph so BFS reachability can
// follow dependency-injection paths that are invisible to static imports:
//
//  1. `add_impl_bridge_edges`   — contract → capabilities bridge.
//     A capability implements a contract via `impl Trait for T` (Rust),
//     `class T(Base)` (Python), or `class T implements I` (TypeScript). The
//     import graph stops at the contract, so we add an edge contract → capability
//     for every known implementor. This is the reverse index from issue #193.
//
//  2. `add_container_wiring_edges` — container → wired services bridge.
//     Containers wire capabilities/agents by referencing their type name
//     (constructor injection, factory, registry). We resolve the container's
//     *used identifiers* against the workspace symbol table and add an edge
//     container → each referenced defining file. Whole-word, AST-derived
//     identifiers keep this precise (no substring guesses).
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
};

/// Add contract → implementor edges (reverse index, issue #193).
///
/// For every trait/interface/base name, look up the file(s) that *define* it
/// (`symbol_definitions`) and wire each defining (contract) file to every
/// implementing file. Edge direction follows reachability: if the contract is
/// reachable, its implementors become reachable too.
pub fn add_impl_bridge_edges(
    top_root: &Path,
    symbol_definitions: &HashMap<String, Vec<PathBuf>>,
    implementations: &HashMap<String, Vec<PathBuf>>,
    forward: &mut HashMap<String, Vec<String>>,
) {
    for (trait_name, impl_paths) in implementations {
        let Some(def_paths) = symbol_definitions.get(trait_name) else {
            continue;
        };
        for def_path in def_paths {
            let def_rel = path_to_relative(def_path, top_root);
            for impl_path in impl_paths {
                let impl_rel = path_to_relative(impl_path, top_root);
                if def_rel != impl_rel {
                    forward.entry(def_rel.clone()).or_default().push(impl_rel);
                }
            }
        }
    }
}

/// Add container → wired-service edges (P1/P2).
///
/// For each `*_container.*` file, collect its AST-derived used identifiers and
/// resolve every one that names a workspace symbol (struct/class/interface/trait)
/// to its defining file. An edge container → defining_file is added, so BFS that
/// reaches the container also reaches everything it wires via DI.
pub fn add_container_wiring_edges(
    all_files: &[String],
    top_root: &Path,
    symbol_definitions: &HashMap<String, Vec<PathBuf>>,
    used_identifiers_for: impl Fn(&Path) -> Vec<String>,
    forward: &mut HashMap<String, Vec<String>>,
) {
    for file_path in all_files {
        if !file_path.contains("_container") {
            continue;
        }
        let abs = top_root.join(file_path);
        let identifiers = used_identifiers_for(&abs);
        let mut seen: HashSet<String> = HashSet::new();
        for ident in &identifiers {
            let Some(def_paths) = symbol_definitions.get(ident) else {
                continue;
            };
            for dp in def_paths {
                let rel = path_to_relative(dp, top_root);
                if rel != *file_path && seen.insert(rel.clone()) {
                    forward.entry(file_path.clone()).or_default().push(rel);
                }
            }
        }
    }
}

/// Free utility: convert absolute path to workspace-relative string.
pub fn path_to_relative(path: &Path, root: &Path) -> String {
    path.strip_prefix(root)
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| path.to_string_lossy().to_string())
}
