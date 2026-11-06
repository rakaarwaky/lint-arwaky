# Step 13: Quality: Code Duplication

## What the rule checks

AES305 (rule code `AES305`, checker `capabilities_code_duplication_analyzer` in the `quality-rules` crate) detects excessive duplicated non-import windows of logic across files.

## Common violation causes

- Two or more files share large non-import windows of duplicated code.
- Copy-pasted logic instead of a shared utility.

## How to fix (concrete steps)

1. Identify the violated rule code (AES305) and the duplicated files from the scan report.
2. Extract the shared logic into a pure utility (`*_utility`).
3. Have both files import and call the shared utility instead of duplicating.
4. Re-run `lint-arwaky-cli scan .` and confirm AES305 no longer fires on those files.

## Verify

```bash
lint-arwaky-cli scan . --filter AES305
# Confirm no AES305 finding remains.
```
