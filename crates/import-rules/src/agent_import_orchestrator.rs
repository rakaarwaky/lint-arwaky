// PURPOSE: ImportOrchestrator — agent that orchestrates import rule checks
// Uses new protocol interfaces — no IAnalyzer, no IArchImportProtocol.

use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use shared::cli_commands::{LintResult, LintResultList};
use shared::common::{ContentString, ErrorMessage, FilePath, FilePathList, ScanError};
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::filesystem::taxonomy_filesystem_vo::{ImportEntry, ParseMetadata};
use shared::config_system::ArchitectureConfig;
use shared::import_rules::contract_cycle_import_protocol::ICycleImportProtocol;
use shared::import_rules::contract_dummy_import_protocol::IDummyImportCheckerProtocol;
use shared::import_rules::contract_import_forbidden_protocol::IImportForbiddenProtocol;
use shared::import_rules::contract_import_mandatory_protocol::IImportMandatoryProtocol;
use shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use shared::import_rules::contract_unused_import_protocol::IUnusedImportProtocol;
use shared::import_rules::DEFAULT_SKIP_DIRS;

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

        let mandatory_result = self.deps.mandatory.run_mandatory_imports(
            &self.config,
            &self.layer_map,
            &files,
            &root_dir,
            &content_map,
            &imports_map,
        );
        let forbidden_result = self.deps.forbidden.check_forbidden_imports(
            &self.config,
            &self.layer_map,
            &files,
            &root_dir,
            &content_map,
            &imports_map,
        );
        let mandatory_results = mandatory_result.unwrap_or_default();
        let forbidden_results = forbidden_result.unwrap_or_default();

        let root_dir_clone = root_dir.clone();
        let deps = &self.deps;
        let layer_map = &self.layer_map;
        let implemented_traits_arc = &implemented_traits;

        let file_violations: Vec<LintResult> =
            ParallelIterator::flat_map(IntoParallelRefIterator::par_iter(&files.values), |file| {
                let mut local_results = Vec::new();
                let content = match content_map.get(file.value()) {
                    Some(c) => c.clone(),
                    None => {
                        return local_results;
                    }
                };
                let file_imports = imports_map
                    .get(file.value())
                    .map(|v| v.as_slice())
                    .unwrap_or(&[]);
                let file_ids = used_identifiers_map
                    .get(file.value())
                    .map(|v| v.as_slice())
                    .unwrap_or(&[]);
                if let Ok(unused) = deps.unused.check_unused_imports(
                    file.value(),
                    &content,
                    file_imports,
                    file_ids,
                    implemented_traits_arc,
                ) {
                    local_results.extend(unused);
                }

                let content_str = ContentString::new(content);
                if let Ok(dummy) = deps.dummy.check_all_dummy(
                    file,
                    &content_str,
                    &root_dir_clone,
                    layer_map,
                    &imports_map,
                ) {
                    local_results.extend(dummy);
                }
                local_results
            })
            .collect();

        let mut results = LintResultList::new(Vec::new());
        results.values.extend(mandatory_results.values);
        results.values.extend(forbidden_results.values);
        results.values.extend(file_violations);

        if let Ok(cycle_violations) = self.deps.cycle.check_cycles(
            &self.config,
            &self.layer_map,
            &files,
            &root_dir,
            &content_map,
            &imports_map,
        ) {
            results.values.extend(cycle_violations);
        }
        Ok(results.values)
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
        let mut results = Vec::new();

        // Sync checks
        let mandatory_result = self.deps.mandatory.run_mandatory_imports(
            &self.config,
            &self.layer_map,
            &file_list,
            &root_dir,
            &content_map,
            &imports_map,
        );
        let forbidden_result = self.deps.forbidden.check_forbidden_imports(
            &self.config,
            &self.layer_map,
            &file_list,
            &root_dir,
            &content_map,
            &imports_map,
        );
        if let Ok(v) = mandatory_result {
            results.extend(v.values);
        }
        if let Ok(v) = forbidden_result {
            results.extend(v.values);
        }

        let cycle_result = self.deps.cycle.check_cycles(
            &self.config,
            &self.layer_map,
            &file_list,
            &root_dir,
            &content_map,
            &imports_map,
        );
        if let Ok(v) = cycle_result {
            results.extend(v);
        }

        // Sync checks via rayon
        use rayon::prelude::*;
        let sync_violations: Vec<LintResult> = file_list
            .values
            .par_iter()
            .flat_map(|file| {
                let mut local = Vec::new();
                let content_str = match content_map.get(file.value()) {
                    Some(c) => c.as_str(),
                    None => return local,
                };
                let file_imports = imports_map
                    .get(file.value())
                    .map(|v| v.as_slice())
                    .unwrap_or(&[]);
                let file_ids = used_identifiers_map
                    .get(file.value())
                    .map(|v| v.as_slice())
                    .unwrap_or(&[]);
                if let Ok(v) = self.deps.unused.check_unused_imports(
                    file.value(),
                    content_str,
                    file_imports,
                    file_ids,
                    &implemented_traits,
                ) {
                    local.extend(v);
                }
                let cs = ContentString::new(content_str.to_string());
                if let Ok(v) = self.deps.dummy.check_all_dummy(
                    file,
                    &cs,
                    &root_dir,
                    &self.layer_map,
                    &imports_map,
                ) {
                    local.extend(v);
                }
                local
            })
            .collect();
        results.extend(sync_violations);

        results
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
