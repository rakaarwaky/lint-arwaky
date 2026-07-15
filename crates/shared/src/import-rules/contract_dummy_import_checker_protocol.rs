// PURPOSE: IDummyImportCheckerProtocol — combined contract for AES204 dummy import checking
//
// Single trait that replaces IArchRuleProtocol + IArchImportProtocol + IDummyAnalyzerPort
// for the DummyImportChecker. Contains ALL function signatures (43 total):
//   - rule_name() from IArchRuleProtocol
//   - check_mandatory_imports() / check_forbidden_imports() from IArchImportProtocol
//   - dummy analysis functions from IDummyAnalyzerPort
//   - AES204 sub-checks 1-6
//   - Pure computation functions (pub(crate)) for dummy/stub detection
//   - Private helpers for brace/indent parsing, symbol extraction, and body detection

use crate::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use crate::cli_commands::taxonomy_severity_vo::Severity;
use crate::common::taxonomy_common_vo::LineNumber;
use crate::common::taxonomy_layer_vo::Identity;
use crate::common::taxonomy_name_vo::SymbolName;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;
use crate::common::taxonomy_message_vo::LintMessage;
use crate::import_rules::taxonomy_language_vo::LanguageVO;
use crate::import_rules::contract_rule_protocol::IAnalyzer;

#[async_trait::async_trait]
pub trait IDummyImportCheckerProtocol: Send + Sync {
    // ═══════════════════════════════════════════════════════════════════════
    // Core protocol methods (13)
    // ═══════════════════════════════════════════════════════════════════════

    /// Rule identity (AES204)
    fn rule_name(&self) -> Identity;

    /// Check that files do not contain dummy imports or dummy functions.
    async fn check_mandatory_imports(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );

    /// No-op for dummy import checker (forbidden imports handled by AES201).
    async fn check_forbidden_imports(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );

    // ═══════════════════════════════════════════════════════════════════════
    // IDummyAnalyzerPort analysis methods (4)
    // ═══════════════════════════════════════════════════════════════════════

    /// Detect ranges of dummy/stub functions (named _use_* or dummy_*).
    fn dummy_function_ranges(
        &self,
        lines: &[&str],
        lang: LanguageVO,
    ) -> Vec<(LineNumber, LineNumber)>;

    /// Extract all imported symbols from source lines (multi-language).
    fn imported_symbols(&self, lines: &[&str], lang: LanguageVO) -> Vec<(SymbolName, LineNumber)>;

    /// Detect trait implementations whose body is empty/stub (dummy impls).
    fn dummy_impl_traits_with_lines(&self, lines: &[&str]) -> Vec<(SymbolName, LineNumber)>;

    /// Check whether a symbol is actually used outside dummy/stub contexts.
    fn is_symbol_used_real(
        &self,
        lines: &[&str],
        symbol: &str,
        dummy_ranges: &[(usize, usize)],
        dummy_impl_traits: &[String],
    ) -> bool;

    // ═══════════════════════════════════════════════════════════════════════
    // AES204 sub-checks (6) — called from check_mandatory_imports
    // ═══════════════════════════════════════════════════════════════════════

    /// Sub-check 1: Detect symbols imported but only used inside dummy functions or stub impls.
    fn check_dummy_imports(
        &self,
        file: &str,
        content: &str,
        violations: &mut Vec<LintResult>,
        analyzer: &dyn IAnalyzer,
        root_dir: &FilePath,
    );

    /// Sub-check 2: Detect dummy functions (named _use_*) that serve only to suppress unused-import warnings.
    fn check_dummy_functions(
        &self,
        file: &str,
        content: &str,
        violations: &mut Vec<LintResult>,
        analyzer: &dyn IAnalyzer,
        root_dir: &FilePath,
    );

    /// Sub-check 3: Detect trait implementations that are stubs (empty body).
    fn check_dummy_impls(
        &self,
        file: &str,
        content: &str,
        violations: &mut Vec<LintResult>,
        analyzer: &dyn IAnalyzer,
        root_dir: &FilePath,
    );

    /// Sub-check 4: Verify taxonomy VO imports are used in real function signatures.
    fn check_taxonomy_intent(
        &self,
        file: &str,
        content: &str,
        violations: &mut Vec<LintResult>,
        analyzer: &dyn IAnalyzer,
        root_dir: &FilePath,
    );

    /// Sub-check 5: Detect aggregate types used only as PhantomData (never called in real code).
    fn check_layer_contract_intent(
        &self,
        file: &str,
        content: &str,
        violations: &mut Vec<LintResult>,
        analyzer: &dyn IAnalyzer,
        root_dir: &FilePath,
    );

    /// Sub-check 6: Detect surface-layer files calling business logic directly.
    fn check_surface_logic(
        &self,
        file: &str,
        content: &str,
        violations: &mut Vec<LintResult>,
    );

    // ═══════════════════════════════════════════════════════════════════════
    // Pure computation functions (4) — used by both DummyImportChecker and
    // ImportAnalyzer within the same crate
    // ═══════════════════════════════════════════════════════════════════════

    /// Detect ranges of dummy functions across all supported languages.
    fn pure_dummy_function_ranges(&self, lines: &[&str], lang: LanguageVO) -> Vec<(LineNumber, LineNumber)>;

    /// Extract all imported symbols from source lines (multi-language).
    fn pure_imported_symbols(&self, lines: &[&str], lang: LanguageVO) -> Vec<(SymbolName, LineNumber)>;

    /// Detect trait implementations whose body is empty/stub (dummy impls).
    fn pure_dummy_impl_traits_with_lines(&self, lines: &[&str]) -> Vec<(SymbolName, LineNumber)>;

    /// Check whether a symbol is actually used outside dummy/stub contexts.
    fn pure_symbol_used_real(&self, 
        lines: &[&str],
        symbol: &str,
        dummy_ranges: &[(usize, usize)],
        dummy_impl_traits: &[String],
    ) -> bool;

    // ═══════════════════════════════════════════════════════════════════════
    // Private helpers — brace/indent parsing (3)
    // ═══════════════════════════════════════════════════════════════════════

    /// Generic range collector with header detection and body extent functions.
    fn pure_collect_ranges<F, G>(&self, 
        lines: &[&str],
        is_header: F,
        body_extent: G,
    ) -> Vec<(LineNumber, LineNumber)>
    where
        Self: Sized,
        F: Fn(&str) -> bool,
        G: Fn(usize, &[&str]) -> usize;

    /// Find the extent of a brace-delimited block starting at `start`.
    fn pure_brace_extent(&self, start: usize, lines: &[&str]) -> usize;

    /// Find the extent of an indent-based block (Python-style) starting at `start`.
    fn pure_indent_extent(&self, start: usize, lines: &[&str]) -> usize;

    // ═══════════════════════════════════════════════════════════════════════
    // Language-specific dummy detection (3)
    // ═══════════════════════════════════════════════════════════════════════

    /// Detect Rust dummy functions (fn _use_* or fn dummy_*).
    fn pure_rust_dummy_function_ranges(&self, lines: &[&str]) -> Vec<(LineNumber, LineNumber)>;

    /// Detect Python dummy functions (def _use_* or def dummy_*).
    fn pure_python_dummy_function_ranges(&self, lines: &[&str]) -> Vec<(LineNumber, LineNumber)>;

    /// Detect JavaScript dummy functions (function _use, function dummy, const _use, const dummy).
    fn pure_js_dummy_function_ranges(&self, lines: &[&str]) -> Vec<(LineNumber, LineNumber)>;

    // ═══════════════════════════════════════════════════════════════════════
    // Language-specific symbol extraction (3)
    // ═══════════════════════════════════════════════════════════════════════

    /// Extract imported symbols from Rust source lines.
    fn pure_rust_imported_symbols(&self, lines: &[&str]) -> Vec<(SymbolName, LineNumber)>;

    /// Parse a single import part and extract the symbol name.
    fn pure_rust_imported_symbol_from_part(&self, part: &str) -> Option<String>;

    /// Extract imported symbols from Python source lines.
    fn pure_python_imported_symbols(&self, lines: &[&str]) -> Vec<(SymbolName, LineNumber)>;

    /// Extract imported symbols from JavaScript source lines.
    fn pure_js_imported_symbols(&self, lines: &[&str]) -> Vec<(SymbolName, LineNumber)>;

    // ═══════════════════════════════════════════════════════════════════════
    // Private helpers — symbol/impl detection (8)
    // ═══════════════════════════════════════════════════════════════════════

    /// Check whether a line number falls within any dummy range.
    fn pure_in_dummy_range(&self, line_no: usize, ranges: &[(usize, usize)]) -> bool;

    /// Extract the trait name from an impl line (e.g., "impl Foo for Bar" → "Foo").
    fn pure_impl_trait_name(&self, line: &str) -> Option<String>;

    /// Find the extent of an impl block and collect its body lines.
    fn pure_impl_block<'a>(&self, lines: &'a [&'a str], start: usize) -> (usize, Vec<&'a str>);

    /// Check whether all methods in a trait impl body are dummy/stub.
    fn pure_trait_impl_is_dummy(&self, lines: &[&str]) -> bool;

    /// Find the extent of a function body and collect its lines.
    fn pure_function_body<'a>(&self, lines: &'a [&'a str], start: usize) -> (usize, Vec<&'a str>);

    /// Check whether a function body is dummy/stub (empty, todo!, unimplemented!, etc.).
    fn pure_function_body_is_dummy(&self, lines: &[&str]) -> bool;

    /// Check if a string starts with short markers like todo!, unimplemented!, panic!, unreachable!.
    fn pure_is_short_marker(&self, inner: &str) -> bool;

    // ═══════════════════════════════════════════════════════════════════════
    // Utility functions (2)
    // ═══════════════════════════════════════════════════════════════════════

    /// Returns the inner FilePath if result is Ok, otherwise returns FilePath::default().
    fn pure_filepath_or_default(&self, result: Result<FilePath, impl std::fmt::Debug>) -> FilePath where Self: Sized;

    /// Returns the &str slice from an OsStr option, falling back to "".
    fn pure_os_str_to_str_opt(&self, opt: Option<&std::ffi::OsStr>) -> &str;

    // ═══════════════════════════════════════════════════════════════════════
    // Private helper — Python class parsing (1)
    // ═══════════════════════════════════════════════════════════════════════

    /// Check if a Python class line inherits from the given aggregate type.
    fn pure_python_class_inherits(&self, line: &str, agg_type: &str) -> bool;
}
