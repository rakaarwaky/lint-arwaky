---
name: aes-agent
description: "AES agent orchestrator scaffolding for Python Rust TS. Create a new agent file or improve/audit an existing one."
when-to-use: "Create a new AES agent or aggregate orchestrator file, or improve/audit an existing one. Pass a folder path (a crate/module/package dir) or a single file path; the language (python/rust/typescript) is auto-detected from the path and routed to the matching reference. Without --dry-run the task runs autonomously without asking for permission; with --dry-run it generates a report only and makes no changes."
argument-hint: '<path> [create|improve] [--dry-run]'
---

# aes-agent

Scaffold AES agent orchestrators — orchestration only: call the aggregate, return shared VOs, zero I/O and zero computation. The audience is the agent creating or validating an agent orchestrator file; the **layer** decides which suffix, which imports, and which structure apply. The layer chain reads `contract_*_aggregate` (created via aes-contract) → agent implements → surface / root call the agent — one agent implements exactly one aggregate. Invariants read by more than one step live in Global Rules below.

## Global Rules

- Argument shape: `<path> [create|improve] [--dry-run]`. `<path>` is a folder (crate/module/package) or a single file; missing mode defaults to improve.
- Language auto-detection: resolve the path, detect python (.py) / rust (.rs) / typescript (.ts/.tsx/.js/.jsx/.mts/.cts) from files present; on mixed or ambiguous input, list the detected languages and proceed with the dominant one, naming it in the report.
- Routing: dispatch to `references/<language>-<skill>.md` (the language HOW-TO) based on detected language; the language HOW-TO owns the workflow steps.
- Mode create: scaffold a new file from the language HOW-TO template section.
- Mode improve: audit the named file/folder against the language HOW-TO rules, list findings, apply fixes.
- Safety model: default (no `--dry-run`) = autonomous, no permission prompts, execute directly. `--dry-run` = read-only: produce the findings report and STOP before any write; the report is the deliverable. This bullet is the single source of truth for mode/safety semantics.
- Report: every run ends with a report block — files touched (dry-run: files that WOULD be touched), language detected, mode, findings/fixes.
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
| [references/python-agent.md](references/python-agent.md) | Python agent HOW-TO — both modes; rules, template, section contract, workflow steps, review, verify |
| [references/rust-agent.md](references/rust-agent.md) | Rust agent HOW-TO — both modes; rules, template, section contract, workflow steps, review, verify |
| [references/typescript-agent.md](references/typescript-agent.md) | TypeScript agent HOW-TO — both modes; rules, template, section contract, workflow steps, review, verify |
| Related skills | aes-contract, aes-capabilities, aes-surface, aes-root, aes-taxonomy |
