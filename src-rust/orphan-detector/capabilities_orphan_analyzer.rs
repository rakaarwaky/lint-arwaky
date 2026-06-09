// arch_orphan_analyzer — Multi-indicator orphan code detection logic.
// Implements IArchOrphanProtocol: check_orphans.

use crate::code_analysis::taxonomy_analysis_vo::FileDefinitionMap;
use crate::code_analysis::taxonomy_analysis_vo::FilePathSet;
use crate::code_analysis::taxonomy_analysis_vo::GraphAnalysisContext;
use crate::code_analysis::taxonomy_analysis_vo::ImportGraph;
use crate::code_analysis::taxonomy_analysis_vo::InboundLinkMap;
use crate::code_analysis::taxonomy_analysis_vo::InheritanceMap;
use crate::code_analysis::taxonomy_analysis_vo::ModuleToFileMap;
use crate::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use crate::code_analysis::taxonomy_analysis_vo::ReachabilityResult;
use crate::layer_rules::contract_rule_protocol::IAnalyzer;
use crate::orphan_detector::contract_orphan_protocol::IOrphanGraphProtocol;
use crate::orphan_detector::contract_orphan_protocol::IOrphanIndicatorProtocol;
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::output_report::taxonomy_severity_vo::Severity;
use crate::shared_common::taxonomy_common_error::ModuleName;
use crate::shared_common::taxonomy_common_vo::ColumnNumber;
use crate::shared_common::taxonomy_common_vo::LineNumber;
use crate::shared_common::taxonomy_definition_vo::LayerDefinition;
use crate::shared_common::taxonomy_error_vo::ErrorCode;
use crate::shared_common::taxonomy_layer_vo::LayerNameVO;
use crate::shared_common::taxonomy_lint_vo::LocationList;
use crate::shared_common::taxonomy_lint_vo::ScopeRef;
use crate::shared_common::taxonomy_message_vo::LintMessage;
use crate::shared_common::taxonomy_name_vo::AdapterName;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use crate::source_parsing::taxonomy_paths_vo::FilePathList;
use async_trait::async_trait;
use std::collections::HashMap;

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

        let import_re = regex::Regex::new(r"(?:from|import)\s+([\w\.]+)");
        let inh_re = regex::Regex::new(r"class\s+\w+\(([^)]+)\)");
        for f in files {
            import_graph.entry(f.clone()).or_default();
            if let Ok(content) = std::fs::read_to_string(f) {
                if let Ok(ref import_re) = import_re {
                    for cap in import_re.captures_iter(&content) {
                        let dep = cap[1].to_string();
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

    pub fn identify_entry_points(&self, files: &[String]) -> Vec<String> {
        files
            .iter()
            .filter(|f| f.contains("__main__") || f.ends_with("main.rs") || f.ends_with("lib.rs"))
            .cloned()
            .collect()
    }
}

/// Evaluate orphan indicators per layer type.
pub struct OrphanIndicatorEvaluator {}

impl Default for OrphanIndicatorEvaluator {
    fn default() -> Self {
        Self::new()
    }
}

impl OrphanIndicatorEvaluator {
    pub fn new() -> Self {
        Self {}
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
        _f: &str,
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
        alive: &[String],
        _def: &LayerDefinition,
    ) -> OrphanIndicatorResult {
        let orphan = !alive.contains(&f.to_string());
        OrphanIndicatorResult::new(orphan, "Surface is unreachable.".into(), Severity::MEDIUM)
    }

    pub fn is_generic_orphan(
        &self,
        f: &str,
        alive: &[String],
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

use crate::shared_common::taxonomy_names_constant::{
    LAYER_AGENT, LAYER_CAPABILITIES, LAYER_CONTRACT, LAYER_INFRASTRUCTURE, LAYER_SURFACES,
    LAYER_TAXONOMY,
};

pub struct ArchOrphanAnalyzer {
    resolver: OrphanGraphResolver,
    evaluator: OrphanIndicatorEvaluator,
}

impl Default for ArchOrphanAnalyzer {
    fn default() -> Self {
        Self::new()
    }
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
        let root_fp = FilePath::new(root_dir).unwrap_or_default();

        // Build comprehensive context
        let context: GraphAnalysisContext = self.resolver.build_graph_context(files, root_dir);

        // Trace reachability
        let entry_points = self.resolver.identify_entry_points(files);
        let alive_files_set: Vec<String> =
            self._trace_reachability(&entry_points, &context.import_graph);

        // Evaluate each file
        for f in files {
            let file_fp = FilePath::new(f.clone()).unwrap_or_default();
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
                definition,
                &context,
                &alive_files_set,
                &layer_vo,
                files,
            );

            if res.is_orphan {
                results.push(self._make_result(f, &res.reason, res.severity));
            }
        }

        results
    }

    fn _make_result(&self, file: &str, msg: &str, sev: Severity) -> LintResult {
        LintResult {
            file: FilePath::new(file.to_string()).unwrap_or_default(),
            line: LineNumber::new(1),
            column: ColumnNumber::new(1),
            code: ErrorCode::raw("AES037"),
            message: LintMessage::new(msg),
            source: Some(AdapterName::raw("architecture")),
            severity: sev,
            enclosing_scope: Some(ScopeRef {
                name: crate::shared_common::taxonomy_suggestion_vo::DescriptionVO::new(
                    String::new(),
                ),
                kind: crate::shared_common::taxonomy_suggestion_vo::DescriptionVO::new(
                    String::new(),
                ),
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
        sorted_layers.sort_by_key(|b| std::cmp::Reverse(b.1.path.value.len()));

        for (name, def) in &sorted_layers {
            if name.value.contains('(') {
                continue;
            }

            if rel.starts_with(&def.path.value)
                || rel.starts_with(def.path.value.split('/').next_back().unwrap_or(""))
            {
                return Some(LayerNameVO::new(&name.value));
            }
        }

        None
    }

    fn _evaluate_layer(
        &self,
        f: &str,
        definition: &LayerDefinition,
        context: &GraphAnalysisContext,
        alive_files_set: &[String],
        layer_vo: &LayerNameVO,
        all_files: &[String],
    ) -> crate::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult {
        if f.ends_with("__init__.py") {
            return crate::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult::new(
                false, String::new(), Severity::HIGH,
            );
        }

        let layer_str = layer_vo.value.to_lowercase();
        let basename = f.split('/').next_back().unwrap_or("");

        if layer_str.contains(LAYER_TAXONOMY) {
            let stem = basename.replace(".rs", "").replace(".py", "");
            let mut imported = false;
            for cf in all_files {
                let cb = cf.split('/').next_back().unwrap_or("");
                if !cb.starts_with("contract_") { continue; }
                if let Ok(c) = std::fs::read_to_string(cf) {
                    if c.contains(&stem) {
                        imported = true; break;
                    }
                }
            }
            return OrphanIndicatorResult::new(!imported, "Taxonomy not imported by any contract.".into(), Severity::LOW);
        }

        if layer_str.contains(LAYER_CONTRACT) {
            let suffix = basename.rsplit('_').next().unwrap_or("").replace(".rs", "");
            let target_prefix = match suffix.as_str() {
                "port" => "infrastructure",
                "protocol" => "capabilities",
                "aggregate" => "agent",
                _ => return OrphanIndicatorResult::new(false, String::new(), Severity::LOW),
            };
            let trait_name = basename.strip_prefix("contract_").unwrap_or(basename).replace(".rs", "");
            let mut has_impl = false;
            for cf in all_files {
                let cb = cf.split('/').next_back().unwrap_or("");
                if !cb.starts_with(target_prefix) { continue; }
                if let Ok(c) = std::fs::read_to_string(cf) {
                    if c.contains(&format!("impl {} for", trait_name)) { has_impl = true; break; }
                }
            }
            return OrphanIndicatorResult::new(!has_impl, format!("Contract {} '{}' not implemented.", suffix, trait_name), Severity::HIGH);
        }

        if layer_str.contains(LAYER_INFRASTRUCTURE) || layer_str.contains(LAYER_CAPABILITIES) {
            let is_wired = self._is_wired_in_container(basename, all_files);
            return self.evaluator.is_infra_cap_orphan(is_wired, alive_files_set.contains(&f.to_string()));
        }

        if layer_str.contains(LAYER_AGENT) {
            let is_wired = self._is_wired_in_container(basename, all_files);
            return self.evaluator.is_agent_orphan(is_wired);
        }

        if layer_str.contains(LAYER_SURFACES) {
            return self.evaluator.is_surface_orphan(f, alive_files_set, definition);
        }

        self.evaluator.is_generic_orphan(f, alive_files_set, &context.inbound_links)
    }

    fn _is_wired_in_container(&self, basename: &str, all_files: &[String]) -> bool {
        let stem = basename.replace(".rs", "").replace(".py", "");
        for f in all_files {
            let fb = f.split('/').next_back().unwrap_or("");
            let suffix = fb.rsplit('_').next().unwrap_or("").replace(".rs", "");
            if suffix != "container" && suffix != "aggregate" && suffix != "registry" { continue; }
            if let Ok(c) = std::fs::read_to_string(f) {
                if c.contains(&stem) || c.contains(&format!("mod {}", stem)) {
                    return true;
                }
            }
        }
        false
    }
}

#[async_trait]
impl IOrphanGraphProtocol for OrphanGraphResolver {
    async fn build_graph_context(
        &self,
        _analyzer: &dyn IAnalyzer,
        full_project_files: &FilePathList,
        root_dir: &FilePath,
    ) -> GraphAnalysisContext {
        let files: Vec<String> = full_project_files
            .iter()
            .map(|f| f.value().to_string())
            .collect();
        self.build_graph_context(&files, root_dir.value())
    }

    async fn resolve_import_to_file(
        &self,
        _analyzer: &dyn IAnalyzer,
        _current_file: &FilePath,
        _module_path: &ModuleName,
        _root_dir: &FilePath,
        _module_to_file: Option<&ModuleToFileMap>,
    ) -> Option<FilePath> {
        None
    }

    async fn identify_entry_points(
        &self,
        _analyzer: &dyn IAnalyzer,
        all_files: &FilePathList,
        _root_dir: &FilePath,
    ) -> FilePathList {
        let files: Vec<String> = all_files.iter().map(|f| f.value().to_string()).collect();
        let result = self.identify_entry_points(&files);
        let paths: Vec<FilePath> = result
            .into_iter()
            .filter_map(|s| FilePath::new(s).ok())
            .collect();
        FilePathList::new(paths)
    }

    async fn trace_reachability(
        &self,
        entry_points: &FilePathList,
        graph: &ImportGraph,
    ) -> ReachabilityResult {
        let mut reachable: std::collections::HashSet<String> = entry_points
            .iter()
            .map(|fp| fp.value().to_string())
            .collect();
        let mut queue: std::collections::VecDeque<String> = reachable.iter().cloned().collect();
        while let Some(current) = queue.pop_front() {
            if let Some(neighbors) = graph.mapping.get(&current) {
                for neighbor in neighbors {
                    if reachable.insert(neighbor.clone()) {
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }
        let paths: FilePathSet = reachable
            .into_iter()
            .filter_map(|s| FilePath::new(s).ok())
            .collect();
        ReachabilityResult::new(paths)
    }
}

#[async_trait]
impl IOrphanIndicatorProtocol for OrphanIndicatorEvaluator {
    async fn is_taxonomy_orphan(
        &self,
        _analyzer: &dyn IAnalyzer,
        f: &FilePath,
        root_dir: &FilePath,
        definition: Option<&LayerDefinition>,
        inbound_links: &InboundLinkMap,
    ) -> OrphanIndicatorResult {
        let def = match definition {
            Some(d) => d,
            None => {
                return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
            }
        };
        self.is_taxonomy_orphan(f.value(), root_dir.value(), def, inbound_links)
    }

    async fn is_contract_orphan(
        &self,
        _analyzer: &dyn IAnalyzer,
        f: &FilePath,
        root_dir: &FilePath,
        file_definitions: &FileDefinitionMap,
        inheritance_map: &InheritanceMap,
    ) -> OrphanIndicatorResult {
        self.is_contract_orphan(
            f.value(),
            root_dir.value(),
            file_definitions,
            inheritance_map,
        )
    }

    async fn is_infra_cap_orphan(
        &self,
        _analyzer: &dyn IAnalyzer,
        f: &FilePath,
        _root_dir: &FilePath,
        alive_files: &ReachabilityResult,
    ) -> OrphanIndicatorResult {
        let is_reachable = alive_files.paths.contains(f);
        self.is_infra_cap_orphan(false, is_reachable)
    }

    async fn is_agent_orphan(
        &self,
        _analyzer: &dyn IAnalyzer,
        _f: &FilePath,
        _root_dir: &FilePath,
    ) -> OrphanIndicatorResult {
        self.is_agent_orphan(false)
    }

    async fn is_surface_orphan(
        &self,
        f: &FilePath,
        alive_files: &ReachabilityResult,
        definition: Option<&LayerDefinition>,
    ) -> OrphanIndicatorResult {
        let alive: Vec<String> = alive_files
            .paths
            .iter()
            .map(|fp| fp.value().to_string())
            .collect();
        let def = match definition {
            Some(d) => d,
            None => {
                return OrphanIndicatorResult::new(false, String::new(), Severity::MEDIUM);
            }
        };
        self.is_surface_orphan(f.value(), &alive, def)
    }

    async fn is_generic_orphan(
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
        self.is_generic_orphan(f.value(), &alive, inbound_links)
    }
}
