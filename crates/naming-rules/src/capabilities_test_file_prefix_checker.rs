// PURPOSE: TestFilePrefixChecker — AES103 test/bench file-prefix discipline
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use shared_common::taxonomy_layer_vo::LayerMapVO;
use shared_common::taxonomy_lint_vo::{LintResult, LintResultList};
use shared_common::taxonomy_path_vo::FilePath;
use shared_common::taxonomy_path_vo::FilePathList;
use shared_common::taxonomy_severity_vo::Severity;
use shared_config_system::taxonomy_config_system_vo::ArchitectureConfig;
use shared_naming_rules::TestSuiteSlot;
use shared_naming_rules::contract_test_file_prefix_protocol::ITestFilePrefixProtocol;
use shared_naming_rules::taxonomy_naming_rules_constant::RULE_CODE_TEST_FILE_PREFIX;
use shared_naming_rules::utility_naming_checker::{
    basename_of, get_stem, rule_exception_set, string_filename_result,
};
use shared_naming_rules::utility_test_prefix::{
    has_legal_prefix, matched_prefix, nested_under, prefix_list, prefixes_for, slot_of,
};

// ─── Block 1: Struct Definition ────────────────────────────

/// Stateless AES103 test-file prefix checker.
///
/// Where AES101 reads a stem's shape and AES102 reads a layer suffix, AES103
/// reads the test-suite prefix of files that sit in `tests/` or `benches/`. It
/// answers two questions: is the file flat (no subdirectory under either
/// directory), and does its stem start with a legal test-type prefix. No
/// internal state.
pub struct TestFilePrefixChecker {}

// ─── Block 2: Protocol Trait Implementation ────────────────

impl ITestFilePrefixProtocol for TestFilePrefixChecker {
    /// FR-NamingRules-003 — AES103: check every file inside `tests/` and
    /// `benches/` against the flat test-type prefix vocabulary.
    fn check_test_file_prefixes(
        &self,
        config: &ArchitectureConfig,
        _layer_map: &LayerMapVO,
        files: &FilePathList,
        _root_dir: &FilePath,
        results: &mut LintResultList,
    ) {
        let exceptions = rule_exception_set(config, RULE_CODE_TEST_FILE_PREFIX);

        let violations: Vec<LintResult> = files
            .values
            .par_iter()
            .filter_map(|f| {
                let f_str = f.to_string();
                let filename = basename_of(&f_str);
                // Rule-level exceptions are matched on the bare file name, the
                // same shape AES101/AES102 accept.
                if exceptions.iter().any(|v| v == filename) {
                    return None;
                }
                self.check_test_file_prefix_internal(&f_str, filename)
            })
            .collect();

        results.values.extend(violations);
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ────────────

impl Default for TestFilePrefixChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl TestFilePrefixChecker {
    pub fn new() -> Self {
        Self {}
    }

    /// AES103 — a file in `tests/` or `benches/` sits flat and starts with a
    /// legal test-type prefix.
    ///
    /// Three findings, in the order they are checked:
    ///   1. the file is nested below `tests/` or `benches/` (the prefix IS the
    ///      virtual folder, so a subdirectory defeats discovery and CI filters)
    ///   2. the file's stem starts with no legal prefix for its directory
    ///   3. the file's stem starts with a prefix belonging to the *other*
    ///      directory (a bench file in `tests/`, or a test file in `benches/`)
    pub fn check_test_file_prefix_internal(
        &self,
        file: &str,
        filename: &str,
    ) -> Option<LintResult> {
        // A nested file is reported by the nesting rule alone: naming its stem
        // against the flat vocabulary would report the same defect twice.
        if let Some((directory, depth)) = nested_under(file) {
            return Some(self.nested_result(file, filename, directory, depth));
        }

        let slot = slot_of(file);
        if slot == TestSuiteSlot::Outside {
            return None;
        }

        // A barrel names no test of its own — `mod.rs` re-exports the support
        // code beside it. Prefix discipline is about the files a test runner
        // discovers, and a barrel is not one. Skipped for the same reason
        // AES101 skips barrels.
        if is_barrel(filename) {
            return None;
        }

        let stem = get_stem(filename)?;
        if has_legal_prefix(stem, slot) {
            return None;
        }
        Some(self.prefix_result(file, filename, stem, slot))
    }

    /// The subdirectory finding. Names the directory and how deep the file sits
    /// so the reader knows which level to flatten.
    fn nested_result(
        &self,
        file: &str,
        filename: &str,
        directory: &'static str,
        depth: usize,
    ) -> LintResult {
        string_filename_result(
            file,
            RULE_CODE_TEST_FILE_PREFIX,
            format!(
                "Test file '{filename}' sits {depth} level(s) below '{directory}/'. \
                 The file-name prefix IS the virtual folder: '{directory}/' must stay flat. \
                 Move the file up to '{directory}/' and let its prefix carry the type — \
                 e.g. 'unit_<subject>.rs' instead of '{directory}/<sub>/{filename}'."
            ),
            Severity::HIGH,
            "TEST_FILE_PREFIX",
            format!(
                "Test file '{filename}' sits {depth} level(s) below '{directory}/'. \
                 The file-name prefix IS the virtual folder: '{directory}/' must stay flat."
            ),
            format!(
                "Move the file up to '{directory}/' and let its prefix carry the type — \
                 e.g. 'unit_<subject>.rs' instead of '{directory}/<sub>/{filename}'."
            ),
        )
    }

    /// The prefix finding. Distinguishes "no legal prefix at all" from "a prefix
    /// from the wrong directory", because the fix differs: the first needs a
    /// rename, the second needs a move.
    fn prefix_result(
        &self,
        file: &str,
        filename: &str,
        stem: &str,
        slot: TestSuiteSlot,
    ) -> LintResult {
        let directory = slot.directory_label();
        let legal = prefix_list(prefixes_for(slot));

        // A prefix that is legal *somewhere* but not here: a test type inside
        // `benches/`, or a bench inside `tests/`.
        let foreign = [TestSuiteSlot::Tests, TestSuiteSlot::Benches]
            .into_iter()
            .filter(|other| *other != slot)
            .find_map(|other| matched_prefix(stem, other).map(|p| (p, other.directory_label())));

        let message = match foreign {
            Some((prefix, home)) => format!(
                "Test file '{filename}' starts with '{prefix}', which belongs in '{home}/', \
                 but this file sits in '{directory}/'. \
                 The prefix is the virtual folder: each test type lives in its own directory. \
                 Move the file to '{home}/' or rename it with a '{directory}/' prefix — \
                 legal '{directory}/' prefixes: {legal}."
            ),
            None => format!(
                "Test file '{filename}' does not start with a legal test-type prefix. \
                 Files in '{directory}/' are typed by their prefix, which is the virtual folder. \
                 Rename the file to '<type>_<subject>.<ext>' — legal '{directory}/' prefixes: {legal}."
            ),
        };

        let (why, fix) = match &foreign {
            Some((prefix, home)) => (
                format!(
                    "Test file '{filename}' starts with '{prefix}', which belongs in '{home}/', \
                     but this file sits in '{directory}/'. The prefix is the virtual folder."
                ),
                format!(
                    "Move the file to '{home}/' or rename it with a '{directory}/' prefix — \
                     legal '{directory}/' prefixes: {legal}."
                ),
            ),
            None => (
                format!(
                    "Test file '{filename}' does not start with a legal test-type prefix. \
                     Files in '{directory}/' are typed by their prefix, which is the virtual folder."
                ),
                format!(
                    "Rename the file to '<type>_<subject>.<ext>' — legal '{directory}/' prefixes: {legal}."
                ),
            ),
        };

        string_filename_result(
            file,
            RULE_CODE_TEST_FILE_PREFIX,
            message,
            Severity::HIGH,
            "TEST_FILE_PREFIX",
            why,
            fix,
        )
    }
}

/// Whether *filename* is a module barrel, which carries no test of its own.
fn is_barrel(filename: &str) -> bool {
    matches!(
        filename,
        "mod.rs" | "lib.rs" | "__init__.py" | "index.ts" | "index.js"
    )
}
