---
name: create-contract-python
description: "Create and validate Python contract layer files in shared domain: pure ABC definitions for protocols and aggregates. Contracts define public promises only, with no implementation, no layer imports, and domain-safe VO-based signatures."
metadata:
  tags: [python, aes, contract, protocol, aggregate, abc, vo]
  triggers:
    - "create contract python"
    - "add contract python"
    - "create protocol python"
    - "create aggregate python"
    - "contract missing python"
    - "validate contract python"
    - "check contract python"
  dependencies: []
  related:
    - create-capabilities-python
    - create-agent-python
    - create-taxonomy-python
---

# create-contract-python

Contract layer = **pure ABC definitions** for the shared domain. No implementation. No layer imports. File: `contract_<concept>_<suffix>.py`.

## Contract Roles

| Suffix | Implemented By | Used By | Example |
| --- | --- | --- | --- |
| `_protocol` | Capabilities | Agent | `contract_import_forbidden_protocol.py` |
| `_aggregate` | Agent | Surface | `contract_import_runner_aggregate.py` |

Interface naming: `I<Name>Protocol`, `I<Name>Aggregate`.

## Definition of Done

1. Contract file uses correct suffix: `_protocol` or `_aggregate`.
2. Contains only ABC class definitions — no method implementations.
3. No private helper method signatures.
4. All methods have proper type annotations.
5. Contracts exported/importable cleanly.
6. Imports only taxonomy and other contract types.
7. Signatures use shared VOs for domain data.
8. Error types from shared taxonomy.
9. Module registered in shared `__init__.py`.
10. `python -c "import <module>"` passes.

---

## Purity and Import Restrictions (AES201)

| Contract File | May Import From | Must Not Import From |
| --- | --- | --- |
| `contract_*_protocol.py` | taxonomy types, other contract types | capabilities, agents, surface, root |
| `contract_*_aggregate.py` | taxonomy types, other contract types | capabilities, agents, surface, root |

---

## Interface Structure Rules

1. Contracts contain ABC class definitions only.
2. No method implementations (`@abstractmethod` body is `...` or `pass`).
3. No private helper signatures.
4. All methods MUST have proper type annotations.
5. ABC classes MUST inherit `ABC` from `abc` module.
6. Error types from shared taxonomy.
7. Naming: `I<Name>Protocol`, `I<Name>Aggregate`.

---

## VO Rules

Contract signatures must use shared VOs, not raw primitives.

| Primitive | Rule |
| --- | --- |
| `str`, `int`, `float` | Forbidden for domain fields/contract values. Use VO. |
| `bool` | Allowed for semantic toggles only. |
| `list[str]`, `dict` | Forbidden for domain collections. Use VO. |

---

## Templates

| File | Purpose |
| --- | --- |
| `templates/contract_name_protocol.py` | Protocol ABC definition |
| `templates/contract_name_aggregate.py` | Aggregate ABC definition |

---

## Workflow

1. **Determine role** — Which layer implements this? Capabilities → `_protocol`. Agent → `_aggregate`.
2. **Identify public methods** — Golden Rule: called by outer layers? YES → keep. NO → private helper (not in contract).
3. **Create file** → `contract_<concept>_<suffix>.py` in shared domain.
4. **Register** → update `__init__.py`.
5. **Verify** → `python -c "import <module>"`.

---

## Verification Checklist

- [ ] Correct suffix: `_protocol` or `_aggregate`.
- [ ] Only ABC class definitions — no implementations.
- [ ] No private helper signatures.
- [ ] All methods type-annotated.
- [ ] ABC inherits from `abc.ABC`.
- [ ] Imports only taxonomy and contract types.
- [ ] No import from capabilities, agents, surface.
- [ ] Signatures use shared VOs.
- [ ] Error types from shared taxonomy.
- [ ] Registered in shared `__init__.py`.
- [ ] `python -c "import <module>"` passes.
