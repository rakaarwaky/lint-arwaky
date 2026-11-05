# AES304 — Quality: Bypass Annotation

## What the rule checks

AES304 (rule code `AES304`, checker `capabilities_check_bypass_checker` in the `quality-rules` crate) detects bypass annotations, panics, and fallback calls that silently discard errors.

- `unwrap_or_default`, `.unwrap()`, `""`, panic annotations, `noqa`, `type: ignore`, `#[allow]`, `@ts-ignore`, `eslint-disable`.

## Common violation causes

- A file silently discards errors with `unwrap_or_default`, `.unwrap()`, `""`, or panic annotations.
- Suppression comments masking real errors instead of fixing the root cause.

## How to fix (concrete steps)

1. Identify the violated rule code (AES304) from the scan report.
2. Replace silent discards with proper error propagation.
3. Wrap errors into taxonomy VOs or return `Result`.
4. Fix the root cause and delete the suppression comment rather than auto-fixing over it.
5. Re-run `lint-arwaky-cli scan .` or `lint-arwaky-cli fix . --filter AES304` and confirm AES304 clears.

## Verify

```bash
lint-arwaky-cli scan . --filter AES304
# Confirm no AES304 finding remains.
```
