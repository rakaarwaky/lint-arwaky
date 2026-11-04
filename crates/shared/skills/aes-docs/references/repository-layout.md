# Repository layout

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
│   ├── surface-a/
│   │   ├── src/
│   │   ├── DESIGN.md  # visual & interaction contract — per surface folder
│   │   └── BACKLOG.md
│   └── shared/
│       ├── src/
│       ├── DATA.md
│       └── BACKLOG.md
```

Same shape for Python `modules/<feature>/` and TypeScript `packages/<feature>/`.
