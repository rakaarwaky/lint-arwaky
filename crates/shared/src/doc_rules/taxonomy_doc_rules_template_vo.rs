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
            "<One diagram or ≤ 3 sentences...>",
            "<what it does — one sentence>",
            "<validation logic, constraints>",
            "<boundary conditions and their handling>",
            "<capability name>",
            "<protocol trait / interface>",
            "<capability method>",
            "<shape, source>",
            "<shape, destination>",
            "<what fails, what the caller sees>",
            "<domain event emitted, if any>",
            "<observable behaviour + expected result>",
            "<target value>",
            "<agent name>",
            "<aggregate name>",
            "<execute>",
            "<request>",
            "<response>",
            "<Business goals, user stories, and scope>",
            "<Task tracking, epics, and implementation status>",
            "<Timeline, phasing, and dependencies>",
            "<System boundaries, tech stack, and infra>",
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
            "<One paragraph of progress summary.>",
            "<evidence>",
            "<None / item name>",
            "<open row IDs>",
            "<what ships broken / None>",
            "<YYYY-MM-DD>",
            "<Business goals and scope>",
            "<Functional requirements specification>",
            "<Timeline, phasing, and dependencies>",
            "<System boundaries, tech stack, and infra>",
        ],
    },
    TemplateProfile {
        doc_name: AGENTS_DOC,
        template_file: "HOW-TO-MAKE-AGENTS.md",
        placeholder_style: PlaceholderStyle::CurlySlots,
        deletable_sections: &[],
        required_h2s: &[
            "Agent Overview",
            "Runtime",
            "Project Structure",
            "Commands",
            "Documentation Awareness",
            "Reference",
        ],
        placeholder_catalog: &[
            "{project-name}",
            "{language and pinned version}",
            "{environment manager and creation method}",
            "{reviewable artifact location}",
            "{package manager and lockfile}",
            "{version probe command}",
            "{environment setup command}",
            "{optional environment activation command}",
            "{path}",
            "{what lives here}",
            "{whole-workspace test command}",
            "{single-package test command}",
            "{single-file test command}",
            "{formatter or linter command}",
            "{architecture scanner command}",
            "{Business goals, user stories, and scope}",
            "{Functional requirements specification}",
            "{Data shapes and kernel definitions}",
            "{UI/UX and visual system}",
            "{System boundaries, tech stack, and infra}",
        ],
    },
    TemplateProfile {
        doc_name: PRD_DOC,
        template_file: "HOW-TO-MAKE-PRD.md",
        placeholder_style: PlaceholderStyle::AngleToken,
        deletable_sections: &[],
        required_h2s: &["Problem Statement", "Project Requirements", "Reference"],
        placeholder_catalog: &[
            "<project-name>",
            "<one behaviour — what it does>",
            "<as a..., I want..., so that...>",
            "<boundary — what this project will deliver>",
            "<explicit exclusions — name them so the argument ends here>",
            "<observable check>",
            "<how to measure this requirement succeeds>",
            "<Timeline, phasing, and dependencies>",
            "<System boundaries, tech stack, and infra>",
        ],
    },
    TemplateProfile {
        doc_name: ROADMAP_DOC,
        template_file: "HOW-TO-MAKE-ROADMAP.md",
        placeholder_style: PlaceholderStyle::AngleToken,
        deletable_sections: &[],
        required_h2s: &["Progress Overview", "Roadmap Status", "Reference"],
        placeholder_catalog: &[
            "<workspace name>",
            "<One paragraph of progress summary.>",
            "<Short imperative name>",
            "<DATA-NAME-ID> / <FEATURE-NAME-ID> / <DESIGN-NAME-ID>",
            "<YYYY-MM-DD>",
            "<Business goals, user stories, and scope>",
            "<System boundaries, tech stack, and infra>",
        ],
    },
    TemplateProfile {
        doc_name: README_DOC,
        template_file: "HOW-TO-MAKE-README.md",
        placeholder_style: PlaceholderStyle::AngleToken,
        deletable_sections: &[],
        required_h2s: &[
            "Project Overview",
            "Prerequisites",
            "Quick Start",
            "Reference",
        ],
        placeholder_catalog: &["<project-name>", "<requiretment>"],
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
        deletable_sections: &[],
        required_h2s: &[
            "Contribution Overview",
            "Contribution Guidelines",
            "Workflow",
            "Commit Convention",
            "Reference",
        ],
        placeholder_catalog: &[
            "<project-name>",
            "<branch-naming-convention>",
            "<reviewer / approval criteria>",
            "<merge strategy: squash / rebase / merge>",
            "<Business goals, user stories, and scope>",
            "<System boundaries, tech stack, and infra>",
        ],
    },
    // The DATA and DESIGN templates ship their only placeholders inside H2/H3
    // heading text, where the angle-token strip blanks them, so their
    // placeholder catalog is intentionally empty: AES606 section-deletion plus
    // the AES605 H2 contract enforce template fidelity for these two docs.
    TemplateProfile {
        doc_name: DATA_DOC,
        template_file: "HOW-TO-MAKE-DATA.md",
        placeholder_style: PlaceholderStyle::AngleToken,
        deletable_sections: &[],
        required_h2s: &["Data Overview", "Data Requirements", "Reference"],
        placeholder_catalog: &[],
    },
    TemplateProfile {
        doc_name: DESIGN_DOC,
        template_file: "HOW-TO-MAKE-DESIGN.md",
        placeholder_style: PlaceholderStyle::AngleToken,
        deletable_sections: &[],
        required_h2s: &["Design Overview", "Design Requirements", "Reference"],
        placeholder_catalog: &[],
    },
];
