// PURPOSE: Defect value objects for AES601-AES610 doc checking rules.
// Extracted to taxonomy layer to satisfy AES404 (stateless utility functions).


// ─── From utility_capabilities_doc_h2_checks ───────────────────────────────
/// Why a section's table shape failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SectionShapeDefectKind {
    /// Table ships columns outside the template's fixed header set.
    ExtraColumns,
    /// A data row repeats the header row.
    HeaderRepeated,
    /// Table is present but missing required column(s).
    MissingColumns,
    /// The section body carries no table at all.
    NotTable,
}

/// A detected table-shape defect in a section.
#[derive(Debug, Clone)]
pub struct SectionShapeDefect {
    pub line: usize,
    pub marker: &'static str,
    pub title: String,
    pub columns: Vec<String>,
    pub kind: SectionShapeDefectKind,
    pub extra_columns: Vec<String>,
}

/// A detected fixed-row defect in an H2-level `| Aspect | Detail |` table.
#[derive(Debug, Clone)]
pub struct H2FixedRowsDefect {
    pub line: usize,
    pub title: String,
    pub missing: Vec<String>,
    pub extra: Vec<String>,
}

/// H2-level: the section titled *title* must carry a table whose header


// ─── From utility_doc_aspect_entry_checks ───────────────────────────────────
/// A defect detected by `detect_aspect_entry_tables` / `detect_aspect_table`.
#[derive(Debug, Clone)]
pub enum AspectEntryDefect {
    /// Expected aspect H4 is missing under the entry H3.
    MissingAspect {
        line: usize,
        h3_title: String,
        aspect: String,
    },
    /// H4 aspect subsections are out of canonical order under the entry H3.
    OrderMismatch {
        line: usize,
        h3_title: String,
        aspects: Vec<String>,
    },
    /// PRD aspect H4 carries an empty body.
    EmptyBody {
        line: usize,
        aspect: String,
        h3_title: String,
    },
    /// Aspect H4 body is not a markdown table.
    NotATable {
        line: usize,
        aspect: String,
        h3_title: String,
    },
    /// Aspect H4 table header is missing required columns.
    HeaderMissing { line: usize, aspect: String },
    /// Aspect H4 table header carries extra columns.
    HeaderExtra {
        line: usize,
        aspect: String,
        extras: Vec<String>,
    },
    /// Rows are not numbered 1..N contiguously.
    RowNumbering {
        line: usize,
        aspect: String,
        h3_title: String,
    },
    /// Bounded aspect row count differs from seed_min.
    RowCount {
        line: usize,
        aspect: String,
        h3_title: String,
        total: usize,
        seed_min: usize,
    },
    /// Open-ended aspect row count is below its minimum.
    RowMinimum {
        line: usize,
        aspect: String,
        h3_title: String,
        total: usize,
        min: usize,
    },
    /// A numbered row's Detail cell is empty or a placeholder dash.
    EmptyDetailCell {
        line: usize,
        aspect: String,
        row_no: String,
    },
}

impl AspectEntryDefect {
    /// The anchor line of this defect, for `DocFinding` construction by the
    /// calling capability.
    pub fn line(&self) -> usize {
        match self {
            AspectEntryDefect::MissingAspect { line, .. }
            | AspectEntryDefect::OrderMismatch { line, .. }
            | AspectEntryDefect::EmptyBody { line, .. }
            | AspectEntryDefect::NotATable { line, .. }
            | AspectEntryDefect::HeaderMissing { line, .. }
            | AspectEntryDefect::HeaderExtra { line, .. }
            | AspectEntryDefect::RowNumbering { line, .. }
            | AspectEntryDefect::RowCount { line, .. }
            | AspectEntryDefect::RowMinimum { line, .. }
            | AspectEntryDefect::EmptyDetailCell { line, .. } => *line,
        }
    }
}

// ─── detect functions ────────────────────────────────────────────────────────

/// Walk every entry H3 under *parent_h2* whose title matches *entry_re* and
/// detect per-aspect H4 subsection defects: presence and canonical order of
/// *aspects*, each H4's `| No | Detail |` table shape, contiguous `1..N`
/// row numbering, bounded/open-ended row counts against *seed_min*, and
/// non-empty `Detail` cells.
///
/// When *is_prd* is true, PRD aspects are free-form prose/tables per the
/// references/prd/template.md template: only presence and non-empty body are enforced.
/// When false (FRD), the strict `| No | Detail |` aspect-table contract is
/// enforced. A legacy entry carrying a single `| Aspect | Detail |` table is


// ─── From utility_doc_common_checks ─────────────────────────────────────────
/// A defect found by `detect_entry_tables`.
#[derive(Debug, Clone)]
pub enum EntryTableDefect {
    /// The entry's table shape is wrong (extra columns, repeated header,
    /// missing columns, or no table at all).
    Shape(SectionShapeDefect),
    /// The entry's first-column row labels do not match the template.
    RowLabelsMismatch {
        line: usize,
        title: String,
        missing: Vec<String>,
        extra: Vec<String>,
    },
}

impl EntryTableDefect {
    /// The anchor line of this defect, for `DocFinding` construction by the
    /// calling capability.
    pub fn line(&self) -> usize {
        match self {
            EntryTableDefect::Shape(d) => d.line,
            EntryTableDefect::RowLabelsMismatch { line, .. } => *line,
        }
    }
}

/// A defect found by `detect_doc_heading`.
#[derive(Debug, Clone)]
pub enum HeadingDefect {
    /// The document opens with a number of H1 headings other than one.
    H1Count { line: usize, count: usize },
    /// A required H2 heading is missing.
    H2Missing { line: usize, missing: Vec<String> },
    /// An H2 heading appears outside the template's required+allowed set.
    H2Unexpected {
        line: usize,
        unexpected: Vec<String>,
    },
}

impl HeadingDefect {
    pub fn line(&self) -> usize {
        match self {
            HeadingDefect::H1Count { line, .. } => *line,
            HeadingDefect::H2Missing { line, .. } => *line,
            HeadingDefect::H2Unexpected { line, .. } => *line,
        }
    }
}

/// A defect found by `detect_doc_line_drift`.
#[derive(Debug, Clone)]
pub enum LineDriftDefect {
    /// The document has a section the template does not declare.
    SectionNotInTemplate { line: usize, section: String },
    /// The document is missing a template section.
    MissingTemplateSection { line: usize, section: String },
    /// A template-fixed line does not match.
    LineMismatch {
        line: usize,
        section: String,
        expected: String,
        found: String,
    },
    /// The document H1 has no title.
    H1NoTitle { line: usize },
}

impl LineDriftDefect {
    pub fn line(&self) -> usize {
        match self {
            LineDriftDefect::SectionNotInTemplate { line, .. } => *line,
            LineDriftDefect::MissingTemplateSection { line, .. } => *line,
            LineDriftDefect::LineMismatch { line, .. } => *line,
            LineDriftDefect::H1NoTitle { line } => *line,
        }
    }
}

/// A defect found by `detect_backlog_qualifier_alignment`.
#[derive(Debug, Clone)]
pub enum BacklogQualifierDefect {
    /// BACKLOG entry token has no matching ID in the paired spec.
    TokenNotDeclared {
        line: usize,
        token: String,
        valid: Vec<String>,
    },
    /// Spec-declared ID has no BACKLOG entry.
    IdNoEntry { line: usize, id: String },
    /// BACKLOG row cites an id the paired spec does not declare.
    StrayId {
        line: usize,
        token: String,
        id: String,
        valid: Vec<String>,
    },
}

impl BacklogQualifierDefect {
    pub fn line(&self) -> usize {
        match self {
            BacklogQualifierDefect::TokenNotDeclared { line, .. } => *line,
            BacklogQualifierDefect::IdNoEntry { line, .. } => *line,
            BacklogQualifierDefect::StrayId { line, .. } => *line,
        }
    }
}

/// A defect found by `detect_member_docs`.
#[derive(Debug, Clone)]
pub enum MemberDocDefect {
    /// Member folder is missing a required specification doc.
    MissingRequiredDoc {
        doc_rel_path: String,
        line: usize,
        role_name: String,
        doc: String,
    },
    /// Member folder carries a forbidden specification doc.
    ForbiddenDoc {
        doc_rel_path: String,
        line: usize,
        role_name: String,
        doc: String,
    },
}

impl MemberDocDefect {
    pub fn line(&self) -> usize {
        match self {
            MemberDocDefect::MissingRequiredDoc { line, .. } => *line,
            MemberDocDefect::ForbiddenDoc { line, .. } => *line,
        }
    }
}

// ─── Entry tables (AES606) ───────────────────────────────────────────────

/// Walk every H3 under *parent_h2* whose title matches *entry_re* and detect
/// table-shape defects and row-label mismatches. Findings anchor to the
/// offending H3 line. The calling capability builds its own DocFinding with
/// its own AES code.
///
/// When the H3 entry carries H4 subsections (modular template shape), the
/// checks scope to the H4 whose body contains a table matching *columns* —


// ─── From utility_doc_fr_format_checks ──────────────────────────────────────
/// A defect found by `detect_fr_id_format`.
#[derive(Debug, Clone)]
pub struct FrIdDefect {
    pub line: usize,
    pub num: String,
    pub title: String,
}

impl FrIdDefect {
    pub fn line(&self) -> usize {
        self.line
    }
}

/// A defect found by `detect_pr_id_parity`.
#[derive(Debug, Clone)]
pub enum PrIdParityDefect {
    /// PR-ID declared in the PRD has no ROADMAP `Roadmap Status` entry.
    PrMissingFromRoadmap { line: usize, id: String },
    /// ROADMAP `PR-` item is not declared in the PRD.
    RoadmapNotInPrd {
        line: usize,
        id: String,
        valid: Vec<String>,
    },
}

impl PrIdParityDefect {
    pub fn line(&self) -> usize {
        match self {
            PrIdParityDefect::PrMissingFromRoadmap { line, .. } => *line,
            PrIdParityDefect::RoadmapNotInPrd { line, .. } => *line,
        }
    }
}

/// A defect found by `detect_api_method_existence`.
#[derive(Debug, Clone)]
pub struct ApiMethodDefect {
    pub line: usize,
    pub subsection: String,
    pub method: String,
}

impl ApiMethodDefect {
    pub fn line(&self) -> usize {
        self.line
    }
}

/// A defect found by `detect_fr_protocol_parity`.
#[derive(Debug, Clone)]
pub struct ProtocolParityDefect {
    pub line: usize,
    pub fr_count: usize,
    pub protocol_count: usize,
    pub direction: String,
}

impl ProtocolParityDefect {
    pub fn line(&self) -> usize {
        self.line
    }
}
