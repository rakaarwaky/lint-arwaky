// PURPOSE: External lint scan business logic, no formatting.
//
// Data Flow:
//   CLI → collect_external_direct → filesystem.build_file_index_with_ignored
//         → detect languages → load config → ExternalLintContext
//         → external_lint.scan_all_with_context → violations
//
// The surface layer performs all pre-computation (language detection, config
// loading) and passes an `ExternalLintContext` to the orchestrator, which
// runs adapters with zero filesystem I/O.
use std::process::Command;
use std::sync::Arc;

use shared::common::FilePath;
use shared::config_system::contract_parser_protocol::IConfigParserProtocol;
use shared::config_system::taxonomy_setting_vo::AdapterEntry;
use shared::external_lint::IExternalLintAggregate;
use shared::external_lint::taxonomy_external_lint_context::ExternalLintContext;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;

use crate::surface_output_component::ViolationItem;

/// Direct external lint scan — no subprocess. Used by the CLI `external`
/// subcommand so that subprocess self-invocation from `scan` terminates.
pub fn collect_external_direct(
    path: Option<FilePath>,
    external_lint: Arc<dyn IExternalLintAggregate>,
    filesystem: Arc<dyn IFilesystemAggregate>,
    config_parser: Arc<dyn IConfigParserProtocol>,
    filter: Option<String>,
    ignored_paths: &[String],
) -> Result<Vec<ViolationItem>, String> {
    let root = match &path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    };
    if !filesystem.path_exists(std::path::Path::new(&root)) {
        return Err(format!("Error: path '{}' does not exist", root));
    }
    let root_fp = FilePath::new(root.clone()).map_err(|_| "invalid path".to_string())?;

    // Build file index for target path (respects config ignored_paths)
    let root_path = std::path::Path::new(&root);
    filesystem.build_file_index_with_ignored(root_path, ignored_paths);

    // Detect languages from discovered files (extension check only — no file I/O)
    let files = filesystem.discover_files(root_path);
    let has_rust = files.iter().any(|f| f.ends_with(".rs"));
    let has_python = files.iter().any(|f| f.ends_with(".py"));
    let has_js = files.iter().any(|f| {
        f.ends_with(".js") || f.ends_with(".jsx") || f.ends_with(".ts") || f.ends_with(".tsx")
    });

    // Load adapter entries from config (pre-computed, no orchestrator I/O)
    let config_entries = load_config_entries(root_path, &*config_parser, &*filesystem);

    let context = ExternalLintContext {
        has_rust,
        has_python,
        has_js,
        ignored_paths: ignored_paths.to_vec(),
        config_entries,
    };

    let scan_results = external_lint.scan_all_with_context(&root_fp, &context);
    let mut violations: Vec<ViolationItem> = scan_results
        .values
        .iter()
        .map(ViolationItem::from_lint_result)
        .collect();

    if let Some(ref filter_str) = filter {
        let filter_upper = filter_str.to_uppercase();
        violations.retain(|v| v.code.code().contains(&filter_upper));
    }

    Ok(violations)
}

/// Walk up from `root_path` looking for lint_arwaky.config.*.yaml files.
/// Returns parsed adapter entries if any config file is found, else empty vec.
fn load_config_entries(
    root_path: &std::path::Path,
    config_parser: &dyn IConfigParserProtocol,
    fs: &dyn IFilesystemAggregate,
) -> Vec<AdapterEntry> {
    let config_names = vec!["lint_arwaky.config.yaml"];
    let start = if root_path.is_file() {
        root_path.parent().unwrap_or(root_path)
    } else {
        root_path
    };
    let mut current: Option<&std::path::Path> = Some(start);
    while let Some(dir) = current {
        for cfg_name in &config_names {
            let cfg_path = dir.join(cfg_name);
            if cfg_path.exists() {
                if let Ok(content) = fs.read_to_string(&cfg_path) {
                    let entries = config_parser.parse_adapter_entries_from_yaml(&content);
                    if !entries.is_empty() {
                        return entries;
                    }
                }
            }
        }
        current = dir.parent().filter(|&p| p != dir);
    }
    Vec::new()
}

pub fn collect_external(
    path: Option<FilePath>,
    _external_lint: Arc<dyn IExternalLintAggregate>,
    filter: Option<String>,
    _filesystem: Arc<dyn IFilesystemAggregate>,
) -> Result<Vec<ViolationItem>, String> {
    let root = match &path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    };
    if !_filesystem.path_exists(std::path::Path::new(&root)) {
        return Err(format!("Error: path '{}' does not exist", root));
    }

    // Use subprocess approach — spawn external linter and parse JSON output
    let exe_path = match std::env::current_exe() {
        Ok(p) => p,
        Err(_) => std::path::PathBuf::from("lint-arwaky-cli"),
    };

    let output = Command::new(&exe_path)
        .args(["external", &root, "--format", "json"])
        .output();

    let mut violations: Vec<ViolationItem> = match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(stdout.trim()) {
                if let Some(results) = val.get("results").and_then(|r| r.as_array()) {
                    results
                        .iter()
                        .filter_map(ViolationItem::from_json_obj)
                        .collect()
                } else if let Some(items) = val.as_array() {
                    items
                        .iter()
                        .filter_map(ViolationItem::from_json_obj)
                        .collect()
                } else {
                    Vec::new()
                }
            } else {
                Vec::new()
            }
        }
        Err(e) => return Err(format!("[error] failed to run external linter: {e}")),
    };

    if let Some(ref filter_str) = filter {
        let filter_upper = filter_str.to_uppercase();
        violations.retain(|v| v.code.code().contains(&filter_upper));
    }

    Ok(violations)
}
