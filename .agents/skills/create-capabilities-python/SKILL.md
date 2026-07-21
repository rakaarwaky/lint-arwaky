---
name: create-capabilities-python
description: "Create and validate Python capabilities layer files following AES rules: concrete implementation of behavior (business logic + external adaptation), 3-block structure, one class per file, protocol ABC contracts, DI for service dependencies, and shared VOs for domain data."
metadata:
    tags: [python, aes, capability, protocol, structure, 3-block-structure, di, vo, role-naming]
    triggers:
        - "create capability python"
        - "add capability python"
        - "fix capability structure python"
        - "create protocol python"
        - "capability missing protocol python"
        - "check capabilities python"
        - "audit capabilities python"
    dependencies: []
    related:
        - create-agent-python
        - create-contract-python
        - create-taxonomy-python
---

# create-capabilities-python

## Purpose

Create and validate Python **capabilities layer** files following AES rules.

A capabilities file contains the **concrete implementation** of the system's behavior. This layer encapsulates both:

- **Business logic**: computations, validations, transformations, assessments
- **External adaptation**: database access, third-party API calls, file system access, infrastructure mechanics

Capabilities hide these implementations behind Contracts, keeping behavior modular, swappable, and fully isolated from orchestration.

A capabilities file must:

- implement one domain protocol ABC,
- follow strict 3-block structure,
- use dependency injection for service collaborators,
- use shared VOs for domain data,
- use Utility standalone functions for low-level technical operations.

## Role Naming (ARCHITECTURE §8)

Capabilities use role suffixes describing their concern. Two families:

**Internal (business logic):**

validator, assessor, calculator, resolver, classifier, selector, mapper, transformer, policy, enricher, evaluator, analyzer, scorer, grader, ranker, filter, checker, reviewer, approver, rejector

**External (adaptation):**

repository, gateway, client, provider, fetcher, reader, writer, scanner, executor, publisher, subscriber, adapter, connector, uploader, downloader, sender, receiver, dispatcher, watcher, monitor

File: `capabilities_<domain>_<role>.py`

## Dependencies (ARCHITECTURE §8)

- **May depend on:** Taxonomy, Contract, Utility.
- **Must NOT depend on / import:** other Capabilities, Agent.

Note: do **not** import `infrastructure_*` — that layer no longer exists; its mechanics now live in the Utility layer. Utility is an allowed dependency.

## Special Rules (ARCHITECTURE §8)

- **No Inter-Capability Dependency:** a capability never imports or calls another capability. They are standalone execution units.
- **Pipeline Aggregation:** multiple capabilities are composed into a sequential pipeline by the **Agent layer**, not by themselves.
- **Shared Logic Extraction (DRY):** if several capabilities need the same technical mechanics, extract it into a reusable standalone function in the **Utility layer**. Capabilities must not duplicate technical code.
- **Contract Implementation:** the capability implements the `protocol_` ABC defined in the Contract layer.
- **State Ownership:** the capability owns business and technical state within its execution scope.
- **Utility Delegation:** low-level technical operations call Utility standalone functions, passing state/data as arguments.
- **No Orchestration:** no flow control across capabilities (looping/branching between capabilities) and no error-escalation policy. Execute one responsibility, return a result.
- **No Domain Definition:** do not define domain models (Entities, Value Objects); only consume and produce Taxonomy.
- **Constant Extraction:** extract reusable constants (magic strings, numbers, patterns) into `taxonomy_<domain>_constant.py` in shared. Capabilities must not contain magic constants.

## Definition of Done

1. ONE implementation class per file.
2. Class inherits ONE domain protocol ABC.
3. Block 2 contains ONLY domain protocol method implementations.
4. Dunder methods, factory classmethods, private helpers in Block 3.
5. No locally defined domain models — Entities/Value Objects are consumed from Taxonomy, not defined here.
6. Service dependencies use DI via protocol interfaces.
7. Value/configuration fields use shared VOs.
8. No inter-capability dependencies (capabilities must not import other capabilities or Agent).
9. Low-level technical operations delegate to Utility standalone functions.
10. Reusable constants extracted to `taxonomy_<domain>_constant.py` in shared.
11. `python -c "import <module>"` passes.

## References

Read these files for detailed rules:

| File | Content |
|------|---------|
| `references/layer-boundaries.md` | Allowed/Forbidden imports and dependencies |
| `references/3-block-structure.md` | Block 1/2/3 definitions, method placement rules |
| `references/helper-vs-utility.md` | Helper vs utility decision, I/O Blocker, decision tree |
| `references/primitive-vo-policy.md` | Primitive policy table, VO construction rules |
| `references/error-handling.md` | Error handling rules with examples |
| `references/examples.md` | All BAD/GOOD code examples |
| `references/commands.md` | Quick heuristic check commands |
| `references/checklist.md` | Verification checklist |

## Templates

Use these templates when creating new files:

| File | Purpose |
|------|---------|
| `templates/capabilities_name.py` | New capabilities implementation file |
| `templates/contract_name_protocol.py` | New protocol ABC definition |

## Workflow

### Step 1: Analyze File Responsibility

Read the file and ask: **"Does this implement protocol behavior?"**

If yes → keep as capabilities. If no → check if it's orchestration (→ agent), domain data (→ taxonomy), or pure technical mechanics (→ utility).

### Step 2: Check Missing Protocol (AES403)

Does the capability class inherit a protocol ABC? If no → create one.

### Step 3: Create Protocol File if Missing

Create `contract_<name>_protocol.py` in the appropriate shared domain folder.

### Step 4: Enforce 3-Block Structure

Reorganize: class definition + `__init__` → protocol methods → dunders/factories/helpers.

### Step 5: Verify Class Discipline

One class, no local data classes, DI via protocol interfaces, shared VOs.

### Step 6: Verify Helper vs Utility Boundary

See `references/helper-vs-utility.md` for the decision tree.

### Step 7: Verify Layer Compliance

No forbidden imports (Agent, other capabilities), no inter-capability dependencies, no business logic leakage, no domain model definition.

### Step 8: Verify Error Handling, VO, and Constants

See `references/error-handling.md` and `references/primitive-vo-policy.md`.

### Step 9: Verify Compilation

```bash
python -c "import <module>"
```

## Quick Commands

```bash
# Check forbidden imports (no agent, no other capabilities)
grep -n "^\s*from\s+.*(agent_|capabilities_)" modules/*/src/capabilities_*.py

# List protocol ABC implementations
grep -n "class.*I[A-Za-z0-9_]*Protocol" modules/*/src/capabilities_*.py
```

## Common Mistakes

- Importing other capabilities or Agent directly.
- Defining domain models (Entities, Value Objects) in capabilities files.
- Using concrete service types as constructor fields.
- Putting private helpers in the protocol ABC.
- Putting constructors in the protocol ABC.
- Placing dunder methods before the domain protocol methods.
- Mixing Block 2 and Block 3 responsibilities.
- Flow control across capabilities / error-escalation policy (orchestration).
- Silent error swallowing with `or ""` or `or 0`.
- Magic constants in capabilities logic (extract to `taxonomy_<domain>_constant.py`).
- Not delegating low-level technical operations to Utility.
