// PURPOSE: TestFilePrefixChecker — AES103 test/bench file-prefix discipline
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use shared_common::taxonomy_layer_vo::LayerMapVO;
use shared_common::taxonomy_path_vo::FilePath;
use shared_common::taxonomy_path_vo::FilePathList;
use shared_common::taxonomy_severity_vo::Severity;
use shared_common::taxonomy_violation_item_vo::ViolationItem;
use shared_config_system::taxonomy_config_system_vo::ArchitectureConfig;
use shared_naming_rules::TestSuiteSlot;
use shared_naming_rules::contract_test_file_prefix_protocol::ITestFilePrefixProtocol;
use shared_naming_rules::taxonomy_naming_rules_constant::{
    AES103_PREFIX_RULE, BENCHES_DIR, RULE_CODE_TEST_FILE_PREFIX,
};
use shared_naming_rules::utility_naming_checker::{
    basename_of, get_stem, rule_exception_set, string_filename_violation,
};
use shared_naming_rules::utility_naming_output::{exemption_clause, naming_finding};
use shared_naming_rules::utility_test_prefix::{
    has_legal_prefix, matched_prefix, nested_under, prefix_list, prefixes_for, slot_of,
};

// ─── Block 1: Module Declaration ──────────────────────────────

/// Stateless AES103 test-file prefix checker.
///
/// Where AES101 reads a stem's shape and AES102 reads a folder suffix, AES103
/// reads the test-suite prefix of files that sit in `tests/` or `benches/`. It
/// answers two questions: does the file sit directly in that directory, and
/// does its stem start with a legal test-type prefix. No internal state.
pub struct TestFilePrefixChecker {}

impl Default for TestFilePrefixChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl TestFilePrefixChecker {
    pub fn new() -> Self {
        Self {}
    }
}

// ─── Block 2: Module Contract ────────────────

impl ITestFilePrefixProtocol for TestFilePrefixChecker {
    /// FR-NamingRules-003 — AES103: check every file inside `tests/` and
    /// `benches/` against the flat test-type prefix vocabulary.
    fn check_test_file_prefixes(
        &self,
        config: &ArchitectureConfig,
        _layer_map: &LayerMapVO,
        files: &FilePathList,
        _root_dir: &FilePath,
        results: &mut Vec<ViolationItem>,
    ) {
        let exceptions = rule_exception_set(config, RULE_CODE_TEST_FILE_PREFIX);

        let violations: Vec<ViolationItem> = files
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

        results.extend(violations);
    }
}

// ─── Block 3: Module Helpers ────────────

impl TestFilePrefixChecker {
    /// AES103 — a file in `tests/` or `benches/` sits directly in that
    /// directory and starts with a legal test-type prefix.
    ///
    /// Three findings, in the order they are checked:
    ///   1. the file sits below `tests/` or `benches/` (the name prefix records
    ///      the test type, so a subdirectory defeats discovery and CI filters)
    ///   2. the file's stem starts with no legal prefix for its directory
    ///   3. the file's stem starts with a prefix belonging to the *other*
    ///      directory (a bench file in `tests/`, or a test file in `benches/`)
    ///
    /// B-6a test seam: `pub` only so cross-crate unit tests can drive the
    /// per-file rule; production traffic enters through
    /// [`ITestFilePrefixProtocol::check_test_file_prefixes`].
    pub fn check_test_file_prefix_internal(
        &self,
        file: &str,
        filename: &str,
    ) -> Option<ViolationItem> {
        // A nested file is reported by the nesting rule alone: naming its stem
        // against the flat vocabulary would report the same defect twice.
        if let Some((directory, depth)) = nested_under(file) {
            return Some(nested_result(file, filename, directory, depth));
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
        Some(prefix_result(file, filename, stem, slot))
    }
}

/// The subdirectory finding. Names the directory, how deep the file sits, and
/// the name the file should carry there.
///
/// The worked example comes from the directory the finding reports: a file
/// under `benches/` is told about a `bench_` name, because a file under
/// `benches/` may not carry a `unit_` name.
fn nested_result(
    file: &str,
    filename: &str,
    directory: &'static str,
    depth: usize,
) -> ViolationItem {
    let levels = if depth == 1 {
        "1 level".to_string()
    } else {
        format!("{depth} levels")
    };
    let problem = format!(
        "Test file '{filename}' sits {levels} inside '{directory}/'."
    );
    let action = format!(
        "Move the file to '{directory}/' and keep the {} type in its name, for example '{}'.",
        type_word(directory),
        example_for(directory)
    );
    let exemption = exemption_clause(RULE_CODE_TEST_FILE_PREFIX, filename);
    let finding = naming_finding(
        &[problem.as_str()],
        &[
            action.as_str(),
            "Then update the imports that name it.",
            "Re-run the check to confirm the finding is gone.",
            exemption.as_str(),
        ],
        &[AES103_PREFIX_RULE],
    );
    string_filename_violation(
        file,
        RULE_CODE_TEST_FILE_PREFIX,
        finding.message,
        Severity::MEDIUM,
        "TEST_FILE_PREFIX",
        finding.why,
        finding.fix,
    )
}

/// The prefix finding. Distinguishes "no legal prefix at all" from "a prefix
/// from the wrong directory", because the fix differs: the first needs a
/// rename, the second needs a move.
fn prefix_result(file: &str, filename: &str, stem: &str, slot: TestSuiteSlot) -> ViolationItem {
    let directory = slot.directory_label();
    let legal = prefix_list(prefixes_for(slot));

    // A prefix that is legal *somewhere* but not here: a test type inside
    // `benches/`, or a bench inside `tests/`.
    let foreign = [TestSuiteSlot::Tests, TestSuiteSlot::Benches]
        .into_iter()
        .filter(|other| *other != slot)
        .find_map(|other| matched_prefix(stem, other).map(|p| (p, other.directory_label())));

    // One action per finding. A prefix that belongs to the other directory
    // is a move: the name is legal, the place is not. A file with no legal
    // prefix is a rename, and the rename is a pick-one, so the legal
    // prefixes are listed on the action line.
    let (problem, action) = match foreign {
        Some((prefix, home)) => (
            format!(
                "Test file '{filename}' starts with '{prefix}', which belongs in '{home}/'."
            ),
            format!("Move the file to '{home}/'."),
        ),
        None => (
            format!(
                "Test file '{filename}' does not start with a legal test prefix."
            ),
            format!(
                "Rename the file to '<type>_<subject>.rs'. Legal '{directory}/' prefixes: {legal}."
            ),
        ),
    };
    let exemption = exemption_clause(RULE_CODE_TEST_FILE_PREFIX, filename);
    let finding = naming_finding(
        &[problem.as_str()],
        &[
            action.as_str(),
            "Then update the imports that name it.",
            "Re-run the check to confirm the finding is gone.",
            exemption.as_str(),
        ],
        &[AES103_PREFIX_RULE],
    );
    string_filename_violation(
        file,
        RULE_CODE_TEST_FILE_PREFIX,
        finding.message,
        Severity::MEDIUM,
        "TEST_FILE_PREFIX",
        finding.why,
        finding.fix,
    )
}

/// The word the directory uses for the kind of file it holds.
fn type_word(directory: &str) -> &'static str {
    if directory == BENCHES_DIR {
        "benchmark"
    } else {
        "test"
    }
}

/// The worked example for one directory: the name a file there should carry.
fn example_for(directory: &str) -> &'static str {
    if directory == BENCHES_DIR {
        "benches/bench_<subject>.rs"
    } else {
        "tests/unit_<subject>.rs"
    }
}

/// Whether *filename* is a module barrel, which carries no test of its own.
fn is_barrel(filename: &str) -> bool {
    matches!(
        filename,
        "mod.rs" | "lib.rs" | "__init__.py" | "index.ts" | "index.js"
    )
}
