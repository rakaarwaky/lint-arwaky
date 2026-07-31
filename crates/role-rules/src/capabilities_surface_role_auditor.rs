use shared::cli_commands::LintResult;

use shared::common::Severity;

use shared::common::utility_language_detector::detect_language_info_from_source;
use shared::common::{LintMessage, SourceContentVO};
use shared::role_rules::AesRoleViolation;
use shared::role_rules::ISurfaceRoleChecker;

// PURPOSE: SurfaceRoleChecker — ISurfaceRoleChecker for AES406: smart/utility/passive surface role checks
//
// ALGORITHM:
//   1. check_fn_count_limit — Counts `fn ` occurrences. If > 15, flags SurfaceRoleViolation.
//   2. check_smart_surface / check_utility_surface / check_passive_surface — no-ops,
//      required by ISurfaceRoleChecker trait but intentionally empty.

// ─── Block 1: Struct Definition ───────────────────────────

pub struct SurfaceRoleChecker {}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl ISurfaceRoleChecker for SurfaceRoleChecker {
    fn check_smart_surface(
        &self,
        _source: &SourceContentVO,
        _violations: &mut Vec<LintResult>,
    ) {
    }
    fn check_utility_surface(
        &self,
        _source: &SourceContentVO,
        _violations: &mut Vec<LintResult>,
    ) {
    }
    fn check_passive_surface(
        &self,
        _source: &SourceContentVO,
        _violations: &mut Vec<LintResult>,
    ) {
    }
    fn check_fn_count_limit(
        &self,
        source: &SourceContentVO,
        violations: &mut Vec<LintResult>,
    ) {
        let content = source.content.value();
        let file = source.file_path.value();
        let li = detect_language_info_from_source(source);
        let fn_keyword = if li.is_py {
            "def "
        } else if li.is_js {
            "function "
        } else {
            "fn "
        };
        let mut count = 0;
        for line in content.lines() {
            let trimmed = line.trim();
            if !trimmed.starts_with("//")
                && !trimmed.starts_with('#')
                && trimmed.contains(fn_keyword)
            {
                count += 1;
                if count > 15 {
                    violations.push(LintResult::new_arch(
                        file,
                        0,
                        "AES406",
                        Severity::HIGH,
                        AesRoleViolation::SurfaceRoleViolation {
                            reason: Some(LintMessage::new(format!(
                                "File {} has too many function declarations (exceeds 15): found {}",
                                file, count
                            ))),
                        },
                    ));
                    return;
                }
            }
        }
    }
}

// ─── Block 3: Constructors ────────────────────────────────

impl Default for SurfaceRoleChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl SurfaceRoleChecker {
    pub fn new() -> Self {
        Self {}
    }
}
