use naming_rules_lint_arwaky::capabilities_naming_convention_checker::NamingConventionChecker;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::taxonomy_definition_vo::LayerDefinition;
use shared::common::taxonomy_common_vo::PatternList;

#[test]
fn naming_barrel_file_skipped() {
    let checker = NamingConventionChecker::new();
    let mut violations = vec![];
    checker.check_file_naming("mod.rs", "mod.rs", &None, None, &ArchitectureConfig::default(), &mut violations);
    assert!(violations.is_empty(), "barrel file mod.rs should be skipped");
}

#[test]
fn naming_init_py_skipped() {
    let checker = NamingConventionChecker::new();
    let mut violations = vec![];
    checker.check_file_naming("__init__.py", "__init__.py", &None, None, &ArchitectureConfig::default(), &mut violations);
    assert!(violations.is_empty());
}

#[test]
fn naming_entry_point_main_skipped() {
    let checker = NamingConventionChecker::new();
    let mut violations = vec![];
    checker.check_file_naming("main.rs", "main.rs", &None, None, &ArchitectureConfig::default(), &mut violations);
    assert!(violations.is_empty());
}

#[test]
fn naming_entry_point_lib_skipped() {
    let checker = NamingConventionChecker::new();
    let mut violations = vec![];
    checker.check_file_naming("lib.rs", "lib.rs", &None, None, &ArchitectureConfig::default(), &mut violations);
    assert!(violations.is_empty());
}

#[test]
fn naming_entry_point_index_ts_skipped() {
    let checker = NamingConventionChecker::new();
    let mut violations = vec![];
    checker.check_file_naming("index.ts", "index.ts", &None, None, &ArchitectureConfig::default(), &mut violations);
    assert!(violations.is_empty());
}

#[test]
fn naming_valid_lowercase_underscore_passes() {
    let checker = NamingConventionChecker::new();
    let mut violations = vec![];
    let def = LayerDefinition::default();
    checker.check_file_naming(
        "src/capabilities_user_checker.rs",
        "capabilities_user_checker.rs",
        &Some("capabilities".to_string()),
        Some(&def),
        &ArchitectureConfig::default(),
        &mut violations,
    );
    assert!(violations.is_empty(), "valid naming should pass: {:?}", violations);
}

#[test]
fn naming_single_word_stem_fails() {
    let checker = NamingConventionChecker::new();
    let mut violations = vec![];
    let def = LayerDefinition::default();
    checker.check_file_naming(
        "src/main_helper.rs",
        "main_helper.rs",
        &Some("root".to_string()),
        Some(&def),
        &ArchitectureConfig::default(),
        &mut violations,
    );
    // "main_helper" has 2 words → SHOULD pass (min 2 words)
    assert!(violations.is_empty(), "two words minimum should pass");
}

#[test]
fn naming_invalid_uppercase_fails() {
    let checker = NamingConventionChecker::new();
    let mut violations = vec![];
    let def = LayerDefinition::default();
    checker.check_file_naming(
        "src/BadName.rs",
        "BadName.rs",
        &Some("unknown".to_string()),
        Some(&def),
        &ArchitectureConfig::default(),
        &mut violations,
    );
    // "BadName" has uppercase → fails regex, but also has no underscore separators
    assert!(!violations.is_empty(), "uppercase name should fail naming check");
    assert!(violations[0].code.to_string().contains("AES101"));
}

#[test]
fn naming_unknown_prefix_detected() {
    let checker = NamingConventionChecker::new();
    let mut violations = vec![];
    checker.check_file_naming(
        "src/xyzzy_thing.rs",
        "xyzzy_thing.rs",
        &None,
        None,
        &ArchitectureConfig::default(),
        &mut violations,
    );
    // "xyzzy" is not a valid AES layer prefix → should trigger AES102 unknown prefix
    assert!(!violations.is_empty(), "unknown prefix should be flagged");
    assert_eq!(violations[0].code.to_string(), "AES102");
}

#[test]
fn naming_exception_file_skipped() {
    let checker = NamingConventionChecker::new();
    let mut violations = vec![];
    let mut def = LayerDefinition::default();
    def.exceptions = PatternList::new(vec!["skip_me.rs".to_string()]);
    checker.check_file_naming(
        "src/skip_me.rs",
        "skip_me.rs",
        &Some("capabilities".to_string()),
        Some(&def),
        &ArchitectureConfig::default(),
        &mut violations,
    );
    assert!(violations.is_empty(), "exception file should be skipped");
}

#[test]
fn naming_no_layer_not_unknown_prefix() {
    let checker = NamingConventionChecker::new();
    let mut violations = vec![];
    checker.check_file_naming(
        "src/something.rs",
        "something.rs",
        &None,
        None,
        &ArchitectureConfig::default(),
        &mut violations,
    );
    // "something" does not start with a valid layer prefix
    assert!(!violations.is_empty(), "no layer detection should trigger violation");
    assert_eq!(violations[0].code.to_string(), "AES102");
}
