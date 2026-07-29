---
name: create-capabilities-python
description: "Create and validate Python capabilities layer files following AES rules: concrete implementation of behavior (business logic + external adaptation), 3-block structure, max 3 types per file, protocol ABC contracts, DI for service dependencies, and shared VOs for domain data."
metadata:
  tags: [python, aes, capabilities, protocol, 3-block-structure, di, vo]
  triggers:
    - "create capabilities python"
    - "add capabilities python"
    - "fix capabilities structure python"
    - "create protocol python"
    - "capabilities missing protocol python"
    - "validate capabilities logic python"
    - "check capabilities python"
    - "audit capabilities python"
  dependencies: []
  related:
    - create-agent-python
    - create-taxonomy-python
    - create-contract-python
---
# create-capabilities-python

Capabilities = **concrete implementation of behavior**: business logic + external adaptation. They implement protocol ABCs defined in Contract layer. File: `capabilities_<domain>_<role>.py`.

## Role Naming  (configurable)

**Internal (business logic):** validator, assessor, calculator, resolver, classifier, selector, mapper, transformer, policy, enricher, evaluator, analyzer, scorer, grader, ranker, filter, checker, reviewer, approver, rejector

**External (adaptation):** repository, gateway, client, provider, fetcher, reader, writer, scanner, executor, publisher, subscriber, adapter, connector, uploader, downloader, sender, receiver, dispatcher, watcher, monitor

## Definition of Done

1. At least one class inherits a protocol ABC (Block 2) — AES403 Rule 2.
2. Block 2 contains ONLY domain protocol method implementations.
3. Dunders, factory classmethods, private helpers → Block 3.
4. No locally defined domain models — consume from Taxonomy only.
5. Service deps via DI using protocol interfaces.
6. Config/value fields use shared VOs.
7. No inter-capability dependencies.
8. Low-level technical ops delegate to Utility standalone functions.
9. Reusable constants → `taxonomy_<domain>_constant.py`.
10. Total class count ≤ 3 — AES403 Rule 3.
11. File imports from `_protocol` module only.
12. `python -c "import <module>"` passes.

---

## AES403 Capability Composition Rules

- **Rule 1:** Internal helper classes without ABC inheritance are ALLOWED (never flagged).
- **Rule 2:** At least one class must inherit a protocol ABC (`class Name(Protocol):`).
- **Rule 3:** Total class count ≤ 3.

---

## Layer Boundaries


| Allowed                                   | Forbidden                                                 |
| ------------------------------------------- | ----------------------------------------------------------- |
| Computation, validation, calculation      | Import from`agent_*`, other `capabilities_*`, `surface_*` |
| Data transformation, business rules       | Inter-capability dependencies                             |
| Domain behavior using shared models       | Locally defined domain data structures                    |
| Protocol ABC implementation               |                                                           |
| External adaptation (I/O, API, DB)        |                                                           |
| Private helpers supporting the impl class |                                                           |
| Calling injected protocol traits          |                                                           |
| Calling Utility standalone functions      |                                                           |

**Allowed imports:** Taxonomy, Contract (protocol only), Utility.

**Special rules:**

- No inter-capability dependencies — capabilities are standalone execution units.
- Pipeline aggregation is done by the Agent layer, not by capabilities.
- Extract shared technical mechanics to Utility layer (DRY).
- Import from `_protocol` module ONLY (guard: `CapabilityNoProtocol` if missing).
- No domain model definitions (Entities, VOs defined here are forbidden).
- Extract magic constants to `taxonomy_<domain>_constant.py`.

---

## The 3-Block Structure

```text
# ─── Block 1: Class Definition & Constructor ───────────────
# ─── Block 2: Protocol ABC Method Implementation ───────────
# ─── Block 3: Dunder Methods, Factories, Helpers ───────────
```

- **Block 1** — class + `__init__` only.
- **Block 2** — ONLY domain protocol method implementations. No `__repr__`, `@classmethod`, `@staticmethod` here.
- **Block 3** — dunders, factory classmethods, private helpers.

### Method Placement

```text
Module-level def?                    → EXTRACT to *_utility.py
@abstractmethod in protocol ABC?     → Block 2
Dunder / factory classmethod?        → Block 3
@staticmethod, pure + no class dep?  → EXTRACT to *_utility.py
Private helper (uses self)?          → Block 3
```

---

## Helper vs Utility

**Keep in Block 3** if ANY: accesses `self`, tightly coupled to this capability, factory method, contains business/domain rules, stateless but single-use.

**Extract to `*_utility.py`** only if ALL: stateless (no `self`/`cls`), pure, no side effects, domain-agnostic, reusable across modules.

> I/O Rule: stateless + I/O + domain-agnostic = taxonomy utility. Stateless + I/O + domain-specific = capabilities.

---

## Templates


| File                                  | Purpose                                    |
| --------------------------------------- | -------------------------------------------- |
| `templates/capabilities_name.py`      | Full capabilities implementation (3-block) |
| `templates/contract_name_protocol.py` | Protocol ABC definition                    |

---

## Workflow

1. **Analyze** — Does this implement protocol behavior? If orchestration → agent. If domain data → taxonomy. If pure mechanics → utility.
2. **Protocol guard** — File MUST import from a `_protocol` module. If missing → flag `CapabilityNoProtocol`.
3. **Create protocol** if missing → `contract_<name>_protocol.py` in shared domain.
4. **Enforce 3-Block** — class+`__init__` → protocol methods → dunders/factories/helpers.
5. **AES403 check** — ≥1 protocol ABC inheritor, ≤3 total classes, DI via protocols, shared VOs.
6. **Helper boundary** — Apply Helper vs Utility rules above.
7. **Layer compliance** — No agent/capability imports, no inter-capability deps, no domain model definitions.
8. **Constants/VOs** — No magic constants, no raw primitives in protocol signatures.
9. **Compile** — `python -c "import <module>"`.

---

## Verification Checklist

- [ ]  3-Block Structure followed.
- [ ]  Block 1: exactly one implementation class + `__init__`.
- [ ]  Block 2: ONLY protocol ABC method implementations.
- [ ]  Block 3: dunders, factories, private helpers.
- [ ]  At least one class inherits a protocol ABC (AES403 Rule 2).
- [ ]  Total class count ≤ 3 (AES403 Rule 3).
- [ ]  File imports from `_protocol` module only.
- [ ]  No local domain models — all imported from `shared/taxonomy`.
- [ ]  Service deps via protocol interfaces (DI).
- [ ]  Shared VOs for config fields and protocol signatures.
- [ ]  No inter-capability imports.
- [ ]  No `agent_*` imports.
- [ ]  Constants extracted to `taxonomy_<domain>_constant.py`.
- [ ]  Low-level ops delegated to Utility.
- [ ]  Zero I/O in protocol method implementations (delegated to injected utility).
- [ ]  `python -c "import <module>"` passes.
