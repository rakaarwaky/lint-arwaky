// PURPOSE: doc invariant rule codes, violation types, and shared vocabulary for the doc-rules feature

// Re-export the build-time generated shape constants (required H2s, table
// column headers, fixed row labels) extracted from `references/*/template.md`
// by `build.rs`. These used to be hand-maintained below; they are now
// generated so the linter tracks template changes automatically.
pub use super::doc_generated::*;

/// ─── AES601 — FR format ─────────────────────────────────────────────────────
/// Violation types for FR-ID and FR-field violations.
pub const RULE_CODE_FR_FORMAT: &str = "AES601";

/// FR-ID is bare (`FR-NNN`) without a feature prefix.
pub const FR_ID_VIOLATION_MISSING_FEATURE_PREFIX: &str = "id_missing_feature_prefix";

/// A method named in the FRD's Protocol API / Aggregate API table has no
/// matching declaration in the feature's contract module.
pub const FR_API_METHOD_VIOLATION_NOT_FOUND: &str = "api_method_not_found";

/// A BACKLOG `## Backlog` row whose ID cell is not a valid FRD FR-ID.
pub const FR_BACKLOG_ID_NOT_FR: &str = "backlog_id_not_fr";

/// An FRD FR-ID that has no corresponding BACKLOG `## Backlog` row.
pub const FR_BACKLOG_ID_MISSING: &str = "backlog_id_missing";

/// A DESIGN.md DS-ID that has no corresponding BACKLOG `## Backlog` row.
pub const DS_BACKLOG_ID_MISSING: &str = "ds_backlog_id_missing";

/// A BACKLOG `## Backlog` row whose ID cell is not a valid DESIGN.md DS-ID.
pub const DS_BACKLOG_ID_NOT_DS: &str = "ds_backlog_id_not_ds";

/// A DATA.md DT-ID that has no corresponding BACKLOG `## Backlog` row.
pub const DT_BACKLOG_ID_MISSING: &str = "dt_backlog_id_missing";

/// A BACKLOG `## Backlog` row whose ID cell is not a valid DATA.md DT-ID.
pub const DT_BACKLOG_ID_NOT_DT: &str = "dt_backlog_id_not_dt";

/// A PRD `PR-<PROJECT>-NNN` item that has no ROADMAP `Roadmap Status` entry.
pub const PR_ROADMAP_ID_MISSING: &str = "pr_roadmap_id_missing";

/// A ROADMAP `Roadmap Status` `PR-<PROJECT>-NNN` item that is not
/// declared in the PRD's `Project Requirements`.
pub const ROADMAP_PR_ID_NOT_PR: &str = "roadmap_pr_id_not_pr";

/// A BACKLOG entry whose H3 qualifier does not equal the qualifier of
/// any ID declared by the paired spec, so the entry cannot be traced
/// back to the spec it is supposed to mirror.
pub const BACKLOG_QUALIFIER_MISMATCH: &str = "backlog_qualifier_mismatch";

/// A BACKLOG `## Backlog Status` entry H3 whose qualifier has no matching
/// PR-ID in the root ROADMAP/PRD pair (root/legacy BACKLOG).
pub const PR_BACKLOG_ID_MISSING: &str = "pr_backlog_id_missing";

/// A token in a BACKLOG entry's `Next`/`Done`/`Blocked` row that is not a
/// valid PR-ID of the root ROADMAP/PRD pair.
pub const BACKLOG_ID_NOT_PR: &str = "backlog_id_not_pr";

/// ─── AES602 — Section structure ─────────────────────────────────────────────
pub const RULE_CODE_SECTION_STRUCTURE: &str = "AES602";

/// Sections appear in an order that does not match the template.
pub const SECTION_STRUCTURE_VIOLATION_ORDER: &str = "order_violation";

/// ─── AES603 — Spec purity ───────────────────────────────────────────────────
pub const RULE_CODE_SPEC_PURITY: &str = "AES603";

/// A spec names a concrete source file.
pub const SPEC_PURITY_VIOLATION_SOURCE_FILE_NAMED: &str = "source_file_named";

/// A spec carries implementation state (checkboxes, status fields, progress).
pub const SPEC_PURITY_VIOLATION_STATUS_LEAK: &str = "status_leak";

/// ─── AES604 — Crosslinks ────────────────────────────────────────────────────
pub const RULE_CODE_CROSSLINKS: &str = "AES604";

/// An FRD does not link its BACKLOG.md in the Reference section.
pub const CROSSLINKS_VIOLATION_NO_BACKLOG_LINK: &str = "no_backlog_link";

/// An FRD does not link its PRD.md in the Reference section.
pub const CROSSLINKS_VIOLATION_NO_PRD_LINK: &str = "no_prd_link";

/// State vocabulary is restated in a sub-doc instead of living only in the master.
pub const CROSSLINKS_VIOLATION_STATE_VOCAB_RESTATED: &str = "state_vocab_restated";

/// ─── AES605 — Doc heading structure ────────────────────────────────────────
pub const RULE_CODE_DOC_STRUCTURE: &str = "AES605";

/// A document carries other than exactly one level-1 heading.
pub const DOC_STRUCTURE_VIOLATION_H1_COUNT: &str = "h1_count";

/// A document is missing one or more required level-2 sections.
pub const DOC_STRUCTURE_VIOLATION_H2_MISSING: &str = "h2_missing";

/// A document carries a level-2 heading outside the agreed template.
pub const DOC_STRUCTURE_VIOLATION_H2_UNEXPECTED: &str = "h2_unexpected";

/// A non-placeholder line drifts from the template — content was changed
/// instead of left verbatim. Every line without `{}` must match the template
/// exactly.
pub const DOC_STRUCTURE_VIOLATION_LINE_DRIFT: &str = "line_drift";

/// A document is missing the YAML frontmatter block.
pub const DOC_STRUCTURE_VIOLATION_FRONTMATTER_MISSING: &str = "frontmatter_missing";

/// The FRD declares a different number of requirements than the feature's
/// contract module declares protocol classes. (Merged into AES601.)
pub const FR_PROTOCOL_PARITY_VIOLATION_COUNT_MISMATCH: &str = "protocol_count_mismatch";

/// ─── AES606 — Template conformance + required document ────────────────────
pub const RULE_CODE_TEMPLATE_CONFORMANCE: &str = "AES606";

/// A section that must carry a table in the template does not.
pub const TEMPLATE_CONFORMANCE_VIOLATION_SECTION_NOT_TABLE: &str = "section_not_table";

/// A table is present but is missing one or more required columns.
pub const TEMPLATE_CONFORMANCE_VIOLATION_TABLE_MISSING_COLUMNS: &str = "table_missing_columns";

/// An entry table's first-column row labels do not match the template's fixed set.
pub const TEMPLATE_CONFORMANCE_VIOLATION_ENTRY_MISSING_ROW: &str = "entry_missing_row";

/// A table body repeats its header row in a data-row position, so the table
/// carries the column names where the template expects filled values.
pub const TEMPLATE_CONFORMANCE_VIOLATION_TABLE_HEADER_REPEATED: &str = "table_header_repeated";

/// A table header carries a column the template's fixed header set does not
/// declare. The header must match the template exactly — no extra columns.
pub const TEMPLATE_CONFORMANCE_VIOLATION_TABLE_EXTRA_COLUMNS: &str = "table_extra_columns";

/// A section where the template prescribes no table at all still carries one,
/// so the table has no template to conform to.
pub const TEMPLATE_CONFORMANCE_VIOLATION_TABLE_NOT_IN_TEMPLATE: &str = "table_not_in_template";

/// A level-3 heading in a position the template does not sanction: under a
/// DESIGN section whose H3s are reserved for `DS-<NAME>-NNN` entry items.
pub const TEMPLATE_CONFORMANCE_VIOLATION_H3_UNEXPECTED: &str = "h3_unexpected";

/// A required root document does not exist at the audit root.
pub const MISSING_DOC_VIOLATION: &str = "missing_required_doc";

/// ─── Shape constants ────────────────────────────────────────────────────────
/// Adapter name the doc checker reports under.
pub const ADAPTER_NAME: &str = "architecture";

/// Document names this feature recognizes, per the document chain.
pub const PRD_DOC: &str = "PRD.md";
pub const ROADMAP_DOC: &str = "ROADMAP.md";
pub const FRD_DOC: &str = "FRD.md";
pub const BACKLOG_DOC: &str = "BACKLOG.md";
pub const DATA_DOC: &str = "DATA.md";
pub const DESIGN_DOC: &str = "DESIGN.md";
pub const README_DOC: &str = "README.md";
pub const AGENTS_DOC: &str = "AGENTS.md";
pub const ARCHITECTURE_DOC: &str = "ARCHITECTURE.md";
pub const CONTRIBUTING_DOC: &str = "CONTRIBUTING.md";

/// The root master, which owns the state vocabulary and the feature roll-up.
/// A legacy root BACKLOG.md is accepted so a workspace can migrate in place.
pub const MASTER_DOC_CANDIDATES: &[&str] = &[ROADMAP_DOC, BACKLOG_DOC];

/// Sections that exist only in the root master, per references/roadmap/template.md.
/// `Roadmap Status` is root-only: feature backlogs carry `Backlog Status`
/// and legitimately reuse `Progress Overview`.
pub const MASTER_ONLY_SECTIONS: &[&str] = &["Roadmap Status"];

/// A folder is a feature folder only when it holds an orchestrator.
pub const ORCHESTRATOR_SUFFIX: &str = "_orchestrator";

/// Kernel folders are not features and carry DATA.md + BACKLOG.md.
pub const KERNEL_DIR: &str = "shared";

// Required H2 sets by document, table column headers, and fixed row labels
// are now generated at build time from the reference templates in
// `crates/shared/skills/aes-docs/references/*/template.md` by
// `crates/shared/src/doc_rules/build.rs` (via Comrak) and re-exported
// through `pub use super::doc_generated::*;` at the top of this module.
// When a template changes, the next `cargo build` regenerates these
// constants automatically; no manual editing is required here.
//
// `DOC_HEADING_CONTRACTS` (below) still names each doc's `*_REQUIRED_H2S`
// and `DOC_HEADING_CONTRACTS` is hand-maintained — it is a cross-doc
// contract table, not a template shape.

/// Empty under the 9-doc layout; `fr_id_heading_re` is the authority on
/// sanctioned H3s. PR-10 bookkeeping: re-populate when FRD H3 shapes return.
pub const FRD_H3_TITLES: &[&str] = &[];

/// H3 subsections of the retired `## API Contract`; still read by
/// `utility_markdown_scanner::api_contract_methods` (AES601 method check).
pub const API_CONTRACT_SUBSECTIONS: &[&str] = &["Protocol API", "Aggregate API"];

/// File extensions a spec must never name, per references/frd/template.md Rule 7.
pub const SOURCE_EXTENSIONS: &[&str] = &["py", "rs", "ts", "tsx"];

// ─── AES606 — Template body-shape conformance ──────────────────────────────
// Table shapes are now generated at build time from the reference templates
// in `crates/shared/skills/aes-docs/references/*/template.md` by
// `crates/shared/src/doc_rules/build.rs` (via Comrak) and re-exported
// through `pub use super::doc_generated::*;` at the top of this module.
//
// `ASPECT_DETAIL_COLUMNS`, `REFERENCE_COLUMNS`, `*_ROW_LABELS`, and
// `*_COLUMNS` sets used to be hand-maintained here; they are now generated
// so the linter tracks template changes automatically.

/// The documents the template requires at the workspace root. `DESIGN.md`,
/// `FRD.md`, `DATA.md`, and root `BACKLOG.md` are intentionally absent: the
/// three are per-feature files, and the root master is `ROADMAP.md`.
pub const REQUIRED_ROOT_DOCS: &[&str] = &[
    PRD_DOC,
    ROADMAP_DOC,
    README_DOC,
    AGENTS_DOC,
    ARCHITECTURE_DOC,
    CONTRIBUTING_DOC,
];

/// A marker file whose presence at an audit root identifies it as a real
/// workspace root rather than a sub-directory the user is auditing alone.
pub const WORKSPACE_ROOT_MARKERS: &[&str] = &["Cargo.toml", "pyproject.toml", "package.json"];

/// ─── AES605 — Document heading structure ──────────────────────────────
/// Mapping from root-document filename to the H2 contracts derived from
/// each document's Section Contract table in the corresponding
/// reference template (`references/<doc>/template.md`). The `required`
/// set is mandatory; the
/// `allowed` set is recognized but not enforced. Level-3+ headings stay
/// free-form per project. Matching is by leading words, so
/// `## Architecture: AES 7-Layer System` satisfies "Architecture".
///
/// Any H2 outside `required ∪ allowed` fires `h2_unexpected`.
/// Every heading inside `required` but absent from the file fires
/// `h2_missing`. A file with zero or more-than-one H1 fires `h1_count`.
///
/// The `DATA_DOC` entry allows item H2s carrying a `DT-` prefix
/// (`## DT-<NAME>-NNN: <title>`); any other H2 is `h2_unexpected`. `allowed`
/// entries are leading-word prefixes, so the `DT-` allowed entry admits ANY H2
/// whose normalized title starts with `dt` — a deliberately looser gate:
/// AES605 only polices H2 shape, while AES601's qualified `DT-<NAME>-NNN`
/// regex is the authority on which H2s count as item ids. The
/// `DESIGN_DOC` entry is transcribed verbatim from the fenced template in
/// `skills/aes-docs/references/references/design/template.md.md`, and `allowed` is empty
/// on purpose: copying that template must produce a file AES605 accepts with
/// no edit. An empty `allowed` makes the set closed, so a DESIGN.md carrying an
/// H2 the template does not name — including the surface-behaviour headings
/// ("Kind", "Entry Points", "States") that an earlier revision of this contract
/// required — fires `h2_unexpected`.
pub type DocH2Contract = (
    &'static str,
    &'static [&'static str],
    &'static [&'static str],
);

pub const DOC_HEADING_CONTRACTS: &[DocH2Contract] = &[
    (AGENTS_DOC, AGENTS_REQUIRED_H2S, &[]),
    (ARCHITECTURE_DOC, ARCHITECTURE_REQUIRED_H2S, &[]),
    (CONTRIBUTING_DOC, CONTRIBUTING_REQUIRED_H2S, &[]),
    (PRD_DOC, PRD_REQUIRED_H2S, &[]),
    (
        // `allowed` is empty on purpose: `Project Structure` belongs to the
        // AGENTS.md contract, and a README that repeats it must be reported
        // rather than passed as a second source of layout truth. An empty
        // `allowed` makes the set closed, so the heading fires
        // `h2_unexpected` and has to be demoted or deleted.
        README_DOC,
        README_REQUIRED_H2S,
        &[],
    ),
    (ROADMAP_DOC, ROADMAP_REQUIRED_H2S, &[]),
    (FRD_DOC, FRD_REQUIRED_H2S, &["API Contract"]),
    (BACKLOG_DOC, BACKLOG_REQUIRED_H2S, &[]),
    (DATA_DOC, DATA_REQUIRED_H2S, &["DT-"]),
    (DESIGN_DOC, DESIGN_REQUIRED_H2S, &[]),
];

// ─── AES606 (expanded) — member docs & template body ─────────────────────
/// Missing FRD.md / BACKLOG.md / DATA.md / DESIGN.md in a member folder.
pub const MEMBER_DOC_VIOLATION: &str = "missing_member_doc";

/// Forbidden spec doc present in a member folder that does not owe it.
pub const MEMBER_DOC_FORBIDDEN_VIOLATION: &str = "forbidden_member_doc";

/// A template placeholder token survived non-prose stripping on a document.
pub const TEMPLATE_BODY_VIOLATION_UNFILLED_PLACEHOLDER: &str = "unfilled_placeholder";

/// A non-deletable template section is entirely absent from a document.
pub const TEMPLATE_BODY_VIOLATION_DELETED_SECTION: &str = "section_deleted";

// ─── AES607 — PR-ID owns exactly one module folder ─────────────────────
/// Every `PR-<PROJECT>-NNN` declared in the root PRD's `Project Requirements`
/// must own exactly one top-level module folder under `crates|modules|packages`
/// whose name matches the PR-ID's project identifier.
pub const RULE_CODE_PR_FOLDER_OWNERSHIP: &str = "AES607";

/// A PRD-declared PR-ID whose project identifier owns no matching module folder
/// under `crates|modules|packages`.
pub const PR_FOLDER_NOT_OWNED_VIOLATION: &str = "pr_folder_not_owned";

// ─── AES606 — FRD Technical Specification constants (modular template) ────
// `FR_FIELDS` (generated by build.rs from references/frd/template.md) is the
// source of truth for the six `Technical Specification` table rows
// (Input, Output, Protocol, Method, Event, Error). `FRD_ROW_LABELS` is its
// alias used by `detect_entry_tables` for AES606 row-label enforcement.
// The per-aspect H4 seed/bounded contract of the prior template is retired.

/// Canonical order of the six FRD `Technical Specification` table rows.
/// Alias of `FR_FIELDS`: the entry-table row labels and the AES601 field
/// list read the same six labels under two names, so the AES601 field
/// check and the AES606 row check cannot drift.
pub const FRD_ASPECTS: &[&str] = FR_FIELDS;

/// `FRD_ASPECTS` is read-only; there is no open-ended aspect and no
/// bounded-one aspect under the modular template. The per-aspect H4
/// contract (open / bounded-one / seed-min) is retired.
pub const FRD_ASPECTS_OPEN: &[&str] = &[];
pub const FRD_ASPECTS_BOUNDED_ONE: &[&str] = &[];

/// Empty under the modular template: each `Technical Specification` row is
/// enforced by `detect_entry_tables` via `FR_FIELDS`/`FRD_ROW_LABELS`, not
/// by per-aspect H4 subsections.
pub const FRD_ASPECT_SEED_MIN: &[(&str, usize)] = &[];

/// Canonical order of the three PRD per-aspect H4 subsections, per the new
/// modular references/prd/template.md.md template.
pub const PRD_ASPECTS: &[&str] = &["User Stories", "Proposed Solution", "Main User Flow"];

/// All three PRD aspects are open-ended (seed minimum 1 each).
pub const PRD_ASPECTS_OPEN: &[&str] = PRD_ASPECTS;

/// Minimum content per PRD aspect. All three are open-ended at 1
/// (one real row / bullet / step; a second only when a second real fact
/// exists — never a placeholder cell).
pub const PRD_ASPECT_SEED_MIN: &[(&str, usize)] = &[
    ("User Stories", 1),
    ("Proposed Solution", 1),
    ("Main User Flow", 1),
];

/// An expected per-aspect H4 subsection is missing under an FR/PR entry.
pub const TEMPLATE_CONFORMANCE_VIOLATION_MISSING_H4_ASPECT: &str = "missing_h4_aspect";

/// Per-aspect H4 subsections are out of canonical order.
pub const TEMPLATE_CONFORMANCE_VIOLATION_H4_ORDER: &str = "h4_order_violation";

/// Numbered rows are not contiguous 1..N.
pub const TEMPLATE_CONFORMANCE_VIOLATION_ROW_NUMBERING: &str = "row_numbering_violation";

/// A bounded aspect does not carry exactly its seed row count.
pub const TEMPLATE_CONFORMANCE_VIOLATION_ROW_COUNT: &str = "row_count_violation";

/// An open-ended aspect carries fewer rows than its seed minimum.
pub const TEMPLATE_CONFORMANCE_VIOLATION_ROW_MINIMUM: &str = "row_minimum_violation";

/// A numbered or `∞` row whose `Detail` cell trims to empty.
pub const TEMPLATE_CONFORMANCE_VIOLATION_EMPTY_DETAIL_CELL: &str = "empty_detail_cell";

// ─── Per-doc rule codes: 1 document = 1 AES code = 1 protocol = 1 capability.
// Each per-doc capability stamps every finding it emits with its own code,
// so a finding's code alone names the document it governs.
pub const RULE_CODE_AGENTS_DOC: &str = "AES601";
pub const RULE_CODE_README_DOC: &str = "AES602";
pub const RULE_CODE_ARCHITECTURE_DOC: &str = "AES603";
pub const RULE_CODE_CONTRIBUTING_DOC: &str = "AES604";
pub const RULE_CODE_PRD_DOC: &str = "AES605";
pub const RULE_CODE_ROADMAP_DOC: &str = "AES606";
pub const RULE_CODE_BACKLOG_DOC: &str = "AES607";
pub const RULE_CODE_DATA_DOC: &str = "AES608";
pub const RULE_CODE_FRD_DOC: &str = "AES609";
pub const RULE_CODE_DESIGN_DOC: &str = "AES610";

// Re-export template conformance types from the template module so that
// `shared_doc_rules::taxonomy_doc_rules_constant` remains the canonical import.
pub use crate::taxonomy_doc_rules_template_vo::{TEMPLATE_PROFILES, TemplateProfile};

#[cfg(test)]
mod tests {
    use super::*;

    /// Load a reference template from skills/aes-docs/references/<doc>/template.md.
    fn load_reference_template(doc: &str) -> String {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../skills/aes-docs/references")
            .join(doc)
            .join("template.md");
        match std::fs::read_to_string(&path) {
            Ok(text) => text,
            Err(e) => panic!("reference template {doc}/template.md must be readable: {e}"),
        }
    }

    /// *columns* must all appear somewhere in the reference template *doc*.
    fn ships_columns(doc: &str, columns: &[&str]) {
        let text = load_reference_template(doc);
        for column in columns {
            assert!(
                text.contains(*column),
                "{doc}/template.md must ship the `{column}` label"
            );
        }
    }

    /// The AES606 row-label / column constants must map to a live reference
    /// template: each set transcribes a table the shipped template actually
    /// contains, so a constant with no template behind it fails here instead
    /// of silently drifting.
    #[test]
    fn row_label_sets_transcribe_their_reference_templates() {
        ships_columns("prd", PRD_ROW_LABELS);
        ships_columns("roadmap", ROADMAP_ROW_LABELS);
        ships_columns("backlog", BACKLOG_ROW_LABELS);
        ships_columns("frd", FRD_ROW_LABELS);
        ships_columns("frd", ASPECT_DETAIL_COLUMNS);
        ships_columns("agents", AGENTS_RUNTIME_ROW_LABELS);
    }

    /// Every doc ships a `*_TEMPLATE_LINES` constant so the line-drift
    /// check tracks template changes. The set covers all 10 doc types.
    #[test]
    fn every_doc_ships_a_template_lines_constant() {
        let all: &[&[&str]] = &[
            AGENTS_TEMPLATE_LINES,
            ARCHITECTURE_TEMPLATE_LINES,
            BACKLOG_TEMPLATE_LINES,
            CONTRIBUTING_TEMPLATE_LINES,
            DATA_TEMPLATE_LINES,
            DESIGN_TEMPLATE_LINES,
            FRD_TEMPLATE_LINES,
            PRD_TEMPLATE_LINES,
            README_TEMPLATE_LINES,
            ROADMAP_TEMPLATE_LINES,
        ];
        assert_eq!(all.len(), 10, "ten doc types, one template-lines constant each");
        for constant in all {
            assert!(
                !constant.is_empty(),
                "every *_TEMPLATE_LINES constant must be non-empty"
            );
        }
    }

    /// The reverse of `ships_columns`, for the PRD entry shape only: the
    /// fenced Project Requirements entry must carry exactly PRD_ROW_LABELS
    /// as its `####` aspect subsections (a label added to the template
    /// without the constant, or dropped from the constant alone, fails
    /// here instead of failing a user who copies the template verbatim).
    #[test]
    fn prd_entry_table_carries_exactly_prd_row_labels() {
        let text = load_reference_template("prd");
        let open = "```markdown";
        let start = text
            .find(open)
            .expect("prd/template.md opens a template block")
            + open.len();
        let rest = &text[start..];
        let end = rest
            .find("```")
            .expect("prd/template.md closes its template block");
        let fence = &rest[..end];
        let entry_start = fence
            .find("### PR-")
            .expect("prd/template.md ships a PR entry in its fence");
        let mut labels: Vec<&str> = fence[entry_start..]
            .lines()
            .filter_map(|line| line.strip_prefix("#### "))
            .map(str::trim)
            .collect();
        labels.sort_unstable();
        labels.dedup();
        let mut expected: Vec<&str> = PRD_ROW_LABELS.to_vec();
        expected.sort_unstable();
        assert_eq!(
            labels, expected,
            "the fenced PR entry's H4 aspects must carry exactly PRD_ROW_LABELS"
        );
    }

    #[test]
    fn header_column_sets_transcribe_their_reference_templates() {
        ships_columns("agents", AGENTS_COMMAND_COLUMNS);

        ships_columns("readme", README_QUICK_START_COLUMNS);
        ships_columns("contributing", CONTRIBUTING_WORKFLOW_COLUMNS);
        ships_columns("contributing", CONTRIBUTING_COMMIT_COLUMNS);
        ships_columns("data", DATA_TYPE_DESCRIPTION_COLUMNS);
        ships_columns("design", DESIGN_VARIANT_VALUE_COLUMNS);
        ships_columns("agents", REFERENCE_COLUMNS);
    }

    /// The AES601 new violation codes added by the PR-ID family and the
    /// BACKLOG qualifier-alignment decisions must map to templates that
    /// ship the corresponding ID / qualifier shapes, so a code with no
    /// template behind it fails here instead of silently drifting.
    #[test]
    fn pr_id_and_backlog_alignment_codes_map_to_their_templates() {
        let prd = load_reference_template("prd");
        let roadmap = load_reference_template("roadmap");
        let backlog = load_reference_template("backlog");
        assert!(
            prd.contains("PR-<PROJECT-NAME-ID>"),
            "prd/template.md ships the `PR-` item shape"
        );
        assert!(
            roadmap.contains("PR-<PROJECT-NAME-ID>"),
            "roadmap/template.md ships the `PR-` item shape"
        );
        assert!(
            backlog.contains("FEATURE-NAME-ID")
                && backlog.contains("DS-NAME-ID")
                && backlog.contains("DT-NAME-ID"),
            "backlog/template.md ships the family-ID disjunction"
        );
        assert_eq!(PR_ROADMAP_ID_MISSING, "pr_roadmap_id_missing");
        assert_eq!(ROADMAP_PR_ID_NOT_PR, "roadmap_pr_id_not_pr");
        assert_eq!(BACKLOG_QUALIFIER_MISMATCH, "backlog_qualifier_mismatch");
        assert_eq!(PR_BACKLOG_ID_MISSING, "pr_backlog_id_missing");
        assert_eq!(BACKLOG_ID_NOT_PR, "backlog_id_not_pr");
    }

    /// The FRD row labels are one list under two names (FR_FIELDS serves the
    /// AES601 field check, FRD_ROW_LABELS the AES606 row check); value
    /// equality here documents the alias the type system cannot.
    #[test]
    fn frd_row_labels_are_the_fr_fields() {
        assert_eq!(FRD_ROW_LABELS, FR_FIELDS, "must stay byte-identical");
        assert_eq!(FRD_ROW_LABELS.len(), 6, "six labels, one per row");
    }

    /// `FRD_ASPECTS` is the canonical order of the six Technical
    /// Specification table rows, so pin the exact ordered sequence — not
    /// just set equality — against `FR_FIELDS` (generated by build.rs).
    #[test]
    fn frd_aspects_are_the_fr_fields() {
        assert_eq!(
            FRD_ASPECTS, FR_FIELDS,
            "FRD_ASPECTS must stay byte-identical to FR_FIELDS"
        );
        assert!(
            FRD_ASPECTS_OPEN.is_empty(),
            "no open-ended aspects under modular template"
        );
        assert!(
            FRD_ASPECTS_BOUNDED_ONE.is_empty(),
            "no bounded-one aspects under modular template"
        );
        assert!(
            FRD_ASPECT_SEED_MIN.is_empty(),
            "per-aspect H4 seed-min contract is retired"
        );
    }

    /// All five PRD aspects are open-ended and seed at 1 real row; row 2
    /// exists only when the aspect carries a second real fact.
    #[test]
    fn prd_aspect_seed_min_all_open_at_one() {
        let mut names: Vec<&str> = PRD_ASPECT_SEED_MIN.iter().map(|(n, _)| *n).collect();
        names.sort_unstable();
        let mut expected: Vec<&str> = PRD_ASPECTS.to_vec();
        expected.sort_unstable();
        assert_eq!(
            names, expected,
            "PRD seed-min map covers every PRD aspect exactly once"
        );
        assert_eq!(
            PRD_ASPECTS_OPEN, PRD_ASPECTS,
            "all PRD aspects are open-ended"
        );
        for (_, min) in PRD_ASPECT_SEED_MIN {
            assert_eq!(*min, 1, "PRD aspects seed at 1");
        }
    }
}
