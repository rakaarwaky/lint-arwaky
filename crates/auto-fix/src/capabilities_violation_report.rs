// PURPOSE: ViolationReport — FR-AutoFix-004 capability.
//
// Implements IViolationReportProtocol: runs the linter pipeline, filters
// fixable violations (AES101, AES203, AES304), applies each via its
// dedicated capability, and reports non-fixable violations.
//
// This is the orchestration capability — it owns the execution pipeline
// but delegates per-violation work to the three dedicated capabilities
// (UnusedImportFix, BypassFix, SymbolRename).

use shared_auto_fix::contract_fix_protocol::{
    IBypassFixProtocol, ISymbolRenameProtocol, IUnusedImportFixProtocol, IViolationReportProtocol,
};
use shared_auto_fix::{FIXABLE_CODES, FixOutcome, FixResult, RUST_KEYWORDS, SkipReason};
use shared_common::taxonomy_common_error::ErrorMessage;
use shared_common::taxonomy_lint_result_vo::LintResult;
use shared_common::taxonomy_message_vo::LintMessage;
use shared_common::taxonomy_path_vo::FilePath;
use shared_common::{AdapterName, Count, DescriptionVO, ErrorCode};
use shared_filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use shared_filesystem::taxonomy_filesystem_vo::FileEntry;
use shared_quality_rules::CodeAnalysisRequest;
use shared_quality_rules::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use std::path::Path;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ViolationReport {
    linter: Arc<dyn ICodeAnalysisAggregate>,
    unused_import_fix: Arc<dyn IUnusedImportFixProtocol>,
    bypass_fix: Arc<dyn IBypassFixProtocol>,
    symbol_rename: Arc<dyn ISymbolRenameProtocol>,
    /// Filesystem IO seam — builds the linter's file list from the requested
    /// path so fix scope is confined to it (#934).
    io: Arc<dyn IFileSystemIOProtocol>,
}

// ─── Block 2: Protocol Trait Implementation ──────────────

impl IViolationReportProtocol for ViolationReport {
    fn report_violations(&self, path: &FilePath, dry_run: bool) -> FixResult {
        // #934: confine fix scope to the requested path — build the linter's
        // file list from `path` instead of an empty list (which made the
        // linter fall back to project-wide discovery and ignored the path).
        let entries = self.build_entries(path);
        let analysis = self
            .linter
            .execute(CodeAnalysisRequest::run_analysis(&entries))
            .into_violations();
        let results = &analysis;

        let naming_violations: Vec<_> = results
            .iter()
            .filter(|r| r.code == ErrorCode::raw("AES101"))
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
        let mut failed_outcomes: Vec<LintMessage> = Vec::new();
        let mut events: Vec<shared_auto_fix::FixApplied> = Vec::new();

        for violation in &naming_violations {
            // #937: extract the actually flagged symbol — the token after
            // "Symbol" when present (e.g. "Symbol MyName does not match
            // snake_case" → MyName, not snake_case), else the first
            // CamelCase/UPPER_CASE token. The old heuristic grabbed the first
            // underscore-bearing token, so "snake_case" shadowed "MyName".
            let msg = violation.message.value();
            let old_name = Self::extract_flagged_symbol(msg);
            if let Some(old_name) = old_name {
                // Keyword conflict detection
                if RUST_KEYWORDS.contains(&old_name.as_str()) {
                    total_fixable -= 1;
                    continue;
                }

                let parts: Vec<&str> = old_name.split('_').collect();
                let new_name = if parts.len() >= 3 {
                    old_name.clone()
                } else {
                    format!("renamed_{}", old_name)
                };

                if old_name != new_name {
                    let outcome = self.symbol_rename.rename_symbol_dry(
                        path.value(),
                        &old_name,
                        &new_name,
                        dry_run,
                    );
                    match &outcome {
                        FixOutcome::Applied { changes } => {
                            fixed_count += *changes;
                            events.push(self.emit_fix_event_impl(
                                &violation.file,
                                "AES101",
                                *changes,
                            ));
                        }
                        FixOutcome::Failed(reason) => {
                            total_fixable -= 1;
                            failed_outcomes.push(LintMessage::new(format!(
                                "  {} | {} | failed: {} | {}:{}",
                                violation.code,
                                violation.message,
                                reason,
                                violation.file,
                                violation.line
                            )));
                        }
                        FixOutcome::Skipped(_) => {
                            total_fixable -= 1;
                            manual_skipped.push(LintMessage::new(format!(
                                "  {} | {} | skipped: {} | {}:{}",
                                violation.code,
                                violation.message,
                                outcome,
                                violation.file,
                                violation.line
                            )));
                        }
                    }
                } else {
                    // #938: extracted new name equals old name (parts.len() >= 3
                    // keeps the name unchanged) — record it as skipped so the
                    // violation stays in the report instead of vanishing.
                    total_fixable -= 1;
                    manual_skipped.push(LintMessage::new(format!(
                        "  {} | {} | skipped: AlreadyValid | {}:{}",
                        violation.code, violation.message, violation.file, violation.line
                    )));
                }
            } else {
                total_fixable -= 1;
            }
        }

        for violation in &bypass_violations {
            let line = violation.line.clone();
            let outcome = self
                .bypass_fix
                .fix_bypass_dry(violation.file.value(), line, dry_run);
            match &outcome {
                FixOutcome::Applied { changes } => {
                    fixed_count += changes;
                    events.push(self.emit_fix_event_impl(&violation.file, "AES304", *changes));
                }
                FixOutcome::Skipped(SkipReason::UnsafeRemoval)
                | FixOutcome::Skipped(SkipReason::AlreadyHasContext) => {
                    total_fixable -= 1;
                    manual_skipped.push(LintMessage::new(format!(
                        "  {} | {} | {}:{}",
                        violation.code, violation.message, violation.file, violation.line
                    )));
                }
                FixOutcome::Failed(reason) => {
                    total_fixable -= 1;
                    failed_outcomes.push(LintMessage::new(format!(
                        "  {} | {} | failed: {} | {}:{}",
                        violation.code, violation.message, reason, violation.file, violation.line
                    )));
                }
                _ => {
                    total_fixable -= 1;
                }
            }
        }

        for violation in &unused_import_violations {
            let line = violation.line.clone();
            let outcome =
                self.unused_import_fix
                    .fix_unused_import_dry(violation.file.value(), line, dry_run);
            match &outcome {
                FixOutcome::Applied { changes } => {
                    fixed_count += *changes;
                    events.push(self.emit_fix_event_impl(&violation.file, "AES203", *changes));
                }
                FixOutcome::Failed(reason) => {
                    total_fixable -= 1;
                    failed_outcomes.push(LintMessage::new(format!(
                        "  {} | {} | failed: {} | {}:{}",
                        violation.code, violation.message, reason, violation.file, violation.line
                    )));
                }
                FixOutcome::Skipped(_) => {
                    total_fixable -= 1;
                }
            }
        }

        let mut manual_steps = self.report_non_fixable(results);
        manual_steps.extend(manual_skipped);
        manual_steps.extend(failed_outcomes.iter().cloned());

        let failed_count = failed_outcomes.len();
        let remaining = if !dry_run && fixed_count > 0 {
            let after_results = self
                .linter
                .execute(CodeAnalysisRequest::run_analysis(&entries))
                .into_violations();
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

        // Exit-code contract (PRD "Exit Code Contract" → `fix` aggregation rule):
        // any per-item `Failed` outcome is a runtime error, not a policy skip.
        // `Skipped` outcomes are policy skips and never raise the error.
        let error = if failed_count > 0 {
            Some(ErrorMessage::new(format!(
                "{} fix attempt(s) failed with a runtime error",
                failed_count
            )))
        } else if !dry_run && fixed_count == 0 && total_fixable > 0 {
            Some(ErrorMessage::new("All fix attempts failed".to_string()))
        } else {
            None
        };

        FixResult::new(DescriptionVO::new(output), error)
    }

    fn report_non_fixable(&self, violations: &[LintResult]) -> Vec<LintMessage> {
        let mut manual: Vec<LintMessage> = Vec::new();
        for r in violations {
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

impl ViolationReport {
    pub fn new(
        linter: Arc<dyn ICodeAnalysisAggregate>,
        unused_import_fix: Arc<dyn IUnusedImportFixProtocol>,
        bypass_fix: Arc<dyn IBypassFixProtocol>,
        symbol_rename: Arc<dyn ISymbolRenameProtocol>,
        io: Arc<dyn IFileSystemIOProtocol>,
    ) -> Self {
        Self {
            linter,
            unused_import_fix,
            bypass_fix,
            symbol_rename,
            io,
        }
    }

    /// Build the linter's `FileEntry` list from the requested path (#934).
    ///
    /// A single source file contributes one entry; a directory is walked
    /// recursively (skipping `DEFAULT_IGNORED_PATHS` and symlinks) and every
    /// source file under it contributes an entry. The linter then reports
    /// violations only for these files, confining fix scope to the path.
    fn build_entries(&self, path: &FilePath) -> Vec<FileEntry> {
        let p = Path::new(path.value());
        if p.is_file() {
            return self.read_entry(p);
        }
        let mut entries: Vec<FileEntry> = Vec::new();
        self.walk_dir(p, &mut entries);
        entries
    }

    fn walk_dir(&self, dir: &Path, entries: &mut Vec<FileEntry>) {
        let empty = shared_common::PatternList::new(Vec::<String>::new());
        for child in self.io.scan_directory_with_ignored(dir, &empty) {
            let ft = match child.symlink_metadata() {
                Ok(ft) => ft,
                Err(_) => continue,
            };
            if ft.is_symlink() {
                continue;
            }
            if ft.is_dir() {
                let name = child.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if shared_common::DEFAULT_IGNORED_PATHS.contains(&name) {
                    continue;
                }
                self.walk_dir(&child, entries);
            } else if ft.is_file() {
                entries.extend(self.read_entry(&child));
            }
        }
    }

    /// One entry per readable lintable source file; non-source or unreadable
    /// files are skipped (the linter filters by `parse_ok` and extension).
    fn read_entry(&self, p: &Path) -> Vec<FileEntry> {
        let content = match self.io.read_to_string(p) {
            Ok(c) => c.value().to_string(),
            Err(_) => return Vec::new(),
        };
        let extension = p
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_string();
        let Some(language) =
            shared_common::taxonomy_language_vo::Language::from_extension(&extension)
        else {
            return Vec::new();
        };
        vec![FileEntry {
            path: p.to_path_buf(),
            extension,
            language,
            size: content.len() as u64,
            content,
            parse_ok: true,
            parse_metadata: None,
        }]
    }

    /// Publish a FixApplied event for the given violation.
    fn emit_fix_event_impl(
        &self,
        path: &FilePath,
        error_code: &str,
        changes: usize,
    ) -> shared_auto_fix::FixApplied {
        shared_auto_fix::FixApplied::new(
            path.clone(),
            AdapterName::raw("violation-report"),
            ErrorCode::raw(error_code.to_string()),
            Count::new(changes as i64),
        )
    }

    /// Extract the symbol an AES101 message flagged (#937).
    ///
    /// The message's own symbol is the token following "Symbol" when that
    /// keyword is present ("Symbol MyName does not match snake_case" →
    /// `MyName`), else the first CamelCase or UPPER_CASE token. The previous
    /// heuristic grabbed the first underscore-bearing token, so the
    /// convention word `snake_case` shadowed the real symbol and the fix
    /// renamed the wrong identifier.
    fn extract_flagged_symbol(message: &str) -> Option<String> {
        let tokens: Vec<&str> = message.split_whitespace().collect();
        if let Some(pos) = tokens.iter().position(|t| *t == "Symbol")
            && pos + 1 < tokens.len()
        {
            return Some(tokens[pos + 1].to_string());
        }
        tokens
            .iter()
            .find(|t| Self::is_camel_or_upper(t))
            .map(|s| s.to_string())
    }

    /// A candidate flagged symbol: starts with an uppercase letter — a
    /// CamelCase or UPPER_CASE identifier, never a snake_case convention word.
    fn is_camel_or_upper(token: &str) -> bool {
        token.chars().next().is_some_and(|c| c.is_ascii_uppercase())
    }
}
