// PURPOSE: OrphanGraphResolver — build graph context and identify entry points for orphan analysis.
use shared::code_analysis::{
    FileDefinitionMap, GraphAnalysisContext, ImportGraph, InboundLinkMap, InheritanceMap,
};

use crate::utility_orphan_graph_resolver;
use crate::utility_orphan_regex_patterns;
use shared::orphan_detector::utility_orphan_filename::file_stem;
use shared::orphan_detector::utility_orphan_io;
use shared::orphan_detector::IOrphanGraphResolverProtocol;
use shared::orphan_detector::{OrphanEntryPatternListVO, OrphanFileListVO};
use std::collections::{HashMap, HashSet};

// ─── Block 1: Struct Definition ───────────────────────────

/// Build graph context and identify entry points for orphan analysis.
pub struct OrphanGraphResolver {}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IOrphanGraphResolverProtocol for OrphanGraphResolver {
    fn build_graph_context(
        &self,
        files: &[OrphanFileListVO],
        root_dir: &str,
    ) -> GraphAnalysisContext {
        // Bridge the contract-level VO collection to the internal helper
        // which still uses raw `&[String]` for backward compatibility with
        // the rest of the orphan-detector graph builder.
        let raw_paths: Vec<String> = files
            .iter()
            .flat_map(|v| v.values.iter().cloned())
            .collect();
        self.build_graph_context_inner(&raw_paths, root_dir)
    }

    fn identify_entry_points(
        &self,
        files: &[OrphanFileListVO],
        configured: &[OrphanEntryPatternListVO],
    ) -> OrphanFileListVO {
        let file_strs: Vec<String> = files
            .iter()
            .flat_map(|v| v.values.iter().cloned())
            .collect();

        let configured_strs: Vec<String> = configured
            .iter()
            .flat_map(|p| p.values.iter().cloned())
            .collect();

        let mut matched: Vec<String> = if configured_strs.is_empty() {
            file_strs
                .iter()
                .filter(|f| {
                    let basename = f.rsplit('/').next().unwrap_or(f);
                    basename.ends_with("_container.rs")
                        || basename.ends_with("_container.py")
                        || basename.ends_with("_container.ts")
                        || basename.ends_with("_container.js")
                        || basename.ends_with("_entry.rs")
                        || basename.ends_with("_entry.py")
                        || basename.ends_with("_entry.ts")
                        || basename.ends_with("_entry.js")
                        || basename.starts_with("root_")
                        || basename == "main.rs"
                        || basename == "lib.rs"
                        || basename == "main.py"
                        || basename == "__main__.py"
                        || basename == "main.ts"
                        || basename == "main.js"
                        || basename == "index.ts"
                        || basename == "index.js"
                })
                .cloned()
                .collect()
        } else {
            file_strs
                .iter()
                .filter(|f| {
                    let basename = f.rsplit('/').next().unwrap_or(f);
                    // Bug 5 fix: Use exact/prefix/suffix match instead of contains()
                    // prevents false positives like "germanic_utils" matching "main"
                    let stem =
                        shared::orphan_detector::utility_orphan_filename::file_stem(basename);
                    configured_strs.iter().any(|pattern| {
                        basename == pattern
                            || (basename.ends_with(pattern)
                                && pattern.starts_with('_'))
                            || stem == *pattern
                            || stem.starts_with(pattern.as_str())
                            || stem.ends_with(pattern.as_str())
                    })
                })
                .cloned()
                .collect()
        };
        matched.sort();
        matched.dedup();
        OrphanFileListVO::new(matched)
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for OrphanGraphResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl OrphanGraphResolver {
    pub fn new() -> Self {
        Self {}
    }

    fn build_graph_context_inner(&self, files: &[String], root_dir: &str) -> GraphAnalysisContext {
        let mut import_graph: HashMap<String, Vec<String>> = HashMap::new();
        let mut inbound_links: HashMap<String, Vec<String>> = HashMap::new();
        let mut inheritance_map: HashMap<String, Vec<String>> = HashMap::new();
        let file_definitions: HashMap<String, Vec<String>> = HashMap::new();

        // Bug 11 fix: Use workspace_root consistently instead of root_dir for path resolution
        let workspace_root = utility_orphan_graph_resolver::find_workspace_root(root_dir);
        let root_path = std::path::Path::new(&workspace_root);

        // Build set of known workspace crate dirs for external dep detection
        let mut workspace_modules: std::collections::HashSet<String> =
            std::collections::HashSet::new();
        // Perf 10: Pre-compute crate_name -> src_dir map
        let mut crate_src_dirs: HashMap<String, std::path::PathBuf> = HashMap::new();
        for ws_dir in &["crates", "packages", "modules"] {
            let ws_path = root_path.join(ws_dir);
            if shared::orphan_detector::utility_orphan_io::is_dir(&ws_path) {
                let entries = shared::orphan_detector::utility_orphan_io::scan_directory(&ws_path);
                for (name, path_str, is_dir_entry) in entries {
                    if !is_dir_entry {
                        continue;
                    }
                    workspace_modules.insert(name.clone());
                    workspace_modules.insert(name.replace('-', "_"));
                    let src_dir = std::path::PathBuf::from(&path_str).join("src");
                    if shared::orphan_detector::utility_orphan_io::is_dir(&src_dir) {
                        crate_src_dirs.insert(name.clone(), src_dir.clone());
                        crate_src_dirs.insert(name.replace('-', "_"), src_dir);
                    }
                }
            }
        }

        // Build crate module index for hyphen-aware resolution
        let crate_module_index =
            utility_orphan_graph_resolver::build_crate_module_index(&crate_src_dirs);

        // Expand files to include all workspace source files for cross-crate import resolution
        // This ensures that when scanning a subfolder, imports from other crates are visible
        let mut all_workspace_files: Vec<String> = files.to_vec();
        let mut seen: HashSet<String> = files.iter().cloned().collect();
        let root_path_obj = std::path::Path::new(&workspace_root);
        for src_dir in crate_src_dirs.values() {
            let workspace_files =
                shared::orphan_detector::utility_orphan_io::scan_directory_recursive(src_dir);
            for f in workspace_files {
                // Normalize to relative path for consistency with initial files
                let rel = std::path::Path::new(&f)
                    .strip_prefix(root_path_obj)
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or(f);
                if seen.insert(rel.clone()) {
                    all_workspace_files.push(rel);
                }
            }
        }
        // Also scan root_*.rs files directly in crates/ directory (not in src/)
        for ws_dir in &["crates", "packages", "modules"] {
            let ws_path = root_path.join(ws_dir);
            if shared::orphan_detector::utility_orphan_io::is_dir(&ws_path) {
                let entries = shared::orphan_detector::utility_orphan_io::scan_directory(&ws_path);
                for (name, path_str, is_dir_entry) in entries {
                    if is_dir_entry {
                        continue; // Skip directories, we already scanned their src/
                    }
                    // Include root_*.rs files directly in crates/
                    if name.starts_with("root_")
                        && (name.ends_with(".rs")
                            || name.ends_with(".py")
                            || name.ends_with(".ts")
                            || name.ends_with(".js"))
                        && !seen.contains(&path_str)
                    {
                        let rel = std::path::Path::new(&path_str)
                            .strip_prefix(root_path_obj)
                            .map(|p| p.to_string_lossy().to_string())
                            .unwrap_or(path_str.clone());
                        seen.insert(rel.clone());
                        all_workspace_files.push(rel);
                    }
                }
            }
        }
        let files = &all_workspace_files;

        // Build a lookup: module_name -> file_path for crate:: resolution
        let mut module_to_file: HashMap<String, String> = HashMap::new();
        for f in files {
            let stem = file_stem(f);
            // Bug 3 fix: Use first-write-wins to prevent HashMap collision
            // when multiple crates have files with the same stem (e.g., helper.rs)
            if !module_to_file.contains_key(&stem) {
                module_to_file.insert(stem.clone(), f.clone());
            }
            if let Some(parent) = f.rsplit('/').nth(1) {
                let module_path = format!("{}/{}", parent, stem);
                if !module_to_file.contains_key(&module_path) {
                    module_to_file.insert(module_path.clone(), f.clone());
                    let normalized_path = module_path.replace('-', "_");
                    if normalized_path != module_path
                        && !module_to_file.contains_key(&normalized_path)
                    {
                        module_to_file.insert(normalized_path, f.clone());
                    }
                }
            }
            // Bug 3 fix: mod.rs -> map by parent directory name (first-write-wins)
            if stem == "mod" {
                if let Some(parent_dir) = f.rsplit('/').nth(1) {
                    if !module_to_file.contains_key(parent_dir) {
                        module_to_file.insert(parent_dir.to_string(), f.clone());
                        let normalized = parent_dir.replace('-', "_");
                        if normalized != parent_dir && !module_to_file.contains_key(&normalized) {
                            module_to_file.insert(normalized, f.clone());
                        }
                    }
                    if let Some(grandparent) = f.rsplit('/').nth(2) {
                        let composite = format!("{}/{}", grandparent, parent_dir);
                        if !module_to_file.contains_key(&composite) {
                            module_to_file.insert(composite.clone(), f.clone());
                            let normalized_composite = composite.replace('-', "_");
                            if normalized_composite != composite
                                && !module_to_file.contains_key(&normalized_composite)
                            {
                                module_to_file.insert(normalized_composite, f.clone());
                            }
                        }
                    }
                }
            }
            // Python __init__.py -> map by parent directory name (same as mod.rs for Python packages)
            if stem == "__init__" {
                if let Some(parent_dir) = f.rsplit('/').nth(1) {
                    if !module_to_file.contains_key(parent_dir) {
                        module_to_file.insert(parent_dir.to_string(), f.clone());
                        let normalized = parent_dir.replace('-', "_");
                        if normalized != parent_dir && !module_to_file.contains_key(&normalized) {
                            module_to_file.insert(normalized, f.clone());
                        }
                    }
                    if let Some(grandparent) = f.rsplit('/').nth(2) {
                        let composite = format!("{}/{}", grandparent, parent_dir);
                        if !module_to_file.contains_key(&composite) {
                            module_to_file.insert(composite.clone(), f.clone());
                            let normalized_composite = composite.replace('-', "_");
                            if normalized_composite != composite
                                && !module_to_file.contains_key(&normalized_composite)
                            {
                                module_to_file.insert(normalized_composite, f.clone());
                            }
                        }
                    }
                }
            }
        }

        // Perf 8: Single-pass file reading
        for f in files {
            import_graph.entry(f.clone()).or_default();
            let content = utility_orphan_io::read_file_safe(f);
            if content.is_empty()
                && !shared::orphan_detector::utility_orphan_io::is_file(&std::path::PathBuf::from(
                    f,
                ))
            {
                continue;
            }

            // Pass 1: #[path = "..."] pub mod (Bug 14 fix — link only the referenced file)
            if let Some(re) = utility_orphan_regex_patterns::pub_mod_path_re() {
                for cap in re.captures_iter(&content) {
                    let mod_path = cap[1].to_string();
                    let base_dir = match std::path::Path::new(f).parent() {
                        Some(p) => p.to_path_buf(),
                        None => continue,
                    };
                    // Bug 11 fix: Use workspace_root consistently for path resolution
                    let root_path = std::path::Path::new(&workspace_root);
                    let Some(resolved_path) =
                        shared::orphan_detector::utility_orphan_path::resolve_module_path(
                            root_path, &base_dir, &mod_path,
                        )
                    else {
                        continue;
                    };
                    let resolved = resolved_path.to_string_lossy().to_string();
                    if shared::orphan_detector::utility_orphan_io::is_file(
                        &std::path::PathBuf::from(&resolved),
                    ) && resolved != *f
                    {
                        import_graph
                            .entry(f.clone())
                            .or_default()
                            .push(resolved.clone());
                        inbound_links.entry(resolved).or_default().push(f.clone());
                    }
                }
            }

            // Pass 2: plain mod (Bug 10 fix)
            if let Some(re) = utility_orphan_regex_patterns::plain_mod_re() {
                for cap in re.captures_iter(&content) {
                    let mod_name = cap[1].to_string();
                    let parent = match std::path::Path::new(f).parent() {
                        Some(p) => p,
                        None => continue,
                    };
                    let candidates = [
                        parent.join(format!("{}.rs", mod_name)),
                        parent.join(&mod_name).join("mod.rs"),
                        parent.join(format!("{}.py", mod_name)),
                        parent.join(&mod_name).join("__init__.py"),
                    ];
                    for candidate in &candidates {
                        // Bug 8 fix: Resolve against workspace_root to be CWD-independent
                        let abs_candidate = if candidate.is_relative() {
                            root_path.join(candidate)
                        } else {
                            candidate.clone()
                        };
                        if shared::orphan_detector::utility_orphan_io::is_file(&abs_candidate) {
                            if let Some(path_str) = candidate.to_str() {
                                let resolved = path_str.to_string();
                                if resolved != *f {
                                    import_graph
                                        .entry(f.clone())
                                        .or_default()
                                        .push(resolved.clone());
                                    inbound_links.entry(resolved).or_default().push(f.clone());
                                    break;
                                }
                            }
                        }
                    }
                }
            }

            // Pass 3: use/import/from
            let Some(import_re) = utility_orphan_regex_patterns::import_re() else {
                continue;
            };
            for cap in import_re.captures_iter(&content) {
                let full_import = cap[1].to_string();
                let cap_end = cap.get(0).map(|m| m.end()).unwrap_or(0);

                // Handle crate:: and lint_arwaky:: imports
                let normalized = if let Some(stripped) = full_import.strip_prefix("lint_arwaky::") {
                    format!("crate::{}", stripped)
                } else {
                    full_import.clone()
                };
                let full_import = &normalized;
                if let Some(path_part) = full_import.strip_prefix("crate::") {
                    let segments: Vec<&str> = path_part.split("::").collect();
                    if segments.len() >= 2 {
                        let mut resolved = false;
                        for i in (1..segments.len()).rev() {
                            let composite = segments[..i].join("/");
                            if let Some(file_path) = module_to_file.get(composite.as_str()) {
                                if file_path != f {
                                    import_graph
                                        .entry(f.clone())
                                        .or_default()
                                        .push(file_path.clone());
                                    inbound_links
                                        .entry(file_path.clone())
                                        .or_default()
                                        .push(f.clone());
                                    resolved = true;
                                    break;
                                }
                            }
                        }
                        if resolved {
                            continue;
                        }
                        for seg in segments[..segments.len() - 1].iter().rev() {
                            if let Some(file_path) = module_to_file.get(*seg) {
                                if file_path != f {
                                    import_graph
                                        .entry(f.clone())
                                        .or_default()
                                        .push(file_path.clone());
                                    inbound_links
                                        .entry(file_path.clone())
                                        .or_default()
                                        .push(f.clone());
                                    resolved = true;
                                    break;
                                }
                            }
                        }
                        if resolved {
                            continue;
                        }
                    }
                    if let Some(seg) = segments.first() {
                        if let Some(file_path) = module_to_file.get(*seg) {
                            if file_path != f {
                                import_graph
                                    .entry(f.clone())
                                    .or_default()
                                    .push(file_path.clone());
                                inbound_links
                                    .entry(file_path.clone())
                                    .or_default()
                                    .push(f.clone());
                                continue;
                            }
                        }
                    }
                    continue;
                }

                if let Some(path_part) = full_import.strip_prefix("super::") {
                    let segments: Vec<&str> = path_part.split("::").collect();
                    if segments.len() >= 2 {
                        let mut found = false;
                        for i in (1..segments.len()).rev() {
                            let composite = segments[..i].join("/");
                            if let Some(file_path) = module_to_file.get(composite.as_str()) {
                                if file_path != f {
                                    import_graph
                                        .entry(f.clone())
                                        .or_default()
                                        .push(file_path.clone());
                                    inbound_links
                                        .entry(file_path.clone())
                                        .or_default()
                                        .push(f.clone());
                                    found = true;
                                    break;
                                }
                            }
                        }
                        if found {
                            continue;
                        }
                        for seg in segments[..segments.len() - 1].iter().rev() {
                            if let Some(resolved) = module_to_file.get(*seg) {
                                if resolved != f {
                                    import_graph
                                        .entry(f.clone())
                                        .or_default()
                                        .push(resolved.clone());
                                    inbound_links
                                        .entry(resolved.clone())
                                        .or_default()
                                        .push(f.clone());
                                    break;
                                }
                            }
                        }
                    } else if let Some(seg) = segments.first() {
                        if let Some(resolved) = module_to_file.get(*seg) {
                            if resolved != f {
                                import_graph
                                    .entry(f.clone())
                                    .or_default()
                                    .push(resolved.clone());
                                inbound_links
                                    .entry(resolved.clone())
                                    .or_default()
                                    .push(f.clone());
                            }
                        }
                    }
                    continue;
                }

                let mut dep = full_import.clone();
                if let Some(dot) = dep.find('.') {
                    dep = dep[..dot].to_string();
                }
                if let Some(colon) = dep.find("::") {
                    dep = dep[..colon].to_string();
                }
                let is_known_local = module_to_file.contains_key(&dep)
                    || (workspace_modules.contains(&dep) && !full_import.contains('.'))
                    || matches!(dep.as_str(), "crate" | "self" | "super");
                if !is_known_local {
                    // Python dotted absolute paths (e.g., modules.cli.src, modules.shared.src.asset)
                    if full_import.contains('.') {
                        // Step 1: Walk dotted path as directories from workspace root
                        // e.g., "modules.cli.src" → <workspace>/modules/cli/src/__init__.py
                        let segments: Vec<&str> = full_import.split('.').collect();
                        let mut walk_dir = std::path::PathBuf::from(&workspace_root);
                        let mut walk_ok = true;
                        for seg in &segments {
                            walk_dir = walk_dir.join(seg);
                            if !shared::orphan_detector::utility_orphan_io::is_dir(&walk_dir) {
                                walk_ok = false;
                                break;
                            }
                        }
                        if walk_ok {
                            // Look for __init__.py or mod.rs in the resolved directory
                            for marker in &["__init__.py", "mod.rs", "index.ts", "index.js"] {
                                let candidate = walk_dir.join(marker);
                                if shared::orphan_detector::utility_orphan_io::is_file(&candidate) {
                                    // Convert absolute path to relative (matching graph convention)
                                    let cand_rel = candidate
                                        .strip_prefix(root_path)
                                        .map(|p| p.to_string_lossy().to_string())
                                        .unwrap_or_else(|_| {
                                            candidate.to_string_lossy().to_string()
                                        });
                                    if cand_rel != *f {
                                        utility_orphan_graph_resolver::add_edge(
                                            &mut import_graph,
                                            &mut inbound_links,
                                            f,
                                            &cand_rel,
                                        );
                                        break;
                                    }
                                }
                            }
                        } else {
                            // Step 2: Try last segment as filename stem
                            if let Some(last_seg) = full_import.rsplit('.').next() {
                                if let Some(target) = module_to_file.get(last_seg) {
                                    if target != f {
                                        utility_orphan_graph_resolver::add_edge(
                                            &mut import_graph,
                                            &mut inbound_links,
                                            f,
                                            target,
                                        );
                                    }
                                }
                            }
                        }

                        // Step 3: Always resolve individual names from `from X import (Y, Z)`
                        // Limit search to current line to avoid matching "import" in comments
                        let line_end = content[cap_end..]
                            .find('\n')
                            .map(|p| cap_end + p)
                            .unwrap_or(content.len());
                        if let Some(import_pos) = content[cap_end..line_end].find("import") {
                            let stmt_start = cap_end + import_pos + 6; // skip "import"
                            let stmt_end = content[stmt_start..]
                                .find('\n')
                                .map(|p| stmt_start + p)
                                .unwrap_or(content.len());
                            let stmt_slice = &content[stmt_start..stmt_end];

                            let names: Vec<&str> = if stmt_slice.contains('(') {
                                // Multi-line: collect from rest of content until ')'
                                let after_paren = &content[stmt_start..];
                                if let Some(close) = after_paren.find(')') {
                                    after_paren[..close]
                                        .split(|c: char| c == ',' || c.is_whitespace())
                                        .map(|s| s.trim())
                                        .filter(|s| {
                                            !s.is_empty()
                                                && s.chars()
                                                    .all(|c| c.is_alphanumeric() || c == '_')
                                        })
                                        .collect()
                                } else {
                                    vec![]
                                }
                            } else {
                                // Single-line: `import Y, Z`
                                stmt_slice
                                    .split(',')
                                    .map(|s| s.trim())
                                    .filter(|s| {
                                        !s.is_empty()
                                            && s.chars().all(|c| c.is_alphanumeric() || c == '_')
                                    })
                                    .collect()
                            };

                            for name in names {
                                // Try stem lookup first
                                if let Some(target) = module_to_file.get(name) {
                                    if target != f {
                                        utility_orphan_graph_resolver::add_edge(
                                            &mut import_graph,
                                            &mut inbound_links,
                                            f,
                                            target,
                                        );
                                        continue;
                                    }
                                }
                                // Try composite dotted path → last segment
                                let full_name_path = format!("{}.{}", full_import, name);
                                for seg in full_name_path.rsplit('.') {
                                    if let Some(target) = module_to_file.get(seg) {
                                        if target != f {
                                            utility_orphan_graph_resolver::add_edge(
                                                &mut import_graph,
                                                &mut inbound_links,
                                                f,
                                                target,
                                            );
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    continue;
                }

                // Workspace crate import resolution using pre-computed crate_src_dirs (Perf 10)
                if let Some(colon_idx) = full_import.find("::") {
                    let crate_name = &full_import[..colon_idx];
                    let rest = &full_import[colon_idx + 2..];
                    let import_list: Vec<String> = if let Some(open_brace) = rest.find('{') {
                        let prefix = &rest[..open_brace];
                        let inner = &rest[open_brace + 1..];
                        let close_brace = inner.rfind('}').unwrap_or(inner.len());
                        let items = inner[..close_brace]
                            .split(',')
                            .map(|s| s.trim())
                            .filter(|s| !s.is_empty());
                        items.map(|item| format!("{}{}", prefix, item)).collect()
                    } else {
                        vec![rest.to_string()]
                    };
                    for import_path in &import_list {
                        let segments: Vec<&str> = import_path.split("::").collect();
                        if segments.is_empty() {
                            continue;
                        }
                        let module_name = segments[0];
                        if let Some(resolved) =
                            utility_orphan_graph_resolver::resolve_workspace_module(
                                &crate_module_index,
                                crate_name,
                                &segments,
                                f,
                            )
                        {
                            utility_orphan_graph_resolver::add_edge(
                                &mut import_graph,
                                &mut inbound_links,
                                f,
                                &resolved,
                            );
                            continue;
                        }
                        if let Some(src_dir) = crate_src_dirs.get(crate_name) {
                            let entries =
                                shared::orphan_detector::utility_orphan_io::scan_directory(src_dir);
                            for (_name, path_str, _is_dir) in entries {
                                let path = std::path::PathBuf::from(&path_str);
                                let stem = path
                                    .file_stem()
                                    .and_then(|s| s.to_str())
                                    .unwrap_or_default();
                                let normalized_stem =
                                    shared::orphan_detector::utility_orphan_detector::normalize_module_component(stem);
                                if (stem == module_name || normalized_stem == module_name)
                                    && path_str != *f
                                {
                                    import_graph
                                        .entry(f.clone())
                                        .or_default()
                                        .push(path_str.to_string());
                                    inbound_links
                                        .entry(path_str.to_string())
                                        .or_default()
                                        .push(f.clone());
                                }
                            }
                        }
                    }
                    continue;
                }

                // Bug 10 fix: Instead of adding edge with stem (which may not be a real file path),
                // resolve dep to the actual file path via module_to_file lookup.
                if let Some(target) = module_to_file.get(&dep) {
                    if target != f {
                        import_graph.entry(f.clone()).or_default().push(target.clone());
                        inbound_links.entry(target.clone()).or_default().push(f.clone());
                    }
                } else if workspace_modules.contains(&dep) && !full_import.contains('.') {
                    // dep is a workspace crate name — try to find its entry point
                    for (dir_name, src_dir) in &crate_src_dirs {
                        if dir_name == &dep || dir_name.replace('-', "_") == dep {
                            for entry_file in &["lib.rs", "__init__.py", "main.rs", "main.py", "index.ts", "index.js"] {
                                let entry_path = src_dir.join(entry_file);
                                if shared::orphan_detector::utility_orphan_io::is_file(&entry_path) {
                                    let rel = entry_path
                                        .strip_prefix(root_path)
                                        .map(|p| p.to_string_lossy().to_string())
                                        .unwrap_or_else(|_| entry_path.to_string_lossy().to_string());
                                    if rel != *f {
                                        import_graph.entry(f.clone()).or_default().push(rel.clone());
                                        inbound_links.entry(rel).or_default().push(f.clone());
                                    }
                                    break;
                                }
                            }
                            break;
                        }
                    }
                }
            }

            // Pass 3b: Python relative imports (`from . import X`, `from .module import X`)
            if let Some(rel_re) = utility_orphan_regex_patterns::python_relative_import_re() {
                for cap in rel_re.captures_iter(&content) {
                    // Group 1 = dots (parenthesized variant), Group 3 = dots (inline variant)
                    let dots = cap.get(1).or_else(|| cap.get(3));
                    // Group 2 = names (parenthesized variant), Group 4 = names (inline variant)
                    let names_raw = cap.get(2).or_else(|| cap.get(4));
                    let (Some(dots_m), Some(names_m)) = (dots, names_raw) else {
                        continue;
                    };
                    let dot_count = dots_m.as_str().len(); // 1=., 2=.., 3=...
                    let names_str = names_m.as_str();

                    // Resolve base directory: . = current file dir, .. = parent, ... = grandparent
                    let file_path = std::path::Path::new(f);
                    let mut base_dir = file_path.parent().map(|p| p.to_path_buf());
                    for _ in 1..dot_count {
                        if let Some(ref dir) = base_dir {
                            base_dir = dir.parent().map(|p| p.to_path_buf());
                        }
                    }
                    let Some(base) = base_dir else { continue };

                    // Parse imported names
                    let names: Vec<&str> = names_str
                        .split(|c: char| c == ',' || c == '(' || c == ')' || c.is_whitespace())
                        .map(|s| s.trim())
                        .filter(|s| {
                            !s.is_empty() && s.chars().all(|c| c.is_alphanumeric() || c == '_')
                        })
                        .collect();

                    for name in names {
                        // Try module_to_file lookup (stem match)
                        if let Some(target) = module_to_file.get(name) {
                            if target != f {
                                utility_orphan_graph_resolver::add_edge(
                                    &mut import_graph,
                                    &mut inbound_links,
                                    f,
                                    target,
                                );
                                continue;
                            }
                        }
                        // Try resolving as file in base directory
                        for ext in &[".py", ".rs", ".ts", ".js"] {
                            let candidate = base.join(format!("{}{}", name, ext));
                            if shared::orphan_detector::utility_orphan_io::is_file(&candidate) {
                                // Normalize to relative path for consistent comparison with f
                                let cand_rel = candidate
                                    .strip_prefix(root_path)
                                    .map(|p| p.to_string_lossy().to_string())
                                    .unwrap_or_else(|_| candidate.to_string_lossy().to_string());
                                if cand_rel != *f {
                                    utility_orphan_graph_resolver::add_edge(
                                        &mut import_graph,
                                        &mut inbound_links,
                                        f,
                                        &cand_rel,
                                    );
                                    break;
                                }
                            }
                        }
                    }
                }
            }

            // Pass 3c: TypeScript/JavaScript imports
            if f.ends_with(".ts")
                || f.ends_with(".js")
                || f.ends_with(".tsx")
                || f.ends_with(".jsx")
            {
                if let Some(ts_re) = utility_orphan_regex_patterns::ts_import_re() {
                    for cap in ts_re.captures_iter(&content) {
                        // Group 2 = named path, 5 = default+named path, 7 = default path,
                        // 9 = namespace path, 11 = side-effect path
                        let import_path = cap
                            .get(2)
                            .or_else(|| cap.get(5))
                            .or_else(|| cap.get(7))
                            .or_else(|| cap.get(9))
                            .or_else(|| cap.get(11));
                        if let Some(path_m) = import_path {
                            let raw = path_m.as_str();
                            if raw.starts_with('.') {
                                if let Some(resolved) =
                                    utility_orphan_graph_resolver::resolve_ts_relative(
                                        f, raw, root_path,
                                    )
                                {
                                    utility_orphan_graph_resolver::add_edge(
                                        &mut import_graph,
                                        &mut inbound_links,
                                        f,
                                        &resolved,
                                    );
                                }
                            }
                        }
                    }
                }
                if let Some(ts_re) = utility_orphan_regex_patterns::ts_export_re() {
                    for cap in ts_re.captures_iter(&content) {
                        // Group 2 = named export path, 3 = wildcard export path
                        let export_path = cap.get(2).or_else(|| cap.get(3));
                        if let Some(path_m) = export_path {
                            let raw = path_m.as_str();
                            if raw.starts_with('.') {
                                if let Some(resolved) =
                                    utility_orphan_graph_resolver::resolve_ts_relative(
                                        f, raw, root_path,
                                    )
                                {
                                    utility_orphan_graph_resolver::add_edge(
                                        &mut import_graph,
                                        &mut inbound_links,
                                        f,
                                        &resolved,
                                    );
                                }
                            }
                        }
                    }
                }
            }

            // Bug 12 fix: Only run class inheritance detection for Python files (.py)
            // prevents false matches in Rust/JS/TS files
            if f.ends_with(".py") {
                // Pass 4: Python class inheritance
                if let Some(re) = utility_orphan_regex_patterns::inh_re() {
                    for cap in re.captures_iter(&content) {
                        for base in cap[1].split(',') {
                            inheritance_map
                                .entry(f.clone())
                                .or_default()
                                .push(base.trim().to_string());
                        }
                    }
                }
            }

            // Pass 5: pub use re-exports (e.g. `pub use crate::common::taxonomy_action_vo;`)
            if let Some(re) = utility_orphan_regex_patterns::pub_use_re() {
                for cap in re.captures_iter(&content) {
                    let path = &cap[1]; // e.g. common::taxonomy_action_vo
                    let segments: Vec<&str> = path.split("::").collect();

                    // Try to resolve the re-exported module to a file
                    // First try the last segment (file stem)
                    if let Some(seg) = segments.last() {
                        if let Some(file_path) = module_to_file.get(*seg) {
                            if file_path != f {
                                utility_orphan_graph_resolver::add_edge(
                                    &mut import_graph,
                                    &mut inbound_links,
                                    f,
                                    file_path,
                                );
                            }
                        }
                    }
                    // Also try composite path with / separator
                    if segments.len() >= 2 {
                        let composite = segments.join("/");
                        if let Some(file_path) = module_to_file.get(composite.as_str()) {
                            if file_path != f {
                                utility_orphan_graph_resolver::add_edge(
                                    &mut import_graph,
                                    &mut inbound_links,
                                    f,
                                    file_path,
                                );
                            }
                        }
                    }
                }
            }

            // Pass 5b: pub use relative re-exports (e.g. `pub use taxonomy_language_vo::LanguageVO;`)
            if let Some(re) = utility_orphan_regex_patterns::pub_use_relative_re() {
                for cap in re.captures_iter(&content) {
                    let path = &cap[1]; // e.g. taxonomy_language_vo::LanguageVO
                                        // Skip prefixes already handled by pub_use_re (Pass 5)
                    if path.starts_with("crate::")
                        || path.starts_with("super::")
                        || path.starts_with("self::")
                        || path == "crate"
                        || path == "super"
                        || path == "self"
                    {
                        continue;
                    }
                    let segments: Vec<&str> = path.split("::").collect();

                    // Try to resolve the re-exported module to a file
                    if let Some(seg) = segments.first() {
                        if let Some(file_path) = module_to_file.get(*seg) {
                            if file_path != f {
                                utility_orphan_graph_resolver::add_edge(
                                    &mut import_graph,
                                    &mut inbound_links,
                                    f,
                                    file_path,
                                );
                            }
                        }
                    }
                }
            }
        }

        // Bug 9 fix: Deduplicate edges to prevent inflating inbound link count
        utility_orphan_graph_resolver::dedup_edges(&mut import_graph);
        utility_orphan_graph_resolver::dedup_edges(&mut inbound_links);

        GraphAnalysisContext::new(
            ImportGraph::new(import_graph),
            InboundLinkMap::new(inbound_links),
            InheritanceMap::new(inheritance_map),
            FileDefinitionMap::new(file_definitions),
        )
    }
}
