// PURPOSE: CapabilitiesOrphanAnalyzer — ICapabilitiesOrphanProtocol for orphan capability detection
use crate::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use crate::code_analysis::taxonomy_analysis_vo::ReachabilityResult;
use crate::orphan_detector::contract_orphan_protocol::ICapabilitiesOrphanProtocol;
use crate::output_report::taxonomy_severity_vo::Severity;
use crate::source_parsing::taxonomy_path_vo::FilePath;

pub struct CapabilitiesOrphanAnalyzer {}

impl Default for CapabilitiesOrphanAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl CapabilitiesOrphanAnalyzer {
    pub fn new() -> Self {
        Self {}
    }
}

impl ICapabilitiesOrphanProtocol for CapabilitiesOrphanAnalyzer {
    fn is_capabilities_orphan(
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
    let is_reachable = alive_files.paths.contains(f);
    is_infra_cap_orphan_raw_wired(false, is_reachable)
}

pub fn is_infra_cap_orphan_raw_wired(is_wired: bool, is_reachable: bool) -> OrphanIndicatorResult {
    let orphan = !is_wired && !is_reachable;
    OrphanIndicatorResult::new(
        orphan,
        "Not wired in container and unreachable.".into(),
        Severity::HIGH,
    )
}

pub fn is_infra_cap_orphan_raw(
    f: &FilePath,
    all_files: &[String],
    is_reachable: bool,
) -> OrphanIndicatorResult {
    let basename = f.basename();
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
    let mut is_wired = false;
    for cf in all_files {
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
                is_wired = true;
                break;
            }
        }
    }
    is_infra_cap_orphan_raw_wired(is_wired, is_reachable)
}

pub fn check_capabilities_orphan(
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
            &format!("capabilities '{}' not wired in container.", stem),
            Severity::HIGH,
        ));
    }
}
