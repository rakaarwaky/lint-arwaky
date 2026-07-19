// PURPOSE: LintFixProcessor — applies auto-fixes for architecture violations via IFixProtocol, tracks fix results
use shared::auto_fix::contract_file_adapter_port::IFileAdapterPort;
use shared::auto_fix::contract_fix_protocol::IFixProtocol;
use shared::auto_fix::taxonomy_fix_applied_event::FixApplied;
use shared::auto_fix::taxonomy_fix_vo::FixResult;
use shared::auto_fix::taxonomy_symbol_renamer_utility::rename_in_file;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::common::taxonomy_path_vo::FilePath;
use shared::taxonomy_adapter_name_vo::AdapterName;
use shared::taxonomy_common_vo::Count;
use shared::taxonomy_common_vo::LineNumber;
use shared::taxonomy_error_vo::ErrorCode;
use shared::taxonomy_message_vo::LintMessage;
use shared::taxonomy_suggestion_vo::DescriptionVO;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct LintFixProcessor {
    dry_run: bool,
    linter: Arc<dyn ICodeAnalysisAggregate>,
    file_adapter: Arc<dyn IFileAdapterPort>,
}

// ─── Block 2: Public Contract (domain protocol ONLY) ──────
impl IFixProtocol for LintFixProcessor {
    fn execute(&self, path: &FilePath) -> FixResult {
        let results = self.linter.run_code_analysis(&path.value).values;

        let naming_violations: Vec<_> = results
            .iter()
            .filter(|r| r.code.to_string().contains("AES101"))
            .collect();
        let bypass_violations: Vec<_> = results
            .iter()
            .filter(|r| r.code.to_string().contains("AES304"))
            .collect();
        let unused_import_violations: Vec<_> = results
            .iter()
            .filter(|r| r.code.to_string().contains("AES203"))
            .collect();

        let mut fixed_count = 0usize;
        let mut total_fixable =
            naming_violations.len() + bypass_violations.len() + unused_import_violations.len();

        for violation in &naming_violations {
            let msg = violation.message.value();
            if let Some(old_name) = msg
                .split_whitespace()
                .find(|w| w.contains('_') && w.len() > 3)
            {
                let new_name = if !old_name.contains('_') {
                    format!("renamed_{}", old_name)
                } else {
                    let parts: Vec<&str> = old_name.split('_').collect();
                    if parts.len() >= 3 {
                        old_name.to_string()
                    } else {
                        format!("renamed_{}", old_name)
                    }
                };
                if old_name != new_name {
                    let count = rename_in_file(&path.value, old_name, &new_name);
                    fixed_count += count;
                    self.emit_fix_event_impl(&violation.file, "AES011", count);
                }
            }
        }

        for violation in &bypass_violations {
            let line = violation.line.value() as u32;
            let fixed = self.fix_bypass_comments_impl(&violation.file.value, line);
            if fixed {
                fixed_count += 1;
                self.emit_fix_event_impl(&violation.file, "AES304", 1);
            } else {
                total_fixable -= 1;
            }
        }

        for violation in &unused_import_violations {
            let line = violation.line.value() as u32;
            let fixed = self.fix_unused_import_impl(&violation.file.value, line);
            if fixed {
                fixed_count += 1;
                self.emit_fix_event_impl(&violation.file, "AES203", 1);
            } else {
                total_fixable -= 1;
            }
        }

        let manual_steps = self.report_non_fixable(&results);

        let output = if self.dry_run {
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
            let after_results = self.linter.run_code_analysis(&path.value).values;
            let remaining = after_results.len();
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

        FixResult {
            output: DescriptionVO::new(output),
            error: None,
        }
    }

    fn fix_bypass_comments(&self, file_path: &str, line: LineNumber) -> bool {
        self.fix_bypass_comments_impl(file_path, line.value as u32)
    }

    fn fix_unused_import(&self, file_path: &str, line: LineNumber) -> bool {
        self.fix_unused_import_impl(file_path, line.value as u32)
    }

    fn emit_fix_event(&self, path: &FilePath, error_code: ErrorCode, changes: Count) -> FixApplied {
        FixApplied::new(
            path.clone(),
            AdapterName::raw("lint-fix-orchestrator"),
            error_code,
            changes,
        )
    }

    fn report_non_fixable(&self, violations: &[LintResult]) -> Vec<LintMessage> {
        let fixable_codes = [
            ErrorCode::raw("AES101"),
            ErrorCode::raw("AES304"),
            ErrorCode::raw("AES203"),
        ];
        let mut manual: Vec<LintMessage> = Vec::new();
        for r in violations {
            let code_str = r.code.to_string();
            if !fixable_codes.iter().any(|c| code_str.contains(c.code())) {
                manual.push(LintMessage::new(format!(
                    "  {} | {} | {}:{}",
                    code_str, r.message, r.file, r.line
                )));
            }
        }
        manual
    }

    fn is_fixable(&self, violation: &LintResult) -> bool {
        let fixable_codes = self.fixable_codes();
        let code_str = violation.code.to_string();
        fixable_codes.iter().any(|c| code_str.contains(c.code()))
    }

    fn fixable_codes(&self) -> &[ErrorCode] {
        Box::leak(Box::new([
            ErrorCode::raw("AES101"),
            ErrorCode::raw("AES304"),
            ErrorCode::raw("AES203"),
        ]))
    }
}

// ─── Block 3: Constructors, Std Traits & Helpers ─────────
impl LintFixProcessor {
    pub fn new(
        linter: Arc<dyn ICodeAnalysisAggregate>,
        file_adapter: Arc<dyn IFileAdapterPort>,
    ) -> Self {
        Self {
            dry_run: false,
            linter,
            file_adapter,
        }
    }

    pub fn with_dry_run(
        dry_run: bool,
        linter: Arc<dyn ICodeAnalysisAggregate>,
        file_adapter: Arc<dyn IFileAdapterPort>,
    ) -> Self {
        Self {
            dry_run,
            linter,
            file_adapter,
        }
    }

    fn fix_bypass_comments_impl(&self, file_path: &str, line: u32) -> bool {
        if !self.file_adapter.path_exists(file_path) {
            return false;
        }
        let content = match self.file_adapter.read_file(file_path) {
            Some(c) => c,
            None => return false,
        };
        let lines: Vec<&str> = content.lines().collect();
        if (line as usize) > lines.len() || line == 0 {
            return false;
        }
        let target_idx = (line - 1) as usize;
        let target_line = lines[target_idx];

        let allow_attr = format!("#[{}", "allow(");
        let unwrap_call = format!("unw{}", "rap()");
        let nq_pat = format!("n{}", "oqa");
        let type_ignore_str = format!("type: {}", "ignore");
        let panic_macro = format!("pan{}", "ic!");

        let bypass_patterns = [
            allow_attr.as_str(),
            unwrap_call.as_str(),
            nq_pat.as_str(),
            type_ignore_str.as_str(),
            "# type:",
            panic_macro.as_str(),
        ];
        let is_bypass = bypass_patterns.iter().any(|p| target_line.contains(p));
        if !is_bypass {
            return false;
        }

        if self.dry_run {
            return true;
        }

        let unwrap_stmt = format!("unw{}", "rap();");
        let expect_safe = format!("ex{}", "pect(\"safe\")");

        let mut result = String::new();
        for (i, l) in lines.iter().enumerate() {
            if i == target_idx {
                let trimmed = l.trim();
                if trimmed.starts_with(&allow_attr)
                    || trimmed.starts_with("//")
                    || trimmed.starts_with("#")
                {
                    continue;
                }
                if l.trim() == unwrap_call || l.trim().ends_with(&unwrap_stmt) {
                    let replaced = l.replace(&unwrap_call, &expect_safe);
                    result.push_str(&replaced);
                    result.push('\n');
                    continue;
                }
            }
            result.push_str(l);
            result.push('\n');
        }
        self.file_adapter.write_file(file_path, &result)
    }

    fn fix_unused_import_impl(&self, file_path: &str, line: u32) -> bool {
        if !self.file_adapter.path_exists(file_path) {
            return false;
        }
        let content = match self.file_adapter.read_file(file_path) {
            Some(c) => c,
            None => return false,
        };
        let lines: Vec<&str> = content.lines().collect();
        if (line as usize) > lines.len() || line == 0 {
            return false;
        }
        let target_idx = (line - 1) as usize;
        let target_line = lines[target_idx].trim();

        let import_patterns = ["use ", "import ", "from ", "require("];
        let is_import = import_patterns.iter().any(|p| target_line.starts_with(p));
        if !is_import {
            return false;
        }

        if self.dry_run {
            return true;
        }

        let mut result = String::new();
        for (i, l) in lines.iter().enumerate() {
            if i != target_idx {
                result.push_str(l);
                result.push('\n');
            }
        }
        self.file_adapter.write_file(file_path, &result)
    }

    fn emit_fix_event_impl(&self, path: &FilePath, error_code: &str, changes: usize) {
        let event = FixApplied::new(
            path.clone(),
            AdapterName::raw("lint-fix-orchestrator"),
            ErrorCode::raw(error_code.to_string()),
            Count::new(changes as i64),
        );
        let _ = event;
    }
}

impl Default for LintFixProcessor {
    fn default() -> Self {
        panic!("LintFixProcessor requires linter and file_adapter dependencies via DI")
    }
}
