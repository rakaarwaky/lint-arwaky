// PURPOSE: DiffChecker — IDiffProtocol implementation for git diff analysis (capabilities layer)

use std::collections::HashSet;

use shared::cli_commands::LintResultList;
use shared::common::taxonomy_common_vo::Count;
use shared::common::taxonomy_git_vo::GitBranchName;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::{FilePathList, RenamedFile, RenamedFileList};
use shared::file_watch::GitDiffResultVO;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::git_hooks::contract_diff_protocol::IDiffProtocol;

use std::sync::Arc;

/// Lintable file extensions (source code only).
const LINTABLE_EXTENSIONS: &[&str] = &["rs", "py", "ts", "js", "jsx", "tsx"];

/// Returns `true` if the file extension is a lintable source type.
pub fn is_lintable_file(fp: &FilePath) -> bool {
    let ext = fp.extension();
    LINTABLE_EXTENSIONS.contains(&ext.as_str())
}

// ─── Block 1: Struct Definition ───────────────────────────

pub struct DiffChecker {
    filesystem: Arc<dyn IFilesystemAggregate>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IDiffProtocol for DiffChecker {
    fn run_git_diff_check(&self, path: &FilePath) -> LintResultList {
        let default_branch = self.get_default_branch_sync(path);
        let changed_files = self.collect_changed_files_sync(path, &default_branch);

        // Filter to lintable source files only
        let _lintable: Vec<FilePath> = changed_files
            .values
            .iter()
            .filter(|f| is_lintable_file(f))
            .cloned()
            .collect();

        // TODO: delegate to linter aggregates for AES analysis on lintable files.
        // Requires linter aggregate integration — returns empty for now.
        LintResultList::new(Vec::new())
    }

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
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl DiffChecker {
    pub fn new(filesystem: Arc<dyn IFilesystemAggregate>) -> Self {
        Self { filesystem }
    }

    fn get_default_branch_sync(&self, project_path: &FilePath) -> String {
        let (stdout, _, success) = self.filesystem.run_git_command(
            &["symbolic-ref", "refs/remotes/origin/HEAD"],
            &project_path.value,
        );
        if success {
            let ref_str = stdout.trim().to_string();
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

    /// Collect renamed files (diff-filter=R) and parse the `old -> new` output.
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
            let args = ["diff", "--name-only", "--diff-filter=R", variant];
            let (stdout, _, success) = self.filesystem.run_git_command(&args, &project_path.value);
            if success && !stdout.trim().is_empty() {
                let pairs: Vec<RenamedFile> = self
                    .filesystem
                    .parse_output_lines(&stdout)
                    .iter()
                    .filter_map(|line| {
                        let parts: Vec<&str> = line.splitn(2, " => ").collect();
                        if parts.len() == 2 {
                            let old_path = FilePath::new(parts[0].to_string()).ok()?;
                            let new_path = FilePath::new(parts[1].to_string()).ok()?;
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
        let (stdout, _, success) = self
            .filesystem
            .run_git_command(&["diff", "--name-only", variant], &project_path.value);
        if success {
            for line in self.filesystem.parse_output_lines(&stdout) {
                if let Ok(fp) = FilePath::new(&line) {
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
        let (stdout, _, success) = self.filesystem.run_git_command(args, &project_path.value);
        if success {
            for line in self.filesystem.parse_output_lines(&stdout) {
                if let Ok(fp) = FilePath::new(&line) {
                    changed_set.insert(fp);
                }
            }
        }
        !changed_set.is_empty()
    }

    fn try_fallback_head_sync(&self, changed_set: &mut HashSet<FilePath>, project_path: &FilePath) {
        let (stdout, _, success) = self
            .filesystem
            .run_git_command(&["diff", "--name-only", "HEAD"], &project_path.value);
        if success {
            for line in self.filesystem.parse_output_lines(&stdout) {
                if let Ok(fp) = FilePath::new(&line) {
                    changed_set.insert(fp);
                }
            }
        }
    }

    fn try_ls_files_sync(&self, changed_set: &mut HashSet<FilePath>, project_path: &FilePath) {
        let (stdout, _, success) = self.filesystem.run_git_command(
            &["ls-files", "--modified", "--others", "--exclude-standard"],
            &project_path.value,
        );
        if success {
            for line in self.filesystem.parse_output_lines(&stdout) {
                if let Ok(fp) = FilePath::new(&line) {
                    changed_set.insert(fp);
                }
            }
        }
    }
}
