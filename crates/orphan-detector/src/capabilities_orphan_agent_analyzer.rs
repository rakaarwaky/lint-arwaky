// PURPOSE: AgentOrphanAnalyzer — IAgentOrphanProtocol for detecting orphan agent files.
// AST-based: uses parser dispatch for aggregate trait extraction.

use crate::taxonomy_orphan_parse_result_vo::FileParseResultVO;
use crate::utility_orphan_parser_dispatch;
use shared::code_analysis::OrphanIndicatorResult;
use shared::common::{FilePath, Severity};
use shared::orphan_detector::{AesOrphanViolation, IAgentOrphanProtocol};

// ─── Block 1: Struct Definition ───────────────────────────

pub struct AgentOrphanAnalyzer {}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IAgentOrphanProtocol for AgentOrphanAnalyzer {
    fn is_agent_orphan(
        &self,
        f: &FilePath,
        _root_dir: &FilePath,
        all_files: &[String],
    ) -> OrphanIndicatorResult {
        let fp = f.value();
        let content = match shared::orphan_detector::utility_orphan_io::read_file_safe(fp) {
            c if c.is_empty() => {
                return OrphanIndicatorResult::new(false, String::new(), Severity::LOW)
            }
            c => c,
        };

        // AST-based aggregate trait extraction
        let aggregate_traits = Self::extract_aggregate_traits(fp, &content);
        if aggregate_traits.is_empty() {
            return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
        }

        // Pre-filter candidate files (surfaces, containers, entries, mains)
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
                    || matches!(
                        cb,
                        "main.rs"
                            | "lib.rs"
                            | "main.py"
                            | "__main__.py"
                            | "main.ts"
                            | "main.js"
                            | "index.ts"
                            | "index.js"
                    )
            })
            .collect();

        // Cache candidate file contents to avoid N×M re-reads
        let mut content_cache: std::collections::HashMap<&String, String> =
            std::collections::HashMap::new();

        let mut any_called = false;
        'outer: for agg_name in &aggregate_traits {
            for cf in &candidates {
                let c = content_cache.entry(cf).or_insert_with(|| {
                    shared::orphan_detector::utility_orphan_io::read_file_safe(cf)
                });
                if Self::content_contains_word(c, agg_name) {
                    any_called = true;
                    break 'outer;
                }
            }
        }

        if !any_called {
            return OrphanIndicatorResult::new(
                true,
                AesOrphanViolation::AgentOrphan {
                    agg_name: aggregate_traits.join(", "),
                    reason: Some(
                        format!(
                            "Agent orphan: aggregates [{}] not called by any surface.",
                            aggregate_traits.join(", ")
                        )
                        .into(),
                    ),
                }
                .to_string(),
                Severity::HIGH,
            );
        }

        OrphanIndicatorResult::new(false, String::new(), Severity::LOW)
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for AgentOrphanAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl AgentOrphanAnalyzer {
    pub fn new() -> Self {
        Self {}
    }

    fn content_contains_word(text: &str, word: &str) -> bool {
        text.split(|c: char| !c.is_alphanumeric() && c != '_')
            .any(|w| w == word)
    }

    /// Extract aggregate trait names using AST parser dispatch.
    /// Replaces 4 regex patterns (re_impl_generic, re_dyn, re_py_class, re_ts_implements).
    fn extract_aggregate_traits(file_path: &str, content: &str) -> Vec<String> {
        let mut traits = match utility_orphan_parser_dispatch::parse_file(file_path, content) {
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
