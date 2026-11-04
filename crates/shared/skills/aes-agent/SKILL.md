---
name: aes-agent
description: AES agent orchestrator scaffolding for Python Rust TS. Use when creating agent files.
when-to-use: "create, add, or audit an AES agent or aggregate orchestrator file (Python, Rust, TypeScript); fix agent structure; validate agent logic."
argument-hint: '<python | rust | typescript> [file path]'
---

# aes-agent

Scaffold AES agent orchestrators — orchestration only: call the aggregate, return shared VOs, zero I/O and zero computation. The audience is the agent creating or validating an agent orchestrator file; the **layer** decides which suffix, which imports, and which structure apply. The layer chain reads `contract_*_aggregate` (created via aes-contract) → agent implements → surface / root call the agent — one agent implements exactly one aggregate. Invariants read by more than one step live in Global Rules below.

## Global Rules

- Naming: file `agent_<domain>_orchestrator` — `_orchestrator` is the only suffix (AES101/AES102).
- Structure: 3-block order — Block 1 type+injected deps → Block 2 aggregate methods only → Block 3 factories/dunders/helpers; ≥1 aggregate, ≤3 types (AES405).
- Imports: shared contracts/taxonomy/utility only — never capabilities, siblings, surface (AES201).
- Behaviour: no I/O, no arithmetic/parsing/normalisation, no local domain data, no magic constants, no silently discarded errors.
- DI: injected aggregate only (`Arc<dyn Trait>` in Rust); shared VOs in signatures.
- Verify: `lint-arwaky-cli scan <layer-path>` → 0. Language compile is fallback only.
- Placement: agent files live in a feature folder — `crates/<feature>/`, `modules/<feature>/`, `packages/<feature>/`; a feature folder needs ≥1 `agent_*_orchestrator` and ≥1 `capabilities_*` file (AES702).
- Register: shared barrel (`__init__.py` / `mod.rs` / `index.ts`) so the root can compose the agent.
- One fact, one home: the language HOW-TO carries the language's suffix, imports, and structure — read it, do not restate here. Cite the linter, not this file, when pointing at a rule. Each bullet states a fact once; references point at the bullet.

## Routing

| File | Purpose |
| --- | --- |
| [references/python-agent.md](references/python-agent.md) | Python agent HOW-TO — rules, template, section contract, workflow steps, review, verify |
| [references/rust-agent.md](references/rust-agent.md) | Rust agent HOW-TO — rules, template, section contract, workflow steps, review, verify |
| [references/typescript-agent.md](references/typescript-agent.md) | TypeScript agent HOW-TO — rules, template, section contract, workflow steps, review, verify |
| Related skills | aes-contract, aes-capabilities, aes-surface, aes-root, aes-taxonomy |
