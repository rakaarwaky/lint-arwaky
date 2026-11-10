use shared_common::taxonomy_path_vo::FilePath;
use shared_common::taxonomy_severity_vo::Severity;
use shared_orphan_rules::contract_orphan_protocol::IContractOrphanProtocol;
use shared_orphan_rules::taxonomy_orphan_rules_vo::FileParseResultVO;
use shared_orphan_rules::utility_orphan_filename::{
    content_contains_whole_word, file_basename, file_suffix,
};
use shared_orphan_rules::utility_orphan_paths::summarize_names;
use shared_quality_rules::taxonomy_quality_rules_vo::{
    InheritanceMap, OrphanIndicatorResult, ReachabilityResult,
};
use std::collections::HashMap;

// ─── Block 1: Module Declaration ────────────────────────────

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
}

// ─── Block 2: Module Contract ────────────────

impl IContractOrphanProtocol for ContractOrphanAnalyzer {
    /// Determines whether a contract is orphaned based on reachability and implementation status.
    ///
    /// A contract is considered reachable when it is directly reachable from an entry file or
    /// when at least one of its implementors is reachable. Protocol and aggregate contracts
    /// are orphaned when no implementation is found, unless they are re-exported from a
    /// configured barrel file.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let result = analyzer.is_contract_orphan(
    ///     &file_path,
    ///     &root_dir,
    ///     &inheritance_map,
    ///     &all_files,
    ///     &content_map,
    ///     &alive_files,
    /// );
    /// assert!(!result.is_orphan);
    /// ```
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
            return OrphanIndicatorResult::not_orphan(Severity::LOW);
        }

        let trait_names = self.extract_trait_names(fp, &content);
        if trait_names.is_empty() {
            return OrphanIndicatorResult::not_orphan(Severity::LOW);
        }

        tracing::debug!(
            trait_names = ?trait_names,
            "is_contract_orphan: extracted trait names"
        );

        // Condition 1: not reachable from any _entry file.
        // P3 (symmetric contract wiring): a contract is also considered reachable
        // when it has an alive implementor — the contract is consumed purely via DI
        // (its capabilities/agents are wired, not statically imported by entry).
        let is_reachable = is_path_alive(fp, alive_files)
            || self.has_alive_implementor(inheritance_map, &trait_names, alive_files);
        if !is_reachable {
            return OrphanIndicatorResult::orphan(
                "CONTRACT_ORPHAN",
                format!(
                    "Contract '{}' is not wired by any container.",
                    summarize_names(&trait_names, 3)
                ),
                "An unreachable contract cannot be consumed by any feature.",
                "Import and wire the contract in root_*_container.",
                Severity::MEDIUM,
            );
        }

        // Use all_files directly — orchestrator already provides full workspace file list
        let search_files: Vec<String> = all_files.to_vec();

        // Condition 2: protocol not implemented by capabilities.
        // Skip traits re-exported in a barrel file — those are intentionally
        // public API and may have their implementors in dependent crates.
        if suffix == "protocol" {
            let unimplemented: Vec<String> = trait_names
                .iter()
                .filter(|tn| {
                    !self.has_trait_implementation(&search_files, tn, content_map)
                        && !Self::is_trait_re_exported_in_barrel(
                            std::slice::from_ref(tn),
                            &search_files,
                            content_map,
                        )
                })
                .cloned()
                .collect();
            if !unimplemented.is_empty() {
                return OrphanIndicatorResult::orphan(
                    "PROTOCOL_NOT_IMPLEMENTED",
                    format!(
                        "Protocol '{}' has no capability implementing it.",
                        summarize_names(&unimplemented, 3)
                    ),
                    "A protocol without an implementor cannot be used via dependency injection.",
                    format!(
                        "Create a capability file implementing '{}'.",
                        summarize_names(&unimplemented, 3)
                    ),
                    Severity::MEDIUM,
                );
            }
        }

        // Whole-contract barrel re-export safety net: if every trait in the
        // file is re-exported in a configured barrel file, the contract is
        // intentionally public and not an orphan.
        if Self::is_trait_re_exported_in_barrel(&trait_names, &search_files, content_map) {
            return OrphanIndicatorResult::not_orphan(Severity::LOW);
        }

        // Condition 3: aggregate not implemented by agent.
        // Skip traits re-exported in a barrel file — intentionally public API.
        if suffix == "aggregate" {
            let unimplemented: Vec<String> = trait_names
                .iter()
                .filter(|tn| {
                    !self.has_trait_implementation(&search_files, tn, content_map)
                        && !Self::is_trait_re_exported_in_barrel(
                            std::slice::from_ref(tn),
                            &search_files,
                            content_map,
                        )
                })
                .cloned()
                .collect();
            if !unimplemented.is_empty() {
                return OrphanIndicatorResult::orphan(
                    "AGGREGATE_NOT_IMPLEMENTED",
                    format!(
                        "Aggregate '{}' has no agent implementing it.",
                        summarize_names(&unimplemented, 3)
                    ),
                    "An aggregate without an implementor cannot be resolved from the container.",
                    format!(
                        "Create an agent file implementing '{}'.",
                        summarize_names(&unimplemented, 3)
                    ),
                    Severity::MEDIUM,
                );
            }
        }

        OrphanIndicatorResult::not_orphan(Severity::LOW)
    }
}

// ─── Block 3: Module Helpers ────────────

use shared_orphan_rules::utility_orphan_paths::is_path_alive;

impl ContractOrphanAnalyzer {
    fn extract_trait_names(&self, file_path: &str, content: &str) -> Vec<String> {
        match shared_orphan_rules::parse_file_content(file_path, content) {
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
            match shared_orphan_rules::parse_file_content(cf, &content) {
                FileParseResultVO::Rust(result) => {
                    let has_impl = result.has_trait_impl(trait_name);
                    tracing::debug!(
                        trait_name = trait_name,
                        has_impl = has_impl,
                        impl_count = result.trait_impls.len(),
                        "has_trait_implementation check"
                    );
                    if has_impl {
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

    /// Determines whether any contract name is re-exported by a configured barrel file.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use std::path::Path;
    ///
    /// // `is_trait_re_exported_in_barrel` is a private helper on
    /// // `ContractOrphanAnalyzer`; it is exercised through the public
    /// // `IContractOrphanProtocol::is_contract_orphan` entry point.
    /// let content_map: HashMap<String, String> = HashMap::new();
    /// ```
    ///
    /// # Arguments
    ///
    /// * `trait_names` - Contract names to search for.
    /// * `search_files` - Files to inspect for configured barrel filenames.
    /// * `content_map` - File contents keyed by path.
    ///
    /// # Returns
    ///
    /// `true` if a configured barrel file contains one of the contract names as a whole word, `false` otherwise.
    fn is_trait_re_exported_in_barrel(
        trait_names: &[String],
        search_files: &[String],
        content_map: &HashMap<String, String>,
    ) -> bool {
        for cf in search_files {
            let cb = file_basename(cf);
            // Barrel file check (single source: shared_common::DEFAULT_RULE_EXCEPTIONS)
            if !shared_common::DEFAULT_RULE_EXCEPTIONS.contains(&cb.as_str()) {
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

    /// Determines whether any trait or interface has an implementation reachable from an entry point.
    ///
    /// # Parameters
    ///
    /// * `inheritance_map` - Maps each trait or interface name to its implementation files.
    /// * `trait_names` - Trait or interface names associated with the contract.
    /// * `alive_files` - Files determined to be reachable from an entry point.
    ///
    /// # Returns
    ///
    /// `true` if at least one implementation of any supplied trait or interface is reachable,
    /// `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let has_implementor = analyzer.has_alive_implementor(
    ///     &inheritance_map,
    ///     &trait_names,
    ///     &alive_files,
    /// );
    /// ```
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
