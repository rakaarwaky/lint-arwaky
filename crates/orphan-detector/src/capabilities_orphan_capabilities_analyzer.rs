// PURPOSE: CapabilitiesOrphanAnalyzer — ICapabilitiesOrphanProtocol for orphan capability detection
use regex::Regex;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use shared::code_analysis::taxonomy_analysis_vo::ReachabilityResult;
use shared::orphan_detector::contract_orphan_protocol::ICapabilitiesOrphanProtocol;
use shared::orphan_detector::taxonomy_violation_orphan_vo::AesOrphanViolation;
use shared::source_parsing::taxonomy_path_vo::FilePath;

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
        root_dir: &FilePath,
        alive_files: &ReachabilityResult,
    ) -> OrphanIndicatorResult {
        is_infra_cap_orphan(f, root_dir, alive_files)
    }
}

pub fn is_infra_cap_orphan(
    f: &FilePath,
    root_dir: &FilePath,
    alive_files: &ReachabilityResult,
) -> OrphanIndicatorResult {
    let is_reachable = alive_files.paths.contains(f);
    if is_reachable {
        return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
    }

    // Check if wired in any container
    let fp = f.value();
    let basename = std::path::Path::new(fp)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");
    let stem = basename.replace(".rs", "").replace(".py", "");

    if let Ok(content) = std::fs::read_to_string(fp) {
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

        // Search for container files in workspace root
        let root = std::path::Path::new(root_dir.value());
        if let Ok(workspace_root) = find_workspace_root(root) {
            if let Ok(wired) = check_wired_in_container(&workspace_root, &identifiers) {
                if wired {
                    return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
                }
            }
        }
    }

    OrphanIndicatorResult::new(
        true,
        AesOrphanViolation::OrphanCode {
            reason: Some("Not reachable from any entry point.".into()),
        }
        .to_string(),
        Severity::MEDIUM,
    )
}

pub fn find_workspace_root(start: &std::path::Path) -> Result<std::path::PathBuf, std::io::Error> {
    let mut current = start.to_path_buf();
    loop {
        if current.join("Cargo.toml").exists() && current.join("crates").is_dir() {
            return Ok(current);
        }
        if !current.pop() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "workspace root not found",
            ));
        }
    }
}

pub fn check_wired_in_container(
    workspace_root: &std::path::Path,
    identifiers: &[String],
) -> Result<bool, std::io::Error> {
    let crates_dir = workspace_root.join("crates");
    check_dir_containers(&crates_dir, identifiers)
}

fn check_dir_containers(
    dir: &std::path::Path,
    identifiers: &[String],
) -> Result<bool, std::io::Error> {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if check_dir_containers(&path, identifiers)? {
                    return Ok(true);
                }
            } else if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.ends_with("_container.rs") {
                    if let Ok(content) = std::fs::read_to_string(&path) {
                        for id in identifiers {
                            if content.contains(id) {
                                return Ok(true);
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(false)
}

pub fn is_infra_cap_orphan_raw_wired(is_wired: bool, is_reachable: bool) -> OrphanIndicatorResult {
    let orphan = !is_wired && !is_reachable;
    OrphanIndicatorResult::new(
        orphan,
        "Not wired in container and unreachable.".into(),
        Severity::MEDIUM,
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
    violations: &mut Vec<shared::cli_commands::taxonomy_result_vo::LintResult>,
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
        violations.push(crate::mk_orphan_result(
            fp,
            &AesOrphanViolation::OrphanCode {
                reason: Some(format!("capabilities '{}' not wired in container.", stem).into()),
            }
            .to_string(),
            Severity::MEDIUM,
            "AES503",
        ));
    }
}
