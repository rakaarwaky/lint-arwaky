# Step 18: Role/Layer: Agent Orchestration Only

## What the rule checks

AES405 (rule code `AES405`, checker `capabilities_agent_*_role_auditor` in the `role-rules` crate) verifies agent files orchestrate only, no I/O, no computation, no local domain data.

## Common violation causes

- An agent file does I/O, computation, or holds local domain data instead of delegating to capabilities/taxonomy/utility.

## How to fix (concrete steps)

1. Identify the violated rule code (AES405) from the scan report.
2. Move I/O/computation/domain data out of the agent file into capabilities/taxonomy/utility.
3. Keep agent files to orchestration only.
4. Re-run `lint-arwaky-cli scan .` and confirm AES405 no longer fires on that file.

## Verify

```bash
lint-arwaky-cli scan . --filter AES405
# Confirm no AES405 finding remains.
```
