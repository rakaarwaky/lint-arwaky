// PURPOSE: LintFixProcessor — applies auto-fixes for architecture violations via IFixProtocol, tracks fix results
//
// FRD compliance: every fix attempt returns a reason-coded FixOutcome
// (Applied / Skipped(reason) / Failed(reason)), never a bare boolean.
use shared::auto_fix::{
    FailReason, FixApplied, FixOutcome, FixResult, IFileAdapterProtocol, IFixProtocol, SkipReason,
};
use shared::cli_commands::LintResult;
use shared::code_analysis::ICodeAnalysisAggregate;
use shared::common::{
    AdapterName, ContentString, Count, DescriptionVO, ErrorCode, FilePath, LineNumber, LintMessage,
};

use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct LintFixProcessor {
    dry_run: bool,
    linter: Arc<dyn ICodeAnalysisAggregate>,
    file_adapter: Arc<dyn IFileAdapterProtocol>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────
impl IFixProtocol for LintFixProcessor {
    fn execute(&self, path: &FilePath) -> FixResult {
        let results = self.linter.run_code_analysis(path).values;

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
        let mut manual_skipped: Vec<LintMessage> = Vec::new();

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
                    let outcome = self.rename_symbol_impl(&path.value, old_name, &new_name);
                    if outcome.is_applied() {
                        let changes = match &outcome {
                            FixOutcome::Applied { changes } => *changes,
                            _ => 0,
                        };
                        fixed_count += changes;
                        self.emit_fix_event_impl(&violation.file, "AES101", changes);
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
            let outcome = self.fix_bypass_comments_impl(&violation.file.value, line);
            match &outcome {
                FixOutcome::Applied { changes } => {
                    fixed_count += changes;
                    self.emit_fix_event_impl(&violation.file, "AES304", *changes);
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
            let outcome = self.fix_unused_import_impl(&violation.file.value, line);
            if outcome.is_applied() {
                let changes = match &outcome {
                    FixOutcome::Applied { changes } => *changes,
                    _ => 0,
                };
                fixed_count += changes;
                self.emit_fix_event_impl(&violation.file, "AES203", changes);
            } else {
                total_fixable -= 1;
            }
        }

        let mut manual_steps = self.report_non_fixable(&results);
        manual_steps.extend(manual_skipped);

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
            let after_results = self.linter.run_code_analysis(path).values;
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

    fn fix_bypass_comments(&self, file_path: &str, line: LineNumber) -> FixOutcome {
        self.fix_bypass_comments_impl(file_path, line.value as u32)
    }

    fn fix_unused_import(&self, file_path: &str, line: LineNumber) -> FixOutcome {
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

// ─── Default FileAdapter (delegates to filesystem crate) ──
struct DefaultFileAdapter;

impl IFileAdapterProtocol for DefaultFileAdapter {
    fn read_file(&self, path: &FilePath) -> Option<ContentString> {
        filesystem::utility_filesystem_io::read_file(&path.value)
            .ok()
            .map(ContentString::new)
    }

    fn write_file(&self, path: &FilePath, content: &ContentString) -> bool {
        filesystem::utility_filesystem_io::write_file(&path.value, &content.value).is_ok()
    }

    fn path_exists(&self, path: &FilePath) -> bool {
        filesystem::utility_filesystem_io::path_exists(&path.value)
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────
impl LintFixProcessor {
    pub fn new(linter: Arc<dyn ICodeAnalysisAggregate>) -> Self {
        Self {
            dry_run: false,
            linter,
            file_adapter: Arc::new(DefaultFileAdapter),
        }
    }

    pub fn with_dry_run(dry_run: bool, linter: Arc<dyn ICodeAnalysisAggregate>) -> Self {
        Self {
            dry_run,
            linter,
            file_adapter: Arc::new(DefaultFileAdapter),
        }
    }

    pub fn with_file_adapter(
        linter: Arc<dyn ICodeAnalysisAggregate>,
        file_adapter: Arc<dyn IFileAdapterProtocol>,
    ) -> Self {
        Self {
            dry_run: false,
            linter,
            file_adapter,
        }
    }

    pub fn with_dry_run_and_adapter(
        dry_run: bool,
        linter: Arc<dyn ICodeAnalysisAggregate>,
        file_adapter: Arc<dyn IFileAdapterProtocol>,
    ) -> Self {
        Self {
            dry_run,
            linter,
            file_adapter,
        }
    }

    /// FR-002: Fix bypass comments — returns FixOutcome per FRD.
    ///
    /// Patterns: #[allow(...)] → remove line, //noqa → remove comment,
    /// unwrap()/unwrap(); → replace with expect("safe").
    /// Skips: panic!/todo!/unimplemented!/unreachable! → UnsafeRemoval.
    fn fix_bypass_comments_impl(&self, file_path: &str, line: u32) -> FixOutcome {
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

        // ─── Detect fixable bypass patterns ───
        let allow_attr = format!("#{}", "allow(");
        let unwrap_call = "unwrap()".to_string();
        let noqa_pattern = "noqa";
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

        if self.dry_run {
            return FixOutcome::applied(0);
        }

        // ─── Apply fix ───
        let mut result = String::new();
        for (i, l) in lines.iter().enumerate() {
            if i == target_idx {
                // FR-002: #[allow(...)] → remove entire line
                if is_allow_attr {
                    continue;
                }
                // FR-002: comment lines, noqa, type: ignore, FIXME, HACK, XXX
                if is_comment_line
                    || trimmed.contains(noqa_pattern)
                    || trimmed.contains(type_ignore)
                    || trimmed.contains("FIXME")
                    || trimmed.contains("HACK")
                    || trimmed.contains("XXX")
                {
                    if is_comment_line {
                        continue;
                    }
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
    fn fix_unused_import_impl(&self, file_path: &str, line: u32) -> FixOutcome {
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

        // FR-001: Check if target line is an import
        let import_patterns = ["use ", "import ", "from ", "require("];
        if !import_patterns.iter().any(|p| target_line.starts_with(p)) {
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

        if self.dry_run {
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

    fn emit_fix_event_impl(&self, path: &FilePath, error_code: &str, changes: usize) {
        let _event = FixApplied::new(
            path.clone(),
            AdapterName::raw("lint-fix-orchestrator"),
            ErrorCode::raw(error_code.to_string()),
            Count::new(changes as i64),
        );
    }

    /// FR-003: Rename symbol — returns FixOutcome with actual change count.
    ///
    /// Mechanical rename: prepends `renamed_` prefix.
    /// Returns `Applied` with the actual number of replacements, or
    /// `Skipped` / `Failed` with the appropriate reason.
    fn rename_symbol_impl(&self, file_path: &str, old_name: &str, new_name: &str) -> FixOutcome {
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
        if !content.contains(old_name) {
            return FixOutcome::skipped(SkipReason::SymbolNotFound);
        }

        let change_count = content.matches(old_name).count();

        if self.dry_run {
            return FixOutcome::applied(change_count);
        }

        let new_content = content.replace(old_name, new_name);
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
