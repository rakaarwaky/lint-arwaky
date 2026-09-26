---
name: aes-docs
description: Adds docstrings, doc comments, JSDoc, types and PRD/ROADMAP/FRD/README/BACKLOG/AGENTS docs. Use when documenting Python, Rust, TS modules, or splitting spec from status backlog.
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
    - cleanup-consolidate
    - fix-bypass
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
---
# aes-docs

> **Purpose**: Route every claim to the correct document, and ensure every public code item is documented.
> **Audience**: The AI agent executing documentation tasks.
> **Scope**: Python, Rust, and TypeScript modules; PRD, ROADMAP, FRD, README, BACKLOG, and AGENTS files.

The **aggregate** defines which document exists, where it lives, who reads it, and which claim belongs where.
Templates, section contracts, exemplars, and per-document craft rules live in [`references/`](references/).

| Document     | Location                          | Audience                     | Focus                                | Length       | Template                                                               |
| ------------ | --------------------------------- | ---------------------------- | ------------------------------------ | ------------ | ---------------------------------------------------------------------- |
| `PRD.md`     | Root workspace                    | Stakeholder, PM, Design, Eng | *What* &amp; *Why*                   | 50–500 lines | [references/HOW-TO-MAKE-PRD.md](references/HOW-TO-MAKE-PRD.md)         |
| `ROADMAP.md` | Root workspace (exactly one)      | Tech Lead, PM, Engineers     | *Index, policy, workspace condition* | 50–500 lines | [references/HOW-TO-MAKE-ROADMAP.md](references/HOW-TO-MAKE-ROADMAP.md) |
| `FRD.md`     | Each **feature** module/crate/pkg (not `shared/`) | Engineer, QA, Tech Lead      | *How* (functionally)                 | 50–500 lines | [references/HOW-TO-MAKE-FRD.md](references/HOW-TO-MAKE-FRD.md)         |
| `BACKLOG.md` | Each **feature** dir, beside its spec (not `shared/`) | Engineer, QA, Tech Lead      | *What is true now*                   | 50–500 lines | [references/HOW-TO-MAKE-BACKLOG.md](references/HOW-TO-MAKE-BACKLOG.md) |
| `README.md`  | Root workspace                    | Developer (new/existing)     | *How to use/run*                     | 50–500 lines | [references/HOW-TO-MAKE-README.md](references/HOW-TO-MAKE-README.md)   |
| `AGENTS.md`  | Root workspace                    | The agent, every session     | *How to work here safely*            | 50–500 lines | [references/HOW-TO-MAKE-AGENTS.md](references/HOW-TO-MAKE-AGENTS.md)   |
| `ARCHITECTURE.md` | Root workspace                | Engineer, Tech Lead, agent   | *Layer boundaries and allowed dependencies* | 50–500 lines | [references/HOW-TO-MAKE-ARCHITECTURE.md](references/HOW-TO-MAKE-ARCHITECTURE.md) |
| `CONTRIBUTING.md` | Root workspace                | New and existing contributor | *How to set up and ship a change*   | 50–500 lines | [references/HOW-TO-MAKE-CONTRIBUTING.md](references/HOW-TO-MAKE-CONTRIBUTING.md) |

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

Every rule is machine-checked by `aa check docs` (capability: `modules/check/src/capabilities_check_docs.py`, shared engine in `modules/shared/src/utility_doc_pack.py`).
A rule cannot drift from the gate. Cite the code, not this file, when pointing at a rule.
Each document's required section set is cross-checked against its reference's contract table, so a
row that stops being enforced is a test failure rather than a silent edit.

| Code                                                                     | Rule                                                                                                                                                                                  |
| ------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `status-in-spec` / `spec-source-path`                                       | Spec and status never share a file. Specs promise; backlogs report. Specs are stateless — no `.py`/`.rs`/`.ts` file names (HOW-TO Rule 9). |
| `spec-without-backlog` / `backlog-without-spec` / `feature-doc-in-shared` | An **FRD** and its backlog are a pair in the same **feature** directory (root PRD pairs with root ROADMAP, not a sibling BACKLOG). Kernel folders (`modules/shared`) are not features: either file under `shared/` fails with `feature-doc-in-shared` (HOW-TO-MAKE-FRD § Scope). |
| `no-master-backlog` / `undefined-state-vocab` / `master-section-missing` | One root master owns the `State`/`Health` vocabulary, the status policy, the roll-up, in-flight branches and risk (`ROADMAP.md`; legacy root `BACKLOG.md` accepted during migration). |
| `state-vocab-restated`                                                   | Definitions live once. Feature files cite them, never repeat them.                                                                                                                    |
| `done-without-evidence` / `unknown-state`                                | Every backlog claim is re-runnable: command + counts + commit hash, and what it excludes.                                                                                             |
| `duplicate-fr-id` / `orphan-fr-ref`                                      | Requirement IDs are unique, stable, and the only thing a backlog row may cite.                                                                                                        |
| `fr-id-format` / `fr-fields-missing`                                     | FRD Rule 1–2: `FR-<FEATURENAME>-NNN: <imperative name>`; each FR states Description, Input, Output, Business Rules, Edge Cases, Error Handling.                                         |
| `api-contract-shape` / `integration-shape` / `nfr-shape`                 | FRD Rule 3/5 + template: API Contract carries both `Protocol API` (leaf capability methods) and `Aggregate API` (orchestrator / capability aggregate exports) tables with exact columns and ≥1 data row each; Integration Points and Non-functional keep their exact columns and at least one data row. |
| `reference-crosslink` / `section-order`                                  | FRD Rule 7 + template: Reference links PRD and BACKLOG; sections follow HOW-TO-MAKE-FRD order.                                                                                         |
| `scenario-empty` / `assumption-empty` / `glossary-empty`                 | FRD Rule 4/6 + template: Test Scenarios, Assumptions, and Glossary each carry bullet items — not empty placeholders.                                                                     |
| `scenario-without-evidence` / `scenario-evidence-count`                  | Each test scenario in a spec has one evidence row: Automated / Proxy / Manual / Gap.                                                                                                  |
| `backlog-columns` / `backlog-row-width`                                  | The Backlog table keeps its nine columns.                                                                                                                                             |
| `*-section-missing`                                                      | Each document carries the sections its audience needs. Section contracts are in the refs.                                                                                             |
| `dead-link` / `root-relative-link`                                       | Pointers resolve from the file that writes them, not only from the repo or skill root.                                                                                                |
| `unreferenced-file`                                                      | Every file under a skill's `references/`, `scripts/`, `assets/` is surfaced by SKILL.md.                                                                                              |
| `absolute-path` / `secret-in-docs`                                       | No machine-specific path and no credential literal in any document.                                                                                                                   |
| `ci-command-drift`                                                       | AGENTS.md commands match CI exactly or are labelled advisory.                                                                                                                        |
| `doc-length` / `doc-thin`                                                | Each document stays inside the size its audience can read.                                                                                                                            |

### Unchecked Invariants (Language Rules)

The checker cannot parse code intent. Enforce these manually (contracts live in the language refs above):

- **Doc comments explain *what* and *why*, never *how*** (the code shows how).
- **Python**: Public classes and functions need docstrings (PEP 257) — [references/HOW-TO-MAKE-PYTHON-DOC.md](references/HOW-TO-MAKE-PYTHON-DOC.md).
- **Rust**: Public items need `///` (plain `//` is invisible to `cargo doc`). Examples must compile — [references/HOW-TO-MAKE-RUST-DOC.md](references/HOW-TO-MAKE-RUST-DOC.md).
- **TypeScript**: Public items need JSDoc — [references/HOW-TO-MAKE-TYPESCRIPT-DOC.md](references/HOW-TO-MAKE-TYPESCRIPT-DOC.md).

---

## Diagnostic Tree

Ask these questions in order. The first "No" dictates your next action.

1. **Can a stakeholder understand this project's purpose in 30 seconds?**
   - *No* → Add `PRD.md` (what/why).
2. **Can a tech lead see every feature, shared policy, and workspace truth in one place?**
   - *No* → Add `ROADMAP.md` (index, definitions, status policy, roll-up, risk).
3. **Can an engineer implement this from the spec alone?**
   - *No* → Add `FRD.md` (how).
4. **Can a reader tell what is actually true today for one feature, and re-run the evidence?**
   - *No* → Add `BACKLOG.md` beside that feature's spec.
5. **Can a developer clone, build, and run in under 10 minutes?**
   - *No* → Add `README.md` (how to use).
6. **Can an agent work here safely without being told twice?**
   - *No* → Add `AGENTS.md` (how to work here).
7. **Can a new engineer tell which layer a file belongs in, and what it may import?**
   - *No* → Add `ARCHITECTURE.md` (layer boundaries).
8. **Can a first-time contributor set up the project and open a passing PR?**
   - *No* → Add `CONTRIBUTING.md` (how to ship a change).

---

## Repository Layout

```text
project-root/
├── PRD.md          # stakeholder alignment (what/why) — 1 per project
├── ROADMAP.md      # feature index, shared policy, workspace condition — 1 per project
├── README.md       # developer onboarding (how to use) — 1 per project
├── AGENTS.md       # operational guide (how the agent works here) — 1 per project
├── ARCHITECTURE.md # layer boundaries and allowed dependencies — 1 per project
├── CONTRIBUTING.md # contributor setup, contribution paths, PR gates — 1 per project
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

```text

Same shape for Python `modules/<feature>/` and TypeScript `packages/<feature>/`.
Cross-cutting rows live in the root master `ROADMAP.md` (legacy root `BACKLOG.md` still accepted), not in a feature backlog.

---

## Workflow

1. **Resolve the repo-root anchor first.** `aa check docs` 
2. **Analyze**: List feature modules and public items. Run `aa check docs <path>`. The findings are your work list.
3. **Draft PRD**: Write root `PRD.md` per [references/HOW-TO-MAKE-PRD.md](references/HOW-TO-MAKE-PRD.md).
4. **Draft Roadmap**: Write root `ROADMAP.md` per [references/HOW-TO-MAKE-ROADMAP.md](references/HOW-TO-MAKE-ROADMAP.md)
5. **Draft FRDs**: Write `FRD.md` in each feature dir per [references/HOW-TO-MAKE-FRD.md](references/HOW-TO-MAKE-FRD.md).
6. **Draft Feature Backlogs**: Write one `BACKLOG.md` per feature, beside its spec, per [references/HOW-TO-MAKE-BACKLOG.md](references/HOW-TO-MAKE-BACKLOG.md).
7. **Draft README**: Write root `README.md` per [references/HOW-TO-MAKE-README.md](references/HOW-TO-MAKE-README.md).
8. **Draft AGENTS**: Write root `AGENTS.md` per [references/HOW-TO-MAKE-AGENTS.md](references/HOW-TO-MAKE-AGENTS.md).
9. **Draft ARCHITECTURE**: Write root `ARCHITECTURE.md` per [references/HOW-TO-MAKE-ARCHITECTURE.md](references/HOW-TO-MAKE-ARCHITECTURE.md) — required when the workspace has more than one layer beyond shared.
10. **Draft CONTRIBUTING**: Write root `CONTRIBUTING.md` per [references/HOW-TO-MAKE-CONTRIBUTING.md](references/HOW-TO-MAKE-CONTRIBUTING.md) — required when the repo accepts external contributions. One section per contribution path the project actually supports; delete the rest.
11. **Document Code**: Add doc comments to all public items per language —
   [references/HOW-TO-MAKE-PYTHON-DOC.md](references/HOW-TO-MAKE-PYTHON-DOC.md),
   [references/HOW-TO-MAKE-RUST-DOC.md](references/HOW-TO-MAKE-RUST-DOC.md),
   [references/HOW-TO-MAKE-TYPESCRIPT-DOC.md](references/HOW-TO-MAKE-TYPESCRIPT-DOC.md) —
   then add type annotations to all signatures.
12. **Verify**: Run `aa check docs <path>`. Then each touched reference's `Verify` block (including the language doc ref).

---

## Verification

### Machine Checks

```bash
aa check docs .                 # invariant audit of every document (strict; every finding gates)
aa check docs . --include-subtrees   # also audit vendor/ and internal/ submodules

```text

A pass means no claim sits in the wrong file, no pointer is broken, and no `Done` row is unevidenced.

### Human Checks

A machine pass does not mean the documents are good. Falsifiable goals, honest exclusions, and a 10-minute Quick Start still need a reader.

---

### Doc Comment Conventions

Per-language rules, templates, section contracts, and Verify blocks:

- Python (PEP 257): [references/HOW-TO-MAKE-PYTHON-DOC.md](references/HOW-TO-MAKE-PYTHON-DOC.md)
- Rust (`///` / rustdoc): [references/HOW-TO-MAKE-RUST-DOC.md](references/HOW-TO-MAKE-RUST-DOC.md)
- TypeScript (JSDoc / TSDoc): [references/HOW-TO-MAKE-TYPESCRIPT-DOC.md](references/HOW-TO-MAKE-TYPESCRIPT-DOC.md)

---

## Pre-flight Checklist

- [ ] `aa check docs <path>` exits 0.
- [ ] Every required document exists in the correct directory.
- [ ] Every `Done` backlog row cites a re-run command, a commit hash, and its exclusions.
- [ ] Documents serve their exact audience (no cross-contamination).
- [ ] Public code items carry doc comments, and surface checks (`cargo doc`, `tsc`, `import`) are clean.
- [ ] Every touched reference file had its specific `Verify` block executed.

---

## Common Mistakes (Anti-Patterns)

The invariant codes above cover the machine-checkable ones. These need a reader:

**Structural**

- **One document for all audiences**: Split by audience. Each file answers one question.
- **FRD at the project root**: It belongs with the feature code, beside its backlog.
- **Feature backlog carrying workspace rows or restating root policy**: Cross-cutting rows and State/Health definitions live once in `ROADMAP.md` (root master).
- **PRD carrying SQL schemas or API detail**: The PRD audience cannot read them. Move to FRD.

**Cadence and code surface**

- **Documents "write &amp; forget"**: Re-run `aa check docs` each sprint. Drift is silent.
- **`//` instead of `///` in Rust**: Plain comments are invisible to the doc generator.
- **Missing module docstrings or undocumented parameters**: The generated API surface stays incomplete.

**Checker false-freights to dodge when authoring specs** (these bite at draft time, before you run the gate):

- **The word "implemented" in a spec file.** `status-in-spec` matches `\b(impl|un)plemented\b` case-insensitively across the *whole* FRD/PRD — so the reference template's `| As Implemented / As Intended |` column and any `implemented` cell header/cell are auto-flagged. Name the column `impl / intended` and use `impl` for the cell. (Same class: `shipped`/`released in v…`, checkbox items, status markers checkmark / cross status markers, progress `%` all trip it.)
- **Scenario-evidence rows without a table header.** `check_scenarios` counts evidence via a markdown-table parser that needs a `| Scenario | … |` header + `|---|` separator line; a headerless block of `| … |` rows parses as **0 rows** and reports `0 evidence row(s)` even when the rows are present. Always emit the header row; keep exactly one row per spec scenario, in spec order.
- **Scenario bullets containing `<`.** The scenario counter skips any spec bullet whose text contains `<` (placeholder convention), so that scenario needs no evidence row — don't write one, or the count is off by one. Rename `<placeholder>` prose to avoid the silent skip.

---

## Related Skills

- `aes-lint-arwaky`
- `cleanup-consolidate`
- `fix-bypass`
