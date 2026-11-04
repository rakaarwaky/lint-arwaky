---
name: aes-capabilities
description: AES capability implementation scaffolding for Python Rust TS. Use when creating capability files.
when-to-use: "Use when creating or auditing a capabilities file (Python, Rust, TypeScript), adding or fixing a capabilities block, or deciding capability vs utility."
argument-hint: "<layer-path>"
---

# aes-capabilities

Scaffold AES capability files — concrete protocol implementations (domain rules + external adaptation). The **layer** decides which suffix, which imports, and which structure apply.

## Global rules

- **Capability vs Utility boundary**: A capability implements a `_protocol` contract with instance state (injected dependencies). A utility is a stateless, domain-agnostic pure function. If a piece of code is stateless and reusable, it is a utility; if it carries DI-wired state, it is a capability.
- **Placement (AES701, AES702, AES703)**: `capabilities_*` files belong in a **feature folder** — a subdirectory of a member directory that is neither `shared/` nor a surface folder. Every feature folder carries `FRD.md` and `BACKLOG.md` (AES702); a capabilities file placed in `shared/` or a surface folder is an AES701/AES703 violation.
- **Naming (AES101/AES102)**: file `capabilities_<domain>_<role>` — role from the internal/external naming lists; forbidden suffixes `_vo`/`_entity`/`_error`/`_event`/`_constant`/`_protocol`/`_aggregate`/`_utility`/`_request`/`_response`.
- **Structure (AES403)**: 3-block order — Block 1 type + ctor → Block 2 protocol methods only → Block 3 factories/dunders/helpers; ≥1 protocol implementor, ≤3 types.
- **Imports (AES201–AES205)**: taxonomy + `_protocol` contracts + utility only — never agents, sibling capabilities, surface, or local domain models.
- **DI**: protocol interfaces only (`Arc<dyn Trait>` in Rust); shared VOs in fields/signatures.
- **Helpers**: stateless domain-agnostic ≥2-consumer functions move to `utility_*`; constants to `taxonomy_*_constant`.
- **Register**: shared barrel (`__init__.py` / `mod.rs` / `index.ts`).
- **Verify**: `lint-arwaky-cli scan <layer-path>` → 0; language compile is fallback only.
- **Related**: aes-contract (upstream protocol), aes-agent (consumes capabilities), aes-surface (consumes agent), aes-utility (stateless helpers), aes-lint-arwaky (gate).

## Routing

| Section | Reference | Purpose |
| ------- | --------- | ------- |
| Language HOW-TOs | [references/python-capabilities.md](references/python-capabilities.md) | Python capabilities — rules, template, section contract, workflow, verify |
| Language HOW-TOs | [references/rust-capabilities.md](references/rust-capabilities.md) | Rust capabilities — rules, template, section contract, workflow, verify |
| Language HOW-TOs | [references/typescript-capabilities.md](references/typescript-capabilities.md) | TypeScript capabilities — rules, template, section contract, workflow, verify |
| Diagnostic tree | [references/diagnostic-tree.md](references/diagnostic-tree.md) | First-failing-check routing for a broken capability file |
| Workflow | [references/workflow.md](references/workflow.md) | Step-by-step workflow from confirm-to-register |
| Verification | [references/verification.md](references/verification.md) | Primary gate + fallback language compile |
| Pre-flight | [references/pre-flight.md](references/pre-flight.md) | Checklist before starting a capability file |
| Anti-patterns | [references/anti-patterns.md](references/anti-patterns.md) | Common structural and import mistakes |
