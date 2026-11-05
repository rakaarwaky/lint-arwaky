# AES703 — Project Structure: Surface Folder Purity

## What the rule checks

AES703 (rule code `AES703`, checker `capabilities_surface_purity_auditor` in the `structure-rules` crate) verifies surface-dominated folders carry only surface files plus permitted support files.

## Common violation causes

- A surface-dominated folder contains non-surface files.

## How to fix (concrete steps)

1. Identify the violated rule code (AES703) from the scan report.
2. Remove non-surface files from surface-dominated folders (keep permitted support files only).
3. Re-run `lint-arwaky-cli scan .` and confirm AES703 no longer fires on that file.

## Verify

```bash
lint-arwaky-cli scan . --filter AES703
# Confirm no AES703 finding remains.
```
