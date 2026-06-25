// PURPOSE: ContractOrphanAnalyzer — IContractOrphanProtocol for orphan contract detection
use regex::Regex;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::taxonomy_analysis_vo::FileDefinitionMap;
use shared::code_analysis::taxonomy_analysis_vo::InheritanceMap;
use shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use shared::orphan_detector::contract_orphan_protocol::IContractOrphanProtocol;
use shared::orphan_detector::taxonomy_violation_orphan_vo::AesOrphanViolation;
use shared::source_parsing::taxonomy_path_vo::FilePath;

pub struct ContractOrphanAnalyzer {}

impl Default for ContractOrphanAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl ContractOrphanAnalyzer {
    pub fn new() -> Self {
        Self {}
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
        is_contract_orphan(f, root_dir, file_definitions, inheritance_map, all_files)
    }
}

pub fn is_contract_orphan(
    f: &FilePath,
    root_dir: &FilePath,
    _file_definitions: &FileDefinitionMap,
    _inheritance_map: &InheritanceMap,
    all_files: &[String],
) -> OrphanIndicatorResult {
    let fp = f.value();
    let basename = match fp.split('/').next_back() {
        Some(b) => b,
        None => "",
    };
    let suffix = match basename.rsplit('_').next() {
        Some(s) => s,
        None => "",
    }
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

    // Build search_files: combine scan-directory files with all workspace .rs files
    let mut search_files: Vec<&str> = all_files.iter().map(|s| s.as_str()).collect();
    let root_path = std::path::Path::new(root_dir.value());
    for ws_dir in &["crates", "packages", "modules"] {
        let ws_path = root_path.join(ws_dir);
        if ws_path.exists() {
            collect_rs_files(&ws_path, &mut search_files);
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
    for cf in search_files {
        let cb = match cf.split('/').next_back() {
            Some(b) => b,
            None => continue,
        };
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
            AesOrphanViolation::ContractOrphan {
                suffix: suffix.clone(),
                trait_name: trait_name.clone(),
                target_layer: target_prefix,
                reason: Some(
                    format!(
                        "Contract {} '{}' not implemented by any {} file.",
                        suffix, trait_name, target_prefix
                    )
                    .into(),
                ),
            }
            .to_string(),
            Severity::LOW,
        );
    }

    // Check 2: port/protocol not called by any orchestrator OR container
    if suffix == "port" || suffix == "protocol" {
        let mut called_by_orchestrator_or_container = false;
        for cf in search_files {
            let cb = match cf.split('/').next_back() {
                Some(b) => b,
                None => continue,
            };
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
                AesOrphanViolation::ContractOrphan {
                    suffix: suffix.clone(),
                    trait_name: trait_name.clone(),
                    target_layer: target_prefix,
                    reason: Some(
                        format!(
                            "Contract {} '{}' not called by any orchestrator or container.",
                            suffix, trait_name
                        )
                        .into(),
                    ),
                }
                .to_string(),
                Severity::LOW,
            );
        }
    }

    // Check 3: aggregate not called by any surface OR container
    if suffix == "aggregate" {
        let mut called_by_surface_or_container = false;
        for cf in search_files {
            let cb = match cf.split('/').next_back() {
                Some(b) => b,
                None => continue,
            };
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
                AesOrphanViolation::ContractOrphan {
                    suffix: suffix.clone(),
                    trait_name: trait_name.clone(),
                    target_layer: target_prefix,
                    reason: Some(
                        format!(
                            "Contract aggregate '{}' not called by any surface or container.",
                            trait_name
                        )
                        .into(),
                    ),
                }
                .to_string(),
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
