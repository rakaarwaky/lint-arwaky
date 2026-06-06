use crate::taxonomy::{Cause, ErrorCode, ErrorMessage, FilePath};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SemanticError {
    #[serde(default)]
    pub path: Option<FilePath>,
    pub message: ErrorMessage,
    #[serde(default)]
    pub error_code: Option<ErrorCode>,
    #[serde(default)]
    pub cause: Option<Cause>,
}

impl SemanticError {
    pub fn new(message: ErrorMessage) -> Self {
        Self { path: None, message, error_code: None, cause: None }
    }
}

impl std::fmt::Display for SemanticError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let target = self.path.as_ref().map(|p| format!(" on {}", p)).unwrap_or_default();
        let code = self.error_code.as_ref().map(|c| format!(" [{}]", c)).unwrap_or_default();
        write!(f, "Semantic Error{}{}: {}", target, code, self.message)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScopeResolutionError {
    #[serde(flatten)]
    pub base: SemanticError,
}

impl ScopeResolutionError {
    pub fn new(message: ErrorMessage) -> Self {
        Self { base: SemanticError::new(message) }
    }
}

impl std::fmt::Display for ScopeResolutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.base)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CallChainError {
    #[serde(flatten)]
    pub base: SemanticError,
}

impl CallChainError {
    pub fn new(message: ErrorMessage) -> Self {
        Self { base: SemanticError::new(message) }
    }
}

impl std::fmt::Display for CallChainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.base)
    }
}
