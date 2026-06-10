// PURPOSE: Analyzer: Surfaces orphan detection logic
use crate::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use crate::code_analysis::taxonomy_analysis_vo::ReachabilityResult;
use crate::orphan_detector::contract_orphan_protocol::ISurfacesOrphanProtocol;
use crate::output_report::taxonomy_severity_vo::Severity;
use crate::shared_common::taxonomy_definition_vo::LayerDefinition;
use crate::source_parsing::taxonomy_path_vo::FilePath;

pub struct SurfacesOrphanAnalyzer {}

impl Default for SurfacesOrphanAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl SurfacesOrphanAnalyzer {
    pub fn new() -> Self {
        Self {}
    }
}

impl ISurfacesOrphanProtocol for SurfacesOrphanAnalyzer {
    fn is_surface_orphan(
        &self,
        f: &FilePath,
        alive_files: &ReachabilityResult,
        definition: Option<&LayerDefinition>,
    ) -> OrphanIndicatorResult {
        is_surface_orphan(f, alive_files, definition)
    }
}

pub fn is_surface_orphan(
    f: &FilePath,
    alive_files: &ReachabilityResult,
    _definition: Option<&LayerDefinition>,
) -> OrphanIndicatorResult {
    let alive: Vec<String> = alive_files
        .paths
        .iter()
        .map(|fp| fp.value().to_string())
        .collect();
    let orphan = !alive.contains(&f.value().to_string());
    OrphanIndicatorResult::new(orphan, "Surface is unreachable.".into(), Severity::MEDIUM)
}

pub fn check_surfaces_orphan(
    fp: &str,
    ctx: &crate::code_analysis::taxonomy_analysis_vo::GraphAnalysisContext,
    violations: &mut Vec<crate::output_report::taxonomy_result_vo::LintResult>,
) {
    let imps = ctx.import_graph.mapping.get(fp);
    if imps.map(std::vec::Vec::is_empty).unwrap_or(true) {
        violations.push(crate::orphan_detector::mk_orphan_result(
            fp,
            "Surface unreachable.",
            Severity::MEDIUM,
        ));
    }
}
