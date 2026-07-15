// PURPOSE: OrphanGraphResolver — build graph context and identify entry points for orphan analysis.
use crate::taxonomy_orphan_filename_helper::file_stem;
use regex::Regex;
use shared::code_analysis::taxonomy_analysis_vo::FileDefinitionMap;
use shared::code_analysis::taxonomy_analysis_vo::GraphAnalysisContext;
use shared::code_analysis::taxonomy_analysis_vo::ImportGraph;
use shared::code_analysis::taxonomy_analysis_vo::InboundLinkMap;
use shared::code_analysis::taxonomy_analysis_vo::InheritanceMap;
use shared::orphan_detector::contract_orphan_graph_resolver_protocol::IOrphanGraphResolverProtocol;
use shared::orphan_detector::taxonomy_orphan_contract_vo::{
    OrphanEntryPatternListVO, OrphanFileListVO,
};
use std::collections::HashMap;
use std::sync::OnceLock;

/// Build graph context and identify entry points for orphan analysis.
pub struct OrphanGraphResolver {}

impl Default for OrphanGraphResolver {
    fn default() -> Self {
        Self::new()
    }
}

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
                            || crate::taxonomy_orphan_filename_helper::file_stem(basename)
                                .contains(pattern)
                    })
                })
                .cloned()
                .collect()
        };
        OrphanFileListVO::new(matched)
    }
}

/// Cached regexes (Perf 1): compiled once via OnceLock.
static PUB_MOD_PATH_RE: OnceLock<Option<Regex>> = OnceLock::new();
static PLAIN_MOD_RE: OnceLock<Option<Regex>> = OnceLock::new();
static IMPORT_RE: OnceLock<Option<Regex>> = OnceLock::new();
static INH_RE: OnceLock<Option<Regex>> = OnceLock::new();

fn pub_mod_path_re() -> Option<&'static Regex> {
    PUB_MOD_PATH_RE
        .get_or_init(|| {
            Regex::new(r#"#\[path\s*=\s*"([^"]+)"\]\s*(?:pub\s+)?mod\s+([a-zA-Z_]+)"#).ok()
        })
        .as_ref()
}

fn plain_mod_re() -> Option<&'static Regex> {
    PLAIN_MOD_RE
        .get_or_init(|| Regex::new(r"(?:pub\s+)?mod\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*;").ok())
        .as_ref()
}

fn import_re() -> Option<&'static Regex> {
    IMPORT_RE
        .get_or_init(|| Regex::new(r"(?:use|import|from)\s+([a-zA-Z_][a-zA-Z0-9_\.:]*)").ok())
        .as_ref()
}

fn inh_re() -> Option<&'static Regex> {
    INH_RE
        .get_or_init(|| Regex::new(r"class\s+\w+\(([^)]+)\)").ok())
        .as_ref()
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

        // Build a lookup: module_name -> file_path for crate:: resolution
        let mut module_to_file: HashMap<String, String> = HashMap::new();
        for f in files {
            let stem = file_stem(f);
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
            if let Ok(entries) = std::fs::read_dir(&ws_path) {
                for entry in entries.flatten() {
                    if let Ok(ft) = entry.file_type() {
                        if !ft.is_dir() {
                            continue;
                        }
                    } else {
                        continue;
                    }
                    if let Some(name) = entry.file_name().to_str().map(|s| s.to_string()) {
                        workspace_modules.insert(name.clone());
                        workspace_modules.insert(name.replace('-', "_"));
                        let src_dir = entry.path().join("src");
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
            let content = match std::fs::read_to_string(f) {
                Ok(c) => c,
                Err(_) => continue,
            };

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
                let full_import = cap[1].to_string();

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
                        let module_name = segments[0];
                        // Bug 3: no ./ prefix — store resolved paths verbatim
                        if let Some(src_dir) = crate_src_dirs.get(crate_name) {
                            if let Ok(entries) = std::fs::read_dir(src_dir) {
                                for entry in entries.flatten() {
                                    let path = entry.path();
                                    if let Some(path_str) = path.to_str() {
                                        let stem = path
                                            .file_stem()
                                            .and_then(|s| s.to_str())
                                            .unwrap_or_default();
                                        if stem == module_name && path_str != *f {
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
                        }
                        continue;
                    }
                }

                // Python/JS relative imports
                import_graph.entry(f.clone()).or_default().push(dep.clone());
                inbound_links.entry(dep).or_default().push(f.clone());
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
