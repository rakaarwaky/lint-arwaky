use shared_common::taxonomy_path_vo::FilePath;
use shared_common::taxonomy_severity_vo::Severity;
use shared_orphan_rules::contract_orphan_protocol::IAgentOrphanProtocol;
use shared_orphan_rules::taxonomy_orphan_rules_vo::FileParseResultVO;
use shared_orphan_rules::utility_orphan_filename::{content_contains_whole_word, file_stem};
use shared_orphan_rules::utility_orphan_paths::paths_equivalent;
use shared_quality_rules::taxonomy_quality_rules_vo::{OrphanIndicatorResult, ReachabilityResult};
use std::collections::HashMap;

// ─── Block 1: Module Declaration ────────────────────────────
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
}
// ─── Block 2: Module Contract ────────────────

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
                return OrphanIndicatorResult::not_orphan(Severity::LOW);
            }
            c => c,
        };

        let aggregate_traits = self.extract_aggregate_traits(fp, &content);

        // Condition 1: not reachable from any _entry file
        let is_alive = alive_files
            .paths
            .iter()
            .any(|af| paths_equivalent(fp, af.value()));

        // Condition 2: not wired in container — check aggregate traits
        let is_wired = if !aggregate_traits.is_empty() {
            let candidates: Vec<&String> = all_files
                .iter()
                .filter(|cf| {
                    let cb = match cf.split('/').next_back() {
                        Some(b) => b,
                        None => return false,
                    };
                    cb.ends_with("_container.rs")
                        || cb.ends_with("_container.py")
                        || cb.ends_with("_container.ts")
                        || cb.ends_with("_container.js")
                        || cb == "lib.rs"
                })
                .collect();

            candidates.iter().any(|cf| {
                let candidate_content = content_map.get(&**cf).cloned().unwrap_or_default();
                aggregate_traits
                    .iter()
                    .any(|t| content_contains_whole_word(&candidate_content, t))
            })
        } else {
            true // no aggregate traits to wire → condition 2 passes vacuously
        };

        // Both conditions must be satisfied for non-orphan
        if is_alive && is_wired {
            return OrphanIndicatorResult::not_orphan(Severity::LOW);
        }

        // Build diagnostic message — one sentence ≤72 visible chars, no code prefix, no WHY:/FIX:.
        let stem = file_stem(fp);
        let message = if !is_alive && !is_wired {
            format!("'{}' is unreachable and not wired in any container.", stem)
        } else if !is_alive {
            format!("'{}' is not reachable from any entry point.", stem)
        } else {
            format!("'{}' is reachable but not wired in any container.", stem)
        };

        let why = if !is_alive && !is_wired {
            "An agent without reachability or wiring cannot coordinate feature flow.".to_string()
        } else if !is_alive {
            "An unreachable agent cannot be invoked.".to_string()
        } else {
            "A wired agent is required for dependency injection resolution.".to_string()
        };

        let fix = if !is_alive && !is_wired {
            format!(
                "Import '{}' in a surface/entry file and wire it in root_*_container.",
                stem
            )
        } else if !is_alive {
            format!("Import '{}' in a surface, capability, or entry file.", stem)
        } else {
            format!(
                "Register '{}' in root_*_container.rs or root_*_container.py.",
                stem
            )
        };

        OrphanIndicatorResult::orphan("AGENT_ORPHAN", message, why, fix, Severity::HIGH)
    }
}

// ─── Block 3: Module Helpers ────────────

impl AgentOrphanAnalyzer {
    fn extract_aggregate_traits(&self, file_path: &str, content: &str) -> Vec<String> {
        match shared_orphan_rules::parse_file_content(file_path, content) {
            FileParseResultVO::Rust(result) => result.aggregate_trait_names(),
            FileParseResultVO::Python(result) => result.aggregate_names(),
            FileParseResultVO::TypeScript(result) => result.aggregate_names(),
            FileParseResultVO::Unsupported => Vec::new(),
        }
    }
}
