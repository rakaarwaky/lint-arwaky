use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::taxonomy_analysis_vo::GraphAnalysisContext;
use shared::code_analysis::taxonomy_analysis_vo::ImportGraph;
use shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use shared::code_analysis::taxonomy_analysis_vo::ReachabilityResult;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::orphan_detector::contract_orphan_aggregate::IOrphanAggregate;
use shared::orphan_detector::contract_orphan_graph_resolver_protocol::IOrphanGraphResolverProtocol;
use shared::orphan_detector::contract_orphan_protocol::{
    IAgentOrphanProtocol, ICapabilitiesOrphanProtocol, IContractOrphanProtocol,
    ISurfacesOrphanProtocol, ITaxonomyOrphanProtocol, IUtilityOrphanProtocol,
};
use shared::role_rules::taxonomy_layer_names_constant::{
    LAYER_AGENT, LAYER_CAPABILITIES, LAYER_CONTRACT, LAYER_SURFACES, LAYER_TAXONOMY, LAYER_UTILITY,
};
use shared::taxonomy_adapter_name_vo::AdapterName;
use shared::taxonomy_common_vo::ColumnNumber;
use shared::taxonomy_common_vo::LineNumber;
use shared::taxonomy_error_vo::ErrorCode;
use shared::taxonomy_layer_vo::LayerNameVO;
use shared::taxonomy_lint_vo::LocationList;
use shared::taxonomy_lint_vo::ScopeRef;
use shared::taxonomy_message_vo::LintMessage;
use shared::taxonomy_suggestion_vo::DescriptionVO;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

/// Dependencies for ArchOrphanAnalyzer to avoid too_many_arguments.
pub struct ArchOrphanDeps {
    pub resolver: Arc<dyn IOrphanGraphResolverProtocol>,
    pub taxonomy_analyzer: Arc<dyn ITaxonomyOrphanProtocol>,
    pub contract_analyzer: Arc<dyn IContractOrphanProtocol>,
    pub capabilities_analyzer: Arc<dyn ICapabilitiesOrphanProtocol>,
    pub utility_analyzer: Arc<dyn IUtilityOrphanProtocol>,
    pub agent_analyzer: Arc<dyn IAgentOrphanProtocol>,
    pub surfaces_analyzer: Arc<dyn ISurfacesOrphanProtocol>,
    pub config: ArchitectureConfig,
}

pub struct ArchOrphanAnalyzer {
    resolver: Arc<dyn IOrphanGraphResolverProtocol>,
    taxonomy_analyzer: Arc<dyn ITaxonomyOrphanProtocol>,
    contract_analyzer: Arc<dyn IContractOrphanProtocol>,
    capabilities_analyzer: Arc<dyn ICapabilitiesOrphanProtocol>,
    utility_analyzer: Arc<dyn IUtilityOrphanProtocol>,
    agent_analyzer: Arc<dyn IAgentOrphanProtocol>,
    surfaces_analyzer: Arc<dyn ISurfacesOrphanProtocol>,
    config: ArchitectureConfig,
}

// ─── Block 2: Aggregate Trait Implementation ──────────────
impl IOrphanAggregate for ArchOrphanAnalyzer {
    fn build_orphan_graph_context(&self, files: &[String], root_dir: &str) -> GraphAnalysisContext {
        let file_vo = shared::orphan_detector::OrphanFileListVO::new(files.to_vec());
        self.resolver.build_graph_context(&[file_vo], root_dir)
    }

    fn identify_orphan_entry_points(&self, files: &[String]) -> HashSet<String> {
        let file_vo = shared::orphan_detector::OrphanFileListVO::new(files.to_vec());
        self.resolver
            .identify_entry_points(&[file_vo], &[])
            .values
            .into_iter()
            .collect()
    }

    fn check_orphans(&self, files: &[String], root_dir: &str) -> Vec<LintResult> {
        if !self.config.enabled.value {
            return Vec::new();
        }

        let ignored: Vec<String> = self
            .config
            .ignored_paths
            .values
            .iter()
            .map(|p| p.value().to_string())
            .collect();
        let filtered_files: Vec<String> = files
            .iter()
            .filter(|f| !shared::orphan_detector::utility_orphan_path::is_path_ignored(f, &ignored))
            .cloned()
            .collect();
        let files = filtered_files.as_slice();

        let mut results: Vec<LintResult> = Vec::new();
        let file_vo = shared::orphan_detector::OrphanFileListVO::new(files.to_vec());
        let context: GraphAnalysisContext = self
            .resolver
            .build_graph_context(std::slice::from_ref(&file_vo), root_dir);

        let configured = self.get_orphan_entry_points();
        let configured_vo = shared::orphan_detector::OrphanEntryPatternListVO::new(configured);
        let entry_points = self
            .resolver
            .identify_entry_points(&[file_vo], &[configured_vo]);
        let alive_files_set: Vec<String> =
            self._trace_reachability(&entry_points.values, &context.import_graph);

        for f in files {
            let file_fp = match FilePath::new(f.clone()) {
                Ok(fp) => fp,
                Err(_) => continue,
            };

            let filename =
                shared::common::utility_layer_detector::extract_filename(file_fp.value());
            let base_layer =
                match shared::common::utility_layer_detector::detect_layer_from_prefix(filename) {
                    Some(l) => l,
                    None => continue,
                };
            let layer_keys: Vec<String> = self
                .config
                .layers
                .keys()
                .map(|k| k.value.to_string())
                .collect();
            let layer_str = shared::common::utility_layer_detector::resolve_specialized_layer(
                &base_layer,
                file_fp.value(),
                &layer_keys,
            );
            let definition = match shared::common::utility_layer_detector::get_layer_def(
                &layer_str,
                &self.config.layers,
            ) {
                Some(d) => d.clone(),
                None => continue,
            };

            let basename = file_fp.basename();
            if definition.exceptions.values.contains(&basename) {
                continue;
            }
            if !definition.orphan.check_orphan.value {
                continue;
            }

            let layer_vo = LayerNameVO::new(&layer_str);
            let res =
                self._evaluate_layer(f, &context, &alive_files_set, &layer_vo, files, root_dir);
            if res.is_orphan {
                let code = match layer_str.to_lowercase() {
                    s if s.contains(LAYER_TAXONOMY) => "AES501",
                    s if s.contains(LAYER_CONTRACT) => "AES502",
                    s if s.contains(LAYER_CAPABILITIES) => "AES503",
                    s if s.contains(LAYER_UTILITY) => "AES504",
                    s if s.contains(LAYER_AGENT) => "AES505",
                    s if s.contains(LAYER_SURFACES) => "AES506",
                    _ => continue,
                };
                results.push(self._make_result(f, &res.reason, res.severity, code));
            }
        }

        results
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────
impl ArchOrphanAnalyzer {
    pub fn new(deps: ArchOrphanDeps) -> Self {
        Self {
            resolver: deps.resolver,
            taxonomy_analyzer: deps.taxonomy_analyzer,
            contract_analyzer: deps.contract_analyzer,
            capabilities_analyzer: deps.capabilities_analyzer,
            utility_analyzer: deps.utility_analyzer,
            agent_analyzer: deps.agent_analyzer,
            surfaces_analyzer: deps.surfaces_analyzer,
            config: deps.config,
        }
    }

    fn _make_result(&self, file: &str, msg: &str, sev: Severity, code: &str) -> LintResult {
        LintResult {
            file: FilePath {
                value: file.to_string(),
            },
            line: LineNumber::new(1),
            column: ColumnNumber::new(1),
            code: ErrorCode::raw(code),
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
        context: &GraphAnalysisContext,
        alive_files_set: &[String],
        layer_vo: &LayerNameVO,
        all_files: &[String],
        root_dir: &str,
    ) -> OrphanIndicatorResult {
        if f.ends_with("__init__.py") {
            return shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult::new(
                false,
                String::new(),
                Severity::HIGH,
            );
        }

        let layer_str = layer_vo.value.to_lowercase();
        let fp = match FilePath::new(f.to_string()) {
            Ok(fp) => fp,
            Err(_) => {
                return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
            }
        };
        let root = FilePath {
            value: root_dir.to_string(),
        };

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

        if layer_str.contains(LAYER_CAPABILITIES) {
            return self
                .capabilities_analyzer
                .is_capabilities_orphan(&fp, &root, &alive_set);
        }

        if layer_str.contains(LAYER_UTILITY) {
            return self.utility_analyzer.is_utility_orphan(
                &fp,
                &root,
                all_files,
                &context.inbound_links,
            );
        }

        if layer_str.contains(LAYER_AGENT) {
            return self.agent_analyzer.is_agent_orphan(&fp, &root, all_files);
        }

        if layer_str.contains(LAYER_SURFACES) {
            return self
                .surfaces_analyzer
                .is_surface_orphan(&fp, &root, &alive_set, None);
        }

        OrphanIndicatorResult::new(false, String::new(), Severity::LOW)
    }

    fn get_orphan_entry_points(&self) -> Vec<String> {
        let mut entry_points = vec![
            "_container.rs".into(),
            "_container.py".into(),
            "_container.ts".into(),
            "_container.js".into(),
            "_entry.rs".into(),
            "_entry.py".into(),
            "_entry.ts".into(),
            "_entry.js".into(),
            "main.rs".into(),
            "lib.rs".into(),
            "main.py".into(),
            "__main__.py".into(),
            "main.ts".into(),
            "main.js".into(),
            "index.ts".into(),
            "index.js".into(),
        ];
        for layer_def in self.config.layers.values() {
            entry_points.extend(layer_def.orphan.orphan_entry_points.values.iter().cloned());
        }
        entry_points.sort();
        entry_points.dedup();
        entry_points
    }
}
