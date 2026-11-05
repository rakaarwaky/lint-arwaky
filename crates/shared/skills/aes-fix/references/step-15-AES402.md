# AES402 — Role/Layer: Contract ABC-Only

## What the rule checks

AES402 (rule code `AES402`, checker `capabilities_contract_*_role_auditor` in the `role-rules` crate) verifies contract files define ABCs/protocols only, no implementation.

## Common violation causes

- A contract file contains method bodies / implementation instead of only ABC signatures.

## How to fix (concrete steps)

1. Identify the violated rule code (AES402) from the scan report.
2. Move method bodies out of the contract file into a capability that implements the protocol.
3. Keep contract files to ABC/protocol signatures only.
4. Re-run `lint-arwaky-cli scan .` and confirm AES402 no longer fires on that file.

## Verify

```bash
lint-arwaky-cli scan . --filter AES402
# Confirm no AES402 finding remains.
```
