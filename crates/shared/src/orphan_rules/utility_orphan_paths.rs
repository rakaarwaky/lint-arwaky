// PURPOSE: Shared path-equivalence helpers for orphan detection.
// Used by: orphan-rules analyzers (contract, agent, utility) so all layers
// compare workspace-relative paths identically.

/// Normalizes a workspace-relative path for comparison.
///
/// Strips a leading `./` prefix and converts backslashes to forward slashes so
/// that paths originating from different sources compare consistently.
pub fn normalize_rel_path(p: &str) -> String {
    p.trim_start_matches("./").replace('\\', "/")
}

/// Determines whether two workspace-relative paths refer to the same file.
///
/// Both inputs use the same `path_to_relative` format (workspace-relative,
/// forward slashes). Matching is strict: exact equality after normalization, or a
/// suffix match that begins at a path-separator boundary. Bare-basename matching
/// is intentionally avoided so that common names such as `mod.rs`, `index.ts`, or
/// `lib.rs` in one module cannot validate an unrelated file in another module.
pub fn paths_equivalent(rel: &str, alive: &str) -> bool {
    let a = normalize_rel_path(rel);
    let b = normalize_rel_path(alive);
    if a == b {
        return true;
    }
    // Suffix match only at a separator boundary. A path without any directory
    // component (bare basename) is only ever equal — never a suffix match — so
    // a bare `lib.rs` cannot validate an unrelated deeper `lib.rs`.
    if a.contains('/') {
        if let Some(rest) = b.strip_suffix(&a) {
            if rest.is_empty() || rest.ends_with('/') {
                return true;
            }
        }
    }
    if b.contains('/') {
        if let Some(rest) = a.strip_suffix(&b) {
            if rest.is_empty() || rest.ends_with('/') {
                return true;
            }
        }
    }
    false
}

/// Determines whether a workspace-relative path corresponds to any reachable file path.
///
/// Paths may be relative or absolute and may include a `./` prefix. Matching uses
/// [`paths_equivalent`], so common file names in different modules never collide.
///
/// # Arguments
///
/// * `rel` - Workspace-relative path to compare.
/// * `alive_files` - Reachability results containing paths known to be reachable.
///
/// # Returns
///
/// `true` if a reachable path matches the supplied path, `false` otherwise.
pub fn is_path_alive(
    rel: &str,
    alive_files: &shared_quality_rules::taxonomy_quality_rules_vo::ReachabilityResult,
) -> bool {
    alive_files
        .paths
        .iter()
        .any(|af| paths_equivalent(rel, af.value()))
}

/// Join up to `cap` names with `, `, appending `, and N more` when truncated.
///
/// Returns `a, b, c` when `names.len() <= cap`, otherwise `a, b, c, and N more`
/// where `N` is the number of remaining names.
pub fn summarize_names(names: &[String], cap: usize) -> String {
    if names.len() <= cap {
        names.join(", ")
    } else {
        let shown = &names[..cap];
        let remaining = names.len() - cap;
        format!("{}, and {} more", shown.join(", "), remaining)
    }
}
