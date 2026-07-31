// PURPOSE: Taxonomy layer — filesystem domain value objects
// Shared across all crates that need file I/O, parsing, or dependency graph types.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Supported programming languages for AST parsing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language {
    Rust,
    Python,
    TypeScript,
    JavaScript,
}

impl Language {
    /// Detect language from file extension.
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext {
            "rs" => Some(Self::Rust),
            "py" => Some(Self::Python),
            "ts" | "tsx" => Some(Self::TypeScript),
            "js" | "jsx" => Some(Self::JavaScript),
            _ => None,
        }
    }

    /// All supported extensions.
    pub fn extensions() -> &'static [&'static str] {
        &["rs", "py", "ts", "tsx", "js", "jsx"]
    }

    /// Human-readable name.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Rust => "rust",
            Self::Python => "python",
            Self::TypeScript => "typescript",
            Self::JavaScript => "javascript",
        }
    }
}

/// A discovered source file with metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    /// Absolute path to the file.
    pub path: PathBuf,
    /// File extension (without dot).
    pub extension: String,
    /// Detected language.
    pub language: Language,
    /// File size in bytes.
    pub size: u64,
}

/// Import type extracted from AST.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ImportType {
    /// Rust: `use foo::bar`
    Use,
    /// Rust: `mod foo`
    Mod,
    /// Python: `import foo` or `from foo import bar`
    Import,
    /// TypeScript/JS: `import foo from 'bar'`
    ImportFrom,
    /// TypeScript/JS: `require('bar')`
    Require,
    /// Re-export (`pub use`, `export ... from`)
    ReExport,
}

/// A single import/dependency relationship.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportEntry {
    /// File that contains the import.
    pub source_file: PathBuf,
    /// Raw import path (as written in source).
    pub raw_path: String,
    /// Resolved absolute module path (if resolvable).
    pub resolved_path: Option<PathBuf>,
    /// Type of import.
    pub import_type: ImportType,
    /// Language of the source file.
    pub language: Language,
    /// Is this a dynamic import? (e.g., `import()`, conditional)
    pub is_dynamic: bool,
    /// Was the import resolved to a file in the workspace?
    pub is_resolved: bool,
}

/// Result of a complete filesystem scan.
#[derive(Debug)]
pub struct FilesystemResult {
    /// All discovered source files.
    pub files: Vec<FileEntry>,
    /// Extracted imports for each file.
    pub imports: Vec<ImportEntry>,
    /// Number of files parsed successfully.
    pub parsed_count: usize,
    /// Number of files that failed to parse.
    pub parse_errors: usize,
    /// Number of imports that couldn't be resolved.
    pub unresolved_imports: usize,
    /// Timing breakdown (milliseconds).
    pub timing: ScanTiming,
}

/// Timing breakdown for scan stages.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ScanTiming {
    pub walk_ms: u64,
    pub cache_ms: u64,
    pub parse_ms: u64,
    pub extract_ms: u64,
    pub graph_ms: u64,
    pub total_ms: u64,
}
