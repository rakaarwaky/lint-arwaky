# AES301 — Quality: File Too Large

## What the rule checks

AES301 (rule code `AES301`, checker `capabilities_line_checker` in the `quality-rules` crate) fires when a file exceeds the layer's configured maximum line count (`code_analysis.max_lines`).

## Common violation causes

- A file exceeds the configured maximum line count.
- A single file accumulating many responsibilities instead of being split.

## How to fix (concrete steps)

1. Identify the violated rule code (AES301) from the scan report.
2. Split the file into smaller, focused modules by responsibility.
3. Move shared helpers to a `*_utility` file if duplicated across modules.
4. Re-run `lint-arwaky-cli scan .` and confirm AES301 no longer fires on that file.

## Verify

```bash
lint-arwaky-cli scan . --filter AES301
# Confirm no AES301 finding remains.
```
