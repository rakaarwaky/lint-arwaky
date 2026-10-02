// PURPOSE: Naming rules scan business logic, no formatting.
//
// Data Flow:
//   CLI → collect_naming → filesystem.execute(FilesystemRequest::FileList).into_file_list() → naming_orchestrator.execute → violations
//
// The naming-rules crate performs zero I/O — it receives &[FileEntry] and
// returns LintResult violations. All filesystem access is handled by the
// filesystem aggregate.
use std::sync::Arc;

use shared_common::FilePath;
use shared_filesystem::FilesystemRequest;
use shared_filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared_filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use shared_filesystem::taxonomy_filesystem_vo::{FileEntry, Language};
use shared_naming_rules::INamingRunnerAggregate;
use shared_naming_rules::taxonomy_naming_rules_request::NamingRequest;
use shared_naming_rules::taxonomy_naming_rules_response::NamingResponse;
use shared_naming_rules::{BENCHES_DIR, TESTS_DIR};

use shared_common::ViolationItem;

pub fn collect_naming(
    path: Option<FilePath>,
    naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
    filter: Option<String>,
    fs_agg: Arc<dyn IFilesystemAggregate>,
    filesystem_io: Arc<dyn IFileSystemIOProtocol>,
    ignored_paths: &[String],
) -> Result<Vec<ViolationItem>, String> {
    // 1. Resolve target path (default: current directory)
    let root = match &path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    };

    // 2. Validate path exists (delegated to filesystem aggregate)
    if !filesystem_io.path_exists(std::path::Path::new(&root)) {
        return Err(format!("Error: path '{}' does not exist", root));
    }
    let _root_fp = FilePath::new(root.clone()).map_err(|_| "invalid path".to_string())?;

    // 3. Build file index for target path (respects config ignored_paths)
    let root_path = std::path::Path::new(&root);
    fs_agg.execute(FilesystemRequest::build_file_index_with_ignored(
        root_path,
        ignored_paths,
    ));

    // 4. Run naming audit — orchestrator does zero I/O, only delegates to the
    //    naming-convention, suffix-policy, and test-prefix protocols
    //    (AES101 + AES102 + AES103). AES103's subject is the test and bench
    //    files, which the index build above prunes, so they are discovered
    //    separately rather than read from the file list.
    let source_files = fs_agg.execute(FilesystemRequest::FileList).into_file_list();
    let test_files = discover_test_entries(fs_agg, &root, ignored_paths);
    let request = NamingRequest::audit_with_tests(&source_files, &test_files);
    let NamingResponse::Audit {
        violations: results,
    } = naming_orchestrator.execute(request)
    else {
        return Err("naming audit returned an unexpected response".to_string());
    };

    // 5. Convert LintResult to ViolationItem for output formatting
    let mut violations: Vec<ViolationItem> = results
        .iter()
        .map(ViolationItem::from_lint_result)
        .collect();

    // 6. Apply optional filter (by violation code)
    if let Some(ref filter_str) = filter {
        let filter_upper = filter_str.to_uppercase();
        violations.retain(|v| v.code.code().contains(&filter_upper));
    }

    // 7. Return violations — CLI formats output and maps exit code
    Ok(violations)
}

/// Build file entries for the test and bench files under *root*, for AES103.
///
/// Entries carry the path and body the naming checkers read; the parse metadata
/// is left default because AES103 judges file *names*, not their contents.
fn discover_test_entries(
    fs_agg: Arc<dyn IFilesystemAggregate>,
    root: &str,
    ignored_paths: &[String],
) -> Vec<FileEntry> {
    let paths = fs_agg
        .execute(FilesystemRequest::discover_files_in_directories(
            std::path::Path::new(root),
            &[TESTS_DIR, BENCHES_DIR],
            ignored_paths,
        ))
        .into_paths();
    paths
        .into_iter()
        .filter_map(|path| {
            let content = fs_agg
                .execute(FilesystemRequest::read_file(std::path::Path::new(&path)))
                .into_content_opt()
                .unwrap_or_default();
            let file_path = FilePath::new(path.clone()).ok()?;
            Some(FileEntry {
                path: std::path::PathBuf::from(&path),
                size: content.len() as u64,
                content,
                extension: file_path
                    .value
                    .rsplit_once('.')
                    .map(|(_, ext)| ext.to_string())
                    .unwrap_or_default(),
                language: Language::Unknown,
                parse_ok: false,
                parse_metadata: None,
            })
        })
        .collect()
}
