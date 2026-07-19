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
use std::path::{Path, PathBuf};
use std::sync::Arc;

use regex::Regex;
use std::sync::OnceLock;

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

    pub fn pub_mod_path_re() -> Option<&'static Regex> {
        static RE: OnceLock<Option<Regex>> = OnceLock::new();
        RE.get_or_init(|| Regex::new(r#"\[path\s*=\s*"([^"]+)"\]\s*pub\s+mod"#).ok())
            .as_ref()
    }

    pub fn plain_mod_re() -> Option<&'static Regex> {
        static RE: OnceLock<Option<Regex>> = OnceLock::new();
        RE.get_or_init(|| Regex::new(r"^\s*mod\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*;").ok())
            .as_ref()
    }

    pub fn import_re() -> Option<&'static Regex> {
        static RE: OnceLock<Option<Regex>> = OnceLock::new();
        RE.get_or_init(|| {
            Regex::new(
                r"(?:use\s+([a-zA-Z_][a-zA-Z0-9_:]*)|from\s+([a-zA-Z_.]+)\s+import|import\s+([a-zA-Z_][a-zA-Z0-9_.]*))",
            )
            .ok()
        })
        .as_ref()
    }

    pub fn inh_re() -> Option<&'static Regex> {
        static RE: OnceLock<Option<Regex>> = OnceLock::new();
        RE.get_or_init(|| Regex::new(r"class\s+\w+\(([^)]+)\)").ok())
            .as_ref()
    }

    /// Given the directory module currently being descended into (`mod_dir`) and
    /// a segment that did not resolve to a real file/dir, check that module's
    /// `mod.rs` for a `pub use ...<segment>` re-export and return the segments of
    /// the re-exported path (e.g. `crate::common::taxonomy_action_vo` ->
    /// `["common", "taxonomy_action_vo"]`). Returns `None` if no matching
    /// re-export is found.
    fn resolve_reexport(&self, mod_dir: &std::path::Path, seg: &str) -> Option<Vec<String>> {
        let mod_rs = mod_dir.join("mod.rs");
        let content = std::fs::read_to_string(&mod_rs).ok()?;
        // Match `pub use crate::...::seg;` or `pub use crate::seg;`
        // (strip a trailing `::*` / `::{...}` / item path).
        let re = regex::Regex::new(
            r"pub use crate::((?:[A-Za-z_][A-Za-z0-9_]*)(?:::[A-Za-z_][A-Za-z0-9_]*)*)",
        )
        .ok()?;
        let seg_norm = seg.replace('-', "_");
        for cap in re.captures_iter(&content) {
            let path = cap.get(1)?.as_str();
            // The re-exported path ends with the segment we are looking for
            // (e.g. `common::taxonomy_action_vo` ends with `taxonomy_action_vo`).
            let leaf = path.rsplit("::").next().unwrap_or(path);
            if leaf.replace('-', "_") == seg_norm {
                return Some(path.split("::").map(|s| s.to_string()).collect());
            }
        }
        None
    }

    /// Resolve a multi-segment external-crate import path to the real leaf
    /// module file, so inbound-links are attached to the actual module and not
    /// just its domain `mod.rs` (which previously caused false AES501/AES502
    /// orphans for cross-crate-consumed foundation modules).
    ///
    /// Imports may name a sub-item, e.g.
    /// `shared::orphan_detector::taxonomy_orphan_result_utility::mk_orphan_result`
    /// — the trailing `mk_orphan_result` is a function, not a module. We walk the
    /// segments, descending through directory modules, and stop at the deepest
    /// segment that actually resolves to a file or directory module. That last
    /// resolved module is the one that should receive the inbound link.
    /// e.g. `shared::orphan_detector::taxonomy_orphan_result_utility`
    ///   -> `crates/shared/src/orphan-detector/taxonomy_orphan_result_utility.rs`
    ///
    /// When a segment cannot be resolved directly it may be a re-export: the
    /// current module's `mod.rs` can contain `pub use crate::common::X;`. In that
    /// case we follow the re-export to `common::X` and resume resolution.
    ///
    /// Returns the resolved `.rs` file path, or `None` if it cannot be found.
    fn resolve_module_file(
        &self,
        _crate_name: &str,
        segments: &[&str],
        src_dir: &std::path::Path,
        importing_file: &str,
    ) -> Option<String> {
        let mut current = src_dir.to_path_buf();
        // Deepest module file resolved so far (returned if a deeper segment is
        // a non-module item like a function/type).
        //
        // NOTE: candidate paths are built with `format!` + `Path::new(&str)`
        // rather than `Path::join(&String)`. `Path::join` with a `&String`
        // argument does not reliably resolve to `AsRef<Path>` and silently
        // produced non-existent candidate paths, which caused leaf modules to
        // never be linked and produced false AES501/AES502 orphans.
        let mut last_resolved: Option<String> = None;
        for (seg_i, seg) in segments.iter().enumerate() {
            // Rust module names use underscores, but the repo's directories use
            // hyphens (e.g. `code_analysis` module lives in `code-analysis/`), and
            // vice versa (`orphan_detector` import -> `orphan-detector/` dir). Try
            // BOTH the underscore and hyphen spellings of every segment against the
            // real filesystem so leaf modules are linked correctly.
            let normalized = seg.replace('-', "_");
            let hyphenated = seg.replace('_', "-");
            let mut file_variants = vec![format!("{}/{}.rs", current.display(), normalized)];
            let mut mod_variants = vec![format!("{}/{}/mod.rs", current.display(), normalized)];
            if hyphenated != normalized {
                file_variants.push(format!("{}/{}.rs", current.display(), hyphenated));
                mod_variants.push(format!("{}/{}/mod.rs", current.display(), hyphenated));
            }
            if seg != &normalized && seg != &hyphenated {
                file_variants.push(format!("{}/{}.rs", current.display(), seg));
                mod_variants.push(format!("{}/{}/mod.rs", current.display(), seg));
            }
            let mut matched: Option<String> = None;
            for cand in file_variants.iter().chain(mod_variants.iter()) {
                if Path::new(cand).is_file() {
                    matched = Some(cand.clone());
                    break;
                }
            }
            if let Some(resolved) = matched {
                if resolved != importing_file {
                    last_resolved = Some(resolved.clone());
                }
                last_resolved = last_resolved.or(Some(resolved.clone()));
                current = Path::new(&resolved)
                    .parent()
                    .map(PathBuf::from)
                    .unwrap_or(current);
                continue;
            } else {
                // The segment did not resolve to a real file/directory module.
                // It may be reachable through a re-export: e.g.
                // `shared::mcp_server::taxonomy_action_vo` where `mcp-server/mod.rs`
                // contains `pub use crate::common::taxonomy_action_vo;`. Follow
                // `pub use` re-exports and resume resolution from the re-exported
                // module's real location.
                if let Some(reexport_segments) = self.resolve_reexport(&current, seg) {
                    // Recurse with the re-exported module path, then continue the
                    // remaining original segments (the sub-item, if any).
                    let mut full: Vec<String> = reexport_segments;
                    for s in &segments[seg_i + 1..] {
                        full.push(s.to_string());
                    }
                    let full_refs: Vec<&str> = full.iter().map(|s| s.as_str()).collect();
                    if let Some(resolved) =
                        self.resolve_module_file(_crate_name, &full_refs, src_dir, importing_file)
                    {
                        return if resolved != importing_file {
                            Some(resolved)
                        } else {
                            last_resolved
                        };
                    }
                }
                return last_resolved;
            }
        }
        let final_mod = format!("{}/mod.rs", current.display());
        if Path::new(&final_mod).is_file() {
            return if final_mod != importing_file {
                Some(final_mod)
            } else {
                last_resolved
            };
        }
        last_resolved
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
            if let Ok(fp) =
                shared::common::taxonomy_path_vo::FilePath::new(ws_path.to_str().unwrap_or(""))
            {
                let entries = self.cache.read_dir(&fp);
                for entry_path in &entries {
                    let path = std::path::Path::new(entry_path.value());
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
            if let Some(re) = Self::pub_mod_path_re() {
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
            if let Some(re) = Self::plain_mod_re() {
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
            let Some(import_re) = Self::import_re() else {
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
                                    break;
                                }
                            }
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
                        // Resolve the FULL import path (all segments) to the real
                        // leaf module file, not just the top-level domain directory.
                        // e.g. `shared::orphan_detector::taxonomy_orphan_result_utility`
                        // must link to
                        // `crates/shared/src/orphan-detector/taxonomy_orphan_result_utility.rs`,
                        // NOT only `crates/shared/src/orphan-detector/mod.rs`.
                        // Previously only the leading segment was resolved, so leaf
                        if let Some(src_dir) = crate_src_dirs.get(crate_name) {
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
            if let Some(re) = Self::inh_re() {
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
