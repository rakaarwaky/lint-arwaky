// PURPOSE: Shared AES402 helpers for the three contract role auditors.
//
// Every language auditor (Rust / Python / TypeScript) runs the same five
// sub-checks. Only the syntax scanning is language specific, so the pieces
// that are identical across languages — contract-file recognition, the single
// LintResult constructor, the I/O exemption, and the language flags — live
// here and are shared by all three.

use shared::common::FilePath;
use shared::common::taxonomy_language_info_vo::LanguageInfo;
use shared::common::taxonomy_language_vo::Language;
use shared::common::taxonomy_lint_result_vo::LintResult;
use shared::common::taxonomy_severity_vo::Severity;
use shared::filesystem::taxonomy_filesystem_vo::FileEntry;

/// True when `path` names a contract-layer file.
///
/// An auditor calls this first so it never spends scan time on a file the
/// orchestrator would not have routed to it, and so a caller that holds a
/// `FileEntry` from a fixture can assert the file is in scope. A contract
/// seam is `contract_<domain>_protocol` or `contract_<domain>_aggregate`, in
/// Rust, Python, or TypeScript.
pub fn is_contract_file(path: &str) -> bool {
    let normalized = path.replace('\\', "/");
    let Some(file_name) = normalized.rsplit('/').next() else {
        return false;
    };
    if !file_name.starts_with("contract") {
        return false;
    }
    // The stem carries the seam, so the extension is matched separately: a
    // `.rs` protocol and a `.py` aggregate are both in scope.
    let stem = strip_extension(file_name);
    stem.ends_with("_protocol") || stem.ends_with("_aggregate")
}

/// Strip a Rust / Python / TypeScript extension from a file name.
fn strip_extension(file_name: &str) -> &str {
    file_name
        .strip_suffix(".rs")
        .or_else(|| file_name.strip_suffix(".py"))
        .or_else(|| file_name.strip_suffix(".ts"))
        .or_else(|| file_name.strip_suffix(".js"))
        .unwrap_or(file_name)
}

/// True when the path is an I/O contract seam that is exempt from the primitive
/// sub-check.
///
/// A low-level file system, network, or process seam must speak in `&str` and
/// `std::io::Error` — that is the correct abstraction there, not a taxonomy
/// VO. Every other contract file is held to the VO rule.
pub fn is_io_exemption(path_str: &str) -> bool {
    path_str.contains("io_protocol") || path_str.contains("filesystem_io")
}

/// The single place an AES402 contract finding is constructed.
///
/// All three auditors build their messages the same way — a headline, the
/// reason with the offending detail, and a fix — so the three languages report
/// defects in one shape and a reader does not have to learn three formats.
pub fn build_violation(
    path: &str,
    line: u32,
    code: &str,
    severity: Severity,
    why: &str,
    fix: &str,
    detail: &str,
) -> LintResult {
    LintResult::new_arch(
        path,
        line as usize,
        code,
        severity,
        format!("{code} CONTRACT_ROLE: {detail}\nWHY? {why}\nHOW TO FIX? {fix}"),
    )
}

/// Resolve the language flags for a file.
///
/// An auditor calls this to confirm the file is a language it can scan before
/// walking its syntax. Returns `None` when the path is not one this linter
/// parses, so the caller skips instead of scanning nothing useful. The
/// classification is keyed on the extension, mirroring the shared detector, so
/// a seam reaches the same auditor here as it does across the pipeline.
pub fn detect_language(file: &FileEntry) -> Option<LanguageInfo> {
    let path_str = file.path.to_string_lossy();
    // The FilePath validates the path shape; a malformed one cannot be scanned.
    FilePath::new(path_str.to_string()).ok()?;
    let lang = match file
        .path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or_default()
    {
        "rs" => Language::Rust,
        "py" => Language::Python,
        "js" | "jsx" | "mjs" | "cjs" | "ts" | "tsx" | "mts" | "cts" => {
            if file.language == Language::JavaScript {
                Language::JavaScript
            } else {
                Language::TypeScript
            }
        }
        _ => return None,
    };
    Some(LanguageInfo::new(
        lang == Language::Rust,
        lang == Language::Python,
        lang == Language::TypeScript || lang == Language::JavaScript,
        lang,
    ))
}

/// True when `layer` names the contract layer.
///
/// The orchestrator passes the layer it dispatched on, so a caller that hands
/// the wrong layer to a contract auditor is a no-op rather than a report.
pub fn is_contract_layer(layer: &str) -> bool {
    layer == "contract" || layer.starts_with("contract(")
}

/// The rule code every AES402 contract finding carries.
///
/// A single place so the three auditors cannot drift apart on the code they
/// report, and so the callers do not repeat the literal.
pub fn rule_code() -> &'static str {
    "AES402"
}
