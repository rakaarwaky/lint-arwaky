---
name: create-capabilities-typescript
description: "Create and validate TypeScript capabilities layer files following AES rules: concrete implementation of behavior (business logic + external adaptation), 3-block structure, max 3 types per file, protocol interface contracts, DI for service dependencies, and shared VOs for domain data."
metadata:
  tags: [typescript, aes, capabilities, protocol, 3-block-structure, di, vo]
  triggers:
    - "create capabilities typescript"
    - "add capabilities typescript"
    - "fix capabilities structure typescript"
    - "create protocol typescript"
    - "capabilities missing protocol typescript"
    - "validate capabilities logic typescript"
    - "check capabilities typescript"
    - "audit capabilities typescript"
  dependencies: []
  related:
    - create-agent-typescript
    - create-taxonomy-typescript
    - create-contract-typescript
---
# create-capabilities-typescript

Capabilities = **concrete implementation of behavior**: business logic + external adaptation. They implement protocol interfaces defined in Contract layer. File: `capabilities_<domain>_<role>.ts`.

## Role Naming (configurable)

**Internal (business logic):** validator, assessor, calculator, resolver, classifier, selector, mapper, transformer, policy, enricher, evaluator, analyzer, scorer, grader, ranker, filter, checker, reviewer, approver, rejector

**External (adaptation):** repository, gateway, client, provider, fetcher, reader, writer, scanner, executor, publisher, subscriber, adapter, connector, uploader, downloader, sender, receiver, dispatcher, watcher, monitor

## Definition of Done

1. At least one class implements a protocol interface (Block 2) — AES403 Rule 2.
2. Block 2 contains ONLY the domain protocol method implementations.
3. Utility methods, static factories, private helpers → Block 3.
4. No locally defined domain models — consume from Taxonomy only.
5. Service deps via DI using protocol interfaces.
6. Config/value fields use shared VOs.
7. No inter-capability dependencies.
8. Low-level technical ops delegate to Utility standalone functions.
9. Reusable constants → `taxonomy_<domain>_constant.ts`.
10. Total type count ≤ 3 (class + interface + enum, not counting `type` aliases) — AES403 Rule 3.
11. File imports from `_protocol` module only.
12. `npx tsc --noEmit` passes.

---

## AES403 Capability Composition Rules

- **Rule 1:** Internal helper classes without `implements` are ALLOWED (never flagged).
- **Rule 2:** At least one class must implement a protocol interface (`class Name implements IProto`).
- **Rule 3:** Total type count ≤ 3 (class + interface + enum, not counting `type` aliases).

---

## Layer Boundaries


| Allowed                                   | Forbidden                                                 |
| ------------------------------------------- | ----------------------------------------------------------- |
| Computation, validation, calculation      | Import from`agent_*`, other `capabilities_*`, `surface_*` |
| Data transformation, business rules       | Inter-capability dependencies                             |
| Domain behavior using shared models       | Locally defined domain data structures                    |
| Protocol interface implementation         |                                                           |
| External adaptation (I/O, API, DB)        |                                                           |
| Private helpers supporting the impl class |                                                           |
| Calling injected protocol methods         |                                                           |
| Calling Utility standalone functions      |                                                           |

**Allowed imports:** Taxonomy, Contract (protocol only), Utility.

**Special rules:**

- No inter-capability dependencies — capabilities are standalone execution units.
- Pipeline aggregation is done by the Agent layer, not by capabilities.
- Extract shared technical mechanics to Utility layer (DRY).
- Import from `_protocol` module ONLY (guard: `CapabilityNoProtocol` if missing).
- No domain model definitions (Entities, VOs defined here are forbidden).
- Extract magic constants to `taxonomy_<domain>_constant.ts`.

---

## The 3-Block Structure

```text
// ─── Block 1: Class Definition & Constructor ───────────────
// ─── Block 2: Protocol Method Implementation ───────────────
// ─── Block 3: Utility Methods, Factories, Helpers ──────────
```

- **Block 1** — class + `constructor` only.
- **Block 2** — ONLY domain protocol method implementations. No `toString`, `static`, `private` here.
- **Block 3** — utility methods (`toString`), static factories, private helpers.

### Method Placement

```text
Module-level function?                    → EXTRACT to *_utility.ts
Defined in protocol interface?            → Block 2
toString / toJSON / static factory?      → Block 3
private helper (uses this)?              → Block 3
Pure static, no class dep?               → EXTRACT to *_utility.ts
```

---

## Helper vs Utility

**Keep in Block 3** if ANY: accesses `this`, tightly coupled to this capability, static factory, contains business/domain rules, stateless but single-use.

**Extract to `*_utility.ts`** only if ALL: stateless (no `this`), pure, no side effects, domain-agnostic, reusable across modules.

> I/O Rule: stateless + I/O + domain-agnostic = taxonomy utility. Stateless + I/O + domain-specific = capabilities.

---

## Templates


| File                                  | Purpose                                    |
| --------------------------------------- | -------------------------------------------- |
| `templates/capabilities_name.ts`      | Full capabilities implementation (3-block) |
| `templates/contract_name_protocol.ts` | Protocol interface definition              |

---

## Workflow

1. **Analyze** — Does this implement protocol behavior? If orchestration → agent. If domain data → taxonomy. If pure mechanics → utility.
2. **Protocol guard** — File MUST import from a `_protocol` module. If missing → flag `CapabilityNoProtocol`.
3. **Create interface** if missing → `contract_<name>_protocol.ts` in shared domain.
4. **Enforce 3-Block** — class+`constructor` → protocol methods → utility/factories/helpers.
5. **AES403 check** — ≥1 protocol interface implementor, ≤3 total types, DI via protocols, shared VOs.
6. **Helper boundary** — Apply Helper vs Utility rules above.
7. **Layer compliance** — No agent/capability imports, no inter-capability deps, no domain model definitions.
8. **Constants/VOs** — No magic constants, no raw primitives in protocol signatures.
9. **Compile** — `npx tsc --noEmit`.

---

## Verification Checklist

- [ ]  3-Block Structure followed.
- [ ]  Block 1: exactly one implementation class + `constructor`.
- [ ]  Block 2: ONLY protocol interface method implementations.
- [ ]  Block 3: utility methods, factories, private helpers.
- [ ]  At least one class implements a protocol interface (AES403 Rule 2).
- [ ]  Total type count ≤ 3 (AES403 Rule 3).
- [ ]  File imports from `_protocol` module only.
- [ ]  No local domain models — all imported from `shared/taxonomy`.
- [ ]  Service deps via protocol interfaces (DI).
- [ ]  Shared VOs for config fields and protocol signatures.
- [ ]  No inter-capability imports.
- [ ]  No `agent_*` imports.
- [ ]  Constants extracted to `taxonomy_<domain>_constant.ts`.
- [ ]  Low-level ops delegated to Utility.
- [ ]  `npx tsc --noEmit` passes.
