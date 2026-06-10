// PURPOSE: ErrorCode — value object for AES error code identification
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

/// error_code_vo — Error code value object.
///
/// Linter error code.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ErrorCode {
    code: String,
}

impl ErrorCode {
    pub fn code(&self) -> &str {
        &self.code
    }
    /// Create a new ErrorCode from a string.
    ///
    /// # Errors
    /// Returns an error if the code is empty.
    pub fn new<S: Into<String>>(code: S) -> Result<Self, String> {
        let code = code.into();
        if code.is_empty() {
            return Err("Error code cannot be empty".to_string());
        }
        Ok(ErrorCode { code })
    }

    /// Create a raw ErrorCode without error validation.
    pub fn raw<S: Into<String>>(code: S) -> Self {
        ErrorCode { code: code.into() }
    }

    /// Returns true if the code is a style error (starts with E, W, or D).
    pub fn is_style(&self) -> bool {
        self.code.starts_with('E') || self.code.starts_with('W') || self.code.starts_with('D')
    }
    pub fn is_logic(&self) -> bool {
        self.code.starts_with('F') || self.code.starts_with('I')
    }
    pub fn is_security(&self) -> bool {
        self.code.starts_with('B')
    }
    pub fn is_architecture(&self) -> bool {
        self.code.starts_with("AES")
    }
}

impl std::ops::Deref for ErrorCode {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.code
    }
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code)
    }
}

impl Hash for ErrorCode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.code.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::ErrorCode;

    #[test]
    fn test_error_code_new() {
        let ec = ErrorCode::new("E123").unwrap_or_default();
        assert_eq!(ec.code, "E123");
        assert!(ec.is_style());
        assert!(!ec.is_logic());
        assert!(!ec.is_security());
        assert!(!ec.is_architecture());

        let ec = ErrorCode::new("W999").unwrap_or_default();
        assert!(ec.is_style());

        let ec = ErrorCode::new("D404").unwrap_or_default();
        assert!(ec.is_style());

        let ec = ErrorCode::new("F001").unwrap_or_default();
        assert!(ec.is_logic());

        let ec = ErrorCode::new("I999").unwrap_or_default();
        assert!(ec.is_logic());

        let ec = ErrorCode::new("B001").unwrap_or_default();
        assert!(ec.is_security());

        let ec = ErrorCode::new("AES123").unwrap_or_default();
        assert!(ec.is_architecture());
    }

    #[test]
    fn test_error_code_invalid() {
        assert!(ErrorCode::new("").is_err());
    }
}
