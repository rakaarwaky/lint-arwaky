# AES6xx — Document Structure Checks

## What the rule checks

Document structure checks live in the `doc-rules` crate. In the codebase these checks emit **AES601–AES610**, one per project document type:

- **AES601** — AGENTS.md conformance (agent-operations doc shape + H2 line-drift).
- **AES602** — README.md conformance.
- **AES603** — ARCHITECTURE.md conformance.
- **AES604** — CONTRIBUTING.md conformance.
- **AES605** — PRD.md conformance.
- **AES606** — ROADMAP.md conformance.
- **AES607** — BACKLOG.md conformance.
- **AES608** — DATA.md conformance.
- **AES609** — FRD.md conformance.
- **AES610** — DESIGN.md conformance.

Checkers: `capabilities_doc_agents`, `capabilities_doc_readme`, `capabilities_doc_architecture`, `capabilities_doc_contributing`, `capabilities_doc_prd`, `capabilities_doc_roadmap`, `capabilities_doc_backlog`, `capabilities_doc_data`, `capabilities_doc_frd`, `capabilities_doc_design`, orchestrated by `agent_doc_orchestrator`.

Each per-doc capability stamps its rule code on findings and checks H1/H2 shape, required sections, aspect entries (exemplar/rules/template/workflow), crosslinks, and line-drift invariants.

## Common violation causes

- Document missing a required H2 section or aspect entry.
- H2 heading text drifted from the registered template (line-drift).
- Missing exemplar/rules/template/workflow entries for an aspect.
- Broken or missing crosslinks between documents.
- Missing required frontmatter/fields for the doc type.

## How to fix (concrete steps)

1. Identify the violated rule code (AES601–AES610) and document from the scan report.
2. Open the named document and add the missing section/aspect entry per the doc's template contract.
3. Restore drifted H2 headings to their registered shape.
4. Repair crosslinks so every referenced document exists and resolves.
5. Re-run `lint-arwaky-cli docs crates/shared/skills` (doc invariant gate) or `lint-arwaky-cli scan .` and confirm the document finding is gone.

## Example

Before (AES604 — CONTRIBUTING.md missing the `workflow` aspect):
```markdown
## Contributing
...
```

After:
```markdown
## Contributing
### Workflow
...
```

## Verify

```bash
lint-arwaky-cli scan .
# Confirm no AES601–AES610 finding remains for the affected document.
# Doc invariant gate: lint-arwaky-cli docs crates/shared/skills
```
