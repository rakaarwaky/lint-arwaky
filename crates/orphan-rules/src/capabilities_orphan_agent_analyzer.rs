use crate::utility_orphan_filename::content_contains_whole_word;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_severity_vo::Severity;
use shared::orphan_rules::IAgentOrphanProtocol;
use shared::orphan_rules::taxonomy_orphan_parse_result_vo::FileParseResultVO;
use shared::quality_rules::taxonomy_analysis_vo::{OrphanIndicatorResult, ReachabilityResult};
use std::collections::HashMap;

pub struct AgentOrphanAnalyzer;

impl Default for AgentOrphanAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl AgentOrphanAnalyzer {
    pub fn new() -> Self {
        Self
    }

    fn extract_aggregate_traits(&self, file_path: &str, content: &str) -> Vec<String> {
        let mut traits = match shared::common::parse_file_content(
            file_path, content,
        ) {
            FileParseResultVO::Rust(result) => result.aggregate_trait_names(),
            FileParseResultVO::Python(result) => result.aggregate_names(),
            FileParseResultVO::TypeScript(result) => result.aggregate_names(),
            FileParseResultVO::Unsupported => Vec::new(),
        };
        traits.sort();
        traits.dedup();
        traits
    }
}

impl IAgentOrphanProtocol for AgentOrphanAnalyzer {
    fn is_agent_orphan(
        &self,
        f: &FilePath,
        _root_dir: &FilePath,
        all_files: &[String],
        content_map: &HashMap<String, String>,
        alive_files: &ReachabilityResult,
    ) -> OrphanIndicatorResult {
        let fp = f.value();
        let content = match content_map.get(fp).cloned().unwrap_or_default() {
            c if c.is_empty() => {
                return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
            }
            c => c,
        };

        let aggregate_traits = self.extract_aggregate_traits(fp, &content);

        // Case 1: Agent implements aggregate traits — check if any surface references them
        if !aggregate_traits.is_empty() {
            let candidates: Vec<&String> = all_files
                .iter()
                .filter(|cf| {
                    let cb = match cf.split('/').next_back() {
                        Some(b) => b,
                        None => return false,
                    };
                    cb.starts_with("surface_")
                        || cb.ends_with("_container.rs")
                        || cb.ends_with("_container.py")
                        || cb.ends_with("_container.ts")
                        || cb.ends_with("_container.js")
                        || cb.ends_with("_entry.rs")
                        || cb.ends_with("_entry.py")
                        || cb.ends_with("_entry.ts")
                        || cb.ends_with("_entry.js")
                        || cb == "main.rs"
                        || cb == "main.py"
                        || cb == "main.ts"
                        || cb == "main.js"
                })
                .collect();

            let is_referenced = candidates.iter().any(|cf| {
                let candidate_content = content_map.get(&**cf).cloned().unwrap_or_default();
                aggregate_traits
                    .iter()
                    .any(|t| content_contains_whole_word(&candidate_content, t))
            });

            if !is_referenced {
                return OrphanIndicatorResult::new(
                    true,
                    format!(
                        "AES505 AGENT_ORPHAN: Aggregate '{}' is unreachable from any surface.\nWHY? Agent aggregate '{}' is not called by any surface or container.\nFIX: Import and use '{}' in a surface_* file or root_*_container.rs.",
                        aggregate_traits.join(", "),
                        aggregate_traits.join(", "),
                        aggregate_traits.join(", ")
                    ),
                    Severity::HIGH,
                );
            }

            return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
        }

        // Case 2: No aggregate traits — check reachability from entry points
        // alive_files contains relative paths; fp is absolute. Compare by suffix.
        let fp_path = std::path::Path::new(fp);
        let is_alive = alive_files.paths.iter().any(|af| {
            let af_val = af.value();
            // Direct match
            if af_val == fp {
                return true;
            }
            // Suffix match: alive file path ends with fp, or fp ends with alive file path
            if let Some(af_path) = std::path::Path::new(af_val).file_name() {
                if let Some(fp_file) = fp_path.file_name() {
                    if af_path == fp_file {
                        return true;
                    }
                }
            }
            // Full path suffix match
            af_val.ends_with(fp) || fp.ends_with(af_val)
        });
        if !is_alive {
            return OrphanIndicatorResult::new(
                true,
                format!(
                    "AES505 AGENT_ORPHAN: '{}' is unreachable from any entry point.\nWHY? Agent file '{}' is not reachable from any entry point (main, lib, container, surface).\nFIX: Import '{}' in a surface_* file or root_*_container.rs, or add it to lib.rs.",
                    shared::common::utility_layer_detector::extract_filename(fp),
                    shared::common::utility_layer_detector::extract_filename(fp),
                    shared::common::utility_layer_detector::extract_filename(fp)
                ),
                Severity::HIGH,
            );
        }

        OrphanIndicatorResult::new(false, String::new(), Severity::LOW)
    }
}
