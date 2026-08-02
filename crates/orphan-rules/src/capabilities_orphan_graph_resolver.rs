use crate::utility_orphan_filename::file_stem;
use crate::utility_orphan_graph_resolver;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::orphan_rules::IOrphanGraphResolverProtocol;
use shared::orphan_rules::IOrphanParserProtocol;
use shared::orphan_rules::taxonomy_orphan_parse_result_vo::{AstImportVO, FileParseResultVO};
use shared::orphan_rules::{OrphanEntryPatternListVO, OrphanFileListVO};
use shared::quality_rules::taxonomy_analysis_vo::{
    GraphAnalysisContext, ImportGraph, InboundLinkMap, InheritanceMap,
};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

struct RustResolveCtx<'a> {
    module_to_file: &'a HashMap<String, Vec<String>>,
    workspace_modules: &'a HashSet<String>,
    crate_module_index: &'a HashMap<String, HashMap<String, String>>,
    crate_src_dirs: &'a HashMap<String, std::path::PathBuf>,
}

struct PythonResolveCtx<'a> {
    module_to_file: &'a HashMap<String, Vec<String>>,
    root_path: &'a std::path::Path,
    workspace_root: &'a str,
}

pub struct OrphanGraphResolver {
    pub parser_dispatcher: Arc<dyn IOrphanParserProtocol>,
    pub filesystem: Arc<dyn IFilesystemAggregate>,
}

impl OrphanGraphResolver {
    pub fn new(
        parser_dispatcher: Arc<dyn IOrphanParserProtocol>,
        filesystem: Arc<dyn IFilesystemAggregate>,
    ) -> Self {
        Self {
            parser_dispatcher,
            filesystem,
        }
    }
}

impl IOrphanGraphResolverProtocol for OrphanGraphResolver {
    fn build_graph_context(
        &self,
        files: &[OrphanFileListVO],
        root_dir: &str,
    ) -> GraphAnalysisContext {
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
                    let stem = crate::utility_orphan_filename::file_stem(basename);
                    configured_strs.iter().any(|pattern| {
                        basename == pattern
                            || stem == *pattern
                            || (pattern.starts_with('_') && stem.ends_with(pattern.as_str()))
                            || (pattern.starts_with('.') && basename.ends_with(pattern.as_str()))
                            || (pattern == "root_" && basename.starts_with("root_"))
                            || (pattern.ends_with(".rs")
                                || pattern.ends_with(".py")
                                || pattern.ends_with(".ts")
                                || pattern.ends_with(".js"))
                                && basename.ends_with(pattern.as_str())
                    })
                })
                .cloned()
                .collect()
        };
        let mut matched = matched;
        matched.sort();
        matched.dedup();
        OrphanFileListVO::new(matched)
    }
}

impl OrphanGraphResolver {
    fn build_graph_context_inner(&self, files: &[String], root_dir: &str) -> GraphAnalysisContext {
        let mut import_graph: HashMap<String, Vec<String>> = HashMap::new();
        let mut inbound_links: HashMap<String, Vec<String>> = HashMap::new();
        let mut inheritance_map: HashMap<String, Vec<String>> = HashMap::new();

        let workspace_root = utility_orphan_graph_resolver::find_workspace_root(root_dir);
        let root_path = std::path::Path::new(&workspace_root);

        let mut workspace_modules: HashSet<String> = HashSet::new();
        let mut crate_src_dirs: HashMap<String, std::path::PathBuf> = HashMap::new();
        for ws_dir in &["crates", "packages", "modules"] {
            let ws_path = root_path.join(ws_dir);
            if ws_path.is_dir() {
                if let Ok(entries) = std::fs::read_dir(&ws_path) {
                    for entry in entries.flatten() {
                        let name = entry.file_name().to_string_lossy().to_string();
                        let path = entry.path();
                        if !path.is_dir() {
                            continue;
                        }
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

        let crate_module_index =
            utility_orphan_graph_resolver::build_crate_module_index(&crate_src_dirs);

        let mut all_workspace_files: Vec<String> = files.to_vec();
        let mut seen: HashSet<String> = files.iter().cloned().collect();
        for src_dir in crate_src_dirs.values() {
            collect_source_files_recursive(src_dir, &mut all_workspace_files, &mut seen, root_path);
        }

        let files = &all_workspace_files;

        let mut module_to_file: HashMap<String, Vec<String>> = HashMap::new();
        for f in files {
            let stem = file_stem(f);
            module_to_file
                .entry(stem.clone())
                .or_default()
                .push(f.clone());
            if let Some(parent) = f.rsplit('/').nth(1) {
                let module_path = format!("{}/{}", parent, stem);
                module_to_file
                    .entry(module_path.clone())
                    .or_default()
                    .push(f.clone());
                let normalized_path = module_path.replace('-', "_");
                if normalized_path != module_path {
                    module_to_file
                        .entry(normalized_path)
                        .or_default()
                        .push(f.clone());
                }
            }
            if (stem == "mod" || stem == "__init__")
                && let Some(parent_dir) = f.rsplit('/').nth(1)
            {
                module_to_file
                    .entry(parent_dir.to_string())
                    .or_default()
                    .push(f.clone());
                let normalized = parent_dir.replace('-', "_");
                if normalized != parent_dir {
                    module_to_file
                        .entry(normalized)
                        .or_default()
                        .push(f.clone());
                }
                if let Some(grandparent) = f.rsplit('/').nth(2) {
                    let composite = format!("{}/{}", grandparent, parent_dir);
                    module_to_file
                        .entry(composite.clone())
                        .or_default()
                        .push(f.clone());
                    let normalized_composite = composite.replace('-', "_");
                    if normalized_composite != composite {
                        module_to_file
                            .entry(normalized_composite)
                            .or_default()
                            .push(f.clone());
                    }
                }
            }
        }

        for f in files {
            import_graph.entry(f.clone()).or_default();
            let content = std::fs::read_to_string(std::path::Path::new(f)).unwrap_or_default();
            if content.is_empty() && !std::path::PathBuf::from(f).is_file() {
                continue;
            }

            match self.parser_dispatcher.parse_file(f, &content) {
                FileParseResultVO::Rust(result) => {
                    for mod_decl in &result.mod_decls {
                        if let Some(ref path_attr) = mod_decl.path_attr {
                            let base_dir = match std::path::Path::new(f).parent() {
                                Some(p) => p.to_path_buf(),
                                None => continue,
                            };
                            if let Some(resolved_path) = self
                                .filesystem
                                .resolve_orphan_module_path(root_path, &base_dir, path_attr)
                            {
                                let resolved = resolved_path.to_string_lossy().to_string();
                                if std::path::PathBuf::from(&resolved).is_file() && resolved != *f {
                                    utility_orphan_graph_resolver::add_edge(
                                        &mut import_graph,
                                        &mut inbound_links,
                                        f,
                                        &resolved,
                                    );
                                }
                            }
                        } else {
                            let parent = match std::path::Path::new(f).parent() {
                                Some(p) => p,
                                None => continue,
                            };
                            let candidates = [
                                parent.join(format!("{}.rs", mod_decl.name)),
                                parent.join(&mod_decl.name).join("mod.rs"),
                            ];
                            for candidate in &candidates {
                                let abs_candidate = if candidate.is_relative() {
                                    root_path.join(candidate)
                                } else {
                                    candidate.clone()
                                };
                                if abs_candidate.is_file()
                                    && let Some(path_str) = candidate.to_str()
                                {
                                    let resolved = path_str.to_string();
                                    if resolved != *f {
                                        utility_orphan_graph_resolver::add_edge(
                                            &mut import_graph,
                                            &mut inbound_links,
                                            f,
                                            &resolved,
                                        );
                                        break;
                                    }
                                }
                            }
                        }
                    }

                    let rust_ctx = RustResolveCtx {
                        module_to_file: &module_to_file,
                        workspace_modules: &workspace_modules,
                        crate_module_index: &crate_module_index,
                        crate_src_dirs: &crate_src_dirs,
                    };
                    for imp in &result.imports {
                        self.resolve_rust_import(
                            f,
                            imp,
                            &rust_ctx,
                            &mut import_graph,
                            &mut inbound_links,
                        );
                    }
                }
                FileParseResultVO::Python(result) => {
                    for (_class_name, bases) in &result.class_bases {
                        for base in bases {
                            inheritance_map
                                .entry(f.clone())
                                .or_default()
                                .push(base.clone());
                        }
                    }
                    let py_ctx = PythonResolveCtx {
                        module_to_file: &module_to_file,
                        root_path,
                        workspace_root: &workspace_root,
                    };
                    for imp in &result.imports {
                        self.resolve_python_import(
                            f,
                            imp,
                            &py_ctx,
                            &mut import_graph,
                            &mut inbound_links,
                        );
                    }
                }
                FileParseResultVO::TypeScript(result) => {
                    for (_class_name, interfaces) in &result.class_implements {
                        for iface in interfaces {
                            inheritance_map
                                .entry(f.clone())
                                .or_default()
                                .push(iface.clone());
                        }
                    }
                    for imp in &result.imports {
                        self.resolve_ts_import(
                            f,
                            imp,
                            &module_to_file,
                            root_path,
                            &mut import_graph,
                            &mut inbound_links,
                        );
                    }
                }
                FileParseResultVO::Unsupported => {}
            }
        }

        utility_orphan_graph_resolver::dedup_edges(&mut import_graph);
        utility_orphan_graph_resolver::dedup_edges(&mut inbound_links);

        GraphAnalysisContext::new(
            ImportGraph::new(import_graph),
            InboundLinkMap::new(inbound_links),
            InheritanceMap::new(inheritance_map),
            all_workspace_files,
        )
    }

    fn resolve_rust_import(
        &self,
        current_file: &str,
        imp: &AstImportVO,
        ctx: &RustResolveCtx<'_>,
        import_graph: &mut HashMap<String, Vec<String>>,
        inbound_links: &mut HashMap<String, Vec<String>>,
    ) {
        let segments = &imp.segments;
        if segments.is_empty() {
            return;
        }

        if segments[0] == "crate" {
            if segments.len() >= 2 {
                for i in (1..segments.len()).rev() {
                    let composite = segments[1..i].join("/");
                    if let Some(file_path) =
                        Self::resolve_module(ctx.module_to_file, &composite, current_file)
                        && file_path != current_file
                    {
                        utility_orphan_graph_resolver::add_edge(
                            import_graph,
                            inbound_links,
                            current_file,
                            file_path,
                        );
                        return;
                    }
                }
                if let Some(file_path) =
                    Self::resolve_module(ctx.module_to_file, &segments[1], current_file)
                    && file_path != current_file
                {
                    utility_orphan_graph_resolver::add_edge(
                        import_graph,
                        inbound_links,
                        current_file,
                        file_path,
                    );
                }
            }
            return;
        }

        if segments[0] == "super" {
            if segments.len() >= 2 {
                for i in (1..segments.len()).rev() {
                    let composite = segments[1..i].join("/");
                    if let Some(file_path) =
                        Self::resolve_module(ctx.module_to_file, &composite, current_file)
                        && file_path != current_file
                    {
                        utility_orphan_graph_resolver::add_edge(
                            import_graph,
                            inbound_links,
                            current_file,
                            file_path,
                        );
                        return;
                    }
                }
            }
            return;
        }

        if segments[0] == "self" {
            if segments.len() >= 2
                && let Some(file_path) =
                    Self::resolve_module(ctx.module_to_file, &segments[1], current_file)
                && file_path != current_file
            {
                utility_orphan_graph_resolver::add_edge(
                    import_graph,
                    inbound_links,
                    current_file,
                    file_path,
                );
            }
            return;
        }

        let crate_name = &segments[0];
        if ctx.workspace_modules.contains(crate_name.as_str())
            || ctx
                .workspace_modules
                .contains(&crate_name.replace('-', "_"))
        {
            let normalized_crate = crate_name.replace('-', "_");
            let seg_refs: Vec<&str> = segments.iter().map(|s| s.as_str()).collect();
            if let Some(resolved) = utility_orphan_graph_resolver::resolve_workspace_module(
                ctx.crate_module_index,
                &normalized_crate,
                &seg_refs[1..],
                current_file,
            ) {
                utility_orphan_graph_resolver::add_edge(
                    import_graph,
                    inbound_links,
                    current_file,
                    &resolved,
                );
                return;
            }

            let lookup_name = if ctx.crate_src_dirs.contains_key(crate_name.as_str()) {
                crate_name.clone()
            } else {
                normalized_crate
            };
            if let Some(src_dir) = ctx.crate_src_dirs.get(&lookup_name) {
                if let Ok(entries) = std::fs::read_dir(src_dir) {
                    let module_name = segments.get(1).map(|s| s.as_str()).unwrap_or("");
                    for entry in entries.flatten() {
                        let path = entry.path();
                        let path_str = path.to_string_lossy().to_string();
                        let stem = path
                            .file_stem()
                            .and_then(|s| s.to_str())
                            .unwrap_or_default();
                        let normalized_stem =
                            crate::utility_orphan_detector::normalize_module_component(stem);
                        if (stem == module_name || normalized_stem == module_name)
                            && path_str != current_file
                        {
                            utility_orphan_graph_resolver::add_edge(
                                import_graph,
                                inbound_links,
                                current_file,
                                &path_str,
                            );
                        }
                    }
                }
            }
            return;
        }

        let dep = &segments[0];
        let is_workspace_dir = matches!(dep.as_str(), "crates" | "packages" | "modules");
        if !is_workspace_dir
            && let Some(target) = Self::resolve_module(ctx.module_to_file, dep, current_file)
            && target != current_file
        {
            utility_orphan_graph_resolver::add_edge(
                import_graph,
                inbound_links,
                current_file,
                target,
            );
        }
    }

    fn resolve_python_import(
        &self,
        current_file: &str,
        imp: &AstImportVO,
        ctx: &PythonResolveCtx<'_>,
        import_graph: &mut HashMap<String, Vec<String>>,
        inbound_links: &mut HashMap<String, Vec<String>>,
    ) {
        let raw = &imp.raw_path;
        if raw.starts_with('.') {
            let dot_count = raw.chars().take_while(|&c| c == '.').count();
            let module_part = raw.trim_start_matches('.');
            let file_path = std::path::Path::new(current_file);
            let mut base_dir = file_path.parent().map(|p| p.to_path_buf());
            for _ in 1..dot_count {
                if let Some(ref dir) = base_dir {
                    base_dir = dir.parent().map(|p| p.to_path_buf());
                }
            }
            let Some(base) = base_dir else { return };
            if !module_part.is_empty() {
                for ext in &[".py", ".rs", ".ts", ".js"] {
                    let candidate = base.join(format!("{}{}", module_part, ext));
                    if candidate.is_file() {
                        let cand_rel = candidate
                            .strip_prefix(ctx.root_path)
                            .map(|p| p.to_string_lossy().to_string())
                            .unwrap_or_else(|_| candidate.to_string_lossy().to_string());
                        if cand_rel != current_file {
                            utility_orphan_graph_resolver::add_edge(
                                import_graph,
                                inbound_links,
                                current_file,
                                &cand_rel,
                            );
                        }
                        break;
                    }
                }
                let pkg_dir = base.join(module_part);
                for marker in &["__init__.py", "mod.rs"] {
                    if pkg_dir.join(marker).is_file() {
                        let cand_rel = pkg_dir
                            .join(marker)
                            .strip_prefix(ctx.root_path)
                            .map(|p| p.to_string_lossy().to_string())
                            .unwrap_or_else(|_| pkg_dir.join(marker).to_string_lossy().to_string());
                        if cand_rel != current_file {
                            utility_orphan_graph_resolver::add_edge(
                                import_graph,
                                inbound_links,
                                current_file,
                                &cand_rel,
                            );
                        }
                        break;
                    }
                }
            }
            if let Some(last_seg) = imp.segments.last()
                && let Some(target) =
                    Self::resolve_module(ctx.module_to_file, last_seg, current_file)
                && target != current_file
            {
                utility_orphan_graph_resolver::add_edge(
                    import_graph,
                    inbound_links,
                    current_file,
                    target,
                );
            }
            return;
        }

        if raw.contains('.') {
            let segments: Vec<&str> = raw.split('.').collect();
            let mut walk_dir = std::path::PathBuf::from(ctx.workspace_root);
            let mut walk_ok = true;
            for seg in &segments {
                walk_dir = walk_dir.join(seg);
                if !walk_dir.is_dir() {
                    walk_ok = false;
                    break;
                }
            }
            if walk_ok {
                for marker in &["__init__.py", "mod.rs", "index.ts", "index.js"] {
                    let candidate = walk_dir.join(marker);
                    if candidate.is_file() {
                        let cand_rel = candidate
                            .strip_prefix(ctx.root_path)
                            .map(|p| p.to_string_lossy().to_string())
                            .unwrap_or_else(|_| candidate.to_string_lossy().to_string());
                        if cand_rel != current_file {
                            utility_orphan_graph_resolver::add_edge(
                                import_graph,
                                inbound_links,
                                current_file,
                                &cand_rel,
                            );
                        }
                        break;
                    }
                }
            } else if let Some(last_seg) = segments.last()
                && let Some(target) =
                    Self::resolve_module(ctx.module_to_file, last_seg, current_file)
                && target != current_file
            {
                utility_orphan_graph_resolver::add_edge(
                    import_graph,
                    inbound_links,
                    current_file,
                    target,
                );
            }
            return;
        }

        if let Some(target) = Self::resolve_module(ctx.module_to_file, raw, current_file)
            && target != current_file
        {
            utility_orphan_graph_resolver::add_edge(
                import_graph,
                inbound_links,
                current_file,
                target,
            );
        }
    }

    fn resolve_ts_import(
        &self,
        current_file: &str,
        imp: &AstImportVO,
        module_to_file: &HashMap<String, Vec<String>>,
        root_path: &std::path::Path,
        import_graph: &mut HashMap<String, Vec<String>>,
        inbound_links: &mut HashMap<String, Vec<String>>,
    ) {
        let raw = &imp.raw_path;
        if raw.starts_with('.') {
            if let Some(resolved) =
                utility_orphan_graph_resolver::resolve_ts_relative(current_file, raw, root_path)
                && resolved != current_file
            {
                utility_orphan_graph_resolver::add_edge(
                    import_graph,
                    inbound_links,
                    current_file,
                    &resolved,
                );
            }
            return;
        }
        if let Some(last_seg) = imp.segments.last()
            && let Some(target) = Self::resolve_module(module_to_file, last_seg, current_file)
            && target != current_file
        {
            utility_orphan_graph_resolver::add_edge(
                import_graph,
                inbound_links,
                current_file,
                target,
            );
        }
    }

    fn resolve_module<'a>(
        module_to_file: &'a HashMap<String, Vec<String>>,
        key: &str,
        importer: &str,
    ) -> Option<&'a String> {
        let candidates = module_to_file.get(key)?;
        if candidates.len() == 1 {
            return candidates.first();
        }
        let importer_crate = importer.split('/').nth(1);
        candidates
            .iter()
            .find(|c| c.split('/').nth(1) == importer_crate)
            .or(candidates.first())
    }
}

fn collect_source_files_recursive(
    dir: &std::path::Path,
    files: &mut Vec<String>,
    seen: &mut HashSet<String>,
    root_path: &std::path::Path,
) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_source_files_recursive(&path, files, seen, root_path);
            } else if let Some(s) = path.to_str() {
                if s.ends_with(".rs")
                    || s.ends_with(".py")
                    || s.ends_with(".ts")
                    || s.ends_with(".js")
                {
                    let rel = path
                        .strip_prefix(root_path)
                        .map(|p| p.to_string_lossy().to_string())
                        .unwrap_or_else(|_| s.to_string());
                    if seen.insert(rel.clone()) {
                        files.push(rel);
                    }
                }
            }
        }
    }
}
