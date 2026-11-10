//! Unit tests for ImportOrchestrator — error handling, disabled rules, and skip records.
//!
//! These tests were formerly inline `#[cfg(test)]` in `agent_import_orchestrator.rs` (Block 3,
//! lines 506–1267). They moved here per AES103 (test files live in `tests/` with a legal prefix).

use import_rules_lint_arwaky::agent_import_orchestrator::ImportOrchestrator;
use import_rules_lint_arwaky::agent_import_orchestrator::ImportOrchestratorDeps;
use import_rules_lint_arwaky::capabilities_cycle_import_analyzer::DependencyCycleAnalyzer;
use import_rules_lint_arwaky::capabilities_dummy_import_checker::DummyImportChecker;
use import_rules_lint_arwaky::capabilities_import_forbidden_checker::{
    ArchImportForbiddenChecker, build_test_import_entries,
};
use import_rules_lint_arwaky::capabilities_import_mandatory_checker::ArchImportMandatoryChecker;
use import_rules_lint_arwaky::capabilities_import_unused_checker::UnusedImportRuleChecker;
use shared_cli_commands::LintResult;
use shared_common::taxonomy_adapter_name_vo::ToolName;
use shared_common::taxonomy_language_vo::ConfigLanguage;
use shared_common::taxonomy_layer_vo::Identity;
use shared_common::taxonomy_lint_vo::LintResultList;
use shared_common::taxonomy_message_vo::{DescriptionVO, LintMessage};
use shared_common::{
    BooleanVO, ContentString, Count, ErrorCode, ErrorMessage, FilePath, FilePathList,
    LayerDefinition, LayerMapVO, LayerNameVO, NamingConfig, PatternList, Severity, SymbolName,
};
use shared_config_system::ArchitectureConfig;
use shared_config_system::taxonomy_config_system_vo::ArchitectureRule;
use shared_filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared_filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use shared_filesystem::contract_filesystem_protocol::IParserProtocol;
use shared_filesystem::contract_filesystem_protocol::IWorkspaceProtocol;
use shared_filesystem::taxonomy_filesystem_request::FilesystemRequest;
use shared_filesystem::taxonomy_filesystem_response::FilesystemResponse;
use shared_filesystem::taxonomy_filesystem_vo::{
    ByteCount, FileEntry, FileExtension, FileMode, GitCommandResult, ImportEntry, Language,
    ParseWarning, ParsedLines, ProjectLanguagesVO, ScanTiming,
};
use shared_import_rules::contract_import_protocol::{
    ICycleImportProtocol, IDummyImportCheckerProtocol, IImportForbiddenProtocol,
    IImportMandatoryProtocol, IUnusedImportProtocol,
};
use shared_import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use shared_import_rules::taxonomy_import_rules_constant::{
    AES200_RULE_CODE, AES201_RULE_CODE, AES202_RULE_CODE, AES203_RULE_CODE, AES204_RULE_CODE,
    AES205_RULE_CODE, VIOLATION_NAME_RULE_SKIP, VIOLATION_NAME_RULE_UNAVAILABLE,
};
use shared_import_rules::taxonomy_import_rules_error::ImportError;
use shared_import_rules::taxonomy_import_rules_request::ImportRequest;
use shared_import_rules::taxonomy_import_rules_response::ImportResponse;
use shared_import_rules::taxonomy_import_rules_vo::DependencyEdge;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

fn test_config() -> ArchitectureConfig {
    let mut layers = HashMap::new();
    layers.insert(
        LayerNameVO::new("capabilities"),
        LayerDefinition {
            forbidden: PatternList::new(vec!["agent", "surface"]),
            allowed: PatternList::new(vec!["taxonomy", "contract", "utility"]),
            ..Default::default()
        },
    );
    layers.insert(
        LayerNameVO::new("taxonomy"),
        LayerDefinition {
            forbidden: PatternList::new(vec!["capabilities", "agent", "surface"]),
            allowed: PatternList::new(vec!["utility"]),
            ..Default::default()
        },
    );
    ArchitectureConfig::new(
        BooleanVO::new(true),
        layers,
        vec![
            ArchitectureRule {
                name: DescriptionVO::new(AES201_RULE_CODE),
                description: DescriptionVO::new(""),
                rule_type: ErrorCode::raw(AES201_RULE_CODE),
                enabled: BooleanVO::new(true),
                scope: LayerNameVO::new("capabilities"),
                exceptions: PatternList::default(),
                allowed: PatternList::default(),
                forbidden: PatternList::default(),
                mandatory: PatternList::default(),
                naming: Default::default(),
                code_analysis: Default::default(),
                role: Default::default(),
                orphan: Default::default(),
            },
            ArchitectureRule {
                name: DescriptionVO::new(AES202_RULE_CODE),
                description: DescriptionVO::new(""),
                rule_type: ErrorCode::raw(AES202_RULE_CODE),
                enabled: BooleanVO::new(true),
                scope: LayerNameVO::new("capabilities"),
                exceptions: PatternList::default(),
                allowed: PatternList::default(),
                forbidden: PatternList::default(),
                mandatory: PatternList::default(),
                naming: Default::default(),
                code_analysis: Default::default(),
                role: Default::default(),
                orphan: Default::default(),
            },
            ArchitectureRule {
                name: DescriptionVO::new(AES203_RULE_CODE),
                description: DescriptionVO::new(""),
                rule_type: ErrorCode::raw(AES203_RULE_CODE),
                enabled: BooleanVO::new(true),
                scope: LayerNameVO::new("capabilities"),
                exceptions: PatternList::default(),
                allowed: PatternList::default(),
                forbidden: PatternList::default(),
                mandatory: PatternList::default(),
                naming: Default::default(),
                code_analysis: Default::default(),
                role: Default::default(),
                orphan: Default::default(),
            },
            ArchitectureRule {
                name: DescriptionVO::new(AES204_RULE_CODE),
                description: DescriptionVO::new(""),
                rule_type: ErrorCode::raw(AES204_RULE_CODE),
                enabled: BooleanVO::new(true),
                scope: LayerNameVO::new("capabilities"),
                exceptions: PatternList::default(),
                allowed: PatternList::default(),
                forbidden: PatternList::default(),
                mandatory: PatternList::default(),
                naming: Default::default(),
                code_analysis: Default::default(),
                role: Default::default(),
                orphan: Default::default(),
            },
            ArchitectureRule {
                name: DescriptionVO::new(AES205_RULE_CODE),
                description: DescriptionVO::new(""),
                rule_type: ErrorCode::raw(AES205_RULE_CODE),
                enabled: BooleanVO::new(true),
                scope: LayerNameVO::new("capabilities"),
                exceptions: PatternList::default(),
                allowed: PatternList::default(),
                forbidden: PatternList::default(),
                mandatory: PatternList::default(),
                naming: Default::default(),
                code_analysis: Default::default(),
                role: Default::default(),
                orphan: Default::default(),
            },
        ],
        NamingConfig::new(Count::new(3)),
        FilePathList::new(vec![]),
        BooleanVO::new(false),
    )
}

// ── Failing checkers that return Err for every call ──
struct FailingMandatory;
impl IImportMandatoryProtocol for FailingMandatory {
    fn rule_name(&self) -> Identity {
        Identity::new(AES202_RULE_CODE)
    }
    fn run_mandatory_imports(
        &self,
        _config: &ArchitectureConfig,
        _layer_map: &LayerMapVO,
        _files: &FilePathList,
        _root_dir: &FilePath,
        _content_map: &HashMap<String, String>,
        _imports_map: &HashMap<String, Vec<ImportEntry>>,
    ) -> Result<LintResultList, ImportError> {
        Err(ImportError::InvalidConfiguration {
            layer: LayerNameVO::new("capabilities"),
            message: ErrorMessage::new("boom"),
        })
    }
}

struct FailingForbidden;
impl IImportForbiddenProtocol for FailingForbidden {
    fn rule_name(&self) -> Identity {
        Identity::new(AES201_RULE_CODE)
    }
    fn check_forbidden_imports(
        &self,
        _config: &ArchitectureConfig,
        _layer_map: &LayerMapVO,
        _files: &FilePathList,
        _root_dir: &FilePath,
        _content_map: &HashMap<String, String>,
        _imports_map: &HashMap<String, Vec<ImportEntry>>,
    ) -> Result<LintResultList, ImportError> {
        Err(ImportError::InvalidConfiguration {
            layer: LayerNameVO::new("capabilities"),
            message: ErrorMessage::new("boom"),
        })
    }
}

struct FailingCycle;
impl ICycleImportProtocol for FailingCycle {
    fn scan(
        &self,
        _config: &ArchitectureConfig,
        _layer_map: &LayerMapVO,
        _files: &[FilePath],
        _root_dir: &FilePath,
        _content_map: &HashMap<String, String>,
        _imports_map: &HashMap<String, Vec<ImportEntry>>,
    ) -> Vec<LintResult> {
        vec![]
    }
    fn check_cycles(
        &self,
        _config: &ArchitectureConfig,
        _layer_map: &LayerMapVO,
        _files: &FilePathList,
        _root_dir: &FilePath,
        _content_map: &HashMap<String, String>,
        _imports_map: &HashMap<String, Vec<ImportEntry>>,
    ) -> Result<Vec<LintResult>, ImportError> {
        Err(ImportError::InvalidConfiguration {
            layer: LayerNameVO::new("capabilities"),
            message: ErrorMessage::new("boom"),
        })
    }
    fn detect_cycle_edges(&self, _edges: &[DependencyEdge]) -> Vec<SymbolName> {
        vec![]
    }
    fn normalize_to_layer(&self, _name: &str) -> LayerNameVO {
        LayerNameVO::new("capabilities")
    }
}

struct FailingUnused;
impl IUnusedImportProtocol for FailingUnused {
    fn find_unused_imports(
        &self,
        _path: &FilePath,
        _content: &str,
        _import_entries: &[ImportEntry],
        _used_identifiers: &[SymbolName],
    ) -> Result<Vec<LintMessage>, ImportError> {
        Err(ImportError::InvalidConfiguration {
            layer: LayerNameVO::new("capabilities"),
            message: ErrorMessage::new("boom"),
        })
    }
    fn check_unused_imports(
        &self,
        _file: &str,
        _content: &str,
        _import_entries: &[ImportEntry],
        _used_identifiers: &[SymbolName],
        _implemented_traits: &HashMap<String, Vec<String>>,
    ) -> Result<Vec<LintResult>, ImportError> {
        Err(ImportError::InvalidConfiguration {
            layer: LayerNameVO::new("capabilities"),
            message: ErrorMessage::new("boom"),
        })
    }
}

struct FailingDummy;
impl IDummyImportCheckerProtocol for FailingDummy {
    fn rule_name(&self) -> Identity {
        Identity::new(AES204_RULE_CODE)
    }
    fn check_dummy_imports(
        &self,
        _file: &FilePath,
        _content: &ContentString,
        _root_dir: &FilePath,
        _layer_map: &LayerMapVO,
        _import_entries: &[ImportEntry],
    ) -> Result<Vec<LintResult>, ImportError> {
        Err(ImportError::InvalidConfiguration {
            layer: LayerNameVO::new("capabilities"),
            message: ErrorMessage::new("boom"),
        })
    }
    fn check_dummy_functions(
        &self,
        _file: &FilePath,
        _content: &ContentString,
        _root_dir: &FilePath,
        _layer_map: &LayerMapVO,
    ) -> Result<Vec<LintResult>, ImportError> {
        Err(ImportError::InvalidConfiguration {
            layer: LayerNameVO::new("capabilities"),
            message: ErrorMessage::new("boom"),
        })
    }
    fn check_dummy_impls(
        &self,
        _file: &FilePath,
        _content: &ContentString,
        _root_dir: &FilePath,
        _layer_map: &LayerMapVO,
    ) -> Result<Vec<LintResult>, ImportError> {
        Err(ImportError::InvalidConfiguration {
            layer: LayerNameVO::new("capabilities"),
            message: ErrorMessage::new("boom"),
        })
    }
    fn check_taxonomy_intent(
        &self,
        _file: &FilePath,
        _content: &ContentString,
        _root_dir: &FilePath,
        _layer_map: &LayerMapVO,
        _import_entries: &[ImportEntry],
    ) -> Result<Vec<LintResult>, ImportError> {
        Err(ImportError::InvalidConfiguration {
            layer: LayerNameVO::new("capabilities"),
            message: ErrorMessage::new("boom"),
        })
    }
    fn check_all_dummy(
        &self,
        _file: &FilePath,
        _content: &ContentString,
        _root_dir: &FilePath,
        _layer_map: &LayerMapVO,
        _imports_map: &HashMap<String, Vec<ImportEntry>>,
    ) -> Result<Vec<LintResult>, ImportError> {
        Err(ImportError::InvalidConfiguration {
            layer: LayerNameVO::new("capabilities"),
            message: ErrorMessage::new("boom"),
        })
    }
}

fn orchestrator_with_deps(
    mandatory: Arc<dyn IImportMandatoryProtocol>,
    forbidden: Arc<dyn IImportForbiddenProtocol>,
    unused: Arc<dyn IUnusedImportProtocol>,
    cycle: Arc<dyn ICycleImportProtocol>,
    dummy: Arc<dyn IDummyImportCheckerProtocol>,
) -> ImportOrchestrator {
    let config = test_config();
    let _layer_map = LayerMapVO::new(config.layers.clone());
    ImportOrchestrator::new(
        ImportOrchestratorDeps {
            mandatory,
            forbidden,
            unused,
            cycle,
            dummy,
            filesystem: Arc::new(MockFilesystem),
            filesystem_io: Arc::new(MockFilesystemIo),
            filesystem_workspace: Arc::new(MockWorkspace),
            filesystem_parser: Arc::new(MockParser),
        },
        config,
        vec![],
    )
}

// ── Mock filesystem deps ──
struct MockFilesystem;
impl IFilesystemAggregate for MockFilesystem {
    fn execute(&self, _request: FilesystemRequest) -> FilesystemResponse {
        FilesystemResponse::Imports { entries: vec![] }
    }
}

struct MockFilesystemIo;
impl IFileSystemIOProtocol for MockFilesystemIo {
    fn path_exists(&self, _path: &Path) -> bool {
        true
    }
    fn is_dir(&self, _path: &Path) -> bool {
        false
    }
    fn is_file(&self, _path: &Path) -> bool {
        false
    }
    fn should_ignore(&self, _path: &FilePath, _ignored: &[String]) -> bool {
        false
    }
    fn canonicalize(&self, _path: &Path) -> Result<PathBuf, std::io::Error> {
        unimplemented!()
    }
    fn canonicalize_path_str(&self, _path: &FilePath) -> FilePath {
        unimplemented!()
    }
    fn is_symlink(&self, _path: &Path) -> bool {
        false
    }
    fn metadata(&self, _path: &Path) -> Result<std::fs::Metadata, std::io::Error> {
        unimplemented!()
    }
    fn symlink_metadata(&self, _path: &Path) -> Result<std::fs::Metadata, std::io::Error> {
        unimplemented!()
    }
    fn get_file_stem<'a>(&self, path: &'a str) -> &'a str {
        path
    }
    fn is_source_file(&self, _path: &Path) -> bool {
        false
    }
    fn is_source_ext(&self, _ext: &FileExtension) -> bool {
        false
    }
    fn get_basename<'a>(&self, path: &'a str) -> &'a str {
        path
    }
    fn get_parent<'a>(&self, path: &'a str) -> &'a str {
        path
    }
    fn is_python_file(&self, _path: &Path) -> bool {
        false
    }
    fn scan_directory_with_ignored(&self, _dir: &Path, _ignored: &PatternList) -> Vec<PathBuf> {
        vec![]
    }
    fn is_ignored_dir(&self, _dir: &Path, _ignored: &PatternList) -> bool {
        false
    }
    fn read_dir_entries_as_pathbuf(&self, _dir: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
        Ok(vec![])
    }
    fn read_to_string(&self, _path: &Path) -> Result<ContentString, std::io::Error> {
        unimplemented!()
    }
    fn write_string(&self, _path: &Path, _content: &str) -> Result<(), std::io::Error> {
        unimplemented!()
    }
    fn copy_file(&self, _src: &Path, _dst: &Path) -> Result<ByteCount, std::io::Error> {
        unimplemented!()
    }
    fn create_dir_all(&self, _path: &Path) -> Result<(), std::io::Error> {
        unimplemented!()
    }
    fn remove_dir_all(&self, _path: &Path) -> Result<(), std::io::Error> {
        unimplemented!()
    }
    fn set_permissions(&self, _path: &Path, _mode: FileMode) -> std::io::Result<()> {
        unimplemented!()
    }
    fn remove_file(&self, _path: &Path) -> std::io::Result<()> {
        unimplemented!()
    }
    fn run_git_command(&self, _args: &[&str], _dir: &str) -> GitCommandResult {
        unimplemented!()
    }
    fn parse_output_lines(&self, _output: &str) -> ParsedLines {
        unimplemented!()
    }
    fn run_external_command_in(
        &self,
        _name: &ToolName,
        _args: &[&str],
        _current_dir: &str,
    ) -> (String, String, bool) {
        unimplemented!()
    }
    fn timing(&self) -> &ScanTiming {
        unimplemented!()
    }
}

struct MockWorkspace;
impl IWorkspaceProtocol for MockWorkspace {
    fn workspace_root(&self, _start: &FilePath) -> Option<PathBuf> {
        None
    }
    fn find_workspace_root_from_path(&self, _start: &Path) -> Result<PathBuf, std::io::Error> {
        unimplemented!()
    }
    fn is_member_path(&self, _path: &FilePath) -> bool {
        false
    }
    fn is_leaf_member_path(&self, _path: &FilePath) -> bool {
        false
    }
    fn detect_source_dir(&self, _project_root: &Path) -> PathBuf {
        unimplemented!()
    }
    fn detect_language_from_path(&self, _path: &str) -> ConfigLanguage {
        unimplemented!()
    }
    fn check_wired_in_container(&self, _workspace_root: &Path, _identifiers: &PatternList) -> bool {
        false
    }
    fn detect_project_languages(&self, _root: &Path) -> ProjectLanguagesVO {
        unimplemented!()
    }
    fn resolve_orphan_module_path(
        &self,
        _root: &Path,
        _base_dir: &Path,
        _module_path: &str,
    ) -> Option<PathBuf> {
        None
    }
}

struct MockParser;
impl IParserProtocol for MockParser {
    fn parse_warnings(&self) -> &[ParseWarning] {
        &[]
    }
    fn import_list(&self) -> Vec<ImportEntry> {
        vec![]
    }
    fn parse_all(&self, _files: &mut [FileEntry]) {}
    fn imports_for(&self, _path: &Path) -> Vec<ImportEntry> {
        vec![]
    }
    fn extract(&self, _path: &Path, _content: &str, _language: Language) -> Vec<ImportEntry> {
        vec![]
    }
    fn resolve_barrel_imports(&self, _root_dir: &Path) {}
}

#[test]
fn rule_unavailable_record_appears_when_checker_fails() {
    let orch = orchestrator_with_deps(
        Arc::new(FailingMandatory),
        Arc::new(FailingForbidden),
        Arc::new(FailingUnused),
        Arc::new(FailingCycle),
        Arc::new(FailingDummy),
    );
    let file_entries = vec![shared_filesystem::taxonomy_filesystem_vo::FileEntry {
        path: PathBuf::from("/tmp/main.rs"),
        extension: "rs".to_string(),
        language: Language::Rust,
        size: "fn main() {}\n".len() as u64,
        content: "fn main() {}\n".to_string(),
        parse_ok: true,
        parse_metadata: None,
    }];
    let results = match orch.execute(ImportRequest::RunAuditWithEntriesAndImports {
        files: file_entries,
        imports_map: HashMap::new(),
    }) {
        ImportResponse::AuditEntries { violations } => violations,
        _ => unreachable!(),
    };
    // Every failing checker must produce a RULE_UNAVAILABLE record.
    for (code, severity, expected_msg) in [
        (
            AES201_RULE_CODE,
            Severity::CRITICAL,
            "AES201 forbidden-import checker is unavailable.",
        ),
        (
            AES202_RULE_CODE,
            Severity::HIGH,
            "AES202 mandatory-import checker is unavailable.",
        ),
        (
            AES203_RULE_CODE,
            Severity::MEDIUM,
            "AES203 unused-import checker is unavailable for this file.",
        ),
        (
            AES204_RULE_CODE,
            Severity::HIGH,
            "AES204 dummy-import checker is unavailable for this file.",
        ),
        (
            AES205_RULE_CODE,
            Severity::CRITICAL,
            "AES205 cycle checker is unavailable.",
        ),
    ] {
        let found = results.iter().find(|r| r.code.code() == code);
        assert!(
            found.is_some(),
            "unavailable record for {code} missing: {results:?}"
        );
        let r = found.unwrap();
        assert_eq!(
            r.violation_name, VIOLATION_NAME_RULE_UNAVAILABLE,
            "{code} unavailable record must use violation_name RULE_UNAVAILABLE"
        );
        assert_eq!(
            r.severity, severity,
            "{code} unavailable record severity mismatch: {:?}",
            r.severity
        );
        assert_eq!(
            r.message.value(),
            expected_msg,
            "{code} unavailable record message mismatch"
        );
        assert_eq!(
            r.line.value(),
            0,
            "{code} unavailable record must use line 0 (unknown location)"
        );
        assert_eq!(
            r.file.value(),
            "unknown",
            "{code} unavailable record must use file 'unknown'"
        );
    }
}

#[test]
fn failing_checker_does_not_block_other_rules() {
    // AES202 (mandatory) fails; AES201 (forbidden) must still produce its finding
    // for real file content, proving a failing checker does not block others.
    let orch = orchestrator_with_deps(
        Arc::new(FailingMandatory),
        Arc::new(ArchImportForbiddenChecker::new()),
        Arc::new(UnusedImportRuleChecker::new()),
        Arc::new(DependencyCycleAnalyzer::new()),
        Arc::new(DummyImportChecker::new()),
    );
    let file = "/tmp/project/capabilities_handler.rs";
    let content = "use agent::runner;\n";
    let entries = build_test_import_entries(file, content);
    let mut imports_map = HashMap::new();
    imports_map.insert(file.to_string(), entries);
    let file_entries = vec![shared_filesystem::taxonomy_filesystem_vo::FileEntry {
        path: PathBuf::from(file),
        extension: "rs".to_string(),
        language: Language::Rust,
        size: content.len() as u64,
        content: content.to_string(),
        parse_ok: true,
        parse_metadata: None,
    }];
    let results = match orch.execute(ImportRequest::RunAuditWithEntriesAndImports {
        files: file_entries,
        imports_map,
    }) {
        ImportResponse::AuditEntries { violations } => violations,
        _ => unreachable!(),
    };
    // AES202 must be unavailable (failing checker)
    let has_aes202_unavailable = results.iter().any(|r| {
        r.code.code() == AES202_RULE_CODE && r.violation_name == VIOLATION_NAME_RULE_UNAVAILABLE
    });
    assert!(has_aes202_unavailable, "AES202 should be unavailable");
    // AES201 finding must still appear despite AES202 failing
    let has_aes201_finding = results
        .iter()
        .any(|r| r.code.code() == AES201_RULE_CODE && r.violation_name == "FORBIDDEN_IMPORT");
    assert!(
        has_aes201_finding,
        "AES201 finding must appear even though AES202 checker failed: {results:?}"
    );
    // Other rules should NOT be unavailable
    let others_unavailable: Vec<_> = results
        .iter()
        .filter(|r| r.violation_name == VIOLATION_NAME_RULE_UNAVAILABLE)
        .filter(|r| r.code.code() != AES202_RULE_CODE)
        .collect();
    assert!(
        others_unavailable.is_empty(),
        "other rules should not be unavailable: {others_unavailable:?}"
    );
}

#[test]
fn disabled_rule_produces_no_findings() {
    let mut config = test_config();
    // Disable AES201 via config
    if let Some(r) = config
        .rules
        .iter_mut()
        .find(|r| r.name.value == AES201_RULE_CODE)
    {
        r.enabled = BooleanVO::new(false);
    }
    let _layer_map = LayerMapVO::new(config.layers.clone());
    let orch = ImportOrchestrator::new(
        ImportOrchestratorDeps {
            mandatory: Arc::new(ArchImportMandatoryChecker::new()),
            forbidden: Arc::new(ArchImportForbiddenChecker::new()),
            unused: Arc::new(UnusedImportRuleChecker::new()),
            cycle: Arc::new(DependencyCycleAnalyzer::new()),
            dummy: Arc::new(DummyImportChecker::new()),
            filesystem: Arc::new(MockFilesystem),
            filesystem_io: Arc::new(MockFilesystemIo),
            filesystem_workspace: Arc::new(MockWorkspace),
            filesystem_parser: Arc::new(MockParser),
        },
        config,
        vec![],
    );
    let file = "/tmp/project/capabilities_handler.rs";
    let content = "use agent::runner;\n";
    let entries = build_test_import_entries(file, content);
    let mut imports_map = HashMap::new();
    imports_map.insert(file.to_string(), entries);
    let file_entries = vec![shared_filesystem::taxonomy_filesystem_vo::FileEntry {
        path: PathBuf::from(file),
        extension: "rs".to_string(),
        language: Language::Rust,
        size: content.len() as u64,
        content: content.to_string(),
        parse_ok: true,
        parse_metadata: None,
    }];
    let results = orch.run_audit_with_entries_and_imports(&file_entries, &imports_map);
    // Disabled AES201 must not appear at all — no finding, no unavailable record
    assert!(
        !results.iter().any(|r| r.code.code() == AES201_RULE_CODE),
        "disabled AES201 must produce no records: {results:?}"
    );
}

#[test]
fn skip_record_for_unparseable_or_empty_files() {
    let config = test_config();
    let _layer_map = LayerMapVO::new(config.layers.clone());
    let orch = ImportOrchestrator::new(
        ImportOrchestratorDeps {
            mandatory: Arc::new(FailingMandatory),
            forbidden: Arc::new(FailingForbidden),
            unused: Arc::new(FailingUnused),
            cycle: Arc::new(FailingCycle),
            dummy: Arc::new(FailingDummy),
            filesystem: Arc::new(MockFilesystem),
            filesystem_io: Arc::new(MockFilesystemIo),
            filesystem_workspace: Arc::new(MockWorkspace),
            filesystem_parser: Arc::new(MockParser),
        },
        config,
        vec![],
    );
    let skipped_files = vec![
        shared_filesystem::taxonomy_filesystem_vo::FileEntry {
            path: PathBuf::from("/tmp/project/broken.rs"),
            extension: "rs".to_string(),
            language: Language::Rust,
            size: 0,
            content: String::new(),
            parse_ok: false,
            parse_metadata: None,
        },
        shared_filesystem::taxonomy_filesystem_vo::FileEntry {
            path: PathBuf::from("/tmp/project/empty.rs"),
            extension: "rs".to_string(),
            language: Language::Rust,
            size: 0,
            content: String::new(),
            parse_ok: true,
            parse_metadata: None,
        },
    ];
    let results = orch.run_audit_with_entries(&skipped_files);
    // Exactly one skip record must appear.
    let skip_records: Vec<_> = results
        .iter()
        .filter(|r| {
            r.code.code() == AES200_RULE_CODE && r.violation_name == VIOLATION_NAME_RULE_SKIP
        })
        .collect();
    assert_eq!(
        skip_records.len(),
        1,
        "exactly one skip record expected for multiple skipped files, got {}: {skip_records:?}",
        skip_records.len()
    );
    // Message must contain the count of skipped files.
    assert!(
        skip_records[0]
            .message
            .value()
            .contains("2 file(s) skipped for parse failure or emptiness"),
        "skip record message must state the count, got: {}",
        skip_records[0].message
    );
    assert!(
        skip_records[0].message.value().contains("broken.rs"),
        "skip record message must name skipped files"
    );
}
