---
name: aes-contract
description: "AES contract protocol scaffolding for Python Rust TS. Create a new contract file or improve/audit an existing one."
when-to-use: "Create a new AES contract file, or improve/audit an existing one. Pass a folder path (a crate/module/package dir) or a single file path; the language (python/rust/typescript) is auto-detected from the path and routed to the matching reference. Without --dry-run the task runs autonomously without asking for permission; with --dry-run it generates a report only and makes no changes."
argument-hint: '<path> [create|improve] [--dry-run]'
metadata:
  tags:
    - python
    - rust
    - typescript
    - aes
    - contract
    - protocol
    - aggregate
    - abc
    - trait
    - interface
    - vo
  related_skills:
    - aes-capabilities
    - aes-agent
    - aes-taxonomy
    - aes-lint-arwaky
---

# aes-contract

Scaffold AES contract files — declarations only: `_protocol` (one file per feature, one trait/class/interface per capability seam, each rich) and `_aggregate` (exactly one `execute()` method returning a taxonomy-defined response enum). The audience is the agent creating or validating a contract file; the **layer** decides which suffix, how rich the protocol is, and how the consumer surface is shaped. The layer chain reads `contract_*_protocol` (inward: capabilities implement) → `contract_*_aggregate` (outward: surface / root call) — a contract owns no implementation. Invariants read by more than one step live in Global Rules below.

## Global Rules

- Argument shape: `<path> [create|improve] [--dry-run]`. `<path>` is a folder (crate/module/package) or a single file; missing mode defaults to improve.
- Language auto-detection: resolve the path, detect python (.py) / rust (.rs) / typescript (.ts/.tsx/.js/.jsx/.mts/.cts) from files present; on mixed or ambiguous input, list the detected languages and proceed with the dominant one, naming it in the report.
- Routing: dispatch to `references/<language>-<skill>.md` (the language HOW-TO) based on detected language; the language HOW-TO owns the workflow steps.
- Mode create: scaffold a new file from the language HOW-TO template section.
- Mode improve: audit the named file/folder against the language HOW-TO rules, list findings, apply fixes.
- Safety model: default (no `--dry-run`) = autonomous, no permission prompts, execute directly. `--dry-run` = read-only: produce the findings report and STOP before any write; the report is the deliverable. This bullet is the single source of truth for mode/safety semantics.
- Report: every run ends with a report block — files touched (dry-run: files that WOULD be touched), language detected, mode, findings/fixes.
- Naming: file `contract_<concept>_<suffix>` — suffix strictly `_protocol` or `_aggregate` (AES101/AES102).
- Protocol shape: one file per feature, one trait/class/interface per capability seam, each rich with named methods.
- Aggregate shape: exactly one `execute()` method; the response is a taxonomy-defined enum of VOs.
- Body: pure declaration only — abstract (`...` / `pass`), trait ending in `;`, interface signature. No default bodies, no helpers, no concrete classes; shared logic belongs in `utility_*` (AES402).
- Imports: taxonomy + other contracts only — never capabilities, agents, surface, root (AES201/AES205).
- Signatures: shared VOs only — no raw primitives for domain values (AES402).
- Register: shared barrel (`__init__.py` / `mod.rs` / `index.ts`) so capabilities and consumers resolve the contract.
- Verify: `lint-arwaky-cli scan <contract-dir>` → 0. Language compile is fallback only.
- One fact, one home: the language HOW-TO carries the language's declaration syntax and section contract — read it, do not restate here. Cite the linter, not this file, when pointing at a rule. Each bullet states a fact once; references point at the bullet.

## Routing

| File | Purpose |
| --- | --- |
| [references/python-contract.md](references/python-contract.md) | Python contract HOW-TO — both modes; rules, template, section contract, workflow steps, review, verify |
| [references/rust-contract.md](references/rust-contract.md) | Rust contract HOW-TO — both modes; rules, template, section contract, workflow steps, review, verify |
| [references/typescript-contract.md](references/typescript-contract.md) | TypeScript contract HOW-TO — both modes; rules, template, section contract, workflow steps, review, verify |
| Related skills | aes-taxonomy, aes-capabilities, aes-agent, aes-surface, aes-lint-arwaky |
