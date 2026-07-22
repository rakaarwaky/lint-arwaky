use std::collections::HashSet;

use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::common::taxonomy_git_vo::GitBranchName;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::common::taxonomy_paths_vo::RenamedFileList;
use shared::file_watch::taxonomy_diff_result_vo::GitDiffResultVO;
use shared::git_hooks::contract_diff_protocol::IDiffProtocol;
use shared::git_hooks::utility_git_io as git_io;

// PURPOSE: DiffChecker — implements IDiffProtocol for git diff analysis (capabilities layer)

// ─── Block 1: Struct Definition ───────────────────────────

pub struct DiffChecker;

// ─── Block 2: Protocol Trait Implementation ───────────────

#[async_trait::async_trait]
impl IDiffProtocol for DiffChecker {
    async fn run_git_diff_check(&self, path: &FilePath) -> LintResultList {
        let default_branch = self.get_default_branch(path);
        let _changed_files = self.collect_changed_files(path, &default_branch);
        LintResultList::new(Vec::new())
    }

    async fn get_diff(&self, path: &FilePath) -> GitDiffResultVO {
        let default_branch = self.get_default_branch(path);
        let changed_files = self.collect_changed_files(path, &default_branch);
        let lintable_vec: Vec<FilePath> = changed_files
            .values
            .iter()
            .filter(|f| {
                let ext = f.extension();
                matches!(
                    ext.as_str(),
                    "rs" | "py"
                        | "ts"
                        | "js"
                        | "jsx"
                        | "tsx"
                        | "md"
                        | "toml"
                        | "json"
                        | "yaml"
                        | "yml"
                )
            })
            .cloned()
            .collect();
        let lintable_files = FilePathList::new(lintable_vec);
        GitDiffResultVO {
            added: FilePathList::new(Vec::new()),
            modified: changed_files.clone(),
            deleted: FilePathList::new(Vec::new()),
            renamed: RenamedFileList::new(vec![]),
            lintable_files,
            all_files: changed_files.clone(),
            total_changed: shared::taxonomy_common_vo::Count::new(changed_files.values.len() as i64),
        }
    }

    async fn get_changed_files(&self, path: &FilePath, base: &GitBranchName) -> FilePathList {
        let branch_str = if base.value().is_empty() || base.value() == "." {
            self.get_default_branch(path)
        } else {
            base.value().to_string()
        };
        self.collect_changed_files(path, &branch_str)
    }

    async fn get_default_branch(&self, path: &FilePath) -> GitBranchName {
        GitBranchName::new(self.get_default_branch(path))
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for DiffChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl DiffChecker {
    pub fn new() -> Self {
        Self
    }

    fn get_default_branch(&self, project_path: &FilePath) -> String {
        let (stdout, _, success) = git_io::run_git_command(
            &["symbolic-ref", "refs/remotes/origin/HEAD"],
            &project_path.value,
        );
        if success {
            let ref_str = stdout.trim().to_string();
            if let Some(branch) = ref_str.rsplit('/').next() {
                if !branch.is_empty() {
                    return branch.to_string();
                }
            }
        }
        "main".to_string()
    }

    fn collect_changed_files(&self, project_path: &FilePath, default_branch: &str) -> FilePathList {
        let mut changed_set: HashSet<FilePath> = HashSet::new();
        let variants = [
            format!("origin/{}...HEAD", default_branch),
            format!("HEAD...origin/{}", default_branch),
            format!("{}...HEAD", default_branch),
            "master...HEAD".to_string(),
        ];
        for variant in &variants {
            if self.try_variant(&mut changed_set, variant, project_path) {
                break;
            }
        }
        if changed_set.is_empty() {
            self.try_fallback_head(&mut changed_set, project_path);
        }
        if changed_set.is_empty() {
            self.try_ls_files(&mut changed_set, project_path);
        }
        FilePathList::new(changed_set.into_iter().collect())
    }

    fn try_variant(
        &self,
        changed_set: &mut HashSet<FilePath>,
        variant: &str,
        project_path: &FilePath,
    ) -> bool {
        let (stdout, _, success) =
            git_io::run_git_command(&["diff", "--name-only", variant], &project_path.value);
        if success {
            for line in git_io::parse_output_lines(&stdout) {
                if let Ok(fp) = FilePath::new(&line) {
                    changed_set.insert(fp);
                }
            }
        }
        !changed_set.is_empty()
    }

    fn try_fallback_head(&self, changed_set: &mut HashSet<FilePath>, project_path: &FilePath) {
        let (stdout, _, success) =
            git_io::run_git_command(&["diff", "--name-only", "HEAD"], &project_path.value);
        if success {
            for line in git_io::parse_output_lines(&stdout) {
                if let Ok(fp) = FilePath::new(&line) {
                    changed_set.insert(fp);
                }
            }
        }
    }

    fn try_ls_files(&self, changed_set: &mut HashSet<FilePath>, project_path: &FilePath) {
        let (stdout, _, success) = git_io::run_git_command(
            &["ls-files", "--modified", "--others", "--exclude-standard"],
            &project_path.value,
        );
        if success {
            for line in git_io::parse_output_lines(&stdout) {
                if let Ok(fp) = FilePath::new(&line) {
                    changed_set.insert(fp);
                }
            }
        }
    }
}
