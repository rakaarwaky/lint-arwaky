use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::code_analysis::taxonomy_analysis_vo::GraphAnalysisContext;
use shared::code_analysis::taxonomy_analysis_vo::ImportGraph;
use shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use shared::code_analysis::taxonomy_analysis_vo::ReachabilityResult;
use shared::common::taxonomy_path_vo::FilePath;

use shared::common::taxonomy_severity_vo::Severity;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::orphan_detector::contract_orphan_aggregate::IOrphanAggregate;
use shared::orphan_detector::contract_orphan_graph_resolver_protocol::IOrphanGraphResolverProtocol;
use shared::orphan_detector::contract_orphan_protocol::{
    IAgentOrphanProtocol, ICapabilitiesOrphanProtocol, IContractOrphanProtocol,
    ISurfacesOrphanProtocol, ITaxonomyOrphanProtocol, IUtilityOrphanProtocol,
};
use shared::orphan_detector::taxonomy_orphan_contract_vo::OrphanFileListVO;

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

use rayon::prelude::*;

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
}

pub struct ArchOrphanAnalyzer {
    deps: ArchOrphanDeps,
    config: ArchitectureConfig,
}

// ─── Block 2: Aggregate Trait Implementation ──────────────
impl IOrphanAggregate for ArchOrphanAnalyzer {
    fn build_orphan_graph_context(
        &self,
        files: &OrphanFileListVO,
        root_dir: &FilePath,
    ) -> GraphAnalysisContext {
        let all_workspace_files = self._expand_workspace_files(files, root_dir);
        let full_files_vo = OrphanFileListVO::new(all_workspace_files);
        self.deps
            .resolver
            .build_graph_context(std::slice::from_ref(&full_files_vo), root_dir.value())
    }

    fn identify_orphan_entry_points(&self, files: &OrphanFileListVO) -> OrphanFileListVO {
        self.deps
            .resolver
            .identify_entry_points(std::slice::from_ref(files), &[])
    }

    fn check_orphans(&self, files: &OrphanFileListVO, root_dir: &FilePath) -> Vec<LintResult> {
        if !self.config.enabled.value {
            return Vec::new();
        }

        let all_workspace_files = self._expand_workspace_files(files, root_dir);

        let file_vo = OrphanFileListVO::new(all_workspace_files);
        let context = self
            .deps
            .resolver
            .build_graph_context(std::slice::from_ref(&file_vo), root_dir.value());

        self._check_orphans_inner(files, root_dir, &context, &file_vo)
    }

    fn scan_orphans(
        &self,
        root_dir: &FilePath,
        ignored: &[String],
    ) -> (GraphAnalysisContext, Vec<LintResult>) {
        let root_path = std::path::Path::new(root_dir.value());
        let mut all_files = Vec::new();
        if root_path.is_dir() {
            if let Ok(dir_path) =
                shared::common::taxonomy_path_vo::DirectoryPath::new(root_dir.value().to_string())
            {
                if let Ok(list) =
                    shared::common::utility_file_handler::scan_directory(&dir_path, ignored)
                {
                    all_files = list.values.iter().map(|f| f.value.clone()).collect();
                }
            }
        }
        let files_vo = OrphanFileListVO::new(all_files);
        let context = self.build_orphan_graph_context(&files_vo, root_dir);
        let results = self.check_orphans_with_context(&files_vo, root_dir, &context);
        (context, results)
    }

    fn check_orphans_with_context(
        &self,
        files: &OrphanFileListVO,
        root_dir: &FilePath,
        context: &GraphAnalysisContext,
    ) -> Vec<LintResult> {
        if !self.config.enabled.value {
            return Vec::new();
        }

        let all_workspace_files = self._expand_workspace_files(files, root_dir);
        let file_vo = OrphanFileListVO::new(all_workspace_files);

        self._check_orphans_inner(files, root_dir, context, &file_vo)
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────
impl ArchOrphanAnalyzer {
    pub fn new(deps: ArchOrphanDeps, config: ArchitectureConfig) -> Self {
        Self { deps, config }
    }

    fn _expand_workspace_files(
        &self,
        files: &OrphanFileListVO,
        root_dir: &FilePath,
    ) -> Vec<String> {
        let root_path = std::path::Path::new(root_dir.value());
        let top_root = shared::common::utility_file_handler::find_workspace_root(root_dir.value())
            .unwrap_or_else(|| root_path.to_path_buf());
        let mut seen: HashSet<String> = files.values.iter().cloned().collect();
        let mut result: Vec<String> = files.values.clone();
        for ws_dir in &["crates", "packages", "modules"] {
            let ws_path = top_root.join(ws_dir);
            if shared::orphan_detector::utility_orphan_io::is_dir(&ws_path) {
                let entries = shared::orphan_detector::utility_orphan_io::scan_directory(&ws_path);
                for (name, _path_str, is_dir_entry) in entries {
                    if !is_dir_entry {
                        continue;
                    }
                    let src_dir = top_root.join(ws_dir).join(&name).join("src");
                    if shared::orphan_detector::utility_orphan_io::is_dir(&src_dir) {
                        let workspace_files =
                            shared::orphan_detector::utility_orphan_io::scan_directory_recursive(
                                &src_dir,
                            );
                        for f in workspace_files {
                            let rel = std::path::Path::new(&f)
                                .strip_prefix(&top_root)
                                .map(|p| p.to_string_lossy().to_string())
                                .unwrap_or(f);
                            if seen.insert(rel.clone()) {
                                result.push(rel);
                            }
                        }
                    }
                }
            }
        }
        result
    }

    fn _check_orphans_inner(
        &self,
        files: &OrphanFileListVO,
        root_dir: &FilePath,
        context: &GraphAnalysisContext,
        file_vo: &OrphanFileListVO,
    ) -> Vec<LintResult> {
        let configured = self.get_orphan_entry_points();
        let configured_vo =
            shared::orphan_detector::taxonomy_orphan_contract_vo::OrphanEntryPatternListVO::new(
                configured,
            );
        let entry_points = self
            .deps
            .resolver
            .identify_entry_points(std::slice::from_ref(file_vo), &[configured_vo]);
        let alive_set = self._trace_reachability(&entry_points.values, &context.import_graph);
        let alive_result = ReachabilityResult::new(
            alive_set
                .iter()
                .filter_map(|s| FilePath::new(s.clone()).ok())
                .collect(),
        );

        let layer_keys: Vec<String> = self
            .config
            .layers
            .keys()
            .map(|k| k.value.to_string())
            .collect();

        let all_files: Vec<String> = file_vo.values.clone();
        let root_dir_str = root_dir.value().to_string();

        files
            .values
            .par_iter()
            .filter_map(|f| {
                self._process_file(
                    f,
                    context,
                    &alive_result,
                    &layer_keys,
                    &all_files,
                    &root_dir_str,
                )
            })
            .collect()
    }

    fn _process_file(
        &self,
        f: &str,
        context: &GraphAnalysisContext,
        alive_result: &ReachabilityResult,
        layer_keys: &[String],
        all_files: &[String],
        root_dir_str: &str,
    ) -> Option<LintResult> {
        let file_fp = FilePath::new(f.to_string()).ok()?;
        let filename = shared::common::utility_layer_detector::extract_filename(file_fp.value());
        let base_layer =
            shared::common::utility_layer_detector::detect_layer_from_prefix(filename)?;
        let layer_str = shared::common::utility_layer_detector::resolve_specialized_layer(
            &base_layer,
            file_fp.value(),
            layer_keys,
        );
        let definition =
            shared::common::utility_layer_detector::get_layer_def(&layer_str, &self.config.layers)?
                .clone();

        let basename = file_fp.basename();
        if definition.exceptions.values.contains(&basename) {
            return None;
        }
        if !definition.orphan.check_orphan.value {
            return None;
        }

        let layer_vo = LayerNameVO::new(&layer_str);
        let res =
            self._evaluate_layer(f, context, alive_result, &layer_vo, all_files, root_dir_str);
        if res.is_orphan {
            let code = match layer_str.to_lowercase() {
                s if s.contains(LAYER_TAXONOMY) => "AES501",
                s if s.contains(LAYER_CONTRACT) => "AES502",
                s if s.contains(LAYER_CAPABILITIES) => "AES503",
                s if s.contains(LAYER_UTILITY) => "AES504",
                s if s.contains(LAYER_AGENT) => "AES505",
                s if s.contains(LAYER_SURFACES) => "AES506",
                _ => return None,
            };
            return Some(self._make_result(f, &res.reason, res.severity, code));
        }
        None
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

    fn _trace_reachability(&self, entry_points: &[String], graph: &ImportGraph) -> HashSet<String> {
        let mut reachable: HashSet<String> = entry_points.iter().cloned().collect();
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

        reachable
    }

    fn _evaluate_layer(
        &self,
        f: &str,
        context: &GraphAnalysisContext,
        alive_result: &ReachabilityResult,
        layer_vo: &LayerNameVO,
        all_files: &[String],
        root_dir: &str,
    ) -> OrphanIndicatorResult {
        // Barrel file exceptions — package markers and re-export files, not logic
        if f.ends_with("__init__.py")
            || f.ends_with("/mod.rs")
            || f.ends_with("\\mod.rs")
            || f.ends_with("/index.ts")
            || f.ends_with("\\index.ts")
            || f.ends_with("/index.js")
            || f.ends_with("\\index.js")
        {
            return OrphanIndicatorResult::new(false, String::new(), Severity::HIGH);
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
            return self.deps.taxonomy_analyzer.is_taxonomy_orphan(
                &fp,
                &root,
                None,
                &context.inbound_links,
            );
        }

        if layer_str.contains(LAYER_CONTRACT) {
            return self.deps.contract_analyzer.is_contract_orphan(
                &fp,
                &root,
                &context.file_definitions,
                &context.inheritance_map,
                all_files,
            );
        }

        if layer_str.contains(LAYER_CAPABILITIES) {
            return self.deps.capabilities_analyzer.is_capabilities_orphan(
                &fp,
                &root,
                alive_result,
            );
        }

        if layer_str.contains(LAYER_UTILITY) {
            return self.deps.utility_analyzer.is_utility_orphan(
                &fp,
                &root,
                all_files,
                &context.inbound_links,
            );
        }

        if layer_str.contains(LAYER_AGENT) {
            return self
                .deps
                .agent_analyzer
                .is_agent_orphan(&fp, &root, all_files);
        }

        if layer_str.contains(LAYER_SURFACES) {
            return self
                .deps
                .surfaces_analyzer
                .is_surface_orphan(&fp, &root, alive_result, None);
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
