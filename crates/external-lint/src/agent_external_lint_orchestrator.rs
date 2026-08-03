// PURPOSE: ExternalLintOrchestrator — agent layer, orchestrates external linter adapters
//
// The orchestrator dynamically selects which adapters to run based on the
// languages detected in the project (Rust, Python, JavaScript/TypeScript).
// It performs a file-system scan to detect language usage before running
// any adapters — avoids running rustfmt on Python-only projects.
//
// Adapters are run sequentially. If an adapter's binary
// is not installed, a warning is printed (not an error) — the scan continues
// with the remaining adapters.
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::common::AdapterNameList;
use shared::common::taxonomy_adapter_name_vo::AdapterName;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::contract_parser_protocol::IConfigParserProtocol;
use shared::config_system::taxonomy_setting_vo::AdapterEntry;
use shared::external_lint::IExternalLintAggregate;
use shared::external_lint::IExternalLintSelectorProtocol;
use shared::external_lint::contract_adapter_protocol::ILinterAdapterProtocol;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ExternalLintDeps {
    pub adapters: HashMap<String, Arc<dyn ILinterAdapterProtocol>>,
    pub filesystem: Arc<dyn IFilesystemAggregate>,
    pub config_parser: Arc<dyn IConfigParserProtocol>,
    pub selector: Arc<dyn IExternalLintSelectorProtocol>,
}

pub struct ExternalLintOrchestrator {
    deps: ExternalLintDeps,
}

// ─── Block 2: Aggregate Trait Implementation ──────────────

impl IExternalLintAggregate for ExternalLintOrchestrator {
    fn scan_all(&self, path: &FilePath) -> LintResultList {
        // FR-001: Detect project languages via filesystem aggregate's discover_files().
        // Lightweight walk (extension check only, no file reading or parsing).
        let root_path = std::path::Path::new(path.value());
        let (has_rs, has_py, has_js) =
            Self::detect_languages_from_fs(&*self.deps.filesystem, root_path);
        let ignored_paths = load_ignored_paths_from_config(
            root_path,
            has_rs,
            has_py,
            has_js,
            &*self.deps.config_parser,
            &*self.deps.filesystem,
        );

        // FR-002: Select adapters using the injected selector + config entries.
        let selected: Vec<String> = self
            .deps
            .selector
            .select_adapters(has_rs, has_py, has_js)
            .iter()
            .map(|a| a.value().to_string())
            .collect();

        // Parse config entries (with weight/timeout) and filter by enabled status
        let config_entries: Vec<AdapterEntry> = load_adapter_entries_from_config(
            root_path,
            has_rs,
            has_py,
            has_js,
            &*self.deps.config_parser,
            &*self.deps.filesystem,
        );
        let adapter_names: Vec<&str> = if config_entries.is_empty() {
            selected.iter().map(|s| s.as_str()).collect()
        } else {
            selected
                .iter()
                .filter(|name| config_entries.iter().any(|e| e.name.value() == **name))
                .map(|s| s.as_str())
                .collect()
        };

        let mut all = Vec::new();
        for name in &adapter_names {
            if let Some(adapter) = self.deps.adapters.get(*name) {
                match adapter.scan(path) {
                    Ok(results) => {
                        all.extend(results.values);
                    }
                    Err(e) => {
                        let err_msg = e.to_string();
                        if err_msg.contains("No such file or directory")
                            || err_msg.contains("os error 2")
                        {
                            eprintln!(
                                "[warn] {} is not installed or not in system PATH. Skipping.",
                                name
                            );
                        } else {
                            eprintln!("[warn] {} adapter failed: {}", name, err_msg);
                        }
                    }
                }
            }
        }
        if !ignored_paths.is_empty() {
            all.retain(|v| !self.deps.filesystem.should_ignore(&v.file, &ignored_paths));
        }
        LintResultList::new(all)
    }

    fn adapter_names(&self) -> AdapterNameList {
        AdapterNameList::new(
            self.deps
                .adapters
                .keys()
                .map(|k| AdapterName::raw(k.clone()))
                .collect(),
        )
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────
impl ExternalLintOrchestrator {
    pub fn new(deps: ExternalLintDeps) -> Self {
        Self { deps }
    }

    /// Detect languages from file extensions using filesystem aggregate.
    fn detect_languages_from_fs(
        fs: &dyn IFilesystemAggregate,
        root: &std::path::Path,
    ) -> (bool, bool, bool) {
        let files = fs.discover_files(root);
        let mut has_rs = false;
        let mut has_py = false;
        let mut has_js = false;
        for f in &files {
            match f.rsplit('.').next() {
                Some("rs") => has_rs = true,
                Some("py") => has_py = true,
                Some("ts") | Some("tsx") | Some("js") | Some("jsx") => has_js = true,
                _ => {}
            }
        }
        (has_rs, has_py, has_js)
    }
}

/// Walk up from `root_path` looking for lint_arwaky.config.*.yaml files.
/// Returns parsed adapter names if any config file is found, else None.
fn all_config_file_names() -> Vec<String> {
    vec![
        "lint_arwaky.config.yaml".to_string(),
        "lint_arwaky.config.python.yaml".to_string(),
        "lint_arwaky.config.rust.yaml".to_string(),
        "lint_arwaky.config.javascript.yaml".to_string(),
    ]
}

fn walk_up_find_config<T>(
    root_path: &Path,
    fs: &dyn IFilesystemAggregate,
    _has_rs: bool,
    _has_py: bool,
    _has_js: bool,
    mut extract: impl FnMut(&str) -> Option<T>,
) -> Option<T> {
    let config_names = all_config_file_names();
    let start = if root_path.is_file() {
        root_path.parent().unwrap_or(root_path)
    } else {
        root_path
    };
    let mut current: Option<&Path> = Some(start);
    while let Some(dir) = current {
        for cfg_name in &config_names {
            let cfg_path = dir.join(cfg_name);
            if cfg_path.exists()
                && let Ok(content) = fs.read_to_string(&cfg_path)
                && let Some(result) = extract(&content)
            {
                return Some(result);
            }
        }
        current = dir.parent().filter(|&p| p != dir);
    }
    None
}

fn load_ignored_paths_from_config(
    root_path: &Path,
    has_rs: bool,
    has_py: bool,
    has_js: bool,
    config_parser: &dyn IConfigParserProtocol,
    fs: &dyn IFilesystemAggregate,
) -> Vec<String> {
    walk_up_find_config(root_path, fs, has_rs, has_py, has_js, |content| {
        let (config, _) = config_parser.parse_config_yaml_with_warnings(content);
        let paths: Vec<String> = config
            .ignored_paths
            .values
            .iter()
            .map(|fp| fp.value.clone())
            .collect();
        if paths.is_empty() { None } else { Some(paths) }
    })
    .unwrap_or_default()
}

fn load_adapter_entries_from_config(
    root_path: &Path,
    has_rs: bool,
    has_py: bool,
    has_js: bool,
    config_parser: &dyn IConfigParserProtocol,
    fs: &dyn IFilesystemAggregate,
) -> Vec<AdapterEntry> {
    walk_up_find_config(root_path, fs, has_rs, has_py, has_js, |content| {
        let entries = config_parser.parse_adapter_entries_from_yaml(content);
        if entries.is_empty() {
            None
        } else {
            Some(entries)
        }
    })
    .unwrap_or_default()
}
