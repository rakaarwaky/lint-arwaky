---
name: aes-docs
description: "Adds PRD/ROADMAP/FRD/README/BACKLOG/DESIGN/AGENTS/DATA docs for Python, Rust, and TypeScript modules. Use when creating or improving a PRD, ROADMAP, FRD, BACKLOG, README, AGENTS, DESIGN, or DATA document, or to split spec from status."
when-to-use: "create, improve, or audit a project document (PRD, ROADMAP, FRD, BACKLOG, README, AGENTS, ARCHITECTURE, CONTRIBUTING, DESIGN, DATA); split spec from status; verify docs with lint-arwaky-cli docs."
argument-hint: '<improve | make | design> [document type] [path]'

---

# aes-docs

Scaffold and audit the ten AES project documents — PRD, ROADMAP, FRD, BACKLOG, README, AGENTS, ARCHITECTURE, CONTRIBUTING, DESIGN, DATA — each with its own rules, exemplar, template, and workflow file under `references/`. The audience is the agent creating or improving a project document; the **document type** decides which reference set loads and which gate applies. A missing document is diagnosed by its reader's question — the reader who could not answer the question is the document to write. Invariants read by more than one step live in Global Rules below.

## Global Rules

- Split spec from status: `PRD` / `FRD` / `DESIGN` state intent, `BACKLOG` and `ROADMAP` state what is true now — status words never appear in spec documents (status-in-spec).
- One document per reader: root documents (`PRD`, `ROADMAP`, `README`, `AGENTS`, `ARCHITECTURE`, `CONTRIBUTING`) live at the workspace root; `FRD` / `BACKLOG` live per feature folder; `DESIGN` per surface folder; `DATA` per shared folder.
- Feature folders carry `FRD.md` and `BACKLOG.md`; a folder with neither capabilities nor an agent is not a feature and owes no document pair (AES702).
- Surface folders carry `DESIGN.md` with the six sections — Kind, Entry Points, Request Shape, States, Error States, Invariants, Change Checklist; every row names this folder's real files (AES703).
- Improve vs make: audit mode checks each document against its rules file and exemplar; make mode scaffolds from the template file in the same reference set.
- Verify: `lint-arwaky-cli docs <workspace>` passes — the document-invariant gate resolves every link and reports any pointer into nothing.
- One fact, one home: each `references/<doc>/` set owns that document's shape — rules, exemplar, template, workflow. Read it, do not restate here. Cite the gate, not this file, when pointing at a rule. Each bullet states a fact once; references point at the bullet.

## Routing

`workflow.md` is the entry point for each doc type; the other files in the set load on demand.

| Document | Location | Focus | Files |
| --- | --- | --- | --- |
| `PRD.md` | Root workspace | *What* & *Why* | [rules](references/prd/rules.md) · [exemplar](references/prd/exemplar.md) · [template](references/prd/template.md) · [workflow](references/prd/workflow.md) |
| `ROADMAP.md` | Root workspace | How the priority progresses | [rules](references/roadmap/rules.md) · [exemplar](references/roadmap/exemplar.md) · [template](references/roadmap/template.md) · [workflow](references/roadmap/workflow.md) |
| `FRD.md` | Each feature folder | *How* functionally | [rules](references/frd/rules.md) · [exemplar](references/frd/exemplar.md) · [template](references/frd/template.md) · [workflow](references/frd/workflow.md) |
| `BACKLOG.md` | Each feature, surface, or shared folder | *What is true now* | [rules](references/backlog/rules.md) · [feature-exemplar](references/backlog/feature-exemplar.md) · [surface-exemplar](references/backlog/surface-exemplar.md) · [shared-exemplar](references/backlog/shared-exemplar.md) · [template](references/backlog/template.md) · [workflow](references/backlog/workflow.md) |
| `README.md` | Root workspace | *How to use/run* | [rules](references/readme/rules.md) · [exemplar](references/readme/exemplar.md) · [template](references/readme/template.md) · [workflow](references/readme/workflow.md) |
| `AGENTS.md` | Root workspace | *How to work here safely* | [rules](references/agents/rules.md) · [exemplar](references/agents/exemplar.md) · [template](references/agents/template.md) · [workflow](references/agents/workflow.md) |
| `ARCHITECTURE.md` | Root workspace | *Layer boundaries and allowed dependencies* | [rules](references/architecture/rules.md) · [exemplar](references/architecture/exemplar.md) · [template](references/architecture/template.md) · [workflow](references/architecture/workflow.md) |
| `CONTRIBUTING.md` | Root workspace | *How to set up and ship a change* | [rules](references/contributing/rules.md) · [exemplar](references/contributing/exemplar.md) · [template](references/contributing/template.md) · [workflow](references/contributing/workflow.md) |
| `DESIGN.md` | Each surface folder | *Visual & interaction contract* | [rules](references/design/rules.md) · [exemplar](references/design/exemplar.md) · [template](references/design/template.md) · [workflow](references/design/workflow.md) |
| `DATA.md` | Shared folder | *What the shared layer manages* | [rules](references/data/rules.md) · [exemplar](references/data/exemplar.md) · [template](references/data/template.md) · [workflow](references/data/workflow.md) |
