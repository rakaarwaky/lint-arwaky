// PURPOSE: ILanguageDetectorProtocol — protocol trait for detecting programming language from file path

use crate::common::taxonomy_language_vo::Language;
use crate::common::taxonomy_path_vo::FilePath;

pub trait ILanguageDetectorProtocol: Send + Sync {
    fn detect(&self, path: &FilePath) -> Language;
    fn is_lintable(&self, path: &FilePath) -> bool;
}
