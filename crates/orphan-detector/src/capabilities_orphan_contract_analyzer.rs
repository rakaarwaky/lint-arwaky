// PURPOSE: ContractOrphanAnalyzer — IContractOrphanProtocol for orphan contract detection
use regex::Regex;
use shared::code_analysis::taxonomy_analysis_vo::FileDefinitionMap;
use shared::code_analysis::taxonomy_analysis_vo::InheritanceMap;
use shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_severity_vo::Severity;
use shared::orphan_detector::contract_orphan_protocol::IContractOrphanProtocol;
use shared::orphan_detector::taxonomy_violation_orphan_vo::AesOrphanViolation;
use shared::orphan_detector::utility_orphan_filename::{file_basename, file_suffix};
use shared::orphan_detector::utility_orphan_io as orphan_io;
use shared::orphan_detector::utility_workspace_scanner::collect_source_files;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::OnceLock;

// ─── Block 1: Struct Definition ───────────────────────────

#[derive(Clone)]
struct SearchFilesCache {
    root: std::path::PathBuf,
    file_count: usize,
    files: Arc<Vec<String>>,
}

impl Default for SearchFilesCache {
    fn default() -> Self {
        Self {
            root: std::path::PathBuf::new(),
            file_count: 0,
            files: Arc::new(Vec::new()),
        }
    }
}

pub struct ContractOrphanAnalyzer {
    search_cache: Mutex<Option<SearchFilesCache>>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IContractOrphanProtocol for ContractOrphanAnalyzer {
    fn is_contract_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
        _file_definitions: &FileDefinitionMap,
        _inheritance_map: &InheritanceMap,
        all_files: &[String],
    ) -> OrphanIndicatorResult {
        let fp = f.value();
        let suffix = file_suffix(fp);

        let content = orphan_io::read_file_safe(fp);
        if content.is_empty() {
            return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
        }

        // Extract ALL trait/interface names from the contract file.
        let trait_names = Self::extract_contract_trait_names(&content);
        if trait_names.is_empty() {
            return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
        }

        // Build search_files: combine scan-directory files with all workspace .rs files (cached).
        let search_files = self.cached_search_files(root_dir, all_files);

        // Check 1: contracts not implemented by expected layer.
        // For each trait, check if it's implemented by the target layer.
        let unimplemented = Self::find_unimplemented_traits(&trait_names, search_files.as_slice());
        if !unimplemented.is_empty() {
            return OrphanIndicatorResult::new(
                true,
                AesOrphanViolation::ContractOrphan {
                    suffix: suffix.clone(),
                    trait_name: unimplemented.join(", "),
                    target_layer: "expected",
                    reason: Some(
                        format!(
                            "Contract {} '{}' not implemented by any expected layer file.",
                            suffix,
                            unimplemented.join(", ")
                        )
                        .into(),
                    ),
                }
                .to_string(),
                Severity::MEDIUM,
            );
        }

        // Check 2: protocol not called by any orchestrator, container, capabilities, or surface.
        if suffix == "protocol" {
            let mut called_by_impl_or_user = false;
            for cf in search_files.as_ref() {
                let cb = file_basename(cf);
                let is_orchestrator = cb.starts_with("agent_")
                    && (cb.ends_with("_orchestrator.rs")
                        || cb.ends_with("_orchestrator.py")
                        || cb.ends_with("_orchestrator.ts")
                        || cb.ends_with("_orchestrator.js"));
                let is_container = cb.ends_with("_container.rs")
                    || cb.ends_with("_container.py")
                    || cb.ends_with("_container.ts")
                    || cb.ends_with("_container.js");
                let is_capabilities = cb.starts_with("capabilities_");
                let is_surface = cb.starts_with("surface_");

                if !is_orchestrator && !is_container && !is_capabilities && !is_surface {
                    continue;
                }
                let c = orphan_io::read_file_safe(cf);
                for trait_name in &trait_names {
                    if c.contains(trait_name.as_str()) {
                        called_by_impl_or_user = true;
                        break;
                    }
                }
                if called_by_impl_or_user {
                    break;
                }
            }
            if !called_by_impl_or_user {
                return OrphanIndicatorResult::new(
                    true,
                    AesOrphanViolation::ContractOrphan {
                        suffix: suffix.clone(),
                        trait_name: trait_names.join(", "),
                        target_layer: "orchestrator/container",
                        reason: Some(
                            format!(
                                "Contract {} '{}' not called by any orchestrator or container.",
                                suffix,
                                trait_names.join(", ")
                            )
                            .into(),
                        ),
                    }
                    .to_string(),
                    Severity::MEDIUM,
                );
            }
        }

        // Check 3: aggregate not called by any surface OR container.
        if suffix == "aggregate" {
            let mut called_by_surface_or_container = false;
            for cf in search_files.as_ref() {
                let cb = file_basename(cf);
                let is_surface = cb.starts_with("surface_");
                let is_container = cb.ends_with("_container.rs")
                    || cb.ends_with("_container.py")
                    || cb.ends_with("_container.ts")
                    || cb.ends_with("_container.js");

                if !is_surface && !is_container {
                    continue;
                }
                let c = orphan_io::read_file_safe(cf);
                for trait_name in &trait_names {
                    if c.contains(trait_name.as_str()) {
                        called_by_surface_or_container = true;
                        break;
                    }
                }
                if called_by_surface_or_container {
                    break;
                }
            }
            if !called_by_surface_or_container {
                return OrphanIndicatorResult::new(
                    true,
                    AesOrphanViolation::ContractOrphan {
                        suffix: suffix.clone(),
                        trait_name: trait_names.join(", "),
                        target_layer: "surface/container",
                        reason: Some(
                            format!(
                                "Contract aggregate '{}' not called by any surface or container.",
                                trait_names.join(", ")
                            )
                            .into(),
                        ),
                    }
                    .to_string(),
                    Severity::MEDIUM,
                );
            }
        }

        OrphanIndicatorResult::new(false, String::new(), Severity::LOW)
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for ContractOrphanAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl ContractOrphanAnalyzer {
    pub fn new() -> Self {
        Self {
            search_cache: Mutex::new(None),
        }
    }

    fn cached_search_files(&self, root_dir: &FilePath, all_files: &[String]) -> Arc<Vec<String>> {
        let root = std::path::Path::new(root_dir.value()).to_path_buf();
        if let Ok(mut guard) = self.search_cache.lock() {
            if let Some(cache) = guard.as_ref() {
                if cache.root == root && cache.file_count == all_files.len() {
                    return cache.files.clone();
                }
            }
            let mut search_files: Vec<String> = all_files.to_vec();
            for ws_dir in &["crates", "packages", "modules"] {
                let ws_path = root.join(ws_dir);
                if ws_path.exists() {
                    collect_source_files(&ws_path, &mut search_files);
                }
            }
            let files = Arc::new(search_files);
            *guard = Some(SearchFilesCache {
                root,
                file_count: all_files.len(),
                files: files.clone(),
            });
            files
        } else {
            Arc::new(all_files.to_vec())
        }
    }

    fn re_contract_rust() -> Option<&'static Regex> {
        static RE: OnceLock<Option<Regex>> = OnceLock::new();
        RE.get_or_init(|| Regex::new(r"(?:pub\s+)?trait\s+([A-Za-z0-9_]+)").ok())
            .as_ref()
    }

    fn re_contract_py() -> Option<&'static Regex> {
        static RE: OnceLock<Option<Regex>> = OnceLock::new();
        RE.get_or_init(|| Regex::new(r"class\s+([A-Za-z0-9_]+)").ok())
            .as_ref()
    }

    fn re_ts_interface_export() -> Option<&'static Regex> {
        static RE: OnceLock<Option<Regex>> = OnceLock::new();
        RE.get_or_init(|| Regex::new(r"export\s+interface\s+([A-Za-z0-9_]+)").ok())
            .as_ref()
    }

    fn re_interface() -> Option<&'static Regex> {
        static RE: OnceLock<Option<Regex>> = OnceLock::new();
        RE.get_or_init(|| Regex::new(r"interface\s+([A-Za-z0-9_]+)").ok())
            .as_ref()
    }

    /// Extract ALL trait/interface names from contract file content.
    /// Uses captures_iter to find multiple matches instead of just the first.
    fn extract_contract_trait_names(content: &str) -> Vec<String> {
        let code_lines: String = content
            .lines()
            .filter(|l| {
                let t = l.trim();
                !t.starts_with("//") && !t.starts_with("/*") && !t.starts_with("*")
            })
            .collect::<Vec<_>>()
            .join("\n");

        let mut traits = Vec::new();

        if let Some(re) = Self::re_contract_rust() {
            for caps in re.captures_iter(&code_lines) {
                traits.push(caps[1].to_string());
            }
        }
        if let Some(re) = Self::re_ts_interface_export() {
            for caps in re.captures_iter(&code_lines) {
                traits.push(caps[1].to_string());
            }
        }
        if let Some(re) = Self::re_interface() {
            for caps in re.captures_iter(&code_lines) {
                traits.push(caps[1].to_string());
            }
        }
        if let Some(re) = Self::re_contract_py() {
            for caps in re.captures_iter(&code_lines) {
                traits.push(caps[1].to_string());
            }
        }

        traits.sort();
        traits.dedup();
        traits
    }

    /// Check which traits are NOT implemented by any expected layer file.
    fn find_unimplemented_traits(trait_names: &[String], search_files: &[String]) -> Vec<String> {
        trait_names
            .iter()
            .filter(|trait_name| !Self::has_trait_implementation(search_files, trait_name))
            .cloned()
            .collect()
    }

    /// Check if any file in the search list implements the given trait.
    fn has_trait_implementation(search_files: &[String], trait_name: &str) -> bool {
        for cf in search_files {
            let c = orphan_io::read_file_safe(cf);
            if Self::check_trait_impl(&c, trait_name) {
                return true;
            }
        }
        false
    }

    /// Check if content contains an implementation of the given trait.
    fn check_trait_impl(content: &str, trait_name: &str) -> bool {
        for line in content.lines() {
            let trimmed = line.trim();
            // Skip comment lines
            if trimmed.starts_with("//")
                || trimmed.starts_with("/*")
                || trimmed.starts_with('*')
                || trimmed.starts_with("#")
            {
                continue;
            }

            // Rust: impl Trait for Type / impl<T> Trait for Type
            if trimmed.starts_with("impl")
                && trimmed.contains(" for ")
                && trimmed.contains(trait_name)
            {
                return true;
            }

            // Python: class Foo(Trait): / class Foo(Base, Trait):
            if let Some(class_pos) = trimmed.find("class ") {
                let after_class = &trimmed[class_pos + 6..];
                if let Some(paren_pos) = after_class.find('(') {
                    let bases = &after_class[paren_pos + 1..];
                    if let Some(paren_end) = bases.find(')') {
                        let base_list = &bases[..paren_end];
                        for base in base_list.split(',') {
                            let cleaned = base.trim();
                            if cleaned == trait_name {
                                return true;
                            }
                        }
                    }
                }
            }

            // TS: class Foo implements Trait
            if let Some(impl_pos) = trimmed.find(" implements ") {
                let after_impl = &trimmed[impl_pos + 12..];
                for implemented in after_impl.split(',').map(|s| s.trim()) {
                    if implemented == trait_name {
                        return true;
                    }
                }
            }
        }
        false
    }
}
