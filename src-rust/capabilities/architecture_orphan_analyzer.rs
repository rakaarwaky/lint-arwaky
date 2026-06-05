// arch_orphan_analyzer — Multi-indicator orphan code detection logic.
// Implements IArchOrphanProtocol: check_orphans.

use crate::contract::architecture_rule_protocol::IAnalyzer;
use crate::taxonomy::{
    AdapterName, ColumnNumber, ErrorCode, FileDefinitionMap, FilePath, FilePathSet,
    GraphAnalysisContext, ImportGraph, InboundLinkMap, InheritanceMap, LayerDefinition,
    LayerNameVO, LineNumber, LintMessage, LintResult, OrphanIndicatorResult, Severity,
};
use std::collections::HashMap;

/// Build graph context and identify entry points for orphan analysis.
pub struct OrphanGraphResolver;

impl OrphanGraphResolver {
    pub fn new() -> Self {
        Self
    }

    pub fn build_graph_context(&self, files: &[String], _root_dir: &str) -> GraphAnalysisContext {
        use std::collections::HashMap;
        let mut import_graph: HashMap<String, Vec<String>> = HashMap::new();
        let mut inbound_links: HashMap<String, Vec<String>> = HashMap::new();
        let mut inheritance_map: HashMap<String, Vec<String>> = HashMap::new();
        let file_definitions: HashMap<String, Vec<String>> = HashMap::new();

        for f in files {
            import_graph.entry(f.clone()).or_default();
            if let Ok(content) = std::fs::read_to_string(f) {
                let import_re = regex::Regex::new(r"(?:from|import)\s+([\w\.]+)").unwrap();
                for cap in import_re.captures_iter(&content) {
                    let dep = cap[1].to_string();
                    import_graph.entry(f.clone()).or_default().push(dep.clone());
                    inbound_links.entry(dep).or_default().push(f.clone());
                }
                let inh_re = regex::Regex::new(r"class\s+\w+\(([^)]+)\)").unwrap();
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
        GraphAnalysisContext::new(
            ImportGraph::new(import_graph),
            InboundLinkMap::new(inbound_links),
            InheritanceMap::new(inheritance_map),
            FileDefinitionMap::new(file_definitions),
        )
    }

    pub fn identify_entry_points(&self, files: &[String]) -> Vec<String> {
        files
            .iter()
            .filter(|f| f.contains("__main__") || f.ends_with("main.rs") || f.ends_with("lib.rs"))
            .cloned()
            .collect()
    }
}

/// Evaluate orphan indicators per layer type.
pub struct OrphanIndicatorEvaluator;

impl OrphanIndicatorEvaluator {
    pub fn new() -> Self {
        Self
    }

    pub fn is_taxonomy_orphan(
        &self,
        f: &str,
        _root: &str,
        _def: &LayerDefinition,
        inbound: &InboundLinkMap,
    ) -> OrphanIndicatorResult {
        let is_orphan = !inbound.mapping.contains_key(f);
        OrphanIndicatorResult::new(
            is_orphan,
            "Taxonomy VO has no inbound imports.".into(),
            Severity::LOW,
        )
    }

    pub fn is_contract_orphan(
        &self,
        f: &str,
        _root: &str,
        _defs: &FileDefinitionMap,
        _inh: &InheritanceMap,
    ) -> OrphanIndicatorResult {
        OrphanIndicatorResult::new(false, String::new(), Severity::LOW)
    }

    pub fn is_infra_cap_orphan(&self, is_wired: bool, is_reachable: bool) -> OrphanIndicatorResult {
        let orphan = !is_wired && !is_reachable;
        OrphanIndicatorResult::new(
            orphan,
            "Not wired in container and unreachable.".into(),
            Severity::HIGH,
        )
    }

    pub fn is_agent_orphan(&self, is_wired: bool) -> OrphanIndicatorResult {
        OrphanIndicatorResult::new(
            !is_wired,
            "Orchestrator not wired in DI container.".into(),
            Severity::HIGH,
        )
    }

    pub fn is_surface_orphan(
        &self,
        f: &str,
        alive: &Vec<String>,
        _def: &LayerDefinition,
    ) -> OrphanIndicatorResult {
        let orphan = !alive.contains(&f.to_string());
        OrphanIndicatorResult::new(orphan, "Surface is unreachable.".into(), Severity::MEDIUM)
    }

    pub fn is_generic_orphan(
        &self,
        f: &str,
        alive: &Vec<String>,
        inbound: &InboundLinkMap,
    ) -> OrphanIndicatorResult {
        let orphan = !alive.contains(&f.to_string()) && !inbound.mapping.contains_key(f);
        OrphanIndicatorResult::new(
            orphan,
            "File is unreachable and has no inbound imports.".into(),
            Severity::MEDIUM,
        )
    }
}

use crate::taxonomy::layer_names_constant::{
    LAYER_AGENT, LAYER_CAPABILITIES, LAYER_CONTRACT, LAYER_INFRASTRUCTURE, LAYER_SURFACES,
    LAYER_TAXONOMY,
};

pub struct ArchOrphanAnalyzer {
    resolver: OrphanGraphResolver,
    evaluator: OrphanIndicatorEvaluator,
}

impl ArchOrphanAnalyzer {
    pub fn new() -> Self {
        Self {
            resolver: OrphanGraphResolver::new(),
            evaluator: OrphanIndicatorEvaluator::new(),
        }
    }

    /// Check orphans for all files in the given list.
    pub fn check_orphans(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &[String],
        root_dir: &str,
    ) -> Vec<LintResult> {
        let mut results: Vec<LintResult> = Vec::new();
        let root_fp = FilePath::new(root_dir);

        // Build comprehensive context
        let context: GraphAnalysisContext = self.resolver.build_graph_context(files, root_dir);

        // Trace reachability
        let entry_points = self.resolver.identify_entry_points(files);
        let alive_files_set: Vec<String> =
            self._trace_reachability(&entry_points, &context.import_graph);

        // Evaluate each file
        for f in files {
            let file_fp = FilePath::new(f.clone());
            let layer_vo = match analyzer.detect_layer(&file_fp, &root_fp) {
                Some(l) => l,
                None => continue,
            };

            let definition = match analyzer.layer_map().values.get(&layer_vo) {
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

            let res = self._evaluate_layer(
                f,
                root_dir,
                definition,
                &context,
                &alive_files_set,
                &layer_vo,
            );

            if res.is_orphan {
                results.push(self._make_result(f, &res.reason, res.severity));
            }
        }

        results
    }

    fn _make_result(&self, file: &str, msg: &str, sev: Severity) -> LintResult {
        LintResult {
            file: FilePath::new(file.to_string()),
            line: LineNumber::new(1),
            column: ColumnNumber::new(1),
            code: ErrorCode::new("AES017"),
            message: LintMessage::new(msg),
            source: Some(AdapterName::new("architecture")),
            severity: sev,
            enclosing_scope: Some(crate::taxonomy::ScopeRef {
                name: String::new(),
                kind: String::new(),
                file: FilePath::new(String::new()),
                start_line: LineNumber::new(0),
                end_line: LineNumber::new(0),
            }),
            related_locations: crate::taxonomy::LocationList::new(Vec::new()),
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

    fn _detect_layer(
        &self,
        file_path: &str,
        root_dir: &str,
        layer_map: &HashMap<LayerNameVO, LayerDefinition>,
    ) -> Option<LayerNameVO> {
        let rel = file_path
            .trim_start_matches(root_dir)
            .trim_start_matches('/');

        // Sort by path-length descending
        let mut sorted_layers: Vec<(&LayerNameVO, &LayerDefinition)> = layer_map.iter().collect();
        sorted_layers.sort_by(|a, b| b.1.path.value.len().cmp(&a.1.path.value.len()));

        for (name, def) in &sorted_layers {
            if name.value.contains('(') {
                continue;
            }

            if rel.starts_with(&def.path.value)
                || rel.starts_with(&def.path.value.split('/').last().unwrap_or(""))
            {
                return Some(LayerNameVO::new(&name.value));
            }
        }

        None
    }

    fn _evaluate_layer(
        &self,
        f: &str,
        root_dir: &str,
        definition: &LayerDefinition,
        context: &GraphAnalysisContext,
        alive_files_set: &Vec<String>,
        layer_vo: &LayerNameVO,
    ) -> crate::taxonomy::OrphanIndicatorResult {
        // Skip barrel files
        if f.ends_with("__init__.py") {
            return crate::taxonomy::OrphanIndicatorResult::new(
                false,
                String::new(),
                Severity::HIGH,
            );
        }

        let layer_str = layer_vo.value.to_lowercase();

        if layer_str.contains(LAYER_TAXONOMY) {
            return self.evaluator.is_taxonomy_orphan(
                f,
                root_dir,
                definition,
                &context.inbound_links,
            );
        }

        if layer_str.contains(LAYER_CONTRACT) {
            return self.evaluator.is_contract_orphan(
                f,
                root_dir,
                &context.file_definitions,
                &context.inheritance_map,
            );
        }

        if layer_str.contains(LAYER_INFRASTRUCTURE) || layer_str.contains(LAYER_CAPABILITIES) {
            let is_wired = self._is_wired_in_container(f, root_dir);
            let is_reachable = alive_files_set.contains(&f.to_string());
            return self.evaluator.is_infra_cap_orphan(is_wired, is_reachable);
        }

        if layer_str.contains(LAYER_AGENT) {
            let is_wired = self._is_wired_in_container(f, root_dir);
            return self.evaluator.is_agent_orphan(is_wired);
        }

        if layer_str.contains(LAYER_SURFACES) {
            return self
                .evaluator
                .is_surface_orphan(f, alive_files_set, definition);
        }

        self.evaluator
            .is_generic_orphan(f, alive_files_set, &context.inbound_links)
    }

    fn _is_wired_in_container(&self, file_path: &str, root_dir: &str) -> bool {
        let stem = file_path
            .split('/')
            .next_back()
            .unwrap_or("")
            .replace(".py", "")
            .replace(".rs", "");

        // Look for container files in agent layer
        let container_dir = if root_dir.ends_with("/src") {
            format!("{}/{}", root_dir, LAYER_AGENT)
        } else {
            format!("{}/src/{}", root_dir, LAYER_AGENT)
        };

        // Scan for container files and check patterns
        self._scan_container_for_patterns(&container_dir, &stem, file_path)
    }

    fn _scan_container_for_patterns(
        &self,
        container_dir: &str,
        stem: &str,
        _file_path: &str,
    ) -> bool {
        use std::path::Path;

        if !Path::new(container_dir).exists() {
            return false;
        }

        // Walk through agent directory looking for container files
        let container_path = Path::new(container_dir);
        if let Ok(entries) = std::fs::read_dir(container_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                let path_str = path.to_string_lossy();

                // Check if this is a container file
                if path_str.to_lowercase().contains("container")
                    && (path_str.ends_with(".py") || path_str.ends_with(".rs"))
                {
                    if let Ok(content) = std::fs::read_to_string(&path) {
                        // Check for module stem in imports
                        if content.contains(&format!("import {}", stem))
                            || content.contains(&format!("from {} ", stem))
                            || content.contains(&stem)
                            || content.contains(&format!("use {};", stem.replace(".", "::")))
                            || content.contains(&format!("use crate::{}", stem))
                        {
                            return true;
                        }
                    }
                }
            }
        }

        // Also scan subdirectories recursively
        self._scan_recursive_for_patterns(container_dir, stem)
    }

    fn _scan_recursive_for_patterns(&self, dir: &str, stem: &str) -> bool {
        use std::path::Path;

        let path = Path::new(dir);
        if !path.is_dir() {
            return false;
        }

        let mut found = false;
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                let entry_path = entry.path();
                if entry_path.is_dir() {
                    if self
                        ._scan_recursive_for_patterns(entry_path.to_string_lossy().as_ref(), stem)
                    {
                        return true;
                    }
                } else if entry_path.is_file() {
                    let path_str = entry_path.to_string_lossy();
                    if path_str.to_lowercase().contains("container")
                        && (path_str.ends_with(".py") || path_str.ends_with(".rs"))
                    {
                        if let Ok(content) = std::fs::read_to_string(&entry_path) {
                            if content.contains(stem) {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        found
    }
}
