// PURPOSE: LintMessage, ComplianceStatus, DisplayContent, Suggestion VOs
// Merged from: taxonomy_message_vo, taxonomy_compliance_vo,
// taxonomy_suggestion_vo, taxonomy_display_content_vo
use crate::string_value_object;
use serde::{Deserialize, Serialize};

string_value_object!(LintMessage);

/// Boolean compliance flag. Written manually because `bool` is not supported
/// by the `string_value_object!` macro (`i64 as bool` is not a valid Rust cast).
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct ComplianceStatus {
    pub value: bool,
}

impl ComplianceStatus {
    pub fn new(value: bool) -> Self {
        Self { value }
    }
    pub fn value(&self) -> bool {
        self.value
    }
}

impl std::fmt::Display for ComplianceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<bool> for ComplianceStatus {
    fn from(v: bool) -> Self {
        Self { value: v }
    }
}

// DescriptionVO and Suggestion follow the standard String-wrapper VO pattern;
// the macro emits the new/value/Display/From/Hash/PartialEq/Deserialize impls.
string_value_object!(DescriptionVO);
string_value_object!(Suggestion);

/// DisplayContent — value object for formatted display output
/// (previews, human-readable sizes, etc.).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct DisplayContent {
    pub value: String,
}

impl DisplayContent {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl From<String> for DisplayContent {
    fn from(value: String) -> Self {
        Self { value }
    }
}

impl std::fmt::Display for DisplayContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
