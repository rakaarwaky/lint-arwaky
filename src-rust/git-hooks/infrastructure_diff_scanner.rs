use crate::file_system::taxonomy_filesystem_error::FileSystemError;
use crate::pipeline_jobs::taxonomy_action_vo::ActionName;
use crate::shared_common::taxonomy_common_error::ErrorMessage;
/// git_diff_scanner — Git-aware file change detection for linting only modified files.
use crate::source_parsing::contract_scanner_provider_port::IScannerProviderPort;
use crate::source_parsing::taxonomy_path_vo::DirectoryPath;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use crate::source_parsing::taxonomy_paths_vo::FilePathList;
use crate::source_parsing::taxonomy_paths_vo::RenamedFile;
use crate::source_parsing::taxonomy_paths_vo::RenamedFileList;

pub struct DiffResult {
    pub added: FilePathList,
    pub modified: FilePathList,
    pub deleted: FilePathList,
    pub renamed: RenamedFileList,
}

impl DiffResult {
    pub fn new(
        added: FilePathList,
        modified: FilePathList,
        deleted: FilePathList,
        renamed: RenamedFileList,
    ) -> Self {
        Self {
            added,
            modified,
            deleted,
            renamed,
        }
    }

    pub fn all_files(&self) -> FilePathList {
        let mut combined: Vec<FilePath> = self.added.values.clone();
        combined.extend(self.modified.values.clone());
        for r in &self.renamed.values {
            combined.push(r.new_path.clone());
        }
        FilePathList::new(combined)
    }
}

pub struct GitDiffScanner {
    root: Option<DirectoryPath>,
}

impl GitDiffScanner {
    pub fn new(root: Option<DirectoryPath>) -> Self {
        Self { root }
    }

    fn parse_diff_output(stdout: &str) -> DiffResult {
        let mut added = Vec::new();
        let mut modified = Vec::new();
        let mut deleted = Vec::new();
        let mut renamed = Vec::new();
        for line in stdout.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.is_empty() {
                continue;
            }
            let status = parts[0];
            match status.chars().next() {
                Some('A') if parts.len() > 1 => {
                    if let Ok(fp) = FilePath::new(parts[1].to_string()) {
                        added.push(fp);
                    }
                }
                Some('M') if parts.len() > 1 => {
                    if let Ok(fp) = FilePath::new(parts[1].to_string()) {
                        modified.push(fp);
                    }
                }
                Some('D') if parts.len() > 1 => {
                    if let Ok(fp) = FilePath::new(parts[1].to_string()) {
                        deleted.push(fp);
                    }
                }
                Some('R') if parts.len() > 2 => {
                    let old = FilePath::new(parts[1].to_string());
                    let new = FilePath::new(parts[2].to_string());
                    if let (Ok(old_fp), Ok(new_fp)) = (old, new) {
                        renamed.push(RenamedFile {
                            old_path: old_fp,
                            new_path: new_fp,
                        });
                    }
                }
                _ => {}
            }
        }
        DiffResult::new(
            FilePathList::new(added),
            FilePathList::new(modified),
            FilePathList::new(deleted),
            RenamedFileList::new(renamed),
        )
    }

    pub fn filter_by_extensions(&self, files: &FilePathList, extensions: &[&str]) -> FilePathList {
        let filtered: Vec<FilePath> = files
            .values
            .iter()
            .filter(|f| extensions.iter().any(|ext| f.value.ends_with(ext)))
            .cloned()
            .collect();
        FilePathList::new(filtered)
    }
}

impl IScannerProviderPort for GitDiffScanner {
    fn scan_directory(&self, path: &DirectoryPath) -> Result<FilePathList, FileSystemError> {
        let work_dir = if path.value.is_empty() {
            "."
        } else {
            &path.value
        };
        let output = std::process::Command::new("git")
            .args(["diff", "--name-status", "HEAD"])
            .current_dir(work_dir)
            .output()
            .map_err(|e| {
                FileSystemError::new(
                    FilePath::new(".".to_string()).unwrap_or_default(),
                    ErrorMessage::new(format!("git diff failed: {}", e)),
                    ActionName::new("git_diff".to_string()),
                )
            })?;

        if !output.status.success() {
            return Err(FileSystemError::new(
                FilePath::new(".".to_string()).unwrap_or_default(),
                ErrorMessage::new(String::from_utf8_lossy(&output.stderr).to_string()),
                ActionName::new("git_diff".to_string()),
            ));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let diff = Self::parse_diff_output(&stdout);
        Ok(diff.all_files())
    }

    fn get_ignored_files(&self) -> FilePathList {
        if let Some(root) = &self.root {
            let output = std::process::Command::new("git")
                .args(["ls-files", "--others", "--ignored", "--exclude-standard"])
                .current_dir(&root.value)
                .output()
                .ok();
            if let Some(out) = output {
                if out.status.success() {
                    let files: Vec<FilePath> = String::from_utf8_lossy(&out.stdout)
                        .lines()
                        .filter(|l| !l.is_empty())
                        .filter_map(|l| FilePath::new(l.to_string()).ok())
                        .collect();
                    return FilePathList::new(files);
                }
            }
        }
        FilePathList::new(Vec::new())
    }
}
