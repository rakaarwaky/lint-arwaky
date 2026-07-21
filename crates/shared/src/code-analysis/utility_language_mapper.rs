// PURPOSE: Stateless utility functions for mapping language detection results
use crate::code_analysis::taxonomy_violation_code_analysis_vo::Language as CodeAnalysisLanguage;
use crate::common::taxonomy_language_vo::Language as CommonLanguage;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::utility_language_detector;

/// Map a file path to the code-analysis Language enum.
pub fn code_analysis_language_from_file(file: &str) -> CodeAnalysisLanguage {
    let Ok(fp) = FilePath::new(file.to_string()) else {
        return CodeAnalysisLanguage::Rust;
    };
    match utility_language_detector::detect_language(&fp) {
        CommonLanguage::Rust => CodeAnalysisLanguage::Rust,
        CommonLanguage::Python => CodeAnalysisLanguage::Python,
        CommonLanguage::JavaScript => CodeAnalysisLanguage::JavaScript,
        CommonLanguage::TypeScript => CodeAnalysisLanguage::TypeScript,
        CommonLanguage::Unknown => CodeAnalysisLanguage::Rust,
    }
}
