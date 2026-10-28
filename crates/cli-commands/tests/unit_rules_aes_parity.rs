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
    let codes = aes_codes_from_rules_doc();

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
fn summary_states_38_rules() {
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
