# Step 41: Project Structure: PR-LA Folder Mapping

## What the rule checks

AES706 (rule code `AES706`, checker `capabilities_requirement_folder_parity` in the `structure-rules` crate) verifies every crate folder's FR ids match its folder name.

- A `PR-LA-001..N` id in PRD.md/ROADMAP.md maps 1:1 to a crate folder by positional order.
- The PR-LA heading title after the colon separator must match the crate folder name.
- Every crate folder at an assigned position must carry an FRD.md document.
- Requirement identifiers in a crate folder's FRD.md must carry a feature prefix derived from the crate folder name.

## Common violation causes

- A crate folder's FR ids don't match its folder name.
- A `PR-LA-001..N` id has no matching crate folder at its position.
- A crate folder has no FRD.md document.
- A PR-LA heading title after the colon does not match the crate folder name.

## How to fix (concrete steps)

1. Identify the violated rule code (AES706) from the scan report.
2. For title mismatches: rename the PR-LA heading in PRD.md/ROADMAP.md so the text after the colon separator matches the crate folder name.
3. For FR prefix mismatches: update the FR ids in the crate folder's FRD.md to use the crate folder's feature prefix.
4. For missing FRD: add an FRD.md document at the crate folder.
5. For unmapped PR-LA ids: add the missing crate folder at the expected position.
6. Re-run `lint-arwaky-cli scan .` and confirm AES706 no longer fires.

## Verify

```bash
lint-arwaky-cli scan . --filter AES706
# Confirm no AES706 finding remains.
```
