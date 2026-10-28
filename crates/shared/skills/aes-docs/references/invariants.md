# aes-docs — Invariants

Machine-checked rules enforced by `lint-arwaky-cli docs` (crate `doc-rules`, AES601–AES606).
Each document's required section set is cross-checked against its reference's contract table.

| Code | Rule |
|------|------|
| `status-in-spec` / `spec-source-path` | Spec and status never share a file. Specs promise; backlogs report. Specs are stateless — no `.py`/`.rs`/`.ts` file names (HOW-TO Rule 9). |
| `spec-without-backlog` / `backlog-without-spec` / `feature-doc-in-shared` | An **FRD** and its backlog are a pair in the same **feature** directory (root PRD pairs with root ROADMAP, not a sibling BACKLOG). Kernel folders (`modules/shared`) are not features: either file under `shared/` fails with `feature-doc-in-shared`. |
| `no-master-backlog` / `undefined-state-vocab` / `master-section-missing` | One root master owns the `State` vocabulary and the roll-up (`ROADMAP.md` `Roadmap Status`; legacy root `BACKLOG.md` accepted during migration). |
| `state-vocab-restated` | Definitions live once. Feature backlogs cite `Roadmap Status`, never restate it under `Backlog Status`. |
| `done-without-evidence` / `unknown-state` | Every backlog `Done` row is re-runnable: command + counts + commit hash, and what it excludes. |
| `duplicate-fr-id` / `orphan-fr-ref` | Requirement IDs are unique, stable, and the only thing a backlog row may cite. |
| `fr-id-format` / `fr-fields-missing` | FRD Rule 1–2: `FR-<FEATURENAME>-NNN: <imperative name>`; each FR entry table carries the twelve `Aspect &#124; Detail` rows: Description, Business Rules, Edge Cases, Capability, Protocol, Method, Input, Output, Error, Event, Test, Metric. |
| `reference-crosslink` / `section-order` | FRD + template: `## Reference` links PRD and BACKLOG; sections follow HOW-TO-MAKE-FRD order (Feature Overview, Functional Requirements, Public Interface, Reference). |
| `backlog-columns` / `backlog-row-width` | The `Backlog Status` entry table keeps its seven `Aspect &#124; Detail` rows: State, Done, In Progress, Blocked, Next, Risk, Updated. |
| `*-section-missing` | Each document carries the H2 set its audience needs. Section contracts are in the refs. |
| `dead-link` / `root-relative-link` | Pointers resolve from the file that writes them, not only from the repo or skill root. |
| `unreferenced-file` | Every file under a skill's `references/`, `scripts/`, `assets/` is surfaced by SKILL.md. |
| `absolute-path` / `secret-in-docs` | No machine-specific path and no credential literal in any document. |
| `ci-command-drift` | AGENTS.md commands match CI exactly or are labelled advisory. |
| `doc-length` / `doc-thin` | Each document stays inside the size its audience can read. |
| `section_not_table` / `table_missing_columns` / `missing_required_doc` | AES606: mandated entry tables carry their template tables; the six required root docs exist at a real workspace root. |

## Unchecked invariants (language rules)

The checker cannot parse code intent. Enforce manually:

- **Doc comments explain *what* and *why*, never *how*** (the code shows how).
- **Python**: Public classes and functions need docstrings (PEP 257) — see `references/HOW-TO-MAKE-PYTHON-DOC.md`.
- **Rust**: Public items need `///` (plain `//` is invisible to `cargo doc`). Examples must compile — see `references/HOW-TO-MAKE-RUST-DOC.md`.
- **TypeScript**: Public items need JSDoc — see `references/HOW-TO-MAKE-TYPESCRIPT-DOC.md`.

## Checker false-freights to dodge

- The word "implemented" in a spec file. `status-in-spec` matches `\b(impl|un)plemented\b` case-insensitively across the whole FRD/PRD — any `Implemented` / `Shipped` status-style cell header or cell is auto-flagged. Name status-style columns `impl / intended` and use `impl` for the cell. (Same class: `shipped` / `released in v…`, checkbox items, status markers, progress `%`.)
- Entry-table rows without a header + separator. AES606's `section_not_table` parser reads `| Aspect | … |` rows via a markdown-table parser that needs the header row + a `|---|` separator line; a headerless block of `| … |` rows parses as **0 rows**. Always emit the header row; keep exactly one row per fixed `Aspect` label, in template order.
