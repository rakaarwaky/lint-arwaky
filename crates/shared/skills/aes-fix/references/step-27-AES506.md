# Step 25: Orphan: Surface Orphan

## What the rule checks

AES506 (rule code `AES506`, checker `capabilities_orphan_surfaces_analyzer` in the `orphan-rules` crate) verifies a surface must be reachable from an entry point.

## Common violation causes

- A surface not reachable from any entry file.

## How to fix (concrete steps)

1. Identify the violated rule code (AES506) from the scan report.
2. Wire the surface so it is reachable from an entry file.
3. Re-run `lint-arwaky-cli scan .` and confirm AES506 no longer fires on that file.

## Verify

```bash
lint-arwaky-cli scan . --filter AES506
# Confirm no AES506 finding remains.
```
