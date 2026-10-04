// PURPOSE: TestSuiteCoverageAuditor — AES704: per-category test-suite coverage

use shared_naming_rules::utility_test_prefix::{
    BENCH_PREFIX, BENCHES_DIR, TEST_TYPE_PREFIXES, TESTS_DIR, prefix_list,
};
use shared_structure_rules::contract_structure_protocol::IStructureTestSuiteProtocol;
use shared_structure_rules::taxonomy_structure_rules_constant as consts;
use shared_structure_rules::taxonomy_structure_rules_request::{
    StructureFinding, StructureRequest,
};
use shared_structure_rules::taxonomy_structure_rules_response::StructureResponse;
use shared_structure_rules::utility_structure_parsers::{self, sorted};

use std::path::Path;

// ─── Block 1: Struct Definition ────────────────────────────

/// AES704: checks that every feature folder holding source carries at least one
/// file per test type in `tests/` and at least one benchmark in `benches/`.
///
/// A folder with no `tests/` directory at all is still a feature that owes a
/// suite, so the missing directory is reported rather than treated as an
/// exemption. Folders that own no source — a pure `shared/` kernel or a
/// surface — are out of scope.
pub struct TestSuiteCoverageAuditor {}

// ─── Block 2: Protocol Trait Implementation ────────────────

impl IStructureTestSuiteProtocol for TestSuiteCoverageAuditor {
    fn audit_test_suite(&self, request: StructureRequest) -> StructureResponse {
        let StructureRequest::AuditAll { root } = request;
        let ws_root = workspace_root(&root);
        let mut findings = Vec::new();

        for member in utility_structure_parsers::member_dirs(&ws_root) {
            for folder in utility_structure_parsers::feature_dirs(&member) {
                if folder_name(&folder) == consts::KERNEL_DIR {
                    continue;
                }
                let inventory = utility_structure_parsers::inventory(&folder);
                let folder_rel = relative_to(&folder, &ws_root);
                // A folder owning neither capabilities nor an orchestrator holds
                // no feature source, so it owes no suite. Mirrors the AES702 gate.
                if !inventory.has_capabilities && !inventory.has_orchestrator {
                    continue;
                }
                check_feature_test_suite(&folder, &folder_rel, &mut findings);
            }
        }

        StructureResponse::Findings {
            findings: sorted(findings),
        }
    }
}

// ─── Block 3: Constructors, Std Traits, Helpers ────────────

impl Default for TestSuiteCoverageAuditor {
    fn default() -> Self {
        Self::new()
    }
}

impl TestSuiteCoverageAuditor {
    pub fn new() -> Self {
        Self {}
    }
}

/// Resolve the workspace root: the nearest ancestor holding a member directory.
fn workspace_root(root: &Path) -> std::path::PathBuf {
    if root.join("crates").is_dir()
        || root.join("modules").is_dir()
        || root.join("packages").is_dir()
    {
        return root.to_path_buf();
    }
    root.parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| root.to_path_buf())
}

fn folder_name(folder: &Path) -> String {
    folder
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or_default()
        .to_string()
}

fn relative_to(folder: &Path, ws_root: &Path) -> String {
    match folder.strip_prefix(ws_root) {
        Ok(p) => p.to_string_lossy().replace('\\', "/"),
        Err(_) => folder.to_string_lossy().replace('\\', "/"),
    }
}

/// The stem of every file sitting directly in *dir*, sorted for stable output.
/// The stems of every collectable file sitting directly in *dir*, sorted.
fn test_stems_in(dir: &Path) -> Vec<String> {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut stems: Vec<String> = entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_file())
        .filter(|p| {
            p.extension()
                .and_then(|ext| ext.to_str())
                .is_some_and(|ext| consts::TEST_FILE_EXTENSIONS.contains(&ext))
        })
        .filter_map(|p| p.file_stem().and_then(|s| s.to_str()).map(String::from))
        .collect();
    stems.sort();
    stems
}

/// AES704 — a feature folder owes one file per test category.
///
/// Reports the missing `tests/` and `benches/` directories themselves, then one
/// finding per absent prefix. Per-category findings (rather than one combined
/// "your suite is incomplete") let a reader see exactly which seam is untested
/// and match the message against the skill's own table.
fn check_feature_test_suite(folder: &Path, rel: &str, findings: &mut Vec<StructureFinding>) {
    let tests_dir = folder.join(TESTS_DIR);
    let benches_dir = folder.join(BENCHES_DIR);

    if !tests_dir.is_dir() {
        findings.push(
            StructureFinding::new(
                consts::RULE_CODE_TEST_SUITE_COVERAGE,
                consts::TEST_SUITE_VIOLATION_MISSING_TESTS_DIR,
                rel.to_string(),
                format!("feature folder '{rel}' has no '{TESTS_DIR}/' directory"),
            )
            .with_reason(
                format!(
                    "A feature folder owes one test file per test type ({}).",
                    prefix_list(required_tests_prefixes())
                ),
                format!("Add a '{TESTS_DIR}/' directory with one file per required test type."),
            ),
        );
    } else {
        let present = test_stems_in(&tests_dir);
        for prefix in required_tests_prefixes() {
            if present.iter().any(|s| s.starts_with(prefix)) {
                continue;
            }
            findings.push(missing_category(
                rel,
                prefix,
                &format!("{TESTS_DIR}/"),
                format!("add '{prefix}<subject>'"),
            ));
        }
    }

    if !benches_dir.is_dir() {
        findings.push(StructureFinding::new(
            consts::RULE_CODE_TEST_SUITE_COVERAGE,
            consts::TEST_SUITE_VIOLATION_MISSING_BENCH_DIR,
            rel.to_string(),
            format!("feature folder '{rel}' has no '{BENCHES_DIR}/' directory"),
        )
        .with_reason(
            format!(
                "A feature folder owes at least one '{BENCH_PREFIX}<subject>' performance regression benchmark."
            ),
            format!("Add a '{BENCHES_DIR}/' directory holding one '{BENCH_PREFIX}<subject>' benchmark."),
        ));
    } else {
        let present = test_stems_in(&benches_dir);
        if !present.iter().any(|s| s.starts_with(BENCH_PREFIX)) {
            findings.push(missing_category(
                rel,
                BENCH_PREFIX,
                &format!("{BENCHES_DIR}/"),
                format!("add '{BENCH_PREFIX}<subject>'"),
            ));
        }
    }
}

/// Build the missing-category finding, reading the purpose out of the shared
/// category table so the message states what the absent test type proves.
fn missing_category(rel: &str, prefix: &str, directory: &str, fix: String) -> StructureFinding {
    let purpose = consts::TEST_SUITE_CATEGORY_PURPOSE
        .iter()
        .find(|(p, _)| *p == prefix)
        .map(|(_, purpose)| *purpose)
        .unwrap_or("its own test type");
    StructureFinding::new(
        consts::RULE_CODE_TEST_SUITE_COVERAGE,
        consts::TEST_SUITE_VIOLATION_MISSING_CATEGORY,
        format!("{rel}/{directory}"),
        format!("feature folder '{rel}' holds no '{prefix}' test file"),
    )
    .with_reason(
        format!("Every test type is required. '{prefix}' proves: {purpose}."),
        format!("{fix} The prefix is the virtual folder."),
    )
}

/// The seven prefixes a `tests/` directory owes, read from the shared
/// AES103/AES704 vocabulary so the two rules cannot drift apart.
fn required_tests_prefixes() -> &'static [&'static str] {
    TEST_TYPE_PREFIXES
}
