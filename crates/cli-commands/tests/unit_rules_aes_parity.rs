// Parity test: the first-cell `| AESxxx |` rows of RULES_AES.md must match
// the pinned emitted inventory (38 codes) so later format work cannot drop a rule.

fn aes_codes_from_rules_doc() -> Vec<String> {
    include_str!("../../../RULES_AES.md")
        .lines()
        .filter_map(|line| {
            let rest = line.trim_start().strip_prefix("| AES")?;
            Some(format!("AES{}", rest.get(0..3)?))
        })
        .collect()
}

#[test]
fn doc_codes_match_emitted_inventory() {
    // Distinct codes: Groups 1–3 also appear in the untouched top-level summary
    // block, so dedup before comparing against the pinned emitted set.
    let mut codes = aes_codes_from_rules_doc();
    codes.sort_unstable();
    codes.dedup();

    let expected = [
        "AES101", "AES102", "AES103", "AES201", "AES202", "AES203", "AES204", "AES205", "AES301",
        "AES302", "AES303", "AES304", "AES305", "AES401", "AES402", "AES403", "AES404", "AES405",
        "AES406", "AES501", "AES502", "AES503", "AES504", "AES505", "AES506", "AES601", "AES602",
        "AES603", "AES604", "AES605", "AES606", "AES607", "AES701", "AES702", "AES703", "AES704",
        "AES705", "AES706",
    ];

    assert_eq!(
        codes.len(),
        expected.len(),
        "RULES_AES.md inventory must equal the pinned 38-code emitted set"
    );
    for code in &expected {
        assert!(
            codes.iter().any(|c| c == code),
            "RULES_AES.md is missing emitted rule code {code}"
        );
    }
}

#[test]
fn metadata_states_38_rules() {
    let doc = include_str!("../../../RULES_AES.md");
    let summary_end = doc
        .lines()
        .position(|line| line.starts_with("## Group"))
        .expect("the summary precedes the first group heading");
    let summary = doc.lines().take(summary_end).collect::<Vec<_>>().join("\n");

    assert!(
        summary.contains("38 rules across 7 groups"),
        "summary must state the 38-rule count"
    );
}

fn assert_heading_then_table(lines: &[&str], start_marker: &str, end_marker: &str) {
    let start = lines
        .iter()
        .position(|l| l.starts_with(start_marker))
        .unwrap_or_else(|| panic!("expected heading `{start_marker}` not found"));
    let end = lines
        .iter()
        .position(|l| l.starts_with(end_marker))
        .unwrap_or_else(|| panic!("expected heading `{end_marker}` not found"));

    let is_heading = |l: &&str| {
        let trimmed = l.trim_start();
        trimmed.starts_with('#')
    };
    let is_blank = |l: &str| l.trim().is_empty();
    let is_table = |l: &str| l.trim_start().starts_with('|');

    for i in start..end {
        if is_heading(&lines[i]) {
            assert!(
                i + 2 < lines.len() && is_blank(lines[i + 1]) && is_table(lines[i + 2]),
                "heading `{}` must be followed by a blank line then a table line",
                lines[i].trim()
            );
        }
    }
}

// Scoped to Group 1-3; the whole-file variant lands in PR 4.
#[test]
fn group1_to_3_headings_are_table_first() {
    let doc = include_str!("../../../RULES_AES.md");
    let lines: Vec<&str> = doc.lines().collect();
    assert_heading_then_table(&lines, "## Group 1:", "## Group 4:");
}

// Scoped to Groups 4-6: every heading from ## Group 4 through the line before
// ## Group 7 must be followed by a blank line then a table line.
#[test]
fn group4_to_6_headings_are_table_first() {
    let doc = include_str!("../../../RULES_AES.md");
    let lines: Vec<&str> = doc.lines().collect();
    assert_heading_then_table(
        &lines,
        "## Group 4: Role Violations",
        "## Group 7: Folder Structure",
    );
}

// PR 4: whole-file structure test — every heading in the entire file is
// followed by a blank line then a table line; top-level tokens are only
// blank / heading / table; H1 is first; no H5.
#[test]
fn whole_file_structure_is_table_first() {
    let doc = include_str!("../../../RULES_AES.md");
    let lines: Vec<&str> = doc.lines().collect();

    // H1 must be the first non-blank line.
    let first = lines.iter().find(|l| !l.trim().is_empty());
    assert!(
        first.is_some_and(|l| l.starts_with("# ")),
        "first non-blank line must be an H1 heading"
    );

    for (i, line) in lines.iter().enumerate() {
        let t = line.trim_start();
        if t.is_empty() {
            continue;
        }
        let is_heading = t.starts_with('#');
        let is_table = t.starts_with('|');
        assert!(
            is_heading || is_table,
            "line {} contains neither heading nor table: `{}`",
            i + 1,
            &t[..t.len().min(60)]
        );
        let level = t.len() - t.trim_start_matches('#').len();
        assert!(
            level <= 4,
            "line {}: heading depth {} exceeds the H4 floor",
            i + 1,
            level
        );
        // Every heading is followed by exactly one blank then a table line.
        if is_heading {
            let next = lines.get(i + 1).unwrap_or(&"");
            let after = lines.get(i + 2).unwrap_or(&"");
            assert!(
                next.trim().is_empty(),
                "line {}: heading not followed by a blank line",
                i + 1
            );
            assert!(
                after.trim_start().starts_with('|'),
                "line {}: heading not followed by a table line",
                i + 1
            );
        }
    }
}

// PR 4: legacy blacklist — none of the retired tokens may appear in the file.
#[test]
fn no_legacy_surfaces_in_rules_doc() {
    let doc = include_str!("../../../RULES_AES.md");

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
        "no-test-file",
        "docs only (not scan/check)",
        "surfaces(",
        "**Severity:** HIGH",
        "**Severity:** MEDIUM",
    ];
    for token in &blacklisted {
        assert!(
            !doc.contains(token),
            "RULES_AES.md still contains retired token `{token}`"
        );
    }
}

// PR 4: distinct codes == 38 == summary total cell.
#[test]
fn distinct_codes_equal_38_and_inventory_total() {
    let doc = include_str!("../../../RULES_AES.md");
    let mut codes = aes_codes_from_rules_doc();
    codes.sort();
    codes.dedup();

    let h3_aes_count = doc.lines().filter(|l| l.starts_with("### AES")).count();
    assert_eq!(
        h3_aes_count, 38,
        "expected 38 `### AES` headings in RULES_AES.md"
    );

    let actual = codes.len();
    assert_eq!(actual, 38, "expected 38 distinct AES codes, found {actual}");

    // The "Total" row of the Group coverage and counts table must show 38.
    let total_row = doc.lines().find(|l| l.starts_with("| Total |"));
    assert!(
        total_row.is_some_and(|r| r.split('|').any(|c| c.trim() == "38")),
        "Group coverage total row must contain the numeral 38"
    );
}
