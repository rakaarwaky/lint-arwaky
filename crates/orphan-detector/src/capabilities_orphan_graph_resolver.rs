// PURPOSE: OrphanGraphResolver — build graph context and identify entry points for orphan analysis.
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
use shared::orphan_detector::utility_orphan_filename::file_stem;
use shared::orphan_detector::utility_orphan_io;
use std::collections::HashMap;
use std::sync::OnceLock;

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

        let matched: Vec<String> = if configured_strs.is_empty() {
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
                    configured_strs.iter().any(|pattern| {
                        basename == pattern
                            || basename.ends_with(pattern)
                            || shared::orphan_detector::utility_orphan_filename::file_stem(basename)
                                .contains(pattern)
                    })
                })
                .cloned()
                .collect()
        };
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

    /// Cached regexes (Perf 1): compiled once via OnceLock.
    fn pub_mod_path_re() -> Option<&'static Regex> {
        static RE: OnceLock<Option<Regex>> = OnceLock::new();
        RE.get_or_init(|| {
            Regex::new(r#"#\[path\s*=\s*"([^"]+)"\]\s*(?:pub\s+)?mod\s+([a-zA-Z_]+)"#).ok()
        })
        .as_ref()
    }

    fn plain_mod_re() -> Option<&'static Regex> {
        static RE: OnceLock<Option<Regex>> = OnceLock::new();
        RE.get_or_init(|| Regex::new(r"(?:pub\s+)?mod\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*;").ok())
            .as_ref()
    }

    fn import_re() -> Option<&'static Regex> {
        static RE: OnceLock<Option<Regex>> = OnceLock::new();
        RE.get_or_init(|| {
            Regex::new(r"(?:use|import|from)\s+([a-zA-Z_][a-zA-Z0-9_\.:]*(?:\{[^}]*\})?)").ok()
        })
        .as_ref()
    }

    fn inh_re() -> Option<&'static Regex> {
        static RE: OnceLock<Option<Regex>> = OnceLock::new();
        RE.get_or_init(|| Regex::new(r"class\s+\w+\(([^)]+)\)").ok())
            .as_ref()
    }

    /// Regex for TypeScript/JavaScript relative imports: `import { X } from './path'` or `from './path'`
    /// Captures the quoted module path (single quotes, double quotes, or backticks).
    fn ts_import_path_re() -> Option<&'static Regex> {
        static RE: OnceLock<Option<Regex>> = OnceLock::new();
        RE.get_or_init(|| {
            Regex::new(r#"(?:from|import)\s+.*?(['\"`])([^'\"`]+)\1"#).ok()
        })
        .as_ref()
    }

    /// Regex for CommonJS require: `require('./path')`
    fn ts_require_path_re() -> Option<&'static Regex> {
        static RE: OnceLock<Option<Regex>> = OnceLock::new();
        RE.get_or_init(|| {
            Regex::new(r#"require\((['\"`])([^'\"`]+)\1\)"#).ok()
        })
        .as_ref()
    }

    /// Regex for `pub use crate::module;` module re-exports (not type re-exports like `::Type`)
    fn pub_use_re() -> Option<&'static Regex> {
        static RE: OnceLock<Option<Regex>> = OnceLock::new();
        RE.get_or_init(|| {
            // Match `pub use crate::module;` but NOT `pub use crate::module::Type;`
            Regex::new(r"pub\s+use\s+(?:crate|super|self)::([a-zA-Z_][a-zA-Z0-9_]*)\s*;").ok()
        })
        .as_ref()
    }

    /// Regex for `pub use module::name;` relative re-exports (no prefix)
    fn pub_use_relative_re() -> Option<&'static Regex> {
        static RE: OnceLock<Option<Regex>> = OnceLock::new();
        RE.get_or_init(|| Regex::new(r"pub\s+use\s+([a-zA-Z_][a-zA-Z0-9_:]*)\s*;").ok())
            .as_ref()
    }

    fn build_graph_context_inner(&self, files: &[String], root_dir: &str) -> GraphAnalysisContext {
        let mut import_graph: HashMap<String, Vec<String>> = HashMap::new();
        let mut inbound_links: HashMap<String, Vec<String>> = HashMap::new();
        let mut inheritance_map: HashMap<String, Vec<String>> = HashMap::new();
        let file_definitions: HashMap<String, Vec<String>> = HashMap::new();

        // Detect workspace root by walking up from root_dir until we find a directory with Cargo.toml
        let workspace_root = Self::find_workspace_root(root_dir);

        // Build set of known workspace crate dirs for external dep detection
        let mut workspace_modules: std::collections::HashSet<String> =
            std::collections::HashSet::new();
        // Perf 10: Pre-compute crate_name -> src_dir map
        let mut crate_src_dirs: HashMap<String, std::path::PathBuf> = HashMap::new();
        let root_path = std::path::Path::new(&workspace_root);
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
        let crate_module_index = Self::build_crate_module_index(&crate_src_dirs);

        // Expand files to include all workspace source files for cross-crate import resolution
        // This ensures that when scanning a subfolder, imports from other crates are visible
        let mut all_workspace_files: Vec<String> = files.to_vec();
        for src_dir in crate_src_dirs.values() {
            let workspace_files =
                shared::orphan_detector::utility_orphan_io::scan_directory_recursive(src_dir);
            for f in workspace_files {
                if !all_workspace_files.contains(&f) {
                    all_workspace_files.push(f);
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
                        && !all_workspace_files.contains(&path_str)
                    {
                        all_workspace_files.push(path_str);
                    }
                }
            }
        }
        let files = &all_workspace_files;

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
            if let Some(re) = Self::pub_mod_path_re() {
                for cap in re.captures_iter(&content) {
                    let mod_path = cap[1].to_string();
                    let base_dir = match std::path::Path::new(f).parent() {
                        Some(p) => p.to_path_buf(),
                        None => continue,
                    };
                    let root_path = std::path::Path::new(root_dir);
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
                        if shared::orphan_detector::utility_orphan_io::is_file(candidate) {
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
                        if let Some(resolved) = Self::resolve_workspace_module(
                            &crate_module_index,
                            crate_name,
                            &segments,
                            f,
                        ) {
                            Self::add_edge(&mut import_graph, &mut inbound_links, f, &resolved);
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

                // Python/JS relative imports
                import_graph.entry(f.clone()).or_default().push(dep.clone());
                inbound_links.entry(dep).or_default().push(f.clone());
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

            // Pass 5: pub use re-exports (e.g. `pub use crate::common::taxonomy_action_vo;`)
            if let Some(re) = Self::pub_use_re() {
                for cap in re.captures_iter(&content) {
                    let path = &cap[1]; // e.g. common::taxonomy_action_vo
                    let segments: Vec<&str> = path.split("::").collect();

                    // Try to resolve the re-exported module to a file
                    // First try the last segment (file stem)
                    if let Some(seg) = segments.last() {
                        if let Some(file_path) = module_to_file.get(*seg) {
                            if file_path != f {
                                Self::add_edge(&mut import_graph, &mut inbound_links, f, file_path);
                            }
                        }
                    }
                    // Also try composite path with / separator
                    if segments.len() >= 2 {
                        let composite = segments.join("/");
                        if let Some(file_path) = module_to_file.get(composite.as_str()) {
                            if file_path != f {
                                Self::add_edge(&mut import_graph, &mut inbound_links, f, file_path);
                            }
                        }
                    }
                }
            }

            // Pass 5b: pub use relative re-exports (e.g. `pub use taxonomy_language_vo::LanguageVO;`)
            if let Some(re) = Self::pub_use_relative_re() {
                for cap in re.captures_iter(&content) {
                    let path = &cap[1]; // e.g. taxonomy_language_vo::LanguageVO
                    let segments: Vec<&str> = path.split("::").collect();

                    // Try to resolve the re-exported module to a file
                    if let Some(seg) = segments.first() {
                        if let Some(file_path) = module_to_file.get(*seg) {
                            if file_path != f {
                                Self::add_edge(&mut import_graph, &mut inbound_links, f, file_path);
                            }
                        }
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

    fn build_crate_module_index(
        crate_src_dirs: &HashMap<String, std::path::PathBuf>,
    ) -> HashMap<String, HashMap<String, String>> {
        let mut index: HashMap<String, HashMap<String, String>> = HashMap::new();
        for (crate_name, src_dir) in crate_src_dirs {
            let mut module_map: HashMap<String, String> = HashMap::new();
            let canonical_src = std::fs::canonicalize(src_dir).unwrap_or_else(|_| src_dir.clone());
            let all_files = shared::orphan_detector::utility_orphan_io::scan_directory_recursive(
                &canonical_src,
            );
            for path_str in all_files {
                if !path_str.ends_with(".rs")
                    && !path_str.ends_with(".py")
                    && !path_str.ends_with(".ts")
                    && !path_str.ends_with(".js")
                {
                    continue;
                }
                let canonical_path = match std::fs::canonicalize(&path_str) {
                    Ok(p) => p,
                    Err(_) => std::path::PathBuf::from(&path_str),
                };
                let stem = canonical_path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or_default()
                    .to_string();
                if stem.is_empty() {
                    continue;
                }
                let canon_str = canonical_path.to_string_lossy().to_string();
                let rel_path = canonical_path
                    .strip_prefix(&canonical_src)
                    .unwrap_or(&canonical_path);
                let rel_str = rel_path.with_extension("").to_string_lossy().to_string();
                let normalized_rel =
                    shared::orphan_detector::utility_orphan_detector::normalize_module_path(
                        &rel_str.replace(std::path::MAIN_SEPARATOR, "/"),
                    );
                module_map.insert(normalized_rel, canon_str.clone());
                module_map.insert(stem.clone(), canon_str.clone());
                module_map.insert(
                    shared::orphan_detector::utility_orphan_detector::normalize_module_component(
                        &stem,
                    ),
                    canon_str.clone(),
                );
                if stem == "mod" || stem == "__init__" || stem == "index" {
                    if let Some(parent_dir) = canonical_path.parent().and_then(|p| p.file_name()) {
                        let parent = parent_dir.to_string_lossy().to_string();
                        module_map.insert(parent.clone(), canon_str.clone());
                        module_map.insert(
                            shared::orphan_detector::utility_orphan_detector::normalize_module_component(
                                &parent,
                            ),
                            canon_str.clone(),
                        );
                    }
                }
            }
            let normalized_name =
                shared::orphan_detector::utility_orphan_detector::normalize_module_component(
                    crate_name,
                );
            index.insert(crate_name.clone(), module_map.clone());
            index.insert(normalized_name, module_map);
        }
        index
    }

    fn resolve_workspace_module(
        index: &HashMap<String, HashMap<String, String>>,
        crate_name: &str,
        segments: &[&str],
        current_file: &str,
    ) -> Option<String> {
        let map = index.get(crate_name)?;
        let seg_str = segments.join("/");
        let normalized =
            shared::orphan_detector::utility_orphan_detector::normalize_module_path(&seg_str);
        if let Some(path) = map.get(&normalized) {
            if path != current_file {
                return Some(path.clone());
            }
        }
        for i in (1..segments.len()).rev() {
            let candidate = segments[..i].join("/");
            let normalized =
                shared::orphan_detector::utility_orphan_detector::normalize_module_path(&candidate);
            if let Some(path) = map.get(&normalized) {
                if path != current_file {
                    return Some(path.clone());
                }
            }
        }
        None
    }

    fn add_edge(
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

    /// Find workspace root by walking up from start_dir until we find a directory with Cargo.toml
    fn find_workspace_root(start_dir: &str) -> String {
        let mut current = std::path::PathBuf::from(start_dir);
        loop {
            // Check if current directory has Cargo.toml (workspace root indicator)
            if current.join("Cargo.toml").exists() {
                // Verify it's a workspace root by checking for crates/packages/modules dirs
                if current.join("crates").exists()
                    || current.join("packages").exists()
                    || current.join("modules").exists()
                {
                    return current.to_string_lossy().to_string();
                }
            }
            // Move up one directory
            if !current.pop() {
                // Reached filesystem root without finding workspace root
                return start_dir.to_string();
            }
        }
    }
}
