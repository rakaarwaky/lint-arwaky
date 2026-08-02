// PURPOSE: DiffChecker — IDiffProtocol implementation for git diff analysis (capabilities layer)

use std::collections::HashSet;

use shared::cli_commands::LintResultList;
use shared::common::{Count, FilePath, FilePathList, GitBranchName, RenamedFileList};
use shared::file_watch::GitDiffResultVO;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::git_hooks::IDiffProtocol;

use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct DiffChecker {
    filesystem: Arc<dyn IFilesystemAggregate>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IDiffProtocol for DiffChecker {
    fn run_git_diff_check(&self, path: &FilePath) -> LintResultList {
        let default_branch = self.get_default_branch_sync(path);
        let _changed_files = self.collect_changed_files_sync(path, &default_branch);
        LintResultList::new(Vec::new())
    }

    fn get_diff(&self, path: &FilePath) -> GitDiffResultVO {
        let default_branch = self.get_default_branch_sync(path);
        let changed_files = self.collect_changed_files_sync(path, &default_branch);
        let lintable_vec: Vec<FilePath> = changed_files
            .values
            .iter()
            .filter(|f| {
                let ext = f.extension();
                matches!(ext.as_str(), "rs" | "py" | "ts" | "js" | "jsx" | "tsx")
            })
            .cloned()
            .collect();
        let lintable_files = FilePathList::new(lintable_vec);
        let total_count = changed_files.values.len() as i64;
        GitDiffResultVO {
            added: FilePathList::new(Vec::new()),
            modified: changed_files.clone(),
            deleted: FilePathList::new(Vec::new()),
            renamed: RenamedFileList::new(vec![]),
            lintable_files,
            all_files: changed_files,
            total_changed: Count::new(total_count),
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

    fn try_fallback_head_sync(
        &self,
        changed_set: &mut HashSet<FilePath>,
        project_path: &FilePath,
    ) {
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

    fn try_ls_files_sync(
        &self,
        changed_set: &mut HashSet<FilePath>,
        project_path: &FilePath,
    ) {
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
