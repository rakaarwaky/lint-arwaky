// Unit tests for ContractOrphanAnalyzer — orphan detection for contract-layer files.
use orphan_rules_lint_arwaky::capabilities_orphan_contract_analyzer::ContractOrphanAnalyzer;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::orphan_rules::{IContractOrphanProtocol, IOrphanParserProtocol};
use shared::quality_rules::taxonomy_analysis_vo::{InheritanceMap, ReachabilityResult};
use std::collections::HashSet;

fn empty_reachability() -> ReachabilityResult {
    ReachabilityResult::new(HashSet::new())
}
use std::sync::Arc;

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
    fn import_list(&self) -> Vec<ImportEntry> {
        Vec::new()
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
    fn resolve_barrel_imports(&self, _: &std::path::Path) {}
}

static EMPTY_STRING_MAP: Lazy<HashMap<String, Vec<std::path::PathBuf>>> = Lazy::new(HashMap::new);
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
    fn canonicalize(&self, path: &std::path::Path) -> Result<std::path::PathBuf, std::io::Error> {
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
    fn write_string(&self, _path: &std::path::Path, _content: &str) -> Result<(), std::io::Error> {
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
    fn discover_source_files(&self, _root: &std::path::Path, _ignored: &[String]) -> Vec<String> {
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
    fn collect_source_files(&self, _dir: &std::path::Path, _ignored: &[String]) -> Vec<FilePath> {
        vec![]
    }
    fn read_lintable_file(&self, _path: &str) -> Option<String> {
        None
    }
    fn used_identifiers_for(&self, _: &std::path::Path) -> Vec<String> {
        vec![]
    }
    fn implemented_traits_map(&self) -> HashMap<String, Vec<String>> {
        HashMap::new()
    }
    fn build_file_index(&self, _: &std::path::Path) {}
    fn build_file_index_with_ignored(&self, _: &std::path::Path, _: &[String]) {}
    fn build_orphan_graph_context(
        &self,
        _root_dir: &std::path::Path,
        _ignored: &[String],
    ) -> shared::filesystem::taxonomy_filesystem_vo::GraphAnalysisContext {
        shared::filesystem::taxonomy_filesystem_vo::GraphAnalysisContext::new(
            shared::filesystem::taxonomy_filesystem_vo::ImportGraph::new(
                std::collections::HashMap::new(),
            ),
            shared::filesystem::taxonomy_filesystem_vo::InboundLinkMap::new(
                std::collections::HashMap::new(),
            ),
            shared::filesystem::taxonomy_filesystem_vo::InheritanceMap::new(
                std::collections::HashMap::new(),
            ),
            vec![],
        )
    }
    fn find_workspace_root(&self, _: &std::path::Path) -> Option<std::path::PathBuf> {
        None
    }
}

fn reachable_for(fp: &FilePath) -> ReachabilityResult {
    ReachabilityResult::new(HashSet::from([fp.clone()]))
}

// ── Tests ───────────────────────────────────────────────

#[test]
fn test_constructor() {
    let _parser: Arc<dyn IOrphanParserProtocol> = Arc::new(MockParser);
    let fs: Arc<dyn IFilesystemAggregate> = Arc::new(MockFilesystem);
    let _analyzer = ContractOrphanAnalyzer::new(fs);
}

#[test]
fn test_empty_content_is_not_orphan() {
    let _parser: Arc<dyn IOrphanParserProtocol> = Arc::new(MockParser);
    let fs: Arc<dyn IFilesystemAggregate> = Arc::new(MockFilesystem);
    let analyzer = ContractOrphanAnalyzer::new(fs);

    let fp = FilePath::new("crates/shared/src/contract_foo_protocol.rs".to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();
    let content_map: HashMap<String, String> = HashMap::new();

    let result = analyzer.is_contract_orphan(
        &fp,
        &root,
        &InheritanceMap::new(HashMap::new()),
        &[],
        &content_map,
        &empty_reachability(),
    );
    assert!(!result.is_orphan);
}

#[test]
fn test_no_traits_is_not_orphan() {
    let _parser: Arc<dyn IOrphanParserProtocol> = Arc::new(MockParser);
    let fs: Arc<dyn IFilesystemAggregate> = Arc::new(MockFilesystem);
    let analyzer = ContractOrphanAnalyzer::new(fs);

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
        &empty_reachability(),
    );
    assert!(!result.is_orphan);
}

#[test]
fn test_trait_not_implemented_is_orphan() {
    let _parser: Arc<dyn IOrphanParserProtocol> = Arc::new(MockParser);
    let fs: Arc<dyn IFilesystemAggregate> = Arc::new(MockFilesystem);
    let analyzer = ContractOrphanAnalyzer::new(fs);

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
        &empty_reachability(),
    );
    assert!(result.is_orphan);
    assert!(result.severity == Severity::MEDIUM);
    assert!(result.reason.contains("IFooProtocol"));
}

#[test]
fn test_trait_implemented_is_not_orphan() {
    let _parser: Arc<dyn IOrphanParserProtocol> = Arc::new(MockParser);
    let fs: Arc<dyn IFilesystemAggregate> = Arc::new(MockFilesystem);
    let analyzer = ContractOrphanAnalyzer::new(fs);

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
        &reachable_for(&fp),
    );
    assert!(!result.is_orphan);
}
