use std::str::FromStr;
// PURPOSE: Language — consolidated language enum for all feature crates
// Merged from: common/taxonomy_language_vo.rs (original), filesystem/taxonomy_filesystem_vo.rs, code-analysis/taxonomy_violation_code_analysis_vo.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language {
    Python,
    JavaScript,
    TypeScript,
    Rust,
    Unknown,
}

impl Language {
    // ── Detection ─────────────────────────────────────────────

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

    /// Detect language from adapter name (clippy, ruff, eslint, etc).
    pub fn from_adapter_name(name: &str) -> Self {
        match name.to_lowercase().as_str() {
            "clippy" | "rust" => Self::Rust,
            "eslint" | "prettier" | "tsc" | "javascript" => Self::JavaScript,
            "ruff" | "mypy" | "bandit" | "python" => Self::Python,
            "typescript" => Self::TypeScript,
            _ => Self::Unknown,
        }
    }

    // ── Metadata ──────────────────────────────────────────────

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
            Self::Unknown => "unknown",
        }
    }

    // ── Language-specific keywords ────────────────────────────

    pub fn struct_keyword(&self) -> &'static str {
        match self {
            Self::Rust => "struct",
            Self::JavaScript | Self::TypeScript => "class/interface",
            Self::Python => "class/Protocol",
            Self::Unknown => "class",
        }
    }

    pub fn interface_kw(&self) -> &'static str {
        match self {
            Self::Rust => "trait",
            Self::JavaScript | Self::TypeScript => "interface",
            Self::Python => "Protocol",
            Self::Unknown => "interface",
        }
    }

}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// ─── Merged from taxonomy_language_info_vo ────────────────────

/// LanguageInfo — pre-computed language flags (is_rs, is_py, is_js, lang).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LanguageInfo {
    pub is_rs: bool,
    pub is_py: bool,
    pub is_js: bool,
    pub lang: Language,
}

impl LanguageInfo {
    pub fn new(is_rs: bool, is_py: bool, is_js: bool, lang: Language) -> Self {
        Self {
            is_rs,
            is_py,
            is_js,
            lang,
        }
    }
}

// ─── Merged from taxonomy_config_language_vo ──────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConfigLanguage {
    Rust,
    Python,
    TypeScript,
}

impl ConfigLanguage {
    pub fn as_str(&self) -> &'static str {
        match self {
            ConfigLanguage::Rust => "rust",
            ConfigLanguage::Python => "python",
            ConfigLanguage::TypeScript => "typescript",
        }
    }

    pub fn config_file_names(&self) -> &'static [&'static str] {
        &["lint_arwaky.config.yaml"]
    }
}

impl std::fmt::Display for ConfigLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for ConfigLanguage {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "rust" => Ok(Self::Rust),
            "python" => Ok(Self::Python),
            "typescript" | "ts" => Ok(Self::TypeScript),
            "javascript" | "js" => Ok(Self::TypeScript),
            other => Err(format!(
                "Unsupported language '{other}'. Supported: rust, python, typescript"
            )),
        }
    }
}
