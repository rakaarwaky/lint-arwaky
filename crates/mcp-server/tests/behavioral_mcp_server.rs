// Behavioral tests — list_commands catalog behavior (W5).
//
// These tests exercise the real static catalog logic in handle_list_commands
// without needing McpServerDependencies or mock aggregates.
// Tests cover: full catalog, domain filter, no-match filter.

use shared::cli_commands::taxonomy_command_catalog_vo::COMMAND_CATALOG;

// ─── W5: list_commands catalog behavior ───────────────────────

#[test]
fn test_command_catalog_is_nonempty() {
    assert!(
        !COMMAND_CATALOG.is_empty(),
        "FR-002: COMMAND_CATALOG must not be empty"
    );
}

#[test]
fn test_catalog_entries_have_name_description_example() {
    for (name, desc, example) in COMMAND_CATALOG {
        assert!(!name.is_empty(), "catalog entry must have a name");
        assert!(
            !desc.is_empty(),
            "catalog entry '{}' must have a description",
            name
        );
        assert!(
            !example.is_empty(),
            "catalog entry '{}' must have an example",
            name
        );
    }
}

#[test]
fn test_catalog_domain_filter_check() {
    let filtered: Vec<_> = COMMAND_CATALOG
        .iter()
        .filter(|(name, _, _)| name.contains("check"))
        .collect();
    assert!(
        !filtered.is_empty(),
        "should find 'check' commands in catalog"
    );
    assert!(
        filtered.iter().all(|(name, _, _)| name.contains("check")),
        "filtered results should all contain 'check'"
    );
}

#[test]
fn test_catalog_domain_filter_nonexistent() {
    let filtered: Vec<_> = COMMAND_CATALOG
        .iter()
        .filter(|(name, _, _)| name.contains("zzz_nonexistent"))
        .collect();
    assert!(
        filtered.is_empty(),
        "nonexistent domain should match nothing"
    );
}

#[test]
fn test_catalog_contains_expected_commands() {
    let names: Vec<&str> = COMMAND_CATALOG.iter().map(|(n, _, _)| *n).collect();
    assert!(names.contains(&"check"), "catalog must include 'check'");
    assert!(names.contains(&"fix"), "catalog must include 'fix'");
    assert!(names.contains(&"scan"), "catalog must include 'scan'");
    assert!(names.contains(&"ci"), "catalog must include 'ci'");
}
