/// mcp_server_validator — Input validation for MCP tools.
use crate::taxonomy::{BooleanVO, ContentString, ErrorMessage, FieldName, MAX_PATH_LENGTH, MAX_PATH_DEPTH, MAX_BATCH_SIZE, MAX_STRING_LENGTH};
use std::path::Path;

#[derive(Debug, Clone)]
pub enum ValidationErrorType {
    PathTraversal,
    PathTooLong,
    PathDepthExceeded,
    TooManyFiles,
    StringTooLong,
    UnsafeInput,
}

#[derive(Debug, Clone)]
pub struct ValidationError {
    pub error_type: ValidationErrorType,
    pub message: ErrorMessage,
    pub field: FieldName,
}

impl ValidationError {
    pub fn new(error_type: ValidationErrorType, message: &str, field: &str) -> Self {
        Self { error_type, message: ErrorMessage::new(message.to_string()), field: FieldName::new(field.to_string()) }
    }
}

#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub is_valid: BooleanVO,
    pub errors: Vec<ValidationError>,
}

impl ValidationResult {
    pub fn valid() -> Self {
        Self { is_valid: BooleanVO::new(true), errors: Vec::new() }
    }

    pub fn invalid(errors: Vec<ValidationError>) -> Self {
        Self { is_valid: BooleanVO::new(false), errors }
    }
}

pub fn validate_path(user_input: &str, allowed_root: Option<&str>, must_exist: bool) -> ValidationResult {
    let mut errors = Vec::new();
    if user_input.len() > MAX_PATH_LENGTH {
        errors.push(ValidationError::new(ValidationErrorType::PathTooLong, &format!("Path length {} exceeds max {}", user_input.len(), MAX_PATH_LENGTH), "path"));
    }
    let depth = Path::new(user_input).components().count();
    if depth > MAX_PATH_DEPTH {
        errors.push(ValidationError::new(ValidationErrorType::PathDepthExceeded, &format!("Path depth {} exceeds max {}", depth, MAX_PATH_DEPTH), "path"));
    }
    if let Some(root) = allowed_root {
        let resolved = Path::new(root).join(user_input);
        if !resolved.exists() && must_exist {
            errors.push(ValidationError::new(ValidationErrorType::UnsafeInput, &format!("Path does not exist: {}", user_input), "path"));
        }
    }
    if errors.is_empty() { ValidationResult::valid() } else { ValidationResult::invalid(errors) }
}

pub fn validate_string_input(value: &str, max_length: Option<usize>, field_name: &str) -> ValidationResult {
    let max = max_length.unwrap_or(MAX_STRING_LENGTH);
    let mut errors = Vec::new();
    if value.len() > max {
        errors.push(ValidationError::new(ValidationErrorType::StringTooLong, &format!("Input length {} exceeds max {}", value.len(), max), field_name));
    }
    if value.contains('\x00') || value.contains('\0') {
        errors.push(ValidationError::new(ValidationErrorType::UnsafeInput, "Input contains null bytes", field_name));
    }
    if errors.is_empty() { ValidationResult::valid() } else { ValidationResult::invalid(errors) }
}
