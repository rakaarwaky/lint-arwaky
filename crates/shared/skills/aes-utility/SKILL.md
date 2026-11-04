---
name: aes-utility
description: "AES stateless utility scaffolding for Python Rust TS. Use when creating utility helper files."
when-to-use: "create, add, or audit an AES utility file (Python, Rust, TypeScript); extract a stateless helper; move a helper to shared; decide utility vs capability."
argument-hint: '<python | rust | typescript> [file path]'
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
---

# aes-utility

Scaffold AES utility files — `utility_<domain>_<role>` free functions that are stateless, domain-agnostic, and shared across layers. The audience is the agent creating or validating a utility file; the **language** decides which declarations are legal. A utility answers "how is a small task done"; a capability answers "what can the system do" — code that does both splits: business logic stays in the capability, the stateless helper moves here. Invariants read by more than one step live in Global Rules below.

## Global Rules

- Naming: file `utility_<domain>_<role>` — role flexible; forbidden suffixes `_vo`/`_entity`/`_error`/`_event`/`_constant`/`_protocol`/`_aggregate` (AES101/AES102).
- Structure: free / module-level / exported functions only — no class, no `struct`/`impl`/trait/`type` alias (Rust), no `interface`/`enum`/`type` alias (TS), no `self`/`this` (AES404).
- State: stateless and deterministic — no `random`/`now()`/`Date.now()`, no global mutable state.
- Domain: domain-agnostic with ≥2 consumers; a single-feature helper stays a private helper inside that feature folder.
- Imports: taxonomy only — never capabilities, agents, surface, contracts, or other `utility_*` files (AES201).
- Register: shared barrel (`__init__.py` / `mod.rs` / `index.ts`) so every member can call the helper.
- Placement: `shared/` folder only — `crates/shared/src/`, `modules/shared/src/`, `packages/shared/src/` (AES701).
- Verify: `lint-arwaky-cli scan <layer-path>` → 0. Language compile is fallback only.
- One fact, one home: the language HOW-TO carries the language's declaration limits and section contract — read it, do not restate here. Cite the linter, not this file, when pointing at a rule. Each bullet states a fact once; references point at the bullet.

## Routing

| File | Purpose |
| --- | --- |
| [references/python-utility.md](references/python-utility.md) | Python utility HOW-TO — rules, template, section contract, workflow steps, review, verify |
| [references/rust-utility.md](references/rust-utility.md) | Rust utility HOW-TO — rules, template, section contract, workflow steps, review, verify |
| [references/typescript-utility.md](references/typescript-utility.md) | TypeScript utility HOW-TO — rules, template, section contract, workflow steps, review, verify |
| Related skills | aes-taxonomy, aes-capabilities, aes-agent, aes-lint-arwaky |
