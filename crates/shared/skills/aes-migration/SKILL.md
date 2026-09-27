---
name: aes-migration
description: AES architecture migration workflow for Python Rust TS. Use when migrating legacy codebases to AES layered architecture.
metadata:
  tags:
    - python
    - rust
    - typescript
    - aes
    - migration
    - refactor
    - architecture
    - dependency-injection
    - phase-gate
  related_skills:
    - aes-lint-arwaky
    - aes-taxonomy
    - aes-contract
    - aes-utility
    - aes-capabilities
    - aes-agent
    - aes-surface
    - aes-root
  triggers:
    - migrate to aes
    - migrate to aes python
    - migrate to aes rust
    - migrate to aes typescript
    - refactor to aes
    - aesify project
    - apply aes architecture
    - convert to aes
    - audit aes compliance
    - fix aes violations
---

# aes-migration

> **Purpose**: Guide phased migration of legacy codebases into AES layered architecture — taxonomy → contract → utility → capabilities → agent → surface → root.
> **Audience**: The agent planning or executing a migration to AES.
> **Scope**: Python, Rust, and TypeScript projects; references contain full migration playbooks.

This skill is a **router** — it points at language-specific migration guides under [`reference/`](reference/) and the layer skills that execute each phase. Rules, templates, and verify blocks live in the referenced HOW-TUs.

| Language | Playbook | Source |
| -------- | -------- | ------ |
| Python   | Phase-based migration workflow | [reference/HOW-TO-MAKE-PYTHON-MIGRATION.md](reference/HOW-TO-MAKE-PYTHON-MIGRATION.md) |
| Rust     | Phase-based migration workflow | [reference/HOW-TO-MAKE-RUST-MIGRATION.md](reference/HOW-TO-MAKE-RUST-MIGRATION.md) |
| TypeScript | Phase-based migration workflow | [reference/HOW-TO-MAKE-TYPESCRIPT-MIGRATION.md](reference/HOW-TO-MAKE-TYPESCRIPT-MIGRATION.md) |

**The dependency model:**

```text
root ── wires ──→ agent / surface / capabilities
                  │               │              │
                  ▼               ▼              ▼
            contract (protocol / aggregate)
                  │
                  ▼
            taxonomy (VOs, entities, errors, events, constants)
                  ▲
            utility ── imports only taxonomy ──┘

```text

Each layer answers one concern. A method or import in the wrong layer is the defect this skill exists to prevent.

---

## Invariants

Every rule is machine-checked by `lint-arwaky-cli scan <project-root>` (see each language playbook § Verify).
A rule cannot drift from the gate. Cite the linter, not this file, when pointing at a rule.

| Phase | Rule | Gate |
| ----- | ---- | ---- |
| 0 — Audit | Scan before touching anything; record baseline violations. | `lint-arwaky-cli scan .` |
| 1 — Taxonomy | Value objects, errors, events, constants only; no I/O; no upward imports (AES201). | `lint-arwaky-cli scan <taxonomy-dir>` |
| 2 — Contract | Protocol = 1 method / feature; aggregate = many methods / exports; pure declarations only. | `lint-arwaky-cli scan <contract-dir>` |
| 3 — Utility | Stateless free functions; taxonomy-only imports; ≥2 consumers (else keep private). | `lint-arwaky-cli scan <utility-dir>` |
| 4 — Capabilities | 3-block structure; ≥1 protocol implementor; ≤3 type declarations per file (AES403). | `lint-arwaky-cli scan <capabilities-dir>` |
| 5 — Agent | Implements aggregate; DI via protocol types (`Arc<dyn Trait>` / constructor injection); ≤3 types (AES405). | `lint-arwaky-cli scan <agent-dir>` |
| 6 — Surface | Imports aggregate (not agent/capabilities); call-site only for contracts (AES201 purpose). | `lint-arwaky-cli scan <surface-dir>` |
| 7 — Root | Wires capabilities → agent → surface; no business logic; entry bootstrap only. | `lint-arwaky-cli scan <root-dir>` |
| 8 — Verify | 0 violations across all layers; language compile clean; tests pass. | `lint-arwaky-cli scan .` → 0; `python -m compileall` / `cargo check` / `npx tsc --noEmit` |

Split import matrices, phase templates, and troubleshooting tables: **read the language HOW-TO** — do not restate them here.

---

## Diagnostic Tree

Ask these questions in order. The first "No" dictates your next action.

1. **Is this a legacy codebase without AES layers?**
   - *No* → verify compliance with `lint-arwaky-cli scan`; fix findings per layer skill.
   - *Yes* → proceed to Phase 0.
2. **Has the baseline audit been run?**
   - *No* → `lint-arwaky-cli scan .`; record violation count to choose strategy (< 10: full, 10–50: phased, > 50: taxonomy-first).
   - *Yes* → proceed to Phase 1.
3. **Does the taxonomy layer expose only domain values (no I/O, no upward imports)?**
   - *No* → strip I/O to capabilities; fix primitives to VOs (AES401); re-scan.
   - *Yes* → proceed to Phase 2.
4. **Is every protocol single-method and every aggregate multi-method?**
   - *No* → split into per-feature protocols; expand aggregate exports (AES402/AES403).
   - *Yes* → proceed to Phase 3.
5. **Does `lint-arwaky-cli scan <project-root>` exit 0?**
   - *No* → fix findings in order: taxonomy → contract → utility → capabilities → agent → surface → root.
   - *Yes* → migration complete; run tests and format.

---

## Workflow

1. **Load the language playbook**: Python → `reference/HOW-TO-MAKE-PYTHON-MIGRATION.md`, Rust → `reference/HOW-TO-MAKE-RUST-MIGRATION.md`, TypeScript → `reference/HOW-TO-MAKE-TYPESCRIPT-MIGRATION.md`.
2. **Phase 0 — Audit**: Run `lint-arwaky-cli scan .`; categorize violations by layer; choose strategy.
3. **Phase 1 — Taxonomy**: Extract VOs, errors, constants; register in shared barrel.
4. **Phase 2 — Contract**: Create `I<Concept>Protocol` (1 method) and `I<Concept>Aggregate` (many methods); register.
5. **Phase 3 — Utility**: Extract stateless helpers; move to shared; register.
6. **Phase 4 — Capabilities**: Implement protocols; follow 3-block structure; verify AES403.
7. **Phase 5 — Agent**: Implement aggregates; inject via constructors; verify AES405.
8. **Phase 6 — Surface**: Delegate to agent aggregate; verify AES201 purpose (call, not impl).
9. **Phase 7 — Root**: Wire containers; bootstrap entry; register in barrel/mod/index.
10. **Phase 8 — Verify**: Full scan → 0 violations; language compile clean; tests green.

---

## Verification

### Machine Checks

```bash
# Phase 0 baseline
lint-arwaky-cli scan .

# Per-phase (replace <dir> with the layer directory)
lint-arwaky-cli scan <taxonomy-dir>
lint-arwaky-cli scan <contract-dir>
lint-arwaky-cli scan <utility-dir>
lint-arwaky-cli scan <capabilities-dir>
lint-arwaky-cli scan <agent-dir>
lint-arwaky-cli scan <surface-dir>
lint-arwaky-cli scan <root-dir>

# Final gate
lint-arwaky-cli scan .   # must be 0
python -m compileall -q <modules>   # Python fallback
cargo check --workspace                    # Rust fallback
npx tsc --noEmit                           # TypeScript fallback

```text

A pass means naming, layer imports, primitives, and roles are clean. Structural judgement
(3-block order, orchestration purity, DI wiring) is **manual** — see language HOW-TO § Rules.

### Human Checks

A machine pass does not mean the migration is right. Protocol thinness, aggregate export
richness, and "only methods outer layers call" still need a reader (layer HOW-TUs).

---

## Pre-flight Checklist

- [ ] `lint-arwaky-cli scan .` baseline captured.
- [ ] Language playbook loaded (`HOW-TO-MAKE-PYTHON-MIGRATION.md` / `HOW-TO-MAKE-RUST-MIGRATION.md` / `HOW-TO-MAKE-TYPESCRIPT-MIGRATION.md`).
- [ ] Each layer skill consulted: `aes-taxonomy`, `aes-contract`, `aes-utility`, `aes-capabilities`, `aes-agent`, `aes-surface`, `aes-root`.
- [ ] Every touched layer's `Verify` block executed.
- [ ] Protocol = one method for one feature; aggregate = one method per export (manual).
- [ ] Files registered in `__init__.py` / `mod.rs` / `index.ts`.
- [ ] Language fallback compile clean if the playbook lists it.

---

## Common Mistakes (Anti-Patterns)

The linter covers naming, imports, and primitives. These need a reader (language playbook § Troubleshooting):

- **Skipping taxonomy**: jumping to capabilities without VOs → primitive leakage (AES401).
- **Mega-protocol**: one protocol with many methods → split into per-feature protocols.
- **Mirror-aggregate**: aggregate that mirrors protocol exactly → add rich export surface.
- **Import inversion**: capabilities importing agent, or surface importing capabilities → route through contract (AES201).
- **Utility with state**: class / `self` / `this` in utility → free functions only (AES404).
- **Agent with business logic**: computation in agent → move to capabilities (AES405).
- **Surface calling capabilities directly**: skip agent → surface must call aggregate (AES201 purpose).
- **Root with business logic**: logic in container/entry → move down to capabilities/agent.
- **Restating playbook content in SKILL.md**: delegate — this file only routes.

---

## Related Skills

- `aes-taxonomy`
- `aes-contract`
- `aes-utility`
- `aes-capabilities`
- `aes-agent`
- `aes-surface`
- `aes-root`
- `aes-lint-arwaky`
