---
name: aes-migration
description: "AES architecture migration workflow for Python Rust TS. Use when migrating legacy codebases to AES layered architecture."
when-to-use: "migrate, refactor, or convert a legacy Python, Rust, or TypeScript codebase into the AES 7-layer architecture; audit AES compliance; plan the phased taxonomy-first migration."
argument-hint: '<python | rust | typescript> <project-root>'
metadata:
  tags:
    - python
    - rust
    - typescript
    - aes
    - migration
    - refactor
    - architecture
    - dependency-injection
    - phase-gate
  related_skills:
    - aes-lint-arwaky
    - aes-taxonomy
    - aes-contract
    - aes-utility
    - aes-capabilities
    - aes-agent
    - aes-surface
    - aes-root
---

# aes-migration

Guide the phased migration of a legacy codebase into the AES layered architecture: Phase 0 audit the baseline, then Phase 1–7 build each layer bottom-up — taxonomy → contract → utility → capabilities → agent → surface → root — and Phase 8 verify to zero. The audience is the agent planning or executing a migration; the **language** decides which playbook loads, and the **violation count** after the baseline audit decides the strategy (< 10: full, 10–50: phased, > 50: taxonomy-first). Each layer's shape is owned by its layer skill; the playbook carries import matrices, phase templates, and troubleshooting. Invariants read by more than one step live in Global Rules below.

## Global Rules

- Baseline first: `lint-arwaky-cli scan .` before touching anything; the recorded violation count picks the strategy.
- Layer order: migrate bottom-up (taxonomy → contract → utility → capabilities → agent → surface → root); a layer starts only when the one below scans clean.
- Per-phase gate: `lint-arwaky-cli scan <layer-dir>` → 0 before moving to the next layer; language compile (`python -m compileall` / `cargo check` / `npx tsc --noEmit`) is fallback only.
- Done: Phase 8 — full scan of the project root exits 0, language compile clean, tests green; commit `migrate: <layer> phase to AES (<rules>)` only when asked.
- Shape ownership: protocol/aggregate shape, structure, DI, wiring — all owned by `aes-layer` — the migration skill routes; it does not restate layer shape.
- One fact, one home: the language playbook carries the phase templates and troubleshooting table — read it, do not restate here. Cite the linter, not this file, when pointing at a rule. Each bullet states a fact once; references point at the bullet.

## Routing

| File | Purpose |
| --- | --- |
| [references/python-migration.md](references/python-migration.md) | Python migration playbook — phases 0–8, import matrix, templates, verify |
| [references/rust-migration.md](references/rust-migration.md) | Rust migration playbook — phases 0–8, import matrix, templates, verify |
| [references/typescript-migration.md](references/typescript-migration.md) | TypeScript migration playbook — phases 0–8, import matrix, templates, verify |
| Related skills | aes-layer, aes-lint-arwaky, aes-docs, aes-migration, aes-testing-suite |
