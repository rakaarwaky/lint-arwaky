# AES505 — Orphan: Agent Orphan

## What the rule checks

AES505 (rule code `AES505`, checker `capabilities_orphan_agent_analyzer` in the `orphan-rules` crate) verifies an agent must be reachable and wired in a container.

## Common violation causes

- An agent not registered in `root_*_container.rs` (or `.py` / `.ts`).
- An agent not reachable from any entry point.

## How to fix (concrete steps)

1. Identify the violated rule code (AES505) from the scan report.
2. Register the agent in `root_*_container.rs` (or `.py` / `.ts`).
3. Ensure the agent is reachable from an entry point.
4. Re-run `lint-arwaky-cli scan .` and confirm AES505 no longer fires on that file.

## Verify

```bash
lint-arwaky-cli scan . --filter AES505
# Confirm no AES505 finding remains.
```
