# AES303 — Quality: Mandatory Definition Missing

## What the rule checks

AES303 (rule code `AES303`, checker `capabilities_mandatory_definition_checker` in the `quality-rules` crate) fires when a file declares no struct/enum/trait/class/interface/type of its own.

- Skipped for `utility` and `surface` layers (no definition required there).

## Common violation causes

- A file holds only functions with no type definition.
- A production file that should declare a type but doesn't.

## How to fix (concrete steps)

1. Identify the violated rule code (AES303) from the scan report.
2. Group the functions into a struct/class or implement an interface/trait.
3. For utility and surface files, no definition is required — they are exempt.
4. Re-run `lint-arwaky-cli scan .` and confirm AES303 no longer fires on that file.

## Verify

```bash
lint-arwaky-cli scan . --filter AES303
# Confirm no AES303 finding remains.
```
