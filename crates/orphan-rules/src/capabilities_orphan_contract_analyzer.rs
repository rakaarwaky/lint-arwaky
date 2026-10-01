use shared_common::taxonomy_path_vo::FilePath;
use shared_common::taxonomy_severity_vo::Severity;
use shared_orphan_rules::contract_orphan_protocol::IContractOrphanProtocol;
use shared_orphan_rules::taxonomy_orphan_rules_vo::FileParseResultVO;
use shared_orphan_rules::utility_orphan_filename::{
    content_contains_whole_word, file_basename, file_suffix,
};
use shared_quality_rules::taxonomy_quality_rules_vo::{
    InheritanceMap, OrphanIndicatorResult, ReachabilityResult,
};
use std::collections::HashMap;

pub struct ContractOrphanAnalyzer;
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
            return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
        }

        let trait_names = self.extract_trait_names(fp, &content);
        if trait_names.is_empty() {
            return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
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
            tracing::debug!(
                unimplemented = ?unimplemented,
                "is_contract_orphan: unimplemented protocols"
            );
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

        // Whole-contract barrel re-export safety net: if every trait in the
        // file is re-exported in a configured barrel file, the contract is
        // intentionally public and not an orphan.
        if Self::is_trait_re_exported_in_barrel(&trait_names, &search_files, content_map) {
            return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
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

/// Normalizes a workspace-relative path for comparison.
///
/// Strips a leading `./` prefix and converts backslashes to forward slashes so
/// that paths originating from different sources compare consistently.
pub fn normalize_rel_path(p: &str) -> String {
    p.trim_start_matches("./").replace('\\', "/")
}

/// Determines whether two workspace-relative paths refer to the same file.
///
/// Both inputs use the same `path_to_relative` format (workspace-relative,
/// forward slashes). Matching is strict: exact equality after normalization, or a
/// suffix match that begins at a path-separator boundary. Bare-basename matching
/// is intentionally avoided so that common names such as `mod.rs`, `index.ts`, or
/// `lib.rs` in one module cannot validate an unrelated file in another module.
pub fn paths_equivalent(rel: &str, alive: &str) -> bool {
    let a = normalize_rel_path(rel);
    let b = normalize_rel_path(alive);
    if a == b {
        return true;
    }
    // Suffix match only at a separator boundary. A path without any directory
    // component (bare basename) is only ever equal — never a suffix match — so
    // a bare `lib.rs` cannot validate an unrelated deeper `lib.rs`.
    if a.contains('/') {
        if let Some(rest) = b.strip_suffix(&a) {
            if rest.is_empty() || rest.ends_with('/') {
                return true;
            }
        }
    }
    if b.contains('/') {
        if let Some(rest) = a.strip_suffix(&b) {
            if rest.is_empty() || rest.ends_with('/') {
                return true;
            }
        }
    }
    false
}

/// Determines whether a workspace-relative path corresponds to any reachable file path.
///
/// Paths may be relative or absolute and may include a `./` prefix. Matching uses
/// [`paths_equivalent`], so common file names in different modules never collide.
///
/// # Examples
///
/// ```rust,ignore
/// assert!(is_path_alive("src/lib.rs", &alive_files));
/// ```
///
/// # Arguments
///
/// * `rel` - Workspace-relative path to compare.
/// * `alive_files` - Reachability results containing paths known to be reachable.
///
/// # Returns
///
/// `true` if a reachable path matches the supplied path, `false` otherwise.
pub fn is_path_alive(rel: &str, alive_files: &ReachabilityResult) -> bool {
    alive_files
        .paths
        .iter()
        .any(|af| paths_equivalent(rel, af.value()))
}

impl ContractOrphanAnalyzer {
    fn extract_trait_names(&self, file_path: &str, content: &str) -> Vec<String> {
        match shared_common::parse_file_content(file_path, content) {
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
            match shared_common::parse_file_content(cf, &content) {
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

impl ContractOrphanAnalyzer {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ContractOrphanAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
