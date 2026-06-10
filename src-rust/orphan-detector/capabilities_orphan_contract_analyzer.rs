// PURPOSE: ContractOrphanAnalyzer — IContractOrphanProtocol for orphan contract detection
use crate::code_analysis::taxonomy_analysis_vo::FileDefinitionMap;
use crate::code_analysis::taxonomy_analysis_vo::InheritanceMap;
use crate::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use crate::orphan_detector::contract_orphan_protocol::IContractOrphanProtocol;
use crate::output_report::taxonomy_severity_vo::Severity;
use crate::source_parsing::taxonomy_path_vo::FilePath;
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
    ) -> OrphanIndicatorResult {
        is_contract_orphan(f, root_dir, file_definitions, inheritance_map)
    }
}

pub fn is_contract_orphan(
    _f: &FilePath,
    _root_dir: &FilePath,
    _file_definitions: &FileDefinitionMap,
    _inheritance_map: &InheritanceMap,
) -> OrphanIndicatorResult {
    OrphanIndicatorResult::new(false, String::new(), Severity::LOW)
}

pub fn check_contract_orphan(
    fp: &str,
    basename: &str,
    files: &[String],
    violations: &mut Vec<crate::output_report::taxonomy_result_vo::LintResult>,
) {
    let suffix = basename
        .rsplit('_')
        .next()
        .unwrap_or("")
        .replace(".rs", "")
        .replace(".py", "");
    let target_prefix = match suffix.as_str() {
        "port" => "infrastructure",
        "protocol" => "capabilities",
        "aggregate" => "agent",
        _ => return,
    };

    // Read contract file to extract the trait/class name
    let trait_name = if let Ok(content) = std::fs::read_to_string(fp) {
        let re_rust = Regex::new(r"pub\s+trait\s+([A-Za-z0-9_]+)").unwrap();
        let re_py = Regex::new(r"class\s+([A-Za-z0-9_]+)").unwrap();
        if let Some(caps) = re_rust.captures(&content) {
            Some(caps[1].to_string())
        } else {
            re_py.captures(&content).map(|caps| caps[1].to_string())
        }
    } else {
        None
    };

    let trait_name = trait_name.unwrap_or_else(|| {
        let stem = basename
            .strip_prefix("contract_")
            .unwrap_or(basename)
            .replace(".rs", "")
            .replace(".py", "");
        stem.split('_')
            .filter(|s| !s.is_empty())
            .map(|s| {
                let mut c = s.chars();
                c.next()
                    .map(|f| f.to_uppercase().to_string() + c.as_str())
                    .unwrap_or_default()
            })
            .collect::<String>()
    });

    let mut has_impl = false;
    for cf in files {
        let cb = cf.split('/').next_back().unwrap_or("");
        if !cb.starts_with(target_prefix) {
            continue;
        }
        if let Ok(c) = std::fs::read_to_string(cf) {
            if c.contains(&format!("impl {} for", trait_name))
                || c.contains(&format!("class {}(", trait_name))
                || c.contains(&format!("class {}", trait_name))
            {
                has_impl = true;
                break;
            }
        }
    }

    if !has_impl {
        violations.push(crate::orphan_detector::mk_orphan_result(
            fp,
            &format!("Contract {} '{}' not implemented.", suffix, trait_name),
            Severity::HIGH,
        ));
    }

    if suffix == "aggregate" {
        let mut called_by_surface = false;
        for cf in files {
            let cb = cf.split('/').next_back().unwrap_or("");
            if !cb.starts_with("surface_") {
                continue;
            }
            if let Ok(c) = std::fs::read_to_string(cf) {
                let stem = basename.replace(".rs", "").replace(".py", "");
                if c.contains(&trait_name) || c.contains(&stem) {
                    called_by_surface = true;
                    break;
                }
            }
        }
        if !called_by_surface {
            violations.push(crate::orphan_detector::mk_orphan_result(
                fp,
                &format!(
                    "Contract aggregate '{}' not called by any surface.",
                    trait_name
                ),
                Severity::HIGH,
            ));
        }
    }
}
