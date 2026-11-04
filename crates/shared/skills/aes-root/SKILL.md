---
name: aes-root
description: "AES composition root and entry wiring for Python Rust TS. Use when creating container entry files."
when-to-use: "create, add, or audit an AES composition root (Python, Rust, TypeScript); wire a feature container; bootstrap an entry point; fix root structure."
argument-hint: '<python | rust | typescript> [file path]'
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
---

# aes-root

Scaffold AES composition roots — `root_<concept>_container` wires one feature's capabilities to its contracts, and `root_<concept>_entry` bootstraps the app and composes every container. The audience is the agent creating or validating a container or entry file; the **layer** decides which suffix, which imports, and which structure apply. Root is the only layer allowed to depend on everything below — nothing below root imports root. Invariants read by more than one step live in Global Rules below.

## Global Rules

- Naming: file `root_<concept>_container` or `root_<concept>_entry` (AES101/AES102). Documented exceptions: `main.rs`, `lib.rs`, `mod.rs`, `__init__.py`, `index.ts`, and other barrel/entry files listed in the HOW-TOs.
- Roles: a container wires one feature's capabilities to its contracts; an entry bootstraps the process and composes every feature container.
- Behaviour: wiring and construction only — no business logic, no orchestration policy, no technical parsing, no UI behaviour; those live in capabilities / agent / utility / surface.
- DI: bind to contract protocol/aggregate types (`Arc<dyn Trait>` in Rust) — concretions never leak above root.
- Register: `mod.rs` / `index.ts` / package barrel as required.
- Verify: `lint-arwaky-cli scan <layer-path>` → 0. Language compile is fallback only.
- One fact, one home: the language HOW-TO carries the language's wiring syntax and section contract — read it, do not restate here. Cite the linter, not this file, when pointing at a rule. Each bullet states a fact once; references point at the bullet.

## Routing

| File | Purpose |
| --- | --- |
| [references/python-root.md](references/python-root.md) | Python root HOW-TO — rules, template, section contract, workflow steps, review, verify |
| [references/rust-root.md](references/rust-root.md) | Rust root HOW-TO — rules, template, section contract, workflow steps, review, verify |
| [references/typescript-root.md](references/typescript-root.md) | TypeScript root HOW-TO — rules, template, section contract, workflow steps, review, verify |
| Related skills | aes-contract, aes-capabilities, aes-agent, aes-surface, aes-lint-arwaky |
