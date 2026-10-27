# HOW TO MAKE DESIGN.md

> **Purpose**: Define the design contract of a surface so designers and
> engineers can implement or audit it without asking the author.
>
> **Audience**: Designers, Surface Engineers, TUI developers.
>
> **Scope**: One DESIGN.md per surface folder.
>
> **Location**: Inside the surface/ folder of each crate/member.
>
> **Length**: 50–500 lines.

---

## Rules

1. **Contract, not screenshots.** The spec names the sections it owns and the
   item IDs it declares; every visual decision must be traceable to a row or
   token in one of them.
2. **Requirement IDs are `DS-<DESIGNNAME>-NNN: <title>`.** Every item heading
   must be H2–H4 (`### DS-DISP-001: Primary Button`), colon-terminated, and
   carry the folder's DESIGNNAME qualifier between the `DS-` prefix and the
   number — a bare `DS-001` (digits only, no NAME token) is **not** accepted.
   Each declared DS-ID must have a matching row in the same folder's
   `## Backlog` table, and every BACKLOG row ID must be a declared DS-ID.
   The same-folder `DESIGN.md`+`BACKLOG.md` pairing drives AES601 backlog
   alignment; a folder holding only one of the pair stays silent.
3. **Items are H3 under `## Design Requirements`.** `### DS-<DESIGNNAME>-NNN:`
   headings sit directly under the H2; H4 and deeper stay free-form detail.
4. **Keep it minimal.** Only `Design Overview`, `Design Requirements`, `Reference`
   are required; the `DS-` item headings are H3s under `Design Requirements`.

---

## Workflow

1. **Create file** → `DESIGN.md` in the surface/ folder.
2. **Section: Design Overview** — one paragraph of what the design looks like.
3. **Section: Design Requirements** — one H3 per requirement,
   `### DS-<DESIGNNAME>-NNN: <short imperative name>`; fill the item's table.
4. **Section: Reference** — link to PRD.md, BACKLOG.md, ROADMAP.md,
   ARCHITECTURE.md.
5. **Verify** → `lint-arwaky-cli docs` passes; section order is correct.

## Template

Copy, fill, delete nothing.

```markdown
# DESIGN — <project-name>

## Design Overview

<One paragraph of what the design looks like.>

## Design Requirements

### DS-<DESIGN-NAME-ID>-001: <Short imperative name>

| Variant | Use case | Token | Value |
|---------|----------|-------|-------|
|         |          |       |       |

### DS-<DESIGN-NAME-ID>-002: <Short imperative name>
### DS-<DESIGN-NAME-ID>-003: <Short imperative name>
### DS-<DESIGN-NAME-ID>-004: <Short imperative name>
### DS-<DESIGN-NAME-ID>-005: <Short imperative name>
### DS-<DESIGN-NAME-ID>-006: <Short imperative name>

## Reference

| Artifact     | Purpose                                         | Link |
|--------------|-------------------------------------------------|------|
| PRD          | <Business goals, user stories, and scope>       | [PRD.md](PRD.md) |
| Backlog      | <Task tracking, epics, and implementation status>| [BACKLOG.md](BACKLOG.md) |
| Roadmap      | <Timeline, phasing, and dependencies>           | [ROADMAP.md](ROADMAP.md) |
| Architecture | <System boundaries, tech stack, and infra>      | [ARCHITECTURE.md](ARCHITECTURE.md) |
```

---

## Section Contract

Only `Design Overview`, `Design Requirements`, `Reference` are required.
Each exists for one reason.

| Section              | Why it belongs here                                           |
| -------------------- | ------------------------------------------------------------- |
| Design Overview      | Orients the reader before the requirements begin.             |
| Design Requirements  | `DS-<DESIGN-NAME-ID>-NNN` items the surface must carry.       |
| Reference            | Connects the spec back to the spec chain.                     |

---

## Verify

```bash
lint-arwaky-cli docs .
# Checks: IDs, orphan refs, section order, status leak, links.
```

On any violation the gate prints `[FAIL] <code> <path>: <message>` and exits
non-zero; every finding gates (strict is the only mode — no advisory tier).
