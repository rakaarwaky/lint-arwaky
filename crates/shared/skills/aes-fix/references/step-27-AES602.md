# Step 27: Document Structure: README.md Conformance

## What the rule checks

AES602 (rule code `AES602`, checker `capabilities_doc_readme` in the `doc-rules` crate) verifies README.md conformance.

## Common violation causes

- README.md missing a required H2 section.
- H2 heading text drifted from the registered template.

## How to fix (concrete steps)

1. Identify the violated rule code (AES602) from the scan report.
2. Add the missing H2 section per README.md's template contract.
3. Restore drifted H2 headings to their registered shape.
4. Re-run `lint-arwaky-cli docs crates/shared/skills` or `lint-arwaky-cli scan .` and confirm the document finding is gone.

## Verify

```bash
lint-arwaky-cli scan . --filter AES602
# Confirm no AES602 finding remains.
```
