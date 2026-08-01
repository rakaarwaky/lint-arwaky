// PURPOSE: utility_orphan_graph_resolver — helper functions for graph resolution
use std::collections::HashMap;

/// Build a crate module index for hyphen-aware resolution.
/// Maps normalized module paths to canonical file paths.
pub fn build_crate_module_index(
    crate_src_dirs: &HashMap<String, std::path::PathBuf>,
) -> HashMap<String, HashMap<String, String>> {
    let mut index: HashMap<String, HashMap<String, String>> = HashMap::new();
    for (crate_name, src_dir) in crate_src_dirs {
        let mut module_map: HashMap<String, String> = HashMap::new();
        let canonical_src = std::fs::canonicalize(src_dir).unwrap_or_else(|_| src_dir.clone());
        let all_files = super::utility_orphan_io::scan_directory_recursive(&canonical_src);
        for path_str in all_files {
            if !path_str.ends_with(".rs")
                && !path_str.ends_with(".py")
                && !path_str.ends_with(".ts")
                && !path_str.ends_with(".js")
            {
                continue;
            }
            // Bug 9/NFR fix: Avoid per-file canonicalize syscall — use path as-is
            // scan_directory_recursive already returns paths within canonical_src.
            let path = std::path::Path::new(&path_str);
            let stem = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or_default()
                .to_string();
            if stem.is_empty() {
                continue;
            }
            let rel_path = path.strip_prefix(&canonical_src).unwrap_or(path);
            let rel_str = rel_path.with_extension("").to_string_lossy().to_string();
            let normalized_rel = super::utility_orphan_detector::normalize_module_path(
                &rel_str.replace(std::path::MAIN_SEPARATOR, "/"),
            );
            module_map.insert(normalized_rel, path_str.clone());
            module_map.insert(stem.clone(), path_str.clone());
            module_map.insert(
                super::utility_orphan_detector::normalize_module_component(&stem),
                path_str.clone(),
            );
            if (stem == "mod" || stem == "__init__" || stem == "index")
                && let Some(parent_dir) = path.parent().and_then(|p| p.file_name())
            {
                let parent = parent_dir.to_string_lossy().to_string();
                module_map.insert(parent.clone(), path_str.clone());
                module_map.insert(
                    super::utility_orphan_detector::normalize_module_component(&parent),
                    path_str.clone(),
                );
            }
        }
        let normalized_name =
            super::utility_orphan_detector::normalize_module_component(crate_name);
        index.insert(crate_name.clone(), module_map.clone());
        index.insert(normalized_name, module_map);
    }
    index
}

/// Resolve a workspace module path to its canonical file path.
pub fn resolve_workspace_module(
    index: &HashMap<String, HashMap<String, String>>,
    crate_name: &str,
    segments: &[&str],
    current_file: &str,
) -> Option<String> {
    let map = index.get(crate_name)?;
    let seg_str = segments.join("/");
    let normalized = super::utility_orphan_detector::normalize_module_path(&seg_str);
    if let Some(path) = map.get(&normalized)
        && path != current_file
    {
        return Some(path.clone());
    }
    for i in (1..segments.len()).rev() {
        let candidate = segments[..i].join("/");
        let normalized = super::utility_orphan_detector::normalize_module_path(&candidate);
        if let Some(path) = map.get(&normalized)
            && path != current_file
        {
            return Some(path.clone());
        }
    }
    None
}

/// Add an edge to the import graph and inbound links.
pub fn add_edge(
    import_graph: &mut HashMap<String, Vec<String>>,
    inbound_links: &mut HashMap<String, Vec<String>>,
    source: &str,
    target: &str,
) {
    import_graph
        .entry(source.to_string())
        .or_default()
        .push(target.to_string());
    inbound_links
        .entry(target.to_string())
        .or_default()
        .push(source.to_string());
}

/// Find workspace root by walking up from start_dir until we find a directory
/// with a project manifest (Cargo.toml, pyproject.toml, package.json) AND
/// a member directory (crates/, packages/, modules/).
pub fn find_workspace_root(start_dir: &str) -> String {
    let mut current = std::path::PathBuf::from(start_dir);
    // Make absolute so parent() doesn't return "" for relative single-segment paths
    if current.is_relative()
        && let Ok(cwd) = std::env::current_dir()
    {
        current = cwd.join(&current);
    }
    loop {
        let has_manifest = current.join("Cargo.toml").exists()
            || current.join("pyproject.toml").exists()
            || current.join("package.json").exists();
        let has_members = current.join("crates").exists()
            || current.join("packages").exists()
            || current.join("modules").exists();
        if has_manifest && has_members {
            return current.to_string_lossy().to_string();
        }
        // Move up one directory
        if !current.pop() {
            // Reached filesystem root without finding workspace root
            return start_dir.to_string();
        }
    }
}

/// Deduplicate edges in a HashMap<String, Vec<String>> by key.
/// This prevents inflating inbound link counts with duplicate entries.
pub fn dedup_edges(map: &mut HashMap<String, Vec<String>>) {
    let keys: Vec<String> = map.keys().cloned().collect();
    for k in keys {
        if let Some(values) = map.get(&k) {
            let mut seen = std::collections::HashSet::new();
            let deduped: Vec<String> = values
                .iter()
                .filter(|x| seen.insert(x.as_str()))
                .cloned()
                .collect();
            if deduped.len() != values.len() {
                map.insert(k, deduped);
            }
        }
    }
}

/// Resolve a TypeScript/JavaScript relative import path against the current file's directory.
/// Handles extensionless imports by trying .ts, .js, .tsx, .jsx, and /index.* files.
/// Returns the relative path (matching graph convention) or None if not found.
pub fn resolve_ts_relative(
    current_file: &str,
    import_path: &str,
    workspace_root: &std::path::Path,
) -> Option<String> {
    let base = std::path::Path::new(current_file).parent()?;
    let joined = base.join(import_path);

    // TS imports omit extension — try appending .ts, .js, .tsx, .jsx, /index.ts, /index.js
    // Use format! instead of with_extension to handle paths containing dots
    // (e.g., "./utils/helper.v2" → "helper.v2.ts", NOT "helper.ts")
    let joined_str = joined.to_string_lossy();
    let candidates: Vec<std::path::PathBuf> = vec![
        std::path::PathBuf::from(format!("{}.ts", joined_str)),
        std::path::PathBuf::from(format!("{}.js", joined_str)),
        std::path::PathBuf::from(format!("{}.tsx", joined_str)),
        std::path::PathBuf::from(format!("{}.jsx", joined_str)),
        joined.join("index.ts"),
        joined.join("index.js"),
    ];

    for cand in &candidates {
        if super::utility_orphan_io::is_file(cand) {
            // Return RELATIVE path (matching graph convention)
            let rel = cand
                .strip_prefix(workspace_root)
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|_| cand.to_string_lossy().to_string());
            if rel != current_file {
                return Some(rel);
            }
        }
    }
    None
}
