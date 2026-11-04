# DESIGN.md sections

Every surface folder documents how its surface looks and behaves in `DESIGN.md` at the folder root, next to its source. A surface-dominated folder carrying no `DESIGN.md` is an **AES703** violation — the linter names the folder and you write the file.

The file is written for the developer who arrives before you and has to change something without reading every source file. It carries six sections:

| Section | Answers |
| --- | --- |
| `## Kind` | Which surface this is — `api`, `mcp`, `cli`, `desktop`, `tui` — and what the layer does and does not own. |
| `## Entry Points` | One row per entry: the file, what the user or caller invokes, and which aggregate it reaches. |
| `## Request Shape` | What a request looks like once it arrives, and what has already been resolved. |
| `## States` | Each state a caller can observe, the condition that produces it, and what the user sees. |
| `## Error States` | Each failure mode, where it is caught, and what the caller gets. |
| `## Invariants` | The rules a change must not break. |
| `## Change Checklist` | Which files to touch for which kind of change. |

Write each row against the real code. A `DESIGN.md` that restates the layer definition instead of naming this folder's files has told the reader nothing they could not get from `ARCHITECTURE.md`, and AES703 will pass while the file stays useless.

Feature folders carry two documents instead — `FRD.md` and `BACKLOG.md` — recording what the feature does and where its work stands. `shared/` folders carry neither.
