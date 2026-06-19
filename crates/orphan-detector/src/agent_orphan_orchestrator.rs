// PURPOSE: IOrphanAggregate — aggregate trait implementing all orphan detection protocols
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_layer_detection_aggregate::ILayerDetectionAggregate;
use shared::code_analysis::taxonomy_analysis_vo::GraphAnalysisContext;
use shared::code_analysis::taxonomy_analysis_vo::ImportGraph;
use shared::code_analysis::taxonomy_analysis_vo::InboundLinkMap;
use shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use shared::code_analysis::taxonomy_analysis_vo::ReachabilityResult;
use shared::orphan_detector::contract_orphan_aggregate::IOrphanAggregate;
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

use shared::orphan_detector::contract_orphan_protocol::{
    IAgentOrphanProtocol, ICapabilitiesOrphanProtocol, IContractOrphanProtocol,
    IInfrastructureOrphanProtocol, ISurfacesOrphanProtocol, ITaxonomyOrphanProtocol,
};
use shared::role_rules::taxonomy_layer_names_constant::{
    LAYER_AGENT, LAYER_CAPABILITIES, LAYER_CONTRACT, LAYER_INFRASTRUCTURE, LAYER_SURFACES,
    LAYER_TAXONOMY,
};

use crate::capabilities_orphan_graph_resolver::OrphanGraphResolver;
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

            if !definition.orphan.check_orphan.value {
                continue;
            }

            let layer_vo = LayerNameVO::new(&layer_str);
            let res =
                self._evaluate_layer(f, &definition, &context, &alive_files_set, &layer_vo, files);

            if res.is_orphan {
                let code = match layer_str.to_lowercase() {
                    s if s.contains(LAYER_TAXONOMY) => "AES501",
                    s if s.contains(LAYER_CONTRACT) => "AES502",
                    s if s.contains(LAYER_CAPABILITIES) => "AES503",
                    s if s.contains(LAYER_INFRASTRUCTURE) => "AES504",
                    s if s.contains(LAYER_AGENT) => "AES505",
                    s if s.contains(LAYER_SURFACES) => "AES506",
                    _ => "AES500",
                };
                results.push(self._make_result(f, &res.reason, res.severity, code));
            }
        }

        results
    }
}

impl ArchOrphanAnalyzer {
    fn _make_result(&self, file: &str, msg: &str, sev: Severity, code: &str) -> LintResult {
        LintResult {
            file: FilePath::new(file.to_string()).unwrap_or_default(),
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
    ) -> OrphanIndicatorResult {
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

impl ILayerDetectionAggregate for ArchOrphanAnalyzer {
    fn detect_layer(&self, file_path: &str, _root_dir: &str) -> Option<String> {
        let path = std::path::Path::new(file_path);
        let filename = path.file_name()?.to_str()?;
        let stem = std::path::Path::new(filename).file_stem()?.to_str()?;

        const PREFIX_MAP: &[(&str, &str)] = &[
            ("taxonomy_", "taxonomy"),
            ("contract_", "contract"),
            ("capabilities_", "capabilities"),
            ("infrastructure_", "infrastructure"),
            ("agent_", "agent"),
            ("surface_", "surfaces"),
            ("root_", "root"),
        ];

        for (prefix, layer) in PREFIX_MAP {
            if stem.starts_with(prefix) {
                return Some(layer.to_string());
            }
        }

        None
    }

    fn get_layer_def(&self, _layer: &str) -> Option<LayerDefinition> {
        let mut def = LayerDefinition::default();
        def.orphan.check_orphan = shared::common::taxonomy_common_vo::BooleanVO::new(true);
        Some(def)
    }

    fn get_orphan_entry_points(&self) -> Vec<String> {
        vec![
            "main.rs".to_string(),
            "lib.rs".to_string(),
            "cli_main_entry.rs".to_string(),
            "mcp_main_entry.rs".to_string(),
            "tui_main_entry.rs".to_string(),
            "root_".to_string(),
        ]
    }
}
