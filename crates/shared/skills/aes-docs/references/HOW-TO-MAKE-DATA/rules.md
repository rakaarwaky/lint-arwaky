# DATA Rules

**Purpose:** Describe what data shapes the shared/kernel layer manages — without a single implementation detail that would stale on refactor.

**Audience:** Engineers, Tech Lead. Anyone who needs to understand the kernel's data contracts.

**Scope:** One DATA.md per shared folder.

**Location:** Inside the shared/ directory of each crate/member.

**Length:** 50–200 lines.

**Not a feature → no DATA.md.** Only `shared/` folders carry DATA.md. Feature folders carry FRD.md; surface folders carry DESIGN.md.

## Rules

1. **Data contracts, not code.** State the shape, invariants, and relationships of every value object the kernel exposes. Never name a source file, class, type signature, or programming-language syntax.
2. **Requirement IDs are `DT-<DATANAME>-NNN: <title>`.** Every item heading is H3 (`### DT-SHAR-001: Severity`) under `## Data Requirements`, colon-terminated, and carries the folder's DATANAME qualifier between the `DT-` prefix and the number — a bare `DT-001` (digits only, no NAME token) is **not** accepted; `DO-` headings are **not** accepted. Each declared DT-ID must have a matching row in the same folder's `## Backlog` table, and every BACKLOG row ID must be a declared DT-ID. The same-folder `DATA.md`+`BACKLOG.md` pairing drives AES601 backlog alignment; a folder holding only one of the pair stays silent.
3. **Items are H3 under `## Data Requirements`.** `### DT-<DATANAME>-NNN: <short imperative name>` headings sit directly under the H2; each item owns its Type/Description table.
4. **No implementation state.** The document must survive every refactor. If renaming a struct or moving a module would require updating DATA.md, the document is too implementation-bound.
5. **Cross-link to BACKLOG.md** in `## Reference`. A reader landing on DATA.md must immediately see the tracking claim.
6. **Keep it minimal.** Only `Data Overview`, `Data Requirements`, `Reference` are required; the `DT-` item headings are H3s under `Data Requirements`.
