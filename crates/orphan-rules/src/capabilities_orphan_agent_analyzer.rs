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

        // Condition 1: not reachable from any _entry file
        let fp_path = std::path::Path::new(fp);
        let is_alive = alive_files.paths.iter().any(|af| {
            let af_val = af.value();
            if af_val == fp {
                return true;
            }
            if let Some(af_path) = std::path::Path::new(af_val).file_name() {
                if let Some(fp_file) = fp_path.file_name() {
                    if af_path == fp_file {
                        return true;
                    }
                }
            }
            af_val.ends_with(fp) || fp.ends_with(af_val)
        });
        if !is_alive {
            return OrphanIndicatorResult::new(
                true,
                format!(
                    "AES505 AGENT_ORPHAN: '{}' is not reachable.\nWHY? Agent file '{}' is not reachable from any _entry file.\nFIX: Import '{}' from a _entry file.",
                    shared::common::utility_layer_detector::extract_filename(fp),
                    shared::common::utility_layer_detector::extract_filename(fp),
                    shared::common::utility_layer_detector::extract_filename(fp)
                ),
                Severity::HIGH,
            );
        }

        // Condition 2: not wired in container — check aggregate traits
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
                        "AES505 AGENT_ORPHAN: '{}' is not wired.\nWHY? Agent file '{}' is not wired in any root_*_container file.\nFIX: Import '{}' in a root_*_container.rs.",
                        shared::common::utility_layer_detector::extract_filename(fp),
                        shared::common::utility_layer_detector::extract_filename(fp),
                        shared::common::utility_layer_detector::extract_filename(fp)
                    ),
                    Severity::HIGH,
                );
            }
        }

        OrphanIndicatorResult::new(false, String::new(), Severity::LOW)
    }
}
