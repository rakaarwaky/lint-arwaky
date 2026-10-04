// PURPOSE: SurfaceRoleChecker — ISurfaceRoleProtocol for AES406: tier-aware
// function-count check plus passive/utility domain logic and method limits.
//
// ALGORITHM:
//   1. `classify_surface_tier` resolves smart / utility / passive from the
//      filename suffix (see utility_surface_role_checker.rs for details).
//   2. Smart surfaces (`_command`/`_controller`/`_page`) are exempt from
//      passive/utility limits.
//   3. Utility surfaces (`_hook`/`_store`/`_action`/`_screen`/`_router`) and
//      passive surfaces (`_component`/`_view`/`_layout`) are checked for
//      excessive methods and domain logic.
//   4. `check_fn_count_limit` enforces a tier-specific function-count ceiling
//      with an AST-to-lexical fallback.

use shared_common::taxonomy_lint_result_vo::LintResult;
use shared_common::taxonomy_severity_vo::Severity;
use shared_filesystem::taxonomy_filesystem_vo::{FileEntry, Language, ParseMetadata};
use shared_role_rules::contract_role_protocol::ISurfaceRoleProtocol;
use shared_role_rules::taxonomy_role_rules_constant::{
    MAX_CONTROL_FLOW, MAX_FN_COUNT_PASSIVE, MAX_FN_COUNT_SMART, MAX_FN_COUNT_UTILITY,
    MAX_PUBLIC_METHODS,
};
use shared_role_rules::taxonomy_role_rules_vo::{SurfaceTier, classify_surface_tier};

// ─── Block 1: Struct Definition ───────────────────────────
pub struct SurfaceRoleChecker {}

// ─── Block 2: Protocol Trait Implementation ───────────────
impl ISurfaceRoleProtocol for SurfaceRoleChecker {
    fn check_smart_surface(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        // Smart surfaces are exempt from the passive/utility method and
        // control-flow limits; the function-count check runs in
        // check_fn_count_limit for all tiers.
        let _ = (file, violations);
    }

    fn check_utility_surface(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        self._check_passive_with_metadata(file, violations);
        self._check_domain_logic(file, violations);
    }

    fn check_passive_surface(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        self._check_passive_with_metadata(file, violations);
        self._check_domain_logic(file, violations);
    }

    fn check_fn_count_limit(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        let tier = classify_surface_tier(stem_of(file));
        let limit = match tier {
            SurfaceTier::Smart => MAX_FN_COUNT_SMART,
            SurfaceTier::Utility => MAX_FN_COUNT_UTILITY,
            SurfaceTier::Passive => MAX_FN_COUNT_PASSIVE,
        };
        let fn_count =
            count_functions(file).unwrap_or_else(|| count_functions_lexical(file, &file.language));
        if fn_count <= limit {
            return;
        }
        let path_str = file.path.to_string_lossy();
        let tier_name = match tier {
            SurfaceTier::Smart => "smart",
            SurfaceTier::Utility => "utility",
            SurfaceTier::Passive => "passive",
        };
        violations.push(LintResult::new_arch_with_name(
            &path_str,
            0,
            "AES406",
            Severity::HIGH,
            format!(
                "AES406 SURFACE_ROLE: {} tier surface has {} functions (max {})\n\
                 WHY: A {}-tier surface with too many functions has too many responsibilities.\n\
                 FIX: Split into smaller surface files, or move logic down to capabilities or an agent.",
                tier_name,
                fn_count,
                limit,
                tier_name,
            ),
            "SURFACE_ROLE",
            format!(
                "A {}-tier surface with too many functions has too many responsibilities.",
                tier_name
            ),
            "Split into smaller surface files, or move logic down to capabilities or an agent.",
        ));
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────
impl Default for SurfaceRoleChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl SurfaceRoleChecker {
    pub fn new() -> Self {
        Self {}
    }

    // ── Passive surface checks using ParseMetadata ──

    fn _check_passive_with_metadata(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        let path_str = file.path.to_string_lossy();
        let fn_count = count_functions(file).unwrap_or(0);
        if fn_count > MAX_PUBLIC_METHODS {
            violations.push(LintResult::new_arch_with_name(
                &path_str,
                0,
                "AES406",
                Severity::HIGH,
                format!(
                    "AES406 SURFACE_ROLE: Surface role boundary violation.\n\
                     WHY: Surface file '{}' has {} functions (max {})\n\
                     FIX: Ensure surface only performs its designated responsibilities.",
                    path_str, fn_count, MAX_PUBLIC_METHODS,
                ),
                "SURFACE_ROLE",
                format!(
                    "Surface file '{}' has {} functions (max {})",
                    path_str, fn_count, MAX_PUBLIC_METHODS
                ),
                "Ensure surface only performs its designated responsibilities.",
            ));
        }
    }

    // ── Domain logic check (control flow count) ──

    fn _check_domain_logic(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        let path_str = file.path.to_string_lossy();
        let control_flow_count = file
            .content
            .lines()
            .filter(|line| {
                let t = line.trim();
                t.starts_with("if ")
                    || t.starts_with("else ")
                    || t.starts_with("for ")
                    || t.starts_with("while ")
                    || t.starts_with("match ")
                    || t.starts_with("switch ")
                    || t.starts_with("try:")
                    || t.starts_with("except")
                    || t.starts_with("catch")
            })
            .count();
        if control_flow_count > MAX_CONTROL_FLOW {
            violations.push(LintResult::new_arch_with_name(
                &path_str,
                0,
                "AES406",
                Severity::HIGH,
                format!(
                    "AES406 SURFACE_ROLE: Complex domain logic detected in a passive/utility surface.\n\
                     WHY: Surface file has {} control flow statements (max {})\n\
                     FIX: Move the complex domain/control logic into capabilities or orchestrator components.",
                    control_flow_count, MAX_CONTROL_FLOW,
                ),
                "SURFACE_ROLE",
                format!(
                    "Surface file has {} control flow statements (max {})",
                    control_flow_count, MAX_CONTROL_FLOW
                ),
                "Move the complex domain/control logic into capabilities or orchestrator components.",
            ));
        }
    }
}

// ─── Counting helpers ─────────────────────────────────────

/// The file stem, or an empty string when the path has no filename.
fn stem_of(file: &FileEntry) -> &str {
    file.path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or_default()
}

/// Count functions from `ParseMetadata` when available; `None` otherwise.
fn count_functions(file: &FileEntry) -> Option<usize> {
    match &file.parse_metadata {
        Some(ParseMetadata::Rust(m)) => Some(m.function_definitions.len()),
        Some(ParseMetadata::Python(m)) => Some(m.function_definitions.len()),
        Some(ParseMetadata::TypeScript(m)) | Some(ParseMetadata::JavaScript(m)) => {
            Some(m.function_definitions.len())
        }
        _ => None,
    }
}

/// Lexical fallback when `ParseMetadata` is absent.
fn count_functions_lexical(file: &FileEntry, lang: &Language) -> usize {
    match lang {
        Language::Rust => file
            .content
            .lines()
            .filter(|l| {
                let t = l.trim_start();
                t.starts_with("fn ")
                    || t.starts_with("pub fn ")
                    || t.starts_with("async fn ")
                    || t.starts_with("pub async fn ")
            })
            .count(),
        Language::Python => file
            .content
            .lines()
            .filter(|l| {
                let t = l.trim_start();
                t.starts_with("def ") || t.starts_with("async def ")
            })
            .count(),
        Language::TypeScript | Language::JavaScript => file
            .content
            .lines()
            .filter(|l| {
                let t = l.trim_start();
                t.starts_with("function ")
                    || t.starts_with("async function ")
                    || t.starts_with("export function ")
                    || (t.starts_with("const ") && t.contains("=>"))
            })
            .count(),
        _ => 0,
    }
}
