# Step 37: Project Structure: Feature Folder Health

## What the rule checks

AES702 (rule code `AES702`, checker `capabilities_feature_health_auditor` in the `structure-rules` crate) verifies a feature folder holds capabilities + orchestrator pairs, has no foreign files, and carries FRD.md + BACKLOG.md doc pairs.

## Common violation causes

- A feature folder lacks a capabilities file, an orchestrator, or its FRD/BACKLOG doc pair.
- Foreign files present in a feature folder.

## How to fix (concrete steps)

1. Identify the violated rule code (AES702) from the scan report.
2. Add the missing capabilities file, orchestrator, or FRD/BACKLOG doc pair.
3. Remove foreign files from the feature folder.
4. Re-run `lint-arwaky-cli scan .` and confirm AES702 no longer fires on that file.

## Verify

```bash
lint-arwaky-cli scan . --filter AES702
# Confirm no AES702 finding remains.
```
