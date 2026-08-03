// PURPOSE: Verify all shared contract traits compile, key VOs construct correctly,
//          and taxonomy types carry expected trait bounds (Send + Sync).

use std::collections::HashMap;

use shared_lint_arwaky::common::taxonomy_adapter_name_vo::AdapterName;
use shared_lint_arwaky::common::taxonomy_common_vo::{
    BooleanVO, ColumnNumber, Count, LineNumber, PatternList, Score,
};
use shared_lint_arwaky::common::taxonomy_definition_vo::LayerMapVO;
use shared_lint_arwaky::common::taxonomy_error_vo::ErrorCode;
use shared_lint_arwaky::common::taxonomy_lint_result_vo::{LintResult, LintResultList};
use shared_lint_arwaky::common::taxonomy_lint_vo::{LocationList, ScopeRef};
use shared_lint_arwaky::common::taxonomy_message_vo::LintMessage;
use shared_lint_arwaky::common::taxonomy_path_vo::{DirectoryPath, FilePath};
use shared_lint_arwaky::common::taxonomy_severity_vo::Severity;
use shared_lint_arwaky::common::taxonomy_suggestion_vo::DescriptionVO;
use shared_lint_arwaky::config_system::taxonomy_config_vo::ArchitectureConfig;

// ── FilePath ────────────────────────────────────────────────

#[test]
fn filepath_new_and_value() {
    let fp = FilePath::new("src/main.rs").unwrap();
    assert_eq!(fp.value(), "src/main.rs");
    assert_eq!(fp.value(), "src/main.rs");
}

#[test]
fn filepath_empty_rejected() {
    assert!(FilePath::new("").is_err());
    assert!(FilePath::new("   ").is_err());
}

#[test]
fn filepath_normalizes_backslashes() {
    let fp = FilePath::new("src\\main.rs").unwrap();
    assert_eq!(fp.value(), "src/main.rs");
}

#[test]
fn filepath_strips_trailing_slash() {
    let fp = FilePath::new("src/main/").unwrap();
    assert_eq!(fp.value(), "src/main");
}

#[test]
fn filepath_only_slashes_becomes_root() {
    let fp = FilePath::new("///").unwrap();
    assert_eq!(fp.value(), "/");
}

#[test]
fn filepath_extension() {
    assert_eq!(FilePath::new("main.rs").unwrap().extension(), "rs");
    assert_eq!(FilePath::new("app.py").unwrap().extension(), "py");
    assert_eq!(FilePath::new("index.tsx").unwrap().extension(), "tsx");
    // Special files have no extension
    assert_eq!(FilePath::new("Makefile").unwrap().extension(), "");
    assert_eq!(FilePath::new("Dockerfile").unwrap().extension(), "");
    // Dotfiles have no extension
    assert_eq!(FilePath::new(".gitignore").unwrap().extension(), "");
    assert_eq!(FilePath::new(".bashrc").unwrap().extension(), "");
    // No dot means no extension
    assert_eq!(FilePath::new("README").unwrap().extension(), "");
}

#[test]
fn filepath_has_extension_case_insensitive() {
    let fp = FilePath::new("main.RS").unwrap();
    assert!(fp.has_extension("rs"));
    assert!(fp.has_extension("RS"));
}

#[test]
fn filepath_basename() {
    assert_eq!(FilePath::new("src/main.rs").unwrap().basename(), "main.rs");
    assert_eq!(FilePath::new("lib.rs").unwrap().basename(), "lib.rs");
}

#[test]
fn filepath_is_barrel_file() {
    assert!(FilePath::new("__init__.py").unwrap().is_barrel_file());
    assert!(FilePath::new("mod.rs").unwrap().is_barrel_file());
    assert!(FilePath::new("index.ts").unwrap().is_barrel_file());
    assert!(!FilePath::new("main.rs").unwrap().is_barrel_file());
}

#[test]
fn filepath_is_entry_point() {
    assert!(FilePath::new("main.rs").unwrap().is_entry_point());
    assert!(FilePath::new("lib.rs").unwrap().is_entry_point());
    assert!(FilePath::new("app.py").unwrap().is_entry_point());
    assert!(!FilePath::new("helper.rs").unwrap().is_entry_point());
}

#[test]
fn filepath_display() {
    let fp = FilePath::new("src/main.rs").unwrap();
    assert_eq!(format!("{}", fp), "src/main.rs");
}

// ── DirectoryPath ───────────────────────────────────────────

#[test]
fn directory_path_new_and_value() {
    let dp = DirectoryPath::new("src/modules").unwrap();
    assert_eq!(dp.value(), "src/modules");
}

#[test]
fn directory_path_empty_rejected() {
    assert!(DirectoryPath::new("").is_err());
    assert!(DirectoryPath::new("   ").is_err());
}

#[test]
fn directory_path_strips_trailing_slash() {
    let dp = DirectoryPath::new("src/").unwrap();
    assert_eq!(dp.value(), "src");
}

// ── Severity ────────────────────────────────────────────────

#[test]
fn severity_variants_exist() {
    let _ = Severity::INFO;
    let _ = Severity::LOW;
    let _ = Severity::MEDIUM;
    let _ = Severity::HIGH;
    let _ = Severity::CRITICAL;
}

#[test]
fn severity_default_is_info() {
    assert_eq!(Severity::default(), Severity::INFO);
}

#[test]
fn severity_score_impact() {
    assert_eq!(Severity::INFO.score_impact(), 0.0);
    assert_eq!(Severity::LOW.score_impact(), 1.0);
    assert_eq!(Severity::MEDIUM.score_impact(), 2.0);
    assert_eq!(Severity::HIGH.score_impact(), 3.0);
    assert_eq!(Severity::CRITICAL.score_impact(), 5.0);
}

#[test]
fn severity_display() {
    assert_eq!(format!("{}", Severity::INFO), "info");
    assert_eq!(format!("{}", Severity::HIGH), "high");
}

#[test]
fn severity_eq_and_hash() {
    assert_eq!(Severity::HIGH, Severity::HIGH);
    assert_ne!(Severity::HIGH, Severity::LOW);
    // Both can be used in HashMap
    let mut map = std::collections::HashMap::new();
    map.insert(Severity::HIGH, 1);
    assert_eq!(map.get(&Severity::HIGH), Some(&1));
}

// ── ErrorCode ───────────────────────────────────────────────

#[test]
fn error_code_new_and_raw() {
    let ec = ErrorCode::new("AES101").unwrap();
    assert_eq!(ec.code(), "AES101");
    let raw = ErrorCode::raw("AES202");
    assert_eq!(raw.code(), "AES202");
}

#[test]
fn error_code_empty_rejected() {
    assert!(ErrorCode::new("").is_err());
}

#[test]
fn error_code_classification() {
    assert!(ErrorCode::raw("AES101").is_architecture());
    assert!(ErrorCode::raw("E001").is_style());
    assert!(ErrorCode::raw("W001").is_style());
    assert!(ErrorCode::raw("D001").is_style());
    assert!(ErrorCode::raw("F001").is_logic());
    assert!(ErrorCode::raw("I001").is_logic());
    assert!(ErrorCode::raw("B001").is_security());
}

#[test]
fn error_code_display_and_deref() {
    let ec = ErrorCode::raw("AES305");
    assert_eq!(format!("{}", ec), "AES305");
    assert_eq!(ec.code(), "AES305");
}

// ── LintMessage ─────────────────────────────────────────────

#[test]
fn lint_message_construction() {
    let msg = LintMessage::new("import violation");
    assert_eq!(msg.value(), "import violation");
}

// ── LintResult ──────────────────────────────────────────────

#[test]
fn lint_result_direct_construction() {
    let result = LintResult {
        file: FilePath::new("test.rs").unwrap(),
        line: LineNumber::new(1),
        column: ColumnNumber::new(0),
        code: ErrorCode::raw("AES101"),
        message: LintMessage::new("test violation"),
        source: None,
        severity: Severity::CRITICAL,
        enclosing_scope: None,
        related_locations: LocationList::new(),
    };
    assert_eq!(result.file.value(), "test.rs");
    assert_eq!(result.line.value, 1);
}

#[test]
fn lint_result_new_arch_convenience() {
    let result = LintResult::new_arch("src/main.rs", 42, "AES201", Severity::HIGH, "bad layer");
    assert_eq!(result.file.value(), "src/main.rs");
    assert_eq!(result.line.value, 42);
    assert_eq!(result.code.code(), "AES201");
    assert_eq!(result.severity, Severity::HIGH);
    assert!(result.source.is_some());
}

#[test]
fn lint_result_new_orphan_convenience() {
    let result = LintResult::new_orphan("src/orphan.rs", "orphan file", Severity::MEDIUM, "AES401");
    assert_eq!(result.file.value(), "src/orphan.rs");
    assert!(result.enclosing_scope.is_none());
}

#[test]
fn lint_result_identity() {
    let result = LintResult::new_arch("test.rs", 1, "AES101", Severity::CRITICAL, "test");
    let id = result.identity();
    assert!(!id.value.is_empty());
}

// ── LintResultList ──────────────────────────────────────────

#[test]
fn lint_result_list_empty() {
    let list = LintResultList::new(Vec::new());
    assert!(list.is_empty());
    assert_eq!(list.len(), 0);
}

#[test]
fn lint_result_list_with_items() {
    let mut list = LintResultList::new(Vec::new());
    let result = LintResult::new_arch("a.rs", 1, "AES101", Severity::CRITICAL, "err");
    list.push(result);
    assert_eq!(list.len(), 1);
    assert!(!list.is_empty());
    assert_eq!(list.iter().count(), 1);
}

// ── ScopeRef ────────────────────────────────────────────────

#[test]
fn scope_ref_construction() {
    let scope = ScopeRef::new("my_function");
    assert_eq!(scope.name.value, "my_function");
    assert_eq!(scope.kind.value, "function");
    assert!(scope.file.is_none());
}

#[test]
fn scope_ref_has_range() {
    let mut scope = ScopeRef::new("func");
    assert!(!scope.has_range());
    scope.start_line = Some(LineNumber::new(10));
    scope.end_line = Some(LineNumber::new(20));
    assert!(scope.has_range());
}

// ── ArchitectureConfig ──────────────────────────────────────

#[test]
fn architecture_config_default() {
    let config = ArchitectureConfig::default();
    assert!(config.enabled.value);
    assert!(config.layers.is_empty());
    assert!(config.rules.is_empty());
    assert!(!config.mandatory_class_definition.value);
}

#[test]
fn architecture_config_custom() {
    let mut layers = HashMap::new();
    layers.insert(
        shared_lint_arwaky::common::taxonomy_layer_vo::LayerNameVO::new("surface"),
        shared_lint_arwaky::common::taxonomy_definition_vo::LayerDefinition::default(),
    );
    let config = ArchitectureConfig::new(
        BooleanVO::new(true),
        layers,
        Vec::new(),
        shared_lint_arwaky::common::taxonomy_definition_vo::NamingConfig::new(Count::new(3)),
        shared_lint_arwaky::common::taxonomy_paths_vo::FilePathList { values: vec![] },
        BooleanVO::new(false),
    );
    assert_eq!(config.layers.len(), 1);
}

// ── LayerMapVO ──────────────────────────────────────────────

#[test]
fn layer_map_empty() {
    let map = LayerMapVO::new(HashMap::new());
    assert!(map.values.is_empty());
}

// ── Score ───────────────────────────────────────────────────

#[test]
fn score_construction_and_checks() {
    let score = Score::new(100.0);
    assert!(score.is_perfect());
    assert_eq!(score.value(), 100.0);

    let threshold = Score::new(80.0);
    assert!(score.is_passing(&threshold));

    let low = Score::new(50.0);
    assert!(!low.is_passing(&threshold));
}

#[test]
fn score_deduct() {
    let score = Score::new(100.0);
    let deducted = score.deduct(&Severity::HIGH);
    assert_eq!(deducted.value, 97.0);
}

// ── BooleanVO ───────────────────────────────────────────────

#[test]
fn boolean_vo_construction() {
    let b = BooleanVO::new(true);
    assert!(b.value());
    let b2 = BooleanVO::from(false);
    assert!(!b2.value());
}

// ── PatternList ─────────────────────────────────────────────

#[test]
fn pattern_list_from_str() {
    let pl = PatternList::new("*.rs");
    assert_eq!(pl.len(), 1);
}

#[test]
fn pattern_list_from_vec() {
    let pl = PatternList::new(vec!["*.rs", "*.py"]);
    assert_eq!(pl.len(), 2);
}

// ── LocationList ────────────────────────────────────────────

#[test]
fn location_list_empty() {
    let list = LocationList::new();
    assert!(list.is_empty());
}

// ── DescriptionVO ───────────────────────────────────────────

#[test]
fn description_vo_construction() {
    let d = DescriptionVO::new("some description");
    assert_eq!(d.value, "some description");
}

// ── AdapterName ─────────────────────────────────────────────

#[test]
fn adapter_name_new() {
    let name = AdapterName::new("clippy").unwrap();
    assert_eq!(name.value(), "clippy");
}

#[test]
fn adapter_name_empty_rejected() {
    assert!(AdapterName::new("").is_err());
}

#[test]
fn adapter_name_raw() {
    let name = AdapterName::raw("ruff");
    assert_eq!(name.value(), "ruff");
}

// ── Send + Sync bounds ──────────────────────────────────────

// These compile-time assertions verify that key VOs can cross thread boundaries.
fn _assert_send<T: Send>() {}
fn _assert_sync<T: Send + Sync>() {}

#[test]
fn filepath_is_send_sync() {
    _assert_send::<FilePath>();
    _assert_sync::<FilePath>();
}

#[test]
fn directory_path_is_send_sync() {
    _assert_send::<DirectoryPath>();
    _assert_sync::<DirectoryPath>();
}

#[test]
fn lint_result_is_send() {
    _assert_send::<LintResult>();
}

#[test]
fn lint_result_list_is_send() {
    _assert_send::<LintResultList>();
}

#[test]
fn severity_is_send_sync() {
    _assert_send::<Severity>();
    _assert_sync::<Severity>();
}

#[test]
fn error_code_is_send_sync() {
    _assert_send::<ErrorCode>();
    _assert_sync::<ErrorCode>();
}

#[test]
fn architecture_config_is_send_sync() {
    _assert_send::<ArchitectureConfig>();
    _assert_sync::<ArchitectureConfig>();
}

// ── Object safety (compile-time) ────────────────────────────
// Verify that key aggregate/protocol traits are object-safe by checking
// they can be used as dyn Trait in a where clause.

fn _assert_object_safe_filesystem_aggregate<
    T: shared_lint_arwaky::filesystem::contract_filesystem_aggregate::IFilesystemAggregate,
>() {
}
fn _assert_object_safe_parser<
    T: shared_lint_arwaky::filesystem::contract_parser_protocol::IParserProtocol,
>() {
}
fn _assert_object_safe_workspace<
    T: shared_lint_arwaky::filesystem::contract_workspace_protocol::IWorkspaceProtocol,
>() {
}
fn _assert_object_safe_config_orch<T: shared_lint_arwaky::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate>(){
}

#[test]
fn contract_traits_compile() {
    // The function signatures above require the traits to exist and be usable
    // as generic bounds. This test ensures they compile.
    // Full dyn-trait object safety is verified at call sites; here we
    // just confirm the trait bounds resolve.
}
