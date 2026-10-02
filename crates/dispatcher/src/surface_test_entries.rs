// PURPOSE: Build AES103 file entries for dispatch paths that need both source
// and test/bench files.
//
// `scan`, `naming`, and `ci` each hand the naming orchestrator two file sets:
// production source for AES101/AES102, and the test/bench files for AES103. That
// second set cannot come from the file index — `DEFAULT_IGNORED_PATHS` lists
// `tests` and `benches`, so the index build prunes exactly the files AES103
// judges.
//
// This lives in the surface layer (not utility) because it needs the
// `IFilesystemAggregate` contract, which utilities are forbidden from importing
// (AES201).
use std::path::{Path, PathBuf};
use std::sync::Arc;

use shared_common::FilePath;
use shared_filesystem::FilesystemRequest;
use shared_filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared_filesystem::taxonomy_filesystem_vo::{FileEntry, Language};
use shared_naming_rules::{BENCHES_DIR, TESTS_DIR};

/// Discover the test and bench files under *root* and return them as entries.
///
/// Reads through `read_lintable_file`, the bounded accessor every other
/// dispatch discovery uses: a generated fixture or a vendored test file must not
/// be able to pull an unbounded file into memory here.
///
/// The entries carry the path and the body, and leave `parse_metadata` default —
/// AES103 judges file *names*, so parsing each test file would be wasted work.
/// A file with an empty body is still returned, because a file with no content is
/// still a name the rule must judge.
pub fn build_test_entries(
    filesystem: &Arc<dyn IFilesystemAggregate>,
    root: &Path,
    ignored: &[String],
) -> Vec<FileEntry> {
    let directories = [TESTS_DIR, BENCHES_DIR];
    let paths = filesystem
        .execute(FilesystemRequest::discover_files_in_directories(
            root,
            &directories,
            ignored,
        ))
        .into_paths();

    paths
        .into_iter()
        .filter_map(|path| entry_for(filesystem, path))
        .collect()
}

/// The entry for one discovered file.
///
/// The path is validated through `FilePath` rather than trusted raw, because the
/// entry's `extension` is read from it and an unparseable path would otherwise
/// reach the rule checkers as an empty string.
fn entry_for(filesystem: &Arc<dyn IFilesystemAggregate>, path: String) -> Option<FileEntry> {
    let file_path = FilePath::new(path.clone()).ok()?;
    let content = filesystem
        .execute(FilesystemRequest::read_lintable_file(&path))
        .into_content_opt()
        .unwrap_or_default();
    Some(FileEntry {
        extension: file_path
            .value
            .rsplit_once('.')
            .map(|(_, ext)| ext.to_string())
            .unwrap_or_default(),
        size: content.len() as u64,
        path: PathBuf::from(&path),
        content,
        // AES103 reads the file name only, so the language and AST are left
        // unknown rather than parsed for nothing.
        language: Language::Unknown,
        parse_ok: false,
        parse_metadata: None,
    })
}
// trigger codacy re-scan
