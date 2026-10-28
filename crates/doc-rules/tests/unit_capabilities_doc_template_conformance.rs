// PURPOSE: unit tests for the doc-rules capabilities — template conformance
// checker (AES606). Extracted from the inline test module that previously lived
// inside crates/doc-rules/src/capabilities_doc_template_conformance_checker.rs
// so the capability source stays free of test-only code.
//
// Each case exercises the smallest input that decides its rule: a section body
// that must be a table, a column set that must be present, fixed row labels
// that must match.
use doc_rules_lint_arwaky::capabilities_doc_template_conformance_checker::{
    TemplateConformanceChecker, backlog_entry_re, ds_entry_re, dt_entry_re, first_column_values,
    fr_entry_re, pr_entry_re,
};
use shared_doc_rules::taxonomy_doc_rules_constant as consts;
use shared_doc_rules::taxonomy_doc_section_vo::Section;
use shared_doc_rules::utility_markdown_scanner::sections;
use std::collections::HashSet;

/// Build one parsed heading section.
fn section(level: usize, title: &str, body: &str, line: usize) -> Section {
    Section::new(level, title.to_string(), body.to_string(), line)
}

#[test]
fn entry_tables_accepts_conforming_frd_entry() {
    let text = "# X\n\n## Functional Requirements\n\n### FR-FEAT-001: Do Thing\n\n\
             | Aspect | Detail |\n| --- | --- |\n| Description | x |\n| Business Rules | x |\n| Edge Cases | x |\n| Capability | x |\n| Protocol | x |\n| Method | x |\n| Input | x |\n| Output | x |\n| Error | x |\n| Event | x |\n| Test | x |\n| Metric | x |\n";
    let parsed = sections(text);
    let re = fr_entry_re().expect("regex compiles");
    let findings = TemplateConformanceChecker::new().check_entry_tables(
        &parsed,
        "Functional Requirements",
        re,
        consts::ASPECT_DETAIL_COLUMNS,
        consts::FRD_ROW_LABELS,
    );
    assert!(
        findings.is_empty(),
        "a conforming FR entry must pass; got {findings:?}"
    );
}

#[test]
fn entry_tables_rejects_missing_row_label() {
    let text = "# X\n\n## Functional Requirements\n\n### FR-FEAT-001: Do Thing\n\n\
             | Aspect | Detail |\n| --- | --- |\n| Description | x |\n| User Story | x |\n";
    let parsed = sections(text);
    let re = fr_entry_re().expect("regex compiles");
    let findings = TemplateConformanceChecker::new().check_entry_tables(
        &parsed,
        "Functional Requirements",
        re,
        consts::ASPECT_DETAIL_COLUMNS,
        consts::FRD_ROW_LABELS,
    );
    assert!(
        findings
            .iter()
            .any(|f| f.violation_type == consts::TEMPLATE_CONFORMANCE_VIOLATION_ENTRY_MISSING_ROW),
        "missing row labels must fire entry_missing_row; got {findings:?}"
    );
    let message = findings[0].message.as_str();
    assert!(
        message.contains("Business Rules"),
        "message must name at least one missing label; got '{message}'"
    );
    assert!(
        message.contains("User Story"),
        "message must name the extra label; got '{message}'"
    );
}

#[test]
fn entry_tables_rejects_no_table() {
    let text = "# X\n\n## Functional Requirements\n\n### FR-FEAT-001: Do Thing\n\njust prose\n";
    let parsed = sections(text);
    let re = fr_entry_re().expect("regex compiles");
    let findings = TemplateConformanceChecker::new().check_entry_tables(
        &parsed,
        "Functional Requirements",
        re,
        consts::ASPECT_DETAIL_COLUMNS,
        consts::FRD_ROW_LABELS,
    );
    assert!(
        findings
            .iter()
            .any(|f| f.violation_type == consts::TEMPLATE_CONFORMANCE_VIOLATION_SECTION_NOT_TABLE),
        "no table must fire section_not_table; got {findings:?}"
    );
}

#[test]
fn prd_entry_rejects_missing_table() {
    let text = "# X\n\n## Project Requirements\n\n### PR-PROJ-001: Do Thing\n\njust prose\n";
    let parsed = sections(text);
    let re = pr_entry_re().expect("regex compiles");
    let findings = TemplateConformanceChecker::new().check_entry_tables(
        &parsed,
        "Project Requirements",
        re,
        consts::ASPECT_DETAIL_COLUMNS,
        consts::PRD_ROW_LABELS,
    );
    assert!(
        findings
            .iter()
            .any(|f| f.violation_type == consts::TEMPLATE_CONFORMANCE_VIOLATION_SECTION_NOT_TABLE),
        "PR entry with no table must fire section_not_table; got {findings:?}"
    );
}

#[test]
fn roadmap_entry_rejects_missing_table() {
    let text = "# X\n\n## Roadmap Status\n\n### PR-PROJ-001: Ship\n\nno table here\n";
    let parsed = sections(text);
    let re = pr_entry_re().expect("regex compiles");
    let findings = TemplateConformanceChecker::new().check_entry_tables(
        &parsed,
        "Roadmap Status",
        re,
        consts::ASPECT_DETAIL_COLUMNS,
        consts::ROADMAP_ROW_LABELS,
    );
    assert!(
        findings
            .iter()
            .any(|f| f.violation_type == consts::TEMPLATE_CONFORMANCE_VIOLATION_SECTION_NOT_TABLE),
        "roadmap entry with no table must fire section_not_table; got {findings:?}"
    );
}

#[test]
fn backlog_entry_rejects_wrong_row_labels() {
    let text = "# X\n\n## Backlog Status\n\n### {RENDER-01}\n\n\
             | Aspect | Detail |\n| --- | --- |\n| State | x |\n| Done | x |\n| Bogus | x |\n";
    let parsed = sections(text);
    let re = backlog_entry_re().expect("regex compiles");
    let findings = TemplateConformanceChecker::new().check_entry_tables(
        &parsed,
        "Backlog Status",
        re,
        consts::ASPECT_DETAIL_COLUMNS,
        consts::BACKLOG_ROW_LABELS,
    );
    assert!(
        findings
            .iter()
            .any(|f| f.violation_type == consts::TEMPLATE_CONFORMANCE_VIOLATION_ENTRY_MISSING_ROW),
        "backlog entry with wrong row labels must fire entry_missing_row; got {findings:?}"
    );
    let message = findings[0].message.as_str();
    assert!(
        message.contains("In Progress") || message.contains("Blocked") || message.contains("Risk"),
        "message must name at least one missing label; got '{message}'"
    );
    assert!(
        message.contains("Bogus"),
        "message must name the extra label; got '{message}'"
    );
}

#[test]
fn data_entry_rejects_header_missing_description() {
    let text = "# X\n\n## Data Requirements\n\n### DT-SHAR-001: Severity\n\n\
             | Type | Notes |\n| --- | --- |\n| vo | level |\n";
    let parsed = sections(text);
    let re = dt_entry_re().expect("regex compiles");
    let findings = TemplateConformanceChecker::new().check_entry_tables(
        &parsed,
        "Data Requirements",
        re,
        consts::DATA_TYPE_DESCRIPTION_COLUMNS,
        &[],
    );
    assert!(
        findings
            .iter()
            .any(|f| f.violation_type
                == consts::TEMPLATE_CONFORMANCE_VIOLATION_TABLE_MISSING_COLUMNS),
        "DATA header lacking Description must fire table_missing_columns; got {findings:?}"
    );
}

#[test]
fn design_entry_rejects_missing_table() {
    let text = "# X\n\n## Design Requirements\n\n### DS-DESIGN-001: X\n\njust prose\n";
    let parsed = sections(text);
    let re = ds_entry_re().expect("regex compiles");
    let findings = TemplateConformanceChecker::new().check_entry_tables(
        &parsed,
        "Design Requirements",
        re,
        consts::DESIGN_VARIANT_VALUE_COLUMNS,
        &[],
    );
    assert!(
        findings
            .iter()
            .any(|f| f.violation_type == consts::TEMPLATE_CONFORMANCE_VIOLATION_SECTION_NOT_TABLE),
        "DESIGN entry with no table must fire section_not_table; got {findings:?}"
    );
}

#[test]
fn design_entry_rejects_repeated_header_row() {
    // The shipped-defect shape: the header row is copied into a data-row
    // position, so the table carries column names where values belong.
    let text = "# X\n\n## Design Requirements\n\n### DS-DESI-001: Outcome Vocabulary\n\n\
             | Variant | Use case | Token | Value|\n\
             |---------|----------|-------|------|\n\
             | Variant | Use case | Token | Value|\n\
             | — | — | — | — |\n";
    let parsed = sections(text);
    let re = ds_entry_re().expect("regex compiles");
    let findings = TemplateConformanceChecker::new().check_entry_tables(
        &parsed,
        "Design Requirements",
        re,
        consts::DESIGN_VARIANT_VALUE_COLUMNS,
        &[],
    );
    assert!(
        findings
            .iter()
            .any(|f| f.violation_type
                == consts::TEMPLATE_CONFORMANCE_VIOLATION_TABLE_HEADER_REPEATED),
        "a data row copying the header must fire table_header_repeated; got {findings:?}"
    );
}

#[test]
fn design_entry_accepts_template_blank_data_row() {
    // HOW-TO-MAKE-DESIGN.md ships header + separator + blank row; real
    // filled values pass the same way (cells differ from the header).
    let text = "# X\n\n## Design Requirements\n\n### DS-DESI-001: Outcome Vocabulary\n\n\
             | Variant | Use case | Token | Value|\n\
             |---------|----------|-------|-------|\n\
             |         |          |       |       |\n";
    let parsed = sections(text);
    let re = ds_entry_re().expect("regex compiles");
    let findings = TemplateConformanceChecker::new().check_entry_tables(
        &parsed,
        "Design Requirements",
        re,
        consts::DESIGN_VARIANT_VALUE_COLUMNS,
        &[],
    );
    assert!(
        findings.is_empty(),
        "the template's blank data row must pass; got {findings:?}"
    );
}

#[test]
fn data_entry_accepts_header_only() {
    let text = "# X\n\n## Data Requirements\n\n### DT-SHAR-001: Severity\n\n\
             | Type | Description |\n| --- | --- |\n| vo | level |\n";
    let parsed = sections(text);
    let re = dt_entry_re().expect("regex compiles");
    let findings = TemplateConformanceChecker::new().check_entry_tables(
        &parsed,
        "Data Requirements",
        re,
        consts::DATA_TYPE_DESCRIPTION_COLUMNS,
        &[],
    );
    assert!(
        findings.is_empty(),
        "header-only DATA entry must pass; got {findings:?}"
    );
}

#[test]
fn fr_entry_rejects_bad_header_missing_detail() {
    let text = "# X\n\n## Functional Requirements\n\n### FR-FEAT-001: Do Thing\n\n\
             | Aspect |\n| --- | |\n| Description | x |\n";
    let parsed = sections(text);
    let re = fr_entry_re().expect("regex compiles");
    let findings = TemplateConformanceChecker::new().check_entry_tables(
        &parsed,
        "Functional Requirements",
        re,
        consts::ASPECT_DETAIL_COLUMNS,
        consts::FRD_ROW_LABELS,
    );
    assert!(
        findings
            .iter()
            .any(|f| f.violation_type
                == consts::TEMPLATE_CONFORMANCE_VIOLATION_TABLE_MISSING_COLUMNS),
        "FR table missing Detail column must fire table_missing_columns; got {findings:?}"
    );
}

#[test]
fn agents_project_structure_rejects_missing_columns() {
    let parsed = vec![section(
        2,
        "Project Structure",
        "| Path |\n| --- |\n| crates/ |\n",
        1,
    )];
    let findings = TemplateConformanceChecker::new().check_h2_table(
        &parsed,
        "Project Structure",
        consts::AGENTS_PATH_PURPOSE_COLUMNS,
    );
    assert!(
        !findings.is_empty()
            && findings.iter().any(|f| f.violation_type
                == consts::TEMPLATE_CONFORMANCE_VIOLATION_TABLE_MISSING_COLUMNS),
        "AGENTS Project Structure table missing Purpose must fire table_missing_columns; got {findings:?}"
    );
}

#[test]
fn readme_quick_start_rejects_missing_columns() {
    let parsed = vec![section(
        2,
        "Quick Start",
        "| Step | Action |\n| --- | --- |\n| 1 | Do it |\n",
        1,
    )];
    let findings = TemplateConformanceChecker::new().check_h2_table(
        &parsed,
        "Quick Start",
        consts::README_QUICK_START_COLUMNS,
    );
    assert!(
        !findings.is_empty()
            && findings.iter().any(|f| f.violation_type
                == consts::TEMPLATE_CONFORMANCE_VIOLATION_TABLE_MISSING_COLUMNS),
        "README Quick Start table missing Command must fire table_missing_columns; got {findings:?}"
    );
}

#[test]
fn contributing_workflow_rejects_missing_columns() {
    let parsed = vec![section(
        2,
        "Workflow",
        "| Step | Action |\n| --- | --- |\n| 1 | Do it |\n",
        1,
    )];
    let findings = TemplateConformanceChecker::new().check_h2_table(
        &parsed,
        "Workflow",
        consts::CONTRIBUTING_WORKFLOW_COLUMNS,
    );
    assert!(
        !findings.is_empty()
            && findings.iter().any(|f| f.violation_type
                == consts::TEMPLATE_CONFORMANCE_VIOLATION_TABLE_MISSING_COLUMNS),
        "CONTRIBUTING Workflow table missing Detail must fire table_missing_columns; got {findings:?}"
    );
}

#[test]
fn entry_tables_skips_non_matching_h3() {
    // Non-matching H3 titles are silently skipped (entry-walking scoping);
    // only matching entries are checked. A prose H3 next to a conforming
    // entry must not fire a finding on its own.
    let text = "# X\n\n## Functional Requirements\n\n### Not An Entry At All\n\njust prose\n\n\
             ### FR-FEAT-001: Do Thing\n\n\
             | Aspect | Detail |\n| --- | --- |\n| Description | x |\n| Business Rules | x |\n| Edge Cases | x |\n| Capability | x |\n| Protocol | x |\n| Method | x |\n| Input | x |\n| Output | x |\n| Error | x |\n| Event | x |\n| Test | x |\n| Metric | x |\n";
    let parsed = sections(text);
    let re = fr_entry_re().expect("regex compiles");
    let findings = TemplateConformanceChecker::new().check_entry_tables(
        &parsed,
        "Functional Requirements",
        re,
        consts::ASPECT_DETAIL_COLUMNS,
        consts::FRD_ROW_LABELS,
    );
    assert!(
        findings.is_empty(),
        "non-matching H3 skipped and conforming entry passes; got {findings:?}"
    );
}

#[test]
fn entry_tables_empty_parent_no_findings() {
    // Parent H2 exists but has no matching H3 entries → no findings
    // (missing-H2 enforcement is AES605's question; empty body is AES606-silent).
    let text = "# X\n\n## Functional Requirements\n\njust prose, no entries\n";
    let parsed = sections(text);
    let re = fr_entry_re().expect("regex compiles");
    let findings = TemplateConformanceChecker::new().check_entry_tables(
        &parsed,
        "Functional Requirements",
        re,
        consts::ASPECT_DETAIL_COLUMNS,
        consts::FRD_ROW_LABELS,
    );
    assert!(
        findings.is_empty(),
        "empty parent H2 yields no findings; got {findings:?}"
    );
}

#[test]
fn first_column_values_only_first_table_header_is_data() {
    // first_column_values aggregates across ALL tables in a body and treats
    // a later table's header row as a data row (single-table assumption,
    // not enforced). Pin that boundary behavior deliberately.
    let body = "| Aspect | Detail |\n| --- | --- |\n| Description | x |\n| Extra Col | y |\n| Second | Header |\n| Data | row |\n";
    let values = first_column_values(body);
    assert!(values.contains("Description"));
    // Current behavior: header_seen skips only the very first `|` line, so
    // a later table's header row first cell leaks into the data values
    // (single-table assumption not enforced). Pin that boundary deliberately.
    assert!(
        values.contains("Second"),
        "later table header first cell leaks in: {values:?}"
    );
}

#[test]
fn backlog_entry_re_matches_single_qualifier() {
    let re = backlog_entry_re().expect("regex compiles");
    assert!(re.is_match("{RENDER-01}"));
    assert!(re.is_match("{FEATURE-NAME-ID}"));
    assert!(!re.is_match("{RENDER-01 | FEATURE-01}"));
}

#[test]
fn agents_runtime_fixed_rows_accepts_conforming() {
    let parsed = vec![section(
        2,
        "Runtime",
        "| Aspect | Detail |\n| --- | --- |\n| Language | Rust |\n| Environment | dev |\n| Artifacts | target/ |\n| Package Manager | cargo |\n| Version Probe | rustc --version |\n| Setup | cargo build |\n| Activation | none |\n",
        1,
    )];
    let findings = TemplateConformanceChecker::new().check_h2_fixed_rows(
        &parsed,
        "Runtime",
        consts::AGENTS_RUNTIME_ROW_LABELS,
    );
    assert!(
        findings.is_empty(),
        "a conforming AGENTS Runtime must pass; got {findings:?}"
    );
}

#[test]
fn agents_runtime_fixed_rows_rejects_missing_row() {
    let parsed = vec![section(
        2,
        "Runtime",
        "| Aspect | Detail |\n| --- | --- |\n| Language | Rust |\n| Environment | dev |\n",
        1,
    )];
    let findings = TemplateConformanceChecker::new().check_h2_fixed_rows(
        &parsed,
        "Runtime",
        consts::AGENTS_RUNTIME_ROW_LABELS,
    );
    assert!(
        findings
            .iter()
            .any(|f| f.violation_type == consts::TEMPLATE_CONFORMANCE_VIOLATION_ENTRY_MISSING_ROW),
        "missing AGENTS Runtime rows must fire entry_missing_row; got {findings:?}"
    );
}
