/// mcp_server_validator — Input validation for MCP tools.
pub use crate::taxonomy::ValidationError;
use crate::taxonomy::{
    BooleanVO, Constraint, ErrorMessage, FieldName, MAX_PATH_DEPTH, MAX_PATH_LENGTH,
    MAX_STRING_LENGTH,
};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub is_valid: BooleanVO,
    pub errors: Vec<ValidationError>,
}

impl ValidationResult {
    pub fn valid() -> Self {
        Self {
            is_valid: BooleanVO::new(true),
            errors: Vec::new(),
        }
    }

    pub fn invalid(errors: Vec<ValidationError>) -> Self {
        Self {
            is_valid: BooleanVO::new(false),
            errors,
        }
    }
}

pub fn validate_path(
    user_input: &str,
    allowed_root: Option<&str>,
    must_exist: bool,
) -> ValidationResult {
    let mut errors = Vec::new();
    if user_input.len() > MAX_PATH_LENGTH {
        errors.push(ValidationError {
            field_name: FieldName::new("path"),
            message: ErrorMessage::new(format!(
                "Path length {} exceeds max {}",
                user_input.len(),
                MAX_PATH_LENGTH
            )),
            constraint: Some(Constraint::new("PathTooLong")),
            value: None,
        });
    }
    let depth = Path::new(user_input).components().count();
    if depth > MAX_PATH_DEPTH {
        errors.push(ValidationError {
            field_name: FieldName::new("path"),
            message: ErrorMessage::new(format!(
                "Path depth {} exceeds max {}",
                depth, MAX_PATH_DEPTH
            )),
            constraint: Some(Constraint::new("PathDepthExceeded")),
            value: None,
        });
    }
    if let Some(root) = allowed_root {
        let resolved = Path::new(root).join(user_input);
        if !resolved.exists() && must_exist {
            errors.push(ValidationError {
                field_name: FieldName::new("path"),
                message: ErrorMessage::new(format!("Path does not exist: {}", user_input)),
                constraint: Some(Constraint::new("UnsafeInput")),
                value: None,
            });
        }
    }
    if errors.is_empty() {
        ValidationResult::valid()
    } else {
        ValidationResult::invalid(errors)
    }
}

pub fn validate_string_input(
    value: &str,
    max_length: Option<usize>,
    field_name: &str,
) -> ValidationResult {
    let max = max_length.unwrap_or(MAX_STRING_LENGTH);
    let mut errors = Vec::new();
    if value.len() > max {
        errors.push(ValidationError {
            field_name: FieldName::new(field_name),
            message: ErrorMessage::new(format!("Input length {} exceeds max {}", value.len(), max)),
            constraint: Some(Constraint::new("StringTooLong")),
            value: None,
        });
    }
    if value.contains('\x00') || value.contains('\0') {
        errors.push(ValidationError {
            field_name: FieldName::new(field_name),
            message: ErrorMessage::new("Input contains null bytes"),
            constraint: Some(Constraint::new("UnsafeInput")),
            value: None,
        });
    }
    if errors.is_empty() {
        ValidationResult::valid()
    } else {
        ValidationResult::invalid(errors)
    }
}
