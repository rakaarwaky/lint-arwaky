# HOW TO MAKE ROADMAP.md

> **Purpose**: Index features, own the state vocabulary, and record
> workspace-level condition.
>
> **Audience**: Tech Lead, PM, Engineers.
>
> **Boundary**: The single source of truth for workspace state,
> cross-cutting risks, and feature tracking.
>
> **Scope**: Exactly one ROADMAP.md at the project root.
>
> **Location**: Project root.
>
> **Length**: 50–500 lines

---

## Rules

1. **Single source of truth.** State vocabulary lives here and nowhere
   else. Feature backlogs cite it; they never redefine it.
2. **Index every feature.** Every feature directory must appear in the
   Roadmap Status as a `PR-<PROJECT-NAME-ID>-NNN` entry, linking to its
   FRD.md and BACKLOG.md (`no-master-roadmap`).
3. **Cross-cutting rows only.** The workspace table is for items that
   span multiple features or infrastructure.
4. **ID scopes are explicit.** The root file uses a workspace prefix
   (`WS-01`); feature rows keep their `<SCOPE>-` prefix and stay in
   their own BACKLOG.md.

---

## Workflow

1. **Create file** → `ROADMAP.md` at repo root.
2. **Section: Progress Overview** — one paragraph of progress summary.
3. **Section: Roadmap Status** — `PR-<PROJECT-NAME-ID>-NNN` items,
   each carrying Priority / State / Dependencies / Updated.
4. **Section: Reference** — link to PRD.md and ARCHITECTURE.md.
5. **Verify** → `lint-arwaky-cli docs` passes; every required section is present.

## Template

Copy, fill, delete nothing.

```markdown
# ROADMAP — <workspace name>

## Progress Overview

<One paragraph of progress summary.>

## Roadmap Status 

### PR-<PROJECT-NAME-ID>-001: <Short imperative name>

| Aspect       | Detail                                      |
|--------------|---------------------------------------------|
| Priority     | P0 / P1 / P2                                |
| State        | Ready / In Progress / In Review / Done      |
| Dependencies | <DATA-NAME-ID> / <FEATURE-NAME-ID> / <DESIGN-NAME-ID> |
| Updated      | <YYYY-MM-DD>                                |

### PR-<PROJECT-NAME-ID>-002: <Short imperative name>
### PR-<PROJECT-NAME-ID>-003: <Short imperative name>
### PR-<PROJECT-NAME-ID>-004: <Short imperative name>

## Reference

| Artifact     | Purpose                                          | Link |
|--------------|--------------------------------------------------|------|
| PRD          | <Business goals, user stories, and scope>        | [PRD.md](PRD.md) |
| Architecture | <System boundaries, tech stack, and infra>       | [ARCHITECTURE.md](ARCHITECTURE.md) | 
```

---

## Section Contract

Every section is required unless marked optional. Each exists for one
reason.

| Section          | Why it belongs here                                                     |
| ---------------- | ----------------------------------------------------------------------- |
| Progress Overview| Workspace progress summary in one paragraph.                            |
| Roadmap Status   | `PR-<PROJECT-NAME-ID>-NNN` items with Priority / State / Dependencies / Updated. |
| Reference        | Connects the roadmap to the problem spec and the layer contract.       |

---

## Verify

```bash
lint-arwaky-cli docs .
# Checks: master root presence, feature pairing, unknown states, missing sections.
```
