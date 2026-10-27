# HOW TO MAKE BACKLOG.md

> **Purpose**: Record what is true right now for one feature or shared folder,
> and on what evidence.
>
> **Audience**: Engineers, QA, Tech Lead.
>
> **Scope**: One BACKLOG.md per feature folder and per shared folder.
>
> **Location**: Inside the feature's directory or inside `shared/`.
>
> **Length**: 50–500 lines
>
> **Not a feature → still needs BACKLOG.** Feature folders (with an
> `agent_*_orchestrator`) carry `FRD.md` + `BACKLOG.md`. Shared/kernel folders
> carry `DATA.md` + `BACKLOG.md`. Surface folders (DESIGN-only) carry
> `DESIGN.md` + `BACKLOG.md`. Every folder type has a BACKLOG.

---

## Rules

1. **Definitions live once.** State vocabulary and status policy belong in
   the root file (ROADMAP.md). Feature backlogs cite them; they never repeat
   them (`state-vocab-restated`, `undefined-state-vocab`).
2. **Feature backlogs carry file-specific content only.** No policy prose, no
   state tables, no copied paragraphs from the root.
3. **Every folder has a BACKLOG.** An `FRD.md` pairs with `BACKLOG.md` in
   feature folders; `DATA.md` pairs with `BACKLOG.md` in shared folders;
   `DESIGN.md` pairs with `BACKLOG.md` in surface folders. The root
   `PRD.md` pairs with root `ROADMAP.md`, not a sibling `BACKLOG.md`.
4. **ID scopes are explicit.** Each folder uses its own prefix
   (`RENDER-01`, `SCRIPT-01`). A row citing `FR-006` is checked against the
   spec (`orphan-fr-id`).
5. **Status is verified, not self-reported.** A Done or Released row must
   contain a backtick code span naming a command and a commit hash
   (`done-without-evidence`).

---

## Workflow

1. **Create file** → `BACKLOG.md` in feature directory.
2. **Section: Progress Overview** — document baseline state.
3. **Section: Backlog Status** — `{<PROJECT-NAME-ID> | <FEATURE-NAME-ID> | DESIGN-NAME-ID}` items,
   each carrying State / Done / In Progress / Blocked / Next / Risk / Updated.
4. **Section: Reference** — link to PRD.md, FRD.md, ROADMAP.md, ARCHITECTURE.md.
5. **Verify** → `lint-arwaky-cli docs` passes; all sections present.

## Template

Copy, fill, delete nothing.

```markdown
# BACKLOG — <feature-name>

## Progress Overview

<One paragraph of progress summary.>

## Backlog Status 

### {<PROJECT-NAME-ID> | <FEATURE-NAME-ID> | DESIGN-NAME-ID}

| Aspect       | Detail                                      |
|--------------|---------------------------------------------|
| State        | Ready / In Progress / In Review / Done      |
| Done         | <evidence>                                  |
| In Progress  | <None / item name>                          |
| Blocked      | <None / item name>                          |
| Next         | <open row IDs>                              |
| Risk         | <what ships broken / None>                  |
| Updated      | <YYYY-MM-DD>                                |

### {<PROJECT-NAME-ID> | <FEATURE-NAME-ID> | DESIGN-NAME-ID}
### {<PROJECT-NAME-ID> | <FEATURE-NAME-ID> | DESIGN-NAME-ID}
### {<PROJECT-NAME-ID> | <FEATURE-NAME-ID> | DESIGN-NAME-ID}

## Reference

| Artifact     | Purpose                                    | Link |
|--------------|--------------------------------------------|------|
| PRD          | <Business goals and scope>                 | [PRD.md](PRD.md) |
| FRD          | <Functional requirements specification>    | [FRD.md](FRD.md) |
| Roadmap      | <Timeline, phasing, and dependencies>      | [ROADMAP.md](ROADMAP.md) |
| Architecture | <System boundaries, tech stack, and infra> | [ARCHITECTURE.md](ARCHITECTURE.md) |
```

---

## Section Contract

Every section is required unless marked optional. Each exists for one
reason.

| Section          | Why it belongs here                                                     |
| ---------------- | ----------------------------------------------------------------------- |
| Progress Overview| Gives the truth before the table, not after it.                         |
| Backlog Status   | `{<PROJECT-NAME-ID> | <FEATURE-NAME-ID> | DESIGN-NAME-ID}` entries with State / Done / In Progress / Blocked / Next / Risk / Updated. |
| Reference        | Connects the backlog to PRD, FRD, Roadmap, Architecture.               |

---

## Verify

```bash
lint-arwaky-cli docs .
# Checks: pairing, columns, unknown states, unevidenced Done, scenario coverage.
```
