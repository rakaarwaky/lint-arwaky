---
name: aes-docs
description: Adds docstrings, doc comments, JSDoc, types and PRD/ROADMAP/FRD/README/BACKLOG/DESIGN/AGENTS docs. Use when documenting Python, Rust, TS modules, or splitting spec from status backlog.
metadata:
  tags:
    - python
    - rust
    - typescript
    - docs
    - docstring
    - doc-comments
    - jsdoc
    - type-hints
    - prd
    - roadmap
    - frd
    - readme
    - backlog
    - agents-md
    - pep257
  related_skills:
    - aes-taxonomy
    - aes-contract
    - aes-lint-arwaky
  triggers:
    - add docs
    - add doc comments
    - document public api
    - add docstring
    - add jsdoc
    - add type hints
    - add prd
    - add frd
    - add backlog
    - add backlog md
    - add feature backlog
    - add root roadmap
    - add roadmap
    - add readme
    - add package readme
    - add crate readme
    - add agents md
    - agents md template
    - port agents md
    - prd template
    - frd template
    - readme template
    - backlog template
    - roadmap template
    - split spec from status
    - move status out of frd
    - audit document invariants
    - add design md
    - design md template
    - brand and style template

when-to-use: Use when documenting Python/Rust/TS modules — docstrings, JSDoc, PRD/ROADMAP/FRD/README/BACKLOG/DESIGN/AGENTS docs — and splitting spec from status backlog; triggers include `add docs`, `add docstring`, `add prd`, `add frd`, `add readme`, `add agents md`, `split spec from status`.
argument-hint: "[python | rust | typescript] [prd | frd | readme | backlog | roadmap | design | agents-md | docs]"
---
# aes-docs

> **Purpose**: Route every claim to the correct document, and ensure every public code item is documented.
> **Audience**: The AI agent executing documentation tasks.
> **Scope**: Python, Rust, and TypeScript modules; PRD, ROADMAP, FRD, README, BACKLOG, DESIGN, and AGENTS files.

The **aggregate** defines which document exists, where it lives, who reads it, and which claim belongs where.
Templates, section contracts, exemplars, and per-document craft rules live in [`references/`](references/).

| Document     | Location                          | Audience                     | Focus                                | Length       | Template                                                               |
| ------------ | --------------------------------- | ---------------------------- | ------------------------------------ | ------------ | ---------------------------------------------------------------------- |
| `PRD.md`     | Root workspace                    | Stakeholder, PM, Design, Eng | *What* &amp; *Why*                   | 50–500 lines | [references/HOW-TO-MAKE-PRD.md](references/HOW-TO-MAKE-PRD.md)         |
| `ROADMAP.md` | Root workspace (exactly one)      | Tech Lead, PM, Engineers     | *Progress, status, cross-cutting links* | 50–500 lines | [references/HOW-TO-MAKE-ROADMAP.md](references/HOW-TO-MAKE-ROADMAP.md) |
| `FRD.md`     | Each **feature** module/crate/pkg (not `shared/`) | Engineer, QA, Tech Lead      | *How* (functionally)                 | 50–500 lines | [references/HOW-TO-MAKE-FRD.md](references/HOW-TO-MAKE-FRD.md)         |
| `BACKLOG.md` | Each **feature** dir, beside its spec (not `shared/`) | Engineer, QA, Tech Lead      | *What is true now*                   | 50–500 lines | [references/HOW-TO-MAKE-BACKLOG.md](references/HOW-TO-MAKE-BACKLOG.md) |
| `README.md`  | Root workspace                    | Developer (new/existing)     | *How to use/run*                     | 50–500 lines | [references/HOW-TO-MAKE-README.md](references/HOW-TO-MAKE-README.md)   |
| `AGENTS.md`  | Root workspace                    | The agent, every session     | *How to work here safely*            | 50–500 lines | [references/HOW-TO-MAKE-AGENTS.md](references/HOW-TO-MAKE-AGENTS.md)   |
| `ARCHITECTURE.md` | Root workspace                | Engineer, Tech Lead, agent   | *Layer boundaries and allowed dependencies* | 50–500 lines | [references/HOW-TO-MAKE-ARCHITECTURE.md](references/HOW-TO-MAKE-ARCHITECTURE.md) |
| `CONTRIBUTING.md` | Root workspace                | New and existing contributor | *How to set up and ship a change*   | 50–500 lines | [references/HOW-TO-MAKE-CONTRIBUTING.md](references/HOW-TO-MAKE-CONTRIBUTING.md) |
| `DESIGN.md`       | Root workspace (optional theme) | Design, Surface Engineer, TUI dev | *Visual & interaction contract*     | 50–500 lines | [references/HOW-TO-MAKE-DESIGN.md](references/HOW-TO-MAKE-DESIGN.md)         |

**Code surface (doc comments)** — same Rules / Template / Section Contract / Verify shape:

| Language   | Focus                          | Template |
| ---------- | ------------------------------ | -------- |
| Python     | PEP 257 docstrings             | [references/HOW-TO-MAKE-PYTHON-DOC.md](references/HOW-TO-MAKE-PYTHON-DOC.md) |
| Rust       | `///` + rustdoc sections       | [references/HOW-TO-MAKE-RUST-DOC.md](references/HOW-TO-MAKE-RUST-DOC.md) |
| TypeScript | JSDoc / TSDoc                  | [references/HOW-TO-MAKE-TYPESCRIPT-DOC.md](references/HOW-TO-MAKE-TYPESCRIPT-DOC.md) |

**The Document Chain**:  
PRD → ROADMAP  → FRD → BACKLOG → README→ AGENTS

Each file answers exactly one audience's question. A claim in the wrong file is the defect this skill exists to prevent.
Doc comments on every public item are the sixth deliverable, in the language's native form.

---

## Invariants

Machine-checked rules enforced by `lint-arwaky-cli docs` (crate `doc-rules`, AES601–AES606). The full invariant table and unchecked language rules live in [`references/invariants.md`](references/invariants.md). Cite the code, not this file, when pointing at a rule.

---

## Diagnostic Tree

Ask these questions in order. The first "No" dictates your next action. The full tree is in [`references/diagnostic.md`](references/diagnostic.md):

1. **Can a stakeholder understand this project's purpose in 30 seconds?** → Add `PRD.md`.
2. **Can a tech lead see every feature, shared policy, and workspace truth in one place?** → Add `ROADMAP.md`.
3. **Can an engineer implement this from the spec alone?** → Add `FRD.md`.
4. **Can a reader tell what is actually true today for one feature, and re-run the evidence?** → Add `BACKLOG.md`.
5. **Can a developer clone, build, and run in under 10 minutes?** → Add `README.md`.
6. **Can an agent work here safely without being told twice?** → Add `AGENTS.md`.
7. **Can a new engineer tell which layer a file belongs in, and what it may import?** → Add `ARCHITECTURE.md`.
8. **Can a first-time contributor set up the project and open a passing PR?** → Add `CONTRIBUTING.md`.

---

## Repository Layout

The full layout diagram, document reference table, and code-surface templates live in [`references/layout.md`](references/layout.md):

```
project-root/
├── PRD.md          # stakeholder alignment (what/why) — 1 per project
├── ROADMAP.md      # progress overview + feature status, shared policy, cross-cutting links — 1 per project
├── README.md       # developer onboarding (how to use) — 1 per project
├── AGENTS.md       # operational guide (how the agent works here) — 1 per project
├── ARCHITECTURE.md # layer boundaries and allowed dependencies — 1 per project
├── CONTRIBUTING.md # contributor setup, contribution paths, PR gates — 1 per project
├── DESIGN.md       # visual & interaction contract (optional theme) — 1 per project
├── crates|modules|packages/
│   ├── feature-a/
│   │   ├── src/
│   │   ├── FRD.md     # engineering specs (how) — per feature crate
│   │   └── BACKLOG.md # feature real condition — beside its spec
│   ├── feature-b/
│   │   ├── src/
│   │   ├── FRD.md
│   │   └── BACKLOG.md
│   └── shared/        # kernel — NO FRD.md / BACKLOG.md (feature-doc-in-shared)
```

Same shape for Python `modules/<feature>/` and TypeScript `packages/<feature>/`. Cross-cutting rows live in the root master `ROADMAP.md` (legacy root `BACKLOG.md` still accepted), not in a feature backlog.

---

## Workflow

The full workflow is in [`references/workflow.md`](references/workflow.md):

1. Resolve the repo-root anchor first. `lint-arwaky-cli docs`
2. Analyze: List feature modules and public items. Run `lint-arwaky-cli docs <path>`. The findings are your work list.
3. Draft PRD: Write root `PRD.md` per `references/HOW-TO-MAKE-PRD.md`.
4. Draft Roadmap: Write root `ROADMAP.md` per `references/HOW-TO-MAKE-ROADMAP.md`.
5. Draft FRDs: Write `FRD.md` in each feature dir per `references/HOW-TO-MAKE-FRD.md`.
6. Draft Feature Backlogs: Write one `BACKLOG.md` per feature, beside its spec, per `references/HOW-TO-MAKE-BACKLOG.md`.
7. Draft README: Write root `README.md` per `references/HOW-TO-MAKE-README.md`.
8. Draft AGENTS: Write root `AGENTS.md` per `references/HOW-TO-MAKE-AGENTS.md`.
9. Draft ARCHITECTURE: Write root `ARCHITECTURE.md` per `references/HOW-TO-MAKE-ARCHITECTURE.md` — required when the workspace has more than one layer beyond shared.
10. Draft CONTRIBUTING: Write root `CONTRIBUTING.md` per `references/HOW-TO-MAKE-CONTRIBUTING.md` — required when the repo accepts external contributions.
11. Draft DESIGN: Write root `DESIGN.md` per `references/HOW-TO-MAKE-DESIGN.md` — optional; create it when the surface exposes visual tokens.
12. Document Code: Add doc comments to all public items per language HOW-TO; then add type annotations to all signatures.
13. Verify: Run `lint-arwaky-cli docs <path>`. Then each touched reference's `Verify` block.


---

## Verification

The full verification gate is in [`references/verification.md`](references/verification.md).

### Machine Checks

```bash
lint-arwaky-cli docs .                 # invariant audit of every document (strict; every finding gates)
```

A pass means no claim sits in the wrong file, no pointer is broken, and no `Done` row is unevidenced.

### Human Checks

A machine pass does not mean the documents are good. Falsifiable goals, honest exclusions, and a 10-minute Quick Start still need a reader.

### Doc Comment Conventions

Per-language rules, templates, section contracts, and Verify blocks:

- Python (PEP 257): `references/HOW-TO-MAKE-PYTHON-DOC.md`
- Rust (`///` / rustdoc): `references/HOW-TO-MAKE-RUST-DOC.md`
- TypeScript (JSDoc / TSDoc): `references/HOW-TO-MAKE-TYPESCRIPT-DOC.md`


---

## Pre-flight Checklist

- [ ] `lint-arwaky-cli docs <path>` exits 0.
- [ ] Every required document exists in the correct directory.
- [ ] Every `Done` backlog row cites a re-run command, a commit hash, and its exclusions.
- [ ] Documents serve their exact audience (no cross-contamination).
- [ ] Public code items carry doc comments, and surface checks (`cargo doc`, `tsc`, `import`) are clean.
- [ ] Every touched reference file had its specific `Verify` block executed.


---

## Related Skills

- `aes-lint-arwaky`
- `aes-taxonomy`
- `aes-contract`
