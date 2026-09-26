---
name: aes-utility
description: AES stateless utility scaffolding for Python Rust TS. Use when creating utility helper files.
metadata:
  tags:
    - python
    - rust
    - typescript
    - aes
    - utility
    - shared
    - stateless
    - pure-function
    - domain-agnostic
    - reusability
  related_skills:
    - aes-lint-arwaky
    - aes-taxonomy
    - aes-capabilities
    - aes-agent
    - cleanup-consolidate
  triggers:
    - create utility
    - create utility python
    - create utility rust
    - create utility typescript
    - add utility
    - add utility python
    - add utility rust
    - extract utility
    - extract utility python
    - extract to utility rust
    - move to utility
    - move to utility rust
    - create helper function
    - create helper function python
    - check utility
    - check utility python
    - audit utility
    - audit utility python
---

# aes-utility

> **Purpose**: Scaffold AES utility files — stateless, domain-agnostic free functions shared across layers.
> **Audience**: The agent creating or validating a utility file.
> **Scope**: Python, Rust, and TypeScript `utility_<domain>_<role>` files — no class/struct/impl.

The **aggregate** decides which suffix, which imports, and which structure apply.
Rules, templates, section contracts, and Verify blocks live in the language HOW-TUs under [`references/`](references/).

| Language | Focus | Body rule | HOW-TO |
| -------- | ----- | --------- | ------ |
| Python | Stateless free functions | No class/struct/impl; taxonomy-only imports | [references/HOW-TO-MAKE-PYTHON-UTILITY.md](references/HOW-TO-MAKE-PYTHON-UTILITY.md) |
| Rust | Stateless free functions | No class/struct/impl; taxonomy-only imports | [references/HOW-TO-MAKE-RUST-UTILITY.md](references/HOW-TO-MAKE-RUST-UTILITY.md) |
| TypeScript | Stateless free functions | No class/struct/impl; taxonomy-only imports | [references/HOW-TO-MAKE-TYPESCRIPT-UTILITY.md](references/HOW-TO-MAKE-TYPESCRIPT-UTILITY.md) |

**The layer chain:**

`utility_*` (beside taxonomy) → used by capabilities / agent / surface — never imports them

Each file answers one layer's job. A method or import in the wrong layer is the defect this skill exists to prevent.

---

## Invariants

Every rule is machine-checked by `lint-arwaky-cli scan <layer-path>` (see each HOW-TO § Verify).
A rule cannot drift from the gate. Cite the linter, not this file, when pointing at a rule.

| Layer | Rule |
| ----- | ---- |
| Naming | File `utility_<domain>_<role>` — suffix flexible; forbidden: `_vo`/`_entity`/`_error`/`_event`/`_constant`/`_protocol`/`_aggregate` (AES101/AES102). |
| Structure | Free / module-level / exported functions only — no class, no `struct`/`impl`/trait, no `self`/`this` (AES404). |
| State | Stateless and deterministic — no `random`/`now()`/`Math.random()`/`Date.now()`, no global mutable state. |
| Domain | Domain-agnostic — no business rules, no layer-name knowledge; ≥2 consumers (else keep as private helper). |
| Imports | Taxonomy only — never capabilities, agent, surface, contract (Rust: not other utilities) (AES201). |
| Register | Shared barrel: `__init__.py` / `mod.rs` / `index.ts`. |
| Verify | `lint-arwaky-cli scan <layer-path>` → 0. Language compile is fallback only. |

Split details, templates, and Section Contract tables: **read the language HOW-TO** — do not restate them here.

---

## Diagnostic Tree

Ask these questions in order. The first "No" dictates your next action.

1. **Is the function stateless, domain-agnostic, and used by ≥2 modules?**
   - *No* → keep as a private helper where it is used.
2. **Does it use instance state or contain business rules?**
   - *Yes* → belongs in capabilities, not utility.
3. **Does the file define a class/struct/impl?**
   - *Yes* → strip to free functions only.
4. **Does it import above taxonomy?**
   - *Yes* → remove (AES201).
5. **Does `lint-arwaky-cli scan <layer-path>` exit 0?**
   - *No* → fix findings, re-scan.

---

## Workflow

1. Confirm stateless, domain-agnostic, ≥2 consumers (else keep private helper).
2. Resolve the shared utility dir beside taxonomy. Run `lint-arwaky-cli scan <layer-path>`.
3. Draft free functions from the language HOW-TO § Template / § Section Contract.
4. Move needed literals to `taxonomy_*_constant`; keep business rules in capabilities.
5. Register in `__init__.py` / `mod.rs` / `index.ts`, then verify with `lint-arwaky-cli scan`.

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

- **Class / `self` / `this` / `struct` / `impl` in utility**: free functions only.
- **Domain-specific or single-consumer helper extracted here**: keep private until ≥2 consumers.
- **Imports from capabilities/agent/surface/contract**: forbidden (AES201).
- **Restating HOW-TO rules in SKILL.md**: delegate — this file only routes.

---

## Related Skills

- `aes-taxonomy`
- `aes-capabilities`
- `cleanup-consolidate`
