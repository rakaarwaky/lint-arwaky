// Unit tests — DiffChecker: lintable filter, get_diff, run_git_diff_check.

use git_hooks_lint_arwaky::capabilities_diff_checker::{DiffChecker, is_lintable_file};
use shared_common::FilePath;
use shared_filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use shared_filesystem::taxonomy_filesystem_vo::{GitCommandResult, ParsedLines, ScanTiming};
use shared_git_hooks::contract_git_hooks_protocol::IDiffDetectionProtocol;
use shared_quality_rules::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared_quality_rules::taxonomy_quality_rules_request::CodeAnalysisRequest;
use shared_quality_rules::taxonomy_quality_rules_response::CodeAnalysisResponse;
use std::sync::Arc;

// ─── Mock IO that returns rename-status output ─────────────

struct RenameMockIo {
    rename_output: String,
}

impl IFileSystemIOProtocol for RenameMockIo {
    fn run_git_command(&self, args: &[&str], _dir: &str) -> GitCommandResult {
        // The rename path uses --name-status --diff-filter=R
        if args.contains(&"--name-status") {
            GitCommandResult::new(self.rename_output.clone(), String::new(), true)
        } else if args[0] == "symbolic-ref" {
            GitCommandResult::new("refs/remotes/origin/main".to_string(), String::new(), true)
        } else {
            GitCommandResult::new(String::new(), String::new(), false)
        }
    }

    fn parse_output_lines(&self, output: &str) -> ParsedLines {
        ParsedLines::new(output.lines().map(String::from).collect())
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
    fn canonicalize_path_str(&self, path: &FilePath) -> FilePath {
        path.clone()
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
    fn is_source_ext(
        &self,
        _ext: &shared_filesystem::taxonomy_filesystem_vo::FileExtension,
    ) -> bool {
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
        _ignored: &shared_common::PatternList,
    ) -> Vec<std::path::PathBuf> {
        vec![]
    }
    fn is_ignored_dir(
        &self,
        _dir: &std::path::Path,
        _ignored: &shared_common::PatternList,
    ) -> bool {
        false
    }
    fn read_dir_entries_as_pathbuf(
        &self,
        _dir: &std::path::Path,
    ) -> Result<Vec<std::path::PathBuf>, std::io::Error> {
        Ok(vec![])
    }
    fn read_to_string(
        &self,
        _path: &std::path::Path,
    ) -> Result<shared_common::ContentString, std::io::Error> {
        Ok(shared_common::ContentString::new(""))
    }
    fn write_string(&self, _path: &std::path::Path, _content: &str) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn copy_file(
        &self,
        _src: &std::path::Path,
        _dst: &std::path::Path,
    ) -> Result<shared_filesystem::taxonomy_filesystem_vo::ByteCount, std::io::Error> {
        Ok(shared_filesystem::taxonomy_filesystem_vo::ByteCount::new(0))
    }
    fn create_dir_all(&self, _path: &std::path::Path) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn remove_dir_all(&self, _path: &std::path::Path) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn set_permissions(
        &self,
        _path: &std::path::Path,
        _mode: shared_filesystem::taxonomy_filesystem_vo::FileMode,
    ) -> std::io::Result<()> {
        Ok(())
    }
    fn remove_file(&self, _path: &std::path::Path) -> std::io::Result<()> {
        Ok(())
    }
    fn run_external_command_in(
        &self,
        _name: &shared_common::ToolName,
        _args: &[&str],
        _current_dir: &str,
    ) -> (String, String, bool) {
        (String::new(), String::new(), false)
    }
}

// ─── No-op linter aggregate ───────────────────────────────

struct NoOpLinter;

impl ICodeAnalysisAggregate for NoOpLinter {
    fn execute(&self, _request: CodeAnalysisRequest) -> CodeAnalysisResponse {
        CodeAnalysisResponse::Analysis {
            violations: Vec::new(),
        }
    }
}

// ─── Rename parser tests ───────────────────────────────────

#[test]
fn renamed_parser_handles_r100_tab_format() {
    // Simulates `git diff --name-status --diff-filter=R` output:
    // R100\t<old>\t<new>
    let output = "R100\tsrc/old_module.rs\tsrc/new_module.rs\n";
    let io = Arc::new(RenameMockIo {
        rename_output: output.to_string(),
    });
    let checker = DiffChecker::new(io, Arc::new(NoOpLinter));
    let path = FilePath::new("/fake/repo".to_string()).unwrap();
    let result = checker.get_diff(&path);

    assert!(
        !result.renamed.values.is_empty(),
        "rename-only diff must produce a non-empty RenamedFileList"
    );
    assert_eq!(result.renamed.values.len(), 1);
    let renamed = &result.renamed.values[0];
    assert_eq!(renamed.old_path.value, "src/old_module.rs");
    assert_eq!(renamed.new_path.value, "src/new_module.rs");
}

#[test]
fn renamed_parser_handles_multiple_renames() {
    let output = "R100\ta.rs\tb.rs\nR85\tc.rs\td.rs\n";
    let io = Arc::new(RenameMockIo {
        rename_output: output.to_string(),
    });
    let checker = DiffChecker::new(io, Arc::new(NoOpLinter));
    let path = FilePath::new("/fake/repo".to_string()).unwrap();
    let result = checker.get_diff(&path);

    assert_eq!(result.renamed.values.len(), 2);
    assert_eq!(result.renamed.values[0].old_path.value, "a.rs");
    assert_eq!(result.renamed.values[0].new_path.value, "b.rs");
    assert_eq!(result.renamed.values[1].old_path.value, "c.rs");
    assert_eq!(result.renamed.values[1].new_path.value, "d.rs");
}

#[test]
fn renamed_parser_ignores_non_rename_lines() {
    // M lines mixed in — parser should skip non-R lines
    let output = "M\tmodified.rs\nR100\told.rs\tnew.rs\n";
    let io = Arc::new(RenameMockIo {
        rename_output: output.to_string(),
    });
    let checker = DiffChecker::new(io, Arc::new(NoOpLinter));
    let path = FilePath::new("/fake/repo".to_string()).unwrap();
    let result = checker.get_diff(&path);

    assert_eq!(result.renamed.values.len(), 1);
    assert_eq!(result.renamed.values[0].old_path.value, "old.rs");
    assert_eq!(result.renamed.values[0].new_path.value, "new.rs");
}

#[test]
fn renamed_parser_empty_output_returns_empty_list() {
    let io = Arc::new(RenameMockIo {
        rename_output: String::new(),
    });
    let checker = DiffChecker::new(io, Arc::new(NoOpLinter));
    let path = FilePath::new("/fake/repo".to_string()).unwrap();
    let result = checker.get_diff(&path);

    assert!(result.renamed.values.is_empty());
}

// ─── Lintable filter (FR-001) ─────────────────────────────

#[test]
fn lintable_rs_is_lintable() {
    assert!(is_lintable_file(
        &FilePath::new("src/main.rs".to_string()).unwrap()
    ));
}

#[test]
fn lintable_py_is_lintable() {
    assert!(is_lintable_file(
        &FilePath::new("app.py".to_string()).unwrap()
    ));
}

#[test]
fn lintable_ts_is_lintable() {
    assert!(is_lintable_file(
        &FilePath::new("index.ts".to_string()).unwrap()
    ));
}

#[test]
fn lintable_js_is_lintable() {
    assert!(is_lintable_file(
        &FilePath::new("script.js".to_string()).unwrap()
    ));
}

#[test]
fn lintable_jsx_is_lintable() {
    assert!(is_lintable_file(
        &FilePath::new("App.jsx".to_string()).unwrap()
    ));
}

#[test]
fn lintable_tsx_is_lintable() {
    assert!(is_lintable_file(
        &FilePath::new("App.tsx".to_string()).unwrap()
    ));
}

#[test]
fn non_lintable_md_not_lintable() {
    assert!(!is_lintable_file(
        &FilePath::new("README.md".to_string()).unwrap()
    ));
}

#[test]
fn non_lintable_toml_not_lintable() {
    assert!(!is_lintable_file(
        &FilePath::new("Cargo.toml".to_string()).unwrap()
    ));
}

#[test]
fn non_lintable_json_not_lintable() {
    assert!(!is_lintable_file(
        &FilePath::new("package.json".to_string()).unwrap()
    ));
}

#[test]
fn non_lintable_yaml_not_lintable() {
    assert!(!is_lintable_file(
        &FilePath::new("config.yaml".to_string()).unwrap()
    ));
}

#[test]
fn non_lintable_lock_not_lintable() {
    assert!(!is_lintable_file(
        &FilePath::new("Cargo.lock".to_string()).unwrap()
    ));
}

#[test]
fn non_lintable_png_not_lintable() {
    assert!(!is_lintable_file(
        &FilePath::new("image.png".to_string()).unwrap()
    ));
}

#[test]
fn non_lintable_empty_ext_not_lintable() {
    assert!(!is_lintable_file(
        &FilePath::new("Makefile".to_string()).unwrap()
    ));
}

#[test]
fn lintable_nested_path_rs() {
    assert!(is_lintable_file(
        &FilePath::new("crates/shared/src/lib.rs".to_string()).unwrap()
    ));
}

#[test]
fn lintable_nested_path_tsx() {
    assert!(is_lintable_file(
        &FilePath::new("src/components/App.tsx".to_string()).unwrap()
    ));
}
