---
name: aes-contract
description: AES contract protocol scaffolding for Python Rust TS. Use when creating contract ABC trait files.
metadata:
  tags:
    - python
    - rust
    - typescript
    - aes
    - contract
    - protocol
    - aggregate
    - abc
    - trait
    - interface
    - vo
  related_skills:
    - aes-capabilities
    - aes-agent
    - aes-taxonomy
    - aes-lint-arwaky
  triggers:
    - create contract
    - create contract python
    - create contract rust
    - create contract typescript
    - add contract
    - add contract python
    - add contract rust
    - create protocol
    - create protocol python
    - create protocol rust
    - create protocol typescript
    - create aggregate
    - create aggregate python
    - create aggregate rust
    - create aggregate typescript
    - contract missing
    - validate contract
    - check contract
    - check contract rust
---
# aes-contract

> **Purpose**: Scaffold AES contract files (`_protocol` / `_aggregate`) — promises only, no implementation.
> **Audience**: The agent creating or validating a contract.
> **Scope**: Python, Rust, and TypeScript contracts in the shared domain next to taxonomy.

The **aggregate** decides which suffix, how thin the protocol is, and how rich the export surface is.
Rules, templates, section contracts, and Verify blocks live in the language HOW-TUs under [`references/`](references/).

| Language   | Declaration                                  | Body rule                          | HOW-TO |
| ---------- | -------------------------------------------- | ---------------------------------- | ------ |
| Python     | `class I<Name>…(ABC)` + `@abstractmethod`    | `...` / `pass` — never real code   | [references/HOW-TO-MAKE-PYTHON-CONTRACT.md](references/HOW-TO-MAKE-PYTHON-CONTRACT.md) |
| Rust       | `pub trait I<Name>…: Send + Sync`            | Ends with `;`, no bodies           | [references/HOW-TO-MAKE-RUST-CONTRACT.md](references/HOW-TO-MAKE-RUST-CONTRACT.md) |
| TypeScript | `export interface I<Name>…`                  | Signature only, no class           | [references/HOW-TO-MAKE-TYPESCRIPT-CONTRACT.md](references/HOW-TO-MAKE-TYPESCRIPT-CONTRACT.md) |

**The contract chain:**

`_protocol` (one method / one feature, inward) → agent → `_aggregate` (many methods / one per export, outward) → surface / root

Each suffix answers one direction. A method in the wrong layer is the defect this skill exists to prevent.

---

## Invariants

Every rule is machine-checked by `lint-arwaky-cli scan <contract-dir>` (see each HOW-TO § Verify).
A rule cannot drift from the gate. Cite the linter, not this file, when pointing at a rule.

| Layer | Rule |
| ----- | ---- |
| Naming | `contract_<concept>_<suffix>.<ext>` — suffix strictly `_protocol` or `_aggregate` (AES101/AES102). |
| Protocol shape | Exactly **one method for one feature** — uniform across every capability in the domain. |
| Aggregate shape | **Many methods, one per export** — the rich consumer surface, not a dump-all `execute()`. |
| Body | Pure declaration only: abstract / trait-with-`;` / interface. No default bodies, no helpers. |
| Imports | Taxonomy + other contracts only — never capabilities, agents, surface, root (AES201/AES205). |
| Signatures | Shared VOs only — no raw primitives for domain values (AES402). |
| Register | `__init__.py` / `mod.rs` / `index.ts`. |
| Verify | `lint-arwaky-cli scan <contract-dir>` → 0. Language compile is fallback only. |

Split standard (one method per feature / many exports), import details, templates, and Section Contract tables: **read the language HOW-TO** — do not restate them here.

---

## Diagnostic Tree

Ask these questions in order. The first "No" dictates your next action.

1. **Is the caller the agent (inward) or the surface (outward)?**
   - *Inward / capabilities implement it* → `_protocol`.
   - *Outward / surface calls it* → `_aggregate`.
2. **Does the protocol have exactly one method for one feature?**
   - *No* → strip extra methods; a second feature is a second capability, not a second method.
3. **Does the aggregate expose one typed method per consumer export?**
   - *No* → split into per-export methods (or collapse a protocol mirror / mega-`execute()`).
4. **Do signatures use shared VOs and no impl-layer imports?**
   - *No* → fix per HOW-TO Rules; `lint-arwaky-cli scan` will re-flag AES402/AES201.
5. **Does `lint-arwaky-cli scan <contract-dir>` exit 0?**
   - *No* → fix findings, re-scan, then implement with `aes-capabilities` or `aes-agent`.

---

## Repository Layout

```text
shared-domain/                  # modules/shared/src | crates/shared/src | packages/shared/src
├── taxonomy_<concept>_vo.*     # VOs the contract signatures use
├── contract_<concept>_protocol.*  # one method / one feature
└── contract_<concept>_aggregate.* # many methods / one per export

```text

Same shape for Python (`_protocol.py`), Rust (`_protocol.rs`), and TypeScript (`_protocol.ts`).

---

## Workflow

1. **Resolve the contract dir** in the shared domain beside taxonomy.
2. **Analyze**: Which suffix? Who implements, who consumes? Run `lint-arwaky-cli scan <contract-dir>` — findings are your work list.
3. **Draft protocol**: One method for one feature per [references/HOW-TO-MAKE-PYTHON-CONTRACT.md](references/HOW-TO-MAKE-PYTHON-CONTRACT.md) (or Rust/TS HOW-TO).
4. **Draft aggregate**: One method per export, same HOW-TO § Template / § Section Contract.
5. **Register** in `__init__.py` / `mod.rs` / `index.ts`.
6. **Verify**: `lint-arwaky-cli scan <contract-dir>` (HOW-TO § Verify), then implement with `aes-capabilities` / `aes-agent`.

---

## Verification

### Machine Checks

```bash
lint-arwaky-cli scan <contract-dir>   # AES101/102, AES201–205, AES402, role → must be 0
# Fallback only:
# python -c "import <shared_package>.contract_<concept>_<suffix>"
# cargo check -p <crate-name>
# npx tsc --noEmit

```text

A pass means naming, layer imports, primitives, and roles are clean. The split standard
(one method / feature, many exports) is **manual** — the linter does not count methods.

### Human Checks

A machine pass does not mean the contract is right. Protocol thinness, aggregate export
richness, and "only methods outer layers call" still need a reader (HOW-TO § Rules).

---

## Pre-flight Checklist

- [ ] `lint-arwaky-cli scan <contract-dir>` exits 0.
- [ ] Every touched HOW-TO's `Verify` block was executed.
- [ ] Protocol = one method for one feature; aggregate = one method per export (manual).
- [ ] File registered in `__init__.py` / `mod.rs` / `index.ts`.
- [ ] Language fallback compile clean if the HOW-TO lists it.

---

## Common Mistakes (Anti-Patterns)

The linter covers naming, imports, and primitives. These need a reader (HOW-TO § Rules):

- **Protocol with several methods**: one feature = one method; split into a second capability.
- **Aggregate that mirrors the protocol** or collapses to one `execute()`: export surface must be rich and typed, one method per consumer verb.
- **Contract importing capabilities/agents/surface/root**: dependency arrow inverted (AES201/AES205).
- **Primitives in signatures**: replace with taxonomy VOs (AES402).
- **Restating HOW-TO rules in SKILL.md**: delegate — this file only routes.

---

## Related Skills

- `aes-taxonomy`
- `aes-capabilities`
- `aes-agent`
- `aes-lint-arwaky`
