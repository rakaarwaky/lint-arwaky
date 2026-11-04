---
name: aes-contract
description: AES contract protocol scaffolding for Python Rust TS. Use when creating contract ABC trait files.
when-to-use: "Use when creating or auditing a contract protocol or aggregate file (Python, Rust, TypeScript), splitting a protocol from an aggregate, or validating contract shape."
argument-hint: "<layer-path>"
---

# aes-contract

Scaffold AES contract files (`_protocol` / `_aggregate`) — promises only, no implementation. The **layer** decides which suffix, how rich the protocol is, and how the consumer surface is shaped.

## Global rules

- **Placement (AES701)**: `contract_*` files belong in the workspace's `shared/` folder beside `taxonomy_*` and `utility_*` — `crates/shared/src/` (Rust), `modules/shared/src/` (Python), `packages/shared/src/` (TypeScript). A `capabilities_*`, `agent_*`, or `surface_*` file found there is an AES701 violation and must move out.
- **Naming (AES101/AES102)**: `contract_<concept>_<suffix>.<ext>` — suffix strictly `_protocol` or `_aggregate`.
- **Protocol shape**: one file per feature, one trait/class/interface per capability seam, each rich with named methods.
- **Aggregate shape**: exactly one `execute()` method; the response is a taxonomy-defined enum of VOs.
- **Body**: pure declaration only — abstract (`...` / `pass`), trait ending in `;`, interface signature. No default bodies, no helpers, no concrete classes; shared logic belongs in `utility_*` (AES402).
- **Imports (AES201/AES205)**: taxonomy + other contracts only — never capabilities, agents, surface, or root.
- **Signatures (AES402)**: shared VOs only — no raw primitives for domain values.
- **Register**: `__init__.py` / `mod.rs` / `index.ts`.
- **Verify**: `lint-arwaky-cli scan <contract-dir>` → 0; language compile is fallback only.
- **Related**: aes-taxonomy (shared VOs), aes-capabilities (implements protocol), aes-agent (implements aggregate), aes-surface (calls aggregate), aes-lint-arwaky (gate).

## Routing

| Section | Reference | Purpose |
| ------- | --------- | ------- |
| Language HOW-TOs | [references/python-contract.md](references/python-contract.md) | Python contract — rules, template, section contract, workflow, verify |
| Language HOW-TOs | [references/rust-contract.md](references/rust-contract.md) | Rust contract — rules, template, section contract, workflow, verify |
| Language HOW-TOs | [references/typescript-contract.md](references/typescript-contract.md) | TypeScript contract — rules, template, section contract, workflow, verify |
| Diagnostic tree | [references/diagnostic-tree.md](references/diagnostic-tree.md) | First-failing-check routing for a broken contract |
| Repository layout | [references/repository-layout.md](references/repository-layout.md) | How protocol and aggregate files sit in `shared/` |
| Workflow | [references/workflow.md](references/workflow.md) | Step-by-step workflow from resolve-to-register |
| Verification | [references/verification.md](references/verification.md) | Primary gate + fallback language compile |
| Pre-flight | [references/pre-flight.md](references/pre-flight.md) | Checklist before drafting a contract file |
| Anti-patterns | [references/anti-patterns.md](references/anti-patterns.md) | Common shape and import mistakes, incl. concrete classes in contracts |
