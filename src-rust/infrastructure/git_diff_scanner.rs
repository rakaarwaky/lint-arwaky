/// git_diff_scanner — Git-aware file change detection for linting only modified files.
use crate::contract::{ICommandExecutorPort, IScannerProviderPort};
use crate::taxonomy::{DirectoryPath, FilePath, FilePathList, GitRef, RenamedFile, RenamedFileList};
use std::sync::Arc;

pub struct DiffResult {
    pub added: FilePathList,
    pub modified: FilePathList,
    pub deleted: FilePathList,
    pub renamed: RenamedFileList,
}

impl DiffResult {
    pub fn new(added: FilePathList, modified: FilePathList, deleted: FilePathList, renamed: RenamedFileList) -> Self {
        Self { added, modified, deleted, renamed }
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
    executor: Option<Arc<dyn ICommandExecutorPort>>,
}

impl GitDiffScanner {
    pub fn new(root: Option<DirectoryPath>, executor: Option<Arc<dyn ICommandExecutorPort>>) -> Self {
        Self { root, executor }
    }

    fn parse_diff_output(stdout: &str) -> DiffResult {
        let mut added = Vec::new();
        let mut modified = Vec::new();
        let mut deleted = Vec::new();
        let mut renamed = Vec::new();
        for line in stdout.lines() {
            let line = line.trim();
            if line.is_empty() { continue; }
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.is_empty() { continue; }
            let status = parts[0];
            match status.chars().next() {
                Some('A') if parts.len() > 1 => added.push(FilePath::new(parts[1].to_string())),
                Some('M') if parts.len() > 1 => modified.push(FilePath::new(parts[1].to_string())),
                Some('D') if parts.len() > 1 => deleted.push(FilePath::new(parts[1].to_string())),
                Some('R') if parts.len() > 2 => renamed.push(RenamedFile {
                    old_path: FilePath::new(parts[1].to_string()),
                    new_path: FilePath::new(parts[2].to_string()),
                }),
                _ => {}
            }
        }
        DiffResult::new(FilePathList::new(added), FilePathList::new(modified), FilePathList::new(deleted), RenamedFileList::new(renamed))
    }

    pub fn filter_by_extensions(&self, files: &FilePathList, extensions: &[&str]) -> FilePathList {
        let filtered: Vec<FilePath> = files.values.iter()
            .filter(|f| extensions.iter().any(|ext| f.value.ends_with(ext)))
            .cloned()
            .collect();
        FilePathList::new(filtered)
    }
}

impl IScannerProviderPort for GitDiffScanner {
    fn scan_directory(&self, path: DirectoryPath) -> FilePathList {
        FilePathList::new(Vec::new())
    }

    fn get_ignored_files(&self) -> FilePathList {
        FilePathList::new(Vec::new())
    }
}
