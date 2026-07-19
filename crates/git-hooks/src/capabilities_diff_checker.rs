// PURPOSE: DiffChecker — IDiffProtocol for git diff analysis (capabilities layer)
// Zero I/O: all git commands delegated to IGitCommandPort via DI.
use std::collections::HashSet;
use std::sync::Arc;

use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::common::taxonomy_paths_vo::RenamedFileList;
use shared::git_hooks::contract_diff_protocol::IDiffProtocol;
use shared::git_hooks::contract_git_command_port::IGitCommandPort;
use shared::git_hooks::taxonomy_diff_result_vo::GitDiffResultVO;

// Block 1: struct Definition
pub struct DiffChecker {
    git_command: Arc<dyn IGitCommandPort>,
}

// Block 2: impl Trait for Struct (Public Contract)
#[async_trait::async_trait]
impl IDiffProtocol for DiffChecker {
    async fn run_git_diff_check(&self, path: &FilePath) -> LintResultList {
        let default_branch = self.resolve_default_branch(path).await;
        let _changed_files = self.collect_changed_files(path, &default_branch).await;
        LintResultList::new(Vec::new())
    }

    async fn get_diff(&self, path: &FilePath) -> GitDiffResultVO {
        let default_branch = self.resolve_default_branch(path).await;
        let changed_files = self.collect_changed_files(path, &default_branch).await;
        let filtered = changed_files.clone();
        GitDiffResultVO {
            added: FilePathList::new(Vec::new()),
            modified: filtered.clone(),
            deleted: FilePathList::new(Vec::new()),
            renamed: RenamedFileList::new(vec![]),
            lintable_files: changed_files.clone(),
            all_files: changed_files,
            total_changed: shared::taxonomy_common_vo::Count::new(filtered.values.len() as i64),
        }
    }

    async fn get_changed_files(&self, path: &FilePath, base: &str) -> FilePathList {
        let branch = if base.is_empty() || base == "." {
            self.resolve_default_branch(path).await
        } else {
            base.to_string()
        };
        self.collect_changed_files(path, &branch).await
    }

    async fn get_default_branch(&self, path: &FilePath) -> String {
        self.resolve_default_branch(path).await
    }
}

// Block 3: constructors & helpers
impl DiffChecker {
    pub fn new(git_command: Arc<dyn IGitCommandPort>) -> Self {
        Self { git_command }
    }

    async fn resolve_default_branch(&self, project_path: &FilePath) -> String {
        self.git_command
            .symbolic_ref(project_path)
            .await
            .unwrap_or_else(|| "main".to_string())
    }

    async fn collect_changed_files(
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
            let files = self.git_command.diff_name_only(variant, project_path).await;
            for file_str in &files {
                if let Ok(fp) = FilePath::new(file_str.clone()) {
                    changed_set.insert(fp);
                }
            }
            if !changed_set.is_empty() {
                break;
            }
        }
        if changed_set.is_empty() {
            let files = self.git_command.diff_name_only("HEAD", project_path).await;
            for file_str in &files {
                if let Ok(fp) = FilePath::new(file_str.clone()) {
                    changed_set.insert(fp);
                }
            }
        }
        if changed_set.is_empty() {
            let files = self.git_command.ls_files_modified(project_path).await;
            for file_str in &files {
                if let Ok(fp) = FilePath::new(file_str.clone()) {
                    changed_set.insert(fp);
                }
            }
        }
        FilePathList::new(changed_set.into_iter().collect())
    }
}
