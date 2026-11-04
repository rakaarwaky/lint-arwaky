---
name: aes-utility
description: "AES stateless utility scaffolding for Python Rust TS. Create a new utility file or improve/audit an existing one."
when-to-use: "Create a new AES utility file, or improve/audit an existing one. Pass a folder path (a crate/module/package dir) or a single file path; the language (python/rust/typescript) is auto-detected from the path and routed to the matching reference. Without --dry-run the task runs autonomously without asking for permission; with --dry-run it generates a report only and makes no changes."
argument-hint: '<path> [create|improve] [--dry-run]'
metadata:
  tags:
    - python
    - rust
    - typescript
    - aes
    - utility
    - shared
    - stateless
    - pure-function
    - domain-agnostic
    - reusability
  related_skills:
    - aes-lint-arwaky
    - aes-taxonomy
    - aes-capabilities
    - aes-agent
---

# aes-utility

Scaffold AES utility files — `utility_<domain>_<role>` free functions that are stateless, domain-agnostic, and shared across layers. The audience is the agent creating or validating a utility file; the **language** decides which declarations are legal. A utility answers "how is a small task done"; a capability answers "what can the system do" — code that does both splits: business logic stays in the capability, the stateless helper moves here. Invariants read by more than one step live in Global Rules below.

## Global Rules

- Argument shape: `<path> [create|improve] [--dry-run]`. `<path>` is a folder (crate/module/package) or a single file; missing mode defaults to improve.
- Language auto-detection: resolve the path, detect python (.py) / rust (.rs) / typescript (.ts/.tsx/.js/.jsx/.mts/.cts) from files present; on mixed or ambiguous input, list the detected languages and proceed with the dominant one, naming it in the report.
- Routing: dispatch to `references/<language>-<skill>.md` (the language HOW-TO) based on detected language; the language HOW-TO owns the workflow steps.
- Mode create: scaffold a new file from the language HOW-TO template section.
- Mode improve: audit the named file/folder against the language HOW-TO rules, list findings, apply fixes.
- Safety model: default (no `--dry-run`) = autonomous, no permission prompts, execute directly. `--dry-run` = read-only: produce the findings report and STOP before any write; the report is the deliverable. This bullet is the single source of truth for mode/safety semantics.
- Report: every run ends with a report block — files touched (dry-run: files that WOULD be touched), language detected, mode, findings/fixes.
- Naming: file `utility_<domain>_<role>` — role flexible; forbidden suffixes `_vo`/`_entity`/`_error`/`_event`/`_constant`/`_protocol`/`_aggregate` (AES101/AES102).
- Structure: free / module-level / exported functions only — no class, no `struct`/`impl`/trait/`type` alias (Rust), no `interface`/`enum`/`type` alias (TS), no `self`/`this` (AES404).
- State: stateless and deterministic — no `random`/`now()`/`Date.now()`, no global mutable state.
- Domain: domain-agnostic with ≥2 consumers; a single-feature helper stays a private helper inside that feature folder.
- Imports: taxonomy only — never capabilities, agents, surface, contracts, or other `utility_*` files (AES201).
- Register: shared barrel (`__init__.py` / `mod.rs` / `index.ts`) so every member can call the helper.
- Placement: `shared/` folder only — `crates/shared/src/`, `modules/shared/src/`, `packages/shared/src/` (AES701).
- Verify: `lint-arwaky-cli scan <layer-path>` → 0. Language compile is fallback only.
- One fact, one home: the language HOW-TO carries the language's declaration limits and section contract — read it, do not restate here. Cite the linter, not this file, when pointing at a rule. Each bullet states a fact once; references point at the bullet.

## Routing

| File | Purpose |
| --- | --- |
| [references/python-utility.md](references/python-utility.md) | Python utility HOW-TO — both modes; rules, template, section contract, workflow steps, review, verify |
| [references/rust-utility.md](references/rust-utility.md) | Rust utility HOW-TO — both modes; rules, template, section contract, workflow steps, review, verify |
| [references/typescript-utility.md](references/typescript-utility.md) | TypeScript utility HOW-TO — both modes; rules, template, section contract, workflow steps, review, verify |
| Related skills | aes-taxonomy, aes-capabilities, aes-agent, aes-lint-arwaky |
