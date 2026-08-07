// PURPOSE: ViolationFormatter — centralized message formatting for AesCodeAnalysisViolation
// All violation messages are produced here (VO empty container pattern).
// Checkers/analyzers return AesCodeAnalysisViolation VOs; this function owns the full message text.

use shared::quality_rules::AesCodeAnalysisViolation;

/// Format the full violation message for an `AesCodeAnalysisViolation`.
///
/// Each variant produces a complete message including the AES rule code,
/// a human-readable category, a WHY? explanation, and a FIX suggestion.
pub fn format_code_analysis_violation(v: &AesCodeAnalysisViolation) -> String {
    match v {
        AesCodeAnalysisViolation::FileTooLarge { reason } => {
            format!(
                "AES301 FILE_TOO_LARGE: File exceeds maximum line count.\nWHY? {}\nFIX: Split the file into smaller, focused modules.",
                reason.as_ref().map(|r| r.value()).unwrap_or_default()
            )
        }
        AesCodeAnalysisViolation::FileTooShort { reason } => {
            format!(
                "AES302 FILE_TOO_SHORT: File is below minimum line count.\nWHY? {}\nFIX: Add meaningful implementation or merge into a related module.",
                reason.as_ref().map(|r| r.value()).unwrap_or_default()
            )
        }
        AesCodeAnalysisViolation::MandatoryClassDefinition { reason } => {
            format!(
                "AES303 MISSING_DEFINITION: File must declare at least one primary symbol.\nWHY? {}\nFIX: Add a struct, enum, trait, class, or interface declaration.",
                reason.as_ref().map(|r| r.value()).unwrap_or_default()
            )
        }
        AesCodeAnalysisViolation::BypassComment { reason } => {
            format!(
                "AES304 BYPASS_COMMENT: Forbidden bypass comment or annotation detected.\nWHY? {}\nFIX: Remove the bypass comment and resolve the issue properly.",
                reason.as_ref().map(|r| r.value()).unwrap_or_default()
            )
        }
        AesCodeAnalysisViolation::UnwrapExpect { reason } => {
            format!(
                "AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.\nWHY? {}\nFIX: Replace the unwrap/expect call with structured error handling.",
                reason.as_ref().map(|r| r.value()).unwrap_or_default()
            )
        }
        AesCodeAnalysisViolation::Panic { reason } => {
            format!(
                "AES304 PANIC: Forbidden panic call detected.\nWHY? {}\nFIX: Return a Result or handle the failure case gracefully without panicking.",
                reason.as_ref().map(|r| r.value()).unwrap_or_default()
            )
        }
        AesCodeAnalysisViolation::Todo { reason } => {
            format!(
                "AES304 TODO: Forbidden todo!() call detected.\nWHY? {}\nFIX: Implement the function body with real logic.",
                reason.as_ref().map(|r| r.value()).unwrap_or_default()
            )
        }
        AesCodeAnalysisViolation::Unimplemented { reason } => {
            format!(
                "AES304 UNIMPLEMENTED: Forbidden unimplemented!() call detected.\nWHY? {}\nFIX: Either implement the missing logic or return a Result::Err.",
                reason.as_ref().map(|r| r.value()).unwrap_or_default()
            )
        }
        AesCodeAnalysisViolation::DeadInheritance { reason } => {
            format!(
                "AES303 DEAD_INHERITANCE: Empty or stub definition detected.\nWHY? {}\nFIX: Add implementation or remove the empty declaration.",
                reason.as_ref().map(|r| r.value()).unwrap_or_default()
            )
        }
        AesCodeAnalysisViolation::CodeDuplication { reason } => {
            format!(
                "AES305 CODE_DUPLICATION: Duplicate code block detected.\nWHY? {}\nFIX: Extract the duplicated logic into a shared function.",
                reason.as_ref().map(|r| r.value()).unwrap_or_default()
            )
        }
    }
}
