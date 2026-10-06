// PURPOSE: doc invariant rule codes, violation types, and shared vocabulary for the doc-rules feature

/// ─── AES601 — FR format ─────────────────────────────────────────────────────
/// Violation types for FR-ID and FR-field violations.
pub const RULE_CODE_FR_FORMAT: &str = "AES601";

/// FR-ID is bare (`FR-NNN`) without a feature prefix.
pub const FR_ID_VIOLATION_MISSING_FEATURE_PREFIX: &str = "id_missing_feature_prefix";

/// FR-ID does not read as an imperative action.
pub const FR_ID_VIOLATION_NOT_IMPERATIVE: &str = "id_not_imperative";

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

/// A scenario in the FRD has no corresponding evidence row in the backlog.
pub const CROSSLINKS_VIOLATION_SCENARIO_NO_EVIDENCE: &str = "scenario_no_evidence";

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
            "User Context",
            "Precedence",
            "Security",
            "Session Start",
            "Runtime",
            "Quick Facts",
            "Pipeline",
            "Git Workflow",
            "Commands",
            "Guided Skills",
            "Definition of Done",
            "Writing Style",
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
        &["Dependency Policy"],
    ),
    (
        CONTRIBUTING_DOC,
        &[
            "Principles",
            "Development Setup",
            "Feature Change",
            "Documentation Change",
            "Quality Verification & PR Process",
        ],
        &[
            "Prerequisites",
            "Running the binaries",
            "Branch Management",
            "Versioning Policy",
            "Issue Closure Policy",
            "Why Contribute",
            "Questions?",
        ],
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
        &[
            "Product Decisions",
            "Exit Code Contract",
            "AES Rule Summary",
            "Feature Map",
            "Reference",
        ],
    ),
    (
        README_DOC,
        &[
            "Prerequisites",
            "Quick Start",
            "Architecture",
            "Project Structure",
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
            "Status Policy",
            "Feature Roll-up",
            "Risk Register",
        ],
        &[
            "Health Definitions",
            "Backlog",
            "Blockers",
            "Dependencies",
            "Release Readiness",
            "Deferred",
            "Change Log",
        ],
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
