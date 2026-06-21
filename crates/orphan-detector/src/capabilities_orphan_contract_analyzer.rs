// PURPOSE: ContractOrphanAnalyzer — IContractOrphanProtocol for orphan contract detection
use regex::Regex;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::taxonomy_analysis_vo::FileDefinitionMap;
use shared::code_analysis::taxonomy_analysis_vo::InheritanceMap;
use shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use shared::orphan_detector::contract_orphan_protocol::IContractOrphanProtocol;
use shared::source_parsing::taxonomy_path_vo::FilePath;

pub struct ContractOrphanAnalyzer {
    ignored_paths: Vec<String>,
}

impl Default for ContractOrphanAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl ContractOrphanAnalyzer {
    pub fn new() -> Self {
        Self {
            ignored_paths: Vec::new(),
        }
    }

    pub fn with_ignored_paths(ignored_paths: Vec<String>) -> Self {
        Self { ignored_paths }
    }
}

impl IContractOrphanProtocol for ContractOrphanAnalyzer {
    fn is_contract_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
        file_definitions: &FileDefinitionMap,
        inheritance_map: &InheritanceMap,
        all_files: &[String],
    ) -> OrphanIndicatorResult {
        is_contract_orphan(
            f,
            root_dir,
            file_definitions,
            inheritance_map,
            all_files,
            &self.ignored_paths,
        )
    }
}

fn find_workspace_root(start_path: &str) -> Option<std::path::PathBuf> {
    let mut current = std::path::Path::new(start_path);
    if current.is_file() {
        current = current.parent()?;
    }
    while let Some(parent) = current.parent() {
        let cargo_toml = current.join("Cargo.toml");
        if cargo_toml.exists() {
            if let Ok(content) = std::fs::read_to_string(&cargo_toml) {
                if content.contains("[workspace]") {
                    return Some(current.to_path_buf());
                }
            }
        }
        current = parent;
    }
    let cargo_toml = current.join("Cargo.toml");
    if cargo_toml.exists() {
        if let Ok(content) = std::fs::read_to_string(&cargo_toml) {
            if content.contains("[workspace]") {
                return Some(current.to_path_buf());
            }
        }
    }
    None
}

fn collect_all_workspace_files(dir: &std::path::Path, ignored_paths: &[String]) -> Vec<String> {
    let mut files = Vec::new();
    if dir.is_dir() {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let dir_name = path.file_name().unwrap_or_default().to_string_lossy();
                    let mut is_ignored = false;
                    for i in ignored_paths {
                        let clean = i.trim_start_matches('/');
                        if dir_name == clean || path.to_string_lossy().contains(i) {
                            is_ignored = true;
                            break;
                        }
                    }
                    if is_ignored {
                        continue;
                    }
                    files.extend(collect_all_workspace_files(&path, ignored_paths));
                } else if let Some(ext) = path.extension() {
                    let ext_str = ext.to_string_lossy();
                    if matches!(ext_str.as_ref(), "rs" | "py" | "js" | "ts") {
                        files.push(path.to_string_lossy().to_string());
                    }
                }
            }
        }
    }
    files
}

pub fn is_contract_orphan(
    f: &FilePath,
    _root_dir: &FilePath,
    _file_definitions: &FileDefinitionMap,
    _inheritance_map: &InheritanceMap,
    all_files: &[String],
    ignored_paths: &[String],
) -> OrphanIndicatorResult {
    let fp = f.value();
    let basename = fp.split('/').next_back().unwrap_or("");
    let suffix = basename
        .rsplit('_')
        .next()
        .unwrap_or("")
        .replace(".rs", "")
        .replace(".py", "")
        .replace(".ts", "")
        .replace(".js", "");

    let content = match std::fs::read_to_string(fp) {
        Ok(c) => c,
        Err(_) => return OrphanIndicatorResult::new(false, String::new(), Severity::LOW),
    };

    let trait_name = extract_contract_trait_name(&content);
    let trait_name = match trait_name {
        Some(t) => t,
        None => return OrphanIndicatorResult::new(false, String::new(), Severity::LOW),
    };

    let mut search_files: Vec<String> = all_files.to_vec();
    if let Some(ws_root) = find_workspace_root(fp) {
        let ws_files = collect_all_workspace_files(&ws_root, ignored_paths);
        for wf in ws_files {
            if !search_files.contains(&wf) {
                search_files.push(wf);
            }
        }
    }

    // Check 1: contract not implemented by expected layer
    let target_prefix = match suffix.as_str() {
        "port" => "infrastructure",
        "protocol" => "capabilities",
        "aggregate" => "agent",
        _ => return OrphanIndicatorResult::new(false, String::new(), Severity::LOW),
    };

    let mut has_impl = false;
    for cf in &search_files {
        let cb = cf.split('/').next_back().unwrap_or("");
        if !cb.starts_with(target_prefix) {
            continue;
        }
        if let Ok(c) = std::fs::read_to_string(cf) {
            if c.contains(&format!("impl {} for", trait_name))
                || c.lines().any(|ln| {
                    let t = ln.trim();
                    t.starts_with("impl") && t.contains(&trait_name) && t.contains(" for")
                })
                || c.contains(&format!("class {}(\\(", trait_name))
                || c.contains(&format!("class {} ", trait_name))
                || c.contains(&format!("class {}:", trait_name))
            {
                has_impl = true;
                break;
            }
        }
    }

    if !has_impl {
        return OrphanIndicatorResult::new(
            true,
            format!("Contract {} '{}' not implemented.", suffix, trait_name),
            Severity::LOW,
        );
    }

    // Check 2: port/protocol not called by any orchestrator OR container
    if suffix == "port" || suffix == "protocol" {
        let mut called_by_orchestrator_or_container = false;
        for cf in &search_files {
            let cb = cf.split('/').next_back().unwrap_or("");
            // Check orchestrator files
            let is_orchestrator = cb.starts_with("agent_")
                && (cb.ends_with("_orchestrator.rs")
                    || cb.ends_with("_orchestrator.py")
                    || cb.ends_with("_orchestrator.ts")
                    || cb.ends_with("_orchestrator.js"));
            // Check container files (DI wiring)
            let is_container = cb.ends_with("_container.rs");

            if !is_orchestrator && !is_container {
                continue;
            }
            if let Ok(c) = std::fs::read_to_string(cf) {
                if c.contains(&trait_name) {
                    called_by_orchestrator_or_container = true;
                    break;
                }
            }
        }
        if !called_by_orchestrator_or_container {
            return OrphanIndicatorResult::new(
                true,
                format!(
                    "Contract {} '{}' not called by any orchestrator or container.",
                    suffix, trait_name
                ),
                Severity::LOW,
            );
        }
    }

    // Check 3: aggregate not called by any surface OR container
    if suffix == "aggregate" {
        let mut called_by_surface_or_container = false;
        for cf in &search_files {
            let cb = cf.split('/').next_back().unwrap_or("");
            // Check surface files
            let is_surface = cb.starts_with("surface_");
            // Check container files (DI wiring)
            let is_container = cb.ends_with("_container.rs");

            if !is_surface && !is_container {
                continue;
            }
            if let Ok(c) = std::fs::read_to_string(cf) {
                if c.contains(&trait_name) {
                    called_by_surface_or_container = true;
                    break;
                }
            }
        }
        if !called_by_surface_or_container {
            return OrphanIndicatorResult::new(
                true,
                format!(
                    "Contract aggregate '{}' not called by any surface or container.",
                    trait_name
                ),
                Severity::LOW,
            );
        }
    }

    OrphanIndicatorResult::new(false, String::new(), Severity::LOW)
}

fn extract_contract_trait_name(content: &str) -> Option<String> {
    let re_rust = Regex::new(r"pub\s+trait\s+([A-Za-z0-9_]+)").ok()?;
    let re_py = Regex::new(r"class\s+([A-Za-z0-9_]+)").ok()?;
    let re_ts_interface = Regex::new(r"export\s+interface\s+([A-Za-z0-9_]+)").ok()?;
    let re_interface = Regex::new(r"interface\s+([A-Za-z0-9_]+)").ok()?;

    if let Some(caps) = re_rust.captures(content) {
        return Some(caps[1].to_string());
    }
    if let Some(caps) = re_ts_interface.captures(content) {
        return Some(caps[1].to_string());
    }
    if let Some(caps) = re_interface.captures(content) {
        return Some(caps[1].to_string());
    }
    re_py.captures(content).map(|caps| caps[1].to_string())
}
