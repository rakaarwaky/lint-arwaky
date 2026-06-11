// PURPOSE: CommandsOrchestrator — orchestrates git hook operations (install, uninstall, run, update)

use git_hooks::contract_commands_aggregate::GitCommandsAggregate;
use git_hooks::taxonomy_diff_result_vo::GitDiffResultVO;
use output_report::taxonomy_result_vo::LintResultList;
use shared_common::taxonomy_common_vo::Count;
use source_parsing::taxonomy_path_vo::FilePath;
use source_parsing::taxonomy_paths_vo::FilePathList;
use source_parsing::taxonomy_paths_vo::RenamedFileList;
use async_trait::async_trait;
use std::collections::HashSet;

pub struct GitCommandsOrchestrator {
    git_path: String,
}

#[async_trait]
impl GitCommandsAggregate for GitCommandsOrchestrator {
    async fn run_git_diff_check(&self, path: &FilePath) -> LintResultList {
        let default_branch = self.get_default_branch(path);
        let _changed_files = self.collect_changed_files(path, &default_branch);
        LintResultList::new(Vec::new())
    }

    async fn get_diff(&self, path: &FilePath) -> GitDiffResultVO {
        let default_branch = self.get_default_branch(path);
        let changed_files = self.collect_changed_files(path, &default_branch);
        let filtered = changed_files.clone();
        GitDiffResultVO {
            added: FilePathList::new(Vec::new()),
            modified: filtered.clone(),
            deleted: FilePathList::new(Vec::new()),
            renamed: RenamedFileList::new(vec![]),
            lintable_files: changed_files.clone(),
            all_files: changed_files,
            total_changed: Count::new(filtered.values.len() as i64),
        }
    }
}

impl Default for GitCommandsOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

impl GitCommandsOrchestrator {
    pub fn new() -> Self {
        let git = std::process::Command::new("which")
            .arg("git")
            .output()
            .ok()
            .filter(|o| o.status.success())
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_else(|| "git".to_string());
        Self { git_path: git }
    }

    fn get_default_branch(&self, project_path: &FilePath) -> String {
        let result = std::process::Command::new(&self.git_path)
            .args(["symbolic-ref", "refs/remotes/origin/HEAD"])
            .current_dir(&project_path.value)
            .output()
            .ok();
        if let Some(output) = result {
            if output.status.success() {
                let ref_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if let Some(branch) = ref_str.rsplit('/').next() {
                    if !branch.is_empty() {
                        return branch.to_string();
                    }
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
        if let Ok(output) = std::process::Command::new(&self.git_path)
            .args(["diff", "--name-only", variant])
            .current_dir(&project_path.value)
            .output()
        {
            if output.status.success() {
                for line in String::from_utf8_lossy(&output.stdout).lines() {
                    let line = line.trim();
                    if !line.is_empty() {
                        let fp = FilePath::new(line).unwrap_or_default();
                        changed_set.insert(fp);
                    }
                }
            }
        }
        !changed_set.is_empty()
    }

    fn try_fallback_head(&self, changed_set: &mut HashSet<FilePath>, project_path: &FilePath) {
        if let Ok(output) = std::process::Command::new(&self.git_path)
            .args(["diff", "--name-only", "HEAD"])
            .current_dir(&project_path.value)
            .output()
        {
            if output.status.success() {
                for line in String::from_utf8_lossy(&output.stdout).lines() {
                    let line = line.trim();
                    if !line.is_empty() {
                        let fp = FilePath::new(line).unwrap_or_default();
                        changed_set.insert(fp);
                    }
                }
            }
        }
    }

    fn try_ls_files(&self, changed_set: &mut HashSet<FilePath>, project_path: &FilePath) {
        if let Ok(output) = std::process::Command::new(&self.git_path)
            .args(["ls-files", "--modified", "--others", "--exclude-standard"])
            .current_dir(&project_path.value)
            .output()
        {
            if output.status.success() {
                for line in String::from_utf8_lossy(&output.stdout).lines() {
                    let line = line.trim();
                    if !line.is_empty() {
                        let fp = FilePath::new(line).unwrap_or_default();
                        changed_set.insert(fp);
                    }
                }
            }
        }
    }
}
