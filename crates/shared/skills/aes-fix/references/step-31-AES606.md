# AES606 — Document Structure: ROADMAP.md Conformance

## What the rule checks

AES606 (rule code `AES606`, checker `capabilities_doc_roadmap` in the `doc-rules` crate) verifies ROADMAP.md conformance.

## Common violation causes

- ROADMAP.md missing a required H2 section.
- H2 heading text drifted from the registered template.

## How to fix (concrete steps)

1. Identify the violated rule code (AES606) from the scan report.
2. Add the missing H2 section per ROADMAP.md's template contract.
3. Restore drifted H2 headings to their registered shape.
4. Re-run `lint-arwaky-cli docs crates/shared/skills` or `lint-arwaky-cli scan .` and confirm the document finding is gone.

## Verify

```bash
lint-arwaky-cli scan . --filter AES606
# Confirm no AES606 finding remains.
```
