// PURPOSE: ContractOrphanAnalyzer — IContractOrphanProtocol for orphan contract detection
use code_analysis::taxonomy_analysis_vo::FileDefinitionMap;
use code_analysis::taxonomy_analysis_vo::InheritanceMap;
use code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use orphan_detector::contract_orphan_protocol::IContractOrphanProtocol;
use output_report::taxonomy_severity_vo::Severity;
use source_parsing::taxonomy_path_vo::FilePath;
use regex::Regex;

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
    _root_dir: &FilePath,
    _file_definitions: &FileDefinitionMap,
    _inheritance_map: &InheritanceMap,
    all_files: &[String],
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

    // Check 1: contract not implemented by expected layer
    let target_prefix = match suffix.as_str() {
        "port" => "infrastructure",
        "protocol" => "capabilities",
        "aggregate" => "agent",
        _ => return OrphanIndicatorResult::new(false, String::new(), Severity::LOW),
    };

    let mut has_impl = false;
    for cf in all_files {
        let cb = cf.split('/').next_back().unwrap_or("");
        if !cb.starts_with(target_prefix) {
            continue;
        }
        if let Ok(c) = std::fs::read_to_string(cf) {
            if c.contains(&format!("impl {} for", trait_name))
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
            Severity::HIGH,
        );
    }

    // Check 2: port/protocol not called by any orchestrator OR container
    if suffix == "port" || suffix == "protocol" {
        let mut called_by_orchestrator_or_container = false;
        for cf in all_files {
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
                Severity::HIGH,
            );
        }
    }

    // Check 3: aggregate not called by any surface OR container
    if suffix == "aggregate" {
        let mut called_by_surface_or_container = false;
        for cf in all_files {
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
                Severity::HIGH,
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
