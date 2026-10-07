// PURPOSE: LintMessage — VO for violation messages
use crate::string_value_object;

string_value_object!(LintMessage);

// Re-export ComplianceStatus so existing imports via taxonomy_message_vo still resolve.
pub use super::taxonomy_compliance_vo::ComplianceStatus;
