use crate::taxonomy::{Cause, ErrorCode, ErrorMessage};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NamingError {
    pub message: ErrorMessage,
    #[serde(default)]
    pub error_code: Option<ErrorCode>,
    #[serde(default)]
    pub cause: Option<Cause>,
}

impl NamingError {
    pub fn new(message: ErrorMessage) -> Self {
        Self { message, error_code: None, cause: None }
    }
}

impl std::fmt::Display for NamingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.error_code {
            Some(code) => write!(f, "Naming Error [{}]: {}", code, self.message),
            None => write!(f, "Naming Error: {}", self.message),
        }
    }
}
