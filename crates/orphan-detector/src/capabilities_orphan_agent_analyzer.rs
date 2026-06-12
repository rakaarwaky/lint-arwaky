// PURPOSE: AgentOrphanAnalyzer — IAgentOrphanProtocol for detecting orphan agent files
// Agent is orphan if the contract aggregate it implements is NOT called by any surface or container.
use regex::Regex;
use shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use shared::orphan_detector::contract_orphan_protocol::IAgentOrphanProtocol;
use shared::output_report::taxonomy_severity_vo::Severity;
use shared::source_parsing::taxonomy_path_vo::FilePath;

pub struct AgentOrphanAnalyzer {}

impl Default for AgentOrphanAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl AgentOrphanAnalyzer {
    pub fn new() -> Self {
        Self {}
    }
}

impl IAgentOrphanProtocol for AgentOrphanAnalyzer {
    fn is_agent_orphan(
        &self,
        f: &FilePath,
        _root_dir: &FilePath,
        all_files: &[String],
    ) -> OrphanIndicatorResult {
        is_agent_orphan_raw(f, all_files)
    }
}

pub fn is_agent_orphan(_f: &FilePath, _root_dir: &FilePath) -> OrphanIndicatorResult {
    is_agent_orphan_raw_wired(false)
}

pub fn is_agent_orphan_raw_wired(is_wired: bool) -> OrphanIndicatorResult {
    OrphanIndicatorResult::new(
        !is_wired,
        "Agent orphan: contract aggregate not called by any surface or container.".into(),
        Severity::HIGH,
    )
}

/// Extract aggregate trait names from agent file content.
/// Looks for: impl IAggregateTrait for Struct, Box<dyn IAggregateTrait>, Arc<dyn IAggregateTrait>
fn extract_aggregate_traits(content: &str) -> Vec<String> {
    let mut traits = Vec::new();

    // Rust: impl ITrait for Struct
    let re_impl = Regex::new(r"impl\s+([A-Za-z0-9_]+)\s+for\s+\s*").ok();
    if let Some(re) = re_impl {
        for cap in re.captures_iter(content) {
            let name = cap[1].to_string();
            if name.contains("Aggregate") || name.ends_with("Aggregate") {
                traits.push(name);
            }
        }
    }

    // Rust: Box<dyn ITrait> or Arc<dyn ITrait>
    let re_dyn = Regex::new(r"(?:Box|Arc)<dyn\s+([A-Za-z0-9_]+)>").ok();
    if let Some(re) = re_dyn {
        for cap in re.captures_iter(content) {
            let name = cap[1].to_string();
            if name.contains("Aggregate") || name.ends_with("Aggregate") {
                traits.push(name);
            }
        }
    }

    // Python: class Struct(ITrait):
    let re_py = Regex::new(r"class\s+\w+\(([^)]+)\)").ok();
    if let Some(re) = re_py {
        for cap in re.captures_iter(content) {
            for part in cap[1].split(',') {
                let name = part.trim().to_string();
                if name.contains("Aggregate") || name.ends_with("Aggregate") {
                    traits.push(name);
                }
            }
        }
    }

    traits.sort();
    traits.dedup();
    traits
}

pub fn is_agent_orphan_raw(f: &FilePath, all_files: &[String]) -> OrphanIndicatorResult {
    let fp = f.value();
    let content = match std::fs::read_to_string(fp) {
        Ok(c) => c,
        Err(_) => return OrphanIndicatorResult::new(false, String::new(), Severity::LOW),
    };

    // Step 1: Find aggregate traits this agent implements
    let aggregate_traits = extract_aggregate_traits(&content);
    if aggregate_traits.is_empty() {
        return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
    }

    // Step 2: Check if any of these aggregates are called by a surface file OR container
    for agg_name in &aggregate_traits {
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
                if c.contains(agg_name) {
                    called_by_surface_or_container = true;
                    break;
                }
            }
        }
        if !called_by_surface_or_container {
            return OrphanIndicatorResult::new(
                true,
                format!(
                    "Agent orphan: aggregate '{}' not called by any surface or container.",
                    agg_name
                ),
                Severity::HIGH,
            );
        }
    }

    OrphanIndicatorResult::new(false, String::new(), Severity::LOW)
}

pub fn check_agent_orphan(
    fp: &str,
    _basename: &str,
    files: &[String],
    violations: &mut Vec<output_report::taxonomy_result_vo::LintResult>,
) {
    let result = is_agent_orphan_raw(&FilePath::new(fp.to_string()).unwrap_or_default(), files);
    if result.is_orphan {
        violations.push(crate::orphan_detector::mk_orphan_result(
            fp,
            &result.reason,
            result.severity,
        ));
    }
}
