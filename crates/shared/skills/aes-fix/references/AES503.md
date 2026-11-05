# AES503 — Orphan: Capabilities Orphan

## What the rule checks

AES503 (rule code `AES503`, checker `capabilities_orphan_capabilities_analyzer` in the `orphan-rules` crate) verifies a capability must be reachable from an entry point and wired in `root_*_container`.

## Common violation causes

- A capability exists but nothing reaches it from an entry point.
- A capability not registered in `root_*_container`.

## How to fix (concrete steps)

1. Identify the violated rule code (AES503) from the scan report.
2. Add an entry-point path that reaches the capability.
3. Register the capability in `root_*_container.rs` (or `.py` / `.ts`).
4. Re-run `lint-arwaky-cli scan .` and confirm AES503 no longer fires on that file.

## Verify

```bash
lint-arwaky-cli scan . --filter AES503
# Confirm no AES503 finding remains.
```
