// PURPOSE: Naming rules scan business logic, no formatting.
//
// Data Flow:
//   CLI → collect_naming → filesystem.file_list() → naming_orchestrator.run_audit_with_entries → violations
//
// The naming-rules crate performs zero I/O — it receives &[FileEntry] and
// returns LintResult violations. All filesystem access is handled by the
// filesystem aggregate.
use std::sync::Arc;

use shared::common::FilePath;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::naming_rules::INamingRunnerAggregate;

use shared::common::ViolationItem;

pub fn collect_naming(
    path: Option<FilePath>,
    naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
    filter: Option<String>,
    fs_agg: Arc<dyn IFilesystemAggregate>,
    ignored_paths: &[String],
) -> Result<Vec<ViolationItem>, String> {
    // 1. Resolve target path (default: current directory)
    let root = match &path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    };

    // 2. Validate path exists (delegated to filesystem aggregate)
    if !fs_agg.path_exists(std::path::Path::new(&root)) {
        return Err(format!("Error: path '{}' does not exist", root));
    }
    let _root_fp = FilePath::new(root.clone()).map_err(|_| "invalid path".to_string())?;

    // 3. Build file index for target path (respects config ignored_paths)
    let root_path = std::path::Path::new(&root);
    fs_agg.build_file_index_with_ignored(root_path, ignored_paths);

    // 4. Run naming audit — orchestrator does zero I/O, only delegates
    //    to naming_convention_checker (AES101) and suffix_prefix_checker (AES102).
    let results = naming_orchestrator.run_audit_with_entries(fs_agg.file_list());

    // 4. Convert LintResult to ViolationItem for output formatting
    let mut violations: Vec<ViolationItem> = results
        .iter()
        .map(ViolationItem::from_lint_result)
        .collect();

    // 5. Apply optional filter (by violation code)
    if let Some(ref filter_str) = filter {
        let filter_upper = filter_str.to_uppercase();
        violations.retain(|v| v.code.code().contains(&filter_upper));
    }

    // 6. Return violations — CLI formats output and maps exit code
    Ok(violations)
}
