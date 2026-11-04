---
name: aes-capabilities
description: "AES capability implementation scaffolding for Python Rust TS. Create a new capabilities file or improve/audit an existing one."
when-to-use: "Create a new AES capabilities file, or improve/audit an existing one. Pass a folder path (a crate/module/package dir) or a single file path; the language (python/rust/typescript) is auto-detected from the path and routed to the matching reference. Without --dry-run the task runs autonomously without asking for permission; with --dry-run it generates a report only and makes no changes."
argument-hint: '<path> [create|improve] [--dry-run]'
metadata:
  tags:
    - python
    - rust
    - typescript
    - aes
    - capabilities
    - protocol
    - role-naming
    - 3-block-structure
    - di
    - vo
  related_skills:
    - aes-lint-arwaky
    - aes-agent
    - aes-taxonomy
    - aes-contract
    - aes-utility
---

# aes-capabilities

Scaffold AES capabilities files — concrete protocol implementations carrying domain rules and external adaptation, in the 3-block structure (type+ctor → protocol methods → factories/helpers). The audience is the agent creating or validating a capabilities file; the **layer** decides which role, which imports, and which structure apply. The layer chain reads `contract_*_protocol` (created via aes-contract) → capabilities implement → agent / root consume — one capabilities file implements one protocol. Invariants read by more than one step live in Global Rules below.

## Global Rules

- Argument shape: `<path> [create|improve] [--dry-run]`. `<path>` is a folder (crate/module/package) or a single file; missing mode defaults to improve.
- Language auto-detection: resolve the path, detect python (.py) / rust (.rs) / typescript (.ts/.tsx/.js/.jsx/.mts/.cts) from files present; on mixed or ambiguous input, list the detected languages and proceed with the dominant one, naming it in the report.
- Routing: dispatch to `references/<language>-<skill>.md` (the language HOW-TO) based on detected language; the language HOW-TO owns the workflow steps.
- Mode create: scaffold a new file from the language HOW-TO template section.
- Mode improve: audit the named file/folder against the language HOW-TO rules, list findings, apply fixes.
- Safety model: default (no `--dry-run`) = autonomous, no permission prompts, execute directly. `--dry-run` = read-only: produce the findings report and STOP before any write; the report is the deliverable. This bullet is the single source of truth for mode/safety semantics.
- Report: every run ends with a report block — files touched (dry-run: files that WOULD be touched), language detected, mode, findings/fixes.
- Naming: file `capabilities_<domain>_<role>`; role from the internal/external naming list; forbidden suffixes `_vo`/`_entity`/`_error`/`_event`/`_constant`/`_protocol`/`_aggregate`/`_utility`/`_request`/`_response` (AES101/AES102).
- Structure: 3-block order — Block 1 type+ctor → Block 2 protocol methods only → Block 3 factories/dunders/helpers; ≥1 protocol implementor, ≤3 types (AES403).
- Imports: taxonomy + `_protocol` contracts + utility only — never agents, siblings, surface, local domain models (AES201–AES205).
- DI: protocol interfaces only (`Arc<dyn Trait>` in Rust); shared VOs in fields/signatures.
- Behaviour: business rules and external adaptation live here; stateless domain-agnostic helpers move to `utility_*`; constants move to `taxonomy_*_constant` (AES404).
- Boundary: capability = "what can the system do" (stateful, protocol behaviour, DI-wired); utility = "how is a small task done" (stateless, pure, shared) — see [references/](references/).
- Register: shared barrel (`__init__.py` / `mod.rs` / `index.ts`) so the root can wire the capability.
- Verify: `lint-arwaky-cli scan <layer-path>` → 0. Language compile is fallback only.
- One fact, one home: the language HOW-TO carries the language's suffix, imports, and structure — read it, do not restate here. Cite the linter, not this file, when pointing at a rule. Each bullet states a fact once; references point at the bullet.

## Routing

| File | Purpose |
| --- | --- |
| [references/python-capabilities.md](references/python-capabilities.md) | Python capabilities HOW-TO — both modes; rules, template, section contract, workflow steps, review, verify |
| [references/rust-capabilities.md](references/rust-capabilities.md) | Rust capabilities HOW-TO — both modes; rules, template, section contract, workflow steps, review, verify |
| [references/typescript-capabilities.md](references/typescript-capabilities.md) | TypeScript capabilities HOW-TO — both modes; rules, template, section contract, workflow steps, review, verify |
| Related skills | aes-contract, aes-utility, aes-agent, aes-root, aes-lint-arwaky |
