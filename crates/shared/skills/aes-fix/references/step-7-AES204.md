# AES204 — Import Graph: Dummy/Stub Import

## What the rule checks

AES204 (rule code `AES204`, checker `capabilities_dummy_import_checker` in the `import-rules` crate) detects placeholder imports, stub functions, and dummy trait impls kept only to satisfy a linter.

## Common violation causes

- An import kept only to satisfy a linter, with stub usage.
- `todo!()`, `unimplemented!()`, or stub functions referencing the import.
- Dummy trait impls that do nothing.

## How to fix (concrete steps)

1. Identify the violated rule code (AES204) and the dummy import from the scan report.
2. Remove the dummy import and its stub.
3. Delete the unused symbol or implement it properly.
4. Re-run `lint-arwaky-cli scan .` and confirm AES204 no longer fires on that file.

## Verify

```bash
lint-arwaky-cli scan . --filter AES204
# Confirm no AES204 finding remains.
```
