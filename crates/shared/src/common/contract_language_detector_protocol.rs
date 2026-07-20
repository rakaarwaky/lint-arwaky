// PURPOSE: ILanguageDetectorProtocol — contract for detecting a file's source language.
// Abstracts language detection so implementations can be swapped via DI.
// The default implementation uses pure utility functions from
// utility_language_detector (free functions, no state).
use super::taxonomy_language_vo::Language;
use super::taxonomy_path_vo::FilePath;

/// Trait that detects the programming language of a file and checks
/// whether it is lintable by this toolchain.
pub trait ILanguageDetectorProtocol: Send + Sync {
    /// Detect the source language of `path`.
    fn detect(&self, path: &FilePath) -> Language;
    /// Whether the file's language is one this linter can process.
    fn is_lintable(&self, path: &FilePath) -> bool;
}
