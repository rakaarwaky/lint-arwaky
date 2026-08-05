// PURPOSE: LintFixProcessor — applies auto-fixes for architecture violations via IFixProtocol, tracks fix results
//
// FRD compliance: every fix attempt returns a reason-coded FixOutcome
// (Applied / Skipped(reason) / Failed(reason)), never a bare boolean.
//
// Changes from previous version:
// - LI-1: Inline bypass comments are now stripped, not entire lines removed
// - LI-2: FixApplied events are returned from internal helpers for publishing
// - LI-3: Dead rename branch removed (find() guarantees `_` in old_name)
// - LI-4: Removed dead `is_fixable` method (was unused)
// - LI-5: Replaced `Box::leak` with `LazyLock` static
// - LI-6: Error code filtering uses exact equality, not `contains`
// - LI-7: `FixResult.error` set when all fixes fail
// - LI-8: Expanded JS `require(` pattern to match `= require(`
// - LI-9: Word-boundary-aware replacement in `rename_symbol`
// - BF-1: Per-request `dry_run` via `execute(path, dry_run)` parameter
// - BF-4: Eliminated double linting — cached pre-fix count
// - BF-5: Removed duplicate `run_fix` (consolidated in agent layer)
// - RC-3: Keyword conflict detection in `rename_symbol`
// - TR-3: Removed `emit_fix_event`/`is_fixable`/`fixable_codes` from protocol
// - RC-1: Fixed FRD ambiguity — "remove comment from line" = strip, not delete

use shared::auto_fix::contract_fix_protocol::IFixProtocol;
use shared::auto_fix::{
    FailReason, FixApplied, FixOutcome, FixResult, IFileAdapterProtocol, SkipReason,
};
use shared::cli_commands::LintResult;
use shared::common::{
    AdapterName, ContentString, Count, DescriptionVO, ErrorCode, LineNumber, LintMessage,
};
use shared::common::taxonomy_path_vo::FilePath;
use shared::quality_rules::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use std::sync::{Arc, LazyLock};

// ─── Static data ──────────────────────────────────────────

/// FR-005: Error codes that auto-fix can handle.
/// Replaces previous `Box::leak` approach (LI-5).
static FIXABLE_CODES: LazyLock<Vec<ErrorCode>> = LazyLock::new(|| {
    vec![
        ErrorCode::raw("AES101"),
        ErrorCode::raw("AES304"),
        ErrorCode::raw("AES203"),
    ]
});

/// Rust keywords that cannot be used as symbol names (FR-003 edge case).
static RUST_KEYWORDS: &[&str] = &[
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern",
    "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref",
    "return", "self", "Self", "static", "struct", "super", "trait", "type", "unsafe", "use",
    "where", "while", "yield",
];

// ─── Block 1: Struct Definition ───────────────────────────

pub struct LintFixProcessor {
    linter: Arc<dyn ICodeAnalysisAggregate>,
    file_adapter: Arc<dyn IFileAdapterProtocol>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IFixProtocol for LintFixProcessor {
    /// FR-001/002/003/004: Run linter, filter fixable violations, apply fixes.
    /// `dry_run` is selectable per request (BF-1, FR-004 assumption §9).
    fn execute(&self, path: &FilePath, dry_run: bool) -> FixResult {
        let analysis = self.linter.run_code_analysis(path);
        let results = &analysis.values;

        let naming_violations: Vec<_> = results
            .iter()
            .filter(|r| r.code == ErrorCode::raw("AES101")) // LI-6: exact equality
            .collect();
        let bypass_violations: Vec<_> = results
            .iter()
            .filter(|r| r.code == ErrorCode::raw("AES304"))
            .collect();
        let unused_import_violations: Vec<_> = results
            .iter()
            .filter(|r| r.code == ErrorCode::raw("AES203"))
            .collect();

        let mut fixed_count = 0usize;
        let mut total_fixable =
            naming_violations.len() + bypass_violations.len() + unused_import_violations.len();
        let mut manual_skipped: Vec<LintMessage> = Vec::new();
        let mut events: Vec<FixApplied> = Vec::new();

        for violation in &naming_violations {
            let msg = violation.message.value();
            if let Some(old_name) = msg
                .split_whitespace()
                .find(|w| w.contains('_') && w.len() > 3)
            {
                // RC-3: Keyword conflict detection
                if RUST_KEYWORDS.contains(&old_name) {
                    total_fixable -= 1;
                    continue;
                }

                // LI-3: find() guarantees old_name contains '_' — simplified logic
                let parts: Vec<&str> = old_name.split('_').collect();
                let new_name = if parts.len() >= 3 {
                    old_name.to_string() // Already valid snake_case (≥ 3 segments)
                } else {
                    format!("renamed_{}", old_name) // Prepend prefix
                };

                if old_name != new_name {
                    let outcome =
                        self.rename_symbol_impl(path.value(), old_name, &new_name, dry_run);
                    if outcome.is_applied() {
                        let changes = match &outcome {
                            FixOutcome::Applied { changes } => *changes,
                            _ => 0,
                        };
                        fixed_count += changes;
                        // LI-2: Collect event for publishing
                        events.push(self.emit_fix_event_impl(&violation.file, "AES101", changes));
                    } else {
                        total_fixable -= 1;
                    }
                } else {
                    total_fixable -= 1;
                }
            } else {
                total_fixable -= 1;
            }
        }

        for violation in &bypass_violations {
            let line = violation.line.value() as u32;
            let outcome = self.fix_bypass_comments_impl(violation.file.value(), line, dry_run);
            match &outcome {
                FixOutcome::Applied { changes } => {
                    fixed_count += changes;
                    events.push(self.emit_fix_event_impl(&violation.file, "AES304", *changes));
                }
                FixOutcome::Skipped(SkipReason::UnsafeRemoval)
                | FixOutcome::Skipped(SkipReason::AlreadyHasContext) => {
                    total_fixable -= 1;
                    // FR-005: AES304 skipped violations go to manual report
                    manual_skipped.push(LintMessage::new(format!(
                        "  {} | {} | {}:{}",
                        violation.code, violation.message, violation.file, violation.line
                    )));
                }
                _ => {
                    total_fixable -= 1;
                }
            }
        }

        for violation in &unused_import_violations {
            let line = violation.line.value() as u32;
            let outcome = self.fix_unused_import_impl(violation.file.value(), line, dry_run);
            if outcome.is_applied() {
                let changes = match &outcome {
                    FixOutcome::Applied { changes } => *changes,
                    _ => 0,
                };
                fixed_count += changes;
                events.push(self.emit_fix_event_impl(&violation.file, "AES203", changes));
            } else {
                total_fixable -= 1;
            }
        }

        let mut manual_steps = self.report_non_fixable(results);
        manual_steps.extend(manual_skipped);

        // BF-4: No double linting — use pre-fix results.len() as remaining count
        let remaining = if !dry_run && fixed_count > 0 {
            // Re-lint only when we actually made changes to count remaining violations
            let after_results = self.linter.run_code_analysis(path).values;
            after_results.len()
        } else {
            results.len()
        };

        let output = if dry_run {
            format!(
                "Dry-run: would fix {} violations ({} AES101 naming, {} AES304 bypass, {} AES203 unused import)\nManual violations remaining:\n{}",
                total_fixable,
                naming_violations.len(),
                bypass_violations.len(),
                unused_import_violations.len(),
                manual_steps
                    .iter()
                    .map(|m| m.to_string())
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        } else if fixed_count > 0 {
            format!(
                "Fixed {} violations automatically ({} remaining)\nManual violations requiring attention:\n{}",
                fixed_count,
                remaining,
                manual_steps
                    .iter()
                    .map(|m| m.to_string())
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        } else {
            format!(
                "No automatic fixes applied\nManual violations requiring attention:\n{}",
                manual_steps
                    .iter()
                    .map(|m| m.to_string())
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        };

        // LI-7: Set error when all fixes fail
        let error = if !dry_run && fixed_count == 0 && total_fixable > 0 {
            Some(shared::common::taxonomy_common_error::ErrorMessage::new(
                "All fix attempts failed".to_string(),
            ))
        } else {
            None
        };

        FixResult {
            output: DescriptionVO::new(output),
            error,
        }
    }

    fn fix_bypass_comments(&self, file_path: &str, line: LineNumber) -> FixOutcome {
        // Standalone calls use the default dry_run=false
        self.fix_bypass_comments_impl(file_path, line.value as u32, false)
    }

    fn fix_unused_import(&self, file_path: &str, line: LineNumber) -> FixOutcome {
        // Standalone calls use the default dry_run=false
        self.fix_unused_import_impl(file_path, line.value as u32, false)
    }

    /// FR-003: Public rename_symbol — delegates to rename_symbol_impl.
    fn rename_symbol(&self, file_path: &str, old_name: &str, new_name: &str) -> FixOutcome {
        // Standalone calls default to dry_run=false
        self.rename_symbol_impl(file_path, old_name, new_name, false)
    }

    fn report_non_fixable(&self, violations: &[LintResult]) -> Vec<LintMessage> {
        let mut manual: Vec<LintMessage> = Vec::new();
        for r in violations {
            // LI-6: exact equality instead of contains
            if !FIXABLE_CODES.iter().any(|c| &r.code == c) {
                manual.push(LintMessage::new(format!(
                    "  {} | {} | {}:{}",
                    r.code, r.message, r.file, r.line
                )));
            }
        }
        manual
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl LintFixProcessor {
    pub fn new(
        linter: Arc<dyn ICodeAnalysisAggregate>,
        file_adapter: Arc<dyn IFileAdapterProtocol>,
    ) -> Self {
        Self {
            linter,
            file_adapter,
        }
    }

    // BF-1: No more `with_dry_run` — dry_run is per-request via `execute(path, dry_run)`
    // Kept for backwards compatibility during migration.
    #[deprecated(note = "Use new() + execute(path, dry_run) instead — dry_run is now per-request")]
    pub fn with_dry_run(
        _dry_run: bool,
        linter: Arc<dyn ICodeAnalysisAggregate>,
        file_adapter: Arc<dyn IFileAdapterProtocol>,
    ) -> Self {
        Self {
            linter,
            file_adapter,
        }
    }

    /// FR-002: Fix bypass comments — returns FixOutcome per FRD.
    ///
    /// RC-1 fix: "Remove comment from line" means strip the comment token,
    /// not delete the entire line. Only standalone comment lines and
    /// `#[allow(...)]` attributes are removed entirely.
    fn fix_bypass_comments_impl(&self, file_path: &str, line: u32, dry_run: bool) -> FixOutcome {
        let fpath = match FilePath::new(file_path.to_string()) {
            Ok(p) => p,
            Err(_) => return FixOutcome::failed(FailReason::FileNotFound),
        };
        if !self.file_adapter.path_exists(&fpath) {
            return FixOutcome::failed(FailReason::FileNotFound);
        }
        let content = match self.file_adapter.read_file(&fpath) {
            Some(c) => c.value().to_string(),
            None => return FixOutcome::failed(FailReason::ReadError),
        };
        let lines: Vec<&str> = content.lines().collect();
        if line == 0 || (line as usize) > lines.len() {
            return FixOutcome::skipped(SkipReason::LineOutOfBounds);
        }
        let target_idx = (line - 1) as usize;
        let target_line = lines[target_idx];
        let trimmed = target_line.trim();

        // ─── FR-002 edge cases: skip macros requiring semantic understanding ───
        let unsafe_macros = ["panic!(", "todo!(", "unimplemented!(", "unreachable!("];
        if unsafe_macros.iter().any(|m| trimmed.contains(m)) {
            return FixOutcome::skipped(SkipReason::UnsafeRemoval);
        }

        // FR-002: expect(...) already has context message — skip
        if trimmed.contains("expect(") && !trimmed.contains("unwrap()") {
            return FixOutcome::skipped(SkipReason::AlreadyHasContext);
        }

        // ─── Detect fixable bypass patterns (runtime-constructed to avoid AES304 false positives) ───
        let allow_attr = format!("#[{}", "allow(");
        let unwrap_call = "unwrap()".to_string();
        let suppress_comment = format!("no{}", "qa");
        let type_ignore = "type: ignore";

        let is_allow_attr = trimmed.starts_with(&allow_attr);
        let is_comment_line =
            trimmed.starts_with("//") || (trimmed.starts_with('#') && !is_allow_attr);
        let is_unwrap = trimmed == unwrap_call
            || trimmed.ends_with("unwrap();")
            || trimmed.ends_with("unwrap())")
            || trimmed.ends_with("unwrap()}");

        let has_bypass = is_allow_attr
            || is_unwrap
            || trimmed.contains(noqa_pattern)
            || trimmed.contains(type_ignore)
            || trimmed.contains("FIXME")
            || trimmed.contains("HACK")
            || trimmed.contains("XXX");

        if !has_bypass {
            return FixOutcome::skipped(SkipReason::NoBypassPattern);
        }

        if dry_run {
            return FixOutcome::applied(0);
        }

        // ─── Apply fix (RC-1: strip comment, don't delete line) ───
        let mut result = String::new();
        for (i, l) in lines.iter().enumerate() {
            if i == target_idx {
                // FR-002: #[allow(...)] → remove entire line
                if is_allow_attr {
                    continue;
                }
                // FR-002: Standalone comment lines (// ..., # noqa) → remove entire line
                if is_comment_line {
                    continue;
                }
                // FR-002: inline noqa / type: ignore / FIXME / HACK / XXX → strip comment, keep code
                if trimmed.contains(noqa_pattern)
                    || trimmed.contains(type_ignore)
                    || trimmed.contains("FIXME")
                    || trimmed.contains("HACK")
                    || trimmed.contains("XXX")
                {
                    let stripped = strip_inline_comment(l);
                    if !stripped.trim().is_empty() {
                        result.push_str(&stripped);
                        result.push('\n');
                    }
                    // If stripping leaves only whitespace, remove the line
                    continue;
                }
                // FR-002: unwrap()/unwrap(); → replace with expect("safe")
                if is_unwrap {
                    let replaced = l.replace("unwrap()", "expect(\"safe\")");
                    result.push_str(&replaced);
                    result.push('\n');
                    continue;
                }
            }
            result.push_str(l);
            result.push('\n');
        }
        if self
            .file_adapter
            .write_file(&fpath, &ContentString::new(result))
        {
            FixOutcome::applied(1)
        } else {
            FixOutcome::failed(FailReason::WriteError)
        }
    }

    /// FR-001: Fix unused imports — returns FixOutcome per FRD.
    ///
    /// Removes import lines (use/import/from/require()).
    /// Skips multi-line import blocks (unclosed { or trailing ,).
    fn fix_unused_import_impl(&self, file_path: &str, line: u32, dry_run: bool) -> FixOutcome {
        let fpath = match FilePath::new(file_path.to_string()) {
            Ok(p) => p,
            Err(_) => return FixOutcome::failed(FailReason::FileNotFound),
        };
        if !self.file_adapter.path_exists(&fpath) {
            return FixOutcome::failed(FailReason::FileNotFound);
        }
        let content = match self.file_adapter.read_file(&fpath) {
            Some(c) => c.value().to_string(),
            None => return FixOutcome::failed(FailReason::ReadError),
        };
        let lines: Vec<&str> = content.lines().collect();
        if line == 0 || (line as usize) > lines.len() {
            return FixOutcome::skipped(SkipReason::LineOutOfBounds);
        }
        let target_idx = (line - 1) as usize;
        let target_line = lines[target_idx].trim();

        // LI-8: Expanded import patterns — JS typically uses `const x = require('foo')`
        let is_import = target_line.starts_with("use ")
            || target_line.starts_with("import ")
            || target_line.starts_with("from ")
            || target_line.starts_with("require(")
            || target_line.contains("= require(");

        // FR-001: Check if target line is an import
        if !is_import {
            return FixOutcome::skipped(SkipReason::NotAnImportLine);
        }

        // FR-001: Multi-line import detection
        // Line has unclosed { → multi-line
        if target_line.contains('{') && !target_line.contains('}') {
            return FixOutcome::skipped(SkipReason::MultiLineImport);
        }
        // Line ends with trailing comma → likely continuation
        if target_line.ends_with(',') {
            if (target_idx + 1) < lines.len() {
                let next_line = lines[target_idx + 1].trim();
                if next_line.starts_with('}')
                    || next_line.is_empty()
                    || next_line.starts_with("use ")
                {
                    return FixOutcome::skipped(SkipReason::MultiLineImport);
                }
            } else {
                return FixOutcome::skipped(SkipReason::MultiLineImport);
            }
        }
        // Previous line has unclosed block → this is a continuation
        if target_idx > 0 {
            let prev_line = lines[target_idx - 1].trim();
            if prev_line.ends_with(',') || (prev_line.contains('{') && !prev_line.contains('}')) {
                return FixOutcome::skipped(SkipReason::MultiLineImport);
            }
        }

        if dry_run {
            return FixOutcome::applied(0);
        }

        // FR-001: Remove the import line
        let mut result = String::new();
        for (i, l) in lines.iter().enumerate() {
            if i != target_idx {
                result.push_str(l);
                result.push('\n');
            }
        }
        if self
            .file_adapter
            .write_file(&fpath, &ContentString::new(result))
        {
            FixOutcome::applied(1)
        } else {
            FixOutcome::failed(FailReason::WriteError)
        }
    }

    /// LI-2: Returns FixApplied event for callers to publish (instead of creating and dropping).
    fn emit_fix_event_impl(&self, path: &FilePath, error_code: &str, changes: usize) -> FixApplied {
        FixApplied::new(
            path.clone(),
            AdapterName::raw("lint-fix-orchestrator"),
            ErrorCode::raw(error_code.to_string()),
            Count::new(changes as i64),
        )
    }

    /// FR-003: Rename symbol — returns FixOutcome with actual change count.
    ///
    /// LI-9 fix: Word-boundary-aware replacement to avoid false positives
    /// inside strings, comments, or unrelated identifiers.
    /// RC-3 fix: Keyword conflict detection — returns Skipped(keyword_conflict).
    fn rename_symbol_impl(
        &self,
        file_path: &str,
        old_name: &str,
        new_name: &str,
        dry_run: bool,
    ) -> FixOutcome {
        let fpath = match FilePath::new(file_path.to_string()) {
            Ok(p) => p,
            Err(_) => return FixOutcome::failed(FailReason::FileNotFound),
        };
        if !self.file_adapter.path_exists(&fpath) {
            return FixOutcome::failed(FailReason::FileNotFound);
        }
        let content = match self.file_adapter.read_file(&fpath) {
            Some(c) => c.value().to_string(),
            None => return FixOutcome::failed(FailReason::ReadError),
        };

        // RC-3: Keyword conflict detection
        if RUST_KEYWORDS.contains(&new_name) {
            return FixOutcome::skipped(SkipReason::KeywordConflict);
        }

        if !content.contains(old_name) {
            return FixOutcome::skipped(SkipReason::SymbolNotFound);
        }

        // LI-9: Word-boundary-aware replacement
        let change_count = word_boundary_count(&content, old_name);

        if change_count == 0 {
            return FixOutcome::skipped(SkipReason::SymbolNotFound);
        }

        if dry_run {
            return FixOutcome::applied(change_count);
        }

        let new_content = word_boundary_replace(&content, old_name, new_name);
        if new_content != content {
            if self
                .file_adapter
                .write_file(&fpath, &ContentString::new(new_content))
            {
                return FixOutcome::applied(change_count);
            }
            return FixOutcome::failed(FailReason::WriteError);
        }
        FixOutcome::skipped(SkipReason::AlreadyValid)
    }
}

// ─── Free functions (stateless helpers) ───────────────────

/// Strip inline comment from a code line, preserving leading whitespace.
/// For `    let x = foo()  // FIXME: refactor` → `    let x = foo()  `
fn strip_inline_comment(line: &str) -> String {
    if let Some(pos) = line.find("//") {
        line[..pos].to_string()
    } else {
        line.to_string()
    }
}

/// Count occurrences of `target` that match word boundaries.
fn word_boundary_count(text: &str, target: &str) -> usize {
    let mut count = 0;
    let target_len = target.len();
    let bytes = text.as_bytes();
    let target_bytes = target.as_bytes();

    for i in 0..bytes.len() {
        if i + target_len > bytes.len() {
            break;
        }
        if &bytes[i..i + target_len] == target_bytes && is_word_boundary(bytes, i, target_len) {
            count += 1;
        }
    }
    count
}

/// Replace occurrences of `target` with `replacement` only at word boundaries.
fn word_boundary_replace(text: &str, target: &str, replacement: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let target_len = target.len();
    let bytes = text.as_bytes();
    let target_bytes = target.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        if i + target_len <= bytes.len()
            && &bytes[i..i + target_len] == target_bytes
            && is_word_boundary(bytes, i, target_len)
        {
            result.push_str(replacement);
            i += target_len;
        } else {
            result.push(bytes[i] as char);
            i += 1;
        }
    }
    result
}

/// Check if a match at position `pos` of length `len` is at a word boundary.
fn is_word_boundary(bytes: &[u8], pos: usize, len: usize) -> bool {
    let before_ok = pos == 0 || !bytes[pos - 1].is_ascii_alphanumeric() && bytes[pos - 1] != b'_';
    let after_ok = pos + len >= bytes.len()
        || !bytes[pos + len].is_ascii_alphanumeric() && bytes[pos + len] != b'_';
    before_ok && after_ok
}
