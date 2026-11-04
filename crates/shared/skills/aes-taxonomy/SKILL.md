---
name: aes-taxonomy
description: "AES taxonomy VO and domain type scaffolding for Python Rust TS. Create a new taxonomy file or improve/audit an existing one."
when-to-use: "Create a new AES taxonomy file, or improve/audit an existing one. Pass a folder path (a crate/module/package dir) or a single file path; the language (python/rust/typescript) is auto-detected from the path and routed to the matching reference. Without --dry-run the task runs autonomously without asking for permission; with --dry-run it generates a report only and makes no changes."
argument-hint: '<path> [create|improve] [--dry-run]'
metadata:
  tags:
    - python
    - rust
    - typescript
    - aes
    - taxonomy
    - shared
    - vo
    - entity
    - error
    - event
    - constant
    - primitive-to-vo
  related_skills:
    - aes-lint-arwaky
    - aes-capabilities
    - aes-agent
    - aes-contract
    - aes-utility
---

# aes-taxonomy

Scaffold AES taxonomy files — `taxonomy_<domain>_<suffix>` for VO / entity / error / event / constant / request / response, the stable language of the domain. The audience is the agent creating or validating a taxonomy file; the **suffix** decides the shape and the body rules. Taxonomy is the bottom layer of the chain `taxonomy → contract → capabilities → agent → surface → root` — every domain value flows through it. Invariants read by more than one step live in Global Rules below.

## Global Rules

- Argument shape: `<path> [create|improve] [--dry-run]`. `<path>` is a folder (crate/module/package) or a single file; missing mode defaults to improve.
- Language auto-detection: resolve the path, detect python (.py) / rust (.rs) / typescript (.ts/.tsx/.js/.jsx/.mts/.cts) from files present; on mixed or ambiguous input, list the detected languages and proceed with the dominant one, naming it in the report.
- Routing: dispatch to `references/<language>-<skill>.md` (the language HOW-TO) based on detected language; the language HOW-TO owns the workflow steps.
- Mode create: scaffold a new file from the language HOW-TO template section.
- Mode improve: audit the named file/folder against the language HOW-TO rules, list findings, apply fixes.
- Safety model: default (no `--dry-run`) = autonomous, no permission prompts, execute directly. `--dry-run` = read-only: produce the findings report and STOP before any write; the report is the deliverable. This bullet is the single source of truth for mode/safety semantics.
- Report: every run ends with a report block — files touched (dry-run: files that WOULD be touched), language detected, mode, findings/fixes.
- Naming: file `taxonomy_<domain>_<suffix>` — suffix strictly `_vo`/`_entity`/`_error`/`_event`/`_constant`/`_request`/`_response` (AES101/AES102).
- Imports: taxonomy + stdlib only — never capabilities, agents, surface, root, or contracts; no I/O (AES201).
- Primitives: domain fields wrap VOs — no raw `str`/`int`/`float`/`String`/`string`/`number` for domain values (AES401).
- Error contract: every `_error` file exposes `error_id` + `error_code` + `message`; ids come from per-feature blocks (common `000X`, feature 1 `1XXX`, …) and never change across releases.
- Construction: VOs validate on construction, are immutable, and constants are pure literals.
- Register: shared barrel (`__init__.py` / `mod.rs` / `index.ts`) so the type is reachable from every member.
- Placement: `shared/` folder only — `crates/shared/src/`, `modules/shared/src/`, `packages/shared/src/` (AES701).
- Verify: `lint-arwaky-cli scan <layer-path>` → 0. Language compile is fallback only.
- One fact, one home: the language HOW-TO carries the language's suffix shapes and section contract — read it, do not restate here. Cite the linter, not this file, when pointing at a rule. Each bullet states a fact once; references point at the bullet.

## Routing

| File | Purpose |
| --- | --- |
| [references/python-taxonomy.md](references/python-taxonomy.md) | Python taxonomy HOW-TO — both modes; rules, template, section contract, workflow steps, review, verify |
| [references/rust-taxonomy.md](references/rust-taxonomy.md) | Rust taxonomy HOW-TO — both modes; rules, template, section contract, workflow steps, review, verify |
| [references/typescript-taxonomy.md](references/typescript-taxonomy.md) | TypeScript taxonomy HOW-TO — both modes; rules, template, section contract, workflow steps, review, verify |
| Related skills | aes-contract, aes-utility, aes-capabilities, aes-agent, aes-lint-arwaky |
