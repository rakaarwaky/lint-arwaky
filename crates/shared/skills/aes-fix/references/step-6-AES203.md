# AES203 — Import Graph: Unused Import

## What the rule checks

AES203 (rule code `AES203`, checker `capabilities_import_unused_checker` in the `import-rules` crate) detects imports that are never used in the file.

## Common violation causes

- An import statement references a module/symbol that is never referenced in the file body.
- Leftover import after refactoring removed its usage.

## How to fix (concrete steps)

1. Identify the violated rule code (AES203) and the unused import from the scan report.
2. Delete the unused import statement.
3. If the symbol is needed elsewhere, add it only where used.
4. Re-run `lint-arwaky-cli scan .` or `lint-arwaky-cli fix <path> --filter AES203` to auto-remove.
5. Confirm AES203 no longer fires on that file.

## Verify

```bash
lint-arwaky-cli scan . --filter AES203
# Confirm no AES203 finding remains.
```
