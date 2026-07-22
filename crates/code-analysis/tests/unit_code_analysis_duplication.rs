// PURPOSE: Unit tests for CodeDuplicationAnalyzer (AES305) — file-level
// similarity detection using window-based hashing.

use code_analysis_lint_arwaky::CodeDuplicationAnalyzer;
use shared::code_analysis::contract_code_metric_analyzer_protocol::ICodeMetricAnalyzerProtocol;
use shared::code_analysis::taxonomy_violation_code_analysis_vo::AesCodeAnalysisViolation;

fn analyzer() -> CodeDuplicationAnalyzer {
    CodeDuplicationAnalyzer::new()
}

fn make_entries(files: Vec<(&str, &str)>) -> Vec<(String, String)> {
    files
        .into_iter()
        .map(|(path, content)| (path.to_string(), content.to_string()))
        .collect()
}

// ─── Happy Path: No duplication ──────────────────────────────────────

#[test]
fn no_duplication_no_violations() {
    let entries = make_entries(vec![
        (
            "file_a.rs",
            "fn alpha() -> i32 {\n    1 + 2 + 3\n}\n\nfn beta() -> String {\n    String::from(\"hello\")\n}\n\nfn gamma() -> bool {\n    true\n}\n\nfn delta() -> f64 {\n    3.14\n}\n\nfn epsilon() -> u8 {\n    255\n}",
        ),
        (
            "file_b.rs",
            "struct Widget {\n    name: String,\n    size: usize,\n}\n\nimpl Widget {\n    fn new() -> Self {\n        Self { name: String::new(), size: 0 }\n    }\n}",
        ),
    ]);
    let violations = analyzer().check_file_similarity_entries(&entries, 5, 50.0);
    assert!(violations.is_empty());
}

// ─── AES305: High duplication detected ───────────────────────────────

#[test]
fn high_duplication_detected() {
    let shared_block = "fn shared_one() -> i32 {\n    42\n}\n\nfn shared_two() -> String {\n    String::from(\"dup\")\n}\n\nfn shared_three() -> bool {\n    true\n}\n\nfn shared_four() -> f64 {\n    2.71\n}\n\nfn shared_five() -> u8 {\n    128\n}";
    let entries = make_entries(vec![
        ("file_a.rs", shared_block),
        ("file_b.rs", shared_block),
    ]);
    let violations = analyzer().check_file_similarity_entries(&entries, 5, 50.0);
    assert!(!violations.is_empty());
    // Both files should be flagged
    assert_eq!(violations.len(), 2);
    for (path, violation) in &violations {
        assert!(path.contains("file_"));
        let msg = violation.to_string();
        assert!(msg.contains("AES305"));
    }
}

// ─── Edge Case: Empty entries ────────────────────────────────────────

#[test]
fn empty_entries_no_violations() {
    let entries: Vec<(String, String)> = Vec::new();
    let violations = analyzer().check_file_similarity_entries(&entries, 5, 50.0);
    assert!(violations.is_empty());
}

// ─── Edge Case: Single file cannot duplicate ─────────────────────────

#[test]
fn single_file_no_duplication() {
    let entries = make_entries(vec![(
        "file_a.rs",
        "fn a() {}\nfn b() {}\nfn c() {}\nfn d() {}\nfn e() {}\nfn f() {}",
    )]);
    let violations = analyzer().check_file_similarity_entries(&entries, 5, 50.0);
    assert!(violations.is_empty());
}

// ─── Edge Case: Files shorter than min_dup_lines ─────────────────────

#[test]
fn short_files_skipped() {
    let entries = make_entries(vec![
        ("file_a.rs", "line1\nline2\nline3"),
        ("file_b.rs", "line1\nline2\nline3"),
    ]);
    let violations = analyzer().check_file_similarity_entries(&entries, 5, 50.0);
    assert!(violations.is_empty());
}

// ─── Edge Case: Below threshold not flagged ──────────────────────────

#[test]
fn below_threshold_not_flagged() {
    // 20 lines total, only 5 shared → 25% < 50% threshold
    let mut content_a = String::new();
    for i in 0..15 {
        content_a.push_str(&format!("fn unique_a_{}() -> i32 {{ {} }}\n", i, i));
    }
    content_a.push_str("fn shared_1() {}\nfn shared_2() {}\nfn shared_3() {}\nfn shared_4() {}\nfn shared_5() {}\n");

    let mut content_b = String::new();
    for i in 0..15 {
        content_b.push_str(&format!("fn unique_b_{}() -> i32 {{ {} }}\n", i, i * 2));
    }
    content_b.push_str("fn shared_1() {}\nfn shared_2() {}\nfn shared_3() {}\nfn shared_4() {}\nfn shared_5() {}\n");

    let entries = make_entries(vec![("file_a.rs", &content_a), ("file_b.rs", &content_b)]);
    let violations = analyzer().check_file_similarity_entries(&entries, 5, 50.0);
    assert!(violations.is_empty());
}

// ─── Protocol trait: handle_duplicates ───────────────────────────────

#[test]
fn handle_duplicates_with_none_path_uses_cwd() {
    // This will scan the current directory — just verify it doesn't panic.
    let result = analyzer().handle_duplicates(None);
    // Result may be empty or non-empty depending on cwd content.
    let _ = result;
}

// ─── Legacy API: check_duplicates ────────────────────────────────────

#[test]
fn legacy_check_duplicates_empty_files() {
    let violations = analyzer().check_duplicates(&[], 5);
    assert!(violations.is_empty());
}
