// PURPOSE: Language — value object enum for supported programming languages (Python, JS, TS, Rust)
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
    pub fn as_str(&self) -> &'static str {
        match self {
            Language::Python => "python",
            Language::JavaScript => "javascript",
            Language::TypeScript => "typescript",
            Language::Rust => "rust",
            Language::Unknown => "unknown",
        }
    }

    /// Return the keyword for declaring a type/interface in this language.
    pub fn type_kw(&self) -> &'static str {
        match self {
            Language::Rust => "type",
            Language::JavaScript | Language::TypeScript => "interface/type",
            Language::Python => "Protocol/type",
            Language::Unknown => "type",
        }
    }

    /// Return the keyword for declaring an interface/trait in this language.
    pub fn interface_kw(&self) -> &'static str {
        match self {
            Language::Rust => "trait",
            Language::JavaScript | Language::TypeScript => "interface",
            Language::Python => "Protocol",
            Language::Unknown => "interface",
        }
    }

    /// Return the keyword for declaring a struct/class in this language.
    pub fn struct_keyword(&self) -> &'static str {
        match self {
            Language::Rust => "struct",
            Language::JavaScript | Language::TypeScript => "class/interface",
            Language::Python => "class/Protocol",
            Language::Unknown => "class",
        }
    }

    /// Return the keyword for inheritance in this language.
    pub fn inherits_kw(&self) -> &'static str {
        match self {
            Language::Rust => "implements",
            Language::JavaScript | Language::TypeScript => "implements/extends",
            Language::Python => "implements/inherits",
            Language::Unknown => "inherits",
        }
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
