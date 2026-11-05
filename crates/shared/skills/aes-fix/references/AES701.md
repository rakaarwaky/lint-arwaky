# AES701 — Project Structure: Shared/Layer Folder Purity

## What the rule checks

AES701 (rule code `AES701`, checker `capabilities_shared_purity_auditor` in the `structure-rules` crate) verifies shared folders hold only taxonomy, utility, and contract files; each shared layer folder carries DATA.md + BACKLOG.md.

## Common violation causes

- A shared folder contains a non-taxonomy/utility/contract file (e.g. an agent or surface).
- A shared layer folder missing its DATA.md + BACKLOG.md doc pair.

## How to fix (concrete steps)

1. Identify the violated rule code (AES701) from the scan report.
2. Remove non-taxonomy/utility/contract files from shared folders.
3. Add DATA.md + BACKLOG.md to each shared layer folder.
4. Re-run `lint-arwaky-cli scan .` and confirm AES701 no longer fires on that file.

## Verify

```bash
lint-arwaky-cli scan . --filter AES701
# Confirm no AES701 finding remains.
```
