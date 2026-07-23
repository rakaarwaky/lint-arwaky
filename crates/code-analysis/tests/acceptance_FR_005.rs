// PURPOSE: Acceptance test for FR-005: Duplicate Code Detection (AES305)
// Compares code blocks and flags identical/highly similar segments.

use code_analysis_lint_arwaky::CodeDuplicationAnalyzer;

fn analyzer() -> CodeDuplicationAnalyzer {
    CodeDuplicationAnalyzer::new()
}

fn make_entries(files: Vec<(&str, &str)>) -> Vec<(String, String)> {
    files
        .into_iter()
        .map(|(p, c)| (p.to_string(), c.to_string()))
        .collect()
}

/// FR-005: Duplicate code detected with AES305
#[test]
fn fr_005_duplicate_code_detected() {
    // Create two files with >50% identical content (min 5 lines window)
    let shared = "fn shared_a() -> i32 {\n    1 + 1\n}\n\nfn shared_b() -> i32 {\n    2 + 2\n}\n\nfn shared_c() -> i32 {\n    3 + 3\n}\n\nfn shared_d() -> i32 {\n    4 + 4\n}\n\nfn shared_e() -> i32 {\n    5 + 5\n}";
    let entries = make_entries(vec![("file_a.rs", shared), ("file_b.rs", shared)]);

    let violations = analyzer().check_file_similarity_entries(&entries, 5, 50.0);
    assert!(!violations.is_empty());

    // Verify violation message contains AES305
    let (_, violation) = &violations[0];
    let msg = violation.to_string();
    assert!(msg.contains("AES305"));
    assert!(msg.contains("CODE_DUPLICATION"));
}

/// FR-005: Min duplicate lines threshold respected
#[test]
fn fr_005_min_duplicate_lines_respected() {
    // Only 3 shared lines with min_dup_lines=5 → no violation
    let entries = make_entries(vec![
        ("file_a.rs", "fn a() {}\nfn b() {}\nfn c() {}\nfn unique_a() {}\nfn unique_a2() {}\nfn unique_a3() {}\nfn unique_a4() {}\nfn unique_a5() {}"),
        ("file_b.rs", "fn a() {}\nfn b() {}\nfn c() {}\nfn unique_b() {}\nfn unique_b2() {}\nfn unique_b3() {}\nfn unique_b4() {}\nfn unique_b5() {}"),
    ]);

    let violations = analyzer().check_file_similarity_entries(&entries, 5, 50.0);
    assert!(violations.is_empty());
}

/// FR-005: Threshold percentage respected
#[test]
fn fr_005_threshold_percentage_respected() {
    // Files share some content but below 50% threshold
    let mut content_a = String::new();
    for i in 0..20 {
        content_a.push_str(&format!("fn unique_a_{}() -> i32 {{ {} }}\n", i, i));
    }

    let mut content_b = String::new();
    for i in 0..20 {
        content_b.push_str(&format!(
            "fn unique_b_{}() -> String {{ String::from(\"{}\") }}\n",
            i, i
        ));
    }

    let entries = make_entries(vec![("file_a.rs", &content_a), ("file_b.rs", &content_b)]);
    let violations = analyzer().check_file_similarity_entries(&entries, 5, 50.0);
    assert!(violations.is_empty());
}

/// FR-005: Algorithm uses window-based hashing with normalized lines
#[test]
fn fr_005_normalized_comparison_ignores_whitespace() {
    // Same logic, different whitespace → should still match after normalization
    let content_a = "fn foo() -> i32 {\n    42\n}\n\nfn bar() -> i32 {\n    43\n}\n\nfn baz() -> i32 {\n    44\n}\n\nfn qux() -> i32 {\n    45\n}\n\nfn quux() -> i32 {\n    46\n}";
    let content_b = "fn foo() -> i32 {\n        42\n}\n\nfn bar() -> i32 {\n        43\n}\n\nfn baz() -> i32 {\n        44\n}\n\nfn qux() -> i32 {\n        45\n}\n\nfn quux() -> i32 {\n        46\n}";

    let entries = make_entries(vec![("file_a.rs", content_a), ("file_b.rs", content_b)]);
    let violations = analyzer().check_file_similarity_entries(&entries, 5, 50.0);
    // After normalization (trim + alphanumeric), these should be identical
    assert!(!violations.is_empty());
}
