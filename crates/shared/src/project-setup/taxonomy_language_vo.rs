// PURPOSE: LanguageConfigVO — value object for programming language configuration
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectLanguage {
    pub value: String,
}

impl ProjectLanguage {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LanguageSource {
    pub language: String,
    pub confidence: u8,
    pub source: String,
}

impl LanguageSource {
    pub fn new(language: impl Into<String>, confidence: u8, source: impl Into<String>) -> Self {
        Self {
            language: language.into(),
            confidence,
            source: source.into(),
        }
    }
}
