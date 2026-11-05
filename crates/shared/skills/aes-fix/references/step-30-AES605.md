# AES605 — Document Structure: PRD.md Conformance

## What the rule checks

AES605 (rule code `AES605`, checker `capabilities_doc_prd` in the `doc-rules` crate) verifies PRD.md conformance.

## Common violation causes

- PRD.md missing a required H2 section.
- H2 heading text drifted from the registered template.

## How to fix (concrete steps)

1. Identify the violated rule code (AES605) from the scan report.
2. Add the missing H2 section per PRD.md's template contract.
3. Restore drifted H2 headings to their registered shape.
4. Re-run `lint-arwaky-cli docs crates/shared/skills` or `lint-arwaky-cli scan .` and confirm the document finding is gone.

## Verify

```bash
lint-arwaky-cli scan . --filter AES605
# Confirm no AES605 finding remains.
```
