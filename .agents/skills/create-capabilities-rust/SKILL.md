---
name: create-capabilities-rust
description: "Create and validate Rust capabilities layer files following AES rules: concrete implementation of behavior (business logic + external adaptation), 3-block structure, max 3 types per file, protocol trait contracts, DI for service dependencies, and shared VOs for domain data."
metadata:
  tags: [rust, aes, capabilities, protocol, 3-block-structure, di, vo]
  triggers:
    - "create capabilities rust"
    - "add capabilities rust"
    - "fix capabilities structure rust"
    - "create protocol rust"
    - "capabilities missing protocol rust"
    - "validate capabilities logic rust"
    - "check capabilities rust"
    - "audit capabilities rust"
  dependencies: []
  related:
    - create-agent-rust
    - create-taxonomy-rust
    - create-contract-rust
---
# create-capabilities-rust

Capabilities = **concrete implementation of behavior**: business logic + external adaptation. They implement protocol traits defined in Contract layer. File: `capabilities_<domain>_<role>.rs`.

## Role Naming (configurable)

**Internal (business logic):** validator, assessor, calculator, resolver, classifier, selector, mapper, transformer, policy, enricher, evaluator, analyzer, scorer, grader, ranker, filter, checker, reviewer, approver, rejector

**External (adaptation):** repository, gateway, client, provider, fetcher, reader, writer, scanner, executor, publisher, subscriber, adapter, connector, uploader, downloader, sender, receiver, dispatcher, watcher, monitor

## Definition of Done

1. At least one struct implements a protocol trait (Block 2) — AES403 Rule 2.
2. Block 2 contains ONLY the domain protocol trait implementation.
3. Constructors, std trait impls, private helpers → Block 3.
4. No locally defined domain models — consume from Taxonomy only.
5. Service deps via `Arc<dyn Trait>`.
6. Config/value fields use shared VOs.
7. No inter-capability dependencies.
8. Low-level technical ops delegate to Utility standalone functions.
9. Reusable constants → `taxonomy_<domain>_constant.rs`.
10. Total struct + enum ≤ 3 — AES403 Rule 3.
11. File imports from `_protocol` module only.
12. `cargo check -p <crate-name>` passes.

---

## AES403 Capability Composition Rules

- **Rule 1:** Internal helper structs without trait impl are ALLOWED (never flagged).
- **Rule 2:** At least one struct must implement a protocol trait (`impl Trait for Struct`).
- **Rule 3:** Total struct + enum count ≤ 3.

---

## Layer Boundaries


| Allowed                                    | Forbidden                                                 |
| -------------------------------------------- | ----------------------------------------------------------- |
| Computation, validation, calculation       | Import from`agent_*`, other `capabilities_*`, `surface_*` |
| Data transformation, business rules        | Inter-capability dependencies                             |
| Domain behavior using shared models        | Locally defined domain data structures                    |
| Protocol trait implementation              |                                                           |
| External adaptation (I/O, API, DB)         |                                                           |
| Private helpers supporting the impl struct |                                                           |
| Calling injected protocol traits           |                                                           |
| Calling Utility standalone functions       |                                                           |

**Allowed imports:** Taxonomy, Contract (protocol only), Utility.

**Special rules:**

- No inter-capability dependencies — capabilities are standalone execution units.
- Pipeline aggregation is done by the Agent layer, not by capabilities.
- Extract shared technical mechanics to Utility layer (DRY).
- Import from `_protocol` module ONLY (guard: `CapabilityNoProtocol` if missing).
- No domain model definitions (Entities, VOs defined here are forbidden).
- Extract magic constants to `taxonomy_<domain>_constant.rs`.

---

## The 3-Block Structure

```text
// ─── Block 1: Struct Definition ────────────────────────────
// ─── Block 2: Protocol Trait Implementation ────────────────
// ─── Block 3: Constructors, Std Traits, Helpers ────────────
```

- **Block 1** — struct definition only.
- **Block 2** — ONLY `impl I<Name>Protocol for <Name>...`. No `impl Default`, `fn new()` here.
- **Block 3** — `fn new()`, std trait impls, private helpers.

### Method Placement

```text
Free function (outside impl)?               → EXTRACT to *_utility.rs
In protocol trait?                          → Block 2
std trait impl (Default/Clone/Display)?     → Block 3
fn new() / constructor?                     → Block 3
Private helper (uses &self)?                → Block 3
Pure fn, no struct dep?                     → EXTRACT to *_utility.rs
```

---

## Helper vs Utility

**Keep in Block 3** if ANY: accesses `self`, tightly coupled to this capability, constructor, contains business/domain rules, stateless but single-use.

**Extract to `*_utility.rs`** only if ALL: stateless (no `self`/`Self`), pure, no side effects, domain-agnostic, reusable across modules.

> I/O Rule: stateless + I/O + domain-agnostic = taxonomy utility. Stateless + I/O + domain-specific = capabilities.

---

## Templates


| File                                  | Purpose                                    |
| --------------------------------------- | -------------------------------------------- |
| `templates/capabilities_name.rs`      | Full capabilities implementation (3-block) |
| `templates/contract_name_protocol.rs` | Protocol trait definition                  |
| `templates/mod.rs`                    | Module registration                        |

---

## Workflow

1. **Analyze** — Does this implement protocol behavior? If orchestration → agent. If domain data → taxonomy. If pure mechanics → utility.
2. **Protocol guard** — File MUST `use shared::..._protocol::I<Name>`. If missing → flag `CapabilityNoProtocol`.
3. **Create trait** if missing → `contract_<name>_protocol.rs` in shared domain.
4. **Enforce 3-Block** — struct def → protocol trait impl → constructors/std traits/helpers.
5. **AES403 check** — ≥1 protocol trait implementor, ≤3 total types, `Arc<dyn Trait>` for DI, shared VOs.
6. **Helper boundary** — Apply Helper vs Utility rules above.
7. **Layer compliance** — No agent/capability imports, no inter-capability deps, no domain model definitions.
8. **Constants/VOs** — No magic constants, no raw primitives in trait signatures.
9. **Compile** — `cargo check -p <crate-name>`.

---

## Verification Checklist

- [ ]  3-Block Structure followed.
- [ ]  Block 1: exactly one implementation struct.
- [ ]  Block 2: ONLY protocol trait implementation (`impl I<Name>Protocol for ...`).
- [ ]  Block 3: constructors, std trait impls, private helpers.
- [ ]  At least one struct implements a protocol trait (AES403 Rule 2).
- [ ]  Total struct + enum ≤ 3 (AES403 Rule 3).
- [ ]  File imports from `_protocol` module only.
- [ ]  No local domain models — all imported from `shared/taxonomy`.
- [ ]  Service deps via `Arc<dyn Trait>`.
- [ ]  Shared VOs for config fields and trait signatures.
- [ ]  No inter-capability imports.
- [ ]  No `agent_*` imports.
- [ ]  Constants extracted to `taxonomy_<domain>_constant.rs`.
- [ ]  Low-level ops delegated to Utility.
- [ ]  `cargo check -p <crate-name>` passes.
