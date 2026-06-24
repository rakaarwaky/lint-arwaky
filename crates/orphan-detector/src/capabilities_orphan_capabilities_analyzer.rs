// PURPOSE: CapabilitiesOrphanAnalyzer — ICapabilitiesOrphanProtocol for orphan capability detection
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use shared::code_analysis::taxonomy_analysis_vo::ReachabilityResult;
use shared::orphan_detector::contract_orphan_protocol::ICapabilitiesOrphanProtocol;
use shared::orphan_detector::taxonomy_orphan_utility::{extract_struct_names, extract_trait_names};
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
    let basename = match std::path::Path::new(fp)
        .file_name()
        .and_then(|n| n.to_str())
    {
        Some(n) => n,
        None => "",
    };
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
                match c.next() {
                    Some(f) => f.to_uppercase().to_string() + c.as_str(),
                    None => String::new(),
                }
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
        AesOrphanViolation::CapabilitiesOrphan {
            stem,
            reason: Some("Not reachable from any entry point.".into()),
        }
        .to_string(),
        Severity::MEDIUM,
    )
}

pub fn find_workspace_root(start: &std::path::Path) -> Result<std::path::PathBuf, std::io::Error> {
    let member_dirs = ["crates", "packages", "modules"];
    let mut current = start.to_path_buf();
    loop {
        let has_cargo = current.join("Cargo.toml").exists() && current.join("Cargo.toml").is_file();
        let has_package_json =
            current.join("package.json").exists() && current.join("package.json").is_file();
        let has_pyproject =
            current.join("pyproject.toml").exists() && current.join("pyproject.toml").is_file();
        let has_member_dir = member_dirs.iter().any(|d| current.join(d).is_dir());

        if has_member_dir && (has_cargo || has_package_json || has_pyproject) {
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
    for dir_name in &["crates", "packages", "modules"] {
        let dir = workspace_root.join(dir_name);
        if dir.is_dir() && check_dir_containers(&dir, identifiers)? {
            return Ok(true);
        }
    }
    Ok(false)
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
                if name.ends_with("_container.rs")
                    || name.ends_with("_container.py")
                    || name.ends_with("_container.ts")
                    || name.ends_with("_container.js")
                {
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
pub fn is_infra_cap_orphan_raw(
    f: &FilePath,
    all_files: &[String],
    is_reachable: bool,
) -> OrphanIndicatorResult {
    let fp = f.value();
    let basename = match fp.split('/').next_back() {
        Some(b) => b,
        None => "",
    };
    let stem = basename.replace(".rs", "").replace(".py", "");

    let content = match std::fs::read_to_string(fp) {
        Ok(c) => c,
        Err(_) => String::new(),
    };
    let mut identifiers: Vec<String> = Vec::new();
    identifiers.extend(extract_struct_names(&content));
    identifiers.extend(extract_trait_names(&content));
    identifiers.push(stem.clone());

    let pascal_stem: String = stem
        .split('_')
        .filter(|s| !s.is_empty())
        .map(|s| {
            let mut c = s.chars();
            match c.next() {
                Some(f) => f.to_uppercase().to_string() + c.as_str(),
                None => String::new(),
            }
        })
        .collect();
    identifiers.push(pascal_stem);

    let mut is_wired = false;
    for cf in all_files {
        let cb = match cf.split('/').next_back() {
            Some(b) => b,
            None => "",
        };
        let csuffix = match cb.rsplit('_').next() {
            Some(s) => s,
            None => "",
        }
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
    let content = match std::fs::read_to_string(fp) {
        Ok(c) => c,
        Err(_) => String::new(),
    };

    let mut identifiers: Vec<String> = Vec::new();
    identifiers.extend(extract_struct_names(&content));
    identifiers.extend(extract_trait_names(&content));
    identifiers.push(stem.clone());

    let pascal_stem: String = stem
        .split('_')
        .filter(|s| !s.is_empty())
        .map(|s| {
            let mut c = s.chars();
            match c.next() {
                Some(f) => f.to_uppercase().to_string() + c.as_str(),
                None => String::new(),
            }
        })
        .collect();
    identifiers.push(pascal_stem);

    let mut wired = false;
    for cf in files {
        let cb = match cf.split('/').next_back() {
            Some(b) => b,
            None => "",
        };
        let csuffix = match cb.rsplit('_').next() {
            Some(s) => s,
            None => "",
        }
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
        violations.push(crate::agent_orphan_orchestrator::mk_orphan_result(
            fp,
            &AesOrphanViolation::CapabilitiesOrphan {
                stem: stem.clone(),
                reason: Some(format!("capabilities '{}' not wired in container.", stem).into()),
            }
            .to_string(),
            Severity::MEDIUM,
            "AES503",
        ));
    }
}
