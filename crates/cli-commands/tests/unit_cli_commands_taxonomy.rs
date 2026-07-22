//! Unit tests for taxonomy value objects consumed by cli-commands.

use shared::cli_commands::taxonomy_format_vo::Format;
use shared::cli_commands::taxonomy_scan_request_vo::{ScanMode, ScanRequest, ScanTarget};
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::common::taxonomy_path_vo::{DirectoryPath, FilePath};
use shared::common::taxonomy_threshold_vo::Threshold;
use std::str::FromStr;

// ─── Format VO ───────────────────────────────────────────────────────────────

#[test]
fn format_from_str_valid() {
    assert_eq!(Format::from_str("text").unwrap(), Format::Text);
    assert_eq!(Format::from_str("json").unwrap(), Format::Json);
    assert_eq!(Format::from_str("sarif").unwrap(), Format::Sarif);
    assert_eq!(Format::from_str("junit").unwrap(), Format::Junit);
}

#[test]
fn format_from_str_case_insensitive() {
    assert_eq!(Format::from_str("TEXT").unwrap(), Format::Text);
    assert_eq!(Format::from_str("JSON").unwrap(), Format::Json);
    assert_eq!(Format::from_str("Sarif").unwrap(), Format::Sarif);
}

#[test]
fn format_from_str_invalid() {
    assert!(Format::from_str("xml").is_err());
    assert!(Format::from_str("").is_err());
}

#[test]
fn format_display() {
    assert_eq!(Format::Text.to_string(), "text");
    assert_eq!(Format::Json.to_string(), "json");
    assert_eq!(Format::Sarif.to_string(), "sarif");
    assert_eq!(Format::Junit.to_string(), "junit");
}

#[test]
fn format_default_is_text() {
    assert_eq!(Format::default(), Format::Text);
}

// ─── Severity VO ─────────────────────────────────────────────────────────────

#[test]
fn severity_score_impact_ordering() {
    assert!(Severity::CRITICAL.score_impact() > Severity::HIGH.score_impact());
    assert!(Severity::HIGH.score_impact() > Severity::MEDIUM.score_impact());
    assert!(Severity::MEDIUM.score_impact() > Severity::LOW.score_impact());
    assert!(Severity::LOW.score_impact() > Severity::INFO.score_impact());
    assert_eq!(Severity::INFO.score_impact(), 0.0);
}

#[test]
fn severity_display() {
    assert_eq!(Severity::CRITICAL.to_string(), "critical");
    assert_eq!(Severity::HIGH.to_string(), "high");
    assert_eq!(Severity::MEDIUM.to_string(), "medium");
    assert_eq!(Severity::LOW.to_string(), "low");
    assert_eq!(Severity::INFO.to_string(), "info");
}

// ─── Threshold VO ────────────────────────────────────────────────────────────

#[test]
fn threshold_default_is_100() {
    assert_eq!(Threshold::default().value(), 100);
}

#[test]
fn threshold_from_u32() {
    let t: Threshold = 80u32.into();
    assert_eq!(t.value(), 80);
}

// ─── FilePath VO ─────────────────────────────────────────────────────────────

#[test]
fn filepath_new_normalizes_separators() {
    let fp = FilePath::new("src\\main\\lib.rs").unwrap();
    assert_eq!(fp.value, "src/main/lib.rs");
}

#[test]
fn filepath_new_rejects_empty() {
    assert!(FilePath::new("").is_err());
    assert!(FilePath::new("   ").is_err());
}

#[test]
fn filepath_extension() {
    let fp = FilePath::new("src/main.rs").unwrap();
    assert_eq!(fp.extension(), "rs");
}

#[test]
fn filepath_is_entry_point() {
    assert!(FilePath::new("src/lib.rs").unwrap().is_entry_point());
    assert!(FilePath::new("src/main.rs").unwrap().is_entry_point());
    assert!(!FilePath::new("src/helper.rs").unwrap().is_entry_point());
}

#[test]
fn filepath_is_barrel_file() {
    assert!(FilePath::new("src/mod.rs").unwrap().is_barrel_file());
    assert!(FilePath::new("src/index.ts").unwrap().is_barrel_file());
    assert!(!FilePath::new("src/main.rs").unwrap().is_barrel_file());
}

// ─── ScanRequest VO ──────────────────────────────────────────────────────────

#[test]
fn scan_request_default_target_is_dot() {
    let target = ScanTarget::default();
    assert_eq!(target.value, ".");
}

#[test]
fn scan_request_new_with_defaults() {
    let req = ScanRequest::new(ScanTarget::new("./src".into()), ScanMode::Check);
    assert_eq!(req.target.value, "./src");
    assert!(req.filter.is_none());
    assert!(req.member.is_none());
    assert_eq!(req.format, Format::Text);
}

// ─── DirectoryPath VO ────────────────────────────────────────────────────────

#[test]
fn directory_path_new_normalizes() {
    let dp = DirectoryPath::new("src\\lib\\").unwrap();
    assert_eq!(dp.value, "src/lib");
}

#[test]
fn directory_path_new_rejects_empty() {
    assert!(DirectoryPath::new("").is_err());
}
