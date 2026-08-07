// PURPOSE: ImportOrchestrator — agent that orchestrates import rule checks
// Uses new protocol interfaces — no IAnalyzer, no IArchImportProtocol.

use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use shared::cli_commands::LintResult;
use shared::common::{ContentString, ErrorMessage, FilePath, FilePathList, ScanError};
use shared::config_system::ArchitectureConfig;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::filesystem::taxonomy_filesystem_vo::{ImportEntry, ParseMetadata};
use shared::import_rules::DEFAULT_SKIP_DIRS;
use shared::import_rules::contract_cycle_import_protocol::ICycleImportProtocol;
use shared::import_rules::contract_dummy_import_protocol::IDummyImportCheckerProtocol;
use shared::import_rules::contract_import_forbidden_protocol::IImportForbiddenProtocol;
use shared::import_rules::contract_import_mandatory_protocol::IImportMandatoryProtocol;
use shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use shared::import_rules::contract_unused_import_protocol::IUnusedImportProtocol;

use shared::common::taxonomy_definition_vo::LayerMapVO;
use tracing::warn;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ImportOrchestratorDeps {
    pub mandatory: Arc<dyn IImportMandatoryProtocol>,
    pub forbidden: Arc<dyn IImportForbiddenProtocol>,
    pub unused: Arc<dyn IUnusedImportProtocol>,
    pub cycle: Arc<dyn ICycleImportProtocol>,
    pub dummy: Arc<dyn IDummyImportCheckerProtocol>,
    pub filesystem: Arc<dyn IFilesystemAggregate>,
}

pub struct ImportOrchestrator {
    deps: ImportOrchestratorDeps,
    layer_map: LayerMapVO,
    config: ArchitectureConfig,
    ignored_paths: Vec<String>,
}

// ─── Block 2: Aggregate Trait Implementation ──────────────

impl IImportRunnerAggregate for ImportOrchestrator {
    fn run_audit(&self, target: &FilePath) -> Result<Vec<LintResult>, ScanError> {
        if !self.config.enabled.value {
            return Ok(Vec::new());
        }
        if !self
            .deps
            .filesystem
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
            .filesystem
            .workspace_root(target)
            .and_then(|p| FilePath::new(p.to_string_lossy().to_string()).ok())
            .unwrap_or_else(|| FilePath::new(".").unwrap_or_default());

        // Pre-read all file contents into a map so capabilities don't do I/O.
        let content_map: HashMap<String, String> = files
            .values
            .iter()
            .filter_map(|f| {
                self.deps
                    .filesystem
                    .read_file(std::path::Path::new(f.value()))
                    .map(|c| (f.value().to_string(), c))
            })
            .collect();

        // Build import map from filesystem's AST parser (avoids re-parsing in checkers)
        let import_list = self.deps.filesystem.import_list();
        let imports_map: HashMap<String, Vec<ImportEntry>> = {
            let mut map: HashMap<String, Vec<ImportEntry>> = HashMap::new();
            for entry in import_list {
                let key = entry.source_file.to_string_lossy().to_string();
                map.entry(key).or_default().push(entry.clone());
            }
            map
        };

        // Build used_identifiers map from filesystem's tree-sitter AST cache
        let used_identifiers_map: HashMap<String, Vec<String>> = files
            .values
            .iter()
            .filter_map(|f| {
                let ids = self
                    .deps
                    .filesystem
                    .used_identifiers_for(std::path::Path::new(f.value()));
                if ids.is_empty() {
                    None
                } else {
                    Some((f.value().to_string(), ids))
                }
            })
            .collect();

        // Build cross-file trait implementation map for implicit trait usage detection.
        // Maps trait_name → [type_names that implement it] across all Rust files.
        let implemented_traits = self.deps.filesystem.implemented_traits_map();

        Ok(self.run_checks(
            &files,
            &content_map,
            &imports_map,
            &used_identifiers_map,
            &implemented_traits,
            &root_dir,
        ))
    }

    fn name(&self) -> &str {
        "import-rules"
    }

    fn run_audit_with_entries(
        &self,
        files: &[shared::filesystem::taxonomy_filesystem_vo::FileEntry],
    ) -> Vec<LintResult> {
        if !self.config.enabled.value {
            return Vec::new();
        }

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

        // Build import map from filesystem's AST parser
        let import_list = self.deps.filesystem.import_list();
        let imports_map: HashMap<String, Vec<ImportEntry>> = {
            let mut map: HashMap<String, Vec<ImportEntry>> = HashMap::new();
            for entry in import_list {
                let key = entry.source_file.to_string_lossy().to_string();
                map.entry(key).or_default().push(entry.clone());
            }
            map
        };

        // Build used_identifiers map from FileEntry.parse_metadata (tree-sitter AST)
        let used_identifiers_map: HashMap<String, Vec<String>> = files
            .iter()
            .filter_map(|f| {
                let ids = match f.parse_metadata.as_ref()? {
                    ParseMetadata::Rust(m) => Some(m.used_identifiers.clone()),
                    ParseMetadata::Python(m) => Some(m.used_identifiers.clone()),
                    ParseMetadata::TypeScript(m) => Some(m.used_identifiers.clone()),
                    ParseMetadata::JavaScript(m) => Some(m.used_identifiers.clone()),
                    _ => None,
                };
                ids.map(|i| (f.path.to_string_lossy().to_string(), i))
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

        self.run_checks(
            &file_list,
            &content_map,
            &imports_map,
            &used_identifiers_map,
            &implemented_traits,
            &root_dir,
        )
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl ImportOrchestrator {
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
        used_identifiers_map: &HashMap<String, Vec<String>>,
        implemented_traits: &HashMap<String, Vec<String>>,
        root_dir: &FilePath,
    ) -> Vec<LintResult> {
        let mut results = Vec::new();

        // Sync checks: mandatory + forbidden
        if let Ok(mandatory) = self.deps.mandatory.run_mandatory_imports(
            &self.config,
            &self.layer_map,
            files,
            root_dir,
            content_map,
            imports_map,
        ) {
            results.extend(mandatory.values);
        }
        if let Ok(forbidden) = self.deps.forbidden.check_forbidden_imports(
            &self.config,
            &self.layer_map,
            files,
            root_dir,
            content_map,
            imports_map,
        ) {
            results.extend(forbidden.values);
        }

        // Cycle detection — prefer resolved imports when available
        let resolved_imports_list = self.deps.filesystem.resolved_import_list();
        let resolved_imports_map: HashMap<String, Vec<ImportEntry>> = {
            let mut map: HashMap<String, Vec<ImportEntry>> = HashMap::new();
            for entry in resolved_imports_list {
                let key = entry.source_file.to_string_lossy().to_string();
                map.entry(key).or_default().push(entry);
            }
            map
        };
        let cycle_map = if resolved_imports_map.is_empty() {
            imports_map
        } else {
            &resolved_imports_map
        };
        if let Ok(cycle) = self.deps.cycle.check_cycles(
            &self.config,
            &self.layer_map,
            files,
            root_dir,
            content_map,
            cycle_map,
        ) {
            results.extend(cycle);
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
                if let Ok(unused) = self.deps.unused.check_unused_imports(
                    file.value(),
                    content,
                    file_imports,
                    file_ids,
                    implemented_traits,
                ) {
                    local.extend(unused);
                }
                let content_str = ContentString::new(content.clone());
                if let Ok(dummy) = self.deps.dummy.check_all_dummy(
                    file,
                    &content_str,
                    root_dir,
                    &self.layer_map,
                    imports_map,
                ) {
                    local.extend(dummy);
                }
                local
            })
            .collect();
        results.extend(file_violations);

        results
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
            let entries = self.deps.filesystem.discover_source_files(path, &ignored);
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
