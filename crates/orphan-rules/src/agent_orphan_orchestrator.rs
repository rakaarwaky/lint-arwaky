use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::quality_rules::taxonomy_analysis_vo::{
    GraphAnalysisContext, ImportGraph, OrphanIndicatorResult, ReachabilityResult,
};

use shared::common::taxonomy_path_vo::FilePath;

use shared::common::taxonomy_severity_vo::Severity;
use shared::config_system::ArchitectureConfig;
use shared::orphan_rules::IOrphanAggregate;

use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::orphan_rules::OrphanFileListVO;
use shared::orphan_rules::{
    IAgentOrphanProtocol, ICapabilitiesOrphanProtocol, IContractOrphanProtocol,
    ISurfacesOrphanProtocol, ITaxonomyOrphanProtocol, IUtilityOrphanProtocol,
};

use shared::common::taxonomy_common_vo::BooleanVO;
use shared::common::taxonomy_definition_vo::LayerDefinition;
use shared::common::taxonomy_definition_vo::OrphanRuleVO;
use shared::common::{
    AdapterName, ColumnNumber, DescriptionVO, ErrorCode, LayerNameVO, LineNumber, LintMessage,
    LocationList, ScopeRef,
};
use shared::role_rules::taxonomy_layer_names_constant::{
    LAYER_AGENT, LAYER_CAPABILITIES, LAYER_CONTRACT, LAYER_SURFACES, LAYER_TAXONOMY, LAYER_UTILITY,
};

use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

// ─── Block 1: Struct Definition ───────────────────────────

/// Dependencies for ArchOrphanAnalyzer to avoid too_many_arguments.
pub struct ArchOrphanDeps {
    pub taxonomy_analyzer: Arc<dyn ITaxonomyOrphanProtocol>,
    pub contract_analyzer: Arc<dyn IContractOrphanProtocol>,
    pub capabilities_analyzer: Arc<dyn ICapabilitiesOrphanProtocol>,
    pub utility_analyzer: Arc<dyn IUtilityOrphanProtocol>,
    pub agent_analyzer: Arc<dyn IAgentOrphanProtocol>,
    pub surfaces_analyzer: Arc<dyn ISurfacesOrphanProtocol>,
    pub filesystem: Arc<dyn IFilesystemAggregate>,
}

pub struct ArchOrphanAnalyzer {
    deps: ArchOrphanDeps,
    config: ArchitectureConfig,
}

// ─── Block 2: Aggregate Trait Implementation ──────────────
impl IOrphanAggregate for ArchOrphanAnalyzer {
    fn build_orphan_graph_context(
        &self,
        _files: &OrphanFileListVO,
        root_dir: &FilePath,
    ) -> GraphAnalysisContext {
        let root_path = std::path::Path::new(root_dir.value());
        let ignored = self.ignored_paths();
        self.deps
            .filesystem
            .build_orphan_graph_context(root_path, &ignored)
    }

    fn identify_orphan_entry_points(&self, files: &OrphanFileListVO) -> OrphanFileListVO {
        crate::utility_orphan_filename::identify_entry_points(std::slice::from_ref(files), &[])
    }

    fn check_orphans(&self, files: &OrphanFileListVO, root_dir: &FilePath) -> Vec<LintResult> {
        if !self.config.enabled.value {
            return Vec::new();
        }
        let context = self.build_orphan_graph_context(files, root_dir);
        let all_files = context.all_workspace_files.clone();
        let full_files_vo = OrphanFileListVO::new(all_files);
        self._check_orphans_inner(files, root_dir, &context, &full_files_vo)
    }

    fn scan_orphans(
        &self,
        root_dir: &FilePath,
        ignored: &[String],
    ) -> (GraphAnalysisContext, Vec<LintResult>) {
        let root_path = std::path::Path::new(root_dir.value());
        let context = self
            .deps
            .filesystem
            .build_orphan_graph_context(root_path, ignored);
        let files_vo = OrphanFileListVO::new(context.all_workspace_files.clone());
        let results = self.check_orphans_with_context(&files_vo, root_dir, &context);
        (context, results)
    }

    fn check_orphans_with_context(
        &self,
        files: &OrphanFileListVO,
        root_dir: &FilePath,
        context: &GraphAnalysisContext,
    ) -> Vec<LintResult> {
        eprintln!(
            "[debug orphan] config.enabled={}, root={}, all_ws_files={}, file_count={}",
            self.config.enabled.value,
            root_dir.value,
            context.all_workspace_files.len(),
            files.values.len(),
        );
        if !self.config.enabled.value {
            return Vec::new();
        }

        // Files are already expanded before graph context build (in scan_orphans),
        // so file_vo is the same as files — no need to expand again.
        self._check_orphans_inner(files, root_dir, context, files)
    }
    fn check_orphans_with_entries(
        &self,
        files: &[shared::filesystem::taxonomy_filesystem_vo::FileEntry],
        context: &GraphAnalysisContext,
    ) -> Vec<LintResult> {
        if !self.config.enabled.value {
            return Vec::new();
        }
        let file_paths: Vec<String> = files
            .iter()
            .filter(|f| f.parse_ok)
            .map(|f| f.path.to_string_lossy().to_string())
            .collect();
        let file_vo = OrphanFileListVO::new(file_paths);
        let root_dir = FilePath::new(".".to_string()).unwrap_or_default();
        self._check_orphans_inner(&file_vo, &root_dir, context, &file_vo)
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────
impl ArchOrphanAnalyzer {
    pub fn new(deps: ArchOrphanDeps, config: ArchitectureConfig) -> Self {
        Self { deps, config }
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
            shared::orphan_rules::taxonomy_orphan_contract_vo::OrphanEntryPatternListVO::new(
                configured,
            );

        // FR-001: Orphan detection must always identify entry points from ALL workspace files
        // (not just the scanned module). This ensures cross-module imports are resolved correctly.
        // When scanning modules/cli/, root_cli_main_entry.py in modules/ is still used as an
        // entry point, so surface files imported by it are not falsely flagged as orphans.
        let all_files = &context.all_workspace_files;

        let entry_points_vo = OrphanFileListVO::new(all_files.clone());
        let entry_points = crate::utility_orphan_filename::identify_entry_points(
            std::slice::from_ref(&entry_points_vo),
            &[configured_vo],
        );

        // Compute top_root early so alive_result can use absolute paths (matching
        // the format used by _process_file for file_fp — fixes path format mismatch
        // that caused false-positive AES506/AES503 orphan violations)
        let root_path = std::path::Path::new(root_dir.value());
        let top_root = self
            .deps
            .filesystem
            .workspace_root(root_dir)
            .unwrap_or_else(|| root_path.to_path_buf());
        let alive_set = self._trace_reachability(&entry_points.values, &context.import_graph);
        let alive_result = ReachabilityResult::new(
            alive_set
                .iter()
                .filter_map(|s| {
                    let abs = top_root.join(s);
                    FilePath::new(abs.to_string_lossy().to_string()).ok()
                })
                .collect(),
        );
        let layer_keys: Vec<String> = self
            .config
            .layers
            .keys()
            .map(|k| k.value.to_string())
            .collect();

        // Convert all_files to absolute paths so sub-analyzers can read file contents.
        // file_vo.values are relative to top_root (workspace root), but sub-analyzers
        // (contract, agent, utility) use orphan_io::read_file_safe() which needs
        // paths resolvable from CWD. Make them absolute by prepending top_root.
        let top_root_str = top_root.to_string_lossy().to_string();

        let all_files: Vec<String> = file_vo
            .values
            .iter()
            .map(|rel| {
                let abs = top_root.join(rel);
                abs.to_string_lossy().to_string()
            })
            .collect();
        // Pre-read all file contents into a map so capabilities don't do I/O.
        let content_map: HashMap<String, String> = all_files
            .iter()
            .filter_map(|f| {
                let c = self
                    .deps
                    .filesystem
                    .read_to_string(std::path::Path::new(f))
                    .unwrap_or_default();
                if c.is_empty() {
                    None
                } else {
                    Some((f.clone(), c))
                }
            })
            .collect();

        let violations: Vec<LintResult> = files
            .values
            .par_iter()
            .filter_map(|f| {
                self._process_file(
                    f,
                    context,
                    &alive_result,
                    &layer_keys,
                    &all_files,
                    &top_root_str,
                    &content_map,
                )
            })
            .collect();
        eprintln!(
            "[debug orphan inner] entry_points={}, violations={}",
            entry_points.values.len(),
            violations.len(),
        );
        violations
    }

    fn _process_file(
        &self,
        f: &str,
        context: &GraphAnalysisContext,
        alive_result: &ReachabilityResult,
        layer_keys: &[String],
        all_files: &[String],
        top_root_str: &str,
        content_map: &HashMap<String, String>,
    ) -> Option<LintResult> {
        static DEBUG_COUNT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let dc = DEBUG_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        // Resolve relative path to absolute so sub-analyzers can read file contents
        let abs_f = std::path::Path::new(top_root_str).join(f);
        let abs_f_str = abs_f.to_string_lossy().to_string();
        let file_fp = match FilePath::new(&abs_f_str) {
            Ok(fp) => fp,
            Err(_) => {
                if dc < 5 {
                    eprintln!(
                        "[debug _process_file] SKIP bad path: f='{}', abs='{}'",
                        f, abs_f_str
                    );
                }
                return None;
            }
        };
        let filename = shared::common::utility_layer_detector::extract_filename(file_fp.value());
        let base_layer = shared::common::utility_layer_detector::detect_layer_from_prefix(filename);
        if base_layer.is_none() {
            if dc < 5 {
                eprintln!(
                    "[debug _process_file] SKIP no layer prefix: f='{}', filename='{}'",
                    f, filename
                );
            }
            return None;
        }
        let base_layer = base_layer.unwrap();
        let layer_str = shared::common::utility_layer_detector::resolve_specialized_layer(
            &base_layer,
            file_fp.value(),
            layer_keys,
        );
        let definition =
            shared::common::utility_layer_detector::get_layer_def(&layer_str, &self.config.layers)
                .cloned()
                .unwrap_or_else(|| LayerDefinition {
                    orphan: OrphanRuleVO {
                        check_orphan: BooleanVO::new(true),
                        ..Default::default()
                    },
                    ..Default::default()
                });

        let basename = file_fp.basename();
        if definition.exceptions.values.contains(&basename) {
            if dc < 5 {
                eprintln!(
                    "[debug _process_file] SKIP exception: f='{}', basename='{}'",
                    f, basename
                );
            }
            return None;
        }
        if !definition.orphan.check_orphan.value {
            if dc < 5 {
                eprintln!("[debug _process_file] SKIP no orphan check: f='{}'", f);
            }
            return None;
        }

        // Check if the corresponding AES rule is disabled in config.rules
        let code = match layer_str.to_lowercase() {
            s if s.contains(LAYER_TAXONOMY) => "AES501",
            s if s.contains(LAYER_CONTRACT) => "AES502",
            s if s.contains(LAYER_CAPABILITIES) => "AES503",
            s if s.contains(LAYER_UTILITY) => "AES504",
            s if s.contains(LAYER_AGENT) => "AES505",
            s if s.contains(LAYER_SURFACES) => "AES506",
            _ => return None,
        };
        if self.is_rule_disabled(code) {
            return None;
        }

        let layer_vo = LayerNameVO::new(&layer_str);
        let res = self._evaluate_layer(
            &abs_f_str,
            context,
            alive_result,
            &layer_vo,
            all_files,
            top_root_str,
            content_map,
        );
        if res.is_orphan {
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
        top_root: &str,
        content_map: &HashMap<String, String>,
    ) -> OrphanIndicatorResult {
        // Barrel file exceptions — package markers and re-export files, not logic
        if f.ends_with("__init__.py")
            || f.ends_with("/mod.rs")
            || f.ends_with("\\mod.rs")
            || f.ends_with("/index.ts")
            || f.ends_with("\\index.ts")
            || f.ends_with("/index.js")
            || f.ends_with("\\index.js")
            || f.ends_with("/index.tsx")
            || f.ends_with("\\index.tsx")
            || f.ends_with("/index.jsx")
            || f.ends_with("\\index.jsx")
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
        // Use top_root as the root directory (absolute path so file operations work)
        let root = FilePath {
            value: top_root.to_string(),
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
                &context.inheritance_map,
                all_files,
                content_map,
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
                content_map,
            );
        }

        if layer_str.contains(LAYER_AGENT) {
            return self
                .deps
                .agent_analyzer
                .is_agent_orphan(&fp, &root, all_files, content_map);
        }

        if layer_str.contains(LAYER_SURFACES) {
            return self.deps.surfaces_analyzer.is_surface_orphan(
                &fp,
                &root,
                alive_result,
                &context.inbound_links,
                None,
            );
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
            "root_".into(),
            "main.rs".into(),
            "lib.rs".into(),
            "main.py".into(),
            "__main__.py".into(),
            "main.ts".into(),
            "main.js".into(),
            "index.ts".into(),
            "index.js".into(),
            "index.tsx".into(),
            "index.jsx".into(),
        ];
        for layer_def in self.config.layers.values() {
            entry_points.extend(layer_def.orphan.orphan_entry_points.values.iter().cloned());
        }
        entry_points.sort();
        entry_points.dedup();
        entry_points
    }

    /// Check if a specific AES rule code is disabled in the config.
    /// Maps AES501-AES506 to their corresponding rules and checks enabled flag.
    pub fn is_rule_disabled(&self, code: &str) -> bool {
        self.config
            .rules
            .iter()
            .find(|r| r.name.value.as_str() == code)
            .map(|r| !r.enabled.value)
            .unwrap_or(false)
    }

    fn ignored_paths(&self) -> Vec<String> {
        self.config
            .ignored_paths
            .values
            .iter()
            .map(|v| v.value.clone())
            .collect()
    }
}
