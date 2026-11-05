# AES401 — Role/Layer: Taxonomy Primitive Purity

## What the rule checks

AES401 (rule code `AES401`, checker `capabilities_taxonomy_role_auditor` in the `role-rules` crate) verifies taxonomy files hold only VOs/constants, no logic.

## Common violation causes

- A taxonomy file contains computation or I/O instead of pure VOs/constants.
- Business logic leaked into a taxonomy primitive file.

## How to fix (concrete steps)

1. Identify the violated rule code (AES401) from the scan report.
2. Move computation/I/O out of the taxonomy file into a capability or utility.
3. Keep taxonomy files to pure VOs/constants only.
4. Re-run `lint-arwaky-cli scan .` and confirm AES401 no longer fires on that file.

## Verify

```bash
lint-arwaky-cli scan . --filter AES401
# Confirm no AES401 finding remains.
```
