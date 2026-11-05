# AES404 — Role/Layer: Utility Purity

## What the rule checks

AES404 (rule code `AES404`, checker `capabilities_utility_*_role_auditor` in the `role-rules` crate) verifies utility files are pure stateless helpers, no I/O, no domain state.

## Common violation causes

- A utility file performs I/O, mutates global state, or holds domain data.

## How to fix (concrete steps)

1. Identify the violated rule code (AES404) from the scan report.
2. Remove I/O, global-state mutation, and domain data from the utility file.
3. Move side effects to a surface or capability; keep utilities pure.
4. Re-run `lint-arwaky-cli scan .` and confirm AES404 no longer fires on that file.

## Verify

```bash
lint-arwaky-cli scan . --filter AES404
# Confirm no AES404 finding remains.
```
