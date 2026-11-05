# AES607 — Document Structure: BACKLOG.md Conformance

## What the rule checks

AES607 (rule code `AES607`, checker `capabilities_doc_backlog` in the `doc-rules` crate) verifies BACKLOG.md conformance.

## Common violation causes

- BACKLOG.md missing a required H2 section.
- H2 heading text drifted from the registered template.

## How to fix (concrete steps)

1. Identify the violated rule code (AES607) from the scan report.
2. Add the missing H2 section per BACKLOG.md's template contract.
3. Restore drifted H2 headings to their registered shape.
4. Re-run `lint-arwaky-cli docs crates/shared/skills` or `lint-arwaky-cli scan .` and confirm the document finding is gone.

## Verify

```bash
lint-arwaky-cli scan . --filter AES607
# Confirm no AES607 finding remains.
```
