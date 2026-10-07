// Unit tests — shared/common taxonomy VOs and utilities.
mod common;

use shared_common::taxonomy_adapter_name_vo::AdapterName;
use shared_common::taxonomy_common_error::ExitCode;
use shared_common::taxonomy_common_vo::{
    BooleanVO, ColumnNumber, Count, ErrorMessage, LineNumber, PatternList, Score, Timestamp,
};
use shared_common::taxonomy_error_vo::{
    ErrorCode, error_code_is_architecture, error_code_is_logic, error_code_is_security,
    error_code_is_style,
};
use shared_common::taxonomy_format_vo::Format;
use shared_common::taxonomy_job_vo::JobId;
use shared_common::taxonomy_job_vo::{AdapterMetadata, McpConfigVO, SuccessStatus};
use shared_common::taxonomy_language_vo::ConfigLanguage;
use shared_common::taxonomy_language_vo::Language;
use shared_common::taxonomy_layer_vo::{FileContentVO, Identity, LayerNameVO, LineContentVO};
use shared_common::taxonomy_lint_vo::{Location, LocationList, ScopeRef};
use shared_common::taxonomy_message_vo::{DescriptionVO, Suggestion};
use shared_common::taxonomy_path_vo::{DirectoryPath, FilePath};
use shared_common::taxonomy_path_vo::{FilePathList, RenamedFile};
use shared_common::taxonomy_severity_vo::Severity;
use shared_common::taxonomy_threshold_vo::Threshold;
use shared_common::{ComplianceStatus, LintMessage};
use shared_common::{LintResult, LintResultList};
use shared_filesystem::utility_command_runner::{run_command, run_command_in_dir};
use std::str::FromStr;

// ── FilePath ────────────────────────────────────────────────
#[test]
fn file_path_rejects_empty() {
    assert!(FilePath::new("").is_err());
    assert!(FilePath::new("   ").is_err());
}

#[test]
fn file_path_normalizes_separators() {
    let path = FilePath::new("a\\\\b/c.rs").expect("valid path");
    assert_eq!(path.value(), "a/b/c.rs");
}

#[test]
fn file_path_collapses_double_slashes() {
    let path = FilePath::new("a//b///c.rs").expect("valid path");
    assert_eq!(path.value(), "a/b/c.rs");
}

#[test]
fn file_path_trims_trailing_slashes() {
    let path = FilePath::new("/tmp/project/").expect("valid path");
    assert_eq!(path.value(), "/tmp/project");
}

#[test]
fn file_path_all_slashes_becomes_root() {
    let path = FilePath::new("///").expect("valid path");
    assert_eq!(path.value(), "/");
}

#[test]
fn file_path_extension() {
    let path = common::fp("src/main.rs");
    assert_eq!(path.extension(), "rs");
    assert!(path.has_extension("RS"));
}

#[test]
fn file_path_extension_ignores_dotfiles() {
    let path = common::fp("/home/user/.bashrc");
    assert_eq!(path.extension(), "");
}

#[test]
fn file_path_extension_special_files() {
    let path = common::fp("/build/Dockerfile");
    assert_eq!(path.extension(), "");
}

#[test]
fn file_path_basename() {
    let path = common::fp("a/b/c.rs");
    assert_eq!(path.basename(), "c.rs");
}

#[test]
fn file_path_barrel_and_entry_point() {
    let barrel = common::fp("pkg/__init__.py");
    assert!(barrel.is_barrel_file());
    let mod_rs = common::fp("src/mod.rs");
    assert!(mod_rs.is_barrel_file());
    let index = common::fp("src/index.ts");
    assert!(index.is_barrel_file());
    assert!(index.is_entry_point());
    let lib_rs = common::fp("src/lib.rs");
    assert!(lib_rs.is_entry_point());
    let regular = common::fp("src/helper.py");
    assert!(!regular.is_barrel_file());
    assert!(!regular.is_entry_point());
}

#[test]
fn file_path_deref_and_display() {
    let path = common::fp("x.rs");
    assert_eq!(path.to_string(), "x.rs");
    let s: &str = &path;
    assert_eq!(s, "x.rs");
}

#[test]
fn directory_path_rejects_empty_and_normalizes() {
    assert!(DirectoryPath::new("").is_err());
    let dir = DirectoryPath::new("tmp\\\\proj/").expect("valid dir");
    // Backslashes → forward slashes, trailing slash removed
    // Double backslash becomes double slash (not collapsed)
    assert_eq!(dir.value(), "tmp//proj");
}

// ── ErrorCode ───────────────────────────────────────────────
#[test]
fn error_code_new_rejects_empty() {
    assert!(ErrorCode::new("").is_err());
    let code = ErrorCode::new("AES101").expect("valid code");
    assert_eq!(code.code(), "AES101");
}

#[test]
fn error_code_raw_skips_validation() {
    let code = ErrorCode::raw("");
    assert_eq!(code.code(), "");
}

#[test]
fn error_code_category_flags() {
    assert!(error_code_is_architecture("AES101"));
    assert!(error_code_is_style("E401"));
    assert!(error_code_is_style("W501"));
    assert!(error_code_is_style("D100"));
    assert!(error_code_is_logic("F401"));
    assert!(error_code_is_logic("I001"));
    assert!(error_code_is_security("B101"));
}

// ── ExitCode ────────────────────────────────────────────────
#[test]
fn exit_code_named_constants() {
    assert_eq!(ExitCode::OK.value(), 0);
    assert_eq!(ExitCode::POLICY_FAIL.value(), 1);
    assert_eq!(ExitCode::RUNTIME_ERROR.value(), 2);
    assert_eq!(ExitCode::PREREQUISITE_MISSING.value(), 3);
}

#[test]
fn exit_code_from_i64_and_display() {
    let code = ExitCode::from(7);
    assert_eq!(code.value(), 7);
    assert_eq!(code.to_string(), "7");
}

#[test]
fn exit_code_matches_std() {
    assert!(ExitCode::OK.matches_std(&ExitCode::OK.to_process_exit_code()));
    assert!(!ExitCode::POLICY_FAIL.matches_std(&ExitCode::OK.to_process_exit_code()));
}

// ── Severity / Score / Threshold ────────────────────────────
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
    assert_eq!(Severity::HIGH.to_string(), "high");
    assert_eq!(Severity::CRITICAL.to_string(), "critical");
}

#[test]
fn score_perfect_and_passing() {
    let score = Score::new(100.0);
    assert!(score.is_perfect());
    assert!(score.is_passing(&Score::new(80.0)));
    assert!(!Score::new(79.0).is_passing(&Score::new(80.0)));
}

#[test]
fn score_deduct_applies_severity_impact() {
    let after = Score::new(100.0).deduct(&Severity::MEDIUM);
    assert!((after.value() - 98.0).abs() < 1e-9);
}

#[test]
fn threshold_defaults_and_value() {
    assert_eq!(Threshold::default().value(), 100);
    assert_eq!(Threshold::new(75).value(), 75);
    assert_eq!(Threshold::from(60), Threshold::new(60));
}

// ── Languages ───────────────────────────────────────────────
#[test]
fn config_language_parse_and_format() {
    assert_eq!(
        ConfigLanguage::from_str("rust").expect("parses"),
        ConfigLanguage::Rust
    );
    assert_eq!(
        ConfigLanguage::from_str("python").expect("parses"),
        ConfigLanguage::Python
    );
    assert_eq!(
        ConfigLanguage::from_str("ts").expect("parses"),
        ConfigLanguage::TypeScript
    );
    assert_eq!(
        ConfigLanguage::from_str("javascript").expect("parses"),
        ConfigLanguage::TypeScript
    );
    assert!(ConfigLanguage::from_str("cobol").is_err());
    assert_eq!(ConfigLanguage::Rust.as_str(), "rust");
    assert_eq!(ConfigLanguage::TypeScript.to_string(), "typescript");
}

#[test]
fn config_language_file_names() {
    assert_eq!(
        ConfigLanguage::Rust.config_file_names(),
        &["lint_arwaky.config.yaml"]
    );
    assert_eq!(ConfigLanguage::TypeScript.config_file_names().len(), 1);
}

#[test]
fn language_detection_from_extension() {
    assert_eq!(Language::from_extension("rs"), Some(Language::Rust));
    assert_eq!(Language::from_extension("py"), Some(Language::Python));
    assert_eq!(Language::from_extension("tsx"), Some(Language::TypeScript));
    assert_eq!(Language::from_extension("js"), Some(Language::JavaScript));
    assert_eq!(Language::from_extension("txt"), None);
}

#[test]
fn language_detection_from_adapter_name() {
    assert_eq!(Language::from_adapter_name("clippy"), Language::Rust);
    assert_eq!(Language::from_adapter_name("ruff"), Language::Python);
    assert_eq!(Language::from_adapter_name("eslint"), Language::JavaScript);
    assert_eq!(
        Language::from_adapter_name("typescript"),
        Language::TypeScript
    );
    assert_eq!(Language::from_adapter_name("nonsense"), Language::Unknown);
}

#[test]
fn language_metadata_keywords() {
    assert_eq!(Language::Rust.interface_kw(), "trait");
    assert_eq!(Language::Python.interface_kw(), "Protocol");
    assert_eq!(Language::TypeScript.struct_keyword(), "class/interface");
    assert_eq!(
        Language::extensions(),
        &["rs", "py", "ts", "tsx", "js", "jsx"]
    );
    assert_eq!(Language::Rust.as_str(), "rust");
}

#[test]
fn language_from_extension() {
    assert_eq!(Language::from_extension("rs"), Some(Language::Rust));
    assert_eq!(Language::from_extension("py"), Some(Language::Python));
    assert_eq!(Language::from_extension("ts"), Some(Language::TypeScript));
    assert_eq!(Language::from_extension("js"), Some(Language::JavaScript));
    assert_eq!(Language::from_extension("txt"), None);
}

// ── Detect language utilities ───────────────────────────────

// ── Format ──────────────────────────────────────────────────
#[test]
fn format_parse_and_display() {
    assert_eq!(Format::Text.to_string(), "text");
    assert_eq!(Format::from_str("json").expect("parses"), Format::Json);
    assert_eq!(Format::from_str("SARIF").expect("parses"), Format::Sarif);
    assert_eq!(Format::from_str("junit").expect("parses"), Format::Junit);
    assert!(Format::from_str("xml").is_err());
    assert_eq!(Format::default(), Format::Text);
}

// ── String-wrapper VOs ──────────────────────────────────────
#[test]
fn adapter_name_validates_and_derefs() {
    assert!(AdapterName::new("").is_err());
    let name = AdapterName::new("clippy").expect("valid adapter");
    assert_eq!(name.value(), "clippy");
    let raw = AdapterName::raw("anything");
    assert_eq!(raw.to_string(), "anything");
}

#[test]
fn job_id_and_identity_string_vos() {
    let job = JobId::new("scan-1");
    assert_eq!(job.value(), "scan-1");
    assert_eq!(JobId::from("scan-2").value(), "scan-2");
    let id = Identity::new("taxonomy");
    assert_eq!(id.value(), "taxonomy");
    assert_eq!(id.to_string(), "taxonomy");
    assert_eq!(Identity::from(String::from("contract")).value(), "contract");
}

#[test]
fn boolean_and_compliance_status() {
    assert!(BooleanVO::new(true).value());
    assert!(!BooleanVO::from(false).value());
    assert!(ComplianceStatus::new(true).value());
    assert!(!ComplianceStatus::from(false).value());
    assert_eq!(ComplianceStatus::new(true).to_string(), "true");
}

#[test]
fn numeric_primitive_vos() {
    assert_eq!(LineNumber::new(12).value(), 12);
    assert_eq!(LineNumber::from(3), LineNumber::new(3));
    assert_eq!(ColumnNumber::new(4).value(), 4);
    assert_eq!(Count::new(2).value(), 2);
    assert_eq!(LineNumber::new(1).to_string(), "1");
}

#[test]
fn timestamp_now_and_new() {
    let now = Timestamp::now();
    assert!(!now.value().is_empty());
    let fixed = Timestamp::new("2026-01-01T00:00:00Z");
    assert_eq!(fixed.value(), "2026-01-01T00:00:00Z");
    assert_eq!(Timestamp::from("x").value(), "x");
}

#[test]
fn pattern_list_from_various_sources() {
    let single = PatternList::new("strict");
    assert_eq!(single.len(), 1);
    assert_eq!(single.values()[0], "strict");
    let many = PatternList::new(vec!["a".to_string(), "b".to_string()]);
    assert_eq!(many.len(), 2);
    let mut pushed = PatternList::new(Vec::<String>::new());
    pushed.push("c".to_string());
    assert!(!pushed.is_empty());
}

#[test]
fn error_message_conversions() {
    let msg = ErrorMessage::new("boom");
    assert_eq!(msg.value(), "boom");
    assert_eq!(ErrorMessage::from("x").value(), "x");
    assert_eq!(ErrorMessage::from(String::from("y")).value(), "y");
    assert_eq!(msg.to_string(), "boom");
}

// ── Lint VOs ────────────────────────────────────────────────
#[test]
fn lint_result_identity_is_unique_per_location() {
    let a = common::violation("a.rs", 1, "AES101", Severity::LOW);
    let b = common::violation("a.rs", 2, "AES101", Severity::LOW);
    assert_ne!(a.identity(), b.identity());
}

#[test]
fn lint_result_new_arch_sets_defaults() {
    let result = LintResult::new_arch("src/x.rs", 5, "AES101", Severity::HIGH, "bad name");
    assert_eq!(result.code.code(), "AES101");
    assert_eq!(result.line.value(), 5);
    assert_eq!(result.severity, Severity::HIGH);
    assert!(result.enclosing_scope.is_some());
    assert_eq!(
        result.source.expect("arch sets source").value(),
        "architecture"
    );
}

#[test]
fn lint_result_new_orphan_has_no_scope() {
    let result = LintResult::new_orphan("x.py", "orphaned", Severity::MEDIUM, "AES501");
    assert!(result.enclosing_scope.is_none());
    assert_eq!(result.column.value(), 0);
}

#[test]
fn lint_result_list_wrapper() {
    let mut list = LintResultList::new(vec![common::violation("a.rs", 1, "AES101", Severity::LOW)]);
    list.append(common::violation("b.rs", 1, "AES101", Severity::LOW));
    assert_eq!(list.len(), 2);
    assert!(!list.is_empty());
    assert!(LintResultList::default().is_empty());
}

#[test]
fn scope_ref_range_detection() {
    let scope = ScopeRef::new("fn main");
    assert!(!scope.has_range());
    assert!(scope.file.is_none());
}

#[test]
fn location_display_default() {
    let loc = Location::new();
    assert_eq!(loc.to_string(), "unknown");
}

#[test]
fn location_list_push_and_deref() {
    let mut list = LocationList::new();
    list.push(Location::new());
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].file, None);
}

// ── Job VOs ─────────────────────────────────────────────────
#[test]
fn success_status_and_adapter_metadata() {
    assert!(SuccessStatus::new(true).value());
    assert!(!SuccessStatus::default().value());
    let meta = AdapterMetadata::new(AdapterName::raw("ruff"), "ruff.Linter".to_string());
    assert_eq!(meta.class_path, "ruff.Linter");
}

#[test]
fn mcp_config_vo_wraps_map() {
    let mut map = std::collections::HashMap::new();
    map.insert("command".to_string(), serde_json::json!("lint-arwaky"));
    let config = McpConfigVO::new(map);
    assert_eq!(config.value().len(), 1);
}

// ── Path-list VOs ───────────────────────────────────────────
#[test]
fn file_path_list_and_renamed_file() {
    let mut list = FilePathList::new(vec![common::fp("old.rs")]);
    list.push(common::fp("new.rs"));
    assert_eq!(list.len(), 2);
    let renamed = RenamedFile::new(common::fp("a.rs"), common::fp("b.rs"));
    assert_eq!(renamed.new_path.basename(), "b.rs");
}

// ── Layer / message / suggestion VOs ────────────────────────
#[test]
fn layer_vo_string_wrappers() {
    let content = FileContentVO::new("fn main() {}");
    assert_eq!(content.value(), "fn main() {}");
    let layer = LayerNameVO::new("capabilities");
    assert_eq!(layer.to_string(), "capabilities");
    let line = LineContentVO::from("let x = 1;");
    assert_eq!(line.value(), "let x = 1;");
    assert_eq!(LineContentVO::from(String::from("y")).value(), "y");
}

#[test]
fn lint_message_vo() {
    let msg = LintMessage::new("bad name");
    assert_eq!(msg.value(), "bad name");
    assert_eq!(msg.to_string(), "bad name");
    assert_eq!(LintMessage::from("x").value(), "x");
}

#[test]
fn suggestion_vo_string_wrappers() {
    assert_eq!(DescriptionVO::from("desc").value(), "desc");
    assert_eq!(Suggestion::from("fix").to_string(), "fix");
}

// ── Utilities ───────────────────────────────────────────────
#[test]
fn run_command_success() {
    let (stdout, stderr, ok) = run_command("echo", &["hello"]);
    assert!(ok);
    assert_eq!(stdout.trim(), "hello");
    assert!(stderr.is_empty());
}

#[test]
fn run_command_missing_binary_fails() {
    let (stdout, stderr, ok) = run_command("lint-arwaky-does-not-exist-xyz", &[]);
    assert!(!ok);
    assert!(stdout.is_empty());
    assert!(stderr.contains("Failed to execute"));
}

#[test]
fn run_command_in_dir_missing_dir_fails() {
    let (_, stderr, ok) = run_command_in_dir("echo", &["x"], Some("/nonexistent-dir-xyz"));
    assert!(!ok);
    assert!(stderr.contains("Failed to execute"));
}

#[test]
fn threshold_rejects_values_above_percentage_range() {
    assert!(Threshold::try_new(0).is_ok());
    assert!(Threshold::try_new(100).is_ok());
    assert!(Threshold::try_new(101).is_err());
    assert!(Threshold::try_new(u32::MAX).is_err());
}
