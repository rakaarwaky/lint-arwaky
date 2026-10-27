// PURPOSE: doc invariant rule codes, violation types, and shared vocabulary for the doc-rules feature

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

/// The API Contract section is missing one of its required subsections.
pub const SECTION_STRUCTURE_VIOLATION_API_SUBSECTION: &str = "api_no_subsection";

/// API Contract carries a level-3 heading outside the fixed pair
/// (`Protocol API`, `Aggregate API`). Authoring one H3 per protocol — or any
/// other invented subsection — is the shape this closes.
pub const SECTION_STRUCTURE_VIOLATION_API_H3_UNEXPECTED: &str = "api_h3_unexpected";

/// A required API Contract subsection does not carry its own column-complete
/// table, so the seam was described in prose or folded into its sibling.
pub const SECTION_STRUCTURE_VIOLATION_API_SUBSECTION_NO_TABLE: &str = "api_subsection_no_table";

/// A required API Contract subsection is duplicated, so the fixed H3 pair
/// (`Protocol API`, `Aggregate API`) no longer appears exactly once each.
pub const SECTION_STRUCTURE_VIOLATION_API_SUBSECTION_DUPLICATED: &str = "api_subsection_duplicated";

/// An FRD carries a level-3 heading the template does not sanction. The
/// template's level-3 set is exactly a requirement heading
/// (`### FR-<FEATURENAME>-NNN:`), `Protocol API`, and `Aggregate API`; every
/// other level-3 heading is an invented section.
pub const SECTION_STRUCTURE_VIOLATION_H3_OFF_TEMPLATE: &str = "h3_off_template";

/// Integration Points is present but is not a table with the required columns.
pub const SECTION_STRUCTURE_VIOLATION_INTEGRATION_NOT_TABLE: &str = "integration_not_table";

/// Non-functional Requirements is present but is not a table with the required columns.
pub const SECTION_STRUCTURE_VIOLATION_NFR_NOT_TABLE: &str = "nfr_not_table";

/// Test Scenarios has no bullet items.
pub const SECTION_STRUCTURE_VIOLATION_SCENARIOS_EMPTY: &str = "scenarios_empty";

/// Glossary has no bullet items.
pub const SECTION_STRUCTURE_VIOLATION_GLOSSARY_EMPTY: &str = "glossary_empty";

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

/// Sections that exist only in the root master, per HOW-TO-MAKE-ROADMAP.
/// `Roadmap Status` is root-only: feature backlogs carry `Backlog Status`
/// and legitimately reuse `Progress Overview`.
pub const MASTER_ONLY_SECTIONS: &[&str] = &["Roadmap Status"];

/// A folder is a feature folder only when it holds an orchestrator.
pub const ORCHESTRATOR_SUFFIX: &str = "_orchestrator";

/// Kernel folders are not features and carry DATA.md + BACKLOG.md.
pub const KERNEL_DIR: &str = "shared";

// Required H2 sets by document — one canonical list per doc. The AES605
// `DOC_HEADING_CONTRACTS` entries and the AES606 `TemplateProfile`s both
// read these, so a template change lands in one place. Each set is
// transcribed from the Section Contract table of its HOW-TO-MAKE-*.md
// reference template.

/// AGENTS required H2s, per HOW-TO-MAKE-AGENTS.md.
pub const AGENTS_REQUIRED_H2S: &[&str] = &[
    "Agent Overview",
    "Runtime",
    "Project Structure",
    "Commands",
    "Documentation Awareness",
    "Reference",
];

/// ARCHITECTURE required H2s, per HOW-TO-MAKE-ARCHITECTURE.md.
pub const ARCHITECTURE_REQUIRED_H2S: &[&str] = &[
    "Purpose",
    "Workspace Organization",
    "Naming Convention",
    "Vertical Slicing Layout",
    "Taxonomy Layer",
    "Contract Layer",
    "Utility Layer",
    "Capabilities Layer",
    "Agent Layer",
    "Surface Layer",
    "Root Layer",
];

/// Per-crate BACKLOG section order, per HOW-TO-MAKE-BACKLOG.
pub const BACKLOG_REQUIRED_H2S: &[&str] = &["Progress Overview", "Backlog Status", "Reference"];

/// CONTRIBUTING required H2s, per HOW-TO-MAKE-CONTRIBUTING.md.
pub const CONTRIBUTING_REQUIRED_H2S: &[&str] = &[
    "Contribution Overview",
    "Contribution Guidelines",
    "Workflow",
    "Commit Convention",
    "Reference",
];

/// DATA required H2s, per HOW-TO-MAKE-DATA.md.
pub const DATA_REQUIRED_H2S: &[&str] = &["Data Overview", "Data Requirements", "Reference"];

/// DESIGN required H2s, per HOW-TO-MAKE-DESIGN.md.
pub const DESIGN_REQUIRED_H2S: &[&str] = &["Design Overview", "Design Requirements", "Reference"];

/// Per-crate FRD section order, per HOW-TO-MAKE-FRD.
pub const FRD_REQUIRED_H2S: &[&str] = &[
    "Feature Overview",
    "Functional Requirements",
    "Public Interface",
    "Reference",
];

/// PRD required H2s, per HOW-TO-MAKE-PRD.md.
pub const PRD_REQUIRED_H2S: &[&str] = &["Problem Statement", "Project Requirements", "Reference"];

/// README required H2s, per HOW-TO-MAKE-README.md.
pub const README_REQUIRED_H2S: &[&str] = &[
    "Project Overview",
    "Prerequisites",
    "Quick Start",
    "Reference",
];

/// ROADMAP required H2s, per HOW-TO-MAKE-ROADMAP.md.
pub const ROADMAP_REQUIRED_H2S: &[&str] = &["Progress Overview", "Roadmap Status", "Reference"];

/// Template section order, per HOW-TO-MAKE-FRD. Alias: AES602's order check
/// and the required-H2 set read the same four headings under two names.
pub const FRD_SECTION_ORDER: &[&str] = FRD_REQUIRED_H2S;

/// Legacy: the only level-3 headings the pre-9-doc `HOW-TO-MAKE-FRD.md`
/// sanctioned, by literal title. A requirement heading
/// (`### FR-<FEATURENAME>-NNN: <name>`) is the third sanctioned shape and is
/// matched by the shared FR-ID pattern rather than listed here, so the
/// accepted set cannot drift between AES601 and AES602 — the same reason
/// `fr_id_heading_re` is shared.
///
/// The 9-doc FRD template has no `Protocol API` / `Aggregate API` H3s; this
/// set empties when the AES602 section-structure checker migrates off the
/// old FRD shape. Until then the set is closed against the structure the
/// checker still enforces: an H3 outside these two titles and the FR-ID
/// shape fires `h3_off_template`. Level-4 and deeper stay free-form.
pub const FRD_H3_TITLES: &[&str] = &["Protocol API", "Aggregate API"];

/// The twelve row labels every FRD `Functional Requirements` entry table
/// carries under its `Aspect | Detail` header, per HOW-TO-MAKE-FRD.
pub const FR_FIELDS: &[&str] = &[
    "Description",
    "Business Rules",
    "Edge Cases",
    "Capability",
    "Protocol",
    "Method",
    "Input",
    "Output",
    "Error",
    "Event",
    "Test",
    "Metric",
];

/// Legacy columns of the retired `## API Contract` tables. The 9-doc FRD
/// template has no API Contract section; kept read-only until the AES602
/// section-structure checker migrates off the old FRD shape. Do not cite in
/// new checks.
pub const API_COLUMNS: &[&str] = &["Method", "Input", "Output", "Error", "Event", "Description"];

/// Legacy: the two level-3 subsections the retired `## API Contract`
/// sections had to carry, in order, and nothing else. `Protocol API` listed
/// every protocol method; `Aggregate API` listed the single `execute` entry
/// point. A third H3 — one per protocol class — was the shape
/// `api_h3_unexpected` reported. Still read by the API-contract checks until
/// the AES602 checker migration removes them.
pub const API_CONTRACT_SUBSECTIONS: &[&str] = &["Protocol API", "Aggregate API"];

/// Legacy columns of the retired `## Integration Points` table; still read
/// by the AES602 integration-shape check until that check is removed with
/// the rest of the old FRD shape.
pub const INTEGRATION_COLUMNS: &[&str] = &["System", "Direction", "Purpose", "Failure mode"];

/// Legacy columns of the retired `## Non-functional Requirements` table;
/// still read by the AES602 NFR-shape check until that check is removed with
/// the rest of the old FRD shape.
pub const NFR_COLUMNS: &[&str] = &["Metric", "Target", "Measurement method"];

/// File extensions a spec must never name, per HOW-TO-MAKE-FRD Rule 9.
pub const SOURCE_EXTENSIONS: &[&str] = &["py", "rs", "ts", "tsx"];

// ─── AES606 — Template body-shape conformance ──────────────────────────────
// Table shapes of the 9-doc layout, transcribed from the fenced template of
// each `HOW-TO-MAKE-*.md`. `ASPECT_DETAIL_COLUMNS` is the shared
// `| Aspect | Detail |` header; the per-doc sets below are its fixed rows
// and the per-section headers. The AES606 entry-table checks that consume
// them arrive with the checker migration; the tests at the bottom of this
// module pin each set to its source template until then.

/// Shared `| Aspect | Detail |` header of every entry table.
pub const ASPECT_DETAIL_COLUMNS: &[&str] = &["Aspect", "Detail"];

/// Shared `| Artifact | Purpose | Link |` header of every document's
/// `## Reference` table, per the HOW-TO-MAKE templates.
pub const REFERENCE_COLUMNS: &[&str] = &["Artifact", "Purpose", "Link"];

/// PRD `Project Requirements` entry row labels — 6, per HOW-TO-MAKE-PRD.md.
pub const PRD_ROW_LABELS: &[&str] = &[
    "Description",
    "User Story",
    "In Scope",
    "Out of Scope",
    "Acceptance",
    "Success Metric",
];

/// ROADMAP `Roadmap Status` entry row labels — 4, per HOW-TO-MAKE-ROADMAP.md.
pub const ROADMAP_ROW_LABELS: &[&str] = &["Priority", "State", "Dependencies", "Updated"];

/// BACKLOG `Backlog Status` entry row labels — 7, per HOW-TO-MAKE-BACKLOG.md.
pub const BACKLOG_ROW_LABELS: &[&str] = &[
    "State",
    "Done",
    "In Progress",
    "Blocked",
    "Next",
    "Risk",
    "Updated",
];

/// FRD `Functional Requirements` entry row labels — 12. Alias of `FR_FIELDS`:
/// both are the same twelve labels the entry table in HOW-TO-MAKE-FRD.md
/// ships, so the AES601 field list and the AES606 row check cannot drift.
pub const FRD_ROW_LABELS: &[&str] = FR_FIELDS;

/// AGENTS `Runtime` entry row labels — 7, per HOW-TO-MAKE-AGENTS.md.
pub const AGENTS_RUNTIME_ROW_LABELS: &[&str] = &[
    "Language",
    "Environment",
    "Artifacts",
    "Package Manager",
    "Version Probe",
    "Setup",
    "Activation",
];

/// Header-based table sets per document (AES606 header checks, not row checks).
pub const AGENTS_PATH_PURPOSE_COLUMNS: &[&str] = &["Path", "Purpose"];
pub const AGENTS_COMMAND_COLUMNS: &[&str] = &["Category", "Command", "Scope", "CI Gate"];
pub const AGENTS_DOCUMENT_AGENT_COLUMNS: &[&str] = &["Document", "Agent Must"];
pub const README_QUICK_START_COLUMNS: &[&str] = &["Step", "Action", "Command"];
pub const CONTRIBUTING_WORKFLOW_COLUMNS: &[&str] = &["Step", "Action", "Detail"];
pub const CONTRIBUTING_COMMIT_COLUMNS: &[&str] = &["Type", "Usage", "Example"];
pub const DATA_TYPE_DESCRIPTION_COLUMNS: &[&str] = &["Type", "Description"];
pub const DESIGN_VARIANT_VALUE_COLUMNS: &[&str] = &["Variant", "Use case", "Token", "Value"];

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
/// HOW-TO-MAKE-*.md template. The `required` set is mandatory; the
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
/// `skills/aes-docs/references/HOW-TO-MAKE-DESIGN.md`, and `allowed` is empty
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
    (FRD_DOC, FRD_REQUIRED_H2S, &[]),
    (BACKLOG_DOC, BACKLOG_REQUIRED_H2S, &[]),
    (DATA_DOC, DATA_REQUIRED_H2S, &["DT-"]),
    (DESIGN_DOC, DESIGN_REQUIRED_H2S, &[]),
];

// ─── AES606 (expanded) — member docs & template body ─────────────────────
/// Missing FRD.md / BACKLOG.md / DATA.md / DESIGN.md in a member folder.
pub const MEMBER_DOC_VIOLATION: &str = "missing_member_doc";

/// A template placeholder token survived non-prose stripping on a document.
pub const TEMPLATE_BODY_VIOLATION_UNFILLED_PLACEHOLDER: &str = "unfilled_placeholder";

/// A non-deletable template section is entirely absent from a document.
pub const TEMPLATE_BODY_VIOLATION_DELETED_SECTION: &str = "section_deleted";

// Re-export template conformance types from the template module so that
// `shared_doc_rules::taxonomy_doc_rules_constant` remains the canonical import.
pub use crate::taxonomy_doc_rules_template_vo::{TEMPLATE_PROFILES, TemplateProfile};

/// A reference template from `crates/shared/skills/aes-docs/references/`,
/// for the tests that pin constants and catalogs to their source templates.
#[cfg(test)]
pub fn load_reference_template(file: &str) -> String {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../skills/aes-docs/references")
        .join(file);
    std::fs::read_to_string(&path).unwrap_or_else(|e| {
        panic!(
            "reference template {} must be readable: {e}",
            path.display()
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// *columns* must all appear somewhere in the reference template *file*.
    fn ships_columns(file: &str, columns: &[&str]) {
        let text = load_reference_template(file);
        for column in columns {
            assert!(
                text.contains(*column),
                "{file} must ship the `{column}` label"
            );
        }
    }

    /// The 14 AES606 row-label / column constants must map to a live
    /// reference template: each set transcribes a table the shipped
    /// HOW-TO-MAKE template actually contains, so a constant with no
    /// template behind it fails here instead of silently drifting.
    #[test]
    fn row_label_sets_transcribe_their_reference_templates() {
        ships_columns("HOW-TO-MAKE-PRD.md", PRD_ROW_LABELS);
        ships_columns("HOW-TO-MAKE-ROADMAP.md", ROADMAP_ROW_LABELS);
        ships_columns("HOW-TO-MAKE-BACKLOG.md", BACKLOG_ROW_LABELS);
        ships_columns("HOW-TO-MAKE-FRD.md", FRD_ROW_LABELS);
        ships_columns("HOW-TO-MAKE-FRD.md", ASPECT_DETAIL_COLUMNS);
        ships_columns("HOW-TO-MAKE-AGENTS.md", AGENTS_RUNTIME_ROW_LABELS);
    }

    #[test]
    fn header_column_sets_transcribe_their_reference_templates() {
        ships_columns("HOW-TO-MAKE-AGENTS.md", AGENTS_PATH_PURPOSE_COLUMNS);
        ships_columns("HOW-TO-MAKE-AGENTS.md", AGENTS_COMMAND_COLUMNS);
        ships_columns("HOW-TO-MAKE-AGENTS.md", AGENTS_DOCUMENT_AGENT_COLUMNS);
        ships_columns("HOW-TO-MAKE-README.md", README_QUICK_START_COLUMNS);
        ships_columns("HOW-TO-MAKE-CONTRIBUTING.md", CONTRIBUTING_WORKFLOW_COLUMNS);
        ships_columns("HOW-TO-MAKE-CONTRIBUTING.md", CONTRIBUTING_COMMIT_COLUMNS);
        ships_columns("HOW-TO-MAKE-DATA.md", DATA_TYPE_DESCRIPTION_COLUMNS);
        ships_columns("HOW-TO-MAKE-DESIGN.md", DESIGN_VARIANT_VALUE_COLUMNS);
        ships_columns("HOW-TO-MAKE-AGENTS.md", REFERENCE_COLUMNS);
    }

    /// The AES601 new violation codes added by the PR-ID family and the
    /// BACKLOG qualifier-alignment decisions must map to templates that
    /// ship the corresponding ID / qualifier shapes, so a code with no
    /// template behind it fails here instead of silently drifting.
    #[test]
    fn pr_id_and_backlog_alignment_codes_map_to_their_templates() {
        let prd = load_reference_template("HOW-TO-MAKE-PRD.md");
        let roadmap = load_reference_template("HOW-TO-MAKE-ROADMAP.md");
        let backlog = load_reference_template("HOW-TO-MAKE-BACKLOG.md");
        assert!(
            prd.contains("PR-<PROJECT-NAME-ID>"),
            "HOW-TO-MAKE-PRD.md ships the `PR-` item shape"
        );
        assert!(
            roadmap.contains("PR-<PROJECT-NAME-ID>"),
            "HOW-TO-MAKE-ROADMAP.md ships the `PR-` item shape"
        );
        assert!(
            backlog.contains("PROJECT-NAME-ID")
                && backlog.contains("FEATURE-NAME-ID")
                && backlog.contains("DESIGN-NAME-ID"),
            "HOW-TO-MAKE-BACKLOG.md ships the qualifier disjunction"
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
        assert_eq!(FRD_ROW_LABELS.len(), 12, "twelve labels, one per row");
    }
}
