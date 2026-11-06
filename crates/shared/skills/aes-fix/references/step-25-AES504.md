# Step 23: Orphan: Utility Orphan

## What the rule checks

AES504 (rule code `AES504`, checker `capabilities_orphan_utility_analyzer` in the `orphan-rules` crate) verifies a utility must be imported by a consumer layer and reachable from an entry point.

## Common violation causes

- A utility is imported only by lower-layer files (violates layer direction).
- A utility not imported by any consumer layer.

## How to fix (concrete steps)

1. Identify the violated rule code (AES504) from the scan report.
2. Add an import from a higher-layer/consumer file.
3. Ensure the utility is reachable from an entry point.
4. Re-run `lint-arwaky-cli scan .` and confirm AES504 no longer fires on that file.

## Verify

```bash
lint-arwaky-cli scan . --filter AES504
# Confirm no AES504 finding remains.
```
