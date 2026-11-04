---
name: aes-root
description: AES composition root and entry wiring for Python Rust TS. Use when creating container entry files.
when-to-use: "Use when creating or auditing a composition root (container or entry file) in Python, Rust, or TypeScript, or when wiring a feature's dependencies at the package top."
argument-hint: "<layer-path>"
---

# aes-root

Scaffold AES composition roots — `root_<concept>_container` wires one feature's capabilities to its contracts; `root_<concept>_entry` bootstraps and starts the process. The **layer** decides which suffix, which imports, and which structure apply.

## Global rules

- **Naming (AES101/AES102)**: file `root_<concept>_container` or `root_<concept>_entry`; documented exceptions (`main.rs`, `lib.rs`, `mod.rs`, `__init__.py`, `index.ts`) are listed in the language HOW-TO.
- **Roles**: container wires one feature's capabilities to contracts; entry bootstraps the app and composes feature containers.
- **Privilege**: root is the only layer allowed to depend on everything below — nothing below root imports root.
- **Behaviour**: no business logic, no orchestration policy, no technical parsing, no UI behaviour — those live in capabilities / agent / utility / surface.
- **DI**: bind to protocol/aggregate types (`Arc<dyn Trait>` in Rust) — concretions never leak above root.
- **Register**: `mod.rs` / `index.ts` / package barrel as required.
- **Verify**: `lint-arwaky-cli scan <layer-path>` → 0; language compile is fallback only.
- **Related**: aes-capabilities (wired), aes-agent (composed), aes-surface (started), aes-contract (bind target) — each carries its own HOW-TO.

## Routing

| Section | Reference | Purpose |
| ------- | --------- | ------- |
| Language HOW-TOs | [references/python-root.md](references/python-root.md) | Python root — rules, template, section contract, workflow, verify |
| Language HOW-TOs | [references/rust-root.md](references/rust-root.md) | Rust root — rules, template, section contract, workflow, verify |
| Language HOW-TOs | [references/typescript-root.md](references/typescript-root.md) | TypeScript root — rules, template, section contract, workflow, verify |
| Diagnostic tree | [references/diagnostic-tree.md](references/diagnostic-tree.md) | First-failing-check routing for a broken root file |
| Workflow | [references/workflow.md](references/workflow.md) | Step sequence from role-determination to register |
| Verification | [references/verification.md](references/verification.md) | Primary gate + fallback language compile |
| Pre-flight | [references/pre-flight.md](references/pre-flight.md) | Checklist before drafting a root file |
| Anti-patterns | [references/anti-patterns.md](references/anti-patterns.md) | Common wiring and layer-leak mistakes |
