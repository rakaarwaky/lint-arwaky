// PURPOSE: unit tests for FR-DOC-008 (MemberDocChecker) and FR-DOC-009
// (TemplateBodyChecker) — the two new AES606 capability seams.
//
// Each case exercises the smallest input that decides its rule: a member
// descriptor that must owe (or not owe) a doc pair, and a document whose
// placeholders (or missing sections) must fire findings when strict mode
// is on.
use doc_rules_lint_arwaky::capabilities_doc_member_doc_checker::MemberDocChecker;
use doc_rules_lint_arwaky::capabilities_doc_template_body_checker::TemplateBodyChecker;
use shared_doc_rules::contract_doc_protocol::{IMemberDocProtocol, ITemplateBodyProtocol};
use shared_doc_rules::taxonomy_doc_audit_context_vo::{
    DocAuditContext, DocRulesConfig, MemberDescriptor, MemberRole, StrictModeOverride,
};
use shared_doc_rules::taxonomy_doc_rules_request::DocSource;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

// ─── Context builders ──────────────────────────────────────

/// An audit context with one member descriptor matching *role* and *path*,
/// carrying *present_docs* as its known document set, under root *root*.
fn member_context(
    root: &Path,
    role: MemberRole,
    path: &str,
    present_docs: &[&str],
    config: Option<&DocRulesConfig>,
    strict_override: StrictModeOverride,
) -> DocAuditContext {
    let member = MemberDescriptor {
        path: PathBuf::from(path),
        role,
        present_docs: present_docs
            .iter()
            .map(|s| s.to_string())
            .collect::<BTreeSet<String>>(),
    };
    DocAuditContext::new(
        root,
        Vec::new(),
        None,
        vec![member],
        config.cloned(),
        strict_override,
    )
}

/// A DocAuditContext with one document *name* / *text* (no members).
fn doc_context(name: &str, text: &str) -> DocAuditContext {
    let text_owned = text.to_string();
    let documents = vec![DocSource {
        path: PathBuf::from(format!("/repo/{name}")),
        text: text_owned.clone(),
    }];
    DocAuditContext::new(
        Path::new("/repo"),
        documents,
        Some(text_owned),
        Vec::new(),
        None,
        StrictModeOverride::UseConfigFile,
    )
}

/// Create a tempdir with Cargo.toml so is_workspace_root returns true.
fn temp_workspace() -> tempfile::TempDir {
    let tmp = tempfile::tempdir().unwrap();
    std::fs::write(tmp.path().join("Cargo.toml"), "[workspace]\nmembers = []\n").unwrap();
    tmp
}

/// One member-doc config variant: `strict_member_docs: true` with
/// *exempt_members*.
fn member_doc_config(exempt_members: Vec<String>) -> DocRulesConfig {
    DocRulesConfig {
        strict_member_docs: true,
        strict_placeholders: false,
        exempt_members,
    }
}

// ─── FR-DOC-008 — MemberDocChecker ─────────────────────────

/// A feature member missing both FRD.md and BACKLOG.md fires two findings
/// when strict mode is on.
#[test]
fn member_doc_checker_feature_missing_pair_fires_findings() {
    let tmp = temp_workspace();
    let config = member_doc_config(Vec::new());
    let context = member_context(
        tmp.path(),
        MemberRole::Feature,
        "crates/billing",
        &[],
        Some(&config),
        StrictModeOverride::UseConfigFile,
    );
    let findings = MemberDocChecker::new().audit_member_docs(&context);
    assert_eq!(
        findings.len(),
        2,
        "a feature with zero docs must owe both FRD.md and BACKLOG.md; got {findings:?}"
    );
    for f in &findings {
        assert_eq!(f.violation_type, "missing_member_doc");
        assert!(!f.why.is_empty(), "finding must carry why; got {f:?}");
        assert!(!f.fix.is_empty(), "finding must carry fix; got {f:?}");
    }
}

/// A member in `exempt_members` is skipped even when strict is on.
#[test]
fn member_doc_checker_exempt_member_is_suppressed() {
    let tmp = temp_workspace();
    let config = member_doc_config(vec!["crates/legacy".to_string()]);
    let context = member_context(
        tmp.path(),
        MemberRole::Feature,
        "crates/legacy",
        &[],
        Some(&config),
        StrictModeOverride::UseConfigFile,
    );
    let findings = MemberDocChecker::new().audit_member_docs(&context);
    assert!(
        findings.is_empty(),
        "an exempt member must produce no findings; got {findings:?}"
    );
}

/// When `strict_member_docs` is off (no config, UseConfigFile), no findings
/// are emitted regardless of the member state.
#[test]
fn member_doc_checker_strict_off_suppresses_findings() {
    let tmp = temp_workspace();
    let context = member_context(
        tmp.path(),
        MemberRole::Feature,
        "crates/billing",
        &[],
        None,
        StrictModeOverride::UseConfigFile,
    );
    let findings = MemberDocChecker::new().audit_member_docs(&context);
    assert!(
        findings.is_empty(),
        "strict off must suppress all member-doc findings; got {findings:?}"
    );
}

/// ForceStrict overrides a disabled config and forces findings on.
#[test]
fn member_doc_checker_force_strict_overrides_config_off() {
    let tmp = temp_workspace();
    let config = DocRulesConfig {
        strict_member_docs: false,
        strict_placeholders: false,
        exempt_members: Vec::new(),
    };
    let context = member_context(
        tmp.path(),
        MemberRole::Kernel,
        "crates/shared",
        &[],
        Some(&config),
        StrictModeOverride::ForceStrict,
    );
    let findings = MemberDocChecker::new().audit_member_docs(&context);
    assert_eq!(
        findings.len(),
        2,
        "ForceStrict must override config-off; a kernel owes DATA.md + BACKLOG.md; got {findings:?}"
    );
}

/// Kernel role owes DATA.md + BACKLOG.md (not FRD.md).
#[test]
fn member_doc_checker_kernel_owes_data_and_backlog() {
    let tmp = temp_workspace();
    let config = member_doc_config(Vec::new());
    // Kernel with only DATA.md present, BACKLOG.md missing
    let context = member_context(
        tmp.path(),
        MemberRole::Kernel,
        "crates/shared",
        &["DATA.md"],
        Some(&config),
        StrictModeOverride::UseConfigFile,
    );
    let findings = MemberDocChecker::new().audit_member_docs(&context);
    assert_eq!(
        findings.len(),
        1,
        "kernel with DATA.md present owes only BACKLOG.md; got {findings:?}"
    );
    assert!(findings[0].message.contains("BACKLOG.md"));
}

/// Surface role owes DESIGN.md + BACKLOG.md (not FRD.md).
#[test]
fn member_doc_checker_surface_owes_design_and_backlog() {
    let tmp = temp_workspace();
    let config = member_doc_config(Vec::new());
    let context = member_context(
        tmp.path(),
        MemberRole::Surface,
        "crates/mcp-server",
        &["DESIGN.md"],
        Some(&config),
        StrictModeOverride::UseConfigFile,
    );
    let findings = MemberDocChecker::new().audit_member_docs(&context);
    assert_eq!(
        findings.len(),
        1,
        "surface with DESIGN.md present owes only BACKLOG.md; got {findings:?}"
    );
    assert!(findings[0].message.contains("BACKLOG.md"));
}

/// A fully compliant feature member produces zero findings.
#[test]
fn member_doc_checker_compliant_feature_no_findings() {
    let tmp = temp_workspace();
    let config = member_doc_config(Vec::new());
    let context = member_context(
        tmp.path(),
        MemberRole::Feature,
        "crates/billing",
        &["FRD.md", "BACKLOG.md"],
        Some(&config),
        StrictModeOverride::UseConfigFile,
    );
    let findings = MemberDocChecker::new().audit_member_docs(&context);
    assert!(
        findings.is_empty(),
        "compliant feature must be clean; got {findings:?}"
    );
}

/// MemberRole::None produces no findings even when strict is on.
#[test]
fn member_doc_checker_none_role_ows_nothing() {
    let tmp = temp_workspace();
    let config = member_doc_config(Vec::new());
    let context = member_context(
        tmp.path(),
        MemberRole::None,
        "crates/bench",
        &[],
        Some(&config),
        StrictModeOverride::UseConfigFile,
    );
    let findings = MemberDocChecker::new().audit_member_docs(&context);
    assert!(
        findings.is_empty(),
        "None role must produce no findings; got {findings:?}"
    );
}

/// Surface member carrying DESIGN.md + BACKLOG.md plus forbidden FRD.md fires
/// a single `forbidden_member_doc` finding; its owed pair is present so only
/// the forbidden-doc half fires.
#[test]
fn member_doc_checker_surface_with_forbidden_frd_fires() {
    let tmp = temp_workspace();
    let config = member_doc_config(Vec::new());
    let context = member_context(
        tmp.path(),
        MemberRole::Surface,
        "crates/mcp-server",
        &["DESIGN.md", "BACKLOG.md", "FRD.md"],
        Some(&config),
        StrictModeOverride::UseConfigFile,
    );
    let findings = MemberDocChecker::new().audit_member_docs(&context);
    assert_eq!(
        findings.len(),
        1,
        "surface carrying forbidden FRD.md must fire exactly one forbidden_member_doc; got {findings:?}"
    );
    assert_eq!(
        findings[0].doc, "crates/mcp-server/FRD.md",
        "doc must name the forbidden FRD.md path; got {:?}",
        findings[0].doc
    );
    assert_eq!(
        findings[0].violation_type, "forbidden_member_doc",
        "surface's forbidden doc must be forbidden_member_doc; got {:?}",
        findings[0].violation_type
    );
    assert!(
        findings[0].message.contains("FRD.md"),
        "message must name the forbidden FRD.md; got {:?}",
        findings[0].message
    );
    assert!(
        findings[0].fix.contains("FRD.md"),
        "fix must name FRD.md; got {:?}",
        findings[0].fix
    );
}

/// Kernel member carrying DATA.md + BACKLOG.md plus forbidden FRD.md fires.
#[test]
fn member_doc_checker_kernel_with_forbidden_frd_fires() {
    let tmp = temp_workspace();
    let config = member_doc_config(Vec::new());
    let context = member_context(
        tmp.path(),
        MemberRole::Kernel,
        "crates/shared",
        &["DATA.md", "BACKLOG.md", "FRD.md"],
        Some(&config),
        StrictModeOverride::UseConfigFile,
    );
    let findings = MemberDocChecker::new().audit_member_docs(&context);
    assert_eq!(
        findings.len(),
        1,
        "kernel carrying forbidden FRD.md must fire exactly one forbidden_member_doc; got {findings:?}"
    );
    assert_eq!(
        findings[0].doc, "crates/shared/FRD.md",
        "doc must name the forbidden FRD.md path; got {:?}",
        findings[0].doc
    );
    assert_eq!(
        findings[0].violation_type, "forbidden_member_doc",
        "kernel's forbidden doc must be forbidden_member_doc; got {:?}",
        findings[0].violation_type
    );
}

/// Feature member carrying FRD.md + BACKLOG.md plus forbidden DATA.md fires.
#[test]
fn member_doc_checker_feature_with_forbidden_data_fires() {
    let tmp = temp_workspace();
    let config = member_doc_config(Vec::new());
    let context = member_context(
        tmp.path(),
        MemberRole::Feature,
        "crates/billing",
        &["FRD.md", "BACKLOG.md", "DATA.md"],
        Some(&config),
        StrictModeOverride::UseConfigFile,
    );
    let findings = MemberDocChecker::new().audit_member_docs(&context);
    assert_eq!(
        findings.len(),
        1,
        "feature carrying forbidden DATA.md must fire exactly one forbidden_member_doc; got {findings:?}"
    );
    assert_eq!(
        findings[0].doc, "crates/billing/DATA.md",
        "doc must name the forbidden DATA.md path; got {:?}",
        findings[0].doc
    );
    assert_eq!(
        findings[0].violation_type, "forbidden_member_doc",
        "feature's forbidden doc must be forbidden_member_doc; got {:?}",
        findings[0].violation_type
    );
}

/// Feature member carrying FRD.md + BACKLOG.md + DESIGN.md fires nothing:
/// FrFormatChecker's Decision-9 pairing explicitly allows a feature folder to
/// carry both FRD.md and DESIGN.md.
#[test]
fn member_doc_checker_feature_with_design_is_clean() {
    let tmp = temp_workspace();
    let config = member_doc_config(Vec::new());
    let context = member_context(
        tmp.path(),
        MemberRole::Feature,
        "crates/billing",
        &["FRD.md", "BACKLOG.md", "DESIGN.md"],
        Some(&config),
        StrictModeOverride::UseConfigFile,
    );
    let findings = MemberDocChecker::new().audit_member_docs(&context);
    assert!(
        findings.is_empty(),
        "feature with FRD + BACKLOG + DESIGN must be clean (DESIGN is allowed for features); got {findings:?}"
    );
}

/// With strict_member_docs off, no forbidden-member-doc findings fire even
/// when a member carries forbidden docs.
#[test]
fn member_doc_checker_forbidden_strict_off_suppresses() {
    let tmp = temp_workspace();
    let context = member_context(
        tmp.path(),
        MemberRole::Surface,
        "crates/mcp-server",
        &["DESIGN.md", "BACKLOG.md", "FRD.md"],
        None,
        StrictModeOverride::UseConfigFile,
    );
    let findings = MemberDocChecker::new().audit_member_docs(&context);
    assert!(
        findings.is_empty(),
        "strict off must suppress forbidden-member-doc findings; got {findings:?}"
    );
}

/// An exempt member carrying forbidden docs is suppressed from the
/// forbidden-member-doc check too (same exemption branch as missing docs).
#[test]
fn member_doc_checker_forbidden_exempt_member_suppressed() {
    let tmp = temp_workspace();
    let config = member_doc_config(vec!["crates/legacy".to_string()]);
    let context = member_context(
        tmp.path(),
        MemberRole::Surface,
        "crates/legacy",
        &["DESIGN.md", "BACKLOG.md", "FRD.md"],
        Some(&config),
        StrictModeOverride::UseConfigFile,
    );
    let findings = MemberDocChecker::new().audit_member_docs(&context);
    assert!(
        findings.is_empty(),
        "exempt member must be clean even when carrying forbidden FRD.md; got {findings:?}"
    );
}

// ─── FR-DOC-009 — TemplateBodyChecker ──────────────────────

/// An AGENTS.md with a known curly-slot placeholder (`{lint command}`) fires
/// an `unfilled_placeholder` finding when strict_placeholders is on.
#[test]
fn template_body_checker_unfilled_placeholder_fires_when_strict_on() {
    let config = DocRulesConfig {
        strict_member_docs: false,
        strict_placeholders: true,
        exempt_members: Vec::new(),
    };
    let text = "---\n\
trigger: always\ndescription: \"Sample.\"\n\
---\n\
# AGENTS\n\n\
## Agent Overview\n\n\
This workspace is <project-name>; follow its conventions.\n\n\
## Runtime\n\n\
| Aspect          | Detail                                          |\n\
|-----------------|-------------------------------------------------|\n\
| Language        | {language and pinned version}                   |\n\
| Environment     | {environment manager and creation method}       |\n\
| Artifacts       | {reviewable artifact location}                  |\n\
| Package Manager | {package manager and lockfile}                  |\n\
| Version Probe   | `{version probe command}`                       |\n\
| Setup           | `{environment setup command}`                    |\n\
| Activation      | `{optional environment activation command}`      |\n\n\
## Project Structure\n\n\
| Path       | Purpose              |\n\
|------------|----------------------|\n\
| {path}     | {what lives here}    |\n\n\
## Commands\n\n\
| Category    | Command                              | Scope      | CI Gate              |\n\
|-------------|--------------------------------------|------------|----------------------|\n\
| Test (all)  | {lint command}                       | All        | ci.yml: Test         |\n\
| Test (unit) | `{single-package test command}`      | One unit   | ci.yml: Test         |\n\
| Test (file) | `{single-file test command}`         | One file   | ci.yml: Test         |\n\
| Lint        | `{formatter or linter command}`      | All        | ci.yml: Format       |\n\
| Type Check  | `{type checker command}`             | All        | ci.yml: Clippy       |\n\
| Arch Scan   | `{architecture scanner command}`     | All        | ci.yml: Self-Lint    |\n\n\
## Reference\n\n\
| Artifact     | Purpose                                    | Link |\n\
|--------------|--------------------------------------------|------|\n\
| README       | What the tool does, every command, install | [README.md](README.md) |\n\
| CONTRIBUTING | Branch, commit, and PR conventions         | [CONTRIBUTING.md](CONTRIBUTING.md) |\n\
| ARCHITECTURE | Layer contract for every crate             | [ARCHITECTURE.md](ARCHITECTURE.md) |\n\
| ROADMAP      | What ships next, in order                  | [ROADMAP.md](ROADMAP.md) |\n\
| PRD          | What the product does and why              | [PRD.md](PRD.md) |\n";
    let context = {
        let documents = vec![DocSource {
            path: PathBuf::from("/repo/AGENTS.md"),
            text: text.to_string(),
        }];
        DocAuditContext::new(
            Path::new("/repo"),
            documents,
            None,
            Vec::new(),
            Some(config),
            StrictModeOverride::UseConfigFile,
        )
    };
    let findings = TemplateBodyChecker::new().audit_template_body(&context);
    assert!(
        findings
            .iter()
            .any(|f| f.violation_type == "unfilled_placeholder"),
        "unfilled placeholder under strict mode must fire; got {findings:?}"
    );
}

/// The same document with strict_placeholders off (no config) produces no
/// placeholder findings.
#[test]
fn template_body_checker_strict_off_suppresses_placeholder_findings() {
    let text = "# AGENTS\n\nSome prose with {lint command} unfilled\n";
    let context = doc_context("AGENTS.md", text);
    let findings = TemplateBodyChecker::new().audit_template_body(&context);
    assert!(
        !findings
            .iter()
            .any(|f| f.violation_type == "unfilled_placeholder"),
        "strict placeholders off must suppress all placeholder findings; got {findings:?}"
    );
}

/// An angle token inside a fenced code block (`Vec<DocFinding>`) is blanked
/// by the scanner and never fires a placeholder finding.
#[test]
fn template_body_checker_placeholder_in_code_fence_not_detected() {
    let config = DocRulesConfig {
        strict_member_docs: false,
        strict_placeholders: true,
        exempt_members: Vec::new(),
    };
    let text = "# README\n\n```rust\nlet x: Vec<DocFinding> = vec![];\n```\n";
    let documents = vec![DocSource {
        path: PathBuf::from("/repo/README.md"),
        text: text.to_string(),
    }];
    let context = DocAuditContext::new(
        Path::new("/repo"),
        documents,
        None,
        Vec::new(),
        Some(config),
        StrictModeOverride::UseConfigFile,
    );
    let findings = TemplateBodyChecker::new().audit_template_body(&context);
    // h2_missing would fire for README (missing required H2s), but no unfilled_placeholder
    // should be in the set for the code fence content.
    assert!(
        !findings
            .iter()
            .any(|f| f.violation_type == "unfilled_placeholder"),
        "fenced code must never fire a placeholder finding; got {findings:?}"
    );
}

/// A clean document (all required H2s present, no placeholder tokens)
/// produces zero findings.
#[test]
fn template_body_checker_clean_document_no_findings() {
    let config = DocRulesConfig {
        strict_member_docs: false,
        strict_placeholders: true,
        exempt_members: Vec::new(),
    };
    let text = "# ROADMAP\n\n## Progress Overview\n\ntext\n\n## Roadmap Status\n\n| ID | Feature | Priority | State | Dependencies | Next Action | Updated |\n| --- | --- | --- | --- | --- | --- | --- |\n| WS-01 | Docs | P0 | Done | — | — | 2026-01-01 |\n\n## Reference\n\nNone.\n";
    let documents = vec![DocSource {
        path: PathBuf::from("/repo/ROADMAP.md"),
        text: text.to_string(),
    }];
    let context = DocAuditContext::new(
        Path::new("/repo"),
        documents,
        None,
        Vec::new(),
        Some(config),
        StrictModeOverride::UseConfigFile,
    );
    let findings = TemplateBodyChecker::new().audit_template_body(&context);
    assert!(
        findings.is_empty(),
        "a clean ROADMAP with all sections and no placeholders must be clean; got {findings:?}"
    );
}

/// A missing non-deletable required H2 fires `section_deleted`.
#[test]
fn template_body_checker_deleted_required_section_fires() {
    let config = DocRulesConfig {
        strict_member_docs: false,
        strict_placeholders: false,
        exempt_members: Vec::new(),
    };
    // ROADMAP keeps two required H2s and omits `## Roadmap Status`, so the
    // message must name that specific gap.
    let text = "# ROADMAP\n\n## Progress Overview\n\nx\n\n## Reference\n\nx\n";
    let documents = vec![DocSource {
        path: PathBuf::from("/repo/ROADMAP.md"),
        text: text.to_string(),
    }];
    let context = DocAuditContext::new(
        Path::new("/repo"),
        documents,
        None,
        Vec::new(),
        Some(config),
        StrictModeOverride::UseConfigFile,
    );
    let findings = TemplateBodyChecker::new().audit_template_body(&context);
    let deleted: Vec<_> = findings
        .iter()
        .filter(|f| f.violation_type == "section_deleted")
        .collect();
    assert!(
        !deleted.is_empty(),
        "a missing non-deletable section must fire section_deleted; got {findings:?}"
    );
    assert!(deleted[0].message.contains("Roadmap Status"));
    assert!(!deleted[0].why.is_empty());
    assert!(!deleted[0].fix.is_empty());
}

/// No section is deletable under the 9-doc layout (`deletable_sections` is
/// empty for every profile), so absence only stays silent for a section
/// outside the required set — here the legacy "Quality Verification & PR
/// Process" heading old CONTRIBUTING templates once carried.
#[test]
fn template_body_checker_deletable_section_absent_no_finding() {
    let config = DocRulesConfig {
        strict_member_docs: false,
        strict_placeholders: false,
        exempt_members: Vec::new(),
    };
    // Every CONTRIBUTING_REQUIRED_H2S present; the legacy section absent.
    let text = "# CONTRIBUTING\n\n## Contribution Overview\n\nx\n\n## Contribution Guidelines\n\nx\n\n## Workflow\n\nx\n\n## Commit Convention\n\nx\n\n## Reference\n\nx\n";
    let documents = vec![DocSource {
        path: PathBuf::from("/repo/CONTRIBUTING.md"),
        text: text.to_string(),
    }];
    let context = DocAuditContext::new(
        Path::new("/repo"),
        documents,
        None,
        Vec::new(),
        Some(config),
        StrictModeOverride::UseConfigFile,
    );
    let findings = TemplateBodyChecker::new().audit_template_body(&context);
    assert!(
        !findings
            .iter()
            .any(|f| f.violation_type == "section_deleted"),
        "deletable section absence must not fire; got {findings:?}"
    );
}

/// A section heading that extends the required title (prefix match)
/// satisfies the requirement; only an entire omission fires `section_deleted`.
#[test]
fn template_body_checker_extended_heading_satisfies_required_section() {
    let config = DocRulesConfig {
        strict_member_docs: false,
        strict_placeholders: false,
        exempt_members: Vec::new(),
    };
    // `## Project Requirements (Prioritized)` extends the required
    // `Project Requirements` by prefix match; only an omission fires.
    let text = "# PRD\n\n## Problem Statement\n\nx\n\n## Project Requirements (Prioritized)\n\nx\n\n\
         ## Reference\n\n x\n\n## Feature Requirements (Prioritized)\n\nx\n";
    let documents = vec![DocSource {
        path: PathBuf::from("/repo/PRD.md"),
        text: text.to_string(),
    }];
    let context = DocAuditContext::new(
        Path::new("/repo"),
        documents,
        None,
        Vec::new(),
        Some(config),
        StrictModeOverride::UseConfigFile,
    );
    let findings = TemplateBodyChecker::new().audit_template_body(&context);
    assert!(
        !findings
            .iter()
            .any(|f| f.violation_type == "section_deleted"),
        "extended headings must satisfy their required section; got {findings:?}"
    );
}

/// `ForceStrict` makes placeholder findings fire even when config is off.
#[test]
fn template_body_checker_force_strict_enables_placeholder_check() {
    let config = DocRulesConfig {
        strict_member_docs: false,
        strict_placeholders: false,
        exempt_members: Vec::new(),
    };
    let text = "---\n\
trigger: always\ndescription: \"Sample.\"\n\
---\n\
# AGENTS\n\n\
## Agent Overview\n\nPlaceholder <project-name> here\n\n\
## Runtime\n\n\
| Aspect          | Detail                                          |\n\
|-----------------|-------------------------------------------------|\n\
| Language        | Rust                                            |\n\
| Environment     | cargo workspace under crates/                   |\n\
| Artifacts       | target/                                         |\n\
| Package Manager | cargo, with Cargo.lock                          |\n\
| Version Probe   | `rustc --version`                               |\n\
| Setup           | `cargo build --release`                         |\n\
| Activation      | None                                            |\n\n\
## Project Structure\n\n\
| Path       | Purpose              |\n\
|------------|----------------------|\n\
| crates/    | Rust workspace crates|\n\n\
## Commands\n\n\
| Category    | Command                              | Scope      | CI Gate              |\n\
|-------------|--------------------------------------|------------|----------------------|\n\
| Test (all)  | `cargo nextest run`                  | All        | ci.yml: Test         |\n\
| Lint        | `cargo fmt --all -- --check`         | All        | ci.yml: Format       |\n\n\
## Reference\n\n\
| Artifact     | Purpose                                    | Link |\n\
|--------------|--------------------------------------------|------|\n\
| README       | What the tool does, every command, install | [README.md](README.md) |\n\
| PRD          | What the product does and why              | [PRD.md](PRD.md) |\n";
    let documents = vec![DocSource {
        path: PathBuf::from("/repo/AGENTS.md"),
        text: text.to_string(),
    }];
    let context = DocAuditContext::new(
        Path::new("/repo"),
        documents,
        None,
        Vec::new(),
        Some(config),
        StrictModeOverride::ForceStrict,
    );
    let findings = TemplateBodyChecker::new().audit_template_body(&context);
    assert!(
        findings
            .iter()
            .any(|f| f.violation_type == "unfilled_placeholder"),
        "ForceStrict must enable placeholder findings; got {findings:?}"
    );
}
