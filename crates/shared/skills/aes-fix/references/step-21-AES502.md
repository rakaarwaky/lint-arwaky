# AES502 — Orphan: Contract Orphan

## What the rule checks

AES502 (rule code `AES502`, checker `capabilities_orphan_contract_analyzer` in the `orphan-rules` crate) verifies a contract/protocol must be wired by a container; an aggregate must have an implementing agent; a protocol must have an implementing capability.

## Common violation causes

- A contract/protocol has no implementing capability or agent.
- A contract not wired by a container.

## How to fix (concrete steps)

1. Identify the violated rule code (AES502) from the scan report.
2. Wire the contract/protocol in `root_*_container.rs` (or `.py` / `.ts`).
3. Ensure an aggregate has an implementing agent and a protocol has an implementing capability.
4. Re-run `lint-arwaky-cli scan .` and confirm AES502 no longer fires on that file.

## Verify

```bash
lint-arwaky-cli scan . --filter AES502
# Confirm no AES502 finding remains.
```
