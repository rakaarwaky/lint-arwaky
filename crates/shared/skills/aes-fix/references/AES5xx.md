# AES5xx — Orphan Detection

## What the rule checks

Orphan detection lives in the `orphan-rules` crate. In the codebase these rules are emitted as **AES501–AES506** (the `xx` in this file's name is the family bucket; the actual rule codes are AES501–AES506). Each check verifies that a file in a given layer is reachable and/or wired so it cannot be dead code:

- **AES501 TAXONOMY_ORPHAN** — a taxonomy file must be imported by a contract or capability to be discoverable, and reachable from an entry point / barrel.
- **AES502 CONTRACT_ORPHAN** — a contract/protocol must be wired by a container; an aggregate must have an implementing agent; a protocol must have an implementing capability.
- **AES503 CAPABILITIES_ORPHAN** — a capability must be reachable from an entry point and wired in `root_*_container`.
- **AES504 UTILITY_ORPHAN** — a utility must be imported by a consumer layer and reachable from an entry point.
- **AES505 AGENT_ORPHAN** — an agent must be reachable and wired in a container.
- **AES506 SURFACE_ORPHAN** — a surface must be reachable from an entry point.

Checkers: `capabilities_orphan_taxonomy_analyzer`, `capabilities_orphan_contract_analyzer`, `capabilities_orphan_capabilities_analyzer`, `capabilities_orphan_utility_analyzer`, `capabilities_orphan_agent_analyzer`, `capabilities_orphan_surfaces_analyzer`, orchestrated by `agent_orphan_orchestrator`.

## Common violation causes

- File exists in a layer folder but nothing imports it.
- Contract/protocol has no implementing capability or agent.
- Agent/capability not registered in `root_*_container.rs` (or `.py` / `.ts`).
- Utility imported only by lower-layer files (violates the layer direction).
- Surface not reachable from any entry file.

## How to fix (concrete steps)

1. Identify the orphaned file's layer and rule code from the scan report (AES501–AES506).
2. Add an import from a higher-layer / consumer file (contract → taxonomy, capability → contract, surface → agent/capability, etc.).
3. Wire the agent/capability in `root_*_container` so the DI container can resolve it.
4. Ensure the file is reachable from an entry point (import chain traces back to `root_*_entry`).
5. If the file is genuinely unused, remove it rather than keeping dead code.
6. Re-run `lint-arwaky-cli scan .` and confirm the orphan finding is gone.

## Example

Before (AES505 AGENT_ORPHAN — agent not wired):
```python
# surface/entry.py imports nothing from agents/
```

After:
```python
# surface/entry.py
from agents.my_agent import AgentMyFeature
# and in root/root_container.py:
container.register(AgentMyFeature)
```

## Verify

```bash
lint-arwaky-cli scan .
# Confirm no AES501–AES506 finding remains for the affected file.
```
