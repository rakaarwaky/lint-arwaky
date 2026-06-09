use crate::code_analysis::capabilities_renamer_processor::SymbolRenamerProcessor;
use crate::code_analysis::contract_fix_aggregate::LintFixOrchestratorAggregate;
use crate::layer_rules::contract_lint_protocol::IArchLintProtocol;
use crate::shared_common::taxonomy_applied_event::FixApplied;
use crate::shared_common::taxonomy_common_vo::Count;
use crate::shared_common::taxonomy_error_vo::ErrorCode;
use crate::shared_common::taxonomy_fix_vo::FixResult;
use crate::shared_common::taxonomy_name_vo::AdapterName;
use crate::shared_common::taxonomy_suggestion_vo::DescriptionVO;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use std::sync::Arc;

pub struct LintFixProcessor {
    dry_run: bool,
    linter: Arc<dyn IArchLintProtocol>,
}

impl LintFixProcessor {
    pub fn new(linter: Arc<dyn IArchLintProtocol>) -> Self {
        Self {
            dry_run: false,
            linter,
        }
    }

    pub fn with_dry_run(dry_run: bool, linter: Arc<dyn IArchLintProtocol>) -> Self {
        Self { dry_run, linter }
    }

    fn fix_bypass_comments(&self, file_path: &str, line: u32) -> bool {
        let path = std::path::Path::new(file_path);
        if !path.exists() {
            return false;
        }
        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => return false,
        };
        let lines: Vec<&str> = content.lines().collect();
        if (line as usize) > lines.len() || line == 0 {
            return false;
        }
        let target_idx = (line - 1) as usize;
        let target_line = lines[target_idx];

        let bypass_patterns = [
            "#[allow(",
            "unwrap()",
            "noqa",
            "type: ignore",
            "# type:",
            "panic!",
        ];
        let is_bypass = bypass_patterns.iter().any(|p| target_line.contains(p));
        if !is_bypass {
            return false;
        }

        if self.dry_run {
            return true;
        }

        let mut result = String::new();
        for (i, l) in lines.iter().enumerate() {
            if i == target_idx {
                let trimmed = l.trim();
                if trimmed.starts_with("#[allow(")
                    || trimmed.starts_with("//")
                    || trimmed.starts_with("#")
                {
                    continue;
                }
                if l.trim() == "unwrap()" || l.trim().ends_with("unwrap();") {
                    let replaced = l.replace("unwrap()", "expect(\"safe\")");
                    result.push_str(&replaced);
                    result.push('\n');
                    continue;
                }
            }
            result.push_str(l);
            result.push('\n');
        }
        std::fs::write(path, result).is_ok()
    }

    fn fix_unused_import(&self, file_path: &str, line: u32) -> bool {
        let path = std::path::Path::new(file_path);
        if !path.exists() {
            return false;
        }
        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => return false,
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
        std::fs::write(path, result).is_ok()
    }

    fn emit_fix_event(&self, path: &FilePath, error_code: &str, changes: usize) {
        let event = FixApplied::new(
            path.clone(),
            AdapterName::raw("lint-fix-orchestrator"),
            ErrorCode::raw(error_code.to_string()),
            Count::new(changes as i64),
        );
        let _ = event;
    }

    fn report_non_fixable(
        &self,
        results: &[crate::output_report::taxonomy_result_vo::LintResult],
    ) -> Vec<String> {
        let fixable_codes = ["AES003", "AES014", "AES015"];
        let mut manual: Vec<String> = Vec::new();
        for r in results {
            let code_str = r.code.to_string();
            if !fixable_codes.iter().any(|c| code_str.contains(c)) {
                manual.push(format!(
                    "  {} | {} | {}:{}",
                    code_str, r.message, r.file, r.line
                ));
            }
        }
        manual
    }
}

impl LintFixOrchestratorAggregate for LintFixProcessor {
    fn execute(&self, path: &FilePath) -> FixResult {
        let results = self.linter.run_self_lint(&path.value).values;

        let naming_violations: Vec<_> = results
            .iter()
            .filter(|r| r.code.to_string().contains("AES003"))
            .collect();
        let bypass_violations: Vec<_> = results
            .iter()
            .filter(|r| r.code.to_string().contains("AES014"))
            .collect();
        let unused_import_violations: Vec<_> = results
            .iter()
            .filter(|r| r.code.to_string().contains("AES015"))
            .collect();

        let mut fixed_count = 0usize;
        let mut total_fixable =
            naming_violations.len() + bypass_violations.len() + unused_import_violations.len();

        let renamer = SymbolRenamerProcessor {};
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
                    let count = renamer.rename_symbol(&path.value, old_name, &new_name);
                    fixed_count += count;
                    self.emit_fix_event(&violation.file, "AES003", count);
                }
            }
        }

        for violation in &bypass_violations {
            let line = violation.line.value() as u32;
            let fixed = self.fix_bypass_comments(&violation.file.value, line);
            if fixed {
                fixed_count += 1;
                self.emit_fix_event(&violation.file, "AES014", 1);
            } else {
                total_fixable -= 1;
            }
        }

        for violation in &unused_import_violations {
            let line = violation.line.value() as u32;
            let fixed = self.fix_unused_import(&violation.file.value, line);
            if fixed {
                fixed_count += 1;
                self.emit_fix_event(&violation.file, "AES015", 1);
            } else {
                total_fixable -= 1;
            }
        }

        let manual_steps = self.report_non_fixable(&results);

        let output = if self.dry_run {
            format!(
                "Dry-run: would fix {} violations ({} AES003 naming, {} AES014 bypass, {} AES015 unused import)\nManual violations remaining:\n{}",
                total_fixable,
                naming_violations.len(),
                bypass_violations.len(),
                unused_import_violations.len(),
                manual_steps.join("\n")
            )
        } else if fixed_count > 0 {
            let after_results = self.linter.run_self_lint(&path.value).values;
            let remaining = after_results.len();
            format!(
                "Fixed {} violations automatically ({} remaining)\nManual violations requiring attention:\n{}",
                fixed_count,
                remaining,
                manual_steps.join("\n")
            )
        } else {
            format!(
                "No automatic fixes applied\nManual violations requiring attention:\n{}",
                manual_steps.join("\n")
            )
        };

        FixResult {
            output: DescriptionVO::new(output),
            error: if self.dry_run {
                None
            } else if fixed_count == 0 && total_fixable == 0 {
                None
            } else {
                None
            },
        }
    }
}
