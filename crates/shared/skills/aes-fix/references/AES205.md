# AES205 — Import Graph: Circular Dependency

## What the rule checks

AES205 (rule code `AES205`, checker `capabilities_cycle_import_analyzer` in the `import-rules` crate) performs circular dependency analysis (3-color DFS) across modules/crates.

- Two modules importing each other form a cycle and trigger a finding.

## Common violation causes

- Two modules import each other, forming a cycle.
- Layer boundary violation that indirectly creates a cycle.

## How to fix (concrete steps)

1. Identify the violated rule code (AES205) and the two modules/layers involved from the scan report.
2. Extract the shared type or trait downward into `taxonomy` or `contract`; both sides import that shared layer instead of each other.
3. Break the cycle so dependencies flow one direction only.
4. Re-run `lint-arwaky-cli scan .` and confirm AES205 no longer fires on that file.

## Verify

```bash
lint-arwaky-cli scan . --filter AES205
# Confirm no AES205 finding remains.
```
