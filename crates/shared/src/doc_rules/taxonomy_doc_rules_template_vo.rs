// PURPOSE: template conformance types and profiles for AES606

use crate::taxonomy_doc_rules_constant::{
    AGENTS_DOC, AGENTS_REQUIRED_H2S, ARCHITECTURE_DOC, ARCHITECTURE_REQUIRED_H2S, BACKLOG_DOC,
    BACKLOG_REQUIRED_H2S, CONTRIBUTING_DOC, CONTRIBUTING_REQUIRED_H2S, DATA_DOC, DATA_REQUIRED_H2S,
    DESIGN_DOC, DESIGN_REQUIRED_H2S, FRD_DOC, FRD_REQUIRED_H2S, PRD_DOC, PRD_REQUIRED_H2S,
    README_DOC, README_REQUIRED_H2S, ROADMAP_DOC, ROADMAP_REQUIRED_H2S,
};

/// A per-template strictness profile with a hard-coded placeholder catalog.
///
/// The catalog is the decision boundary (Decision Q3): surviving prose tokens
/// are matched only against these known markers, so language constructs such
/// as `Vec<DocFinding>` never fire a false positive.
#[derive(Debug, Clone)]
pub struct TemplateProfile {
    pub doc_name: &'static str,
    pub template_file: &'static str,
    /// H2 titles a copied template may drop without a finding. Empty for
    /// every profile under the 9-doc layout; the field goes away together
    /// with the template-body checker branch that still reads it.
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
/// than passing silently. Every entry must appear unwrapped in that fenced
/// block: backtick-wrapped cells (command placeholders) are excluded because
/// the prose strip blanks inline code, which `matches_placeholder_catalog`
/// does not exempt. `every_catalog_entry_fires_on_a_verbatim_template_copy`
/// pins this invariant for all profiles.
pub const TEMPLATE_PROFILES: &[TemplateProfile] = &[
    TemplateProfile {
        doc_name: FRD_DOC,
        template_file: "HOW-TO-MAKE-FRD.md",
        deletable_sections: &[],
        required_h2s: FRD_REQUIRED_H2S,
        placeholder_catalog: &[
            "<feature-name>",
            "<FEATURE-NAME-ID>",
            "<Short imperative name>",
            "<One diagram or ≤ 3 sentences: where this feature sits, what calls it, what it calls.>",
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
    // The BACKLOG H3 title line ships the whole `{<PROJECT-NAME-ID> |
    // <FEATURE-NAME-ID> | DESIGN-NAME-ID}` qualifier line, so all three
    // members are catalogued: the two angle-bracketed ones plus the bare
    // `DESIGN-NAME-ID`, which the substring matcher fires on even without
    // angle brackets (curly-slot markers prove the matcher needs no `<>`).
    TemplateProfile {
        doc_name: BACKLOG_DOC,
        template_file: "HOW-TO-MAKE-BACKLOG.md",
        deletable_sections: &[],
        required_h2s: BACKLOG_REQUIRED_H2S,
        placeholder_catalog: &[
            "<feature-name>",
            "<PROJECT-NAME-ID>",
            "<FEATURE-NAME-ID>",
            "DESIGN-NAME-ID",
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
    // The migrated HOW-TO-MAKE-AGENTS.md ships its tokens as angle
    // brackets — the form that reference ships — not `{...}` slots;
    // command cells stay backtick-wrapped in the template and are
    // therefore not catalogued.
    TemplateProfile {
        doc_name: AGENTS_DOC,
        template_file: "HOW-TO-MAKE-AGENTS.md",
        deletable_sections: &[],
        required_h2s: AGENTS_REQUIRED_H2S,
        placeholder_catalog: &[
            "<project-name>",
            "<One paragraph: what AI agents must know before generating code, docs, or suggestions in this workspace.>",
            "<language and pinned version>",
            "<environment manager and creation method>",
            "<reviewable artifact location>",
            "<package manager and lockfile>",
            "<path>",
            "<what lives here>",
            "<job name>",
            "<Business goals, user stories, and scope>",
            "<Functional requirements specification>",
            "<Data shapes and kernel definitions>",
            "<UI/UX and visual system>",
            "<System boundaries, tech stack, and infra>",
        ],
    },
    TemplateProfile {
        doc_name: PRD_DOC,
        template_file: "HOW-TO-MAKE-PRD.md",
        deletable_sections: &[],
        required_h2s: PRD_REQUIRED_H2S,
        placeholder_catalog: &[
            "<project-name>",
            "<PROJECT-NAME-ID>",
            "<Short imperative name>",
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
        deletable_sections: &[],
        required_h2s: ROADMAP_REQUIRED_H2S,
        placeholder_catalog: &[
            "<workspace name>",
            "<PROJECT-NAME-ID>",
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
        deletable_sections: &[],
        required_h2s: README_REQUIRED_H2S,
        placeholder_catalog: &["<project-name>", "<requiretment>"],
    },
    // `feature-name` is intentionally uncatalogued for ARCHITECTURE: the
    // reference file's bare ``` fence at line 95 closes the markdown block
    // opened at line 60, so `modules/<feature-name>/src/` (line 97) is loose
    // prose outside the verbatim_copy extraction (lines 61-94) and never
    // reaches the catalog matcher.
    TemplateProfile {
        doc_name: ARCHITECTURE_DOC,
        template_file: "HOW-TO-MAKE-ARCHITECTURE.md",
        deletable_sections: &[],
        required_h2s: ARCHITECTURE_REQUIRED_H2S,
        placeholder_catalog: &["<Project>", "<One paragraph."],
    },
    TemplateProfile {
        doc_name: CONTRIBUTING_DOC,
        template_file: "HOW-TO-MAKE-CONTRIBUTING.md",
        deletable_sections: &[],
        required_h2s: CONTRIBUTING_REQUIRED_H2S,
        placeholder_catalog: &[
            "<project-name>",
            "<reviewer / approval criteria>",
            "<merge strategy: squash / rebase / merge>",
            "<Business goals, user stories, and scope>",
            "<System boundaries, tech stack, and infra>",
        ],
    },
    // The DATA and DESIGN catalogs carry the single-line tokens their
    // fenced templates ship unfilled: the H1 `<project-name>`, the
    // H3 item-title tokens (`<Short imperative name>`, the `-NNN: `
    // ID qualifiers `<DATA-NAME-ID>` / `<DESIGN-NAME-ID>`), the
    // Reference-table purpose cells, and — for DESIGN — the overview
    // `<One paragraph of what the design looks like.>`, which its
    // template ships as one complete line. The DATA overview stays
    // excluded: its `<One paragraph: ...>` span genuinely runs across
    // two lines, and the line-scoped matcher only sees single-line
    // substrings.
    TemplateProfile {
        doc_name: DATA_DOC,
        template_file: "HOW-TO-MAKE-DATA.md",
        deletable_sections: &[],
        required_h2s: DATA_REQUIRED_H2S,
        placeholder_catalog: &[
            "<project-name>",
            "<Short imperative name>",
            "<DATA-NAME-ID>",
            "<Business goals, user stories, and scope>",
            "<Task tracking, epics, and implementation status>",
            "<Timeline, phasing, and dependencies>",
            "<System boundaries, tech stack, and infra>",
        ],
    },
    TemplateProfile {
        doc_name: DESIGN_DOC,
        template_file: "HOW-TO-MAKE-DESIGN.md",
        deletable_sections: &[],
        required_h2s: DESIGN_REQUIRED_H2S,
        placeholder_catalog: &[
            "<project-name>",
            "<Short imperative name>",
            "<DESIGN-NAME-ID>",
            "<One paragraph of what the design looks like.>",
            "<Business goals, user stories, and scope>",
            "<Task tracking, epics, and implementation status>",
            "<Timeline, phasing, and dependencies>",
            "<System boundaries, tech stack, and infra>",
        ],
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    /// Load a reference template from skills/aes-docs/references/.
    fn load_reference_template(file: &str) -> String {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../skills/aes-docs/references")
            .join(file);
        match std::fs::read_to_string(&path) {
            Ok(text) => text,
            Err(e) => panic!("reference template {file} must be readable: {e}"),
        }
    }

    /// The document an author gets by copying the fenced template verbatim.
    fn verbatim_copy(profile: &TemplateProfile) -> String {
        let text = load_reference_template(profile.template_file);
        let open = "```markdown";
        let start = text
            .find(open)
            .unwrap_or_else(|| panic!("{} opens a template block", profile.template_file))
            + open.len();
        let rest = &text[start..];
        let end = rest
            .find("```")
            .unwrap_or_else(|| panic!("{} closes its template block", profile.template_file));
        rest[..end].to_string()
    }

    /// Inline placeholder matcher for tests — taxonomy(vo) must not import
    /// utility_markdown_scanner (AES201). Mirrors the utility function's
    /// marker-protection invariant without the non-prose stripping.
    fn matches_placeholder_catalog_inline(text: &str, catalog: &[&str]) -> Vec<(usize, String)> {
        let mut markers: Vec<&str> = catalog.to_vec();
        markers.sort_by_key(|marker| std::cmp::Reverse(marker.len()));
        let mut hits: Vec<(usize, String)> = Vec::new();
        for marker in markers {
            let mut pos = 0;
            while let Some(idx) = text[pos..].find(marker) {
                hits.push((pos + idx, marker.to_string()));
                pos += idx + marker.len();
            }
        }
        hits
    }

    /// A verbatim template copy must trip every catalogue marker — a profile
    /// entry no copy can fire is dead data (Decision Q3's matching path).
    #[test]
    fn every_catalog_entry_fires_on_a_verbatim_template_copy() {
        for profile in TEMPLATE_PROFILES {
            let copy = verbatim_copy(profile);
            let hits = matches_placeholder_catalog_inline(&copy, profile.placeholder_catalog);
            for marker in profile.placeholder_catalog {
                assert!(
                    hits.iter().any(|(_, hit)| hit.as_str() == *marker),
                    "{}/{}: catalog entry {marker:?} never fires on a verbatim copy",
                    profile.doc_name,
                    profile.template_file,
                );
            }
        }
    }

    /// The AGENTS copy specifically fires its catalog (review Issue 2 asks for
    /// this case by name).
    #[test]
    fn agents_verbatim_copy_fires_its_catalog() {
        let profile = TEMPLATE_PROFILES
            .iter()
            .find(|p| p.doc_name == AGENTS_DOC)
            .expect("AGENTS profile exists");
        let hits = matches_placeholder_catalog_inline(
            &verbatim_copy(profile),
            profile.placeholder_catalog,
        );
        assert!(!hits.is_empty(), "AGENTS copy must fire the catalog");
    }
}
