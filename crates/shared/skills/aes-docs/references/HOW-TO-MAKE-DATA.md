# HOW TO MAKE DATA.md

> **Purpose**: Describe what data shapes the shared/kernel layer manages — without
> a single implementation detail that would stale on refactor.
>
> **Audience**: Engineers, Tech Lead. Anyone who needs to understand the kernel's
> data contracts.
>
> **Scope**: One DATA.md per shared folder.
>
> **Location**: Inside the shared/ directory of each crate/member.
>
> **Length**: 50–200 lines.
>
> **Not a feature → no DATA.md.** Only `shared/` folders carry DATA.md.
> Feature folders carry FRD.md; surface folders carry DESIGN.md.

---

## Rules

1. **Data contracts, not code.** State the shape, invariants, and relationships
   of every value object the kernel exposes. Never name a source file, class,
   type signature, or programming-language syntax.
2. **Requirement IDs are `DT-<DATANAME>-NNN: <title>`.** Every item heading is
   H2 (`## DT-SHAR-001: Severity`), colon-terminated, and carries the
   folder's DATANAME qualifier between the `DT-` prefix and the number — a
   bare `DT-001` (digits only, no NAME token) is **not** accepted; `DO-`
   headings are **not** accepted. Each declared DT-ID must have a matching
   row in the same folder's `## Backlog` table, and every BACKLOG row ID must
   be a declared DT-ID. The same-folder `DATA.md`+`BACKLOG.md` pairing drives
   AES601 backlog alignment; a folder holding only one of the pair stays
   silent.
3. **Items are H2.** `## DT-<DATANAME>-NNN: <short imperative name>` headings
   sit at the top level of the document body; each item owns its
   Type/Description table.
4. **No implementation state.** The document must survive every refactor. If
   renaming a struct or moving a module would require updating DATA.md, the
   document is too implementation-bound.
5. **Cross-link to BACKLOG.md** in `## Reference`. A reader landing on DATA.md
   must immediately see the tracking claim.
6. **Keep it minimal.** Only two sections are required: Data Overview,
   Reference. The `DT-` item headings are allowed H2s.

---

## Workflow

1. **Create file** → `DATA.md` in the shared/ directory.
2. **Section: Data Overview** — one paragraph: what the kernel manages, who
   reads/writes it.
3. **Items: `## DT-<DATANAME>-NNN: <short imperative name>`** — one H2 per
   data concept; fill the item's Type/Description table.
4. **Section: Reference** — link to PRD.md, BACKLOG.md, ROADMAP.md,
   ARCHITECTURE.md.
5. **Verify** → `lint-arwaky-cli docs` passes; section order is correct.

## Template

Copy, fill, delete nothing.

```markdown
# DATA Requirements

## Data Overview

<One paragraph: what data shapes this kernel manages, who reads/writes them,
what business domain they serve. No code.>

## DT-<DATANAME>-001: <Short imperative name>
| Type  | Description |
|-------|-------------|

## DT-<DATANAME>-002: <Short imperative name>
## DT-<DATANAME>-003: <Short imperative name>
## DT-<DATANAME>-004: <Short imperative name>
## DT-<DATANAME>-005: <Short imperative name>
## DT-<DATANAME>-006: <Short imperative name>

## Reference

- PRD: <link to root PRD.md>
- Backlog: [BACKLOG.md](BACKLOG.md)
- ROADMAP: [ROADMAP.md](ROADMAP.md)
- Architecture: [ARCHITECTURE.md](ARCHITECTURE.md)
```

---

## Section Contract

Only the two H2s above are required. Each exists for one reason.

| Section        | Why it belongs here                                        |
|---|---|
| Data Overview  | Orients the reader before the data items begin.            |
| Reference      | Separates spec promise from backlog claim.                 |

---

## Verify

```bash
lint-arwaky-cli docs .
# Checks: IDs, orphan refs, section order, status leak, links.
```

On any violation the gate prints `[FAIL] <code> <path>: <message>` and exits
non-zero; every finding gates (strict is the only mode — no advisory tier).
