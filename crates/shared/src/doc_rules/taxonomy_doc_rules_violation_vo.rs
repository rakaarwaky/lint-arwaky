// PURPOSE: Violation-name vocabulary shared across all doc-rule utilities.
// Every AESD01/AESD06 violation name used by `utility_capabilities_doc_h2_checks`,
// `utility_doc_aspect_entry_checks`, `utility_doc_common_checks`, and
// `utility_doc_fr_format_checks` lives here, keyed behind method names so no
// utility ever hardcodes a violation string. Capabilities pass a
// `DocViolationNames` instance into each check call, so violation names are
// owned by the capability layer while utilities stay name-free (AES201).
//
// All names match the existing `TEMPLATE_CONFORMANCE_VIOLATION_*` /
// `FR_*` / `DOC_STRUCTURE_VIOLATION_*` constants in
// `taxonomy_doc_rules_constant.rs`, so consumers keep reporting the same
// strings today — and any future rename happens in one place.

use crate::taxonomy_doc_rules_constant as consts;

/// A documented set of violation names, one per check family the utilities
/// emit. Methods return `&'static str` so findings keep stamping
/// `DocFinding.violation_type` exactly as before.
#[derive(Debug, Clone, Copy)]
pub struct DocViolationNames;

impl DocViolationNames {
    /// AESD06 — an entry table's first-column row labels do not match the template.
    pub const fn entry_missing_row(self) -> &'static str {
        consts::TEMPLATE_CONFORMANCE_VIOLATION_ENTRY_MISSING_ROW
    }

    /// AESD06 — a table header carries a column the template's fixed header set does not declare.
    pub const fn table_extra_columns(self) -> &'static str {
        consts::TEMPLATE_CONFORMANCE_VIOLATION_TABLE_EXTRA_COLUMNS
    }

    /// AESD06 — a table body repeats its header row in a data-row position.
    pub const fn table_header_repeated(self) -> &'static str {
        consts::TEMPLATE_CONFORMANCE_VIOLATION_TABLE_HEADER_REPEATED
    }

    /// AESD06 — a section is present but carries no markdown table at all.
    pub const fn section_not_table(self) -> &'static str {
        consts::TEMPLATE_CONFORMANCE_VIOLATION_SECTION_NOT_TABLE
    }

    /// AESD06 — a table is present but is missing one or more required columns.
    pub const fn table_missing_columns(self) -> &'static str {
        consts::TEMPLATE_CONFORMANCE_VIOLATION_TABLE_MISSING_COLUMNS
    }

    /// AESD06 — a required aspect H4 is missing under an entry.
    pub const fn missing_h4_aspect(self) -> &'static str {
        consts::TEMPLATE_CONFORMANCE_VIOLATION_MISSING_H4_ASPECT
    }

    /// AESD06 — expected-aspect H4s are out of canonical order.
    pub const fn h4_order_violation(self) -> &'static str {
        consts::TEMPLATE_CONFORMANCE_VIOLATION_H4_ORDER
    }

    /// AESD06 — an aspect row's Detail cell is empty or a placeholder dash.
    pub const fn empty_detail_cell(self) -> &'static str {
        consts::TEMPLATE_CONFORMANCE_VIOLATION_EMPTY_DETAIL_CELL
    }

    /// AESD06 — aspect table rows are not numbered contiguously 1..N.
    pub const fn row_numbering_violation(self) -> &'static str {
        consts::TEMPLATE_CONFORMANCE_VIOLATION_ROW_NUMBERING
    }

    /// AESD06 — an open-ended aspect table carries fewer than its seed minimum rows.
    pub const fn row_minimum_violation(self) -> &'static str {
        consts::TEMPLATE_CONFORMANCE_VIOLATION_ROW_MINIMUM
    }

    /// AESD06 — a bounded aspect table does not carry exactly its seed row count.
    pub const fn row_count_violation(self) -> &'static str {
        consts::TEMPLATE_CONFORMANCE_VIOLATION_ROW_COUNT
    }

    /// AESD01 — BACKLOG entry token does not match any declared ID of its paired spec.
    pub const fn backlog_qualifier_mismatch(self) -> &'static str {
        consts::BACKLOG_QUALIFIER_MISMATCH
    }

    /// AESD05 — the document opens with a number of H1 headings other than one.
    pub const fn h1_count_violation(self) -> &'static str {
        consts::DOC_STRUCTURE_VIOLATION_H1_COUNT
    }

    /// AESD05 — a required H2 heading is missing from the document.
    pub const fn h2_missing(self) -> &'static str {
        consts::DOC_STRUCTURE_VIOLATION_H2_MISSING
    }

    /// AESD05 — an H2 heading appears outside the template's required+allowed set.
    pub const fn h2_unexpected(self) -> &'static str {
        consts::DOC_STRUCTURE_VIOLATION_H2_UNEXPECTED
    }

    /// AESD05 — AGENTS.md has no YAML frontmatter block.
    pub const fn frontmatter_missing(self) -> &'static str {
        consts::DOC_STRUCTURE_VIOLATION_FRONTMATTER_MISSING
    }

    /// AESD05 — AGENTS.md section content drifts from the template.
    pub const fn line_drift(self) -> &'static str {
        consts::DOC_STRUCTURE_VIOLATION_LINE_DRIFT
    }

    /// AESD01 — an FR-ID heading is missing its `FR-<FEATURE>` prefix.
    pub const fn fr_id_missing_feature_prefix(self) -> &'static str {
        consts::FR_ID_VIOLATION_MISSING_FEATURE_PREFIX
    }

    /// AESD01 — a PR-ID declared in the PRD has no ROADMAP `Roadmap Status` entry.
    pub const fn pr_roadmap_id_missing(self) -> &'static str {
        consts::PR_ROADMAP_ID_MISSING
    }

    /// AESD01 — a ROADMAP `PR-` item is not declared in the PRD.
    pub const fn roadmap_pr_id_not_pr(self) -> &'static str {
        consts::ROADMAP_PR_ID_NOT_PR
    }

    /// AESD01 — a promised API method is not declared by any protocol/aggregate trait.
    pub const fn fr_api_method_not_found(self) -> &'static str {
        consts::FR_API_METHOD_VIOLATION_NOT_FOUND
    }

    /// AESD01 — the FRD requirement count does not match the contract seam count.
    pub const fn protocol_count_mismatch(self) -> &'static str {
        consts::FR_PROTOCOL_PARITY_VIOLATION_COUNT_MISMATCH
    }

    /// AESD06 — the shared `| Aspect | Detail |` column set.
    pub const fn aspect_detail_columns(self) -> &'static [&'static str] {
        consts::ASPECT_DETAIL_COLUMNS
    }
}

/// The documented violation-name set used by every per-doc capability.
pub const DOC_VIOLATION_NAMES: DocViolationNames = DocViolationNames;
