use crate::utility_orphan_filename::{file_basename, file_suffix};
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_severity_vo::Severity;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::orphan_rules::taxonomy_orphan_parse_result_vo::FileParseResultVO;
use shared::orphan_rules::{AesOrphanViolation, IContractOrphanProtocol, IOrphanParserProtocol};
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
    pub parser_dispatcher: Arc<dyn IOrphanParserProtocol>,
}

impl ContractOrphanAnalyzer {
    pub fn new(
        parser_dispatcher: Arc<dyn IOrphanParserProtocol>,
        filesystem: Arc<dyn IFilesystemAggregate>,
    ) -> Self {
        Self {
            search_cache: Mutex::new(None),
            parser_dispatcher,
            filesystem,
        }
    }

    fn extract_trait_names(&self, file_path: &str, content: &str) -> Vec<String> {
        match self.parser_dispatcher.parse_file(file_path, content) {
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
            match self.parser_dispatcher.parse_file(cf, &content) {
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
            // Collect additional source files from workspace dirs
            for ws_dir in &["crates", "packages", "modules"] {
                let ws_path = top_root.join(ws_dir);
                if ws_path.exists() {
                    // Walk workspace dirs to collect source files
                    if let Ok(entries) = std::fs::read_dir(&ws_path) {
                        for entry in entries.flatten() {
                            let member_path = entry.path();
                            if !member_path.is_dir() {
                                continue;
                            }
                            let src_dir = member_path.join("src");
                            if src_dir.is_dir() {
                                Self::collect_source_files_recursive(&src_dir, &mut search_files);
                            }
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

    fn collect_source_files_recursive(dir: &std::path::Path, files: &mut Vec<String>) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    Self::collect_source_files_recursive(&path, files);
                } else if let Some(s) = path.to_str() {
                    if s.ends_with(".rs")
                        || s.ends_with(".py")
                        || s.ends_with(".ts")
                        || s.ends_with(".js")
                    {
                        files.push(s.to_string());
                    }
                }
            }
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
                AesOrphanViolation::ContractOrphan {
                    suffix: suffix.clone(),
                    trait_name: trait_names.join(", "),
                    target_layer: "surface",
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

        OrphanIndicatorResult::new(false, String::new(), Severity::LOW)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use once_cell::sync::Lazy;
    use shared::common::taxonomy_config_language_vo::ConfigLanguage;
    use shared::common::taxonomy_path_vo::FilePath;
    use shared::common::taxonomy_severity_vo::Severity;
    use shared::common::taxonomy_source_vo::ContentString;
    use shared::filesystem::taxonomy_filesystem_vo::*;
    use shared::orphan_rules::taxonomy_orphan_parse_result_vo::*;
    use std::collections::HashMap;

    // ── Mock Parser ─────────────────────────────────────────

    struct MockParser;

    impl IOrphanParserProtocol for MockParser {
        fn parse_file(&self, path: &str, content: &str) -> FileParseResultVO {
            if path.ends_with(".rs") {
                let mut traits = Vec::new();
                let mut trait_impls = Vec::new();
                for line in content.lines() {
                    let trimmed = line.trim();
                    if trimmed.starts_with("pub trait ") || trimmed.starts_with("trait ") {
                        let raw = trimmed
                            .trim_start_matches("pub ")
                            .trim_start_matches("trait ");
                        // Strip bounds: "IFooProtocol: Send + Sync {" → "IFooProtocol"
                        let name = if let Some(pos) = raw.find(':') {
                            raw[..pos].trim().to_string()
                        } else {
                            raw.trim_end_matches('{').trim().to_string()
                        };
                        traits.push(AstTraitDefVO {
                            name,
                            is_pub: trimmed.starts_with("pub trait"),
                        });
                    }
                    if trimmed.starts_with("impl ") && trimmed.contains("for ") {
                        let rest = trimmed.trim_start_matches("impl ");
                        if let Some(pos) = rest.find(" for ") {
                            let trait_name = rest[..pos].trim().to_string();
                            trait_impls.push(AstTraitImplVO {
                                trait_name,
                                type_name: rest[pos + 5..].trim_end_matches('{').trim().to_string(),
                                has_generics: false,
                                line: 0,
                                is_dummy: false,
                            });
                        }
                    }
                }
                FileParseResultVO::Rust(RustParseResultVO {
                    traits,
                    trait_impls,
                    ..Default::default()
                })
            } else {
                FileParseResultVO::Unsupported
            }
        }

        fn is_supported(&self, path: &str) -> bool {
            path.ends_with(".rs")
        }
    }

    // ── Mock Filesystem (stubs for IFilesystemAggregate) ────

    struct MockFilesystem;

    impl shared::filesystem::contract_parser_protocol::IParserProtocol for MockFilesystem {
        fn parse_warnings(&self) -> &[ParseWarning] {
            &[]
        }
        fn import_list(&self) -> &[ImportEntry] {
            &[]
        }
        fn parse_all(&self, _files: &mut [FileEntry]) {}
        fn imports_for(&self, _path: &std::path::Path) -> Vec<ImportEntry> {
            vec![]
        }
        fn extract(
            &self,
            _path: &std::path::Path,
            _content: &str,
            _language: shared::common::taxonomy_language_vo::Language,
        ) -> Vec<ImportEntry> {
            vec![]
        }
    }

    static EMPTY_STRING_MAP: Lazy<HashMap<String, Vec<std::path::PathBuf>>> =
        Lazy::new(HashMap::new);
    static EMPTY_PATH_MAP: Lazy<HashMap<std::path::PathBuf, Vec<std::path::PathBuf>>> =
        Lazy::new(HashMap::new);

    impl shared::filesystem::contract_graph_protocol::IGraphProtocol for MockFilesystem {
        fn build_graph(
            &self,
            _imports: &[ImportEntry],
            _files: &[FileEntry],
            _definitions: &[DefinitionEntry],
            _implementations: &[ImplEntry],
        ) {
        }
        fn symbol_definitions(&self) -> &HashMap<String, Vec<std::path::PathBuf>> {
            &EMPTY_STRING_MAP
        }
        fn implementations(&self) -> &HashMap<String, Vec<std::path::PathBuf>> {
            &EMPTY_STRING_MAP
        }
        fn dependents(&self, _path: &std::path::Path) -> Vec<std::path::PathBuf> {
            vec![]
        }
        fn dependencies(&self, _path: &std::path::Path) -> Vec<std::path::PathBuf> {
            vec![]
        }
        fn reachable(&self, _from: &std::path::Path, _to: &std::path::Path) -> bool {
            false
        }
        fn reverse_links(&self) -> &HashMap<std::path::PathBuf, Vec<std::path::PathBuf>> {
            &EMPTY_PATH_MAP
        }
    }

    impl shared::filesystem::contract_workspace_protocol::IWorkspaceProtocol for MockFilesystem {
        fn workspace_root(&self, start: &FilePath) -> Option<std::path::PathBuf> {
            std::path::Path::new(start.value())
                .parent()
                .map(|p| p.to_path_buf())
        }
        fn find_workspace_root_from_path(
            &self,
            start: &std::path::Path,
        ) -> Result<std::path::PathBuf, std::io::Error> {
            Ok(start.to_path_buf())
        }
        fn is_member_path(&self, _path: &FilePath) -> bool {
            false
        }
        fn is_leaf_member_path(&self, _path: &FilePath) -> bool {
            false
        }
        fn detect_source_dir(&self, project_root: &std::path::Path) -> std::path::PathBuf {
            project_root.join("src")
        }
        fn detect_language_from_path(&self, path: &str) -> ConfigLanguage {
            if path.ends_with(".rs") {
                ConfigLanguage::Rust
            } else if path.ends_with(".py") {
                ConfigLanguage::Python
            } else {
                ConfigLanguage::TypeScript
            }
        }
        fn check_wired_in_container(
            &self,
            _workspace_root: &std::path::Path,
            _identifiers: &[String],
        ) -> bool {
            false
        }
        fn resolve_orphan_module_path(
            &self,
            _root: &std::path::Path,
            _base_dir: &std::path::Path,
            _module_path: &str,
        ) -> Option<std::path::PathBuf> {
            None
        }
    }

    impl shared::filesystem::contract_tool_resolution_protocol::IToolResolutionProtocol
        for MockFilesystem
    {
        fn is_executable_in_path(&self, _executable: &ToolName) -> bool {
            false
        }
        fn is_binary_available(&self, _bin_name: &ToolName) -> bool {
            false
        }
        fn has_local_bin(&self, _working_dir: &std::path::Path, _executable: &ToolName) -> bool {
            false
        }
        fn resolve_js_cmd(
            &self,
            _executable: &ToolName,
            _args: Vec<String>,
            _working_dir: &FilePath,
        ) -> Option<Vec<String>> {
            None
        }
        fn resolve_js_working_dir(&self, path: &FilePath) -> FilePath {
            path.clone()
        }
        fn resolve_cargo_working_dir(&self, path: &FilePath) -> FilePath {
            path.clone()
        }
        fn resolve_cargo_lock_working_dir(&self, path: &FilePath) -> FilePath {
            path.clone()
        }
        fn has_config_file(&self, _dir: &std::path::Path) -> bool {
            false
        }
        fn has_cargo_toml(&self, _path: &FilePath) -> Option<FilePath> {
            None
        }
        fn has_cargo_lock(&self, _path: &FilePath) -> Option<FilePath> {
            None
        }
        fn is_python_file_recursive(&self, _path: &FilePath) -> bool {
            false
        }
        fn default_working_dir(&self, path: &FilePath) -> FilePath {
            path.clone()
        }
    }

    impl shared::filesystem::contract_filesystem_io_protocol::IFileSystemIOProtocol for MockFilesystem {
        fn path_exists(&self, _path: &std::path::Path) -> bool {
            false
        }
        fn is_dir(&self, _path: &std::path::Path) -> bool {
            false
        }
        fn is_file(&self, _path: &std::path::Path) -> bool {
            false
        }
        fn should_ignore(&self, _path: &FilePath, _ignored: &[String]) -> bool {
            false
        }
        fn canonicalize(
            &self,
            path: &std::path::Path,
        ) -> Result<std::path::PathBuf, std::io::Error> {
            Ok(path.to_path_buf())
        }
        fn canonicalize_path_str(&self, path: &FilePath) -> String {
            path.value.clone()
        }
        fn is_symlink(&self, _path: &std::path::Path) -> bool {
            false
        }
        fn metadata(&self, _path: &std::path::Path) -> Result<std::fs::Metadata, std::io::Error> {
            Err(std::io::Error::new(std::io::ErrorKind::NotFound, "mock"))
        }
        fn symlink_metadata(
            &self,
            _path: &std::path::Path,
        ) -> Result<std::fs::Metadata, std::io::Error> {
            Err(std::io::Error::new(std::io::ErrorKind::NotFound, "mock"))
        }
        fn get_file_stem<'a>(&self, path: &'a str) -> &'a str {
            path.rsplit('/').next().unwrap_or(path)
        }
        fn is_source_file(&self, _path: &std::path::Path) -> bool {
            false
        }
        fn is_source_ext(&self, _ext: &FileExtension) -> bool {
            false
        }
        fn get_basename<'a>(&self, path: &'a str) -> &'a str {
            path.rsplit('/').next().unwrap_or(path)
        }
        fn get_parent<'a>(&self, path: &'a str) -> &'a str {
            path.rsplit('/').nth(1).unwrap_or(path)
        }
        fn is_python_file(&self, _path: &std::path::Path) -> bool {
            false
        }
        fn scan_directory_with_ignored(
            &self,
            _dir: &std::path::Path,
            _ignored: &[String],
        ) -> Vec<std::path::PathBuf> {
            vec![]
        }
        fn is_ignored_dir(&self, _dir: &std::path::Path, _ignored: &[String]) -> bool {
            false
        }
        fn read_dir_entries_as_pathbuf(
            &self,
            _dir: &std::path::Path,
        ) -> Result<Vec<std::path::PathBuf>, std::io::Error> {
            Ok(vec![])
        }
        fn read_to_string(&self, _path: &std::path::Path) -> Result<String, std::io::Error> {
            Ok(String::new())
        }
        fn write_string(
            &self,
            _path: &std::path::Path,
            _content: &str,
        ) -> Result<(), std::io::Error> {
            Ok(())
        }
        fn copy_file(
            &self,
            _src: &std::path::Path,
            _dst: &std::path::Path,
        ) -> Result<u64, std::io::Error> {
            Ok(0)
        }
        fn create_dir_all(&self, _path: &std::path::Path) -> Result<(), std::io::Error> {
            Ok(())
        }
        fn remove_dir_all(&self, _path: &std::path::Path) -> Result<(), std::io::Error> {
            Ok(())
        }
        fn set_permissions(&self, _path: &std::path::Path, _mode: u32) -> std::io::Result<()> {
            Ok(())
        }
        fn remove_file(&self, _path: &std::path::Path) -> std::io::Result<()> {
            Ok(())
        }
        fn run_git_command(&self, _args: &[&str], _dir: &str) -> (String, String, bool) {
            (String::new(), String::new(), false)
        }
        fn parse_output_lines(&self, output: &str) -> Vec<String> {
            output.lines().map(String::from).collect()
        }
        fn run_external_command_in(
            &self,
            _name: &str,
            _args: &[&str],
            _current_dir: &str,
        ) -> (String, String, bool) {
            (String::new(), String::new(), false)
        }
        fn timing(&self) -> &ScanTiming {
            static TIMING: ScanTiming = ScanTiming {
                walk_ms: 0,
                cache_ms: 0,
                parse_ms: 0,
                extract_ms: 0,
                graph_ms: 0,
                total_ms: 0,
            };
            &TIMING
        }
    }

    impl IFilesystemAggregate for MockFilesystem {
        fn file_list(&self) -> &[FileEntry] {
            &[]
        }
        fn read_cached(&self, _path: &FilePath) -> ContentString {
            ContentString::default()
        }
        fn get_file_content(&self, _path: &std::path::Path) -> Option<String> {
            None
        }
        fn has_file(&self, _path: &std::path::Path) -> bool {
            false
        }
        fn collect_file_entries(&self, _files: &[String]) -> Vec<(std::path::PathBuf, String)> {
            vec![]
        }
        fn discover_source_files(
            &self,
            _root: &std::path::Path,
            _ignored: &[String],
        ) -> Vec<String> {
            vec![]
        }
        fn read_file(&self, _path: &std::path::Path) -> Option<String> {
            None
        }
        fn scan_directory(&self, _root: &std::path::Path) -> Vec<String> {
            vec![]
        }
        fn discover_files(&self, _root: &std::path::Path) -> Vec<String> {
            vec![]
        }
        fn collect_source_files(
            &self,
            _dir: &std::path::Path,
            _ignored: &[String],
        ) -> Vec<FilePath> {
            vec![]
        }
        fn read_lintable_file(&self, _path: &str) -> Option<String> {
            None
        }
    }

    // ── Tests ───────────────────────────────────────────────

    #[test]
    fn test_constructor() {
        let parser: Arc<dyn IOrphanParserProtocol> = Arc::new(MockParser);
        let fs: Arc<dyn IFilesystemAggregate> = Arc::new(MockFilesystem);
        let _analyzer = ContractOrphanAnalyzer::new(parser, fs);
    }

    #[test]
    fn test_empty_content_is_not_orphan() {
        let parser: Arc<dyn IOrphanParserProtocol> = Arc::new(MockParser);
        let fs: Arc<dyn IFilesystemAggregate> = Arc::new(MockFilesystem);
        let analyzer = ContractOrphanAnalyzer::new(parser, fs);

        let fp = FilePath::new("crates/shared/src/contract_foo_protocol.rs".to_string()).unwrap();
        let root = FilePath::new(".".to_string()).unwrap();
        let content_map: HashMap<String, String> = HashMap::new();

        let result = analyzer.is_contract_orphan(
            &fp,
            &root,
            &InheritanceMap::new(HashMap::new()),
            &[],
            &content_map,
        );
        assert!(!result.is_orphan);
    }

    #[test]
    fn test_no_traits_is_not_orphan() {
        let parser: Arc<dyn IOrphanParserProtocol> = Arc::new(MockParser);
        let fs: Arc<dyn IFilesystemAggregate> = Arc::new(MockFilesystem);
        let analyzer = ContractOrphanAnalyzer::new(parser, fs);

        let fp = FilePath::new("crates/shared/src/contract_foo_protocol.rs".to_string()).unwrap();
        let root = FilePath::new(".".to_string()).unwrap();
        let mut content_map = HashMap::new();
        content_map.insert(
            fp.value().to_string(),
            "use something::Foo;\nfn do_thing() {}".to_string(),
        );

        let result = analyzer.is_contract_orphan(
            &fp,
            &root,
            &InheritanceMap::new(HashMap::new()),
            &[],
            &content_map,
        );
        assert!(!result.is_orphan);
    }

    #[test]
    fn test_trait_not_implemented_is_orphan() {
        let parser: Arc<dyn IOrphanParserProtocol> = Arc::new(MockParser);
        let fs: Arc<dyn IFilesystemAggregate> = Arc::new(MockFilesystem);
        let analyzer = ContractOrphanAnalyzer::new(parser, fs);

        let fp = FilePath::new("crates/shared/src/contract_foo_protocol.rs".to_string()).unwrap();
        let root = FilePath::new(".".to_string()).unwrap();
        let mut content_map = HashMap::new();
        content_map.insert(
            fp.value().to_string(),
            "pub trait IFooProtocol: Send + Sync {\n    fn do_thing(&self);\n}".to_string(),
        );
        content_map.insert(
            "crates/shared/src/other_file.rs".to_string(),
            "fn something_else() {}".to_string(),
        );

        let all_files = vec![
            fp.value().to_string(),
            "crates/shared/src/other_file.rs".to_string(),
        ];

        let result = analyzer.is_contract_orphan(
            &fp,
            &root,
            &InheritanceMap::new(HashMap::new()),
            &all_files,
            &content_map,
        );
        assert!(result.is_orphan);
        assert!(result.severity == Severity::MEDIUM);
        assert!(result.reason.contains("IFooProtocol"));
    }

    #[test]
    fn test_trait_implemented_is_not_orphan() {
        let parser: Arc<dyn IOrphanParserProtocol> = Arc::new(MockParser);
        let fs: Arc<dyn IFilesystemAggregate> = Arc::new(MockFilesystem);
        let analyzer = ContractOrphanAnalyzer::new(parser, fs);

        let fp = FilePath::new("crates/shared/src/contract_foo_protocol.rs".to_string()).unwrap();
        let root = FilePath::new(".".to_string()).unwrap();
        let mut content_map = HashMap::new();
        content_map.insert(
            fp.value().to_string(),
            "pub trait IFooProtocol: Send + Sync {\n    fn do_thing(&self);\n}".to_string(),
        );
        content_map.insert(
            "crates/shared/src/capabilities_foo.rs".to_string(),
            "impl IFooProtocol for Foo {\n    fn do_thing(&self) {}\n}".to_string(),
        );

        let all_files = vec![
            fp.value().to_string(),
            "crates/shared/src/capabilities_foo.rs".to_string(),
        ];

        let result = analyzer.is_contract_orphan(
            &fp,
            &root,
            &InheritanceMap::new(HashMap::new()),
            &all_files,
            &content_map,
        );
        assert!(!result.is_orphan);
    }
}
