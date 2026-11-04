---
name: aes-migration
description: AES architecture migration workflow for Python Rust TS. Use when migrating legacy codebases to AES layered architecture.
when-to-use: "Use when migrating, refactoring, or converting a legacy Python, Rust, or TypeScript codebase into the AES 7-layer architecture, auditing AES compliance, or planning the phased taxonomy-first migration."
argument-hint: "<project-root>"
---

# aes-migration

Guide the phased migration of a legacy codebase into the AES layered architecture: Phase 0 audits the baseline, Phases 1–7 build each layer bottom-up (taxonomy → contract → utility → capabilities → agent → surface → root), and Phase 8 verifies to zero. Each layer's shape is owned by its layer skill; the language playbook carries import matrices, phase templates, and troubleshooting.

## Global rules

- **Baseline first**: run `lint-arwaky-cli scan .` before touching anything; the recorded violation count picks the strategy (< 10: full, 10–50: phased, > 50: taxonomy-first).
- **Layer order**: migrate bottom-up — taxonomy → contract → utility → capabilities → agent → surface → root; a layer starts only when the one below scans clean.
- **Per-phase gate**: `lint-arwaky-cli scan <layer-dir>` → 0 before moving to the next layer; language compile (`python -m compileall` / `cargo check` / `npx tsc --noEmit`) is fallback only.
- **Done**: Phase 8 — full scan of the project root exits 0, language compile clean, tests green; commit `migrate: <layer> phase to AES (<rules>)` only when asked.
- **Shape ownership**: protocol/aggregate shape is `aes-contract`, structure is `aes-capabilities`, DI is `aes-agent`, wiring is `aes-root` — this skill routes; it does not restate layer shape.
- **One fact, one home**: the language playbook carries the phase templates and troubleshooting table — read it, do not restate here. Cite the linter, not this file, when pointing at a rule.
- **Related**: aes-lint-arwaky (gate), aes-taxonomy, aes-contract, aes-utility, aes-capabilities, aes-agent, aes-surface, aes-root — one per layer phase.

## Routing

| Section | Reference | Purpose |
| ------- | --------- | ------- |
| Language playbooks | [references/python-migration.md](references/python-migration.md) | Python phase 0–8 playbook — import matrix, templates, verify |
| Language playbooks | [references/rust-migration.md](references/rust-migration.md) | Rust phase 0–8 playbook — import matrix, templates, verify |
| Language playbooks | [references/typescript-migration.md](references/typescript-migration.md) | TypeScript phase 0–8 playbook — import matrix, templates, verify |
| Phase gates | [references/phase-gates.md](references/phase-gates.md) | Per-phase rule and gate table |
| Diagnostic tree | [references/diagnostic-tree.md](references/diagnostic-tree.md) | Strategy selection and first-failing-check routing |
| Workflow | [references/workflow.md](references/workflow.md) | Phase 0–8 step sequence |
| Verification | [references/verification.md](references/verification.md) | Baseline, per-phase, and final gate commands |
| Pre-flight | [references/pre-flight.md](references/pre-flight.md) | Baseline captured, playbook loaded, layer skills consulted |
| Anti-patterns | [references/anti-patterns.md](references/anti-patterns.md) | Thin-protocol, fat-aggregate, import-inversion, layer leaks |
