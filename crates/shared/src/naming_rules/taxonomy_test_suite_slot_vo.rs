// PURPOSE: TestSuiteSlot — which test-suite directory a path belongs to
//
// The slot is a label both AES103 (which file names are legal) and AES704
// (which category a folder still owes) read, so it is a domain value rather
// than a helper-local detail. The path-to-slot classification itself is a pure
// function and stays in the utility layer.
/// Which test-suite slot a path belongs to, if any.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestSuiteSlot {
    /// Directly inside `tests/` — one of the seven test types.
    Tests,
    /// Directly inside `benches/` — the benchmark type.
    Benches,
    /// Outside both directories, so no test-type prefix applies.
    Outside,
}

impl TestSuiteSlot {
    /// The label used in a violation message, e.g. `tests`.
    pub fn directory_label(self) -> &'static str {
        match self {
            Self::Tests => crate::taxonomy_naming_rules_constant::TESTS_DIR,
            Self::Benches => crate::taxonomy_naming_rules_constant::BENCHES_DIR,
            Self::Outside => "(none)",
        }
    }
}
