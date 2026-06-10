// PURPOSE: InfrastructureOrphanAnalyzer — IInfrastructureOrphanProtocol for orphan infrastructure detection
use crate::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use crate::code_analysis::taxonomy_analysis_vo::ReachabilityResult;
use crate::orphan_detector::contract_orphan_protocol::IInfrastructureOrphanProtocol;
use crate::output_report::taxonomy_severity_vo::Severity;
use crate::source_parsing::taxonomy_path_vo::FilePath;

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
        is_infra_cap_orphan(f, alive_files)
    }
}

pub fn is_infra_cap_orphan(
    f: &FilePath,
    alive_files: &ReachabilityResult,
) -> OrphanIndicatorResult {
    crate::orphan_detector::capabilities_orphan_capabilities_analyzer::is_infra_cap_orphan(
        f,
        alive_files,
    )
}

pub fn check_infrastructure_orphan(
    fp: &str,
    basename: &str,
    files: &[String],
    violations: &mut Vec<crate::output_report::taxonomy_result_vo::LintResult>,
) {
    let stem = basename.replace(".rs", "").replace(".py", "");
    let pascal_stem: String = stem
        .split('_')
        .filter(|s| !s.is_empty())
        .map(|s| {
            let mut c = s.chars();
            c.next()
                .map(|f| f.to_uppercase().to_string() + c.as_str())
                .unwrap_or_default()
        })
        .collect();
    let mut wired = false;
    for cf in files {
        let cb = cf.split('/').next_back().unwrap_or("");
        let csuffix = cb
            .rsplit('_')
            .next()
            .unwrap_or("")
            .replace(".rs", "")
            .replace(".py", "");
        if csuffix != "container" && csuffix != "aggregate" && csuffix != "registry" {
            continue;
        }
        if let Ok(c) = std::fs::read_to_string(cf) {
            if c.contains(&stem) || c.contains(&format!("mod {}", stem)) || c.contains(&pascal_stem)
            {
                wired = true;
                break;
            }
        }
    }
    if !wired {
        violations.push(crate::orphan_detector::mk_orphan_result(
            fp,
            &format!("infrastructure '{}' not wired in container.", stem),
            Severity::HIGH,
        ));
    }
}
