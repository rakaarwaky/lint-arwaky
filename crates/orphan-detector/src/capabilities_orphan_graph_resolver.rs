// PURPOSE: OrphanGraphResolver — build graph context and identify entry points for orphan analysis.
use shared::code_analysis::taxonomy_analysis_vo::FileDefinitionMap;
use shared::code_analysis::taxonomy_analysis_vo::GraphAnalysisContext;
use shared::code_analysis::taxonomy_analysis_vo::ImportGraph;
use shared::code_analysis::taxonomy_analysis_vo::InboundLinkMap;
use shared::code_analysis::taxonomy_analysis_vo::InheritanceMap;
use shared::orphan_detector::contract_orphan_graph_resolver_protocol::IOrphanGraphResolverProtocol;
use shared::orphan_detector::taxonomy_orphan_contract_vo::{
    OrphanEntryPatternListVO, OrphanFileListVO,
};

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
        if configured.is_empty() || configured.iter().all(|p| p.values.is_empty()) {
            return OrphanFileListVO::new(Vec::new());
        }
        let file_strs: Vec<String> = files
            .iter()
            .flat_map(|v| v.values.iter().cloned())
            .collect();
        let configured_strs: Vec<String> = configured
            .iter()
            .flat_map(|p| p.values.iter().cloned())
            .collect();
        let matched: Vec<String> = file_strs
            .iter()
            .filter(|f| {
                configured_strs
                    .iter()
                    .any(|pattern| f.ends_with(pattern) || f.contains(pattern))
            })
            .cloned()
            .collect();
        OrphanFileListVO::new(matched)
    }
}

impl OrphanGraphResolver {
    pub fn new() -> Self {
        Self {}
    }

    fn build_graph_context_inner(&self, files: &[String], root_dir: &str) -> GraphAnalysisContext {
        use std::collections::HashMap;
        let mut import_graph: HashMap<String, Vec<String>> = HashMap::new();
        let mut inbound_links: HashMap<String, Vec<String>> = HashMap::new();
        let mut inheritance_map: HashMap<String, Vec<String>> = HashMap::new();
        let file_definitions: HashMap<String, Vec<String>> = HashMap::new();

        // Build a lookup: module_name -> file_path for crate:: resolution
        let mut module_to_file: HashMap<String, String> = HashMap::new();
        for f in files {
            let basename = match f.split('/').next_back() {
                Some(b) => b,
                None => "",
            };
            let stem = basename
                .replace(".rs", "")
                .replace(".py", "")
                .replace(".ts", "")
                .replace(".js", "");
        // Map module stem to file path
        module_to_file.insert(stem.clone(), f.clone());
        // Also map with parent dir prefix for disambiguation
        if let Some(parent) = f.rsplit('/').nth(1) {
            let module_path = format!("{}/{}", parent, stem);
            module_to_file.insert(module_path, f.clone());
        }
    }
        }

        // DEBUG: check if taxonomy_path_vo is in module_to_file
        if let Some(debug_file) = module_to_file.get("taxonomy_path_vo") {
            eprintln!(
                "[DEBUG graph] module_to_file has taxonomy_path_vo -> {}",
                debug_file
            );
        } else {
            eprintln!(
                "[DEBUG graph] module_to_file MISSING taxonomy_path_vo (total entries: {})",
                module_to_file.len()
            );
        }

        // Build set of known workspace crate dirs for external dep detection
        let mut workspace_modules: std::collections::HashSet<String> =
            std::collections::HashSet::new();
        let root_path = std::path::Path::new(root_dir);
        for ws_dir in &["crates", "packages", "modules"] {
            let ws_path = root_path.join(ws_dir);
            if let Ok(entries) = std::fs::read_dir(&ws_path) {
                for entry in entries.flatten() {
                    if entry.path().is_dir() {
                        if let Some(name) = entry.file_name().to_str().map(|s| s.to_string()) {
                            workspace_modules.insert(name.clone());
                            workspace_modules.insert(name.replace('-', "_"));
                        }
                    }
                }
            }
        }

        // Also handle pub mod declarations with #[path] attributes (lib.rs pattern)
        let pub_mod_re =
            regex::Regex::new(r#"#\[path\s*=\s*"([^"]+)"\]\s*(?:pub\s+)?mod\s+([a-zA-Z_]+)"#).ok();
        for f in files {
            if let Ok(content) = std::fs::read_to_string(f) {
                if let Some(ref re) = pub_mod_re {
                    for cap in re.captures_iter(&content) {
                        let mod_path = cap[1].to_string();
                        let _mod_name = cap[2].to_string();
                        // Resolve: lib.rs has #[path = "layer-rules/mod.rs"] pub mod layer_rules
                        // → find files in layer-rules/ directory
                        let base_dir = match std::path::Path::new(f).parent() {
                            Some(p) => p.to_string_lossy().to_string(),
                            None => String::from("."),
                        };
                        let resolved_dir = format!(
                            "{}/{}",
                            base_dir,
                            mod_path.replace("/mod.rs", "").replace("mod.rs", ".")
                        );
                        // Find all .rs files in that directory
                        if let Ok(entries) = std::fs::read_dir(&resolved_dir) {
                            for entry in entries.flatten() {
                                let path = entry.path();
                                if path.is_file() {
                                    if let Some(ext) = path.extension() {
                                        if ext == "rs" || ext == "py" {
                                            if let Some(path_str) = path.to_str() {
                                                let resolved = path_str.to_string();
                                                if resolved != *f {
                                                    import_graph
                                                        .entry(f.clone())
                                                        .or_default()
                                                        .push(resolved.clone());
                                                    inbound_links
                                                        .entry(resolved)
                                                        .or_default()
                                                        .push(f.clone());
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        let import_re = regex::Regex::new(r"(?:use|import|from)\s+([a-zA-Z_][a-zA-Z0-9_\.:]*)");
        let inh_re = regex::Regex::new(r"class\s+\w+\(([^)]+)\)");
        for f in files {
            import_graph.entry(f.clone()).or_default();
            if let Ok(content) = std::fs::read_to_string(f) {
                if let Ok(ref import_re) = import_re {
                    for cap in import_re.captures_iter(&content) {
                        let full_import = cap[1].to_string();

                        // Handle crate:: and lint_arwaky:: imports (lint_arwaky = crate in main.rs)
                        let normalized =
                            if let Some(stripped) = full_import.strip_prefix("lint_arwaky::") {
                                format!("crate::{}", stripped)
                            } else {
                                full_import.clone()
                            };
                        let full_import = &normalized;
                        if let Some(path_part) = full_import.strip_prefix("crate::") {
                            // Extract module segments: source_parsing::taxonomy_path_vo::FilePath
                            // Module path is all segments except the last (item name)
                            // e.g. ["source_parsing", "taxonomy_path_vo", "FilePath"]
                            //   → module segments: ["source_parsing", "taxonomy_path_vo"]
                            //   → try "taxonomy_path_vo" first, then "source_parsing"
                            let segments: Vec<&str> = path_part.split("::").collect();
                            if segments.len() >= 2 {
                                // Try module segments from most specific to least
                                // Skip the last segment (item name like FilePath, LintResult)
                                let mut resolved = false;
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
                            // Fallback: single-segment import like use crate::taxonomy_foo
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

                        // Handle super:: imports
                        if let Some(path_part) = full_import.strip_prefix("super::") {
                            let segments: Vec<&str> = path_part.split("::").collect();
                            if segments.len() >= 2 {
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

                        // Skip external crates — dynamic check against local modules + workspace dirs
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

                        // Try to resolve workspace crate imports: e.g. "import_rules::root_import_rules_container"
                        if let Some(colon_idx) = full_import.find("::") {
                            let crate_name = &full_import[..colon_idx];
                            let rest = &full_import[colon_idx + 2..];
                            let segments: Vec<&str> = rest.split("::").collect();
                            if !segments.is_empty() {
                                let module_name = segments[0];
                                let member_roots = ["crates", "packages", "modules"];
                                let mut member_dirs = Vec::new();
                                for root in &member_roots {
                                    let base = root_path.join(root).to_string_lossy().to_string();
                                    let cn = crate_name.replace('_', "-");
                                    for path in &[
                                        format!("{}/{}/src", base, cn),
                                        format!("{}/{}/src", base, crate_name),
                                        format!("{}/{}", base, cn),
                                        format!("{}/{}", base, crate_name),
                                        format!("./{}/{}/src", root, cn),
                                        format!("{}/{}/src", root, cn),
                                    ] {
                                        member_dirs.push(path.clone());
                                    }
                                }
                                for dir in &member_dirs {
                                    if let Ok(entries) = std::fs::read_dir(dir) {
                                        for entry in entries.flatten() {
                                            let path = entry.path();
                                            if let Some(path_str) = path.to_str() {
                                                let stem =
                                                    match path.file_stem().and_then(|s| s.to_str())
                                                    {
                                                        Some(s) => s,
                                                        None => "",
                                                    };
                                                if stem == module_name && path_str != *f {
                                                    import_graph
                                                        .entry(f.clone())
                                                        .or_default()
                                                        .push(format!(
                                                            "./{}",
                                                            path_str.trim_start_matches("./")
                                                        ));
                                                    inbound_links
                                                        .entry(format!(
                                                            "./{}",
                                                            path_str.trim_start_matches("./")
                                                        ))
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
                }
                if let Ok(ref inh_re) = inh_re {
                    for cap in inh_re.captures_iter(&content) {
                        for base in cap[1].split(',') {
                            inheritance_map
                                .entry(f.clone())
                                .or_default()
                                .push(base.trim().to_string());
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
}
