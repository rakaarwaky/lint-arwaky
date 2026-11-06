# Step 20: Orphan: Taxonomy Orphan

## What the rule checks

AES501 (rule code `AES501`, checker `capabilities_orphan_taxonomy_analyzer` in the `orphan-rules` crate) verifies a taxonomy file must be imported by a contract or capability to be discoverable, and reachable from an entry point / barrel.

## Common violation causes

- A taxonomy file exists but nothing imports it.
- A taxonomy file not reachable from any entry point.

## How to fix (concrete steps)

1. Identify the orphaned file's layer and rule code (AES501) from the scan report.
2. Add an import from a contract or capability so the taxonomy is discoverable.
3. Ensure the file is reachable from an entry point / barrel.
4. Re-run `lint-arwaky-cli scan .` and confirm AES501 no longer fires on that file.

## Verify

```bash
lint-arwaky-cli scan . --filter AES501
# Confirm no AES501 finding remains.
```
