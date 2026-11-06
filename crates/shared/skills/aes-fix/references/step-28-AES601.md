# Step 26: Document Structure: AGENTS.md Conformance

## What the rule checks

AES601 (rule code `AES601`, checker `capabilities_doc_agents` in the `doc-rules` crate) verifies AGENTS.md conformance: agent-operations doc shape + H2 line-drift.

## Common violation causes

- AGENTS.md missing a required H2 section or aspect entry.
- H2 heading text drifted from the registered template (line-drift).
- Missing exemplar/rules/template/workflow entries for an aspect.

## How to fix (concrete steps)

1. Identify the violated rule code (AES601) and document from the scan report.
2. Add the missing H2 section / aspect entry per AGENTS.md's template contract.
3. Restore drifted H2 headings to their registered shape.
4. Repair crosslinks so every referenced document exists and resolves.
5. Re-run `lint-arwaky-cli docs crates/shared/skills` or `lint-arwaky-cli scan .` and confirm the document finding is gone.

## Verify

```bash
lint-arwaky-cli scan . --filter AES601
# Confirm no AES601 finding remains.
```
