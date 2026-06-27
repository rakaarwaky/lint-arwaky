use code_analysis_lint_arwaky::capabilities_code_duplication_analyzer::CodeDuplicationAnalyzer;
use std::io::Write;

fn tempdir() -> std::path::PathBuf {
    let p = std::env::temp_dir().join(format!(
        "code_dup_test_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir_all(&p).unwrap();
    p
}

#[test]
fn reported_line_numbers_are_1_indexed_not_byte_offsets() {
    let dir = tempdir();
    let file_path = dir.join("a.rs");
    let mut f = std::fs::File::create(&file_path).unwrap();
    for i in 0..5 {
        writeln!(f, "filler line {i}").unwrap();
    }
    writeln!(f, "dup alpha").unwrap();
    writeln!(f, "dup beta").unwrap();
    writeln!(f, "dup gamma").unwrap();
    for i in 0..5 {
        writeln!(f, "more filler {i}").unwrap();
    }
    writeln!(f, "dup alpha").unwrap();
    writeln!(f, "dup beta").unwrap();
    writeln!(f, "dup gamma").unwrap();

    let analyzer = CodeDuplicationAnalyzer::new();
    let violations = analyzer.check_duplicates(&[file_path.to_string_lossy().to_string()], 3);

    let dup_msg = violations
        .iter()
        .map(|v| match v {
            shared::code_analysis::taxonomy_violation_code_analysis_vo::AesCodeAnalysisViolation::CodeDuplication { reason } => {
                reason.as_ref().map(|r| r.value.clone()).unwrap_or_default()
            }
            _ => String::new(),
        })
        .find(|m| m.contains("Duplicate block"))
        .expect("expected a CodeDuplication violation");

    assert!(
        dup_msg.contains(":6"),
        "expected line 6 in violation, got: {dup_msg}"
    );
    assert!(
        dup_msg.contains(":14"),
        "expected line 14 in violation, got: {dup_msg}"
    );
}

#[test]
fn no_duplicates_emits_no_block_violation() {
    let dir = tempdir();
    let file_path = dir.join("unique.rs");
    let mut f = std::fs::File::create(&file_path).unwrap();
    for i in 0..30 {
        writeln!(f, "fn unique_{i}() {{ println!({i}); }}").unwrap();
    }
    let violations = CodeDuplicationAnalyzer::new()
        .check_duplicates(&[file_path.to_string_lossy().to_string()], 5);
    assert!(
        violations.is_empty(),
        "expected no violations for unique content"
    );
}

#[test]
fn two_files_sharing_block_reports_both_locations() {
    let dir = tempdir();
    let a = dir.join("a.rs");
    let b = dir.join("b.rs");
    for (path, prefix) in [(&a, "alpha"), (&b, "beta")] {
        let mut f = std::fs::File::create(path).unwrap();
        for i in 0..20 {
            writeln!(f, "{prefix}_line_{i}").unwrap();
        }
        writeln!(f, "shared one").unwrap();
        writeln!(f, "shared two").unwrap();
        writeln!(f, "shared three").unwrap();
        writeln!(f, "shared four").unwrap();
    }
    let violations = CodeDuplicationAnalyzer::new().check_duplicates(
        &[
            a.to_string_lossy().to_string(),
            b.to_string_lossy().to_string(),
        ],
        4,
    );
    let dup = violations
        .iter()
        .map(|v| match v {
            shared::code_analysis::taxonomy_violation_code_analysis_vo::AesCodeAnalysisViolation::CodeDuplication { reason } => {
                reason.as_ref().map(|r| r.value.clone()).unwrap_or_default()
            }
            _ => String::new(),
        })
        .find(|m| m.contains("Duplicate block"))
        .unwrap();
    assert!(dup.contains("a.rs:21"), "missing a.rs line in: {dup}");
    assert!(dup.contains("b.rs:21"), "missing b.rs line in: {dup}");
}
