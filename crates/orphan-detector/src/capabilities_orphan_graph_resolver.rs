// PURPOSE: OrphanGraphResolver — build graph context and identify entry points for orphan analysis.
use shared::code_analysis::taxonomy_analysis_vo::FileDefinitionMap;
use shared::code_analysis::taxonomy_analysis_vo::GraphAnalysisContext;
use shared::code_analysis::taxonomy_analysis_vo::ImportGraph;
use shared::code_analysis::taxonomy_analysis_vo::InboundLinkMap;
use shared::code_analysis::taxonomy_analysis_vo::InheritanceMap;
use shared::orphan_detector::contract_orphan_graph_resolver_protocol::IOrphanGraphResolverProtocol;
use shared::orphan_detector::contract_orphan_protocol::{
    IOrphanFileCachePort, IOrphanFilenameExtractorProtocol,
};
use shared::orphan_detector::taxonomy_orphan_contract_vo::{
    OrphanEntryPatternListVO, OrphanFileListVO,
};
use std::collections::HashMap;
use std::sync::Arc;

// ═══════════════════════════════════════════════════════════════════════════════
// IMPORTS FROM TAXONOMY UTILITY
// ═══════════════════════════════════════════════════════════════════════════════

use shared::orphan_detector::taxonomy_graph_regex_utility::{
    import_re, inh_re, plain_mod_re, pub_mod_path_re,
};

/// Build graph context and identify entry points for orphan analysis.
// ─── Block 1: Struct Definition ───────────────────────────
pub struct OrphanGraphResolver {
    extractor: Arc<dyn IOrphanFilenameExtractorProtocol>,
    cache: Arc<dyn IOrphanFileCachePort>,
}

// ─── Block 2: Public Contract (domain protocol ONLY) ──────
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

        let matched: Vec<String> = if configured_strs.is_empty() {
            file_strs
                .iter()
                .filter(|f| {
                    let basename = f.rsplit('/').next().unwrap_or(f);
                    basename.ends_with("_entry.rs")
                        || basename.ends_with("_entry.py")
                        || basename.ends_with("_entry.ts")
                        || basename.ends_with("_entry.js")
                        || basename.starts_with("root_")
                        || basename == "main.rs"
                        || basename == "lib.rs"
                        || basename == "main.py"
                        || basename == "__main__.py"
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
                    configured_strs.iter().any(|pattern| {
                        basename == pattern
                            || basename.ends_with(pattern)
                            || self
                                .extractor
                                .file_stem(&shared::common::taxonomy_path_vo::FilePath {
                                    value: basename.to_string(),
                                })
                                .value
                                .contains(pattern)
                    })
                })
                .cloned()
                .collect()
        };
        OrphanFileListVO::new(matched)
    }
}

// ─── Block 3: Constructors, Std Traits & Helpers ─────────
impl OrphanGraphResolver {
    pub fn new(
        extractor: Arc<dyn IOrphanFilenameExtractorProtocol>,
        cache: Arc<dyn IOrphanFileCachePort>,
    ) -> Self {
        Self { extractor, cache }
    }

    /// Resolve a multi-segment external-crate import path to the real leaf
    /// module file, so inbound-links are attached to the actual module and not
    /// just its domain `mod.rs` (which previously caused false AES501/AES502
    /// orphans for cross-crate-consumed foundation modules).
    ///
    /// e.g. `shared::orphan_detector::taxonomy_orphan_result_utility`
    ///   -> `crates/shared/src/orphan-detector/taxonomy_orphan_result_utility.rs`
    ///      (or `.../orphan-detector/taxonomy_orphan_result_utility/mod.rs`)
    fn resolve_module_file(
        &self,
        _crate_name: &str,
        segments: &[&str],
        src_dir: &std::path::Path,
        importing_file: &str,
    ) -> Option<String> {
        // Walk the crate src dir following each import segment as a path
        // component (normalize `-` -> `_` to match crate dir naming).
        let mut current = src_dir.to_path_buf();
        for seg in segments {
            let normalized = seg.replace('-', "_");
            let candidate_file = current.join(format!("{}.rs", normalized));
            let candidate_mod = current.join(&normalized).join("mod.rs");
            if candidate_file.is_file() {
                let p = candidate_file.to_string_lossy().to_string();
                return if p != importing_file { Some(p) } else { None };
            } else if candidate_mod.is_file() {
                // Segment is a directory module; descend for the next segment.
                current = current.join(&normalized);
                continue;
            } else {
                // No such file/dir at this level — cannot resolve deeper.
                return None;
            }
        }
        // If we consumed all segments inside directory modules, the final
        // directory's mod.rs is the resolved module file.
        let final_mod = current.join("mod.rs");
        if final_mod.is_file() {
            let p = final_mod.to_string_lossy().to_string();
            return if p != importing_file { Some(p) } else { None };
        }
        None
    }

    fn build_graph_context_inner(&self, files: &[String], root_dir: &str) -> GraphAnalysisContext {
        let mut import_graph: HashMap<String, Vec<String>> = HashMap::new();
        let mut inbound_links: HashMap<String, Vec<String>> = HashMap::new();
        let mut inheritance_map: HashMap<String, Vec<String>> = HashMap::new();
        let file_definitions: HashMap<String, Vec<String>> = HashMap::new();

        // Build a lookup: module_name -> file_path for crate:: resolution
        let mut module_to_file: HashMap<String, String> = HashMap::new();
        for f in files {
            let stem = self
                .extractor
                .file_stem(&shared::common::taxonomy_path_vo::FilePath { value: f.clone() })
                .value;
            module_to_file.insert(stem.clone(), f.clone());
            if let Some(parent) = f.rsplit('/').nth(1) {
                let module_path = format!("{}/{}", parent, stem);
                module_to_file.insert(module_path.clone(), f.clone());
                let normalized_path = module_path.replace('-', "_");
                if normalized_path != module_path {
                    module_to_file.insert(normalized_path, f.clone());
                }
            }
            // Bug 13: mod.rs -> map by parent directory name
            if stem == "mod" {
                if let Some(parent_dir) = f.rsplit('/').nth(1) {
                    module_to_file.insert(parent_dir.to_string(), f.clone());
                    let normalized = parent_dir.replace('-', "_");
                    if normalized != parent_dir {
                        module_to_file.insert(normalized, f.clone());
                    }
                    if let Some(grandparent) = f.rsplit('/').nth(2) {
                        let composite = format!("{}/{}", grandparent, parent_dir);
                        module_to_file.insert(composite.clone(), f.clone());
                        let normalized_composite = composite.replace('-', "_");
                        if normalized_composite != composite {
                            module_to_file.insert(normalized_composite, f.clone());
                        }
                    }
                }
            }
        }

        // Build set of known workspace crate dirs for external dep detection
        let mut workspace_modules: std::collections::HashSet<String> =
            std::collections::HashSet::new();
        // Perf 10: Pre-compute crate_name -> src_dir map
        let mut crate_src_dirs: HashMap<String, std::path::PathBuf> = HashMap::new();
        let root_path = std::path::Path::new(root_dir);
        for ws_dir in &["crates", "packages", "modules"] {
            let ws_path = root_path.join(ws_dir);
            let entries = self.cache.read_dir(ws_path.to_str().unwrap_or(""));
            for entry_path in &entries {
                let path = std::path::Path::new(entry_path);
                if path.is_dir() {
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        let name = name.to_string();
                        workspace_modules.insert(name.clone());
                        workspace_modules.insert(name.replace('-', "_"));
                        let src_dir = path.join("src");
                        if src_dir.is_dir() {
                            crate_src_dirs.insert(name.clone(), src_dir.clone());
                            crate_src_dirs.insert(name.replace('-', "_"), src_dir);
                        }
                    }
                }
            }
        }

        // Perf 8: Single-pass file reading
        for f in files {
            import_graph.entry(f.clone()).or_default();
            let fp = shared::common::taxonomy_path_vo::FilePath { value: f.clone() };
            let content = self.cache.read_cached(&fp).value;
            if content.is_empty() {
                continue;
            }

            // Pass 1: #[path = "..."] pub mod (Bug 14 fix — link only the referenced file)
            if let Some(re) = pub_mod_path_re() {
                for cap in re.captures_iter(&content) {
                    let mod_path = cap[1].to_string();
                    let base_dir = match std::path::Path::new(f).parent() {
                        Some(p) => p.to_string_lossy().to_string(),
                        None => String::from("."),
                    };
                    let resolved = if mod_path.starts_with('/') {
                        mod_path.clone()
                    } else {
                        format!("{}/{}", base_dir, mod_path)
                    };
                    if std::path::Path::new(&resolved).exists() && resolved != *f {
                        import_graph
                            .entry(f.clone())
                            .or_default()
                            .push(resolved.clone());
                        inbound_links.entry(resolved).or_default().push(f.clone());
                    }
                }
            }

            // Pass 2: plain mod (Bug 10 fix)
            if let Some(re) = plain_mod_re() {
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
                        if candidate.is_file() {
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
            let Some(import_re) = import_re() else {
                continue;
            };
            for cap in import_re.captures_iter(&content) {
                let full_import = cap
                    .get(1)
                    .or_else(|| cap.get(2))
                    .or_else(|| cap.get(3))
                    .map(|m| m.as_str().to_string())
                    .unwrap_or_default();
                if full_import.is_empty() {
                    continue;
                }

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

                // Python/JS-style absolute dot-separated imports
                // e.g. from modules.shared.src.common.taxonomy_common_vo import X
                // Convert dots to path separators and resolve against root_dir
                if full_import.contains('.') && !full_import.contains("::") {
                    let path_from_dots = full_import.replace('.', "/");
                    let mut resolved_abs = false;
                    for ext in &[".py", ".ts", ".js", ".rs"] {
                        let candidate = root_path.join(format!("{}{}", path_from_dots, ext));
                        if let Some(candidate_str) = candidate.to_str() {
                            if std::path::Path::new(candidate_str).exists() && candidate_str != *f {
                                import_graph
                                    .entry(f.clone())
                                    .or_default()
                                    .push(candidate_str.to_string());
                                inbound_links
                                    .entry(candidate_str.to_string())
                                    .or_default()
                                    .push(f.clone());
                                resolved_abs = true;
                                break;
                            }
                        }
                    }
                    if !resolved_abs {
                        let init_candidate =
                            root_path.join(format!("{}/__init__.py", path_from_dots));
                        if let Some(init_str) = init_candidate.to_str() {
                            if std::path::Path::new(init_str).exists() && init_str != *f {
                                import_graph
                                    .entry(f.clone())
                                    .or_default()
                                    .push(init_str.to_string());
                                inbound_links
                                    .entry(init_str.to_string())
                                    .or_default()
                                    .push(f.clone());
                                resolved_abs = true;
                            }
                        }
                    }
                    if resolved_abs {
                        continue;
                    }
                }

                let mut dep = full_import.clone();
                if let Some(dot) = dep.find('.') {
                    dep = dep[..dot].to_string();
                }
                if let Some(colon) = dep.find("::") {
                    dep = dep[..colon].to_string();
                }
                let is_known_local = module_to_file.contains_key(&dep)
                    || workspace_modules.contains(&dep)
                    || matches!(dep.as_str(), "crate" | "self" | "super");
                if !is_known_local {
                    continue;
                }

                // Workspace crate import resolution using pre-computed crate_src_dirs (Perf 10)
                if let Some(colon_idx) = full_import.find("::") {
                    let crate_name = &full_import[..colon_idx];
                    let rest = &full_import[colon_idx + 2..];
                    let segments: Vec<&str> = rest.split("::").collect();
                    if !segments.is_empty() {
                        // Bug 3: no ./ prefix — store resolved paths verbatim
                        if let Some(src_dir) = crate_src_dirs.get(crate_name) {
                            // Resolve the FULL import path (all segments) to the real
                            // leaf module file, not just the top-level domain directory.
                            // e.g. `shared::orphan_detector::taxonomy_orphan_result_utility`
                            // must link to
                            // `crates/shared/src/orphan-detector/taxonomy_orphan_result_utility.rs`,
                            // NOT only `crates/shared/src/orphan-detector/mod.rs`.
                            // Previously only the leading segment was resolved, so leaf
                            // taxonomy/contract module files appeared to have zero inbound
                            // links and were falsely flagged as AES501/AES502 orphans.
                            if let Some(resolved) =
                                self.resolve_module_file(crate_name, &segments, src_dir, f)
                            {
                                import_graph
                                    .entry(f.clone())
                                    .or_default()
                                    .push(resolved.clone());
                                inbound_links.entry(resolved).or_default().push(f.clone());
                            }
                        }
                        continue;
                    }
                }

                // Python/JS relative imports — only add if resolved to a file path
                if let Some(resolved_path) = module_to_file.get(&dep) {
                    if resolved_path != f {
                        import_graph
                            .entry(f.clone())
                            .or_default()
                            .push(resolved_path.clone());
                        inbound_links
                            .entry(resolved_path.clone())
                            .or_default()
                            .push(f.clone());
                    }
                }
                // If it's a workspace module (e.g., "shared") but not resolved to a specific file,
                // we do not add it to the import graph to avoid polluting it with directory names.
            }

            // Pass 4: Python class inheritance
            if let Some(re) = inh_re() {
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
        GraphAnalysisContext::new(
            ImportGraph::new(import_graph),
            InboundLinkMap::new(inbound_links),
            InheritanceMap::new(inheritance_map),
            FileDefinitionMap::new(file_definitions),
        )
    }
}
