# AES705 — Project Structure: Member-Root Placement

## What the rule checks

AES705 (rule code `AES705`, checker `capabilities_member_root_placement_auditor` in the `structure-rules` crate) verifies `crates/`, `modules/`, `packages/` hold folders (not loose files).

## Common violation causes

- A loose `.rs`/`.py`/`.ts` file sits directly in `crates/`, `modules/`, or `packages/` instead of in a folder.

## How to fix (concrete steps)

1. Identify the violated rule code (AES705) from the scan report.
2. Move the loose file into a folder under `crates/`, `modules/`, or `packages/`.
3. Re-run `lint-arwaky-cli scan .` and confirm AES705 no longer fires on that file.

## Verify

```bash
lint-arwaky-cli scan . --filter AES705
# Confirm no AES705 finding remains.
```
