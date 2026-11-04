---
name: aes-taxonomy
description: AES taxonomy VO and domain type scaffolding for Python Rust TS. Use when creating taxonomy files.
when-to-use: "Use when creating or auditing a taxonomy VO, entity, error, event, constant, request, or response file in Python, Rust, or TypeScript, or when wrapping a raw primitive in a value object."
argument-hint: "<layer-path>"
---

# aes-taxonomy

Scaffold AES taxonomy files (VO / entity / error / event / constant / request / response) — the stable language of the domain. The **layer** decides which suffix, which imports, and which structure apply.

## Global rules

- **Placement (AES701)**: `taxonomy_*` files belong in the workspace's `shared/` folder beside `utility_*` and `contract_*` files — `crates/shared/src/` (Rust), `modules/shared/src/` (Python), `packages/shared/src/` (TypeScript). Placing a taxonomy file in a feature folder leaves the domain types unreachable from every member; a `capabilities_*`, `agent_*`, or `surface_*` file found inside `shared/` is an AES701 violation and must move out.
- **Naming (AES101/AES102)**: file `taxonomy_<domain>_<suffix>` — suffix strictly `_vo`/`_entity`/`_error`/`_event`/`_constant`/`_request`/`_response`.
- **Imports (AES201)**: taxonomy + stdlib only — never capabilities, agents, surface, root, or contracts; no I/O.
- **Primitives (AES401)**: domain fields wrap VOs — no raw `str`/`int`/`float`/`String`/`string`/`number` for domain values.
- **Error contract**: every `_error` file exposes `error_id` + `error_code` + `message`. Ids come from per-feature blocks — common `000X`, feature 1 `1XXX`, feature 2 `2XXX`, …; a feature is a folder with an `agent_*_orchestrator` file. Ids never change across releases.
- **Construction**: VOs validate on construction and are immutable; constants are pure literals.
- **Register**: shared barrel (`__init__.py` / `mod.rs` / `index.ts`).
- **Verify**: `lint-arwaky-cli scan <layer-path>` → 0; language compile is fallback only.
- **Related**: aes-contract (signatures use these VOs), aes-utility (stateless helpers), aes-capabilities (consume VOs), aes-agent (implement aggregates) — each carries its own HOW-TO.

## Routing

| Section | Reference | Purpose |
| ------- | --------- | ------- |
| Language HOW-TOs | [references/python-taxonomy.md](references/python-taxonomy.md) | Python taxonomy — rules, template, section contract, workflow, verify |
| Language HOW-TOs | [references/rust-taxonomy.md](references/rust-taxonomy.md) | Rust taxonomy — rules, template, section contract, workflow, verify |
| Language HOW-TOs | [references/typescript-taxonomy.md](references/typescript-taxonomy.md) | TypeScript taxonomy — rules, template, section contract, workflow, verify |
| Diagnostic tree | [references/diagnostic-tree.md](references/diagnostic-tree.md) | Suffix selection and first-failing-check routing |
| Workflow | [references/workflow.md](references/workflow.md) | Step sequence from resolve to register |
| Verification | [references/verification.md](references/verification.md) | Primary gate + fallback language compile |
| Pre-flight | [references/pre-flight.md](references/pre-flight.md) | Checklist before drafting a taxonomy file |
| Anti-patterns | [references/anti-patterns.md](references/anti-patterns.md) | Common primitive, import, and I/O mistakes |
