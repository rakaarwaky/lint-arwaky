---
name: aes-capabilities
description: "AES capability implementation scaffolding for Python Rust TS. Use when creating capabilities files."
when-to-use: "create, add, or audit an AES capabilities file (Python, Rust, TypeScript); fix capabilities structure; validate protocol-implementation logic; decide capability vs utility."
argument-hint: '<python | rust | typescript> [file path]'
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
| [references/python-capabilities.md](references/python-capabilities.md) | Python capabilities HOW-TO — rules, template, section contract, workflow steps, review, verify |
| [references/rust-capabilities.md](references/rust-capabilities.md) | Rust capabilities HOW-TO — rules, template, section contract, workflow steps, review, verify |
| [references/typescript-capabilities.md](references/typescript-capabilities.md) | TypeScript capabilities HOW-TO — rules, template, section contract, workflow steps, review, verify |
| Related skills | aes-contract, aes-utility, aes-agent, aes-root, aes-lint-arwaky |
