# Step 41: Project Structure: PR-LA Folder Mapping

## What the rule checks

AES706 (rule code `AES706`, checker `capabilities_requirement_folder_parity` in the `structure-rules` crate) verifies every crate folder's FR ids match its folder name.

## Common violation causes

- A crate folder's FR ids don't match its folder name.

## How to fix (concrete steps)

1. Identify the violated rule code (AES706) from the scan report.
2. Align the crate folder's FR ids with its folder name (rename folder or fix FR ids).
3. Re-run `lint-arwaky-cli scan .` and confirm AES706 no longer fires on that file.

## Verify

```bash
lint-arwaky-cli scan . --filter AES706
# Confirm no AES706 finding remains.
```
