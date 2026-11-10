// PURPOSE: ImportOrchestrator — agent that orchestrates import rule checks
// Uses new protocol interfaces — no IAnalyzer, no IArchImportProtocol.

use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use shared_cli_commands::LintResult;
use shared_common::{
    ContentString, ErrorMessage, FilePath, FilePathList, ScanError, Severity, SymbolName,
};
use shared_config_system::ArchitectureConfig;
use shared_filesystem::FilesystemRequest;
use shared_filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared_filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use shared_filesystem::contract_filesystem_protocol::IParserProtocol;
use shared_filesystem::contract_filesystem_protocol::IWorkspaceProtocol;
use shared_filesystem::taxonomy_filesystem_vo::{FileEntry, ImportEntry, ParseMetadata};
use shared_import_rules::DEFAULT_SKIP_DIRS;
use shared_import_rules::contract_import_protocol::ICycleImportProtocol;
use shared_import_rules::contract_import_protocol::IDummyImportCheckerProtocol;
use shared_import_rules::contract_import_protocol::IImportForbiddenProtocol;
use shared_import_rules::contract_import_protocol::IImportMandatoryProtocol;
use shared_import_rules::contract_import_protocol::IUnusedImportProtocol;
use shared_import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use shared_import_rules::taxonomy_import_rules_constant::{
    AES200_RULE_CODE, AES201_RULE_CODE, AES202_RULE_CODE, AES203_RULE_CODE, AES204_RULE_CODE,
    AES205_RULE_CODE, VIOLATION_NAME_RULE_SKIP, VIOLATION_NAME_RULE_UNAVAILABLE,
};
use shared_import_rules::taxonomy_import_rules_request::ImportRequest;
use shared_import_rules::taxonomy_import_rules_response::ImportResponse;

use shared_common::taxonomy_layer_vo::LayerMapVO;
use tracing::warn;

// ─── Block 1: Module Declaration ───────────────────────────

pub struct ImportOrchestratorDeps {
    pub mandatory: Arc<dyn IImportMandatoryProtocol>,
    pub forbidden: Arc<dyn IImportForbiddenProtocol>,
    pub unused: Arc<dyn IUnusedImportProtocol>,
    pub cycle: Arc<dyn ICycleImportProtocol>,
    pub dummy: Arc<dyn IDummyImportCheckerProtocol>,
    pub filesystem: Arc<dyn IFilesystemAggregate>,
    pub filesystem_io: Arc<dyn IFileSystemIOProtocol>,
    pub filesystem_workspace: Arc<dyn IWorkspaceProtocol>,
    pub filesystem_parser: Arc<dyn IParserProtocol>,
}

pub struct ImportOrchestrator {
    deps: ImportOrchestratorDeps,
    layer_map: LayerMapVO,
    config: ArchitectureConfig,
    ignored_paths: Vec<String>,
}

impl ImportOrchestrator {
    pub fn run_audit(&self, target: &FilePath) -> Result<Vec<LintResult>, ScanError> {
        if !self.config.enabled.value {
            return Ok(Vec::new());
        }
        if !self
            .deps
            .filesystem_io
            .path_exists(std::path::Path::new(target.value()))
        {
            return Err(ScanError::new(
                FilePath::new(target.value().to_string()).unwrap_or_default(),
                ErrorMessage::new(format!("Target path does not exist: {}", target.value())),
            ));
        }

        let files = self.collect_files(target);

        let root_dir = self
            .deps
            .filesystem_workspace
            .workspace_root(target)
            .and_then(|p| FilePath::new(p.to_string_lossy().to_string()).ok())
            .unwrap_or_else(|| FilePath::new(".").unwrap_or_default());

        let content_map: HashMap<String, String> = files
            .values
            .iter()
            .filter_map(|f| {
                self.deps
                    .filesystem
                    .execute(FilesystemRequest::read_file(std::path::Path::new(
                        f.value(),
                    )))
                    .into_content_opt()
                    .map(|c| (f.value().to_string(), c))
            })
            .collect();

        let import_list = self.deps.filesystem_parser.import_list();
        let imports_map = Self::build_imports_map(&import_list);

        let used_identifiers_map: HashMap<String, Vec<SymbolName>> = files
            .values
            .iter()
            .filter_map(|f| {
                let ids = self
                    .deps
                    .filesystem
                    .execute(FilesystemRequest::used_identifiers(std::path::Path::new(
                        f.value(),
                    )))
                    .into_identifiers();
                if ids.is_empty() {
                    None
                } else {
                    Some((
                        f.value().to_string(),
                        ids.into_iter().map(SymbolName::new).collect(),
                    ))
                }
            })
            .collect();

        let implemented_traits = self
            .deps
            .filesystem
            .execute(FilesystemRequest::ImplementedTraitsMap)
            .into_traits_map();

        Ok(self.run_checks(
            &files,
            &content_map,
            &imports_map,
            &used_identifiers_map,
            &implemented_traits,
            &root_dir,
        ))
    }

    pub fn name(&self) -> &str {
        "import-rules"
    }

    pub fn run_audit_with_entries(&self, files: &[FileEntry]) -> Vec<LintResult> {
        if !self.config.enabled.value {
            return Vec::new();
        }
        let import_list = self.deps.filesystem_parser.import_list();
        let imports_map = Self::build_imports_map(&import_list);
        self.run_audit_with_entries_and_imports(files, &imports_map)
    }
    pub fn run_audit_with_entries_and_imports(
        &self,
        files: &[FileEntry],
        imports_map: &HashMap<String, Vec<ImportEntry>>,
    ) -> Vec<LintResult> {
        if !self.config.enabled.value {
            return Vec::new();
        }

        // Count files skipped for parse failure or emptiness, state once for all five rules.
        let skipped: Vec<String> = files
            .iter()
            .filter(|f| !f.parse_ok || f.content.is_empty())
            .map(|f| f.path.to_string_lossy().to_string())
            .collect();
        let skipped_count = skipped.len();

        let file_paths: Vec<FilePath> = files
            .iter()
            .filter(|f| f.parse_ok && !f.content.is_empty())
            .filter_map(|f| FilePath::new(f.path.to_string_lossy().to_string()).ok())
            .collect();
        let file_list = FilePathList::new(file_paths);

        let content_map: HashMap<String, String> = files
            .iter()
            .filter(|f| f.parse_ok)
            .map(|f| (f.path.to_string_lossy().to_string(), f.content.clone()))
            .collect();

        // Build used_identifiers map from FileEntry.parse_metadata (tree-sitter AST)
        let used_identifiers_map: HashMap<String, Vec<SymbolName>> = files
            .iter()
            .filter_map(|f| {
                let ids = match f.parse_metadata.as_ref()? {
                    ParseMetadata::Rust(m) => Some(m.used_identifiers.clone()),
                    ParseMetadata::Python(m) => Some(m.used_identifiers.clone()),
                    ParseMetadata::TypeScript(m) => Some(m.used_identifiers.clone()),
                    ParseMetadata::JavaScript(m) => Some(m.used_identifiers.clone()),
                    _ => None,
                };
                ids.map(|i| {
                    (
                        f.path.to_string_lossy().to_string(),
                        i.into_iter().map(SymbolName::new).collect(),
                    )
                })
            })
            .collect();

        // Build cross-file trait implementation map for implicit trait usage detection.
        let implemented_traits: HashMap<String, Vec<String>> = {
            use std::collections::hash_map::Entry;
            let mut map: HashMap<String, Vec<String>> = HashMap::new();
            for entry in files.iter() {
                if let Some(ParseMetadata::Rust(meta)) = &entry.parse_metadata {
                    for impl_block in &meta.impl_blocks {
                        if let Some(ref trait_name) = impl_block.trait_name {
                            match map.entry(trait_name.clone()) {
                                Entry::Vacant(v) => {
                                    v.insert(vec![impl_block.implementor_type.clone()]);
                                }
                                Entry::Occupied(mut o) => {
                                    let types = o.get_mut();
                                    if !types.contains(&impl_block.implementor_type) {
                                        types.push(impl_block.implementor_type.clone());
                                    }
                                }
                            }
                        }
                    }
                }
            }
            map
        };

        let root_dir = FilePath::new(".".to_string()).unwrap_or_default();

        let mut results = self.run_checks(
            &file_list,
            &content_map,
            imports_map,
            &used_identifiers_map,
            &implemented_traits,
            &root_dir,
        );

        if skipped_count > 0 {
            let mut skipped_sorted = skipped.clone();
            skipped_sorted.sort();
            let names = skipped_sorted.join(", ");
            results.push(LintResult::new_arch_with_name(
                "unknown",
                0,
                AES200_RULE_CODE,
                Severity::INFO,
                format!(
                    "{} file(s) skipped for parse failure or emptiness: {}",
                    skipped_count, names
                ),
                VIOLATION_NAME_RULE_SKIP,
                "Skipped files are not checked by any import rule; their content is unavailable.",
                "Fix parse errors or add content so the file can be analyzed.",
            ));
        }

        results
    }

    pub fn new(
        deps: ImportOrchestratorDeps,
        config: ArchitectureConfig,
        ignored_paths: Vec<String>,
    ) -> Self {
        let layer_map = LayerMapVO::new(config.layers.clone());

        Self {
            deps,
            config,
            layer_map,
            ignored_paths,
        }
    }

    /// Shared audit pipeline — the single place where all import checks are composed.
    /// Both trait methods (`run_audit`, `run_audit_with_entries`) are thin adapters over this.
    fn run_checks(
        &self,
        files: &FilePathList,
        content_map: &HashMap<String, String>,
        imports_map: &HashMap<String, Vec<ImportEntry>>,
        used_identifiers_map: &HashMap<String, Vec<SymbolName>>,
        implemented_traits: &HashMap<String, Vec<String>>,
        root_dir: &FilePath,
    ) -> Vec<LintResult> {
        let mut results = Vec::new();

        // Sync checks: mandatory + forbidden
        if self.rule_enabled(AES202_RULE_CODE) {
            match self.deps.mandatory.run_mandatory_imports(
                &self.config,
                &self.layer_map,
                files,
                root_dir,
                content_map,
                imports_map,
            ) {
                Ok(mandatory) => results.extend(mandatory.values),
                Err(_) => results.push(Self::rule_unavailable_record(
                    AES202_RULE_CODE,
                    Severity::HIGH,
                    "AES202 mandatory-import checker is unavailable.",
                )),
            }
        }
        if self.rule_enabled(AES201_RULE_CODE) {
            match self.deps.forbidden.check_forbidden_imports(
                &self.config,
                &self.layer_map,
                files,
                root_dir,
                content_map,
                imports_map,
            ) {
                Ok(forbidden) => results.extend(forbidden.values),
                Err(_) => results.push(Self::rule_unavailable_record(
                    AES201_RULE_CODE,
                    Severity::CRITICAL,
                    "AES201 forbidden-import checker is unavailable.",
                )),
            }
        }

        // Cycle detection — prefer resolved imports when available
        let resolved_imports_list = self
            .deps
            .filesystem
            .execute(FilesystemRequest::ResolvedImportList)
            .into_imports();
        let resolved_imports_map = Self::build_imports_map(&resolved_imports_list);
        let cycle_map = if resolved_imports_map.is_empty() {
            imports_map
        } else {
            &resolved_imports_map
        };
        if self.rule_enabled(AES205_RULE_CODE) {
            match self.deps.cycle.check_cycles(
                &self.config,
                &self.layer_map,
                files,
                root_dir,
                content_map,
                cycle_map,
            ) {
                Ok(cycle) => results.extend(cycle),
                Err(_) => results.push(Self::rule_unavailable_record(
                    AES205_RULE_CODE,
                    Severity::CRITICAL,
                    "AES205 cycle checker is unavailable.",
                )),
            }
        }

        // Per-file checks: unused + dummy (parallelized via rayon)
        let file_violations: Vec<LintResult> = files
            .values
            .par_iter()
            .flat_map(|file| {
                let mut local = Vec::new();
                let Some(content) = content_map.get(file.value()) else {
                    return local;
                };
                let file_imports = imports_map
                    .get(file.value())
                    .map(|v| v.as_slice())
                    .unwrap_or(&[]);
                let file_ids = used_identifiers_map
                    .get(file.value())
                    .map(|v| v.as_slice())
                    .unwrap_or(&[]);
                if self.rule_enabled(AES203_RULE_CODE) {
                    match self.deps.unused.check_unused_imports(
                        file.value(),
                        content,
                        file_imports,
                        file_ids,
                        implemented_traits,
                    ) {
                        Ok(unused) => local.extend(unused),
                        Err(_) => local.push(Self::rule_unavailable_record(
                            AES203_RULE_CODE,
                            Severity::MEDIUM,
                            "AES203 unused-import checker is unavailable for this file.",
                        )),
                    }
                }
                let content_str = ContentString::new(content.clone());
                if self.rule_enabled(AES204_RULE_CODE) {
                    match self.deps.dummy.check_all_dummy(
                        file,
                        &content_str,
                        root_dir,
                        &self.layer_map,
                        imports_map,
                    ) {
                        Ok(dummy) => local.extend(dummy),
                        Err(_) => local.push(Self::rule_unavailable_record(
                            AES204_RULE_CODE,
                            Severity::HIGH,
                            "AES204 dummy-import checker is unavailable for this file.",
                        )),
                    }
                }
                local
            })
            .collect();
        results.extend(file_violations);

        results
    }

    /// Build a `HashMap` keyed by source-file path, mapping each file to its
    /// import entries. Shared by the `run_audit`, `run_audit_with_entries`,
    /// and `run_checks` (resolved-imports) call sites.
    fn build_imports_map(import_list: &[ImportEntry]) -> HashMap<String, Vec<ImportEntry>> {
        import_list.iter().fold(HashMap::new(), |mut map, entry| {
            let key = entry.source_file.to_string_lossy().to_string();
            map.entry(key).or_default().push(entry.clone());
            map
        })
    }

    /// Build a single "rule unavailable" audit record when a checker returns an error.
    /// Other rules still report; this record carries the failing rule's code, message,
    /// and severity so the audit surface never silently drops a rule.
    fn rule_unavailable_record(code: &str, severity: Severity, message: &str) -> LintResult {
        LintResult::new_arch_with_name(
            "unknown",
            0,
            code,
            severity,
            message,
            VIOLATION_NAME_RULE_UNAVAILABLE,
            "A checker failed to run, so this rule produced no findings for this audit.",
            "Re-run the audit after the checker error is resolved.",
        )
    }

    /// Return true when the rule with the given code is enabled in config.
    /// AES201–AES204 now check their own `enabled` flag like AES205 already does.
    /// When no rule config exists for a code, the rule is enabled by default.
    fn rule_enabled(&self, code: &str) -> bool {
        self.config
            .rules
            .iter()
            .find(|r| r.name.value == code)
            .map(|r| r.enabled.value)
            .unwrap_or(true)
    }

    fn collect_files(&self, target: &FilePath) -> FilePathList {
        let path = Path::new(target.value());
        let mut files = Vec::new();
        if path.is_dir() {
            let mut ignored = self.ignored_paths.clone();
            for d in DEFAULT_SKIP_DIRS {
                let entry = format!("/{}", d);
                if !ignored.contains(&entry) {
                    ignored.push(entry);
                }
            }
            let entries = self
                .deps
                .filesystem
                .execute(FilesystemRequest::discover_source_files(path, &ignored))
                .into_paths();
            files.extend(entries.iter().filter_map(|f| FilePath::new(f.clone()).ok()));
        } else if path.is_file() {
            match FilePath::new(path.to_string_lossy().to_string()) {
                Ok(fp) => files.push(fp),
                Err(e) => warn!(
                    path = %path.to_string_lossy(),
                    error = %e,
                    "invalid file path"
                ),
            }
        }
        FilePathList::new(files)
    }
}
// ─── Block 2: Module Contract ──────────────

impl IImportRunnerAggregate for ImportOrchestrator {
    fn execute(&self, request: ImportRequest) -> ImportResponse {
        match request {
            ImportRequest::RunAudit { target } => {
                let result = self.run_audit(&target);
                ImportResponse::Audit { result }
            }
            ImportRequest::RunAuditWithEntries { files } => {
                let violations = self.run_audit_with_entries(&files);
                ImportResponse::AuditEntries { violations }
            }
            ImportRequest::RunAuditWithEntriesAndImports { files, imports_map } => {
                let violations = self.run_audit_with_entries_and_imports(&files, &imports_map);
                ImportResponse::AuditEntries { violations }
            }
            ImportRequest::Name => ImportResponse::Name {
                name: self.name().to_string(),
            },
        }
    }
}

// ─── Block 3: Module Helpers ─────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capabilities_cycle_import_analyzer::DependencyCycleAnalyzer;
    use crate::capabilities_dummy_import_checker::DummyImportChecker;
    use crate::capabilities_import_forbidden_checker::ArchImportForbiddenChecker;
    use crate::capabilities_import_mandatory_checker::ArchImportMandatoryChecker;
    use crate::capabilities_import_unused_checker::UnusedImportRuleChecker;
    use shared_cli_commands::LintResultList;
    use shared_common::ContentString;
    use shared_common::taxonomy_adapter_name_vo::ToolName;
    use shared_common::taxonomy_error_vo::ErrorCode;
    use shared_common::taxonomy_language_vo::ConfigLanguage;
    use shared_common::taxonomy_layer_vo::{Identity, LayerDefinition, LayerMapVO, LayerNameVO};
    use shared_common::taxonomy_message_vo::DescriptionVO;
    use shared_common::taxonomy_message_vo::LintMessage;
    use shared_common::taxonomy_path_vo::FilePathList;
    use shared_common::{BooleanVO, Count, NamingConfig, PatternList};
    use shared_config_system::ArchitectureConfig;
    use shared_config_system::taxonomy_config_system_vo::ArchitectureRule;
    use shared_filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
    use shared_filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
    use shared_filesystem::contract_filesystem_protocol::IParserProtocol;
    use shared_filesystem::contract_filesystem_protocol::IWorkspaceProtocol;
    use shared_filesystem::taxonomy_filesystem_request::FilesystemRequest;
    use shared_filesystem::taxonomy_filesystem_response::FilesystemResponse;
    use shared_filesystem::taxonomy_filesystem_vo::ScanTiming;
    use shared_filesystem::taxonomy_filesystem_vo::{
        ByteCount, FileExtension, FileMode, GitCommandResult, ImportEntry, Language, ParseWarning,
        ParsedLines, ProjectLanguagesVO,
    };
    use shared_import_rules::contract_import_protocol::{
        ICycleImportProtocol, IDummyImportCheckerProtocol, IImportForbiddenProtocol,
        IImportMandatoryProtocol, IUnusedImportProtocol,
    };
    use shared_import_rules::taxonomy_import_rules_error::ImportError;
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
        let layer_map = LayerMapVO::new(config.layers.clone());
        ImportOrchestrator {
            deps: ImportOrchestratorDeps {
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
            layer_map,
            ignored_paths: vec![],
        }
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
        fn check_wired_in_container(
            &self,
            _workspace_root: &Path,
            _identifiers: &PatternList,
        ) -> bool {
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
        // Include a file with content so per-file checks (AES203, AES204) run.
        let files = FilePathList::new(vec![FilePath::new("/tmp/main.rs").unwrap()]);
        let content_map =
            HashMap::from([("/tmp/main.rs".to_string(), "fn main() {}\n".to_string())]);
        let results = orch.run_checks(
            &files,
            &content_map,
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
            &FilePath::new("/tmp").unwrap(),
        );
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
        let entries =
            crate::capabilities_import_forbidden_checker::build_test_import_entries(file, content);
        let mut imports_map = HashMap::new();
        imports_map.insert(file.to_string(), entries);
        let files = FilePathList::new(vec![FilePath::new(file).unwrap()]);
        let content_map = HashMap::from([(file.to_string(), content.to_string())]);
        let results = orch.run_checks(
            &files,
            &content_map,
            &imports_map,
            &HashMap::new(),
            &HashMap::new(),
            &FilePath::new("/tmp").unwrap(),
        );
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
        let layer_map = LayerMapVO::new(config.layers.clone());
        let orch = ImportOrchestrator {
            deps: ImportOrchestratorDeps {
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
            layer_map,
            ignored_paths: vec![],
        };
        let file = "/tmp/project/capabilities_handler.rs";
        let content = "use agent::runner;\n";
        let entries =
            crate::capabilities_import_forbidden_checker::build_test_import_entries(file, content);
        let mut imports_map = HashMap::new();
        imports_map.insert(file.to_string(), entries);
        let files = vec![shared_filesystem::taxonomy_filesystem_vo::FileEntry {
            path: std::path::PathBuf::from(file),
            extension: "rs".to_string(),
            language: shared_filesystem::taxonomy_filesystem_vo::Language::Rust,
            size: content.len() as u64,
            content: content.to_string(),
            parse_ok: true,
            parse_metadata: None,
        }];
        let results = orch.run_audit_with_entries_and_imports(&files, &imports_map);
        // Disabled AES201 must not appear at all — no finding, no unavailable record
        assert!(
            !results.iter().any(|r| r.code.code() == AES201_RULE_CODE),
            "disabled AES201 must produce no records: {results:?}"
        );
    }

    #[test]
    fn skip_record_for_unparseable_or_empty_files() {
        let config = test_config();
        let layer_map = LayerMapVO::new(config.layers.clone());
        let orch = ImportOrchestrator {
            deps: ImportOrchestratorDeps {
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
            layer_map,
            ignored_paths: vec![],
        };
        let skipped_files = vec![
            shared_filesystem::taxonomy_filesystem_vo::FileEntry {
                path: std::path::PathBuf::from("/tmp/project/broken.rs"),
                extension: "rs".to_string(),
                language: Language::Rust,
                size: 0,
                content: String::new(),
                parse_ok: false,
                parse_metadata: None,
            },
            shared_filesystem::taxonomy_filesystem_vo::FileEntry {
                path: std::path::PathBuf::from("/tmp/project/empty.rs"),
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
}

// ─── Block 3: Module Helpers ──────
