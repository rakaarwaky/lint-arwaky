# Step 34: Document Structure: FRD.md Conformance

## What the rule checks

AES609 (rule code `AES609`, checker `capabilities_doc_frd` in the `doc-rules` crate) verifies FRD.md conformance.

## Common violation causes

- FRD.md missing a required H2 section.
- H2 heading text drifted from the registered template.

## How to fix (concrete steps)

1. Identify the violated rule code (AES609) from the scan report.
2. Add the missing H2 section per FRD.md's template contract.
3. Restore drifted H2 headings to their registered shape.
4. Re-run `lint-arwaky-cli docs crates/shared/skills` or `lint-arwaky-cli scan .` and confirm the document finding is gone.

## Verify

```bash
lint-arwaky-cli scan . --filter AES609
# Confirm no AES609 finding remains.
```
