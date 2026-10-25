// PURPOSE: template conformance types and profiles for AES606

use crate::taxonomy_doc_rules_constant::{
    AGENTS_DOC, ARCHITECTURE_DOC, BACKLOG_DOC, BACKLOG_REQUIRED_H2S, CONTRIBUTING_DOC, DATA_DOC,
    DESIGN_DOC, FRD_DOC, FRD_REQUIRED_H2S, PRD_DOC, README_DOC, ROADMAP_DOC,
};

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
#[derive(Debug, Clone)]
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
