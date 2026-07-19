// PURPOSE: ArchOrphanAnalyzer — aggregate implementing all 6 orphan detection protocols (AES501-506)
//
// Orphan detection works by building a full import dependency graph across
// all source files, then tracing reachability from known entry points
// (_container.rs, main.rs, lib.rs, index.ts, etc.). Files that are not
// reachable are flagged as orphans.
//
// Each AES layer has its own orphan protocol (ITaxonomyOrphanProtocol for
// AES501, IContractOrphanProtocol for AES502, etc.) because different
// layers have different definition/reachability criteria:
//   - Taxonomy:  checked via inbound links (imports from other files)
//   - Contract:  checked via file definitions + inheritance map
//   - Infrastructure/Capabilities: checked via alive set (reachability)
//   - Agent:     checked via cross-file references
//   - Surfaces:  checked via alive set + optional entry detection
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_layer_detection_aggregate::ILayerDetectionAggregate;
use shared::code_analysis::taxonomy_analysis_vo::GraphAnalysisContext;
use shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use shared::code_analysis::taxonomy_analysis_vo::ReachabilityResult;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_aggregate::IOrphanAggregate;
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

use shared::orphan_detector::contract_orphan_graph_resolver_protocol::IOrphanGraphResolverProtocol;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct ArchOrphanAnalyzer {
    resolver: Arc<dyn IOrphanGraphResolverProtocol>,
    taxonomy_analyzer: Arc<dyn ITaxonomyOrphanProtocol>,
    contract_analyzer: Arc<dyn IContractOrphanProtocol>,
    capabilities_analyzer: Arc<dyn ICapabilitiesOrphanProtocol>,
    infrastructure_analyzer: Arc<dyn IInfrastructureOrphanProtocol>,
    agent_analyzer: Arc<dyn IAgentOrphanProtocol>,
    surfaces_analyzer: Arc<dyn ISurfacesOrphanProtocol>,
}

// ─── Block 2: Public Contract (domain aggregate ONLY) ─────
impl IOrphanAggregate for ArchOrphanAnalyzer {
    fn build_orphan_graph_context(&self, files: &[String], root_dir: &str) -> GraphAnalysisContext {
        // Bridge the raw &[String] parameter from IOrphanAggregate into the
        // VO-typed contract surface of IOrphanGraphResolverProtocol.
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

    /// Check orphans for all files in the given list via the checker aggregate for layer detection.
    ///
    /// This is the main orphan detection pipeline:
    ///   1. Build the import graph context (all imports between files)
    ///   2. Identify entry points (containers, main files, index files)
    ///   3. Trace reachability from entry points via BFS on the import graph
    ///   4. For each file not in the reachable set:
    ///      a. Detect its AES layer from filename prefix
    ///      b. Dispatch to layer-specific orphan protocol
    ///      c. If the protocol confirms it's orphan, add violation with appropriate AES code
    fn check_orphans(
        &self,
        layer_detector: &dyn ILayerDetectionAggregate,
        files: &[String],
        root_dir: &str,
    ) -> Vec<LintResult> {
        // Global gate: skip all orphan checks if architecture checker is disabled
        let config = layer_detector.config();
        if !config.enabled.value {
            return Vec::new();
        }

        let mut results: Vec<LintResult> = Vec::new();
        // Build comprehensive context — bridge &[String] -> &[OrphanFileListVO].
        let file_vo = shared::orphan_detector::OrphanFileListVO::new(files.to_vec());
        let context: GraphAnalysisContext = self
            .resolver
            .build_graph_context(std::slice::from_ref(&file_vo), root_dir);

        // Trace reachability: BFS from all entry points through the import graph
        let configured = layer_detector.get_orphan_entry_points();
        let configured_vo = shared::orphan_detector::OrphanEntryPatternListVO::new(configured);
        let entry_points = self
            .resolver
            .identify_entry_points(&[file_vo], &[configured_vo]);

        // Evaluate each file: alive (reachable) vs orphan (unreachable)
        for f in files {
            let file_fp = match FilePath::new(f.clone()) {
                Ok(fp) => fp,
                Err(_) => continue,
            };
            let layer_str = match layer_detector.detect_layer(f, root_dir) {
                Some(l) => l,
                None => continue,
            };

            let definition = match layer_detector.get_layer_def(&layer_str) {
                Some(d) => d,
                None => continue,
            };

            // Skip files in the layer's exception list (e.g., mod.rs, __init__.py)
            let basename = file_fp.basename();
            if definition.exceptions.values.contains(&basename) {
                continue;
            }

            // Skip if the specific orphan rule for this layer is disabled
            let rule_code = match layer_str.to_lowercase() {
                s if s.contains(LAYER_TAXONOMY) => "AES501",
                s if s.contains(LAYER_CONTRACT) => "AES502",
                s if s.contains(LAYER_CAPABILITIES) => "AES503",
                s if s.contains(LAYER_INFRASTRUCTURE) => "AES504",
                s if s.contains(LAYER_AGENT) => "AES505",
                s if s.contains(LAYER_SURFACES) => "AES506",
                _ => "",
            };
            if !rule_code.is_empty() && !config.is_rule_enabled(rule_code) {
                continue;
            }

            let layer_vo = LayerNameVO::new(&layer_str);
            let res = self._evaluate_layer(
                f,
                &context,
                &entry_points.values,
                &layer_vo,
                files,
                root_dir,
            );

            // If the layer-specific protocol confirms orphan status, emit the appropriate AES code
            if res.is_orphan {
                let code = match layer_str.to_lowercase() {
                    s if s.contains(LAYER_TAXONOMY) => "AES501",
                    s if s.contains(LAYER_CONTRACT) => "AES502",
                    s if s.contains(LAYER_CAPABILITIES) => "AES503",
                    s if s.contains(LAYER_INFRASTRUCTURE) => "AES504",
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

// ─── Block 3: Constructors, Std Traits & Helpers ─────────
impl Default for ArchOrphanAnalyzer {
    fn default() -> Self {
        Self::new(
            Arc::new(crate::capabilities_orphan_graph_resolver::OrphanGraphResolver::new(
                Arc::new(crate::capabilities_orphan_filename_extractor::OrphanFilenameExtractor::new()),
            )),
            Arc::new(crate::capabilities_orphan_taxonomy_analyzer::TaxonomyOrphanAnalyzer::new(Arc::new(crate::capabilities_orphan_filename_extractor::OrphanFilenameExtractor::new()))),
            Arc::new(crate::capabilities_orphan_contract_analyzer::ContractOrphanAnalyzer::new(Arc::new(crate::capabilities_orphan_filename_extractor::OrphanFilenameExtractor::new()))),
            Arc::new(crate::capabilities_orphan_capabilities_analyzer::CapabilitiesOrphanAnalyzer::new(Arc::new(crate::capabilities_orphan_filename_extractor::OrphanFilenameExtractor::new()), Arc::new(crate::infrastructure_file_cache::OrphanFileCache::new()))),
            Arc::new(crate::capabilities_orphan_infrastructure_analyzer::InfrastructureOrphanAnalyzer::new(Arc::new(crate::capabilities_orphan_filename_extractor::OrphanFilenameExtractor::new()), Arc::new(crate::infrastructure_file_cache::OrphanFileCache::new()))),
            Arc::new(crate::capabilities_orphan_agent_analyzer::AgentOrphanAnalyzer::new()),
            Arc::new(crate::capabilities_orphan_surfaces_analyzer::SurfacesOrphanAnalyzer::new(Arc::new(crate::capabilities_orphan_filename_extractor::OrphanFilenameExtractor::new()))),
        )
    }
}

impl ArchOrphanAnalyzer {
    pub fn new(
        resolver: Arc<dyn IOrphanGraphResolverProtocol>,
        taxonomy_analyzer: Arc<dyn ITaxonomyOrphanProtocol>,
        contract_analyzer: Arc<dyn IContractOrphanProtocol>,
        capabilities_analyzer: Arc<dyn ICapabilitiesOrphanProtocol>,
        infrastructure_analyzer: Arc<dyn IInfrastructureOrphanProtocol>,
        agent_analyzer: Arc<dyn IAgentOrphanProtocol>,
        surfaces_analyzer: Arc<dyn ISurfacesOrphanProtocol>,
    ) -> Self {
        Self {
            resolver,
            taxonomy_analyzer,
            contract_analyzer,
            capabilities_analyzer,
            infrastructure_analyzer,
            agent_analyzer,
            surfaces_analyzer,
        }
    }

    /// Build a LintResult from orphan analysis output.
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

    /// Evaluate layer-specific orphan detection via dispatched protocols.
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
            let all_files_fp: Vec<FilePath> = all_files
                .iter()
                .filter_map(|s| FilePath::new(s.clone()).ok())
                .collect();
            return self.contract_analyzer.is_contract_orphan(
                &fp,
                &root,
                &context.file_definitions,
                &context.inheritance_map,
                &all_files_fp,
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
            let all_files_fp: Vec<FilePath> = all_files
                .iter()
                .filter_map(|s| FilePath::new(s.clone()).ok())
                .collect();
            return self
                .agent_analyzer
                .is_agent_orphan(&fp, &root, &all_files_fp);
        }

        if layer_str.contains(LAYER_SURFACES) {
            return self
                .surfaces_analyzer
                .is_surface_orphan(&fp, &alive_set, None);
        }

        OrphanIndicatorResult::new(false, String::new(), Severity::LOW)
    }
}

impl ILayerDetectionAggregate for ArchOrphanAnalyzer {
    fn config(&self) -> &shared::config_system::taxonomy_config_vo::ArchitectureConfig {
        // ArchOrphanAnalyzer does not own the config; return default (enabled=true)
        // The real config check happens at the caller (ImportOrchestrator / CLI surface).
        static EMPTY: std::sync::OnceLock<
            shared::config_system::taxonomy_config_vo::ArchitectureConfig,
        > = std::sync::OnceLock::new();
        EMPTY.get_or_init(shared::config_system::taxonomy_config_vo::ArchitectureConfig::default)
    }
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
        def.exceptions.values = vec![
            "mod.rs".to_string(),
            "__init__.py".to_string(),
            "index.ts".to_string(),
            "index.js".to_string(),
            "py.typed".to_string(),
        ];
        Some(def)
    }

    fn get_orphan_entry_points(&self) -> Vec<String> {
        vec![
            "_container.rs".to_string(),
            "_container.py".to_string(),
            "_container.ts".to_string(),
            "_container.js".to_string(),
            "_entry.rs".to_string(),
            "_entry.py".to_string(),
            "_entry.ts".to_string(),
            "_entry.js".to_string(),
            "main.rs".to_string(),
            "lib.rs".to_string(),
            "main.py".to_string(),
            "main.ts".to_string(),
            "main.js".to_string(),
            "index.ts".to_string(),
            "index.js".to_string(),
        ]
    }
}

pub fn mk_orphan_result(file: &str, msg: &str, sev: Severity, code: &str) -> LintResult {
    LintResult {
        file: FilePath {
            value: file.to_string(),
        },
        line: LineNumber::new(0),
        column: ColumnNumber::new(0),
        code: ErrorCode::raw(code),
        message: LintMessage::new(msg),
        source: Some(AdapterName::raw("architecture")),
        severity: sev,
        enclosing_scope: None,
        related_locations: LocationList::new(),
    }
}
