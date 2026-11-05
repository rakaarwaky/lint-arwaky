# AES608 — Document Structure: DATA.md Conformance

## What the rule checks

AES608 (rule code `AES608`, checker `capabilities_doc_data` in the `doc-rules` crate) verifies DATA.md conformance.

## Common violation causes

- DATA.md missing a required H2 section.
- H2 heading text drifted from the registered template.

## How to fix (concrete steps)

1. Identify the violated rule code (AES608) from the scan report.
2. Add the missing H2 section per DATA.md's template contract.
3. Restore drifted H2 headings to their registered shape.
4. Re-run `lint-arwaky-cli docs crates/shared/skills` or `lint-arwaky-cli scan .` and confirm the document finding is gone.

## Verify

```bash
lint-arwaky-cli scan . --filter AES608
# Confirm no AES608 finding remains.
```
