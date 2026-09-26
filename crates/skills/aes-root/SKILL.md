---
name: aes-root
description: AES composition root and entry wiring for Python Rust TS. Use when creating container entry files.
metadata:
  tags:
    - python
    - rust
    - typescript
    - aes
    - root
    - container
    - entry
    - composition
    - di
    - wiring
  related_skills:
    - aes-lint-arwaky
    - aes-capabilities
    - aes-agent
    - aes-contract
    - aes-taxonomy
    - aes-surface
  triggers:
    - create root
    - create root python
    - create root rust
    - create root typescript
    - add root
    - add root python
    - add root rust
    - create container
    - create container python
    - create container rust
    - create entry
    - create entry python
    - create entry rust
    - wire dependencies
    - wire dependencies python
    - check root
    - audit root
    - audit root typescript
---

# aes-root

> **Purpose**: Scaffold AES composition roots — containers wire capabilities to contracts; entries bootstrap and start the process.
> **Audience**: The agent creating or validating a container or entry file.
> **Scope**: Python, Rust, and TypeScript `root_<concept>_<container|entry>` files (plus documented barrel/entry exceptions).

The **aggregate** decides which suffix, which imports, and which structure apply.
Rules, templates, section contracts, and Verify blocks live in the language HOW-TUs under [`references/`](references/).

| Language | Focus | Body rule | HOW-TO |
| -------- | ----- | --------- | ------ |
| Python | Container / entry | Wires only; no business/orchestration logic | [references/HOW-TO-MAKE-PYTHON-ROOT.md](references/HOW-TO-MAKE-PYTHON-ROOT.md) |
| Rust | Container / entry | Wires only; no business/orchestration logic | [references/HOW-TO-MAKE-RUST-ROOT.md](references/HOW-TO-MAKE-RUST-ROOT.md) |
| TypeScript | Container / entry | Wires only; no business/orchestration logic | [references/HOW-TO-MAKE-TYPESCRIPT-ROOT.md](references/HOW-TO-MAKE-TYPESCRIPT-ROOT.md) |

**The layer chain:**

root (constructs everything below) → agent / surface / capabilities / contract / utility / taxonomy — nothing below imports root

Each file answers one layer's job. A method or import in the wrong layer is the defect this skill exists to prevent.

---

## Invariants

Every rule is machine-checked by `lint-arwaky-cli scan <layer-path>` (see each HOW-TO § Verify).
A rule cannot drift from the gate. Cite the linter, not this file, when pointing at a rule.

| Layer | Rule |
| ----- | ---- |
| Naming | File `root_<concept>_container` or `root_<concept>_entry` (AES101/AES102). Documented exceptions: `main.rs`, `lib.rs`, `mod.rs`, `__init__.py`, `index.ts`, barrel/entry files, etc. |
| Roles | Container: wire one feature's capabilities to contracts. Entry: bootstrap the app and compose feature containers. |
| Privilege | Root may instantiate and wire components — that is its job; the only layer allowed to depend on everything below. |
| Behaviour | No business logic, no orchestration policy, no technical parsing, no UI behaviour — those live in capabilities / agent / utility / surface. |
| Register | `mod.rs` / `index.ts` / package barrel as required. |
| Verify | `lint-arwaky-cli scan <layer-path>` → 0. Language compile is fallback only. |

Split details, templates, and Section Contract tables: **read the language HOW-TO** — do not restate them here.

---

## Diagnostic Tree

Ask these questions in order. The first "No" dictates your next action.

1. **Container or Entry?**
   - *Wire one feature* → `_container`; *bootstrap all* → `_entry`.
2. **Does the file contain business / orchestration / parsing / UI logic?**
   - *Yes* → move down to capabilities / agent / utility / surface.
3. **Are bindings exposed as contracts (not concretions)?**
   - *Concretions leak* → bind to protocol/aggregate types (`Arc<dyn Trait>` in Rust).
4. **Does anything below root import from root?**
   - *Yes* → invert: root depends downward only.
5. **Does `lint-arwaky-cli scan <layer-path>` exit 0?**
   - *No* → fix findings, re-scan.

---

## Workflow

1. Determine role — Container (wire one feature) or Entry (bootstrap all).
2. Resolve the package top. Run `lint-arwaky-cli scan <layer-path>` — findings are your work list.
3. Create `root_<concept>_<suffix>` from the language HOW-TO § Template / § Section Contract.
4. Wire dependencies: instantiate capabilities, bind them to contract protocol/aggregate types.
5. Compose: the entry builds every container and starts the surface or CLI loop.
6. Register the module, verify with `lint-arwaky-cli scan`.

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

- **Business / orchestration / parsing / UI logic in root**: forbidden — route down a layer.
- **Concretions visible above root**: bind to contracts so agents/surfaces see interfaces, not classes.
- **Wrong suffix / undocumented name**: `_container` or `_entry` only (exceptions listed in HOW-TO).
- **Restating HOW-TO rules in SKILL.md**: delegate — this file only routes.

---

## Related Skills

- `aes-capabilities`
- `aes-agent`
- `aes-surface`
- `aes-contract`
