// PURPOSE: Container-aware wiring propagation for DI traceability (issues #191-193).
// Stateless, domain-agnostic — reusable across crates.
//
// Synthetic-edge builders complement the import graph so BFS reachability can
// follow dependency-injection paths that are invisible to static imports:
//
//  1. `add_impl_bridge_edges`    — contract → capabilities bridge.
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
//
//  3. `add_lib_rs_edges`         — crate entry cross-links for Rust workspaces.
//     All `lib.rs` roots are mutually linked so BFS escapes any single crate.
//
//  4. `build_reverse_index`      — inbound-link map derived from the forward graph
//     once every synthetic edge has been added, keeping import-only and
//     DI-derived inbound links consistent for all orphan analyzers.
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
};

/// Adds graph edges from contract-defining files to their implementing files.
///
/// Each contract is resolved through `symbol_definitions`. Missing definitions and
/// self-edges are skipped; paths are stored relative to `top_root`.
///
/// # Parameters
///
/// * `top_root` — Workspace root used to relativize file paths.
/// * `symbol_definitions` — Maps contract names to the files that define them.
/// * `implementations` — Maps contract names to their implementing files.
/// * `forward` — Forward graph to update with the resulting edges.
///
/// # Examples
///
/// ```
/// use std::{collections::HashMap, path::PathBuf};
///
/// let mut definitions = HashMap::new();
/// definitions.insert("Service".to_string(), vec![PathBuf::from("/workspace/service.rs")]);
///
/// let mut implementations = HashMap::new();
/// implementations.insert(
///     "Service".to_string(),
///     vec![PathBuf::from("/workspace/service_impl.rs")],
/// );
///
/// let mut forward = HashMap::new();
/// add_impl_bridge_edges(
///     PathBuf::from("/workspace").as_path(),
///     &definitions,
///     &implementations,
///     &mut forward,
/// );
///
/// assert_eq!(
///     forward.get("service.rs"),
///     Some(&vec!["service_impl.rs".to_string()])
/// );
/// ```
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
            let mut added = false;
            for impl_path in impl_paths {
                let impl_rel = path_to_relative(impl_path, top_root);
                if def_rel == impl_rel {
                    continue;
                }
                let existing = forward.entry(def_rel.clone()).or_default();
                if !existing.contains(&impl_rel) {
                    existing.push(impl_rel);
                    added = true;
                }
            }
            // Remove the entry if every implementation was a self-edge — an
            // empty adjacency list is indistinguishable from "no edges".
            if !added {
                forward.remove(&def_rel);
            }
        }
    }
}

/// Adds edges from dependency-injection container files to the files defining
/// the workspace symbols they reference.
///
/// Container files are identified by paths containing `_container`. Unresolved
/// identifiers, self-edges, and duplicate targets are skipped.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
/// use std::path::Path;
///
/// let mut symbols = HashMap::new();
/// symbols.insert(
///     "Service".to_string(),
///     vec![Path::new("/workspace/src/service.rs").to_path_buf()],
/// );
///
/// let mut forward = HashMap::new();
/// add_container_wiring_edges(
///     &["src/app_container.rs".to_string()],
///     Path::new("/workspace"),
///     &symbols,
///     |_| vec!["Service".to_string()],
///     &mut forward,
/// );
///
/// assert_eq!(
///     forward["src/app_container.rs"],
///     vec!["src/service.rs".to_string()]
/// );
/// ```
///
/// # Arguments
///
/// * `all_files` - Workspace-relative file paths to inspect.
/// * `top_root` - Workspace root used to resolve and relativize paths.
/// * `symbol_definitions` - Definitions of workspace symbols keyed by name.
/// * `used_identifiers_for` - Extracts identifiers referenced by a file.
/// * `forward` - Forward dependency graph to update.
pub fn add_container_wiring_edges(
    all_files: &[String],
    top_root: &Path,
    symbol_definitions: &HashMap<String, Vec<PathBuf>>,
    used_identifiers_for: impl Fn(&Path) -> Vec<String>,
    forward: &mut HashMap<String, Vec<String>>,
) {
    for file_path in all_files {
        // A container is identified by its file stem (e.g. `root_app_container.rs`),
        // not by directories along the path that happen to contain `_container`.
        let is_container = Path::new(file_path)
            .file_stem()
            .and_then(|s| s.to_str())
            .is_some_and(|stem| stem.contains("_container"));
        if !is_container {
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

/// Converts a path under the workspace root into a workspace-relative string.
///
/// Paths outside the workspace root are returned as strings without modification.
///
/// # Examples
///
/// ```
/// use std::path::Path;
///
/// let root = Path::new("workspace");
/// let path = Path::new("workspace/src/lib.rs");
///
/// assert_eq!(path_to_relative(path, root), "src/lib.rs");
/// ```
pub fn path_to_relative(path: &Path, root: &Path) -> String {
    path.strip_prefix(root)
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| path.to_string_lossy().to_string())
}

/// Cross-links every `lib.rs` entry point with every other, so BFS reachability
/// can escape a single Rust crate through its crate root.
///
/// # Arguments
///
/// * `all_files` - Workspace-relative file paths to inspect.
/// * `forward` - Forward dependency graph to update.
pub fn add_lib_rs_edges(all_files: &[String], forward: &mut HashMap<String, Vec<String>>) {
    let lib_rs_files: Vec<String> = all_files
        .iter()
        .filter(|f| f.ends_with("/lib.rs"))
        .cloned()
        .collect();
    for lib in &lib_rs_files {
        for other_lib in &lib_rs_files {
            if lib != other_lib {
                forward
                    .entry(lib.clone())
                    .or_default()
                    .push(other_lib.clone());
            }
        }
    }
}

/// Builds the inbound-link map from the forward dependency graph.
///
/// Must be called after every synthetic edge (impl bridges, container wiring,
/// lib.rs cross-links) has been added so all analyzers observe the same
/// inbound links.
///
/// # Arguments
///
/// * `forward` - Forward dependency graph containing all edges.
///
/// # Returns
///
/// A map from each target path to the paths that reference it.
pub fn build_reverse_index(forward: &HashMap<String, Vec<String>>) -> HashMap<String, Vec<String>> {
    let mut reverse: HashMap<String, Vec<String>> = HashMap::new();
    for (src, targets) in forward {
        for tgt in targets {
            reverse.entry(tgt.clone()).or_default().push(src.clone());
        }
    }
    reverse
}
