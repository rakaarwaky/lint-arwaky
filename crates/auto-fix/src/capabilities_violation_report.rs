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
use shared_quality_rules::CodeAnalysisRequest;
use shared_quality_rules::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ViolationReport {
    linter: Arc<dyn ICodeAnalysisAggregate>,
    unused_import_fix: Arc<dyn IUnusedImportFixProtocol>,
    bypass_fix: Arc<dyn IBypassFixProtocol>,
    symbol_rename: Arc<dyn ISymbolRenameProtocol>,
}

// ─── Block 2: Protocol Trait Implementation ──────────────

impl IViolationReportProtocol for ViolationReport {
    fn report_violations(&self, path: &FilePath, dry_run: bool) -> FixResult {
        let analysis = self
            .linter
            .execute(CodeAnalysisRequest::run_analysis(&[]))
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
        let mut events: Vec<shared_auto_fix::FixApplied> = Vec::new();

        for violation in &naming_violations {
            let msg = violation.message.value();
            if let Some(old_name) = msg
                .split_whitespace()
                .find(|w| w.contains('_') && w.len() > 3)
            {
                // Keyword conflict detection
                if RUST_KEYWORDS.contains(&old_name) {
                    total_fixable -= 1;
                    continue;
                }

                let parts: Vec<&str> = old_name.split('_').collect();
                let new_name = if parts.len() >= 3 {
                    old_name.to_string()
                } else {
                    format!("renamed_{}", old_name)
                };

                if old_name != new_name {
                    let outcome = self.symbol_rename.rename_symbol_dry(
                        path.value(),
                        old_name,
                        &new_name,
                        dry_run,
                    );
                    if outcome.is_applied() {
                        let changes = match &outcome {
                            FixOutcome::Applied { changes } => *changes,
                            _ => 0,
                        };
                        fixed_count += changes;
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

        let remaining = if !dry_run && fixed_count > 0 {
            let after_results = self
                .linter
                .execute(CodeAnalysisRequest::run_analysis(&[]))
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

        let error = if !dry_run && fixed_count == 0 && total_fixable > 0 {
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
    ) -> Self {
        Self {
            linter,
            unused_import_fix,
            bypass_fix,
            symbol_rename,
        }
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
}
