# AES406 — Role/Layer: Surface Event Mapping

## What the rule checks

AES406 (rule code `AES406`, checker `capabilities_surface_role_auditor` in the `role-rules` crate) verifies surface files map input events to VOs, tier-aware.

## Common violation causes

- A surface file contains business logic instead of only mapping input events to VOs.

## How to fix (concrete steps)

1. Identify the violated rule code (AES406) from the scan report.
2. Move business logic out of the surface file into a capability.
3. Keep surface files to mapping input events to VOs only.
4. Re-run `lint-arwaky-cli scan .` and confirm AES406 no longer fires on that file.

## Verify

```bash
lint-arwaky-cli scan . --filter AES406
# Confirm no AES406 finding remains.
```
