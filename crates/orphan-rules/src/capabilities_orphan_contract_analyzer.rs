use crate::utility_orphan_filename::{content_contains_whole_word, file_basename, file_suffix};
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_severity_vo::Severity;
use shared::orphan_rules::contract_orphan_protocol::IContractOrphanProtocol;
use shared::orphan_rules::taxonomy_orphan_parse_result_vo::FileParseResultVO;
use shared::quality_rules::taxonomy_analysis_vo::{
    InheritanceMap, OrphanIndicatorResult, ReachabilityResult,
};
use std::collections::HashMap;

pub struct ContractOrphanAnalyzer;

impl Default for ContractOrphanAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl ContractOrphanAnalyzer {
    pub fn new() -> Self {
        Self
    }

    fn extract_trait_names(&self, file_path: &str, content: &str) -> Vec<String> {
        match shared::common::parse_file_content(file_path, content) {
            FileParseResultVO::Rust(result) => result.trait_names(),
            FileParseResultVO::Python(result) => result.class_names(),
            FileParseResultVO::TypeScript(result) => result.trait_names(),
            FileParseResultVO::Unsupported => Vec::new(),
        }
    }

    fn has_trait_implementation(
        &self,
        search_files: &[String],
        trait_name: &str,
        content_map: &HashMap<String, String>,
    ) -> bool {
        for cf in search_files {
            let content = content_map.get(cf).cloned().unwrap_or_default();
            if content.is_empty() {
                continue;
            }
            match shared::common::parse_file_content(cf, &content) {
                FileParseResultVO::Rust(result) => {
                    if result.has_trait_impl(trait_name) {
                        return true;
                    }
                }
                FileParseResultVO::Python(result) => {
                    if result
                        .class_bases
                        .iter()
                        .any(|(_, bases)| bases.iter().any(|b| b == trait_name))
                    {
                        return true;
                    }
                }
                FileParseResultVO::TypeScript(result) => {
                    if result
                        .class_implements
                        .iter()
                        .any(|(_, ifaces)| ifaces.iter().any(|i| i == trait_name))
                    {
                        return true;
                    }
                }
                FileParseResultVO::Unsupported => {}
            }
        }
        false
    }

    fn is_trait_re_exported_in_barrel(
        trait_names: &[String],
        search_files: &[String],
        content_map: &HashMap<String, String>,
    ) -> bool {
        for cf in search_files {
            let cb = file_basename(cf);
            // Barrel file check (single source: shared::common::DEFAULT_RULE_EXCEPTIONS)
            if !shared::common::DEFAULT_RULE_EXCEPTIONS.contains(&cb.as_str()) {
                continue;
            }
            let barrel_content = content_map.get(cf).cloned().unwrap_or_default();
            for trait_name in trait_names {
                if content_contains_whole_word(&barrel_content, trait_name) {
                    return true;
                }
            }
        }
        false
    }

    /// P3 (symmetric contract wiring): a contract is considered reachable when at
    /// least one of its trait/interface names has an implementor that is alive
    /// (reachable from an entry point). This closes the DI gap where a contract is
    /// consumed only via its implementors and never statically imported.
    fn has_alive_implementor(
        &self,
        inheritance_map: &InheritanceMap,
        trait_names: &[String],
        alive_files: &ReachabilityResult,
    ) -> bool {
        trait_names.iter().any(|tn| {
            inheritance_map
                .mapping
                .get(tn)
                .map(|impl_files| {
                    impl_files
                        .iter()
                        .any(|impl_rel| is_path_alive(impl_rel, alive_files))
                })
                .unwrap_or(false)
        })
    }
}

/// Robust path equality between a workspace-relative path (as stored in graph
/// maps) and the alive-set entries (absolute or relative, possibly with a `./`
/// prefix). Matches on suffix so both representations agree.
fn is_path_alive(rel: &str, alive_files: &ReachabilityResult) -> bool {
    alive_files.paths.iter().any(|af| {
        let af_val = af.value();
        af_val == rel
            || af_val.ends_with(rel)
            || rel.ends_with(af_val.trim_start_matches("./"))
            || std::path::Path::new(af_val)
                .file_name()
                .zip(std::path::Path::new(rel).file_name())
                .map(|(a, b)| a == b)
                .unwrap_or(false)
    })
}

impl IContractOrphanProtocol for ContractOrphanAnalyzer {
    fn is_contract_orphan(
        &self,
        f: &FilePath,
        _root_dir: &FilePath,
        inheritance_map: &InheritanceMap,
        all_files: &[String],
        content_map: &HashMap<String, String>,
        alive_files: &ReachabilityResult,
    ) -> OrphanIndicatorResult {
        let fp = f.value();
        let suffix = file_suffix(fp);
        let content = content_map.get(fp).cloned().unwrap_or_default();
        if content.is_empty() {
            return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
        }

        let trait_names = self.extract_trait_names(fp, &content);
        if trait_names.is_empty() {
            return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
        }

        // Condition 1: not reachable from any _entry file.
        // P3 (symmetric contract wiring): a contract is also considered reachable
        // when it has an alive implementor — the contract is consumed purely via DI
        // (its capabilities/agents are wired, not statically imported by entry).
        let is_reachable = alive_files.paths.contains(f)
            || self.has_alive_implementor(inheritance_map, &trait_names, alive_files);
        if !is_reachable {
            return OrphanIndicatorResult::new(
                true,
                format!(
                    "AES502 CONTRACT_ORPHAN: Contract {} '{}' is not reachable.\nWHY? Contract {} '{}' is not reachable from any _entry file.\nFIX: Import '{}' from a _entry file.",
                    suffix,
                    trait_names.join(", "),
                    suffix,
                    trait_names.join(", "),
                    trait_names.join(", ")
                ),
                Severity::MEDIUM,
            );
        }

        // Use all_files directly — orchestrator already provides full workspace file list
        let search_files: Vec<String> = all_files.to_vec();

        if Self::is_trait_re_exported_in_barrel(&trait_names, &search_files, content_map) {
            return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
        }

        // Condition 2: protocol not implemented by capabilities
        if suffix == "protocol" {
            let unimplemented: Vec<String> = trait_names
                .iter()
                .filter(|tn| !self.has_trait_implementation(&search_files, tn, content_map))
                .cloned()
                .collect();
            if !unimplemented.is_empty() {
                return OrphanIndicatorResult::new(
                    true,
                    format!(
                        "AES502 CONTRACT_ORPHAN: Contract protocol '{}' is not implemented.\nWHY? Contract protocol '{}' is not implemented by any capabilities_* file.\nFIX: Implement '{}' in a capabilities_* file.",
                        unimplemented.join(", "),
                        unimplemented.join(", "),
                        unimplemented.join(", ")
                    ),
                    Severity::MEDIUM,
                );
            }
        }

        // Condition 3: aggregate not implemented by agent
        if suffix == "aggregate" {
            let unimplemented: Vec<String> = trait_names
                .iter()
                .filter(|tn| !self.has_trait_implementation(&search_files, tn, content_map))
                .cloned()
                .collect();
            if !unimplemented.is_empty() {
                return OrphanIndicatorResult::new(
                    true,
                    format!(
                        "AES502 CONTRACT_ORPHAN: Contract aggregate '{}' is not implemented.\nWHY? Contract aggregate '{}' is not implemented by any agent_* file.\nFIX: Implement '{}' in an agent_* file.",
                        unimplemented.join(", "),
                        unimplemented.join(", "),
                        unimplemented.join(", ")
                    ),
                    Severity::MEDIUM,
                );
            }
        }

        OrphanIndicatorResult::new(false, String::new(), Severity::LOW)
    }
}
