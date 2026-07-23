---
name: create-contract-python
description: "Create and validate Python contract layer files in shared domain: pure ABC definitions for protocols and aggregates. Contracts define public promises only, with no implementation, no layer imports, and domain-safe VO-based signatures."
metadata:
  tags: [python, aes, contract, protocol, aggregate, abc, shared, di, vo]
  triggers:
    - "create contract python"
    - "add contract python"
    - "create protocol python"
    - "create aggregate python"
    - "fix contract python"
    - "check contract python"
    - "audit contract python"
  dependencies: []
  related:
    - create-taxonomy-python
    - create-capabilities-python
    - create-agent-python
---

# create-contract-python

## Purpose

Create and validate Python **contract layer** files in shared domain.

Contracts are pure ABC definitions.

They define the **WHAT**: public promises, stable interfaces, polymorphism boundaries, DI boundaries.

They MUST NOT define the **HOW**: no implementation, no private helpers, no I/O, no business logic, no layer imports.

Two contract suffixes serve different roles:

- `_protocol` → implemented by Capabilities (inbound behavior interface)
- `_aggregate` → implemented by Agent (facade for Surface to access feature behavior)

## Definition of Done

1. Contract file uses correct suffix: `_protocol` or `_aggregate`.
2. Contract contains only ABC definitions.
3. Contract contains no method implementations or default method bodies.
4. Contract contains no private helper signatures.
5. ABC inherits from `ABC`.
6. All methods use `@abstractmethod` decorator.
7. Contract imports only taxonomy and contract types.
8. Contract signatures use shared VOs for domain data.
9. New contract module is registered in `__init__.py`.
10. `python -c "import <module>"` passes.

## References

| File                                | Content                                |
| ----------------------------------- | -------------------------------------- |
| `references/contract-roles.md`      | Two suffix types and naming convention |
| `references/purity-imports.md`      | AES201 import restrictions             |
| `references/abc-structure-rules.md` | 7 ABC structure rules                  |
| `references/primitive-vo-policy.md` | Primitive policy table                 |
| `references/examples.md`            | All BAD/GOOD code examples             |
| `references/commands.md`            | Quick heuristic check commands         |
| `references/checklist.md`           | Verification checklist                 |

## Templates

| File                                   | Purpose                      |
| -------------------------------------- | ---------------------------- |
| `templates/contract_name_protocol.py`  | New protocol ABC definition  |
| `templates/contract_name_aggregate.py` | New aggregate ABC definition |

## Workflow

### Step 1: Determine the Contract Role

Ask: **"Which layer will implement this interface?"**

| Implemented By | Suffix       |
| -------------- | ------------ |
| Capabilities   | `_protocol`  |
| Agent          | `_aggregate` |

### Step 2: Identify Public Methods

Apply the Golden Rule: Is this method called by outer layers? YES → keep in contract. NO → make it a private helper.

### Step 3: Create Contract File

Create `contract_<concept>_<suffix>.py` in the appropriate shared domain folder.

### Step 4: Register Module

Update the domain `__init__.py`.

### Step 5: Verify

```bash
python -c "import <module>"
```

## Quick Commands

```bash
# List contract ABCs
grep -n "^class I[A-Za-z0-9_]*Protocol\|^class I[A-Za-z0-9_]*Aggregate" modules/shared/src/**/contract_*.py

# Check forbidden imports
grep -n "from capabilities_\|from agent_\|from surface_" modules/shared/src/*/contract_*.py
```

## Common Mistakes

- Putting implementation logic in contract files.
- Adding default method bodies to contract ABCs.
- Importing concrete layer types into contracts.
- Using wrong suffix for contract files.
- Leaking implementation details into contract ABCs.
- Forgetting `@abstractmethod` decorators on methods.
- Forgetting to inherit from `ABC`.
- Using raw `str` for domain values in contract signatures.
- Forgetting to register contract modules in `__init__.py`.
