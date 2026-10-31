// Parity test: the first-cell `| AESxxx |` rows of ARCHITECTURE.md §13 must
// match the pinned emitted inventory (41 codes) so later format work cannot
// drop a rule. ARCHITECTURE.md is the single rules document (formerly
// RULES_AES.md, retired when the inventory table moved into §13).

fn aes_codes_from_rules_doc() -> Vec<String> {
    include_str!("../../../ARCHITECTURE.md")
        .lines()
        .filter_map(|line| {
            let rest = line.trim_start().strip_prefix("| AES")?;
            Some(format!("AES{}", rest.get(0..3)?))
        })
        .collect()
}

#[test]
fn doc_codes_match_emitted_inventory() {
    // Distinct codes: the §13 inventory table plus the §14–20 group
    // coverage tables all carry `| AESxxx |` first-cell rows, so dedup
    // before comparing against the pinned emitted set.
    let mut codes = aes_codes_from_rules_doc();
    codes.sort_unstable();
    codes.dedup();

    let expected = [
        "AES101", "AES102", "AES103", "AES201", "AES202", "AES203", "AES204", "AES205", "AES301",
        "AES302", "AES303", "AES304", "AES305", "AES401", "AES402", "AES403", "AES404", "AES405",
        "AES406", "AES501", "AES502", "AES503", "AES504", "AES505", "AES506", "AES601", "AES602",
        "AES603", "AES604", "AES605", "AES606", "AES607", "AES608", "AES609", "AES610", "AES701",
        "AES702", "AES703", "AES704", "AES705", "AES706",
    ];

    assert_eq!(
        codes.len(),
        expected.len(),
        "ARCHITECTURE.md inventory must equal the pinned 41-code emitted set"
    );
    for code in &expected {
        assert!(
            codes.iter().any(|c| c == code),
            "ARCHITECTURE.md is missing emitted rule code {code}"
        );
    }
}

#[test]
fn metadata_states_41_rules() {
    let doc = include_str!("../../../ARCHITECTURE.md");
    let summary_end = doc
        .lines()
        .position(|line| line.starts_with("## 14."))
        .expect("the §13 summary precedes the first group section");
    let summary = doc.lines().take(summary_end).collect::<Vec<_>>().join("\n");

    assert!(
        summary.contains("41 rules across 7 groups"),
        "summary must state the 41-rule count"
    );
}

// ARCHITECTURE.md §13 now holds the 41-row inventory table; the structure
// invariants the old RULES_AES.md had (heading-then-table-first, H1-first,
// no-H5) applied to that dedicated table file, not to a prose architecture
// document, so they are no longer asserted here. What remains is the
// invariant that matters for the single source of truth: the §13 inventory
// heading sits directly above its table.
#[test]
fn rule_inventory_table_follows_its_heading() {
    let doc = include_str!("../../../ARCHITECTURE.md");
    let lines: Vec<&str> = doc.lines().collect();
    let heading = lines
        .iter()
        .position(|l| l.starts_with("### Rule inventory"))
        .expect("expected `### Rule inventory` heading in ARCHITECTURE.md §13");
    let mut i = heading + 1;
    while i < lines.len() && lines[i].trim().is_empty() {
        i += 1;
    }
    assert!(
        i < lines.len() && lines[i].trim_start().starts_with('|'),
        "`### Rule inventory` must be followed by its table"
    );
    // The table itself must open with the pinned 5-column header.
    assert!(
        lines[i].starts_with("| Code | Group | Name | Severity | Ran by |"),
        "inventory table header drifted from the pinned shape"
    );
}

// Legacy blacklist — none of the retired tokens may appear in the rules doc.
// Three tokens were dropped from the original RULES_AES-era blacklist:
// - "no-test-file": retired v1 violation_type; the substring "no test" now
//   appears in live §14 AES103 prose ("name no test and are skipped"), so a
//   hyphenated token would be a false negative and the live wording cannot
//   be removed without breaking AES103 detail.
// - "docs only (not scan/check)": retired ran-by phrasing from the pre-v2
//   group-6 scope table; the live group-6 ran-by column now reads
//   "check/scan, docs", making the retired token irrelevant.
// - "surfaces(": the AES201 import matrix (§15) and AES406 role table
//   (§17) use `surfaces(command|…)`, `surfaces(hook|…)`, `surfaces(component|…)`
//   as live scope notation, so the hyphenated form of this token is a valid
//   current concept, not a retired one.
#[test]
fn no_legacy_surfaces_in_rules_doc() {
    let doc = include_str!("../../../ARCHITECTURE.md");

    // Pinned blacklist from the design doc's "Removed (legacy)" table.
    let blacklisted = [
        "**Severity:**",
        "36 rules",
        "field_missing",
        "h3_off_template",
        "api_subsection_no_table",
        "scenarios_empty",
        "glossary_empty",
        "Project Quick Facts",
        "Feature Roll-up",
        "System Overview",
        "**Severity:** HIGH",
        "**Severity:** MEDIUM",
    ];
    for token in &blacklisted {
        assert!(
            !doc.contains(token),
            "ARCHITECTURE.md still contains retired token `{token}`"
        );
    }
}

// Distinct codes == 41 == summary total cell.
#[test]
fn distinct_codes_equal_41_and_inventory_total() {
    let doc = include_str!("../../../ARCHITECTURE.md");
    let mut codes = aes_codes_from_rules_doc();
    codes.sort();
    codes.dedup();

    // One `### AESxxx` heading per rule in §14–20.
    let h3_aes_count = doc.lines().filter(|l| l.starts_with("### AES")).count();
    assert_eq!(
        h3_aes_count, 41,
        "expected 41 `### AES` headings in ARCHITECTURE.md §14–20"
    );

    let actual = codes.len();
    assert_eq!(actual, 41, "expected 41 distinct AES codes, found {actual}");

    // The "Total" row of the Group coverage and counts table must show 41.
    let total_row = doc.lines().find(|l| l.starts_with("| Total |"));
    assert!(
        total_row.is_some_and(|r| r.split('|').any(|c| c.trim() == "41")),
        "Group coverage total row must contain the numeral 41"
    );
}
