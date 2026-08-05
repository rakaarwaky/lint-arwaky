// PURPOSE: Stateless utility functions for mapping language detection results
use shared::common::taxonomy_language_vo::Language;
use shared::common::taxonomy_path_vo::FilePath;

/// Map a file path to the Language enum by extension.
pub fn code_analysis_language_from_file(file: &str) -> Language {
    let Ok(fp) = FilePath::new(file.to_string()) else {
        return Language::Rust;
    };
    match fp.extension().as_str() {
        "py" => Language::Python,
        "js" | "jsx" | "mjs" | "cjs" => Language::JavaScript,
        "ts" | "tsx" | "mts" | "cts" => Language::TypeScript,
        _ => Language::Rust,
    }
}
