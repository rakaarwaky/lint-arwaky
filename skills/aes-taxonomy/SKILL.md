---
name: aes-taxonomy
description: AES taxonomy VO and domain type scaffolding for Python Rust TS. Use when creating taxonomy files.
metadata:
  tags:
    - python
    - rust
    - typescript
    - aes
    - taxonomy
    - shared
    - vo
    - entity
    - error
    - event
    - constant
    - primitive-to-vo
  related_skills:
    - aes-lint-arwaky
    - aes-capabilities
    - aes-agent
    - aes-contract
    - aes-utility
  triggers:
    - create taxonomy
    - create taxonomy python
    - create taxonomy rust
    - create taxonomy typescript
    - add taxonomy
    - move dataclass to taxonomy
    - move dataclass to taxonomy typescript
    - create vo
    - create vo python
    - create vo rust
    - create vo typescript
    - create error taxonomy
    - create error taxonomy python
    - create error taxonomy rust
    - create error taxonomy typescript
    - create constant taxonomy
    - create constant taxonomy python
    - create constant taxonomy rust
    - create constant taxonomy typescript
    - check taxonomy
    - audit taxonomy
    - audit taxonomy python
    - check taxonomy typescript
---

# aes-taxonomy

> **Purpose**: Scaffold AES taxonomy files (VO / entity / error / event / constant) — the stable language of the domain.
> **Audience**: The agent creating or validating a taxonomy file.
> **Scope**: Python, Rust, and TypeScript `taxonomy_<domain>_<suffix>` files in the shared domain.

The **aggregate** decides which suffix, which imports, and which structure apply.
Rules, templates, section contracts, and Verify blocks live in the language HOW-TUs under [`references/`](references/).

| Language | Focus | Body rule | HOW-TO |
| -------- | ----- | --------- | ------ |
| Python | Taxonomy VO / entity / error / event / constant | Strict suffix set; no I/O; no upward imports | [references/HOW-TO-MAKE-PYTHON-TAXONOMY.md](references/HOW-TO-MAKE-PYTHON-TAXONOMY.md) |
| Rust | Taxonomy VO / entity / error / event / constant | Strict suffix set; no I/O; no upward imports | [references/HOW-TO-MAKE-RUST-TAXONOMY.md](references/HOW-TO-MAKE-RUST-TAXONOMY.md) |
| TypeScript | Taxonomy VO / entity / error / event / constant | Strict suffix set; no I/O; no upward imports | [references/HOW-TO-MAKE-TYPESCRIPT-TAXONOMY.md](references/HOW-TO-MAKE-TYPESCRIPT-TAXONOMY.md) |

**The layer chain:**

`taxonomy_*_vo|entity|error|event|constant` (bottom layer) → contract → capabilities → agent → surface → root

Each file answers one layer's job. A method or import in the wrong layer is the defect this skill exists to prevent.

---

## Invariants

Every rule is machine-checked by `lint-arwaky-cli scan <layer-path>` (see each HOW-TO § Verify).
A rule cannot drift from the gate. Cite the linter, not this file, when pointing at a rule.

| Layer | Rule |
| ----- | ---- |
| Naming | File `taxonomy_<domain>_<suffix>` — suffix strictly `_vo`/`_entity`/`_error`/`_event`/`_constant` (AES101/AES102). |
| Imports | Taxonomy + stdlib only — never capabilities, agents, surface, root, contracts; no I/O (AES201). |
| Primitives | Domain fields wrap VOs — no raw `str`/`int`/`float`/`String`/`string`/`number` for domain values (AES401). |
| Construction | VOs validate on construction; immutable; constants are pure literals. |
| Register | Shared barrel: `__init__.py` / `mod.rs` / `index.ts`. |
| Verify | `lint-arwaky-cli scan <layer-path>` → 0. Language compile is fallback only. |

Split details, templates, and Section Contract tables: **read the language HOW-TO** — do not restate them here.

---

## Diagnostic Tree

Ask these questions in order. The first "No" dictates your next action.

1. **Is this a domain value, identity, failure, fact, or literal?**
   - *Value* → `_vo`; *identity* → `_entity`; *failure* → `_error`; *fact* → `_event`; *literal* → `_constant`.
2. **Does the file import anything above taxonomy or touch I/O?**
   - *Yes* → strip the import / move I/O to capabilities or utility.
3. **Are domain fields raw primitives?**
   - *Yes* → wrap each in a VO (AES401).
4. **Is the file registered in the shared barrel?**
   - *No* → register; else the module is dead code.
5. **Does `lint-arwaky-cli scan <layer-path>` exit 0?**
   - *No* → fix findings, re-scan.

---

## Workflow

1. Resolve the shared taxonomy dir beside contracts.
2. Determine type (VO / Entity / Error / Event / Constant). Run `lint-arwaky-cli scan <layer-path>` — findings are your work list.
3. Draft the file from the language HOW-TO § Template / § Section Contract.
4. Register in `__init__.py` / `mod.rs` / `index.ts`.
5. Verify with `lint-arwaky-cli scan <layer-path>` (HOW-TO § Verify), then wire through `aes-contract` / `aes-capabilities` as needed.

---

## Verification

### Machine Checks

```bash
lint-arwaky-cli scan <layer-path>   # AES101/102, AES201–205, AES401–406 → must be 0
# Fallback only: language compile (python -c import / cargo check / npx tsc --noEmit)

```text

A pass means naming, imports, primitives, and roles are clean. Structural judgement
(tier choice, block order, helper-vs-utility, "orchestration only") is **manual** — see HOW-TO § Rules.

### Human Checks

A machine pass does not mean the file is right. Layer purpose, structural order, and
"only what this layer may do" still need a reader (HOW-TO § Rules).

---

## Pre-flight Checklist

- [ ] `lint-arwaky-cli scan <layer-path>` exits 0.
- [ ] Every touched HOW-TO's `Verify` block was executed.
- [ ] File registered in `__init__.py` / `mod.rs` / `index.ts`.
- [ ] Language fallback compile clean if the HOW-TO lists it.
- [ ] Related skills considered for the next layer up/down.

---

## Common Mistakes (Anti-Patterns)

The linter covers naming, imports, and primitives. These need a reader (HOW-TO § Rules):

- **Primitives in domain fields**: wrap in a VO (AES401).
- **I/O or upward imports in taxonomy**: forbidden — move to capabilities/utility.
- **Computed "constants"**: pure literals only; computed values go to Utility.
- **Restating HOW-TO rules in SKILL.md**: delegate — this file only routes.

---

## Related Skills

- `aes-contract`
- `aes-utility`
- `aes-capabilities`
