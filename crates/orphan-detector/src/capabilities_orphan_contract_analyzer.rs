use shared::code_analysis::taxonomy_analysis_vo::FileDefinitionMap;
use shared::code_analysis::taxonomy_analysis_vo::InheritanceMap;
use shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_protocol::{
    IContractOrphanProtocol, IOrphanFileCachePort, IOrphanFilenameExtractorProtocol,
};
use shared::orphan_detector::taxonomy_contract_check_utility::is_contract_orphan;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct ContractOrphanAnalyzer {
    extractor: Arc<dyn IOrphanFilenameExtractorProtocol>,
    cache: Arc<dyn IOrphanFileCachePort>,
}

// ─── Block 2: Public Contract (domain protocol ONLY) ──────
impl IContractOrphanProtocol for ContractOrphanAnalyzer {
    fn is_contract_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
        _file_definitions: &FileDefinitionMap,
        _inheritance_map: &InheritanceMap,
        all_files: &[FilePath],
    ) -> OrphanIndicatorResult {
        let search_files = self.build_search_files(all_files, root_dir);
        let mut contents: std::collections::HashMap<String, String> =
            std::collections::HashMap::new();
        let mut basenames: std::collections::HashMap<String, String> =
            std::collections::HashMap::new();

        for path in &search_files {
            if contents.contains_key(path) {
                continue;
            }
            let fp = FilePath {
                value: path.clone(),
            };
            let content = self.cache.read_cached(&fp).value;
            if !content.is_empty() {
                contents.insert(path.clone(), content.clone());
                let basename = self.extractor.file_basename(&fp).value;
                basenames.insert(path.clone(), basename);
            }
        }

        is_contract_orphan(f, &contents, &basenames, self.extractor.as_ref())
    }
}

// ─── Block 3: Constructors, Std Traits & Helpers ─────────
impl ContractOrphanAnalyzer {
    pub fn new(
        extractor: Arc<dyn IOrphanFilenameExtractorProtocol>,
        cache: Arc<dyn IOrphanFileCachePort>,
    ) -> Self {
        Self { extractor, cache }
    }

    fn build_search_files(&self, all_files: &[FilePath], root_dir: &FilePath) -> Vec<String> {
        let mut search_files: Vec<String> =
            all_files.iter().map(|fp| fp.value().to_string()).collect();
        let root_path = std::path::Path::new(root_dir.value());
        for ws_dir in &["crates", "packages", "modules"] {
            let ws_path = root_path.join(ws_dir);
            if ws_path.exists() {
                self.collect_source_files(&ws_path, &mut search_files);
            }
        }
        search_files
    }

    fn collect_source_files(&self, dir: &std::path::Path, files: &mut Vec<String>) {
        let dir_str = dir.to_str().unwrap_or("");
        let dir_fp = shared::common::taxonomy_path_vo::FilePath::new(dir_str).unwrap_or_default();
        if self.cache.is_symlink(&dir_fp).value() {
            return;
        }

        let entries = self.cache.read_dir(&dir_fp);
        for entry_path in &entries {
            let path = std::path::Path::new(entry_path.value());
            if path.is_dir() {
                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if name == "target" || name == ".git" || name == "node_modules" {
                    continue;
                }
                self.collect_source_files(path, files);
            } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if matches!(ext, "rs" | "py" | "ts" | "js" | "tsx" | "jsx") {
                    files.push(entry_path.value().to_string());
                }
            }
        }
    }
}
