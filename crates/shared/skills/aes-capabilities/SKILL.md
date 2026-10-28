---
name: aes-capabilities
description: AES capability implementation scaffolding for Python Rust TS. Use when creating capability files.
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
  triggers:
    - create capabilities
    - create capabilities python
    - create capabilities rust
    - create capabilities typescript
    - add capabilities
    - add capabilities python
    - add capabilities rust
    - fix capabilities structure
    - create protocol
    - create protocol python
    - create protocol rust
    - create protocol typescript
    - capabilities missing protocol
    - validate capabilities logic
    - check capabilities
    - check capabilities python
    - audit capabilities
    - audit capabilities python
    - audit capabilities rust

when-to-use: Use when creating AES capability files — `capabilities_<domain>_<role>` — with 3-block structure, ≥1 protocol implementor, ≤3 types; triggers include `create capabilities`, `add capabilities`, `create protocol`, `validate capabilities logic`, `audit capabilities`.
argument-hint: "[python | rust | typescript] [--check]"
---

# aes-capabilities

> **Purpose**: Scaffold AES capability files — concrete protocol implementations (domain rules + external adaptation).
> **Audience**: The agent creating or validating a capability file.
> **Scope**: Python, Rust, and TypeScript `capabilities_<domain>_<role>` files — 3-block structure, ≥1 protocol implementor, ≤3 types.

## Capability vs Utility — Defining the Boundary

A capability and a utility are fundamentally different layers. Confusing them causes misplaced code, wrong imports, and linter violations.

| | **Capability** (`capabilities_*`) | **Utility** (`utility_*`)|
|---|---|---|
| **Core question** | What can the system **do**? | How is a small task **done**?|
| **Nature** | Concrete protocol behaviour — domain rules, external adaptation, DI-wired | Stateless, domain-agnostic, pure free functions|
| **Responsibility** | Implements a `_protocol` contract; orchestrates inputs into domain outcomes | Performs narrow, reusable operations with no business logic|
| **State** | Has instance state via injected dependencies (`Arc<dyn Trait>`, DI params) | Absolutely no state — pure functions only|
| **Location** | Feature folder (`crates/<feature>/`, `modules/<feature>/`, `packages/<feature>/`) | Shared folder (`shared/`) beside taxonomy and contract files|
| **Consumers** | Consumed by agent orchestrators or root composition | Consumed by capabilities, agents, surfaces, or other utilities|
| **Examples** | `capabilities_user_auth`, `capabilities_data_persist`, `capabilities_rate_limit` | `utility_string_sanitizer`, `utility_date_formatter`, `utility_hasher`|

**Decision rule**: Ask "Does this code implement a protocol behaviour with business rules?" If yes → capability. Ask "Does this code perform a narrow, stateless, reusable operation with no business logic?" If yes → utility. If it does both, split it: business logic stays in the capability; the stateless helper moves to utility.

The **layer** decides which suffix, which imports, and which structure apply.
Rules, templates, section contracts, and Verify blocks live in the language HOW-TUs under [`references/`](references/).

| Language | Focus | Body rule | HOW-TO|
| -------- | ----- | --------- | ------|
| Python | Protocol implementation | 3-block; ≥1 protocol; ≤3 types | [references/HOW-TO-MAKE-PYTHON-CAPABILITIES.md](references/HOW-TO-MAKE-PYTHON-CAPABILITIES.md)|
| Rust | Protocol implementation | 3-block; ≥1 protocol; ≤3 types | [references/HOW-TO-MAKE-RUST-CAPABILITIES.md](references/HOW-TO-MAKE-RUST-CAPABILITIES.md)|
| TypeScript | Protocol implementation | 3-block; ≥1 protocol; ≤3 types | [references/HOW-TO-MAKE-TYPESCRIPT-CAPABILITIES.md](references/HOW-TO-MAKE-TYPESCRIPT-CAPABILITIES.md)|

**The layer chain:**

`contract_*_protocol` (created via aes-contract) → **capabilities implement** → agent / root consume

Each file answers one layer's job. A method or import in the wrong layer is the defect this skill exists to prevent.

---

## Invariants

The full invariant table, placement rules, diagnostic tree, workflow, verification,
pre-flight checklist, and mistake patterns live in [`references/invariants.md`](references/invariants.md).

---

---

## Diagnostic Tree

Ask these questions in order. The first "No" dictates your next action. Full tree in [`references/invariants.md`](references/invariants.md):

1. **Is this protocol behaviour (not a stateless helper or pure function)?** → Otherwise it's a utility (see `aes-utility`).
2. **Does a `_protocol` contract exist for this behaviour?** → Otherwise create it first with `aes-contract`.
3. **Block 2 contain only protocol methods? ≥1 implementor? ≤3 types?** → Restructure to 1→2→3; split or extract helpers.
4. **Any forbidden imports or local domain models?** → Strip; shared needs go through contract or root.
5. **Does `lint-arwaky-cli scan <layer-path>` exit 0?** → Fix findings, re-scan.

## Workflow

1. Confirm the work is protocol behaviour (not orchestration / data / mechanics).
2. Ensure a `_protocol` contract exists — if missing, run `aes-contract` first.
3. Resolve the feature capability dir. Run `lint-arwaky-cli scan <layer-path>` — findings are your work list.
4. Implement the 3 blocks from the language HOW-TO § Template / § Section Contract.
5. Strip forbidden imports / inter-capability deps / local domain models; move constants and pure helpers out.
6. Register in the shared barrel, verify with `lint-arwaky-cli scan`, then wire via `aes-agent` / `aes-root`.

## Verification

### Machine Checks

```bash
lint-arwaky-cli scan <layer-path>   # AES101/102, AES201–205, AES401–406 → must be 0
# Fallback only: language compile (python -c import / cargo check / npx tsc --noEmit)
```

### Human Checks

A machine pass does not mean the file is right. Layer purpose, structural order, and "only what this layer may do" still need a reader (HOW-TO § Rules).

## Pre-flight Checklist

- [ ] `lint-arwaky-cli scan <layer-path>` exits 0.
- [ ] Every touched HOW-TO's `Verify` block was executed.
- [ ] File registered in `__init__.py` / `mod.rs` / `index.ts`.
- [ ] Language fallback compile clean if the HOW-TO lists it.
- [ ] Related skills considered for the next layer up/down.

## Common Mistakes (Anti-Patterns)

- **Stateless helper with no protocol behaviour in a capability file**: extract to `utility_*` — it belongs in shared, not the feature folder.
- **Business logic mixed with a pure helper**: split — business rules stay; stateless logic moves to utility.
- **Capabilities importing each other**: shared needs go through contract or root, not sibling imports.
- **Restating HOW-TO rules in SKILL.md**: delegate — this file only routes.

---

## Related Skills

- `aes-contract`
- `aes-utility`
- `aes-agent`
