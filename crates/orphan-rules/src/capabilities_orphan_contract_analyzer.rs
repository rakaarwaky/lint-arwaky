use crate::utility_orphan_filename::{file_basename, file_suffix};
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_severity_vo::Severity;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::orphan_rules::taxonomy_orphan_parse_result_vo::FileParseResultVO;
use shared::orphan_rules::IContractOrphanProtocol;
use shared::quality_rules::taxonomy_analysis_vo::{InheritanceMap, OrphanIndicatorResult};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

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
    pub filesystem: Arc<dyn IFilesystemAggregate>,
    search_cache: Mutex<Option<SearchFilesCache>>,
}

impl ContractOrphanAnalyzer {
    pub fn new(filesystem: Arc<dyn IFilesystemAggregate>) -> Self {
        Self {
            search_cache: Mutex::new(None),
            filesystem,
        }
    }

    fn extract_trait_names(&self, file_path: &str, content: &str) -> Vec<String> {
        match shared::orphan_rules::taxonomy_parser_dispatcher::parse_file_content(
            file_path, content,
        ) {
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
            match shared::orphan_rules::taxonomy_parser_dispatcher::parse_file_content(cf, &content)
            {
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

    fn is_referenced_by_layers(
        trait_names: &[String],
        search_files: &[String],
        prefix_patterns: &[&str],
        suffix_patterns: &[&str],
        content_map: &HashMap<String, String>,
    ) -> bool {
        for cf in search_files {
            let cb = file_basename(cf);
            let matches_prefix = prefix_patterns.iter().any(|p| cb.starts_with(p));
            let matches_suffix = suffix_patterns.iter().any(|s| cb.ends_with(s));
            if !matches_prefix && !matches_suffix {
                continue;
            }
            let content = content_map.get(cf).cloned().unwrap_or_default();
            for trait_name in trait_names {
                if Self::content_contains_word(&content, trait_name) {
                    return true;
                }
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
            let is_barrel = matches!(
                cb.as_str(),
                "__init__.py" | "mod.rs" | "index.ts" | "index.js"
            );
            if !is_barrel {
                continue;
            }
            let barrel_content = content_map.get(cf).cloned().unwrap_or_default();
            for trait_name in trait_names {
                if Self::content_contains_word(&barrel_content, trait_name) {
                    return true;
                }
            }
        }
        false
    }

    fn content_contains_word(text: &str, word: &str) -> bool {
        text.split(|c: char| !c.is_alphanumeric() && c != '_')
            .any(|w| w == word)
    }

    fn cached_search_files(&self, root_dir: &FilePath, all_files: &[String]) -> Arc<Vec<String>> {
        let root = std::path::Path::new(root_dir.value()).to_path_buf();
        let top_root = self
            .filesystem
            .find_workspace_root_from_path(&root)
            .unwrap_or_else(|_| root.clone());
        if let Ok(mut guard) = self.search_cache.lock() {
            if let Some(cache) = guard.as_ref()
                && cache.root == top_root
                && cache.file_count == all_files.len()
            {
                return cache.files.clone();
            }
            let mut search_files: Vec<String> = all_files.to_vec();
            // Collect additional source files from workspace dirs via filesystem
            let ignored: Vec<String> = vec![
                "target".into(),
                "node_modules".into(),
                ".git".into(),
                "dist".into(),
                "build".into(),
                "__pycache__".into(),
                ".venv".into(),
                "tests".into(),
            ];
            for ws_dir in &["crates", "packages", "modules"] {
                let ws_path = top_root.join(ws_dir);
                if self.filesystem.is_dir(&ws_path) {
                    let discovered = self.filesystem.discover_source_files(&ws_path, &ignored);
                    for f in discovered {
                        if search_files.iter().all(|existing| existing != &f) {
                            search_files.push(f);
                        }
                    }
                }
            }
            let files = Arc::new(search_files);
            *guard = Some(SearchFilesCache {
                root: top_root,
                file_count: all_files.len(),
                files: files.clone(),
            });
            files
        } else {
            Arc::new(all_files.to_vec())
        }
    }
}

impl IContractOrphanProtocol for ContractOrphanAnalyzer {
    fn is_contract_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
        _inheritance_map: &InheritanceMap,
        all_files: &[String],
        content_map: &HashMap<String, String>,
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

        let search_files = self.cached_search_files(root_dir, all_files);

        if Self::is_trait_re_exported_in_barrel(&trait_names, &search_files, content_map) {
            return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
        }

        let unimplemented: Vec<String> = trait_names
            .iter()
            .filter(|tn| !self.has_trait_implementation(&search_files, tn, content_map))
            .cloned()
            .collect();
        if !unimplemented.is_empty() {
            return OrphanIndicatorResult::new(
                true,
                format!(
                    "AES502 CONTRACT_ORPHAN: Contract {} '{}' is orphaned.\nWHY? Contract {} '{}' is not implemented by any {} file.\nFIX: Implement '{}' in a capabilities_* file.",
                    suffix, unimplemented.join(", "), suffix, unimplemented.join(", "), "expected", unimplemented.join(", ")
                ),
                Severity::MEDIUM,
            );
        }

        if suffix == "protocol"
            && !Self::is_referenced_by_layers(
                &trait_names,
                &search_files,
                &["agent_", "capabilities_", "surface_"],
                &[
                    "_container.rs",
                    "_container.py",
                    "_container.ts",
                    "_container.js",
                ],
                content_map,
            )
        {
            return OrphanIndicatorResult::new(
                true,
                format!(
                    "AES502 CONTRACT_ORPHAN: Contract {} '{}' is orphaned.\nWHY? Contract {} '{}' is not implemented by any {} file.\nFIX: Implement '{}' in a capabilities_* file.",
                    suffix, trait_names.join(", "), suffix, trait_names.join(", "), "orchestrator/container", trait_names.join(", ")
                ),
                Severity::MEDIUM,
            );
        }

        if suffix == "aggregate"
            && !Self::is_referenced_by_layers(
                &trait_names,
                &search_files,
                &["surface_"],
                &[
                    "_container.rs",
                    "_container.py",
                    "_container.ts",
                    "_container.js",
                ],
                content_map,
            )
        {
            return OrphanIndicatorResult::new(
                true,
                format!(
                    "AES502 CONTRACT_ORPHAN: Contract {} '{}' is orphaned.\nWHY? Contract {} '{}' is not implemented by any {} file.\nFIX: Implement '{}' in a capabilities_* file.",
                    suffix, trait_names.join(", "), suffix, trait_names.join(", "), "surface", trait_names.join(", ")
                ),
                Severity::MEDIUM,
            );
        }

        OrphanIndicatorResult::new(false, String::new(), Severity::LOW)
    }
}
