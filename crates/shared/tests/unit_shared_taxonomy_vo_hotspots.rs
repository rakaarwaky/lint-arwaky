// Unit tests — shared taxonomy VOs flagged as untested hotspots by repowise.
//
// Targets (per plan item 1.4):
//   taxonomy_lint_result_vo, taxonomy_layer_vo, taxonomy_message_vo,
//   taxonomy_adapter_name_vo, taxonomy_suggestion_vo, and the filesystem VO
//   surface exercised by `unit_shared_filesystem_vo.rs`.
mod common;

use shared_lint_arwaky::common::taxonomy_adapter_name_vo::AdapterName;
use shared_lint_arwaky::common::taxonomy_layer_vo::{
    FileContentVO, Identity, LayerNameVO, LineContentVO,
};
use shared_lint_arwaky::common::taxonomy_lint_result_vo::{LintResult, LintResultList};
use shared_lint_arwaky::common::taxonomy_message_vo::{ComplianceStatus, LintMessage};
use shared_lint_arwaky::common::taxonomy_severity_vo::Severity;
use shared_lint_arwaky::common::taxonomy_suggestion_vo::{
    ClassPath, DescriptionVO, LogOutput, MetadataVO, StdError, StdOutput, Suggestion,
};

// ── LintResult ──────────────────────────────────────────────
#[test]
fn lint_result_new_arch_populates_identity_fields() {
    let r = LintResult::new_arch("src/foo.rs", 12, "AES101", Severity::default(), "bad name");
    assert_eq!(r.file.value(), "src/foo.rs");
    assert_eq!(r.line, shared_lint_arwaky::common::taxonomy_common_vo::LineNumber::new(12));
    assert_eq!(r.code, shared_lint_arwaky::common::taxonomy_error_vo::ErrorCode::raw("AES101"));
    assert_eq!(r.message.value(), "bad name");
    assert_eq!(r.source.as_deref(), Some("architecture"));
    assert!(r.enclosing_scope.is_some());
}

#[test]
fn lint_result_new_orphan_has_no_enclosing_scope() {
    let r = LintResult::new_orphan("src/orphan.rs", "no importers", Severity::default(), "AES501");
    assert!(r.enclosing_scope.is_none());
    assert_eq!(r.file.value(), "src/orphan.rs");
    assert_eq!(r.message.value(), "no importers");
}

#[test]
fn lint_result_identity_roundtrip_is_stable() {
    let a = LintResult::new_arch("a.rs", 1, "AES1", Severity::default(), "x");
    let b = LintResult::new_arch("a.rs", 1, "AES1", Severity::default(), "x");
    assert_eq!(a.identity(), b.identity());

    let c = LintResult::new_arch("a.rs", 2, "AES1", Severity::default(), "x");
    assert_ne!(a.identity(), c.identity());
}

// ── LintResultList ──────────────────────────────────────────
#[test]
fn lint_result_list_wrapper_behaves() {
    let mut list = LintResultList::new(Vec::new());
    assert!(list.is_empty());

    list.push(LintResult::new_arch("a.rs", 1, "AES1", Severity::default(), "x"));
    list.append(LintResult::new_arch("b.rs", 2, "AES2", Severity::default(), "y"));
    assert_eq!(list.len(), 2);
    assert!(!list.is_empty());
    assert_eq!(list.iter().count(), 2);
}

// ── layer VOs ───────────────────────────────────────────────
#[test]
fn layer_vos_store_and_display() {
    let fc = FileContentVO::new("fn main() {}");
    assert_eq!(fc.value(), "fn main() {}");
    assert_eq!(fc.to_string(), "fn main() {}");

    let id = Identity::new("taxonomy");
    assert_eq!(id.value(), "taxonomy");
    assert_eq!(Identity::from("x"), Identity::new("x"));

    let name = LayerNameVO::new("capabilities");
    assert_eq!(name.value(), "capabilities");

    let line = LineContentVO::new("    let x = 1;");
    assert_eq!(line.value(), "    let x = 1;");
}

#[test]
fn layer_vos_compare_by_value() {
    assert_eq!(LayerNameVO::new("a"), LayerNameVO::new("a"));
    assert_ne!(LayerNameVO::new("a"), LayerNameVO::new("b"));
}

// ── message VOs ─────────────────────────────────────────────
#[test]
fn lint_message_stores_text() {
    let m = LintMessage::new("hello world");
    assert_eq!(m.value(), "hello world");
    assert_eq!(m.to_string(), "hello world");
    assert_eq!(LintMessage::from("x"), LintMessage::new("x"));
}

#[test]
fn compliance_status_wraps_bool() {
    let pass = ComplianceStatus::new(true);
    assert!(pass.value());
    assert_eq!(pass.to_string(), "true");
    assert_eq!(ComplianceStatus::from(false), ComplianceStatus::new(false));
}

// ── AdapterName ─────────────────────────────────────────────
#[test]
fn adapter_name_validates_non_empty() {
    assert!(AdapterName::new("").is_err());
    assert!(AdapterName::new("   ").is_err());
    let a = AdapterName::new("  clippy  ").expect("trimmed name");
    assert_eq!(a.value(), "clippy");
    assert_eq!(a.to_string(), "clippy");
    assert_eq!(&*a, "clippy");
}

#[test]
fn adapter_name_raw_skips_validation() {
    let a = AdapterName::raw("static-name");
    assert_eq!(a.value(), "static-name");
}

// ── suggestion VOs ──────────────────────────────────────────
#[test]
fn suggestion_strings_wrap_and_display() {
    assert_eq!(DescriptionVO::new("fix it").value(), "fix it");
    assert_eq!(Suggestion::from("s").to_string(), "s");
    assert_eq!(ClassPath::new("a::b").value(), "a::b");
    assert_eq!(LogOutput::new("out").value(), "out");
    assert_eq!(StdError::new("err").value(), "err");
    assert_eq!(StdOutput::new("out").value(), "out");
}

#[test]
fn metadata_vo_wraps_map() {
    let mut map = std::collections::HashMap::new();
    map.insert("k".to_string(), serde_json::json!("v"));
    let m = MetadataVO::new(map.clone());
    assert_eq!(m.value().get("k").and_then(|v| v.as_str()), Some("v"));
}
