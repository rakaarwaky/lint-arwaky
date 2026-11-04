---
name: aes-docs
description: "Adds PRD/ROADMAP/FRD/README/BACKLOG/DESIGN/AGENTS/DATA docs for Python, Rust, and TypeScript modules. Use when creating or improving a PRD, ROADMAP, FRD, BACKLOG, README, AGENTS, DESIGN, or DATA document, or to split spec from status."
when-to-use: "Use when creating, improving, or auditing a project document (PRD, ROADMAP, FRD, BACKLOG, README, AGENTS, ARCHITECTURE, CONTRIBUTING, DESIGN, DATA), or to split spec from status."
argument-hint: "<path-to-project-dir> [doc-type] [--improve | --make | --design]"
---

# aes-docs

Scaffold and audit the ten AES project documents. Each document has its own rules, exemplar, template, and workflow under `references/<type>/`. When `--improve` is passed, each document is audited against its rules and exemplar; when `--make` or `--design` is passed, the template is scaffolded from scratch.

## Global rules

- **Split spec from status**: `PRD` / `FRD` / `DESIGN` state intent; `BACKLOG` and `ROADMAP` state what is true now — status words never appear in spec documents (`status-in-spec`).
- **One document per reader**: root documents (`PRD`, `ROADMAP`, `README`, `AGENTS`, `ARCHITECTURE`, `CONTRIBUTING`) live at the workspace root; `FRD` / `BACKLOG` live per feature folder; `DESIGN` per surface folder; `DATA` per shared folder.
- **Feature folders carry `FRD.md` and `BACKLOG.md`**; a folder with neither capabilities nor an agent is not a feature and owes no document pair (AES702).
- **Surface folders carry `DESIGN.md`** with the six sections — Kind, Entry Points, Request Shape, States, Error States, Invariants, Change Checklist; every row names this folder's real files (AES703).
- **Improve vs make**: audit mode checks each document against its rules file and exemplar; make mode scaffolds from the template file in the same reference set.
- **Verify**: `lint-arwaky-cli docs <workspace>` passes — the document-invariant gate resolves every link and reports any pointer into nothing.
- **Related**: aes-doc-comments (code-surface doc comments), aes-lint-arwaky (gate) — each carries its own HOW-TO.

## Routing

| Section | Reference | Purpose |
| ------- | --------- | ------- |
| `PRD.md` | [references/prd/workflow.md](references/prd/workflow.md) | *What* & *Why* — stakeholder alignment |
| `ROADMAP.md` | [references/roadmap/workflow.md](references/roadmap/workflow.md) | How the priority progresses |
| `FRD.md` | [references/frd/workflow.md](references/frd/workflow.md) | *How* functionally — per feature folder |
| `BACKLOG.md` | [references/backlog/workflow.md](references/backlog/workflow.md) | *What is true now* — per feature/surface/shared folder |
| `README.md` | [references/readme/workflow.md](references/readme/workflow.md) | *How to use/run* — developer onboarding |
| `AGENTS.md` | [references/agents/workflow.md](references/agents/workflow.md) | *How to work here safely* |
| `ARCHITECTURE.md` | [references/architecture/workflow.md](references/architecture/workflow.md) | *Layer boundaries and allowed dependencies* |
| `CONTRIBUTING.md` | [references/contributing/workflow.md](references/contributing/workflow.md) | *How to set up and ship a change* |
| `DESIGN.md` | [references/design/workflow.md](references/design/workflow.md) | *Visual & interaction contract* — per surface folder |
| `DATA.md` | [references/data/workflow.md](references/data/workflow.md) | *What the shared layer manages* — per shared folder |
| Diagnostic tree | [references/diagnostic-tree.md](references/diagnostic-tree.md) | Which document to add next, from the reader's question |
| Repository layout | [references/repository-layout.md](references/repository-layout.md) | Where each document lives in the tree |
