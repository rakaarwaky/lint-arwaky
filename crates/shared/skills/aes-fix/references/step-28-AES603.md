# AES603 — Document Structure: ARCHITECTURE.md Conformance

## What the rule checks

AES603 (rule code `AES603`, checker `capabilities_doc_architecture` in the `doc-rules` crate) verifies ARCHITECTURE.md conformance.

## Common violation causes

- ARCHITECTURE.md missing a required H2 section.
- H2 heading text drifted from the registered template.

## How to fix (concrete steps)

1. Identify the violated rule code (AES603) from the scan report.
2. Add the missing H2 section per ARCHITECTURE.md's template contract.
3. Restore drifted H2 headings to their registered shape.
4. Re-run `lint-arwaky-cli docs crates/shared/skills` or `lint-arwaky-cli scan .` and confirm the document finding is gone.

## Verify

```bash
lint-arwaky-cli scan . --filter AES603
# Confirm no AES603 finding remains.
```
