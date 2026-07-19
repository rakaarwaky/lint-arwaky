// PURPOSE: Surface role utility functions (AES406)
//
// These pure functions support surface role auditing. They are extracted here
// so they can be unit-tested directly without needing to construct a
// SurfaceRoleChecker or feed it a full SourceContentVO.

use crate::taxonomy_adapter_name_vo::AdapterName;
use crate::taxonomy_common_vo::{ColumnNumber, LineNumber};
use crate::taxonomy_error_vo::ErrorCode;
use crate::taxonomy_lint_vo::LocationList;
use crate::taxonomy_message_vo::LintMessage;
use crate::taxonomy_path_vo::FilePath;

/// Generate AES406 passive violation detail message.
pub fn aes406_passive_violation_details(file: &str, details: &str) -> String {
    format!(
        "AES406 SURFACE_ROLE: Surface file '{}' contains active domain logic:\n{}\nWHY? Surfaces must be passive I/O boundaries.\nFIX: Move logic to capabilities/agent layers.",
        file, details
    )
}

/// Check if the file is a surface file by filename prefix `surface_` or `surfaces_` or directory `surfaces/`.
pub fn is_in_surfaces(f: &FilePath) -> bool {
    let path_str = f.to_string();
    let basename = match path_str.rsplit('/').next() {
        Some(s) => s,
        None => &path_str,
    };
    let stem = match basename.rfind('.') {
        Some(pos) => &basename[..pos],
        None => basename,
    };
    if stem.starts_with("surface_") || stem.starts_with("surfaces_") {
        return true;
    }
    if let Some(parent) = path_str.rsplit('/').nth(1) {
        if parent == "surfaces" || parent == "surface" || parent == "cli_commands" {
            return true;
        }
    }
    false
}

/// Check if the file is a barrel/init file.
pub fn is_init(f: &FilePath) -> bool {
    let path_str = f.to_string();
    path_str.ends_with("__init__.py")
        || path_str.ends_with("mod.rs")
        || path_str.ends_with("index.ts")
        || path_str.ends_with("index.js")
}

/// Create an AdapterName from a string (helper for _report_aes0306).
pub fn make_adapter(name: &str) -> Option<AdapterName> {
    AdapterName::new(name).ok()
}
