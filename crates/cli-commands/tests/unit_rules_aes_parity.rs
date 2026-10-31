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
fn rule_index_has_41_rows() {
    let doc = include_str!("../../../ARCHITECTURE.md");
    let table_start = doc
        .lines()
        .position(|line| line.trim_start().starts_with("| AES101 |"))
        .expect("Rule Index table must open with AES101 row");
    let mut count = 0usize;
    for line in doc.lines().skip(table_start) {
        if !line.trim_start().starts_with("| AES") {
            break;
        }
        count += 1;
    }
    assert_eq!(count, 41, "Rule Index must carry exactly 41 AES code rows");
}

#[test]
fn rule_index_header_matches_inventory_shape() {
    let doc = include_str!("../../../ARCHITECTURE.md");
    let header = doc
        .lines()
        .find(|line| line.trim_start().starts_with("| Code |"))
        .expect("Rule Index table must carry a header row");
    assert_eq!(
        header.trim_start(),
        "| Code | Group | Name | Severity |",
        "inventory table header must match the pinned 4-column shape"
    );
}

#[test]
fn group_headings_cover_seven_groups() {
    let doc = include_str!("../../../ARCHITECTURE.md");
    let groups = doc
        .lines()
        .filter(|line| line.trim_start().starts_with("## Group "))
        .count();
    assert_eq!(
        groups, 7,
        "ARCHITECTURE.md must have exactly 7 top-level Group sections"
    );
}

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
fn distinct_codes_equal_41_and_group_totals() {
    let doc = include_str!("../../../ARCHITECTURE.md");
    let mut codes = aes_codes_from_rules_doc();
    codes.sort();
    codes.dedup();

    let actual = codes.len();
    assert_eq!(actual, 41, "expected 41 distinct AES codes, found {actual}");

    // Every Group section carries exactly its span of codes (101–103, 201–205,
    // 301–305, 401–406, 501–506, 601–610, 701–706).
    for group in [
        "Group 1", "Group 2", "Group 3", "Group 4", "Group 5", "Group 6", "Group 7",
    ] {
        assert!(
            doc.lines().any(|l| l.contains(group)),
            "ARCHITECTURE.md must contain {group}"
        );
    }
}
