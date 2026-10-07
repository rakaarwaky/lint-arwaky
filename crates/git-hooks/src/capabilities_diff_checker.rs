// PURPOSE: DiffChecker — FR-001 protocol implementation (capabilities layer)
//
// IDiffDetectionProtocol covers both git diff detection AND the check that
// runs the lint pipeline over changed files. Two capabilities collapsed into
// one seam because they always execute together in the pre-commit flow.

use std::collections::HashSet;
use std::path::Path;

use shared_cli_commands::{LintResult, LintResultList};
use shared_common::taxonomy_adapter_name_vo::AdapterName;
use shared_common::taxonomy_common_vo::{ColumnNumber, Count, LineNumber};
use shared_common::taxonomy_error_vo::ErrorCode;
use shared_common::taxonomy_git_vo::GitBranchName;
use shared_common::taxonomy_lint_vo::LocationList;
use shared_common::taxonomy_message_vo::LintMessage;
use shared_common::taxonomy_path_vo::FilePath;
use shared_common::taxonomy_paths_vo::{FilePathList, RenamedFile, RenamedFileList};
use shared_common::taxonomy_severity_vo::Severity;
use shared_file_watch::GitDiffResultVO;
use shared_filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use shared_filesystem::taxonomy_filesystem_vo::FileEntry;
use shared_git_hooks::contract_git_hooks_protocol::IDiffDetectionProtocol;
use shared_git_hooks::taxonomy_git_hooks_constant::LINTABLE_EXTENSIONS;
use shared_quality_rules::Language;
use shared_quality_rules::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared_quality_rules::taxonomy_quality_rules_request::CodeAnalysisRequest;

use std::sync::Arc;

/// Returns `true` if the file extension is a lintable source type.
pub fn is_lintable_file(fp: &FilePath) -> bool {
    let ext = fp.extension();
    LINTABLE_EXTENSIONS.contains(&ext.as_str())
}

/// Code carried by analysis-failure results, so "analysis could not run" is
/// reported distinctly from AES rule violations in `run_git_diff_check` output.
pub const ANALYSIS_FAILURE_CODE: &str = "GIT_HOOK_ANALYSIS_FAILURE";

/// One analysis-failure result (read error or aggregate panic). Non-empty
/// output blocks the commit like a violation, but the code and CRITICAL
/// severity identify it as an infrastructure failure, not an AES finding.
fn analysis_failure(file: &str, msg: impl Into<String>) -> LintResult {
    LintResult {
        file: FilePath::new(file.to_string()).unwrap_or_default(),
        line: LineNumber::new(0),
        column: ColumnNumber::new(0),
        code: ErrorCode::raw(ANALYSIS_FAILURE_CODE),
        message: LintMessage::new(msg),
        source: Some(AdapterName::raw("git-hooks")),
        severity: Severity::CRITICAL,
        enclosing_scope: None,
        related_locations: LocationList::new(),
        violation_name: String::new(),
        why: String::new(),
        fix: String::new(),
    }
}

// ─── Block 1: Struct Definition ───────────────────────────

pub struct DiffChecker {
    io: Arc<dyn IFileSystemIOProtocol>,
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IDiffDetectionProtocol for DiffChecker {
    fn get_diff(&self, path: &FilePath) -> GitDiffResultVO {
        let default_branch = self.get_default_branch_sync(path);

        // Classify files by change type using --diff-filter
        let added = self.collect_by_filter(path, &default_branch, "A");
        let modified = self.collect_by_filter(path, &default_branch, "M");
        let deleted = self.collect_by_filter(path, &default_branch, "D");
        let renamed = self.collect_by_filter_renamed(path, &default_branch);

        // Merge all into a deduplicated set for the full list
        let mut all_set: HashSet<FilePath> = HashSet::new();
        all_set.extend(added.values.iter().cloned());
        all_set.extend(modified.values.iter().cloned());
        all_set.extend(deleted.values.iter().cloned());
        // Renamed files appear as old_path -> new_path; add the new name
        for renamed_file in &renamed.values {
            all_set.insert(renamed_file.new_path.clone());
        }

        // If classification returned nothing, fall back to unclassified collection
        if all_set.is_empty() {
            let all_changed = self.collect_changed_files_sync(path, &default_branch);
            all_set.extend(all_changed.values);
        }

        let all_vec: Vec<FilePath> = all_set.into_iter().collect();
        let lintable_vec: Vec<FilePath> = all_vec
            .iter()
            .filter(|f| is_lintable_file(f))
            .cloned()
            .collect();

        GitDiffResultVO {
            added,
            modified,
            deleted,
            renamed,
            lintable_files: FilePathList::new(lintable_vec),
            all_files: FilePathList::new(all_vec.clone()),
            total_changed: Count::new(all_vec.len() as i64),
        }
    }

    fn get_changed_files(&self, path: &FilePath, base: &GitBranchName) -> FilePathList {
        let branch_str = if base.value().is_empty() || base.value() == "." {
            self.get_default_branch_sync(path)
        } else {
            base.value().to_string()
        };
        self.collect_changed_files_sync(path, &branch_str)
    }

    fn get_default_branch(&self, path: &FilePath) -> GitBranchName {
        GitBranchName::new(self.get_default_branch_sync(path))
    }

    fn run_git_diff_check(&self, path: &FilePath) -> LintResultList {
        let default_branch = self.get_default_branch_sync(path);
        let changed_files = self.collect_changed_files_sync(path, &default_branch);

        // Filter to lintable source files only
        let lintable: Vec<FilePath> = changed_files
            .values
            .iter()
            .filter(|f| is_lintable_file(f))
            .cloned()
            .collect();
        if lintable.is_empty() {
            return LintResultList::new(Vec::new());
        }

        // Build FileEntry list for the linter aggregate, tracking read failures.
        let mut entries: Vec<FileEntry> = Vec::with_capacity(lintable.len());
        let mut failures: Vec<LintResult> = Vec::new();
        for fp in &lintable {
            let joined = Path::new(&path.value).join(&fp.value);
            match self.io.read_to_string(&joined) {
                Ok(content) => entries.push(FileEntry {
                    path: joined,
                    extension: fp.extension(),
                    language: Language::from_extension(&fp.extension())
                        .unwrap_or(Language::Unknown),
                    size: content.value.len() as u64,
                    content: content.value,
                    parse_ok: true,
                    parse_metadata: None,
                }),
                // Deleted or renamed-away files still appear in git diff; they
                // cannot be analysed and are skipped silently (FRD: invalid
                // FilePath from git output → skipped silently). A symlink whose
                // target is missing is NOT a deletion: the entry is on disk and
                // the read failure is an analysis failure. Any other read
                // failure is likewise reported distinctly.
                Err(e)
                    if e.kind() == std::io::ErrorKind::NotFound && !self.io.is_symlink(&joined) => {
                }
                Err(e) => failures.push(analysis_failure(
                    &fp.value,
                    format!("failed to read changed file for analysis: {e}"),
                )),
            }
        }

        if !entries.is_empty() {
            // Delegate to the linter aggregate: AES analysis over changed files.
            // A panicking aggregate must not crash the hook: the failure is
            // reported distinctly from violations (FRD Integration Points).
            let violations = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                self.code_analysis_linter
                    .execute(CodeAnalysisRequest::run_analysis(&entries))
                    .into_violations()
            }))
            .unwrap_or_else(|_| {
                vec![analysis_failure(
                    &path.value,
                    "AES analysis panicked over changed files",
                )]
            });
            failures.extend(violations);
        }
        LintResultList::new(failures)
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl DiffChecker {
    pub fn new(
        io: Arc<dyn IFileSystemIOProtocol>,
        code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    ) -> Self {
        Self {
            io,
            code_analysis_linter,
        }
    }

    fn get_default_branch_sync(&self, project_path: &FilePath) -> String {
        let result = self.io.run_git_command(
            &["symbolic-ref", "refs/remotes/origin/HEAD"],
            &project_path.value,
        );
        if result.success {
            let ref_str = result.stdout.trim().to_string();
            if let Some(branch) = ref_str.rsplit('/').next()
                && !branch.is_empty()
            {
                return branch.to_string();
            }
        }
        "main".to_string()
    }

    /// Collect changed files using multiple diff-variant fallback strategies.
    fn collect_changed_files_sync(
        &self,
        project_path: &FilePath,
        default_branch: &str,
    ) -> FilePathList {
        let mut changed_set: HashSet<FilePath> = HashSet::new();
        let variants = [
            format!("origin/{}...HEAD", default_branch),
            format!("HEAD...origin/{}", default_branch),
            format!("{}...HEAD", default_branch),
            "master...HEAD".to_string(),
        ];
        for variant in &variants {
            if self.try_variant_sync(&mut changed_set, variant, project_path) {
                break;
            }
        }
        if changed_set.is_empty() {
            self.try_fallback_head_sync(&mut changed_set, project_path);
        }
        if changed_set.is_empty() {
            self.try_ls_files_sync(&mut changed_set, project_path);
        }
        let mut vec = Vec::with_capacity(changed_set.len());
        vec.extend(changed_set);
        FilePathList::new(vec)
    }

    /// Collect files matching a `--diff-filter` character (A/M/D).
    fn collect_by_filter(
        &self,
        project_path: &FilePath,
        default_branch: &str,
        filter: &str,
    ) -> FilePathList {
        let mut result_set: HashSet<FilePath> = HashSet::new();
        let variants = [
            format!("origin/{}...HEAD", default_branch),
            format!("HEAD...origin/{}", default_branch),
            format!("{}...HEAD", default_branch),
            "master...HEAD".to_string(),
        ];
        for variant in &variants {
            let args = ["diff", "--name-only", "--diff-filter", filter, variant];
            if self.try_variant_with_args(&mut result_set, &args, project_path) {
                break;
            }
        }
        if result_set.is_empty() {
            let args = ["diff", "--name-only", "--diff-filter", filter, "HEAD"];
            self.try_variant_with_args(&mut result_set, &args, project_path);
        }
        let mut vec = Vec::with_capacity(result_set.len());
        vec.extend(result_set);
        FilePathList::new(vec)
    }

    /// Collect renamed files (diff-filter=R) and parse the `R<score>\t<old>\t<new>` output.
    fn collect_by_filter_renamed(
        &self,
        project_path: &FilePath,
        default_branch: &str,
    ) -> RenamedFileList {
        let variants = [
            format!("origin/{}...HEAD", default_branch),
            format!("HEAD...origin/{}", default_branch),
            format!("{}...HEAD", default_branch),
            "master...HEAD".to_string(),
        ];
        for variant in &variants {
            let args = ["diff", "--name-status", "--diff-filter=R", variant];
            let result = self.io.run_git_command(&args, &project_path.value);
            if result.success && !result.stdout.trim().is_empty() {
                let pairs: Vec<RenamedFile> = self
                    .io
                    .parse_output_lines(&result.stdout)
                    .lines
                    .iter()
                    .filter_map(|line| {
                        let parts: Vec<&str> = line.splitn(3, '\t').collect();
                        if parts.len() == 3 && parts[0].starts_with('R') {
                            let old_path = FilePath::new(parts[1].to_string()).ok()?;
                            let new_path = FilePath::new(parts[2].to_string()).ok()?;
                            Some(RenamedFile::new(old_path, new_path))
                        } else {
                            None
                        }
                    })
                    .collect();
                if !pairs.is_empty() {
                    return RenamedFileList::new(pairs);
                }
            }
        }
        RenamedFileList::new(vec![])
    }

    fn try_variant_sync(
        &self,
        changed_set: &mut HashSet<FilePath>,
        variant: &str,
        project_path: &FilePath,
    ) -> bool {
        let result = self
            .io
            .run_git_command(&["diff", "--name-only", variant], &project_path.value);
        if result.success {
            for line in self.io.parse_output_lines(&result.stdout).lines.iter() {
                if let Ok(fp) = FilePath::new(line.as_str()) {
                    changed_set.insert(fp);
                }
            }
        }
        !changed_set.is_empty()
    }

    fn try_variant_with_args(
        &self,
        changed_set: &mut HashSet<FilePath>,
        args: &[&str],
        project_path: &FilePath,
    ) -> bool {
        let result = self.io.run_git_command(args, &project_path.value);
        if result.success {
            for line in self.io.parse_output_lines(&result.stdout).lines.iter() {
                if let Ok(fp) = FilePath::new(line.as_str()) {
                    changed_set.insert(fp);
                }
            }
        }
        !changed_set.is_empty()
    }

    fn try_fallback_head_sync(&self, changed_set: &mut HashSet<FilePath>, project_path: &FilePath) {
        let result = self
            .io
            .run_git_command(&["diff", "--name-only", "HEAD"], &project_path.value);
        if result.success {
            for line in self.io.parse_output_lines(&result.stdout).lines.iter() {
                if let Ok(fp) = FilePath::new(line.as_str()) {
                    changed_set.insert(fp);
                }
            }
        }
    }

    fn try_ls_files_sync(&self, changed_set: &mut HashSet<FilePath>, project_path: &FilePath) {
        let result = self.io.run_git_command(
            &["ls-files", "--modified", "--others", "--exclude-standard"],
            &project_path.value,
        );
        if result.success {
            for line in self.io.parse_output_lines(&result.stdout).lines.iter() {
                if let Ok(fp) = FilePath::new(line.as_str()) {
                    changed_set.insert(fp);
                }
            }
        }
    }
}
