# Step 4: Import Graph: Forbidden Layer Import

## What the rule checks

AES201 (rule code `AES201`, checker `capabilities_import_forbidden_checker` in the `import-rules` crate) enforces unidirectional dependency flow across the 7-layer AES architecture: upper layers may not import layers below their allowed boundary.

- Each layer has an allowed import boundary; imports crossing that boundary (lower → upper) trigger a finding.
- Barrel files (`mod.rs`, `__init__.py`, `index.ts`) and entry points are exempt.
- `utility` layer may import from `taxonomy` only. Forbidden: `agent`, `surface`, `contract`, `capabilities`, `root`, `utility` (self-imports).

## Common violation causes

- A surface imports an agent directly instead of going through capabilities/contract.
- A capability imports a surface or agent (upward dependency).
- A lower-layer file imports a file from a layer above it.
- **`utility`→`utility` self-import:** two utility files importing each other (e.g. after an AES301 split created `utility_agent_role_helpers.rs` and `utility_capabilities_role_helpers.rs` that import each other).

## How to fix (concrete steps)

1. Identify the violated rule code (AES201) and the two files/layers involved from the scan report.
2. For `utility`→`utility` self-imports: consolidate the shared helper functions into a single `taxonomy` layer module (e.g. `taxonomy_role_rules_entity.rs`) — taxonomy is importable by `utility` per the layer matrix — delete the two utility helper modules, and update all importers to `crate::taxonomy_role_rules_entity`.
3. For other violations: remove the forbidden import so the dependency points in the allowed direction (upper → lower only).
4. Route through the correct layer boundary — depend on a contract protocol/aggregate injected via DI rather than reaching upward.
5. Update any code that referenced the removed import path.
6. Re-run `lint-arwaky-cli scan .` and confirm AES201 no longer fires on that file.

## Verify

```bash
lint-arwaky-cli scan . --filter AES201
# Confirm no AES201 finding remains.
```
