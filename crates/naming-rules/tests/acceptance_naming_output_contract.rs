// Acceptance tests — the output contract of the naming findings.
//
// Every naming finding carries three texts, and this file pins what a reader
// gets: three texts that do not restate one another, one clause per line, a
// line inside the width budget, ASCII only, and one named problem rather than
// a list of guesses.
use naming_rules_lint_arwaky::capabilities_naming_convention_checker::NamingConventionChecker;
use naming_rules_lint_arwaky::capabilities_suffix_policy_checker::SuffixPolicyChecker;
use naming_rules_lint_arwaky::capabilities_test_file_prefix_checker::TestFilePrefixChecker;
use shared_common::PatternList;
use shared_common::SuffixPolicyVO;
use shared_common::taxonomy_layer_vo::LayerNameVO;
use shared_common::taxonomy_layer_vo::{LayerDefinition, LayerMapVO};
use shared_common::taxonomy_severity_vo::Severity;
use shared_common::taxonomy_violation_item_vo::ViolationItem;
use shared_naming_rules::NAMING_LINE_WIDTH;
use shared_naming_rules::SUFFIX_POLICY_STRICT;
use shared_naming_rules::taxonomy_naming_audit_stats_vo::NamingAuditStats;
use shared_naming_rules::utility_naming_output::render_naming_summary;
use std::collections::HashMap;

/// The layer map the AES102 checks run against.
fn layer_map() -> LayerMapVO {
    let mut def = LayerDefinition::default();
    def.naming.suffix_policy = SuffixPolicyVO::new(SUFFIX_POLICY_STRICT.to_string());
    def.naming.allowed_suffix =
        PatternList::new(vec!["checker".to_string(), "adapter".to_string()]);
    def.naming.forbidden_suffix = PatternList::new(vec!["vo".to_string()]);

    let mut layers = HashMap::new();
    layers.insert(LayerNameVO::new("capabilities"), def);
    LayerMapVO::new(layers)
}

/// The AES101 finding for a stem in the given folder.
fn aes101(file: &str, filename: &str, folder: &str) -> ViolationItem {
    let checker = NamingConventionChecker::new();
    let layer = Some(LayerNameVO::new(folder));
    checker
        .check_file_naming_internal(file, filename, &layer, None, 3)
        .expect("the stem must break a rule")
}

/// The AES101 finding for a stem that names no known folder.
fn aes101_without_prefix(file: &str, filename: &str) -> ViolationItem {
    let checker = NamingConventionChecker::new();
    checker
        .check_file_naming_internal(file, filename, &None, None, 3)
        .expect("a stem without a folder prefix must break a rule")
}

/// The AES102 finding for a stem in the capabilities folder.
fn aes102(file: &str, filename: &str) -> ViolationItem {
    let map = layer_map();
    let suffix_map = SuffixPolicyChecker::build_suffix_to_layer_map(&map);
    let def = map.values.get(&LayerNameVO::new("capabilities"));
    let layer = Some(LayerNameVO::new("capabilities"));
    let checker = SuffixPolicyChecker::new();
    checker
        .check_domain_suffixes_internal(file, filename, def, &layer, &suffix_map)
        .expect("the suffix must break a rule")
}

/// The AES103 finding for a file in `tests/` or `benches/`.
fn aes103(file: &str, filename: &str) -> ViolationItem {
    let checker = TestFilePrefixChecker::new();
    checker
        .check_test_file_prefix_internal(file, filename)
        .expect("the file must break a rule")
}

/// The three texts one finding carries.
fn texts_of(finding: &ViolationItem) -> (String, String, String) {
    (
        finding.message.value().to_string(),
        finding.why.clone(),
        finding.fix.clone(),
    )
}

/// The severity level of one finding, as the summary names it.
fn level_of(finding: &ViolationItem) -> &'static str {
    match &finding.severity {
        Severity::CRITICAL => "critical",
        Severity::HIGH => "high",
        Severity::MEDIUM => "medium",
        Severity::LOW => "low",
        Severity::INFO => "info",
    }
}

/// One finding of every kind the naming rules raise.
fn sample_findings() -> Vec<ViolationItem> {
    vec![
        aes101_without_prefix("src/badname.rs", "badname.rs"),
        aes101(
            "src/capabilities_UserChecker.rs",
            "capabilities_UserChecker.rs",
            "capabilities",
        ),
        aes101(
            "src/capabilities-user-checker.rs",
            "capabilities-user-checker.rs",
            "capabilities",
        ),
        aes101("src/taxonomy_user.rs", "taxonomy_user.rs", "taxonomy"),
        aes102("src/capabilities_user_vo.rs", "capabilities_user_vo.rs"),
        aes102(
            "src/capabilities_user_handler.rs",
            "capabilities_user_handler.rs",
        ),
        aes103("crates/calc/tests/helpers.rs", "helpers.rs"),
        aes103("crates/calc/tests/inner/unit_adder.rs", "unit_adder.rs"),
        aes103("crates/calc/benches/inner/bench_adder.rs", "bench_adder.rs"),
    ]
}

// ─── The three fields ──────────────────────────────────────────────────────

#[test]
fn every_finding_carries_three_texts_that_do_not_restate_one_another() {
    for finding in sample_findings() {
        let (message, why, fix) = texts_of(&finding);
        assert_ne!(
            message,
            why,
            "the message states the problem and the why states the rule; got {message}"
        );
        assert_ne!(
            message,
            fix,
            "the message states the problem and the fix states the action; got {message}"
        );
        assert_ne!(
            why,
            fix,
            "the why states the rule and the fix states the action; got {why}"
        );
    }
}

#[test]
fn every_field_is_present() {
    for finding in sample_findings() {
        let (message, why, fix) = texts_of(&finding);
        assert!(!message.trim().is_empty(), "the problem must be stated");
        assert!(!why.trim().is_empty(), "the rule must be stated");
        assert!(!fix.trim().is_empty(), "the action must be stated");
    }
}

// ─── The width budget ──────────────────────────────────────────────────────

#[test]
fn every_line_of_a_finding_stays_inside_the_width_budget() {
    for finding in sample_findings() {
        let (message, why, fix) = texts_of(&finding);
        for line in message.lines().chain(why.lines()).chain(fix.lines()) {
            let width = line.chars().count();
            assert!(
                width <= NAMING_LINE_WIDTH,
                "a line of a finding is {width} characters, over the {NAMING_LINE_WIDTH} budget: \
                 {line}"
            );
        }
    }
}

#[test]
fn the_width_budget_is_seventy_two_of_eighty_columns() {
    assert_eq!(NAMING_LINE_WIDTH, 72);
}

// ─── The character repertoire ──────────────────────────────────────────────

#[test]
fn every_finding_is_ascii() {
    for finding in sample_findings() {
        let (message, why, fix) = texts_of(&finding);
        for text in [message, why, fix] {
            assert!(
                text.is_ascii(),
                "the emitted text must be ASCII so it survives a terminal that cannot render \
                 Unicode: {text}"
            );
        }
    }
}

#[test]
fn no_finding_carries_an_em_dash() {
    for finding in sample_findings() {
        let (message, why, fix) = texts_of(&finding);
        for text in [message, why, fix] {
            assert!(
                !text.contains('\u{2014}'),
                "the emitted text must not carry an em dash: {text}"
            );
        }
    }
}

// ─── AES101: one problem, one action ──────────────────────────────────────

#[test]
fn a_casing_failure_and_a_word_count_failure_read_differently() {
    let casing = aes101(
        "src/capabilities_UserChecker.rs",
        "capabilities_UserChecker.rs",
        "capabilities",
    );
    let words = aes101("src/taxonomy_user.rs", "taxonomy_user.rs", "taxonomy");
    assert!(
        casing.message.value().contains("uppercase"),
        "a casing failure must name the casing: {}",
        casing.message.value()
    );
    assert!(
        words.message.value().contains("words"),
        "a word-count failure must name the word count: {}",
        words.message.value()
    );
}

#[test]
fn no_aes101_finding_offers_a_list_of_guesses() {
    for finding in sample_findings() {
        if finding.code.code() != "AES101" {
            continue;
        }
        assert!(
            !finding.message.value().contains("may have"),
            "a finding names the one rule that broke, not three guesses: {}",
            finding.message.value()
        );
    }
}

#[test]
fn the_aes101_fix_names_the_file_to_rename_to() {
    let finding = aes101(
        "src/capabilities_UserChecker.rs",
        "capabilities_UserChecker.rs",
        "capabilities",
    );
    assert!(
        finding.fix.contains("capabilities_user_checker.rs"),
        "the fix must name the file to rename to: {}",
        finding.fix
    );
}

#[test]
fn a_stem_with_no_folder_prefix_is_told_the_prefixes_it_may_use() {
    let finding = aes101_without_prefix("src/badname.rs", "badname.rs");
    for prefix in ["capabilities_", "taxonomy_", "utility_"] {
        assert!(
            finding.fix.contains(prefix),
            "the fix must list the legal prefixes so the reader can pick one: {}",
            finding.fix
        );
    }
}

// ─── AES102: the fix closes the loop ───────────────────────────────────────

#[test]
fn the_aes102_fix_names_the_reference_update_and_the_exemption() {
    let finding = aes102("src/capabilities_user_vo.rs", "capabilities_user_vo.rs");
    assert!(
        finding.fix.contains("update the imports"),
        "renaming a file breaks the imports that name it; the fix must say so: {}",
        finding.fix
    );
    assert!(
        finding.fix.contains("architecture.rules.AES102.exceptions"),
        "the fix must name the exemption mechanism: {}",
        finding.fix
    );
}

#[test]
fn the_aes102_fix_names_one_action() {
    for finding in sample_findings() {
        if finding.code.code() != "AES102" {
            continue;
        }
        assert!(
            !finding.fix.contains(" or "),
            "a finding names one action, never a choice between two: {}",
            finding.fix
        );
    }
}

#[test]
fn the_aes101_fix_names_one_action() {
    for finding in sample_findings() {
        if finding.code.code() != "AES101" {
            continue;
        }
        assert!(
            !finding.fix.contains(" or "),
            "a finding names one action, never a choice between two: {}",
            finding.fix
        );
    }
}

#[test]
fn every_finding_names_the_exemption_mechanism() {
    for finding in sample_findings() {
        let (_, _, fix) = texts_of(&finding);
        let code = finding.code.code();
        assert!(
            fix.contains(&format!("architecture.rules.{code}.exceptions")),
            "the fix of {code} must name its exemption list: {fix}"
        );
    }
}

// ─── AES103: the example comes from the directory ──────────────────────────

#[test]
fn the_nesting_example_comes_from_the_directory_the_finding_reports() {
    let under_tests = aes103("crates/calc/tests/inner/unit_adder.rs", "unit_adder.rs");
    assert!(
        under_tests.fix.contains("tests/unit_<subject>.rs"),
        "a file under tests/ is told about a test name: {}",
        under_tests.fix
    );
    let under_benches = aes103("crates/calc/benches/inner/bench_adder.rs", "bench_adder.rs");
    assert!(
        under_benches.fix.contains("benches/bench_<subject>.rs"),
        "a file under benches/ is told about a benchmark name: {}",
        under_benches.fix
    );
    assert!(
        !under_benches.fix.contains("unit_<subject>"),
        "a file under benches/ may not carry a test name: {}",
        under_benches.fix
    );
}

#[test]
fn the_depth_reads_one_level_and_two_levels() {
    let one = aes103("crates/calc/tests/inner/unit_adder.rs", "unit_adder.rs");
    assert!(
        one.message.value().contains("1 level inside"),
        "one level reads as one level: {}",
        one.message.value()
    );
    let two = aes103(
        "crates/calc/tests/inner/deeper/unit_adder.rs",
        "unit_adder.rs",
    );
    assert!(
        two.message.value().contains("2 levels inside"),
        "two levels read as two levels: {}",
        two.message.value()
    );
}

// ─── The severity levels ──────────────────────────────────────────────────

#[test]
fn the_findings_carry_the_documented_severity_levels() {
    let missing_prefix = aes101_without_prefix("src/badname.rs", "badname.rs");
    assert_eq!(missing_prefix.severity, Severity::HIGH);

    let casing = aes101(
        "src/capabilities_UserChecker.rs",
        "capabilities_UserChecker.rs",
        "capabilities",
    );
    assert_eq!(casing.severity, Severity::MEDIUM);

    let forbidden = aes102("src/capabilities_user_vo.rs", "capabilities_user_vo.rs");
    assert_eq!(forbidden.severity, Severity::HIGH);

    let unknown = aes102(
        "src/capabilities_user_handler.rs",
        "capabilities_user_handler.rs",
    );
    assert_eq!(unknown.severity, Severity::MEDIUM);

    let nested = aes103("crates/calc/tests/inner/unit_adder.rs", "unit_adder.rs");
    assert_eq!(nested.severity, Severity::MEDIUM);
}

#[test]
fn the_rules_carry_more_than_one_severity_level() {
    let mut levels: Vec<&str> = sample_findings().iter().map(level_of).collect();
    levels.sort_unstable();
    levels.dedup();
    assert!(
        levels.len() > 1,
        "the naming rules must carry more than one level so a reader can triage; got {levels:?}"
    );
}

// ─── The audit summary ─────────────────────────────────────────────────────

#[test]
fn the_summary_counts_the_files_each_rule_read() {
    let stats = NamingAuditStats {
        aes101_inspected: 342,
        aes102_inspected: 342,
        aes103_inspected: 28,
    };
    let summary = render_naming_summary(&stats, &[]);
    assert!(
        summary.contains("342 files checked"),
        "the summary must say how many files the rules read: {summary}"
    );
    assert!(
        summary.contains("0 findings"),
        "a clean run must say so: {summary}"
    );
}

#[test]
fn the_summary_counts_the_findings_per_level() {
    let stats = NamingAuditStats {
        aes101_inspected: 2,
        aes102_inspected: 2,
        aes103_inspected: 0,
    };
    let findings = vec![
        aes101_without_prefix("src/badname.rs", "badname.rs"),
        aes101(
            "src/capabilities_UserChecker.rs",
            "capabilities_UserChecker.rs",
            "capabilities",
        ),
    ];
    let summary = render_naming_summary(&stats, &findings);
    assert!(
        summary.contains("2 findings: 1 high, 1 medium"),
        "the summary must count the findings per level: {summary}"
    );
    assert!(
        summary.contains("Nothing was renamed or moved"),
        "the summary must say that the tool changed nothing: {summary}"
    );
}
