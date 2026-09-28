// PURPOSE: ErrorCode, ErrorId — value objects for AES error code and numeric id identification
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
}

/// error_id_vo — Stable numeric error identifier.
///
/// Block allocation: common errors use `000X`; per-feature errors use `1XXX`
/// for feature 1, `2XXX` for feature 2, and so on. Ids never change across
/// releases — reformatting the message is compatible, changing the id is not.
#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, Copy, Hash, PartialOrd, Ord,
)]
#[serde(transparent)]
pub struct ErrorId {
    value: u16,
}

impl ErrorId {
    /// Numeric value of the error id.
    pub fn value(&self) -> u16 {
        self.value
    }

    /// Create a new ErrorId.
    ///
    /// # Errors
    /// Returns an error if the id is zero (zero is reserved as "unset").
    pub fn new(value: u16) -> Result<Self, String> {
        if value == 0 {
            return Err("Error id cannot be zero".to_string());
        }
        Ok(ErrorId { value })
    }

    /// Create an ErrorId without validation (for constants).
    pub const fn raw(value: u16) -> Self {
        ErrorId { value }
    }
}

impl std::ops::Deref for ErrorId {
    type Target = u16;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::fmt::Display for ErrorId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

/// Classifies an error code string as a style error (starts with E, W, or D).
pub fn error_code_is_style(code: &str) -> bool {
    code.starts_with('E') || code.starts_with('W') || code.starts_with('D')
}

/// Classifies an error code string as a logic error (starts with F or I).
pub fn error_code_is_logic(code: &str) -> bool {
    code.starts_with('F') || code.starts_with('I')
}

/// Classifies an error code string as a security error (starts with B).
pub fn error_code_is_security(code: &str) -> bool {
    code.starts_with('B')
}

/// Classifies an error code string as an architecture error (starts with AES).
pub fn error_code_is_architecture(code: &str) -> bool {
    code.starts_with("AES")
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
