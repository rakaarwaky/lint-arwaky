// PURPOSE: CapabilitiesOrphanAnalyzer — ICapabilitiesOrphanProtocol for orphan capability detection
use crate::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use crate::code_analysis::taxonomy_analysis_vo::ReachabilityResult;
use crate::orphan_detector::contract_orphan_protocol::ICapabilitiesOrphanProtocol;
use crate::output_report::taxonomy_severity_vo::Severity;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use regex::Regex;

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
    // If reachable from entry point, not orphan
    if is_reachable {
        return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
    }
    OrphanIndicatorResult::new(
        true,
        "Not reachable from any entry point.".into(),
        Severity::HIGH,
    )
}

pub fn is_infra_cap_orphan_raw_wired(is_wired: bool, is_reachable: bool) -> OrphanIndicatorResult {
    let orphan = !is_wired && !is_reachable;
    OrphanIndicatorResult::new(
        orphan,
        "Not wired in container and unreachable.".into(),
        Severity::HIGH,
    )
}

/// Extract public struct names from Rust file content.
fn extract_struct_names(content: &str) -> Vec<String> {
    let re = Regex::new(r"(?:pub\s+)?struct\s+([A-Za-z0-9_]+)").ok();
    let mut names = Vec::new();
    if let Some(re) = re {
        for cap in re.captures_iter(content) {
            let name = cap[1].to_string();
            if name != "Self" && !name.is_empty() {
                names.push(name);
            }
        }
    }
    names
}

/// Extract public trait names from Rust file content.
fn extract_trait_names(content: &str) -> Vec<String> {
    let re = Regex::new(r"(?:pub\s+)?trait\s+([A-Za-z0-9_]+)").ok();
    let mut names = Vec::new();
    if let Some(re) = re {
        for cap in re.captures_iter(content) {
            names.push(cap[1].to_string());
        }
    }
    names
}

pub fn is_infra_cap_orphan_raw(
    f: &FilePath,
    all_files: &[String],
    is_reachable: bool,
) -> OrphanIndicatorResult {
    let fp = f.value();
    let basename = fp.split('/').next_back().unwrap_or("");
    let stem = basename.replace(".rs", "").replace(".py", "");

    let content = std::fs::read_to_string(fp).unwrap_or_default();
    let mut identifiers: Vec<String> = Vec::new();
    identifiers.extend(extract_struct_names(&content));
    identifiers.extend(extract_trait_names(&content));
    identifiers.push(stem.clone());

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
    identifiers.push(pascal_stem);

    let mut is_wired = false;
    for cf in all_files {
        let cb = cf.split('/').next_back().unwrap_or("");
        let csuffix = cb
            .rsplit('_')
            .next()
            .unwrap_or("")
            .replace(".rs", "")
            .replace(".py", "");
        if csuffix != "container" {
            continue;
        }
        if let Ok(c) = std::fs::read_to_string(cf) {
            for id in &identifiers {
                if c.contains(id) {
                    is_wired = true;
                    break;
                }
            }
            if is_wired {
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
    let content = std::fs::read_to_string(fp).unwrap_or_default();

    let mut identifiers: Vec<String> = Vec::new();
    identifiers.extend(extract_struct_names(&content));
    identifiers.extend(extract_trait_names(&content));
    identifiers.push(stem.clone());

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
    identifiers.push(pascal_stem);

    let mut wired = false;
    for cf in files {
        let cb = cf.split('/').next_back().unwrap_or("");
        let csuffix = cb
            .rsplit('_')
            .next()
            .unwrap_or("")
            .replace(".rs", "")
            .replace(".py", "");
        if csuffix != "container" {
            continue;
        }
        if let Ok(c) = std::fs::read_to_string(cf) {
            for id in &identifiers {
                if c.contains(id) {
                    wired = true;
                    break;
                }
            }
            if wired {
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
