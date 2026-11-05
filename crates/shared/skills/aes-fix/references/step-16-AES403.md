# AES403 — Role/Layer: Capabilities Protocol Implementation

## What the rule checks

AES403 (rule code `AES403`, checker `capabilities_capabilities_*_role_auditor` in the `role-rules` crate) verifies capability files implement a protocol, with no stray agent/surface logic.

## Common violation causes

- A capability file calls other agents or surfaces directly instead of implementing its protocol.

## How to fix (concrete steps)

1. Identify the violated rule code (AES403) from the scan report.
2. Remove direct agent/surface calls from the capability file.
3. Implement the protocol through its contract; delegate orchestration to agents.
4. Re-run `lint-arwaky-cli scan .` and confirm AES403 no longer fires on that file.

## Verify

```bash
lint-arwaky-cli scan . --filter AES403
# Confirm no AES403 finding remains.
```
