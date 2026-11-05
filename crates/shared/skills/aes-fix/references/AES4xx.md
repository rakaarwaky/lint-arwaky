# AES4xx — Role/Layer Enforcement

## What the rule checks

Role/layer enforcement lives in the `role-rules` crate. In the codebase these checks emit **AES401–AES406** (the `xx` in this file's name is the family bucket; the actual rule codes are AES401–AES406). Each check verifies that a file's contents match the role of the layer it lives in:

- **AES401** — taxonomy primitive usage + constant purity (taxonomy files hold only VOs/constants, no logic).
- **AES402** — contract role (contract files define ABCs/protocols only, no implementation).
- **AES403** — capabilities role (capability files implement a protocol, no stray agent/surface logic).
- **AES404** — utility role (utility files are pure stateless helpers, no I/O, no domain state).
- **AES405** — agent role (agent files orchestrate only, no I/O, no computation).
- **AES406** — surface role (surface files map input events to VOs, tier-aware).

Checkers: `capabilities_taxonomy_role_auditor`, `capabilities_contract_*_role_auditor`, `capabilities_capabilities_*_role_auditor`, `capabilities_utility_*_role_auditor`, `capabilities_agent_*_role_auditor`, `capabilities_surface_role_auditor`, orchestrated by `agent_role_orchestrator` via `capabilities_role_classifier`.

## Common violation causes

- A taxonomy file contains computation or I/O instead of pure VOs/constants.
- A contract file contains method bodies / implementation instead of only ABC signatures.
- A capability file calls other agents or surfaces directly instead of implementing its protocol.
- A utility file performs I/O, mutates global state, or holds domain data.
- An agent file does I/O, computation, or holds local domain data (should delegate to capabilities/taxonomy/utility).
- A surface file contains business logic instead of only mapping input events to VOs.

## How to fix (concrete steps)

1. Identify the file's layer and the violated rule code (AES401–AES406) from the scan report.
2. Move the offending logic to the correct layer:
   - computation/I/O from agent → capabilities
   - domain data from agent/surface → taxonomy
   - pure helpers from any layer → utility
   - implementation from contract → capability
3. Rewrite the file to carry only the constructs its layer role permits (see each AES4xx sub-check).
4. Remove forbidden imports for the layer (e.g. agent must not import `capabilities_*`, `agent_*`, `surface_*`).
5. Re-run `lint-arwaky-cli scan .` and confirm the role/layer finding is gone.

## Example

Before (AES405 agent role — agent does I/O):
```python
def execute(request):
    with open("data.txt") as f:  # forbidden I/O in agent
        return f.read()
```

After:
```python
def execute(request):
    return self._dep.fetch_data()  # delegate I/O to capabilities
```

## Verify

```bash
lint-arwaky-cli scan .
# Confirm no AES401–AES406 finding remains for the affected file.
```
