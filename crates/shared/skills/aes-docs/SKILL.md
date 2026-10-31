---
name: aes-docs
description: "Adds PRD/ROADMAP/FRD/README/BACKLOG/DESIGN/AGENTS/DATA docs for Python, Rust, and TypeScript modules. 
when-to-use: "Use when the task is to create or improve a PRD, ROADMAP, FRD, BACKLOG, README, AGENTS, DESIGN, or DATA document, or to split spec from status."
argument-hint: "--docpath <path> [--improve | --design | --make]"
metadata:
  tags:
    - docs
    - prd
    - roadmap
    - frd
    - readme
    - backlog
    - agents-md
  related_skills:
    - aes-doc-comments
    - aes-lint-arwaky
---
# aes-docs

The aggregated efines which document exists, where it lives, and  to improve make and design it.

| Document          | Location                                   | Focus                                       | workflow                                                                                           |
| ----------------- | ------------------------------------------ | ------------------------------------------- | -------------------------------------------------------------------------------------------------- |
| `PRD.md`          | Root workspace                             | *What* &amp; *Why*                          | [references/prd/workflow.md.](references/prd/workflow.md)                   |
| `ROADMAP.md`      | Root workspace                             | How the priority progrees                   | [references/roadmap/workflow.md](references/roadmap/workflow.md)           |
| `FRD.md`          | Each Feature Folder                        | *How* functionally                          | [references/frd/workflow.md](references/frd/workflow.md)                   |
| `BACKLOG.md`      | Each feature **|** surface | shared folder | *What is true now*                          | [references/backlog/workflow.md](references/backlog/workflow.md)           |
| `README.md`       | Root workspace                             | *How to use/run*                            | [references/readme/workflow.md](references/readme/workflow.md)             |
| `AGENTS.md`       | Root workspace                             | *How to work here safely*                   | [references/agents/workflow.md](references/agents/workflow.md)             |
| `ARCHITECTURE.md` | Root workspace                             | *Layer boundaries and allowed dependencies* | [references/architecture/workflow.md](references/architecture/workflow.md) |
| `CONTRIBUTING.md` | Root workspace                             | *How to set up and ship a change*           | [references/contributing/workflow.md](references/contributing/workflow.md) |
| `DESIGN.md`       | Each surface folder                        | *Visual &amp; interaction contract*         | [references/design/workflow.md](references/design/workflow.md)             |
| `DATA.md`         | Shared folder                              | *What the shared layer manages*             | [references/data/workflow.md](references/data/workflow.md)                 |




---

## Diagnostic Tree

Ask these questions in order. The first "No" dictates your next action.

1. **Can a stakeholder understand this project's purpose in 30 seconds?**
   - *No* → Add `PRD.md`
2. **Can a tech lead see every feature, shared policy, and workspace truth in one place?**
   - *No* → Add `ROADMAP.md`
3. **Can an engineer implement this from the spec alone?**
   - *No* → Add `FRD.md` 
4. **Can a reader tell what is actually true today for one feature, and re-run the evidence?**
   - *No* → Add `BACKLOG.md` 
5. **Can a developer clone, build, and run in under 10 minutes?**
   - *No* → Add `README.md`
6. **Can an agent work here safely without being told twice?**
   - *No* → Add `AGENTS.md` 
7. **Can a new engineer tell which layer a file belongs in, and what it may import?**
   - *No* → Add `ARCHITECTURE.md`
8. **Can a first-time contributor set up the project and open a passing PR?**
   - *No* → Add `CONTRIBUTING.md` 

---

## Repository Layout

```text
project-root/
├── PRD.md          # stakeholder alignment (what/why) — 1 per project
├── ROADMAP.md      # progress overview + feature status, shared policy, cross-cutting links — 1 per project
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
│   ├── surface-a/
│   │   ├── src/
│   │   ├── DESIGN.md
│   │   └── BACKLOG.md
│   ├── surface-b/
│   │   ├── src/
│   │   ├── FRD.md
│   │   └── BACKLOG.md
│   └── shared/    
│   │   ├── src/
│   │   ├── Data.md
│   │   └── BACKLOG.md   
Same shape for Python `modules/<feature>/` and TypeScript `packages/<feature>/`.
```


---


---
