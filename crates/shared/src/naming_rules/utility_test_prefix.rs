// PURPOSE: Test/bench prefix detection helpers shared by AES103 and AES704
//
// Both rules read the same fact — which test type a path belongs to — so the
// classification lives here once instead of being written twice in two crates.
// This module decides nothing about layout; it only labels a path.

use std::borrow::Cow;

use crate::taxonomy_test_suite_slot_vo::TestSuiteSlot;

// ─── Test-suite layout vocabulary ──────────────────────────────────────────
// Re-exported so AES704 (in the structure-rules crate) labels paths with the
// identical vocabulary AES103 enforces, and the two cannot drift apart.
pub use crate::taxonomy_naming_rules_constant::{
    ALL_TESTS_DIR_PREFIXES, BENCH_PREFIX, BENCHES_DIR, TEST_SUPPORT_DIRS, TEST_SUPPORT_PREFIXES,
    TEST_TYPE_PREFIXES, TESTS_DIR,
};

/// Classify *path* into a test-suite slot.
///
/// The slot is read from the file's own parent directory: a path counts as a
/// test file only when it sits *directly* in `tests/` (or in a support
/// directory beside them). A file at `tests/sub/unit_foo.rs` is reported as
/// nested by the caller rather than silently accepted here, so the flat-layout
/// rule stays a visible finding instead of an assumption.
pub fn slot_of(path: &str) -> TestSuiteSlot {
    match leaf_directory_of(path).as_deref() {
        Some(TESTS_DIR) => TestSuiteSlot::Tests,
        Some(BENCHES_DIR) => TestSuiteSlot::Benches,
        Some(other) if TEST_SUPPORT_DIRS.contains(&other) => TestSuiteSlot::Tests,
        _ => TestSuiteSlot::Outside,
    }
}

/// The legal prefixes for *slot*, or an empty slice for [`TestSuiteSlot::Outside`].
///
/// `tests/` accepts the seven test types plus the support prefixes; `benches/`
/// accepts only the benchmark prefix.
pub fn prefixes_for(slot: TestSuiteSlot) -> &'static [&'static str] {
    match slot {
        TestSuiteSlot::Tests => ALL_TESTS_DIR_PREFIXES,
        TestSuiteSlot::Benches => &[BENCH_PREFIX],
        TestSuiteSlot::Outside => &[],
    }
}

/// Whether *stem* starts with one of *slot*'s legal prefixes.
pub fn has_legal_prefix(stem: &str, slot: TestSuiteSlot) -> bool {
    prefixes_for(slot).iter().any(|p| stem.starts_with(p))
}

/// The first legal prefix *stem* starts with, or `None` when it starts with none.
pub fn matched_prefix(stem: &str, slot: TestSuiteSlot) -> Option<&'static str> {
    prefixes_for(slot)
        .iter()
        .copied()
        .find(|p| stem.starts_with(p))
}

/// Render *prefixes* as a comma-separated list for a violation message.
pub fn prefix_list(prefixes: &[&str]) -> String {
    prefixes.join(", ")
}

/// The name of the directory a file sits directly in, when there is one.
///
/// `leaf_directory_of("crates/calc/tests/unit_foo.rs")` is `Some("tests")` —
/// only the final segment, because that is the one the layout vocabulary names.
/// A file at the scan root has no containing directory, so it yields `None`.
///
/// Returns an owned segment only on the rare Windows path that needed separator
/// rewriting; a POSIX path borrows the leaf straight from the input.
pub fn leaf_directory_of(path: &str) -> Option<Cow<'_, str>> {
    if path.contains('\\') {
        let rewritten = path.replace('\\', "/");
        return leaf_of_normalized(&rewritten).map(|leaf| Cow::Owned(leaf.to_string()));
    }
    leaf_of_normalized(path).map(Cow::Borrowed)
}

/// The containing directory's final segment of an already slash-normalized path.
fn leaf_of_normalized(normalized: &str) -> Option<&str> {
    // Split the *directory* part off first, then take its last segment: taking
    // the last segment of the whole path would return the file name.
    let (parent, _file) = normalized.rsplit_once('/')?;
    let leaf = parent.rsplit('/').next()?;
    if leaf.is_empty() { None } else { Some(leaf) }
}

/// The `tests/` or `benches/` directory *path* is nested inside, when the file
/// sits deeper than one level below it.
///
/// Returns `(directory, levels_below)`. A file at `tests/unit_foo.rs` has no
/// nesting, so this yields `None`; a file at `tests/sub/unit_foo.rs` yields
/// `Some(("tests", 1))`.
///
/// `tests/` is matched first: a path carrying both names nests under whichever
/// is nearer the file, and in every real layout `tests/` is the inner one.
/// A registered support directory (`tests/common/`) counts as flat, since it is
/// the one sanctioned place for shared support code.
pub fn nested_under(path: &str) -> Option<(&'static str, usize)> {
    let normalized = if path.contains('\\') {
        path.replace('\\', "/")
    } else {
        path.to_string()
    };
    let segments: Vec<&str> = normalized.split('/').filter(|s| !s.is_empty()).collect();
    // The levels a file sits below `name`, or `None` when the registered support
    // directory makes it as deep as the layout allows.
    //
    // The exemption is deliberately narrow: a support directory is honoured only
    // directly under `tests/`, and only for the file immediately inside it. A
    // blanket "anything below `common`" test would let `tests/common/deep/…` and
    // `benches/common/…` both slip past the flat-layout rule.
    let levels_below = |name: &'static str| -> Option<usize> {
        let index = segments.iter().rposition(|s| *s == name)?;
        let levels = segments.len() - index - 2;
        if levels >= 1 && supports_shared_code(name, segments.get(index + 1)) {
            return None;
        }
        Some(levels)
    };
    if let Some(levels) = levels_below(TESTS_DIR)
        && levels >= 1
    {
        return Some((TESTS_DIR, levels));
    }
    levels_below(BENCHES_DIR)
        .filter(|levels| *levels >= 1)
        .map(|levels| (BENCHES_DIR, levels))
}

/// Whether `segment` is a registered support directory for files directly under
/// `parent`.
///
/// `tests/common/` is sanctioned because Rust reaches shared support code through
/// a module declaration. `benches/` has no such convention, so `benches/common/`
/// gets none — the vocabulary is per-directory, not global.
fn supports_shared_code(parent: &str, segment: Option<&&str>) -> bool {
    parent == TESTS_DIR && segment.is_some_and(|s| TEST_SUPPORT_DIRS.contains(s))
}
