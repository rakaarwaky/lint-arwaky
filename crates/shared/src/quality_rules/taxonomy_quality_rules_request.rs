// PURPOSE: CodeAnalysisRequest — request payload for the code_analysis aggregate

use shared_filesystem::taxonomy_filesystem_vo::FileEntry;

pub enum CodeAnalysisRequest {
    /// Run quality checks on pre-parsed file entries from the filesystem crate.
    RunAnalysis { files: Vec<FileEntry> },
    /// Return the name of the capability.
    Name,
}

impl CodeAnalysisRequest {
    pub fn run_analysis(files: &[FileEntry]) -> Self {
        Self::RunAnalysis {
            files: files.to_vec(),
        }
    }
    pub fn name() -> Self {
        Self::Name
    }
}
