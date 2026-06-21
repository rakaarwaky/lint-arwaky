// PURPOSE: OrphanGraphResolver — build graph context and identify entry points for orphan analysis.
use shared::code_analysis::taxonomy_analysis_vo::FileDefinitionMap;
use shared::code_analysis::taxonomy_analysis_vo::GraphAnalysisContext;
use shared::code_analysis::taxonomy_analysis_vo::ImportGraph;
use shared::code_analysis::taxonomy_analysis_vo::InboundLinkMap;
use shared::code_analysis::taxonomy_analysis_vo::InheritanceMap;

/// Build graph context and identify entry points for orphan analysis.
pub struct OrphanGraphResolver {}

impl Default for OrphanGraphResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl OrphanGraphResolver {
    pub fn new() -> Self {
        Self {}
    }

    pub fn build_graph_context(&self, files: &[String], _root_dir: &str) -> GraphAnalysisContext {
        use std::collections::HashMap;
        let mut import_graph: HashMap<String, Vec<String>> = HashMap::new();
        let mut inbound_links: HashMap<String, Vec<String>> = HashMap::new();
        let mut inheritance_map: HashMap<String, Vec<String>> = HashMap::new();
        let file_definitions: HashMap<String, Vec<String>> = HashMap::new();

        // Build a lookup: module_name -> file_path for crate:: resolution
        let mut module_to_file: HashMap<String, String> = HashMap::new();
        for f in files {
            let basename = f.split('/').next_back().unwrap_or("");
            let stem = basename
                .replace(".rs", "")
                .replace(".py", "")
                .replace(".ts", "")
                .replace(".js", "");
            // Map module stem to file path
            module_to_file.insert(stem.clone(), f.clone());
            // Also map with underscores replaced (for mod.rs references)
            if let Some(parent) = f.rsplit('/').nth(1) {
                let module_path = format!("{}/{}", parent, stem);
                module_to_file.insert(module_path, f.clone());
            }
        }

        // Build set of known workspace crate dirs for external dep detection
        let mut workspace_modules: std::collections::HashSet<String> =
            std::collections::HashSet::new();
        for ws_dir in &["crates", "packages", "modules"] {
            if let Ok(entries) = std::fs::read_dir(ws_dir) {
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
                        let base_dir = std::path::Path::new(f)
                            .parent()
                            .unwrap_or(std::path::Path::new("."))
                            .to_string_lossy()
                            .to_string();
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
                            // Extract module segments: capabilities_agent_role_auditor::AgentRoleChecker
                            // → try to find file matching capabilities_agent_role_auditor
                            let segments: Vec<&str> = path_part.split("::").collect();
                            if !segments.is_empty() {
                                let module_name = segments[0];
                                if let Some(resolved) = module_to_file.get(module_name) {
                                    if resolved != f {
                                        import_graph
                                            .entry(f.clone())
                                            .or_default()
                                            .push(resolved.clone());
                                        inbound_links
                                            .entry(resolved.clone())
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
                            // Resolve relative to parent module
                            let segments: Vec<&str> = path_part.split("::").collect();
                            if !segments.is_empty() {
                                let module_name = segments[0];
                                if let Some(resolved) = module_to_file.get(module_name) {
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
                                    for path in &[
                                        format!("./{}/{}/src", root, crate_name.replace('_', "-")),
                                        format!("{}/{}/src", root, crate_name.replace('_', "-")),
                                        format!("./{}/{}/src", root, crate_name),
                                        format!("{}/{}/src", root, crate_name),
                                        format!("./{}/{}", root, crate_name.replace('_', "-")),
                                        format!("{}/{}", root, crate_name.replace('_', "-")),
                                        format!("./{}/{}", root, crate_name),
                                        format!("{}/{}", root, crate_name),
                                    ] {
                                        member_dirs.push(path.clone());
                                    }
                                }
                                for dir in &member_dirs {
                                    if let Ok(entries) = std::fs::read_dir(dir) {
                                        for entry in entries.flatten() {
                                            let path = entry.path();
                                            if let Some(path_str) = path.to_str() {
                                                let stem = path
                                                    .file_stem()
                                                    .and_then(|s| s.to_str())
                                                    .unwrap_or("");
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

    pub fn identify_entry_points(&self, files: &[String], configured: &[String]) -> Vec<String> {
        if configured.is_empty() {
            return Vec::new();
        }

        files
            .iter()
            .filter(|f| {
                configured
                    .iter()
                    .any(|pattern| f.ends_with(pattern) || f.contains(pattern))
            })
            .cloned()
            .collect()
    }
}
