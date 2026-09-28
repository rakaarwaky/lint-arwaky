// PURPOSE: contract tests for doc-rules — each invariant must fire on a bad
// document and stay silent on a conforming one. Findings carry a code
// (AES601–AES605) plus a violation_type, and both are asserted.
use doc_rules_lint_arwaky::root_doc_rules_container::RootDocRulesContainer;
use shared::doc_rules::taxonomy_doc_request::DocRequest;
use shared::doc_rules::taxonomy_doc_response::DocResponse;

use std::fs;
use std::path::Path;

/// A FRD that satisfies every structural invariant.
fn conforming_frd() -> String {
    r#"# FRD — sample

## Reference

- PRD: [PRD.md](../../PRD.md)
- Backlog: [BACKLOG.md](BACKLOG.md) — real condition for this feature.

## System Overview

The feature does the work a surface delegates to.

## Functional Requirements

### FR-SAMPLE-001: Do The Thing

- **Description**: The feature performs its single responsibility.
- **Input**: A request value object.
- **Output**: A response value object.
- **Business Rules**: The capability is stateless.
- **Edge Cases**: An absent input yields a default.
- **Error Handling**: Failures surface as a reason-coded outcome.

## API Contract

### Protocol API

| Method | Input | Output | Error | Event | Description |
| --- | --- | --- | --- | --- | --- |
| `execute` | `SampleRequest` | `SampleResponse` | Reason-coded | — | Single composite entry point. |

### Aggregate API

| Method | Input | Output | Error | Event | Description |
| --- | --- | --- | --- | --- | --- |
| `execute` | `SampleRequest` | `SampleResponse` | Reason-coded | — | Routes to the capability. |

## Integration Points

| System | Direction | Purpose | Failure mode |
| --- | --- | --- | --- |
| Shared domain | in | Supplies value objects | None — types only |

## Non-functional Requirements

| Metric | Target | Measurement method |
| --- | --- | --- |
| Dispatch cost | One call per request | Read the dispatch table |

## Test Scenarios

- A conforming request produces a conforming response.

## Assumptions & Constraints

- The capability is stateless.

## Glossary

- **Sample**: A value object used in this example.
"#
    .to_string()
}

/// Build a workspace whose only feature folder is a real feature, so AES605
/// (folder health) does not fire on a conforming document set.
fn write_workspace(dir: &Path, frd: &str) {
    let feature = dir.join("crates/sample");
    fs::create_dir_all(feature.join("src")).unwrap();
    fs::write(feature.join("FRD.md"), frd).unwrap();
    fs::write(feature.join("BACKLOG.md"), "# BACKLOG — sample\n").unwrap();
    fs::write(
        feature.join("src/agent_sample_orchestrator.rs"),
        "//! sample orchestrator\n",
    )
    .unwrap();
    fs::write(dir.join("PRD.md"), "# PRD — sample\n").unwrap();
    fs::write(dir.join("ROADMAP.md"), "# ROADMAP — sample\n").unwrap();
}

/// Build a workspace with a conforming AGENTS.md so AES606 stays silent.
fn write_agents_workspace(dir: &Path) {
    let feature = dir.join("crates/sample");
    fs::create_dir_all(feature.join("src")).unwrap();
    fs::write(feature.join("FRD.md"), "# FRD — sample\n").unwrap();
    fs::write(feature.join("BACKLOG.md"), "# BACKLOG — sample\n").unwrap();
    fs::write(
        feature.join("src/agent_sample_orchestrator.rs"),
        "//! sample orchestrator\n",
    )
    .unwrap();
    fs::write(dir.join("PRD.md"), "# PRD — sample\n").unwrap();
    fs::write(dir.join("ROADMAP.md"), "# ROADMAP — sample\n").unwrap();
    fs::write(
        dir.join("AGENTS.md"),
        "# Sample AGENTS.md\n\n\
## Precedence\n\n1. Safety rules.\n\n\
## Security\n\n- Explicit approval is required before destructive actions.\n\n\
## Architecture\n\nSee [ARCHITECTURE.md](ARCHITECTURE.md) for the layer specification.\n\n\
## Contributing\n\nSetup and code style live in [CONTRIBUTING.md](CONTRIBUTING.md).\n\n\
## License\n\nMIT. See [LICENSE](LICENSE).\n\n\
## Git Workflow\n\nEvery change must use a worktree or branch.\n\n\
## Commands\n\n```bash\ncargo nextest run --workspace\n```\n\n\
## Definition of Done\n\nA change is done when tests pass.\n\n\
## Related Documents\n\n- [PRD.md](PRD.md): Product requirements.\n",
    )
    .unwrap();
}

fn write_bad_agents(dir: &Path, agent_text: &str) {
    write_agents_workspace(dir);
    fs::write(dir.join("AGENTS.md"), agent_text).unwrap();
}

fn audit(root: &Path) -> Vec<(String, String, String)> {
    let orchestrator = RootDocRulesContainer::orchestrator();
    match orchestrator.execute(DocRequest::audit_all(root)) {
        DocResponse::Findings { findings } => findings
            .into_iter()
            .map(|f| (f.code.to_string(), f.violation_type.to_string(), f.message))
            .collect(),
    }
}

/// Does any finding carry this (code, violation_type) pair?
fn has(findings: &[(String, String, String)], code: &str, violation_type: &str) -> bool {
    findings
        .iter()
        .any(|(c, v, _)| c == code && v == violation_type)
}

#[test]
fn conforming_frd_reports_no_findings() {
    let tmp = tempfile::tempdir().unwrap();
    write_workspace(tmp.path(), &conforming_frd());
    let findings = audit(tmp.path());
    assert!(findings.is_empty(), "expected clean, got: {findings:#?}");
}

#[test]
fn aes601_fires_on_bare_fr_id() {
    let tmp = tempfile::tempdir().unwrap();
    let frd = conforming_frd().replace("### FR-SAMPLE-001:", "### FR-001:");
    write_workspace(tmp.path(), &frd);
    let findings = audit(tmp.path());
    assert!(
        has(&findings, "AES601", "id_missing_feature_prefix"),
        "expected id_missing_feature_prefix, got: {findings:#?}"
    );
}

#[test]
fn aes601_reports_each_missing_field_with_its_own_type() {
    let tmp = tempfile::tempdir().unwrap();
    let frd = conforming_frd()
        .replace("- **Edge Cases**: An absent input yields a default.\n", "")
        .replace(
            "- **Error Handling**: Failures surface as a reason-coded outcome.\n",
            "",
        );
    write_workspace(tmp.path(), &frd);
    let findings = audit(tmp.path());
    let field_findings: Vec<&str> = findings
        .iter()
        .filter(|(c, v, _)| c == "AES601" && v == "field_missing")
        .map(|(_, _, m)| m.as_str())
        .collect();
    assert_eq!(field_findings.len(), 1, "one finding per FR: {findings:#?}");
    let message = field_findings[0];
    assert!(
        message.contains("Edge Cases") && message.contains("Error Handling"),
        "the message must name each missing field, got: {message}"
    );
}

#[test]
fn aes602_fires_when_aggregate_api_is_absent() {
    let tmp = tempfile::tempdir().unwrap();
    let frd = conforming_frd()
        .replace("### Aggregate API", "### Other API")
        .replace(
            "| `execute` | `SampleRequest` | `SampleResponse` | Reason-coded | — | Routes to the capability. |\n",
            "",
        );
    write_workspace(tmp.path(), &frd);
    assert!(
        has(&audit(tmp.path()), "AES602", "api_no_subsection"),
        "expected api_no_subsection, got: {:#?}",
        audit(tmp.path())
    );
}

#[test]
fn aes602_fires_when_integration_points_is_a_bullet_list() {
    let tmp = tempfile::tempdir().unwrap();
    let frd = conforming_frd().replace(
        "| System | Direction | Purpose | Failure mode |\n| --- | --- | --- | --- |\n| Shared domain | in | Supplies value objects | None — types only |",
        "- Shared domain supplies value objects",
    );
    write_workspace(tmp.path(), &frd);
    assert!(has(&audit(tmp.path()), "AES602", "integration_not_table"));
}

#[test]
fn aes602_fires_when_nfr_is_prose() {
    let tmp = tempfile::tempdir().unwrap();
    let frd = conforming_frd().replace(
        "| Metric | Target | Measurement method |\n| --- | --- | --- |\n| Dispatch cost | One call per request | Read the dispatch table |",
        "- Dispatch cost is one call per request.",
    );
    write_workspace(tmp.path(), &frd);
    assert!(has(&audit(tmp.path()), "AES602", "nfr_not_table"));
}

#[test]
fn aes602_fires_when_sections_are_out_of_order() {
    let tmp = tempfile::tempdir().unwrap();
    let scrambled = conforming_frd()
        .replace("## Test Scenarios", "## @@MARK@@")
        .replace("## Glossary", "## Test Scenarios")
        .replace("## @@MARK@@", "## Glossary");
    write_workspace(tmp.path(), &scrambled);
    assert!(has(&audit(tmp.path()), "AES602", "order_violation"));
}

#[test]
fn aes602_fires_when_scenarios_have_no_bullets() {
    let tmp = tempfile::tempdir().unwrap();
    let frd = conforming_frd().replace(
        "- A conforming request produces a conforming response.",
        "1. A conforming request produces a conforming response.",
    );
    write_workspace(tmp.path(), &frd);
    assert!(has(&audit(tmp.path()), "AES602", "scenarios_empty"));
}

#[test]
fn aes602_fires_when_glossary_has_no_bullets() {
    let tmp = tempfile::tempdir().unwrap();
    let frd = conforming_frd().replace(
        "- **Sample**: A value object used in this example.",
        "| Sample | A value object |",
    );
    write_workspace(tmp.path(), &frd);
    assert!(has(&audit(tmp.path()), "AES602", "glossary_empty"));
}

#[test]
fn aes603_fires_when_a_spec_names_a_source_file() {
    let tmp = tempfile::tempdir().unwrap();
    let frd = conforming_frd().replace(
        "The feature does the work a surface delegates to.",
        "The feature reads settings from main.rs at startup.",
    );
    write_workspace(tmp.path(), &frd);
    let findings = audit(tmp.path());
    assert!(
        has(&findings, "AES603", "source_file_named"),
        "expected source_file_named, got: {findings:#?}"
    );
    // The message must name the offending file, not just the rule.
    let message = findings
        .iter()
        .find(|(c, v, _)| c == "AES603" && v == "source_file_named")
        .map(|(_, _, m)| m.as_str())
        .unwrap_or_default();
    assert!(
        message.contains("main.rs"),
        "the message must name the file, got: {message}"
    );
}

#[test]
fn aes603_fires_when_a_spec_carries_status() {
    let tmp = tempfile::tempdir().unwrap();
    let frd = conforming_frd().replace(
        "The feature does the work a surface delegates to.",
        "The feature is fully implemented and shipped in v3.7.",
    );
    write_workspace(tmp.path(), &frd);
    assert!(has(&audit(tmp.path()), "AES603", "status_leak"));
}

#[test]
fn aes603_distinguishes_status_leak_from_source_file_naming() {
    // The same document trips both purity types under one code.
    let tmp = tempfile::tempdir().unwrap();
    let frd = conforming_frd().replace(
        "The feature does the work a surface delegates to.",
        "Implemented: main.rs is wired and shipped in v3.7.",
    );
    write_workspace(tmp.path(), &frd);
    let findings = audit(tmp.path());
    assert!(has(&findings, "AES603", "status_leak"));
    assert!(has(&findings, "AES603", "source_file_named"));
}

#[test]
fn aes604_fires_when_the_frd_omits_its_backlog_link() {
    let tmp = tempfile::tempdir().unwrap();
    let frd = conforming_frd().replace(
        "- Backlog: [BACKLOG.md](BACKLOG.md) — real condition for this feature.\n",
        "",
    );
    write_workspace(tmp.path(), &frd);
    assert!(has(&audit(tmp.path()), "AES604", "no_backlog_link"));
}

#[test]
fn aes604_fires_when_the_frd_omits_its_prd_link() {
    let tmp = tempfile::tempdir().unwrap();
    let frd = conforming_frd().replace("- PRD: [PRD.md](../../PRD.md)\n", "");
    write_workspace(tmp.path(), &frd);
    assert!(has(&audit(tmp.path()), "AES604", "no_prd_link"));
}

#[test]
fn aes604_fires_when_a_feature_backlog_restates_master_sections() {
    let tmp = tempfile::tempdir().unwrap();
    write_workspace(tmp.path(), &conforming_frd());
    fs::write(
        tmp.path().join("crates/sample/BACKLOG.md"),
        "# BACKLOG — sample\n\n## Current Condition\n\n## Risk Register\n\n- A risk.\n",
    )
    .unwrap();
    assert!(has(&audit(tmp.path()), "AES604", "state_vocab_restated"));
}

#[test]
fn aes605_fires_when_a_doc_pair_folder_has_no_orchestrator() {
    let tmp = tempfile::tempdir().unwrap();
    write_workspace(tmp.path(), &conforming_frd());
    fs::remove_file(
        tmp.path()
            .join("crates/sample/src/agent_sample_orchestrator.rs"),
    )
    .unwrap();
    assert!(has(&audit(tmp.path()), "AES605", "no_orchestrator"));
}

#[test]
fn aes605_fires_when_a_kernel_folder_carries_a_doc_pair() {
    let tmp = tempfile::tempdir().unwrap();
    write_workspace(tmp.path(), &conforming_frd());
    fs::create_dir_all(tmp.path().join("crates/shared")).unwrap();
    fs::write(tmp.path().join("crates/shared/FRD.md"), "# FRD\n").unwrap();
    fs::write(tmp.path().join("crates/shared/BACKLOG.md"), "# BACKLOG\n").unwrap();
    assert!(has(&audit(tmp.path()), "AES605", "shared_has_docs"));
}

#[test]
fn prose_only_documents_are_audited_for_purity() {
    let tmp = tempfile::tempdir().unwrap();
    write_workspace(tmp.path(), &conforming_frd());
    fs::write(
        tmp.path().join("PRD.md"),
        "# PRD — sample\n\nThe checker reads defaults from main.rs and dispatches in lib.rs.\n",
    )
    .unwrap();
    assert!(
        has(&audit(tmp.path()), "AES603", "source_file_named"),
        "a PRD is a spec and must be audited for purity"
    );
}

#[test]
fn findings_are_deduplicated_and_ordered() {
    let tmp = tempfile::tempdir().unwrap();
    let frd = conforming_frd().replace("### FR-SAMPLE-001:", "### FR-001:");
    write_workspace(tmp.path(), &frd);
    let first = audit(tmp.path());
    let second = audit(tmp.path());
    assert_eq!(first, second, "two runs must agree");
    let mut sorted = first.clone();
    sorted.sort_by(|a, b| {
        (a.0.clone(), a.1.clone(), a.2.clone()).cmp(&(b.0.clone(), b.1.clone(), b.2.clone()))
    });
    assert_eq!(first, sorted, "findings must arrive sorted");
}

// ── AES606: Agent doc heading structure ────────────────────────────────────

#[test]
fn aes606_conforming_agents_reports_no_findings() {
    let tmp = tempfile::tempdir().unwrap();
    write_agents_workspace(tmp.path());
    let findings = audit(tmp.path());
    assert!(
        !has(&findings, "AES606", "h1_count") && !has(&findings, "AES606", "h2_missing"),
        "expected clean AGENTS.md, got: {findings:#?}"
    );
}

#[test]
fn aes606_fires_when_no_h1() {
    let tmp = tempfile::tempdir().unwrap();
    write_bad_agents(
        tmp.path(),
        "## Security\n\n- Explicit approval is required.\n",
    );
    let findings = audit(tmp.path());
    assert!(has(&findings, "AES606", "h1_count"));
}

#[test]
fn aes606_fires_when_multiple_h1() {
    let tmp = tempfile::tempdir().unwrap();
    write_bad_agents(
        tmp.path(),
        "# One\n\n# Two\n\n## Security\n\n- Explicit approval.\n",
    );
    let findings = audit(tmp.path());
    assert!(has(&findings, "AES606", "h1_count"));
}

#[test]
fn aes606_does_not_falsely_fire_on_sharp_commented_commands() {
    // A bash comment such as `# Tests (matches CI "Tests" job)` must NOT
    // count as an extra H1 — fenced code blocks are stripped first.
    let tmp = tempfile::tempdir().unwrap();
    write_bad_agents(
        tmp.path(),
        "\
# Sample AGENTS.md

## Security

- Be safe.

## Commands

```bash
# Tests (matches CI \"Tests\" job)
cargo nextest run
```

## Related Documents\n",
    );
    let findings = audit(tmp.path());
    assert!(
        !has(&findings, "AES606", "h1_count"),
        "fenced code block contents must not be treated as headings; got: {findings:#?}"
    );
}

#[test]
fn aes606_fires_when_required_h2_is_absent() {
    let tmp = tempfile::tempdir().unwrap();
    // Precedence and Definition of Done are missing from the required set.
    write_bad_agents(
        tmp.path(),
        "# Sample AGENTS.md\n\n\
## Security\n\n- Be safe.\n\n\
## Git Workflow\n\nUse a worktree.\n\n\
## Commands\n\n```bash\ntrue\n```\n\n\
## Related Documents\n\n- [PRD.md](PRD.md).\n",
    );
    let findings = audit(tmp.path());
    assert!(has(&findings, "AES606", "h2_missing"));
    let message = findings
        .iter()
        .find(|(c, v, _)| c == "AES606" && v == "h2_missing")
        .map(|(_, _, m)| m.as_str())
        .unwrap();
    // The message names the absent section(s), not just the rule.
    assert!(
        message.contains("Precedence") || message.contains("Definition of Done"),
        "the message must name the absent section(s); got: {message}"
    );
}

#[test]
fn aes606_allows_extra_and_free_h3_headings() {
    // Extra H2s and H3 headings are allowed; only required H2s are enforced.
    let tmp = tempfile::tempdir().unwrap();
    write_bad_agents(
        tmp.path(),
        "\
# Sample AGENTS.md

## Precedence

1. Safety rules.

## Security

- Be safe.

## Architecture

See [ARCHITECTURE.md](ARCHITECTURE.md) for the layer specification.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

MIT. See [LICENSE](LICENSE).

## Git Workflow

Use a worktree.

## Commands

```bash
true
```

## Definition of Done

Tests pass.

## Related Documents

- [PRD.md](PRD.md).\
",
    );
    let findings = audit(tmp.path());
    assert!(
        !has(&findings, "AES606", "h2_missing") && !has(&findings, "AES606", "h2_unexpected"),
        "extra H2/H3 headings should not trigger h2_missing; got: {findings:#?}"
    );
}

#[test]
fn aes606_fires_when_architecture_or_contributing_or_license_is_absent() {
    // Removing any of the three newest required sections must fire h2_missing.
    let tmp = tempfile::tempdir().unwrap();
    write_bad_agents(
        tmp.path(),
        "\
# Sample AGENTS.md

## Precedence

1. Safety rules.

## Security

- Be safe.

## License

MIT. See [LICENSE](LICENSE).

## Git Workflow

Use a worktree.

## Commands

```bash
true
```

## Definition of Done

Tests pass.

## Related Documents

- [PRD.md](PRD.md).\
",
    );
    let findings = audit(tmp.path());
    assert!(
        has(&findings, "AES606", "h2_missing"),
        "missing Architecture and Contributing should fire; got: {findings:#?}"
    );
    let message = findings
        .iter()
        .find(|(c, v, _)| c == "AES606" && v == "h2_missing")
        .map(|(_, _, m)| m.as_str())
        .unwrap();
    assert!(
        message.contains("Architecture") && message.contains("Contributing"),
        "message must name both missing sections; got: {message}"
    );
}

#[test]
fn aes606_fires_on_an_h2_outside_the_template() {
    // "Random Notes" is in neither the required nor the allowed set, so it
    // must be reported for demotion to H3 or removal.
    let tmp = tempfile::tempdir().unwrap();
    write_bad_agents(
        tmp.path(),
        "\
# Sample AGENTS.md

## Precedence

1. Safety rules.

## Security

- Be safe.

## Architecture

See [ARCHITECTURE.md](ARCHITECTURE.md) for the layer specification.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

MIT. See [LICENSE](LICENSE).

## Random Notes

Anything the author wanted to jot down.

## Git Workflow

Use a worktree.

## Commands

```bash
true
```

## Definition of Done

Tests pass.

## Related Documents

- [PRD.md](PRD.md).\
",
    );
    let findings = audit(tmp.path());
    assert!(
        has(&findings, "AES606", "h2_unexpected"),
        "an off-template H2 must fire h2_unexpected; got: {findings:#?}"
    );
    let message = findings
        .iter()
        .find(|(c, v, _)| c == "AES606" && v == "h2_unexpected")
        .map(|(_, _, m)| m.as_str())
        .unwrap();
    assert!(
        message.contains("random notes"),
        "message must name the off-template heading; got: {message}"
    );
}

#[test]
fn aes606_allows_the_project_specific_h2_set() {
    // The optional-but-agreed headings this project uses must stay silent.
    let tmp = tempfile::tempdir().unwrap();
    write_bad_agents(
        tmp.path(),
        "\
# Sample AGENTS.md

## Precedence

1. Safety rules.

## Security

- Be safe.

## Project Overview

What this project is.

## Build & dev

How to build it.

## Format & lint

How to lint it.

## Quality gates

The gate pipeline.

## Self-lint

How the project lints itself.

## Scan test projects

The fixture workspaces.

## MCP server & TUI

The server entry points.

## Architecture: AES 7-Layer System

See [ARCHITECTURE.md](ARCHITECTURE.md).

## Naming Convention

The filename rule.

## Workspace Packages Structure

Where each package lives.

## Skills & Roles

The skill catalogue.

## Branch Management

Which branches exist.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

MIT. See [LICENSE](LICENSE).

## Git Workflow

Use a worktree.

## Exit Code Contract

The exit codes.

## Pitfalls

Known traps.

## Commands

```bash
true
```

## Definition of Done

Tests pass.

## Related Documents

- [PRD.md](PRD.md).\
",
    );
    let findings = audit(tmp.path());
    assert!(
        !has(&findings, "AES606", "h2_unexpected") && !has(&findings, "AES606", "h2_missing"),
        "the project's own H2 set must be accepted; got: {findings:#?}"
    );
}
