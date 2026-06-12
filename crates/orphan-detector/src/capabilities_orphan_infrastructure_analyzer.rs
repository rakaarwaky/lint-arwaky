// PURPOSE: InfrastructureOrphanAnalyzer — IInfrastructureOrphanProtocol for orphan infrastructure detection
use shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use shared::code_analysis::taxonomy_analysis_vo::ReachabilityResult;
use shared::orphan_detector::contract_orphan_protocol::IInfrastructureOrphanProtocol;
use shared::output_report::taxonomy_severity_vo::Severity;
use shared::source_parsing::taxonomy_path_vo::FilePath;

pub struct InfrastructureOrphanAnalyzer {}

impl Default for InfrastructureOrphanAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl InfrastructureOrphanAnalyzer {
    pub fn new() -> Self {
        Self {}
    }
}

impl IInfrastructureOrphanProtocol for InfrastructureOrphanAnalyzer {
    fn is_infrastructure_orphan(
        &self,
        f: &FilePath,
        _root_dir: &FilePath,
        alive_files: &ReachabilityResult,
    ) -> OrphanIndicatorResult {
        let is_reachable = alive_files.paths.contains(f);
        if is_reachable {
            return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
        }
        OrphanIndicatorResult::new(
            true,
            "Not reachable from any entry point.".into(),
            Severity::HIGH,
        )
    }
}

pub fn check_infrastructure_orphan(
    fp: &str,
    basename: &str,
    files: &[String],
    violations: &mut Vec<shared::output_report::taxonomy_result_vo::LintResult>,
) {
    crate::orphan_detector::capabilities_orphan_capabilities_analyzer::check_capabilities_orphan(
        fp, basename, files, violations,
    )
}
