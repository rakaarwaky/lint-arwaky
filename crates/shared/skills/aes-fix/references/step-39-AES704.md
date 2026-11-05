# AES704 — Project Structure: Test-Suite Coverage

## What the rule checks

AES704 (rule code `AES704`, checker `capabilities_test_suite_coverage_auditor` in the `structure-rules` crate) verifies every feature folder owning source carries at least one file per test type (`contract_`, `unit_`, `integration_`, `dogfood_`, `smoke_`, `e2e_`, `acceptance_`) plus one `bench_` benchmark; the prefix is the virtual folder (shared with AES103 vocabulary).

## Common violation causes

- A feature folder's `tests/`/`benches/` is missing a test-type category or the bench file.

## How to fix (concrete steps)

1. Identify the violated rule code (AES704) and the missing test type from the scan report.
2. Add one file per missing test type with the correct flat prefix (`contract_`, `unit_`, `integration_`, `dogfood_`, `smoke_`, `e2e_`, `acceptance_`) in `tests/` or `bench_<subject>` in `benches/`.
3. Keep tests/benches flat — no subdirectories; the prefix IS the virtual folder.
4. Re-run `lint-arwaky-cli scan .` and confirm AES704 no longer fires on that file.

## Verify

```bash
lint-arwaky-cli scan . --filter AES704
# Confirm no AES704 finding remains.
```
