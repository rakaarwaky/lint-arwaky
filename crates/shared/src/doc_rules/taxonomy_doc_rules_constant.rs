// PURPOSE: doc invariant rule codes, violation types, and shared vocabulary for the doc-rules feature

/// ─── AES601 — FR format ─────────────────────────────────────────────────────
/// Violation types for FR-ID and FR-field violations.
pub const RULE_CODE_FR_FORMAT: &str = "AES601";

/// FR-ID is bare (`FR-NNN`) without a feature prefix.
pub const FR_ID_VIOLATION_MISSING_FEATURE_PREFIX: &str = "id_missing_feature_prefix";

/// A required FR field is absent from a requirement block.
pub const FR_FIELDS_VIOLATION_FIELD_MISSING: &str = "field_missing";

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

/// Per-crate FRD section order, per HOW-TO-MAKE-FRD.
pub const FRD_REQUIRED_H2S: &[&str] = &[
    "Feature Overview",
    "Functional Requirements",
    "Public Interface",
    "Reference",
];

/// Per-crate BACKLOG section order, per HOW-TO-MAKE-BACKLOG.
pub const BACKLOG_REQUIRED_H2S: &[&str] = &["Progress Overview", "Backlog Status", "Reference"];

/// Template section order, per HOW-TO-MAKE-FRD.
pub const FRD_SECTION_ORDER: &[&str] = &[
    "Feature Overview",
    "Functional Requirements",
    "Public Interface",
    "Reference",
];

/// The only level-3 headings `HOW-TO-MAKE-FRD.md` sanctions, by literal
/// title. A requirement heading (`### FR-<FEATURENAME>-NNN: <name>`) is the
/// third sanctioned shape and is matched by the shared FR-ID pattern rather
/// than listed here, so the accepted set cannot drift between AES601 and
/// AES602 — the same reason `fr_id_heading_re` is shared.
///
/// The set is closed: an FRD level-3 heading outside these two titles and the
/// FR-ID shape fires `h3_off_template`. Level-4 and deeper stay free-form,
/// which is where an author puts detail that must not become a section.
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

/// Columns the API Contract tables must carry.
pub const API_COLUMNS: &[&str] = &["Method", "Input", "Output", "Error", "Event", "Description"];

/// The two level-3 subsections `## API Contract` must carry, in order, and
/// nothing else. `Protocol API` lists every protocol method; `Aggregate API`
/// lists the single `execute` entry point. A FRD that invents a third H3 —
/// one per protocol class — is the shape `api_h3_unexpected` reports.
pub const API_CONTRACT_SUBSECTIONS: &[&str] = &["Protocol API", "Aggregate API"];

/// Columns Integration Points must carry.
pub const INTEGRATION_COLUMNS: &[&str] = &["System", "Direction", "Purpose", "Failure mode"];

/// Columns Non-functional Requirements must carry.
pub const NFR_COLUMNS: &[&str] = &["Metric", "Target", "Measurement method"];

/// File extensions a spec must never name, per HOW-TO-MAKE-FRD Rule 9.
pub const SOURCE_EXTENSIONS: &[&str] = &["py", "rs", "ts", "tsx"];

// ─── AES606 — Template body-shape conformance ──────────────────────────────
// Table shapes enforced by AES606 under the new 9-doc layout. `ASPECT_DETAIL_COLUMNS`
// is the shared `| Aspect | Detail |` header; per-doc row-label sets below are
// enforced via `entry_missing_row` on each entry's entry table.

/// Shared `| Aspect | Detail |` header of every entry table.
pub const ASPECT_DETAIL_COLUMNS: &[&str] = &["Aspect", "Detail"];

/// PRD `Project Requirements` entry row labels — 6 per the target spec.
pub const PRD_ROW_LABELS: &[&str] = &[
    "Description",
    "User Story",
    "In Scope",
    "Out of Scope",
    "Acceptance",
    "Success Metric",
];

/// ROADMAP `Roadmap Status` entry row labels — 4 per the target spec.
pub const ROADMAP_ROW_LABELS: &[&str] = &["Priority", "State", "Dependencies", "Updated"];

/// BACKLOG `Backlog Status` entry row labels — 7 per the target spec.
pub const BACKLOG_ROW_LABELS: &[&str] = &[
    "State",
    "Done",
    "In Progress",
    "Blocked",
    "Next",
    "Risk",
    "Updated",
];

/// FRD `Functional Requirements` entry row labels — 12 per the target spec.
pub const FRD_ROW_LABELS: &[&str] = &[
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

/// AGENTS `Runtime` entry row labels — 7 per the target spec.
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
    (
        AGENTS_DOC,
        &[
            "Agent Overview",
            "Runtime",
            "Project Structure",
            "Commands",
            "Documentation Awareness",
            "Reference",
        ],
        &[],
    ),
    (
        ARCHITECTURE_DOC,
        &[
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
        ],
        &[],
    ),
    (
        CONTRIBUTING_DOC,
        &[
            "Contribution Overview",
            "Contribution Guidelines",
            "Workflow",
            "Commit Convention",
            "Reference",
        ],
        &[],
    ),
    (
        PRD_DOC,
        &["Problem Statement", "Project Requirements", "Reference"],
        &[],
    ),
    (
        // `allowed` is empty on purpose: `Project Structure` belongs to the
        // AGENTS.md contract, and a README that repeats it must be reported
        // rather than passed as a second source of layout truth. An empty
        // `allowed` makes the set closed, so the heading fires
        // `h2_unexpected` and has to be demoted or deleted.
        README_DOC,
        &[
            "Project Overview",
            "Prerequisites",
            "Quick Start",
            "Reference",
        ],
        &[],
    ),
    (
        ROADMAP_DOC,
        &["Progress Overview", "Roadmap Status", "Reference"],
        &[],
    ),
    (
        FRD_DOC,
        &[
            "Feature Overview",
            "Functional Requirements",
            "Public Interface",
            "Reference",
        ],
        &[],
    ),
    (
        BACKLOG_DOC,
        &["Progress Overview", "Backlog Status", "Reference"],
        &[],
    ),
    (
        DATA_DOC,
        &["Data Overview", "Data Requirements", "Reference"],
        &["DT-"],
    ),
    (
        DESIGN_DOC,
        &["Design Overview", "Design Requirements", "Reference"],
        &[],
    ),
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
pub use crate::taxonomy_doc_rules_template_vo::{
    PlaceholderStyle, TEMPLATE_PROFILES, TemplateProfile,
};
