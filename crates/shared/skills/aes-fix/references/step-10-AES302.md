# AES302 — Quality: File Too Short

## What the rule checks

AES302 (rule code `AES302`, checker `capabilities_line_checker` in the `quality-rules` crate) fires when a file is below the configured minimum line count (`code_analysis.min_lines`).

## Common violation causes

- A file has fewer lines than the configured minimum.
- A file that should be merged into a parent module or deleted.

## How to fix (concrete steps)

1. Identify the violated rule code (AES302) from the scan report.
2. Merge the component into the parent module so it meets the minimum size.
3. Or delete the file if it no longer serves a purpose.
4. Re-run `lint-arwaky-cli scan .` and confirm AES302 no longer fires on that file.

## Verify

```bash
lint-arwaky-cli scan . --filter AES302
# Confirm no AES302 finding remains.
```
