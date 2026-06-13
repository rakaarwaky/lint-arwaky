// PURPOSE: GitDiffService — shared git diff/config operations used by git hooks domain (infrastructure layer)
use shared::git_hooks::taxonomy_hook_error::GitHookError;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use std::collections::HashSet;

pub struct GitDiffService;

impl GitDiffService {
    pub fn new() -> Self {
        Self
    }

    pub fn get_default_branch(&self, project_path: &FilePath) -> String {
        let result = std::process::Command::new("git")
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

    pub fn collect_changed_files(
        &self,
        project_path: &FilePath,
        default_branch: &str,
    ) -> Vec<FilePath> {
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
        changed_set.into_iter().collect()
    }

    pub fn try_variant(
        &self,
        changed_set: &mut HashSet<FilePath>,
        variant: &str,
        project_path: &FilePath,
    ) -> bool {
        if let Ok(output) = std::process::Command::new("git")
            .args(["diff", "--name-only", variant])
            .current_dir(&project_path.value)
            .output()
        {
            if output.status.success() {
                for line in String::from_utf8_lossy(&output.stdout).lines() {
                    let line = line.trim();
                    if !line.is_empty() {
                        changed_set.insert(FilePath::new(line).unwrap_or_default());
                    }
                }
            }
        }
        !changed_set.is_empty()
    }

    pub fn try_fallback_head(
        &self,
        changed_set: &mut HashSet<FilePath>,
        project_path: &FilePath,
    ) {
        if let Ok(output) = std::process::Command::new("git")
            .args(["diff", "--name-only", "HEAD"])
            .current_dir(&project_path.value)
            .output()
        {
            if output.status.success() {
                for line in String::from_utf8_lossy(&output.stdout).lines() {
                    let line = line.trim();
                    if !line.is_empty() {
                        changed_set.insert(FilePath::new(line).unwrap_or_default());
                    }
                }
            }
        }
    }

    pub fn try_ls_files(
        &self,
        changed_set: &mut HashSet<FilePath>,
        project_path: &FilePath,
    ) {
        if let Ok(output) = std::process::Command::new("git")
            .args([
                "ls-files",
                "--modified",
                "--others",
                "--exclude-standard",
            ])
            .current_dir(&project_path.value)
            .output()
        {
            if output.status.success() {
                for line in String::from_utf8_lossy(&output.stdout).lines() {
                    let line = line.trim();
                    if !line.is_empty() {
                        changed_set.insert(FilePath::new(line).unwrap_or_default());
                    }
                }
            }
        }
    }

    pub fn resolve_config_path(&self, base_path: &str) -> String {
        format!("{}/lint_arwaky.config.yaml", base_path)
    }

    pub fn validate_config_exists(&self, config_path: &str) -> Result<(), String> {
        if std::path::Path::new(config_path).exists() {
            Ok(())
        } else {
            Err(format!("Config file not found: {}", config_path))
        }
    }
}
