# aes-docs — Repository Layout

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

Same shape for Python `modules/<feature>/` and TypeScript `packages/<feature>/`.
Cross-cutting rows live in the root master `ROADMAP.md` (legacy root `BACKLOG.md` still accepted), not in a feature backlog.

## Document reference

| Document     | Location                          | Audience                     | Focus                                | Length       | Template                                                               |
|--------------|-----------------------------------|------------------------------|--------------------------------------|--------------|------------------------------------------------------------------------|
| `PRD.md`     | Root workspace                    | Stakeholder, PM, Design, Eng | *What* &amp; *Why*                   | 50–500 lines | `references/HOW-TO-MAKE-PRD.md` |
| `ROADMAP.md` | Root workspace (exactly one)      | Tech Lead, PM, Engineers     | *Progress, status, cross-cutting links* | 50–500 lines | `references/HOW-TO-MAKE-ROADMAP.md` |
| `FRD.md`     | Each **feature** module/crate/pkg (not `shared/`) | Engineer, QA, Tech Lead | *How* (functionally)                 | 50–500 lines | `references/HOW-TO-MAKE-FRD.md` |
| `BACKLOG.md` | Each **feature** dir, beside its spec (not `shared/`) | Engineer, QA, Tech Lead | *What is true now*                   | 50–500 lines | `references/HOW-TO-MAKE-BACKLOG.md` |
| `README.md`  | Root workspace                    | Developer (new/existing)     | *How to use/run*                     | 50–500 lines | `references/HOW-TO-MAKE-README.md` |
| `AGENTS.md`  | Root workspace                    | The agent, every session     | *How to work here safely*            | 50–500 lines | `references/HOW-TO-MAKE-AGENTS.md` |
| `ARCHITECTURE.md` | Root workspace                | Engineer, Tech Lead, agent   | *Layer boundaries and allowed dependencies* | 50–500 lines | `references/HOW-TO-MAKE-ARCHITECTURE.md` |
| `CONTRIBUTING.md` | Root workspace                | New and existing contributor | *How to set up and ship a change*   | 50–500 lines | `references/HOW-TO-MAKE-CONTRIBUTING.md` |
| `DESIGN.md`       | Root workspace (optional theme) | Design, Surface Engineer, TUI dev | *Visual & interaction contract*     | 50–500 lines | `references/HOW-TO-MAKE-DESIGN.md` |

**Code surface (doc comments):**

| Language   | Focus                          | Template |
|------------|--------------------------------|----------|
| Python     | PEP 257 docstrings             | `references/HOW-TO-MAKE-PYTHON-DOC.md` |
| Rust       | `///` + rustdoc sections       | `references/HOW-TO-MAKE-RUST-DOC.md` |
| TypeScript | JSDoc / TSDoc                  | `references/HOW-TO-MAKE-TYPESCRIPT-DOC.md` |

**The Document Chain**: PRD → ROADMAP → FRD → BACKLOG → README → AGENTS

Each file answers exactly one audience's question. A claim in the wrong file is the defect this skill exists to prevent. Doc comments on every public item are the sixth deliverable, in the language's native form.
