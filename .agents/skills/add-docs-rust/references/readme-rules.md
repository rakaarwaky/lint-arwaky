# README Rules

## Purpose

README describes **HOW TO USE/RUN** for developers (new and existing).

## Location

- Lives in **root workspace only** (`<workspace-root>/README.md`)
- One README per project

## Audience

- New developers (onboarding)
- Existing developers (reference)
- Contributors

## Focus

- Quick Start (clone → build → run in < 10 minutes)
- Architecture (high-level diagram)
- Project Structure (show FRD.md locations)
- Available Commands
- Configuration (env vars, config files)
- Testing
- Contributing (branching, PR conventions)
- License

## Rules

1. **Quick Start first** — developer should be able to run in < 10 minutes
2. **Keep concise** — 1-2 pages max
3. **Link to other docs** — don't copy PRD/FRD content
4. **Update when setup changes** — keep current
5. **Show project structure** — where are FRD.md files?
6. **Include badges** — build status, coverage, version
7. **One README per repo** — for monorepo, root + per-crate READMEs

## Anti-Patterns

- ❌ README = essay 10 pages → keep concise, link to other docs
- ❌ README without Quick Start → add clone → build → run instructions
- ❌ README without project structure → add directory tree
- ❌ README without configuration section → add env vars, config files
- ❌ README "write & forget" → update each sprint/release
- ❌ README copying PRD/FRD content → link instead
