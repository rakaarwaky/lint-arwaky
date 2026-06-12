// PURPOSE: IOrphanAggregate — aggregate trait implementing all orphan detection protocols
use shared::shared::code_analysis::contract_layer_detection_aggregate::ILayerDetectionAggregate;
use shared::shared::code_analysis::taxonomy_analysis_vo::FileDefinitionMap;
use shared::shared::code_analysis::taxonomy_analysis_vo::GraphAnalysisContext;
use shared::shared::code_analysis::taxonomy_analysis_vo::ImportGraph;
use shared::shared::code_analysis::taxonomy_analysis_vo::InboundLinkMap;
use shared::shared::code_analysis::taxonomy_analysis_vo::InheritanceMap;
use shared::shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use shared::shared::code_analysis::taxonomy_analysis_vo::ReachabilityResult;
use shared::orphan_detector::contract_orphan_aggregate::IOrphanAggregate;
use shared::shared::output_report::taxonomy_result_vo::LintResult;
use shared::shared::output_report::taxonomy_severity_vo::Severity;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::taxonomy_adapter_name_vo::AdapterName;
use shared::taxonomy_common_vo::ColumnNumber;
use shared::taxonomy_common_vo::LineNumber;
use shared::taxonomy_definition_vo::LayerDefinition;
use shared::taxonomy_error_vo::ErrorCode;
use shared::taxonomy_layer_vo::LayerNameVO;
use shared::taxonomy_lint_vo::LocationList;
use shared::taxonomy_lint_vo::ScopeRef;
use shared::taxonomy_message_vo::LintMessage;
use shared::taxonomy_suggestion_vo::DescriptionVO;
use std::collections::HashSet;
use std::sync::Arc;

// Only contract layer imports for indicators!
use orphan_detector::contract_orphan_protocol::{
    IAgentOrphanProtocol, ICapabilitiesOrphanProtocol, IContractOrphanProtocol,
    IInfrastructureOrphanProtocol, ISurfacesOrphanProtocol, ITaxonomyOrphanProtocol,
};

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
                            // Extract module segments: layer_rules::capabilities_naming_checker::ArchNamingChecker
                            // → try to find file matching layer_rules/capabilities_naming_checker
                            let segments: Vec<&str> = path_part.split("::").collect();
                            if segments.len() >= 2 {
                                // crate::layer_rules::capabilities_naming_checker::ArchNamingChecker
                                // → segments[1] = "capabilities_naming_checker" → lookup
                                let module_name = segments[1];
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

                        // Skip external crates
                        let mut dep = full_import.clone();
                        if let Some(dot) = dep.find('.') {
                            dep = dep[..dot].to_string();
                        }
                        if let Some(colon) = dep.find("::") {
                            dep = dep[..colon].to_string();
                        }
                        if matches!(
                            dep.as_str(),
                            "crate"
                                | "self"
                                | "super"
                                | "std"
                                | "core"
                                | "alloc"
                                | "serde"
                                | "tokio"
                                | "regex"
                                | "once_cell"
                                | "thiserror"
                                | "anyhow"
                                | "async_trait"
                                | "clap"
                                | "chrono"
                                | "tracing"
                                | "rand"
                                | "toml"
                                | "serde_json"
                                | "serde_yaml"
                                | "mcp_sdk_rs"
                                | "reqwest"
                                | "futures"
                                | "dashmap"
                                | "rustpython"
                                | "rustpython_vm"
                                | "rustpython_parser"
                                | "num_traits"
                                | "enum_dispatch"
                                | "pyo3"
                                | "nom"
                                | "log"
                                | "env_logger"
                                | "colored"
                                | "indicatif"
                                | "uuid"
                                | "sha2"
                                | "hex"
                                | "base64"
                        ) {
                            continue;
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
        files
            .iter()
            .filter(|f| {
                if !configured.is_empty() {
                    configured
                        .iter()
                        .any(|pattern| f.ends_with(pattern) || f.contains(pattern))
                } else {
                    f.contains("__main__")
                        || f.ends_with("main.rs")
                        || f.ends_with("lib.rs")
                        || f.ends_with("cli_main_entry.rs")
                        || f.ends_with("mcp_main_entry.rs")
                        || f.ends_with("tui_main_entry.rs")
                }
            })
            .cloned()
            .collect()
    }
}

use orphan_detector::taxonomy_layer_names_constant::{
    LAYER_AGENT, LAYER_CAPABILITIES, LAYER_CONTRACT, LAYER_INFRASTRUCTURE, LAYER_SURFACES,
    LAYER_TAXONOMY,
};

pub struct ArchOrphanAnalyzer {
    resolver: OrphanGraphResolver,
    taxonomy_analyzer: Arc<dyn ITaxonomyOrphanProtocol>,
    contract_analyzer: Arc<dyn IContractOrphanProtocol>,
    capabilities_analyzer: Arc<dyn ICapabilitiesOrphanProtocol>,
    infrastructure_analyzer: Arc<dyn IInfrastructureOrphanProtocol>,
    agent_analyzer: Arc<dyn IAgentOrphanProtocol>,
    surfaces_analyzer: Arc<dyn ISurfacesOrphanProtocol>,
}

impl ArchOrphanAnalyzer {
    pub fn new(
        taxonomy_analyzer: Arc<dyn ITaxonomyOrphanProtocol>,
        contract_analyzer: Arc<dyn IContractOrphanProtocol>,
        capabilities_analyzer: Arc<dyn ICapabilitiesOrphanProtocol>,
        infrastructure_analyzer: Arc<dyn IInfrastructureOrphanProtocol>,
        agent_analyzer: Arc<dyn IAgentOrphanProtocol>,
        surfaces_analyzer: Arc<dyn ISurfacesOrphanProtocol>,
    ) -> Self {
        Self {
            resolver: OrphanGraphResolver::new(),
            taxonomy_analyzer,
            contract_analyzer,
            capabilities_analyzer,
            infrastructure_analyzer,
            agent_analyzer,
            surfaces_analyzer,
        }
    }
}

impl IOrphanAggregate for ArchOrphanAnalyzer {
    fn build_orphan_graph_context(&self, files: &[String], root_dir: &str) -> GraphAnalysisContext {
        self.resolver.build_graph_context(files, root_dir)
    }

    fn identify_orphan_entry_points(&self, files: &[String]) -> HashSet<String> {
        self.resolver
            .identify_entry_points(files, &[])
            .into_iter()
            .collect()
    }

    /// Check orphans for all files in the given list via the checker aggregate for layer detection.
    fn check_orphans(
        &self,
        layer_detector: &dyn ILayerDetectionAggregate,
        files: &[String],
        root_dir: &str,
    ) -> Vec<LintResult> {
        let mut results: Vec<LintResult> = Vec::new();
        let _root_fp = FilePath::new(root_dir).unwrap_or_default();

        // Build comprehensive context
        let context: GraphAnalysisContext = self.resolver.build_graph_context(files, root_dir);

        // Trace reachability
        let configured = layer_detector.get_orphan_entry_points();
        let entry_points = self.resolver.identify_entry_points(files, &configured);
        let alive_files_set: Vec<String> =
            self._trace_reachability(&entry_points, &context.import_graph);

        // Evaluate each file
        for f in files {
            let file_fp = FilePath::new(f.clone()).unwrap_or_default();
            let layer_str = match layer_detector.detect_layer(f, root_dir) {
                Some(l) => l,
                None => continue,
            };

            let definition = match layer_detector.get_layer_def(&layer_str) {
                Some(d) => d,
                None => continue,
            };

            let basename = file_fp.basename();
            if definition.exceptions.values.contains(&basename) {
                continue;
            }

            if !definition.check_orphan.value {
                continue;
            }

            let layer_vo = LayerNameVO::new(&layer_str);
            let res =
                self._evaluate_layer(f, &definition, &context, &alive_files_set, &layer_vo, files);

            if res.is_orphan {
                results.push(self._make_result(f, &res.reason, res.severity));
            }
        }

        results
    }
}

impl ArchOrphanAnalyzer {
    fn _make_result(&self, file: &str, msg: &str, sev: Severity) -> LintResult {
        LintResult {
            file: FilePath::new(file.to_string()).unwrap_or_default(),
            line: LineNumber::new(1),
            column: ColumnNumber::new(1),
            code: ErrorCode::raw("AES030"),
            message: LintMessage::new(msg),
            source: Some(AdapterName::raw("architecture")),
            severity: sev,
            enclosing_scope: Some(ScopeRef {
                name: DescriptionVO::new(String::new()),
                kind: DescriptionVO::new(String::new()),
                file: None,
                start_line: None,
                end_line: None,
            }),
            related_locations: LocationList::new(),
        }
    }

    fn _trace_reachability(&self, entry_points: &[String], graph: &ImportGraph) -> Vec<String> {
        use std::collections::VecDeque;

        let mut reachable: std::collections::HashSet<String> =
            entry_points.iter().cloned().collect();
        let mut queue: VecDeque<String> = entry_points.iter().cloned().collect();

        while let Some(current) = queue.pop_front() {
            if let Some(neighbors) = graph.mapping.get(&current) {
                for neighbor in neighbors {
                    if reachable.insert(neighbor.clone()) {
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }

        reachable.into_iter().collect()
    }

    fn _evaluate_layer(
        &self,
        f: &str,
        definition: &LayerDefinition,
        context: &GraphAnalysisContext,
        alive_files_set: &[String],
        layer_vo: &LayerNameVO,
        all_files: &[String],
    ) -> shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult {
        if f.ends_with("__init__.py") {
            return shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult::new(
                false,
                String::new(),
                Severity::HIGH,
            );
        }

        let _ = definition;
        let layer_str = layer_vo.value.to_lowercase();
        let fp = FilePath::new(f.to_string()).unwrap_or_default();
        let root = FilePath::new(String::new()).unwrap_or_default();

        if layer_str.contains(LAYER_TAXONOMY) {
            return self.taxonomy_analyzer.is_taxonomy_orphan(
                &fp,
                &root,
                None,
                &context.inbound_links,
            );
        }

        if layer_str.contains(LAYER_CONTRACT) {
            return self.contract_analyzer.is_contract_orphan(
                &fp,
                &root,
                &context.file_definitions,
                &context.inheritance_map,
                all_files,
            );
        }

        let alive_set = ReachabilityResult::new(
            alive_files_set
                .iter()
                .filter_map(|s| FilePath::new(s.clone()).ok())
                .collect(),
        );

        if layer_str.contains(LAYER_INFRASTRUCTURE) {
            return self
                .infrastructure_analyzer
                .is_infrastructure_orphan(&fp, &root, &alive_set);
        }

        if layer_str.contains(LAYER_CAPABILITIES) {
            return self
                .capabilities_analyzer
                .is_capabilities_orphan(&fp, &root, &alive_set);
        }

        if layer_str.contains(LAYER_AGENT) {
            return self.agent_analyzer.is_agent_orphan(&fp, &root, all_files);
        }

        if layer_str.contains(LAYER_SURFACES) {
            return self
                .surfaces_analyzer
                .is_surface_orphan(&fp, &alive_set, None);
        }

        self._is_generic_orphan_helper(&fp, &alive_set, &context.inbound_links)
    }

    fn _is_generic_orphan_helper(
        &self,
        f: &FilePath,
        alive_files: &ReachabilityResult,
        inbound_links: &InboundLinkMap,
    ) -> OrphanIndicatorResult {
        let alive: Vec<String> = alive_files
            .paths
            .iter()
            .map(|fp| fp.value().to_string())
            .collect();
        let orphan = !alive.contains(&f.value().to_string())
            && !inbound_links.mapping.contains_key(f.value());
        OrphanIndicatorResult::new(
            orphan,
            "File is unreachable and has no inbound imports.".into(),
            Severity::MEDIUM,
        )
    }
}
