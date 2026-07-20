// PURPOSE: Language detection — pure, stateless taxonomy utility.
//
// Detects programming language from a `FilePath` by extension. Domain-agnostic,
// free functions only (no struct, no port/contract import) so it stays a legal
// taxonomy utility per AES201 and the helper-vs-utility boundary.
use crate::common::taxonomy_language_vo::Language;
use crate::common::taxonomy_path_vo::FilePath;

/// Detect the programming language of a file from its extension.
pub fn detect_language(path: &FilePath) -> Language {
    match path.extension().as_str() {
        "py" => Language::Python,
        "js" | "jsx" | "mjs" | "cjs" => Language::JavaScript,
        "ts" | "tsx" | "mts" | "cts" => Language::TypeScript,
        "rs" => Language::Rust,
        _ => Language::Unknown,
    }
}

/// Whether the file's language is one this linter can process.
pub fn is_lintable(path: &FilePath) -> bool {
    matches!(
        detect_language(path),
        Language::Python | Language::JavaScript | Language::TypeScript | Language::Rust
    )
}
