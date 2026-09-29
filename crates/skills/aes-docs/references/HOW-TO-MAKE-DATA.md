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
2. **One contract per row.** Each table row describes one data concept — a value
   object, entity, event, request, or response. Give it an ID, a field name, a
   type label, and what it represents.
3. **No implementation state.** The document must survive every refactor. If
   renaming a struct or moving a module would require updating DATA.md, the
   document is too implementation-bound.
4. **Cross-link to BACKLOG.md** in `## Reference`. A reader landing on DATA.md
   must immediately see the tracking claim.
5. **Keep it minimal.** Only four sections are required: Reference, Data
   Overview, Value Objects, Assumptions & Constraints.

---

## Workflow

1. **Create file** → `DATA.md` in the shared/ directory.
2. **Section: Reference** — link to BACKLOG.md.
3. **Section: Data Overview** — one paragraph: what the kernel manages, who reads/writes it.
4. **Section: Value Objects** — a table with ID, Field, Type, Description columns.
5. **Section: Assumptions & Constraints** — name any implicit data requirements.
6. **Verify** → `aa check docs` passes; section order is correct.

## Template

Copy, fill, delete nothing.

```markdown
# DATA — <shared-folder-name>

## Reference

- PRD: <link to root PRD.md>
- Backlog: [BACKLOG.md](BACKLOG.md)

## Data Overview

<One paragraph: what data shapes this kernel manages, who reads/writes them,
what business domain they serve. No code.>

## Value Objects

| ID | Field | Type | Description |
|---|---|---|---|
| DO-001 | <field-name> | vo/event/entity/request/response | <what this shape represents> |
| DO-002 | <field-name> | vo/event/entity/request/response | <what this shape represents> |

## Assumptions & Constraints

- <Assumption or constraint — name it, scope it, date it if it expires.>
```

---

## Section Contract

Every section is required. Each exists for one reason.

| Section | Why it belongs here |
|---|---|
| Reference | Separates spec promise from backlog claim. |
| Data Overview | Orients the reader before details begin. |
| Value Objects | The conceptual shapes the kernel manages. |
| Assumptions & Constraints | Implicit data requirements made explicit. |

---

## Verify

```bash
aa check docs
# path form: aa check docs .
# Checks: IDs, orphan refs, section order, status leak, links.
```

On any violation the gate prints `[FAIL] <code> <path>: <message>` and exits
non-zero; every finding gates (strict is the only mode — no advisory tier).
