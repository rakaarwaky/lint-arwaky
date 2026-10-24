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
pub const MASTER_ONLY_SECTIONS: &[&str] = &[
    "State Definitions",
    "Status Policy",
    "Feature Roll-up",
    "Risk Register",
];

/// A folder is a feature folder only when it holds an orchestrator.
pub const ORCHESTRATOR_SUFFIX: &str = "_orchestrator";

/// Kernel folders are not features and carry DATA.md + BACKLOG.md.
pub const KERNEL_DIR: &str = "shared";

/// Per-crate section order for `##` headings, per HOW-TO-MAKE-FRD.
pub const FRD_REQUIRED_H2S: &[&str] = &[
    "Reference",
    "System Overview",
    "Functional Requirements",
    "API Contract",
    "Integration Points",
    "Non-functional Requirements",
    "Test Scenarios",
    "Assumptions & Constraints",
    "Glossary",
];

/// Per-crate BACKLOG section order, per HOW-TO-MAKE-BACKLOG.
pub const BACKLOG_REQUIRED_H2S: &[&str] = &[
    "Current Condition",
    "Backlog",
    "Scenario Evidence",
    "Blockers",
    "Dependencies",
    "Release Readiness",
    "Deferred",
    "Change Log",
];

/// Template section order, per HOW-TO-MAKE-FRD.
pub const FRD_SECTION_ORDER: &[&str] = &[
    "Reference",
    "System Overview",
    "Functional Requirements",
    "API Contract",
    "Integration Points",
    "Non-functional",
    "Test Scenarios",
    "Assumptions",
    "Glossary",
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

/// The six fields every requirement must state, per HOW-TO-MAKE-FRD Rule 2.
pub const FR_FIELDS: &[&str] = &[
    "Description",
    "Input",
    "Output",
    "Business Rules",
    "Edge Cases",
    "Error Handling",
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
// The columns each mandated section must carry, transcribed verbatim from
// the fenced templates in `HOW-TO-MAKE-BACKLOG.md` and `HOW-TO-MAKE-ROADMAP.md`.
// These are the table shapes AES606 enforces: a section present but carrying
// the wrong body shape (bullets, prose, or a table missing a required column)
// reports `section_not_table` or `table_missing_columns`.

/// The `## Backlog` table of a BACKLOG.md.
pub const BACKLOG_TABLE_COLUMNS: &[&str] = &[
    "ID",
    "Priority",
    "State",
    "Dependencies",
    "Next Action",
    "Updated",
];

/// The `## Scenario Evidence` table of a BACKLOG.md.
pub const SCENARIO_EVIDENCE_COLUMNS: &[&str] = &[
    "Scenario",
    "Kind",
    "Test file",
    "Test name",
    "Last verified",
];

/// The `## Release Readiness` table of a BACKLOG.md.
pub const RELEASE_READINESS_COLUMNS: &[&str] = &["Area", "Status", "Notes"];

/// The `## Change Log` table of a BACKLOG.md.
pub const CHANGE_LOG_COLUMNS: &[&str] = &["Date", "Change", "By"];

/// The `## State Definitions` table of a ROADMAP.md.
pub const STATE_DEFINITIONS_COLUMNS: &[&str] = &["State", "Meaning"];

/// The `## Feature Roll-up` table of a ROADMAP.md. Columns:
/// ID | Feature | Priority | State | Dependencies | Next Action | Updated.
/// PR #1113 removed the `Health` column and dropped `Spec`/`Backlog` in
/// favor of `Feature`, `Dependencies`, and `Next Action`, so the enforced
/// shape is the seven-column one, not the ten-column template shape.
pub const FEATURE_ROLLUP_COLUMNS: &[&str] = &[
    "ID",
    "Feature",
    "Priority",
    "State",
    "Dependencies",
    "Next Action",
    "Updated",
];

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
/// The `DESIGN_DOC` entry is transcribed verbatim from the fenced template in
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
            "Runtime",
            "Project Quick Facts",
            "Pipeline",
            "Project Structure",
            "Commands",
            "Related Documents",
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
            "Principles",
            "Development Setup",
            "Quality Verification & PR Process",
        ],
        &[],
    ),
    (
        PRD_DOC,
        &[
            "Problem Statement",
            "Goals & Success Metrics",
            "User Personas",
            "Scope",
            "Feature Requirements",
            "Non-functional Requirements",
            "Open Questions / Risks",
        ],
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
            "Prerequisites",
            "Quick Start",
            "Architecture",
            "Available Scripts/Commands",
            "Configuration",
            "Testing",
            "Contributing",
            "License",
        ],
        &[],
    ),
    (
        ROADMAP_DOC,
        &[
            "Current Condition",
            "State Definitions",
            "Feature Roll-up",
            "Risk Register",
        ],
        &[],
    ),
    (
        FRD_DOC,
        &[
            "Reference",
            "System Overview",
            "Functional Requirements",
            "API Contract",
            "Integration Points",
            "Non-functional Requirements",
            "Test Scenarios",
            "Assumptions & Constraints",
            "Glossary",
        ],
        &[],
    ),
    (
        BACKLOG_DOC,
        &[
            "Current Condition",
            "Backlog",
            "Scenario Evidence",
            "Blockers",
            "Dependencies",
            "Release Readiness",
            "Deferred",
            "Change Log",
        ],
        &[],
    ),
    (
        DATA_DOC,
        &[
            "Reference",
            "Data Overview",
            "Data Domain",
            "Assumptions & Constraints",
        ],
        &[],
    ),
    (
        DESIGN_DOC,
        &["Brand & Style", "Components", "Reference"],
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

/// Template style for placeholder tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaceholderStyle {
    /// AGENTS.md uses {...} slots plus frontmatter.
    CurlySlots,
    /// Most other templates use <...> angle tokens.
    AngleToken,
}

/// A per-template strictness profile with a hard-coded placeholder catalog.
///
/// The catalog is the decision boundary (Decision Q3): surviving prose tokens
/// are matched only against these known markers, so language constructs such
/// as `Vec<DocFinding>` never fire a false positive.
pub struct TemplateProfile {
    pub doc_name: &'static str,
    pub template_file: &'static str,
    pub placeholder_style: PlaceholderStyle,
    pub deletable_sections: &'static [&'static str],
    pub required_h2s: &'static [&'static str],
    pub placeholder_catalog: &'static [&'static str],
}

/// Hard-coded per-template placeholder catalogs (Decision Q3).
///
/// Each entry mirrors the fenced template in
/// `crates/shared/skills/aes-docs/references/HOW-TO-MAKE-<doc_name>.md`. The
/// catalog lists the placeholder tokens the template ships unfilled, so a
/// draft copied verbatim from the template is caught by FR-DOC-009 rather
/// than passing silently.
pub const TEMPLATE_PROFILES: &[TemplateProfile] = &[
    TemplateProfile {
        doc_name: FRD_DOC,
        template_file: "HOW-TO-MAKE-FRD.md",
        placeholder_style: PlaceholderStyle::AngleToken,
        deletable_sections: &[],
        required_h2s: FRD_REQUIRED_H2S,
        placeholder_catalog: &[
            "<feature-name>",
            "<FEATURENAME>",
            "<YYYY-MM-DD>",
            "<link to root PRD.md>",
            "<One diagram or ≤ 3 sentences...>",
            "<None | item>",
            "<what it does — one sentence>",
            "<shape, source>",
            "<N>",
        ],
    },
    TemplateProfile {
        doc_name: BACKLOG_DOC,
        template_file: "HOW-TO-MAKE-BACKLOG.md",
        placeholder_style: PlaceholderStyle::AngleToken,
        deletable_sections: &[],
        required_h2s: BACKLOG_REQUIRED_H2S,
        placeholder_catalog: &[
            "<feature-name>",
            "<YYYY-MM-DD>",
            "<N>",
            "<one line>",
            "<fix target>",
        ],
    },
    TemplateProfile {
        doc_name: AGENTS_DOC,
        template_file: "HOW-TO-MAKE-AGENTS.md",
        placeholder_style: PlaceholderStyle::CurlySlots,
        deletable_sections: &[],
        required_h2s: &[
            "Runtime",
            "Project Quick Facts",
            "Pipeline",
            "Project Structure",
            "Commands",
            "Related Documents",
        ],
        placeholder_catalog: &[
            "{architecture scanner command}",
            "{environment manager and creation method}",
            "{test runner invocation}",
            "{lint command}",
            "{type-check command}",
        ],
    },
    TemplateProfile {
        doc_name: PRD_DOC,
        template_file: "HOW-TO-MAKE-PRD.md",
        placeholder_style: PlaceholderStyle::AngleToken,
        deletable_sections: &[],
        required_h2s: &[
            "Problem Statement",
            "Goals & Success Metrics",
            "User Personas",
            "Scope",
            "Feature Requirements",
            "Non-functional Requirements",
            "Open Questions / Risks",
        ],
        placeholder_catalog: &[
            "<feature-name>",
            "<YYYY-MM-DD>",
            "<link to root FRD.md>",
            "<one sentence>",
        ],
    },
    TemplateProfile {
        doc_name: ROADMAP_DOC,
        template_file: "HOW-TO-MAKE-ROADMAP.md",
        placeholder_style: PlaceholderStyle::AngleToken,
        deletable_sections: &[],
        required_h2s: &[
            "Current Condition",
            "State Definitions",
            "Feature Roll-up",
            "Risk Register",
        ],
        placeholder_catalog: &["<YYYY-MM-DD>", "<feature-name>", "<N>"],
    },
    TemplateProfile {
        doc_name: README_DOC,
        template_file: "HOW-TO-MAKE-README.md",
        placeholder_style: PlaceholderStyle::AngleToken,
        deletable_sections: &[],
        required_h2s: &[
            "Prerequisites",
            "Quick Start",
            "Architecture",
            "Available Scripts/Commands",
            "Configuration",
            "Testing",
            "Contributing",
            "License",
        ],
        placeholder_catalog: &["<feature-name>", "<YYYY-MM-DD>", "<one line>"],
    },
    TemplateProfile {
        doc_name: ARCHITECTURE_DOC,
        template_file: "HOW-TO-MAKE-ARCHITECTURE.md",
        placeholder_style: PlaceholderStyle::AngleToken,
        deletable_sections: &[],
        required_h2s: &[
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
        placeholder_catalog: &["<feature-name>", "<YYYY-MM-DD>", "<one line>"],
    },
    TemplateProfile {
        doc_name: CONTRIBUTING_DOC,
        template_file: "HOW-TO-MAKE-CONTRIBUTING.md",
        placeholder_style: PlaceholderStyle::AngleToken,
        deletable_sections: &["Quality Verification & PR Process"],
        required_h2s: &[
            "Principles",
            "Development Setup",
            "Quality Verification & PR Process",
        ],
        placeholder_catalog: &["<YYYY-MM-DD>", "<link to CODEOWNERS>", "<N>"],
    },
    TemplateProfile {
        doc_name: DATA_DOC,
        template_file: "HOW-TO-MAKE-DATA.md",
        placeholder_style: PlaceholderStyle::AngleToken,
        deletable_sections: &[],
        required_h2s: &[
            "Reference",
            "Data Overview",
            "Data Domain",
            "Assumptions & Constraints",
        ],
        placeholder_catalog: &["<feature-name>", "<YYYY-MM-DD>", "<one line>"],
    },
    TemplateProfile {
        doc_name: DESIGN_DOC,
        template_file: "HOW-TO-MAKE-DESIGN.md",
        placeholder_style: PlaceholderStyle::AngleToken,
        deletable_sections: &[],
        required_h2s: &["Brand & Style", "Components", "Reference"],
        placeholder_catalog: &["<feature-name>", "<YYYY-MM-DD>", "<one line>"],
    },
];
