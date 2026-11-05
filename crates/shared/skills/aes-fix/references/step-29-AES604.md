# AES604 — Document Structure: CONTRIBUTING.md Conformance

## What the rule checks

AES604 (rule code `AES604`, checker `capabilities_doc_contributing` in the `doc-rules` crate) verifies CONTRIBUTING.md conformance.

## Common violation causes

- CONTRIBUTING.md missing a required H2 section.
- H2 heading text drifted from the registered template.

## How to fix (concrete steps)

1. Identify the violated rule code (AES604) from the scan report.
2. Add the missing H2 section per CONTRIBUTING.md's template contract.
3. Restore drifted H2 headings to their registered shape.
4. Re-run `lint-arwaky-cli docs crates/shared/skills` or `lint-arwaky-cli scan .` and confirm the document finding is gone.

## Verify

```bash
lint-arwaky-cli scan . --filter AES604
# Confirm no AES604 finding remains.
```
