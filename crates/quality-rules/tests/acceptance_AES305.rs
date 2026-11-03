// PURPOSE: Acceptance test AES305 — code duplication detection
// Create two files with >50% overlap, verify violation
use std::path::PathBuf;

use quality_rules_lint_arwaky::capabilities_code_duplication_analyzer::CodeDuplicationAnalyzer;
use shared_config_system::ArchitectureConfig;
use shared_quality_rules::ICodeMetricAnalyzerProtocol;

use std::sync::Arc;

fn analyzer() -> CodeDuplicationAnalyzer {
    CodeDuplicationAnalyzer::from_config(Arc::new(ArchitectureConfig::default()))
}

/// Build an AES305 analyzer with custom `CodeAnalysisRuleVO` fields.
fn analyzer_with_rulevo(
    rulevo: shared_quality_rules::CodeAnalysisRuleVO,
) -> CodeDuplicationAnalyzer {
    let mut config = ArchitectureConfig::default();
    config.rules.push(shared_config_system::ArchitectureRule {
        name: shared_common::taxonomy_message_vo::DescriptionVO::new("AES305".to_string()),
        code_analysis: rulevo,
        ..Default::default()
    });
    CodeDuplicationAnalyzer::from_config(Arc::new(config))
}

#[test]
fn two_identical_files_produces_violation() {
    let ana = analyzer();
    let content = (0..30)
        .map(|i| format!("fn function_{}() {{ let x = {}; return x; }}", i, i))
        .collect::<Vec<_>>()
        .join("\n");

    let entries = vec![
        (PathBuf::from("src/a.rs"), content.clone()),
        (PathBuf::from("src/b.rs"), content),
    ];

    let violations = ana.handle_duplicates_entries(&entries);
    let msg = match &violations.first() {
        Some((
            _,
            shared_quality_rules::AesCodeAnalysisViolation::CodeDuplication { reason: Some(msg) },
        )) => msg.to_string(),
        _ => String::new(),
    };
    assert!(
        msg.contains("(identical)") && msg.contains("threshold: 75%"),
        "no-config default must classify byte-identical files as identical at the 75% default, got: {msg}"
    );
}

#[test]
fn two_different_files_no_violation() {
    let ana = analyzer();
    let entries = vec![
        (
            PathBuf::from("src/a.rs"),
            "fn alpha() { let a = 1; let b = 2; let c = 3; }\n".to_string(),
        ),
        (
            PathBuf::from("src/b.rs"),
            "fn beta() { let x = 10; let y = 20; let z = 30; }\n".to_string(),
        ),
    ];

    let violations = ana.handle_duplicates_entries(&entries);
    assert!(
        violations.is_empty(),
        "No violation expected for completely different files"
    );
}

#[test]
fn single_file_no_violation() {
    let ana = analyzer();
    let entries = vec![(
        PathBuf::from("src/a.rs"),
        "fn only() { return 1; }\n".to_string(),
    )];

    let violations = ana.handle_duplicates_entries(&entries);
    assert!(
        violations.is_empty(),
        "Single file should not produce duplication"
    );
}

#[test]
fn empty_entries_no_violation() {
    let ana = analyzer();
    let violations = ana.handle_duplicates_entries(&[]);
    assert!(violations.is_empty());
}

#[test]
fn files_below_min_lines_no_violation() {
    let ana = analyzer();
    // Short files that have identical content but below min_dup_lines
    let entries = vec![
        (PathBuf::from("src/a.rs"), "line1\nline2\n".to_string()),
        (PathBuf::from("src/b.rs"), "line1\nline2\n".to_string()),
    ];

    let violations = ana.handle_duplicates_entries(&entries);
    assert!(
        violations.is_empty(),
        "Files below min_dup_lines should not be flagged"
    );
}

#[test]
fn repeated_block_counts_each_occurrence_not_one() {
    let ana = analyzer_with_rulevo(shared_quality_rules::CodeAnalysisRuleVO {
        duplication_threshold: Some(0.0),
        ..Default::default()
    });
    // Default min_lines = 10. One 10-line block repeated 3 times = 30 lines.
    // Sliding window of 10 → 21 windows per file, but only 1 distinct window
    // (the block itself repeats identically). Deduped count = 1 → 1/21 ≈ 5%.
    // Correct occurrence count = 21 → 21/21 = 100%.
    let block: Vec<String> = (0..10).map(|i| format!("let var_{i} = {i};")).collect();
    let block_str = block.join("\n");
    let content = format!("{block_str}\n{block_str}\n{block_str}");

    let entries = vec![
        (PathBuf::from("src/repeat_a.rs"), content.clone()),
        (PathBuf::from("src/repeat_b.rs"), content),
    ];

    let violations = ana.handle_duplicates_entries(&entries);
    let msg = match &violations.first() {
        Some((
            _,
            shared_quality_rules::AesCodeAnalysisViolation::CodeDuplication { reason: Some(msg) },
        )) => msg.to_string(),
        _ => String::new(),
    };
    assert!(
        msg.contains("100% of this file's non-import windows"),
        "every repeated occurrence must count toward the similarity percentage, got: {msg}"
    );
}

#[test]
fn violation_message_contains_percentage() {
    let ana = analyzer_with_rulevo(shared_quality_rules::CodeAnalysisRuleVO {
        duplication_threshold: Some(10.0),
        ..Default::default()
    });
    let content = (0..20)
        .map(|i| format!("fn func_{}() {{ let x = {}; }}", i, i))
        .collect::<Vec<_>>()
        .join("\n");

    let entries = vec![
        (PathBuf::from("src/dup_a.rs"), content.clone()),
        (PathBuf::from("src/dup_b.rs"), content),
    ];

    let violations = ana.handle_duplicates_entries(&entries);
    let msg = match &violations.first() {
        Some((
            _,
            shared_quality_rules::AesCodeAnalysisViolation::CodeDuplication { reason: Some(msg) },
        )) => msg.to_string(),
        _ => String::new(),
    };
    assert!(
        msg.contains("AES305"),
        "Violation message should contain AES305, got: {msg}"
    );
}

// ─── Threshold-split tests ──────────────────────────────────────────────────

/// Byte-identical files fall through to "similar" when identical_threshold
/// equals the pct (strict `>` fails) but similar_threshold is below it.
#[test]
fn byte_identical_files_fall_to_similar_when_identical_threshold_equals_pct() {
    let content = (0..20)
        .map(|i| format!("fn shared_{}() {{ let x = {}; return x; }}", i, i))
        .collect::<Vec<_>>()
        .join("\n");

    let entries = vec![
        (PathBuf::from("src/sim_a.rs"), content.clone()),
        (PathBuf::from("src/sim_b.rs"), content),
    ];

    let ana = analyzer_with_rulevo(shared_quality_rules::CodeAnalysisRuleVO {
        identical_threshold: Some(100.0),
        similar_threshold: Some(50.0),
        ..Default::default()
    });
    let violations = ana.handle_duplicates_entries(&entries);
    let msg = match &violations.first() {
        Some((
            _,
            shared_quality_rules::AesCodeAnalysisViolation::CodeDuplication { reason: Some(msg) },
        )) => msg.to_string(),
        _ => String::new(),
    };
    assert!(
        msg.contains("(similar)") && msg.contains("threshold: 50%"),
        "strict > against an equal threshold must push byte-identical files into the similar path, got: {msg}"
    );
}

/// Files that differ only in non-alphanumeric characters: raw windows differ
/// (identical_pct low) while normalized windows match (similar_pct high).
/// This isolates the "similar fires, identical does not" branch.
#[test]
fn similar_fires_when_raw_windows_differ_but_normalized_match() {
    // 10 lines each; same alphanumeric content, but different non-alphanumeric
    // characters (`=` vs `==`). `normalize_line` removes both since `=` is
    // neither alphanumeric nor whitespace, so the normalized windows match
    // while the raw windows do not.
    let lines_a: Vec<String> = (0..10)
        .map(|i| format!("fn shared_{i}() {{ let x = {i}; }}"))
        .collect();
    let lines_b: Vec<String> = (0..10)
        .map(|i| format!("fn shared_{i}() {{ let x == {i}; }}"))
        .collect();

    // Add unique tail lines so the file has more windows than just the shared block.
    let file_a = format!(
        "{}\nunique_a_tail_1\nunique_a_tail_2\nunique_a_tail_3\nunique_a_tail_4",
        lines_a.join("\n")
    );
    let file_b = format!(
        "{}\nunique_b_tail_1\nunique_b_tail_2\nunique_b_tail_3\nunique_b_tail_4",
        lines_b.join("\n")
    );

    let entries = vec![
        (PathBuf::from("src/eq_a.rs"), file_a),
        (PathBuf::from("src/eq_b.rs"), file_b),
    ];

    // identical_threshold high (90%) → identical_pct (low) does not exceed it.
    // similar_threshold low (10%) → similar_pct (high) exceeds it.
    let ana = analyzer_with_rulevo(shared_quality_rules::CodeAnalysisRuleVO {
        identical_threshold: Some(90.0),
        similar_threshold: Some(10.0),
        ..Default::default()
    });
    let violations = ana.handle_duplicates_entries(&entries);
    let msg = match &violations.first() {
        Some((
            _,
            shared_quality_rules::AesCodeAnalysisViolation::CodeDuplication { reason: Some(msg) },
        )) => msg.to_string(),
        _ => String::new(),
    };
    assert!(
        msg.contains("(similar)") && msg.contains("threshold: 10%"),
        "raw windows differ but normalized windows match → similar path must fire, got: {msg}"
    );
}

/// Byte-identical files with a low identical_threshold: the "identical" path fires.
#[test]
fn identical_fires_when_raw_windows_match() {
    let block: Vec<String> = (0..10)
        .map(|i| format!("fn shared_{i}() {{ let z = {i}; return z; }}"))
        .collect();
    let content = format!(
        "{}\nunique_tail_1\nunique_tail_2\nunique_tail_3\nunique_tail_4\nunique_tail_5",
        block.join("\n")
    );

    let entries = vec![
        (PathBuf::from("src/copy_a.rs"), content.clone()),
        (PathBuf::from("src/copy_b.rs"), content),
    ];

    let ana = analyzer_with_rulevo(shared_quality_rules::CodeAnalysisRuleVO {
        identical_threshold: Some(1.0),
        similar_threshold: Some(90.0),
        ..Default::default()
    });
    let violations = ana.handle_duplicates_entries(&entries);
    let msg = match &violations.first() {
        Some((
            _,
            shared_quality_rules::AesCodeAnalysisViolation::CodeDuplication { reason: Some(msg) },
        )) => msg.to_string(),
        _ => String::new(),
    };
    assert!(
        msg.contains("(identical)") && msg.contains("threshold: 1%"),
        "byte-identical windows with a low identical_threshold must fire the identical path, got: {msg}"
    );
}

/// No threshold is exceeded: both pcts are below their respective thresholds.
#[test]
fn neither_classification_fires_when_below_both_thresholds() {
    let block: Vec<String> = (0..10).map(|i| format!("let var_{i} = {i};")).collect();
    let long_tail_a: Vec<String> = (0..40)
        .map(|i| format!("let unique_a_{i} = {i};"))
        .collect();
    let long_tail_b: Vec<String> = (0..40)
        .map(|i| format!("let unique_b_{i} = {i};"))
        .collect();

    let file_a = format!("{}\n{}", block.join("\n"), long_tail_a.join("\n"));
    let file_b = format!("{}\n{}", block.join("\n"), long_tail_b.join("\n"));

    let entries = vec![
        (PathBuf::from("src/partial_a.rs"), file_a),
        (PathBuf::from("src/partial_b.rs"), file_b),
    ];

    let ana = analyzer_with_rulevo(shared_quality_rules::CodeAnalysisRuleVO {
        identical_threshold: Some(100.0),
        similar_threshold: Some(100.0),
        ..Default::default()
    });
    let violations = ana.handle_duplicates_entries(&entries);
    assert!(
        violations.is_empty(),
        "No violation expected when neither threshold is exceeded"
    );
}

/// Backward compat: only `duplication_threshold` is set (no new fields).
/// Both new thresholds must fall back to it, not to the built-in defaults.
#[test]
fn dup_threshold_fallback_used_when_new_fields_unset() {
    let block: Vec<String> = (0..10)
        .map(|i| format!("fn dup_{i}() {{ let w = {i}; return w; }}"))
        .collect();
    let content = block.join("\n");

    let entries = vec![
        (PathBuf::from("src/fb_a.rs"), content.clone()),
        (PathBuf::from("src/fb_b.rs"), content),
    ];

    // duplication_threshold = 99.0 → both thresholds fall back to 99.0.
    let ana = analyzer_with_rulevo(shared_quality_rules::CodeAnalysisRuleVO {
        duplication_threshold: Some(99.0),
        ..Default::default()
    });
    let violations = ana.handle_duplicates_entries(&entries);
    let msg = match &violations.first() {
        Some((
            _,
            shared_quality_rules::AesCodeAnalysisViolation::CodeDuplication { reason: Some(msg) },
        )) => msg.to_string(),
        _ => String::new(),
    };
    assert!(
        msg.contains("threshold: 99%"),
        "duplication_threshold must be the fallback for both new thresholds, got: {msg}"
    );
}

/// K=5 truncation: more than 5 distinct duplicated ranges must show a `(+N more)` suffix.
#[test]
fn k5_truncation_shows_more_suffix() {
    // Construct two files with 7 separate 10-line shared blocks, each separated
    // by 5 unique lines, producing 7 distinct collapsed ranges.
    let make_block = |prefix: &str, i: u32| -> String {
        (0..10)
            .map(|j| format!("{prefix}_{i}_{j}"))
            .collect::<Vec<_>>()
            .join("\n")
    };
    let make_unique = |prefix: &str, i: u32| -> String {
        (0..5)
            .map(|j| format!("unique_{prefix}_{i}_{j}"))
            .collect::<Vec<_>>()
            .join("\n")
    };

    let mut file_a = String::new();
    let mut file_b = String::new();
    for i in 0..7 {
        file_a.push_str(&make_block("block", i));
        file_a.push('\n');
        file_b.push_str(&make_block("block", i));
        file_b.push('\n');
        if i < 6 {
            file_a.push_str(&make_unique("a", i));
            file_a.push('\n');
            file_b.push_str(&make_unique("b", i));
            file_b.push('\n');
        }
    }

    let entries = vec![
        (PathBuf::from("src/many_a.rs"), file_a),
        (PathBuf::from("src/many_b.rs"), file_b),
    ];

    let ana = analyzer_with_rulevo(shared_quality_rules::CodeAnalysisRuleVO {
        identical_threshold: Some(1.0),
        similar_threshold: Some(1.0),
        ..Default::default()
    });
    let violations = ana.handle_duplicates_entries(&entries);
    let msg = match &violations.first() {
        Some((
            _,
            shared_quality_rules::AesCodeAnalysisViolation::CodeDuplication { reason: Some(msg) },
        )) => msg.to_string(),
        _ => String::new(),
    };
    let tail = msg
        .split_once(" Blocks: ")
        .map(|(_, b)| b.to_string())
        .unwrap_or_default();
    // 5 blocks + the (+2 more) suffix each contain "duplicated in" — a regression
    // that prints all 7 ranges and appends the suffix would still pass the old
    // assertion, so pin the exact count.
    assert_eq!(
        tail.matches("duplicated in").count(),
        5,
        "K=5 truncation must report exactly 5 duplicated ranges before the (+2 more) suffix, got: {msg}"
    );
    // The first 5 range start-values are present; the 6th/7th (start values 76, 91) are truncated.
    for start in [1, 16, 31, 46, 61] {
        assert!(
            tail.contains(&format!("lines {start}-")),
            "first-5 range start {start} must be present, got: {msg}"
        );
    }
    assert!(
        !tail.contains("lines 76-") && !tail.contains("lines 91-"),
        "truncated ranges (starts 76/91) must not appear, got: {msg}"
    );
    // Issue 17: the first collapsed block must list a concrete
    // `src/many_b.rs:lines A-B` cross-file segment (the other-file overlap
    // lookup fix, Round-1 Issue 1, has observable output).
    // Issue 23: pin the exact window range for block 0 (lines 1–10, the
    // min_lines=5 window spanning the full 10-line shared block), not just
    // the start line.
    assert!(
        tail.contains("src/many_b.rs:lines 1-10"),
        "first block must report the exact cross-file range src/many_b.rs:lines 1-10, got: {msg}"
    );
    // Issue 23: pin a non-first block. Block 1 spans lines 16–25 of the
    // other file; the old `v.first()` behavior would report 1–10 there, so
    // this assertion only passes when the overlap lookup is used.
    assert!(
        tail.contains("src/many_b.rs:lines 16-25"),
        "second block must report its own cross-file range src/many_b.rs:lines 16-25, got: {msg}"
    );
}

/// Exactly K=5 duplicated ranges: all 5 reported with NO `(+N more)` truncation suffix.
#[test]
fn exactly_k5_ranges_no_truncation_suffix() {
    // Construct two files with exactly 5 separate 10-line shared blocks, each
    // separated by 5 unique lines, producing exactly 5 distinct collapsed ranges.
    // With TOP_K = 5, no truncation suffix must appear.
    let make_block = |prefix: &str, i: u32| -> String {
        (0..10)
            .map(|j| format!("{prefix}_{i}_{j}"))
            .collect::<Vec<_>>()
            .join("\n")
    };
    let make_unique = |prefix: &str, i: u32| -> String {
        (0..5)
            .map(|j| format!("unique_{prefix}_{i}_{j}"))
            .collect::<Vec<_>>()
            .join("\n")
    };

    let mut file_a = String::new();
    let mut file_b = String::new();
    for i in 0..5 {
        file_a.push_str(&make_block("block", i));
        file_a.push('\n');
        file_b.push_str(&make_block("block", i));
        file_b.push('\n');
        if i < 4 {
            file_a.push_str(&make_unique("a", i));
            file_a.push('\n');
            file_b.push_str(&make_unique("b", i));
            file_b.push('\n');
        }
    }

    let entries = vec![
        (PathBuf::from("src/exact_a.rs"), file_a),
        (PathBuf::from("src/exact_b.rs"), file_b),
    ];

    let ana = analyzer_with_rulevo(shared_quality_rules::CodeAnalysisRuleVO {
        identical_threshold: Some(1.0),
        similar_threshold: Some(1.0),
        ..Default::default()
    });
    let violations = ana.handle_duplicates_entries(&entries);
    let msg = match &violations.first() {
        Some((
            _,
            shared_quality_rules::AesCodeAnalysisViolation::CodeDuplication { reason: Some(msg) },
        )) => msg.to_string(),
        _ => String::new(),
    };
    let tail = msg
        .split_once(" Blocks: ")
        .map(|(_, b)| b.to_string())
        .unwrap_or_default();

    // All 5 ranges must be present.
    for start in [1, 16, 31, 46, 61] {
        assert!(
            tail.contains(&format!("lines {start}-")),
            "range starting at {start} must be present, got: {msg}"
        );
    }
    // Each block reports its own exact cross-file range in the other file.
    for (os, oe) in [(1, 10), (16, 25), (31, 40), (46, 55), (61, 70)] {
        assert!(
            tail.contains(&format!("src/exact_b.rs:lines {os}-{oe}")),
            "block must report cross-file range src/exact_b.rs:lines {os}-{oe}, got: {msg}"
        );
    }
    // Exactly 5 "duplicated in" occurrences (no hidden 6th range).
    assert_eq!(
        tail.matches("duplicated in").count(),
        5,
        "exactly 5 ranges must be reported, got: {msg}"
    );
    // No truncation suffix must appear for exactly K=5 ranges.
    assert!(
        !tail.contains("(+") && !tail.contains("more)"),
        "no (+N more) truncation suffix must appear for exactly K=5 ranges, got: {msg}"
    );
}

/// Import-only windows must not count toward the shared-window percentage.
/// Two files that differ only in import lines (rest identical) should show
/// a lower pct than files with no import lines.
#[test]
fn import_windows_excluded_from_shared_count() {
    // 20 lines: 10 import lines + 10 logic lines. Both files have the same
    // import lines and the same logic lines, but the import lines are excluded
    // from the percentage calculation.
    let imports: Vec<String> = (0..10)
        .map(|i| format!("use std::io::{{Read, Write}}; // import_{i}"))
        .collect();
    let logic: Vec<String> = (0..10)
        .map(|i| format!("fn logic_{i}() {{ let x = {i}; return x; }}"))
        .collect();

    let content_a = format!("{}\n{}", imports.join("\n"), logic.join("\n"));
    let content_b = format!("{}\n{}", imports.join("\n"), logic.join("\n"));

    let entries = vec![
        (PathBuf::from("src/imp_a.rs"), content_a),
        (PathBuf::from("src/imp_b.rs"), content_b),
    ];

    let ana = analyzer_with_rulevo(shared_quality_rules::CodeAnalysisRuleVO {
        identical_threshold: Some(0.0),
        similar_threshold: Some(0.0),
        ..Default::default()
    });
    let violations = ana.handle_duplicates_entries(&entries);
    let msg = match &violations.first() {
        Some((
            _,
            shared_quality_rules::AesCodeAnalysisViolation::CodeDuplication { reason: Some(msg) },
        )) => msg.to_string(),
        _ => String::new(),
    };
    // With import lines excluded, only logic windows count. The 10 logic lines
    // produce windows that are shared, but import windows are excluded from both
    // numerator and denominator. Assert the message references "non-import windows".
    assert!(
        msg.contains("non-import windows"),
        "message must reference non-import windows, got: {msg}"
    );
    // The pct should be based on logic windows only (not the full 20 lines).
    // 20 lines, min_lines default = 5 → 16 total windows. The 6 all-import
    // windows (lines 1–6) are excluded; the 10 remaining windows are shared.
    assert!(
        msg.contains("10 of 10 windows are non-unique"),
        "denominator must count only non-import windows (10 logic windows), got: {msg}"
    );
}
