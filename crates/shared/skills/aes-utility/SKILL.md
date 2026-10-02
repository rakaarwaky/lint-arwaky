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
    - aes-taxonomy
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
> **Scope**: Python, Rust, and TypeScript `utility_<domain>_<role>` files — no class/struct/impl/interface/enum/type alias.

## Utility vs Capability — Defining the Boundary

A utility and a capability are fundamentally different layers. Confusing them causes misplaced code, wrong imports, and linter violations.

| | **Utility** (`utility_*`) | **Capability** (`capabilities_*`) |
|---|---|---|
| **Core question** | How is a small task **done**? | What can the system **do**? |
| **Nature** | Stateless, domain-agnostic, pure free functions | Concrete protocol behaviour — domain rules, external adaptation, DI-wired |
| **Responsibility** | Performs narrow, reusable operations with no business logic | Implements a `_protocol` contract; orchestrates inputs into domain outcomes |
| **State** | Absolutely no state — pure functions only | Has instance state via injected dependencies (`Arc<dyn Trait>`, DI params) |
| **Location** | Shared folder (`shared/`) beside taxonomy and contract files | Feature folder (`crates/<feature>/`, `modules/<feature>/`, `packages/<feature>/`) |
| **Consumers** | Consumed by capabilities, agents, surfaces, or other utilities | Consumed by agent orchestrators or root composition |
| **Examples** | `utility_string_sanitizer`, `utility_date_formatter`, `utility_hasher` | `capabilities_user_auth`, `capabilities_data_persist`, `capabilities_rate_limit` |

**Decision rule**: Ask "Does this code perform a narrow, stateless, reusable operation with no business logic?" If yes → utility. Ask "Does this code implement a protocol behaviour with business rules?" If yes → capability. If it does both, split it: business logic stays in the capability; the stateless helper moves to utility.

The **layer** decides which suffix, which imports, and which structure apply.
Rules, templates, section contracts, and Verify blocks live in the language HOW-TUs under [`references/`](references/).

| Language | Focus | Body rule | HOW-TO |
| -------- | ----- | --------- | ------ |
| Python | Stateless free functions | No class; no `self` in function params (redundant with class check); taxonomy-only imports | [references/HOW-TO-MAKE-PYTHON-UTILITY.md](references/HOW-TO-MAKE-PYTHON-UTILITY.md) |
| Rust | Stateless free functions | No struct/impl/trait/type-alias; taxonomy-only imports (not other utilities) | [references/HOW-TO-MAKE-RUST-UTILITY.md](references/HOW-TO-MAKE-RUST-UTILITY.md) |
| TypeScript | Stateless free functions | No class/interface/enum/type-alias; taxonomy-only imports (not other utilities) | [references/HOW-TO-MAKE-TYPESCRIPT-UTILITY.md](references/HOW-TO-MAKE-TYPESCRIPT-UTILITY.md) |

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
| Structure | Free / module-level / exported functions only — no class, no `struct`/`impl`/trait/`type` alias (Rust), no `interface`/`enum`/`type` alias (TS), no `self`/`this` (AES404). |
| State | Stateless and deterministic — no `random`/`now()`/`Math.random()`/`Date.now()`, no global mutable state. |
| Domain | Domain-agnostic — no business rules, no layer-name knowledge; ≥2 consumers (else keep as private helper). |
| Imports | Taxonomy only — never capabilities, agent, surface, contract, or other `utility_*` files (AES201). |
| Register | Shared barrel: `__init__.py` / `mod.rs` / `index.ts`. |
| Verify | `lint-arwaky-cli scan <layer-path>` → 0. Language compile is fallback only. |

Split details, templates, and Section Contract tables: **read the language HOW-TO** — do not restate them here.

---

## Placement (AES701)

A `utility_*` file belongs in the workspace's **`shared/` folder**, beside `taxonomy_*` and `contract_*` files:

```text
crates/shared/src/     # Rust
modules/shared/src/     # Python
packages/shared/src/    # TypeScript
```

The `shared` folder is locked to taxonomy, utility, and contract files. A `capabilities_*`,
`agent_*`, or `surface_*` file found there is an **AES701** violation and must be moved out.
A utility that only one feature uses does not belong in `shared/` — keep it a private
helper inside that feature folder.

---

## Diagnostic Tree

Ask these questions in order. The first "No" dictates your next action.

1. **Is the function stateless, domain-agnostic, and used by ≥2 modules?**
   - *No* → keep as a private helper where it is used, or move to capabilities if it has business rules.
2. **Does it implement protocol behaviour with business rules or injected dependencies?**
   - *Yes* → belongs in capabilities, not utility. See `aes-capabilities` skill.
3. **Does the file define a class/struct/impl/trait/type alias?**
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

- **Class / `self` / `this` / `struct` / `impl` / `trait` / `type` alias in utility**: free functions only.
- **Domain-specific or single-consumer helper extracted here**: keep private until ≥2 consumers.
- **Imports from capabilities/agent/surface/contract**: forbidden (AES201).
- **Restating HOW-TO rules in SKILL.md**: delegate — this file only routes.

---

## Related Skills

- `aes-taxonomy`
- `aes-capabilities`
- `aes-root`
