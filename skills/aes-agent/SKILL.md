---
name: aes-agent
description: AES agent orchestrator scaffolding for Python Rust TS. Use when creating agent files.
metadata:
  tags:
    - python
    - rust
    - typescript
    - aes
    - agent
    - aggregate
    - orchestrator
    - structure
    - 3-block-structure
    - di
    - orchestration
    - vo
  related_skills:
    - aes-lint-arwaky
    - aes-capabilities
    - aes-taxonomy
    - aes-contract
    - aes-surface
  triggers:
    - create agent
    - create agent python
    - create agent rust
    - create agent typescript
    - add agent
    - add agent python
    - add agent rust
    - fix agent structure
    - create aggregate
    - create aggregate python
    - create aggregate rust
    - create aggregate typescript
    - agent missing aggregate
    - validate agent logic
    - check agent
    - check agent python
    - audit agent
    - audit agent python
    - audit agent rust
---

# aes-agent

> **Purpose**: Scaffold AES agent orchestrators — orchestration only: call the aggregate, return shared VOs, zero I/O and zero computation.
> **Audience**: The agent creating or validating an agent orchestrator file.
> **Scope**: Python, Rust, and TypeScript `agent_<domain>_orchestrator` files — one aggregate, 3-block structure.

The **aggregate** decides which suffix, which imports, and which structure apply.
Rules, templates, section contracts, and Verify blocks live in the language HOW-TUs under [`references/`](references/).

| Language | Focus | Body rule | HOW-TO |
| -------- | ----- | --------- | ------ |
| Python | Orchestrator | One aggregate; 3-block; zero I/O/computation | [references/HOW-TO-MAKE-PYTHON-AGENT.md](references/HOW-TO-MAKE-PYTHON-AGENT.md) |
| Rust | Orchestrator | One aggregate; 3-block; zero I/O/computation | [references/HOW-TO-MAKE-RUST-AGENT.md](references/HOW-TO-MAKE-RUST-AGENT.md) |
| TypeScript | Orchestrator | One aggregate; 3-block; zero I/O/computation | [references/HOW-TO-MAKE-TYPESCRIPT-AGENT.md](references/HOW-TO-MAKE-TYPESCRIPT-AGENT.md) |

**The layer chain:**

`contract_*_aggregate` (created via aes-contract) → **agent implements** → surface / root call the agent

Each file answers one layer's job. A method or import in the wrong layer is the defect this skill exists to prevent.

---

## Invariants

Every rule is machine-checked by `lint-arwaky-cli scan <layer-path>` (see each HOW-TO § Verify).
A rule cannot drift from the gate. Cite the linter, not this file, when pointing at a rule.

| Layer | Rule |
| ----- | ---- |
| Naming | File `agent_<domain>_orchestrator` — `_orchestrator` is the only suffix (AES101/AES102). |
| Structure | 3-block order: Block 1 type+injected deps → Block 2 aggregate methods only → Block 3 factories/dunders/helpers. ≥1 aggregate, ≤3 types (AES405). |
| Imports | Shared contracts/taxonomy/utility only — never capabilities, siblings, surface (AES201). |
| Behaviour | No I/O, no arithmetic/parsing/normalisation, no local domain data, no magic constants, no silently discarded errors. |
| DI | Injected aggregate only (`Arc<dyn Trait>` in Rust); shared VOs in signatures. |
| Register | Shared barrel so the root can compose the agent. |
| Verify | `lint-arwaky-cli scan <layer-path>` → 0. Language compile is fallback only. |

Split details, templates, and Section Contract tables: **read the language HOW-TO** — do not restate them here.

---

## Diagnostic Tree

Ask these questions in order. The first "No" dictates your next action.

1. **Is this orchestration only — or does it compute / touch I/O / hold domain data?**
   - *Computes/I-O/data* → move to capabilities / taxonomy / utility.
2. **Does an `_aggregate` contract exist?**
   - *No* → create it first with `aes-contract`.
3. **Block 2 only aggregate methods? ≥1 aggregate? ≤3 types?**
   - *No* → restructure to 1→2→3; extract free functions to utility.
4. **Any silently discarded errors or raw primitives in signatures?**
   - *Yes* → propagate/`Result`; wrap primitives in VOs.
5. **Does `lint-arwaky-cli scan <layer-path>` exit 0?**
   - *No* → fix findings, re-scan.

---

## Workflow

1. Confirm orchestration only — computation → capabilities, domain data → taxonomy, pure helpers → utility.
2. Ensure an `_aggregate` contract exists — if missing, run `aes-contract` first.
3. Resolve the feature agent dir. Run `lint-arwaky-cli scan <layer-path>` — findings are your work list.
4. Implement the 3 blocks from the language HOW-TO § Template / § Section Contract; inject the aggregate via DI.
5. Check imports, I/O, computation, error handling, and primitives against HOW-TO § Rules.
6. Register in the shared barrel, verify with `lint-arwaky-cli scan`, then compose via `aes-root` / consume via `aes-surface`.

---

## Verification

### Machine Checks

```bash
lint-arwaky-cli scan <layer-path>   # AES101/102, AES201–205, AES401–406 → must be 0
# Fallback only: language compile (python -c import / cargo check / npx tsc --noEmit)

```text

A pass means naming, imports, primitives, and roles are clean. Structural judgement
(tier choice, block order, helper-vs-utility, "orchestration only") is **manual** — see HOW-TO § Rules.

### Human Checks

A machine pass does not mean the file is right. Layer purpose, structural order, and
"only what this layer may do" still need a reader (HOW-TO § Rules).

---

## Pre-flight Checklist

- [ ] `lint-arwaky-cli scan <layer-path>` exits 0.
- [ ] Every touched HOW-TO's `Verify` block was executed.
- [ ] File registered in `__init__.py` / `mod.rs` / `index.ts`.
- [ ] Language fallback compile clean if the HOW-TO lists it.
- [ ] Related skills considered for the next layer up/down.

---

## Common Mistakes (Anti-Patterns)

The linter covers naming, imports, and primitives. These need a reader (HOW-TO § Rules):

- **Computation or I/O in the agent**: arithmetic/parsing/FS/network belong below the agent layer.
- **Silently discarded errors**: no `or ""` / `unwrap_or_default()` / `?? ""` on results.
- **Agent implements protocol instead of aggregate (or both)**: exactly one aggregate, 3-block order.
- **Restating HOW-TO rules in SKILL.md**: delegate — this file only routes.

---

## Related Skills

- `aes-contract`
- `aes-capabilities`
- `aes-surface`
