# AES2xx — Import Graph Validation

## What the rule checks

Import graph validation lives in the `import-rules` crate. In the codebase these checks emit **AES201–AES205** (the `xx` in this file's name is the family bucket; the actual rule codes are AES201–AES205). Each check enforces unidirectional dependency flow across the 7-layer AES architecture:

- **AES201** — Forbidden layer imports: upper layers may not import layers below their allowed boundary (unidirectional flow compliance).
- **AES202** — Mandatory layer imports: contract & aggregate enforcement — a capability must import its contract, an agent must import its aggregate.
- **AES203** — Unused import detection: imports that are never used in the file.
- **AES204** — Dummy/stub import & function detection: placeholder imports, stub functions, dummy trait impls.
- **AES205** — Circular dependency analysis (3-color DFS): import cycles between modules/crates.

Checkers: `capabilities_import_forbidden_checker` (AES201), `capabilities_import_mandatory_checker` (AES202), `capabilities_import_unused_checker` (AES203), `capabilities_dummy_import_checker` (AES204), `capabilities_cycle_import_analyzer` (AES205), orchestrated by `agent_import_orchestrator`.

## Common violation causes

- A surface imports an agent directly instead of going through capabilities/contract.
- A capability imports a surface or agent (upward dependency).
- A file imports a module it never references (unused import).
- A file contains `todo!()`, `unimplemented!()`, or stub functions (dummy import/function).
- Two modules import each other, forming a cycle.

## How to fix (concrete steps)

1. Identify the violated rule code (AES201–AES205) and the two files/layers involved from the scan report.
2. For AES201/AES202: remove the forbidden import or add the mandatory import so the dependency points in the allowed direction (upper → lower only, never lower → upper).
3. For AES203: delete the unused import statement.
4. For AES204: replace the stub/dummy with a real implementation, or remove the dead import/function.
5. For AES205: break the cycle by introducing an intermediate abstraction (e.g. a protocol in `contract_`) that both sides depend on instead of each other.
6. Re-run `lint-arwaky-cli scan .` and confirm the import finding is gone.

## Example

Before (AES201 — surface imports agent directly, upward flow):
```python
# surface/my_surface.py
from agents.my_agent import AgentMyFeature  # forbidden
```

After:
```python
# surface/my_surface.py
from capabilities.my_capability import IMyCapability  # allowed downward flow
```

## Verify

```bash
lint-arwaky-cli scan .
# Confirm no AES201–AES205 finding remains for the affected file.
```
