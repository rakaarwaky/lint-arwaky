# DESIGN Rules

**Purpose:** Define the design contract of a surface so designers and engineers can implement or audit it without asking the author.

**Audience:** Designers, Surface Engineers, UI/UX Designer.

**Scope:** One DESIGN.md per surface folder.

**Location:** Inside the surface/ folder of each crate/member.

**Length:** 50–500 lines.

## Rules

1. **Contract, not screenshots.** The spec names the sections it owns and the item IDs it declares; every visual decision must be traceable to a row or token in one of them.
2. **Requirement IDs are `DS-<DESIGNNAME>-NNN: <title>`.** Every item heading must be H2–H4 (`### DS-DISP-001: Primary Button`), colon-terminated, and carry the folder's DESIGNNAME qualifier between the `DS-` prefix and the number — a bare `DS-001` (digits only, no NAME token) is **not** accepted. Each declared DS-ID must have a matching row in the same folder's `## Backlog` table, and every BACKLOG row ID must be a declared DS-ID. The same-folder `DESIGN.md`+`BACKLOG.md` pairing drives AES601 backlog alignment; a folder holding only one of the pair stays silent.
3. **Items are H3 under `## Design Requirements`.** `### DS-<DESIGNNAME>-NNN:` headings sit directly under the H2; H4 and deeper stay free-form detail.
4. **Keep it minimal.** Only `Design Overview`, `Design Requirements`, `Reference` are required; the `DS-` item headings are H3s under `Design Requirements`.

## Section Contract

| Section | Why it belongs here |
| --------- | --------------------- |
| Design Overview | Orients the reader before the requirements begin. |
| Design Requirements | `DS-<DESIGNNAME>-NNN` items the surface must carry. |
| Reference | Connects the spec back to the spec chain. |
