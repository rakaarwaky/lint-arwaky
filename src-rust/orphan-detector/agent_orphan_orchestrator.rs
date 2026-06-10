// PURPOSE: Orchestrator: Agent orchestrator for orphan code detection
use crate::code_analysis::taxonomy_analysis_vo::FileDefinitionMap;
use crate::code_analysis::taxonomy_analysis_vo::GraphAnalysisContext;
use crate::code_analysis::taxonomy_analysis_vo::ImportGraph;
use crate::code_analysis::taxonomy_analysis_vo::InboundLinkMap;
use crate::code_analysis::taxonomy_analysis_vo::InheritanceMap;
use crate::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use crate::code_analysis::taxonomy_analysis_vo::ReachabilityResult;
use crate::layer_rules::contract_rule_protocol::IAnalyzer;
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::output_report::taxonomy_severity_vo::Severity;
use crate::shared_common::taxonomy_adapter_name_vo::AdapterName;
use crate::shared_common::taxonomy_common_vo::ColumnNumber;
use crate::shared_common::taxonomy_common_vo::LineNumber;
use crate::shared_common::taxonomy_definition_vo::LayerDefinition;
use crate::shared_common::taxonomy_error_vo::ErrorCode;
use crate::shared_common::taxonomy_layer_vo::LayerNameVO;
use crate::shared_common::taxonomy_lint_vo::LocationList;
use crate::shared_common::taxonomy_lint_vo::ScopeRef;
use crate::shared_common::taxonomy_message_vo::LintMessage;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use std::sync::Arc;

// Only contract layer imports for indicators!
use crate::orphan_detector::contract_orphan_protocol::{
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

        let import_re = regex::Regex::new(r"(?:use|import|from)\s+([a-zA-Z_][a-zA-Z0-9_\.]*)");
        let inh_re = regex::Regex::new(r"class\s+\w+\(([^)]+)\)");
        for f in files {
            import_graph.entry(f.clone()).or_default();
            if let Ok(content) = std::fs::read_to_string(f) {
                if let Ok(ref import_re) = import_re {
                    for cap in import_re.captures_iter(&content) {
                        let mut dep = cap[1].to_string();
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

use crate::shared_common::taxonomy_layer_names_constant::{
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
                analyzer,
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
            code: ErrorCode::raw("AES030"),
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

    fn _evaluate_layer(
        &self,
        analyzer: &dyn IAnalyzer,
        f: &str,
        definition: &LayerDefinition,
        context: &GraphAnalysisContext,
        alive_files_set: &[String],
        layer_vo: &LayerNameVO,
        _all_files: &[String],
    ) -> crate::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult {
        if f.ends_with("__init__.py") {
            return crate::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult::new(
                false,
                String::new(),
                Severity::HIGH,
            );
        }

        if self._check_dispatch_annotation(f) {
            return crate::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult::new(
                false,
                String::new(),
                Severity::HIGH,
            );
        }

        let layer_str = layer_vo.value.to_lowercase();

        if layer_str.contains(LAYER_TAXONOMY) {
            let fp = FilePath::new(f.to_string()).unwrap_or_default();
            let root = FilePath::new("".to_string()).unwrap_or_default();
            return self.taxonomy_analyzer.is_taxonomy_orphan(
                analyzer,
                &fp,
                &root,
                Some(definition),
                &context.inbound_links,
            );
        }

        if layer_str.contains(LAYER_CONTRACT) {
            let fp = FilePath::new(f.to_string()).unwrap_or_default();
            let root = FilePath::new("".to_string()).unwrap_or_default();
            return self.contract_analyzer.is_contract_orphan(
                analyzer,
                &fp,
                &root,
                &context.file_definitions,
                &context.inheritance_map,
            );
        }

        if layer_str.contains(LAYER_INFRASTRUCTURE) {
            let fp = FilePath::new(f.to_string()).unwrap_or_default();
            let root = FilePath::new("".to_string()).unwrap_or_default();
            let alive_set = ReachabilityResult::new(
                alive_files_set
                    .iter()
                    .filter_map(|s| FilePath::new(s.clone()).ok())
                    .collect(),
            );
            return self
                .infrastructure_analyzer
                .is_infrastructure_orphan(analyzer, &fp, &root, &alive_set);
        }

        if layer_str.contains(LAYER_CAPABILITIES) {
            let fp = FilePath::new(f.to_string()).unwrap_or_default();
            let root = FilePath::new("".to_string()).unwrap_or_default();
            let alive_set = ReachabilityResult::new(
                alive_files_set
                    .iter()
                    .filter_map(|s| FilePath::new(s.clone()).ok())
                    .collect(),
            );
            return self
                .capabilities_analyzer
                .is_capabilities_orphan(analyzer, &fp, &root, &alive_set);
        }

        if layer_str.contains(LAYER_AGENT) {
            let fp = FilePath::new(f.to_string()).unwrap_or_default();
            let root = FilePath::new("".to_string()).unwrap_or_default();
            return self.agent_analyzer.is_agent_orphan(analyzer, &fp, &root);
        }

        if layer_str.contains(LAYER_SURFACES) {
            let fp = FilePath::new(f.to_string()).unwrap_or_default();
            let alive_set = ReachabilityResult::new(
                alive_files_set
                    .iter()
                    .filter_map(|s| FilePath::new(s.clone()).ok())
                    .collect(),
            );
            return self
                .surfaces_analyzer
                .is_surface_orphan(&fp, &alive_set, Some(definition));
        }

        let fp = FilePath::new(f.to_string()).unwrap_or_default();
        let alive_set = ReachabilityResult::new(
            alive_files_set
                .iter()
                .filter_map(|s| FilePath::new(s.clone()).ok())
                .collect(),
        );
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

    fn _check_dispatch_annotation(&self, file_path: &str) -> bool {
        if let Ok(content) = std::fs::read_to_string(file_path) {
            for line in content.lines().take(30) {
                let trimmed = line.trim();
                if trimmed == "// aes: wired-by-dispatch" || trimmed == "# aes: wired-by-dispatch" {
                    return true;
                }
            }
        }
        false
    }
}

pub fn check_all_orphans(
    files: &[String],
    _root_dir: &str,
    eps: &std::collections::HashSet<String>,
    ctx: &crate::code_analysis::taxonomy_analysis_vo::GraphAnalysisContext,
    violations: &mut Vec<crate::output_report::taxonomy_result_vo::LintResult>,
) {
    for fp in files {
        if eps.contains(fp)
            || fp.ends_with("mod.rs")
            || fp.ends_with("__init__.py")
            || fp.ends_with("/index.ts")
            || fp.ends_with("/index.js")
        {
            continue;
        }

        // Skip orphan check if file has dispatch annotation
        if let Ok(fc) = std::fs::read_to_string(fp) {
            let first_lines: Vec<&str> = fc.lines().take(30).collect();
            let has_annotation = first_lines.iter().any(|l| {
                let t = l.trim();
                t == "// aes: wired-by-dispatch" || t == "# aes: wired-by-dispatch"
            });
            if has_annotation {
                continue;
            }
        }

        let basename = fp.split('/').next_back().unwrap_or("");
        let prefix = basename.split('_').next().unwrap_or("");

        if prefix == "taxonomy" {
            crate::orphan_detector::capabilities_orphan_taxonomy_analyzer::check_taxonomy_orphan(
                fp, basename, files, violations,
            );
            continue;
        }

        if prefix == "contract" {
            crate::orphan_detector::capabilities_orphan_contract_analyzer::check_contract_orphan(
                fp, basename, files, violations,
            );
            continue;
        }

        if prefix == "capabilities" {
            crate::orphan_detector::capabilities_orphan_capabilities_analyzer::check_capabilities_orphan(
                fp,
                basename,
                files,
                violations,
            );
            continue;
        }

        if prefix == "infrastructure" {
            crate::orphan_detector::capabilities_orphan_infrastructure_analyzer::check_infrastructure_orphan(
                fp,
                basename,
                files,
                violations,
            );
            continue;
        }

        if prefix == "agent" {
            crate::orphan_detector::capabilities_orphan_agent_analyzer::check_agent_orphan(
                fp, basename, files, violations,
            );
            continue;
        }

        if prefix == "surface" {
            crate::orphan_detector::capabilities_orphan_surfaces_analyzer::check_surfaces_orphan(
                fp, ctx, violations,
            );
            continue;
        }
    }
}
