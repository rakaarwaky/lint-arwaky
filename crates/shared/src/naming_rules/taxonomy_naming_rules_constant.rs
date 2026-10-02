// PURPOSE: naming constants — shared rule codes, adapter names, and layer prefixes for naming-rules feature

/// Rule code for AES101 — Naming Convention Consistency
pub const RULE_CODE_NAMING_CONVENTION: &str = "AES101";

/// Rule code for AES102 — Suffix/Prefix Layer Alignment
pub const RULE_CODE_SUFFIX_PREFIX: &str = "AES102";

/// Rule code for AES103 — Test File Prefix Discipline
pub const RULE_CODE_TEST_FILE_PREFIX: &str = "AES103";

/// Adapter name for architecture lint
pub const ADAPTER_NAME: &str = "architecture";

/// AES layer prefixes (must match extract_layer_from_prefix in LayerDetectionAnalyzer)
pub const LAYER_PREFIXES: &[&str] = &[
    "taxonomy_",
    "contract_",
    "utility_",
    "capabilities_",
    "agent_",
    "surface_",
    "root_",
];

/// Separator for snake_case naming
pub const SNAKE_CASE_SEPARATOR: &str = "_";

/// Suffix policy value for strict enforcement
pub const SUFFIX_POLICY_STRICT: &str = "strict";

/// Source file extensions recognized by naming checks
pub const SOURCE_EXTENSIONS: &[&str] = &["rs", "py", "js", "ts", "jsx", "tsx"];

/// Marker in specialized layer names (e.g., "agent(orchestrator)")
pub const SPECIALIZED_LAYER_MARKER: &str = "(";

/// Default minimum word count for stem validation in naming checks
pub const MIN_WORDS_DEFAULT: usize = 3;

// ─── AES103 — Test/bench file prefix vocabulary ─────────────────────────────
//
// The aes-testing-suite skill fixes one test layout for all three languages:
// tests live in `tests/`, benchmarks in `benches/`, and the flat file-name
// prefix IS the virtual folder. These constants are the machine-readable half
// of that table; the skill's markdown table is the human-readable half.

/// The directory every test file lives in.
pub const TESTS_DIR: &str = "tests";

/// The directory every benchmark file lives in.
pub const BENCHES_DIR: &str = "benches";

/// The seven test types that live in `tests/`, each answering one question.
///
/// AES704 requires one file per prefix; AES103 accepts them plus the support
/// prefixes below, because a name that is legal and a category that is present
/// are two different questions.
pub const TEST_TYPE_PREFIXES: &[&str] = &[
    "contract_",
    "unit_",
    "integration_",
    "dogfood_",
    "smoke_",
    "e2e_",
    "acceptance_",
];

/// Prefixes for files in `tests/` that support the suite without being one of
/// the seven test types.
///
/// Each is a convention the repository already follows:
///
/// - `regression_` — TEST.md §2.0 requires one per CRITICAL/WARNING fix
/// - `behavioral_` — behaviour-first coverage of a surface's real rendering
/// - `mock_` — a shared filesystem/seam double, not a test itself
/// - `fixture_` — shared setup data, same role as `mock_`
///
/// A file with one of these prefixes is legal in `tests/` and is NOT counted
/// toward the AES704 category floor.
pub const TEST_SUPPORT_PREFIXES: &[&str] = &["regression_", "behavioral_", "mock_", "fixture_"];

/// The benchmark type prefix, the only legal stem start inside `benches/`.
pub const BENCH_PREFIX: &str = "bench_";

/// Directories inside `tests/` that hold shared support code rather than tests.
///
/// Rust reaches them with `mod common;`, so a support module cannot live flat
/// beside the test files without colliding with them. The directory is the one
/// exception to the flat-layout rule; AES704 does not read it for categories.
pub const TEST_SUPPORT_DIRS: &[&str] = &["common"];

/// The seven test types plus the support prefixes, as one list.
///
/// Kept here beside the two sets it joins so the vocabulary cannot drift: a
/// prefix added to either set is legal the moment it lands.
pub const ALL_TESTS_DIR_PREFIXES: &[&str] = &[
    "contract_",
    "unit_",
    "integration_",
    "dogfood_",
    "smoke_",
    "e2e_",
    "acceptance_",
    "regression_",
    "behavioral_",
    "mock_",
    "fixture_",
];
