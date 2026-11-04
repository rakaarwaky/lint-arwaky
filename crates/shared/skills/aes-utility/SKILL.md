---
name: aes-utility
description: AES stateless utility scaffolding for Python Rust TS. Use when creating utility helper files.
when-to-use: "Use when creating, extracting, or auditing a stateless utility helper file (Python, Rust, TypeScript), or when deciding whether code belongs in utility versus capabilities."
argument-hint: "<layer-path>"
---

# aes-utility

Scaffold AES utility files — stateless, domain-agnostic free functions shared across layers. The **layer** decides which suffix, which imports, and which structure apply; rules, templates, and section contracts live in the language HOW-TOs.

## Global rules

- **Naming (AES101/AES102)**: file `utility_<domain>_<role>` — suffix flexible; forbidden: `_vo`/`_entity`/`_error`/`_event`/`_constant`/`_protocol`/`_aggregate`.
- **Structure (AES404)**: free / module-level / exported functions only — no class, no `struct`/`impl`/trait/`type` alias (Rust), no `interface`/`enum`/`type` alias (TS), no `self`/`this`.
- **State**: stateless and deterministic — no `random`/`now()`/`Math.random()`/`Date.now()`, no global mutable state.
- **Domain**: domain-agnostic — no business rules, no layer-name knowledge; ≥2 consumers (else keep as a private helper).
- **Imports (AES201)**: taxonomy only — never capabilities, agent, surface, contract, or other `utility_*` files.
- **Placement (AES701)**: `shared/` folder beside `taxonomy_*` and `contract_*` files — `crates/shared/src/` (Rust), `modules/shared/src/` (Python), `packages/shared/src/` (TypeScript); a single-consumer helper stays a private helper in its feature folder.
- **Utility vs Capability boundary**: "Does this code perform a narrow, stateless, reusable operation with no business logic?" → utility. "Does it implement protocol behaviour with business rules?" → capability; split if it does both — business logic stays in the capability, the stateless helper moves to utility.
- **Register**: shared barrel — `__init__.py` / `mod.rs` / `index.ts`.
- **Verify**: `lint-arwaky-cli scan <layer-path>` → 0. Language compile is fallback only.
- **Related**: aes-taxonomy (upstream), aes-capabilities (consumer layer), aes-root (compose), aes-lint-arwaky (gate) — each carries its own HOW-TO.

## Routing

| Section | Reference | Purpose |
| ------- | --------- | ------- |
| Language HOW-TOs | [references/python-utility.md](references/python-utility.md) | Python utility — rules, template, section contract, verify |
| Language HOW-TOs | [references/rust-utility.md](references/rust-utility.md) | Rust utility — rules, template, section contract, verify |
| Language HOW-TOs | [references/typescript-utility.md](references/typescript-utility.md) | TypeScript utility — rules, template, section contract, verify |
| Invariants | [references/invariants.md](references/invariants.md) | Layer rule table: naming, structure, state, domain, imports, register |
| Placement | [references/placement.md](references/placement.md) | AES701 shared-folder layout and move-out rule |
| Diagnostic tree | [references/diagnostic-tree.md](references/diagnostic-tree.md) | First-failing-check routing for a suspect utility file |
| Workflow | [references/workflow.md](references/workflow.md) | Step sequence from confirm-utility to register |
| Verification | [references/verification.md](references/verification.md) | Machine gate + fallback gate + human checks |
| Pre-flight | [references/pre-flight.md](references/pre-flight.md) | Checklist before committing a utility file |
| Anti-patterns | [references/anti-patterns.md](references/anti-patterns.md) | Common structure, import, and placement mistakes |
