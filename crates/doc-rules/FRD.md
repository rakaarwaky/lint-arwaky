# Doc Rules — FRD

## Feature Overview

The doc-rules crate enforces the AES document-invariant contract across the workspace. It audits every recognized `.md` document against six rules (AES601–AES606): FR-ID format and field completeness with FR/protocol class parity, section structure shape, spec purity, crosslink integrity, document heading structure, and template body-shape conformance with required-document presence. AES606 is answered by four capability seams — table-shape conformance, required-document presence, member-document presence, and template-body placeholder conformance — so this document declares nine requirements against nine protocol classes. Findings carry a machine-readable `violation_type` so consumers can route on them without parsing prose.

Each rule is one capability behind one protocol seam; AES606 carries four seams, so the nine requirements below map one-for-one onto the nine capability seams the agent coordinates. The agent walks the document chain once, hands every capability the same audit context, and merges what they report; the filesystem walk itself lives in the shared utility layer, because an agent may not perform I/O. No external tool is involved.

**Path scoping.** The audit request carries one path, and that path is the **audit root**, not a filter. This differs from `scan`/`check`, which index the whole workspace and then narrow the findings to the target: `docs <path>` discovers the document chain starting at `<path>` and audits only what it finds there. Two consequences follow for a sub-directory run. First, root-level documents outside `<path>` are not audited at all. Second, crosslink and master-document invariants (AES604) resolve against `<path>` — a crate directory has no sibling `PRD.md`/`ROADMAP.md`, so the invariants that look for them behave as they would in a workspace that has none. A whole-workspace audit is the intended default; a sub-directory run is a narrower, self-contained audit, not a filtered view of the full one.

## Functional Requirements

### FR-DOC-001: FR Format, PR-ID Family, Protocol-Class Parity, and BACKLOG Qualifier Alignment (AES601)

| Aspect         | Detail                                      |
|----------------|---------------------------------------------|
| Description    | {FR-DOC-001 description}                     |
| Business Rules | {business rules}                          |
| Edge Cases     | {edge cases}                              |

- **Description**: Every requirement heading states its feature prefix, a PRD and its ROADMAP agree on the `PR-<PROJECT>-NNN` item set, an FRD declares exactly as many requirements as its feature's contract module declares capability seams, every method its API tables promise is one the contract module declares, and every BACKLOG entry qualifier aligns with the qualifier of at least one ID declared by its paired spec.
- **Input**: An `FRD.md`, `DATA.md`, `PRD.md`, `ROADMAP.md`, or `BACKLOG.md` inside the audit context, plus the audit root for resolving the feature's shared contract module.
- **Output**: A list of `DocFinding` values, each carrying `AES601` and one of `id_missing_feature_prefix`, `protocol_count_mismatch`, `api_method_not_found`, `pr_roadmap_id_missing`, `roadmap_pr_id_not_pr`, `backlog_id_not_fr`, `backlog_id_missing`, `ds_backlog_id_not_ds`, `ds_backlog_id_missing`, `dt_backlog_id_not_dt`, `dt_backlog_id_missing`, `backlog_qualifier_mismatch`, `pr_backlog_id_missing`, or `backlog_id_not_pr`.
- **Business Rules**:
  - A requirement ID must read `FR-<FEATURE>-NNN: <imperative name>`; a bare `FR-NNN` reports `id_missing_feature_prefix`.
  - The same shared FR-ID pattern drives the ID check and the parity counter, so an accepted ID set cannot drift between them. Entry row-label validation (`field_missing`-class defects) is owned by AES606 `entry_missing_row`, never by AES601.
  - One protocol class is one capability seam and one seam is one requirement, so the FRD's requirement count must equal the feature's `I*Protocol` count; aggregate traits are not seams and are excluded.
  - The mismatch message names both counts and both fix directions.
  - Each row of the `Protocol API` and `Aggregate API` tables promises a method to an integrator; a method no protocol or aggregate trait declares reports `api_method_not_found`.
  - The method finding names both remedies: delete the promise from the table, or declare the method in the contract module.
  - Only the first cell of a table row is a promise. A method named in a row's prose or in a body paragraph is not one, so a doc that discusses a name the code never declares stays silent.
  - The finding anchors to its own table row's line, not to the enclosing heading, so a report points at the row a reader has to fix.
  - The check applies to `FRD.md` and `DATA.md`; parity and the method check apply to `FRD.md` alone, because only an FRD names a feature's contract module.
  - **PR-ID family.** A `PR-<PROJECT>-NNN:` heading (H2–H4, colon-terminated) in the PRD's `Project Requirements` must have a matching ROADMAP `Roadmap Status` entry; a PRD item absent from the ROADMAP reports `pr_roadmap_id_missing` anchored to the ROADMAP's `## Roadmap Status` heading, and a ROADMAP item absent from the PRD reports `roadmap_pr_id_not_pr` anchored to the item's own heading line.
  - **BACKLOG qualifier alignment.** A BACKLOG entry's H3 title is a qualifier token `{Q}` where Q is one of `PROJECT-NAME-ID`, `FEATURE-NAME-ID`, `DESIGN-NAME-ID`; the paired spec (FRD, DESIGN, DATA, or PRD+ROADMAP depending on location) declares IDs `<PREFIX>-<QUALIFIER>-NNN`, so alignment is qualifier-level. A BACKLOG entry whose qualifier no paired-spec ID carries reports `backlog_qualifier_mismatch` anchored to the entry's H3 line. A paired-spec qualifier with no BACKLOG entry reports the family's missing-code (`backlog_id_missing` / `ds_backlog_id_missing` / `dt_backlog_id_missing` / `pr_backlog_id_missing`) anchored to `## Backlog Status`. A `PR-`/`FR-`/`DS-`/`DT-` token in an entry's `Next`/`Done`/`Blocked` row that the paired spec does not declare reports the family's not-code (`backlog_id_not_fr` / `ds_backlog_id_not_ds` / `dt_backlog_id_not_dt` / `backlog_id_not_pr`) anchored to the entry's H3 line.
- **Edge Cases**: A feature crate whose name uses `-` is resolved to the `_` spelling of its shared module. A feature with no shared contract module cannot mismatch and cannot owe a method, so both checks stay silent. A method declared on any trait in the module satisfies a row, so a seam's declaration and the aggregate's `execute` are read the same way. A BACKLOG entry whose H3 title is not a single brace-wrapped token is skipped (malformed entries are silent). A paired spec that declares no IDs makes the missing-direction silent for that pair; the stray-ID direction still fires on every token.
- **Error Handling**: An unreadable contract module yields no parity or method finding rather than a false violation. The parity finding is anchored to the requirements section, falling back to the first line when the document has none; a method finding is anchored to its own row.

### FR-DOC-002: Section Structure (AES602)

| Aspect         | Detail                                      |
|----------------|---------------------------------------------|
| Description    | {FR-DOC-002 description}                     |
| Business Rules | {business rules}                          |
| Edge Cases     | {edge cases}                              |

- **Description**: A requirement document reads in template order: FRD and DATA H2 sections must follow the shared template H2 set.
- **Input**: An `FRD.md` or `DATA.md` inside the audit context, parsed into its level-1 and level-2 sections once per document.
- **Output**: A list of `DocFinding` values, each carrying `AES602` and `order_violation`.
- **Business Rules**:
  - Sections must appear in the FRD template order; a reordering reports `order_violation` anchored to the first out-of-order section.
  - The section tree is parsed once per document.
  - The full order set applies to `FRD.md`; `DATA.md` is held to the same order rule.
- **Edge Cases**: A document whose parse yields no sections produces no findings; level-3 headings are free-form and outside this rule's scope.
- **Error Handling**: A document whose parse yields no sections produces no findings; an unparseable document is a parse skip, never a violation.

### FR-DOC-003: Spec Purity (AES603)

| Aspect         | Detail                                      |
|----------------|---------------------------------------------|
| Description    | {FR-DOC-003 description}                     |
| Business Rules | {business rules}                          |
| Edge Cases     | {edge cases}                              |

- **Description**: A specification carries no implementation state and names no source file.
- **Input**: Every promise-bearing document inside the audit context — `FRD.md`, `DATA.md`, `PRD.md`, `ROADMAP.md`.
- **Output**: A list of `DocFinding` values, each carrying `AES603` and one of `status_leak` or `source_file_named`.
- **Business Rules**:
  - The document set is this capability's own decision: a `BACKLOG.md` reports progress, which is its job, so purity applies to the specs alone.
  - A status leak is a checkbox task item, a `Status:` field, an implementation-state claim, a release-state claim, a status marker, or a progress percentage.
  - A table row that defines status vocabulary is a definition, not a claim, so it is skipped.
  - A macro invocation such as `unimplemented!` names a language token rather than making a claim about this feature's progress, so a match followed by `!` is skipped.
  - A source-file reference names a concrete `.py`, `.rs`, `.ts`, or `.tsx` path, and the finding quotes it.
- **Edge Cases**: A prose-only document with no sections is still audited for purity — section shape is another rule's question.
- **Error Handling**: A pattern that fails to compile leaves the check silent rather than reporting every line as a violation.

### FR-DOC-004: Crosslinks and State-Vocabulary Placement (AES604)

| Aspect         | Detail                                      |
|----------------|---------------------------------------------|
| Description    | {FR-DOC-004 description}                     |
| Business Rules | {business rules}                          |
| Edge Cases     | {edge cases}                              |

- **Description**: A requirement document links the documents it depends on, and the state vocabulary lives once, in the root master.
- **Input**: The requirement documents plus the root master text carried by the audit context, and each document's position relative to the audit root.
- **Output**: A list of `DocFinding` values, each carrying `AES604` and one of `no_backlog_link`, `no_prd_link`, or `state_vocab_restated`.
- **Business Rules**:
  - The Reference section of an `FRD.md` or `DATA.md` must link its `BACKLOG.md` and its `PRD.md`, or a reader cannot reach the report that says whether the promise was kept.
  - The master owns the state vocabulary, so the root document is the one allowed to carry those sections; a feature `BACKLOG.md` restating them reports `state_vocab_restated` and names every section it restated.
  - The restatement finding is raised only when a master exists to be the single home.
- **Edge Cases**: A root-level legacy `BACKLOG.md` is accepted as the master during migration, so a workspace can move to `ROADMAP.md` in place.
- **Error Handling**: A document with no Reference section is silent on crosslinks rather than reported for a section it never claimed to carry.

### FR-DOC-005: Document Heading Structure (AES605)

| Aspect         | Detail                                      |
|----------------|---------------------------------------------|
| Description    | {FR-DOC-005 description}                     |
| Business Rules | {business rules}                          |
| Edge Cases     | {edge cases}                              |

- **Description**: A recognized document opens with exactly one level-1 heading, carries every required level-2 section, and holds no level-2 heading outside its template.
- **Input**: Every document inside the audit context that has a registered H2 contract.
- **Output**: A list of `DocFinding` values, each carrying `AES605` and one of `h1_count`, `h2_missing`, or `h2_unexpected`.
- **Business Rules**:
  - Only a document with a registered H2 contract has a heading shape to hold; a document without one is skipped rather than reported.
  - Exactly one level-1 heading opens the file; zero or more than one reports `h1_count`.
  - Every required level-2 heading must be present; the finding names each absent section.
  - The H2 set is closed: a heading outside the required plus allowed union reports `h2_unexpected` and names the heading and its remedy.
  - Headings inside fenced code blocks are ignored, so a shell comment such as `# Tests (matches CI "Tests" job)` never reads as a heading.
  - H2 matching is case-insensitive, punctuation-insensitive, leading-word, and drops a leading list index, so `## 11. Root Layer` satisfies `Root Layer`.
- **Edge Cases**: Level-3 and deeper headings are free-form per project and are never reported.
- **Error Handling**: Heading counts are document-level findings and anchor to line 0, because a missing heading has no single line to blame.

### FR-DOC-006: Template Body-Shape Conformance (AES606)

| Aspect         | Detail                                      |
|----------------|---------------------------------------------|
| Description    | {FR-DOC-006 description}                     |
| Business Rules | {business rules}                          |
| Edge Cases     | {edge cases}                              |

- **Description**: Every table-shaped section the HOW-TO templates mandate in `BACKLOG.md` and `ROADMAP.md` carries its template table, and no non-deletable template section has been removed.
- **Input**: Every `BACKLOG.md` and `ROADMAP.md` inside the audit context.
- **Output**: A list of `DocFinding` values, each carrying `AES606` and one of `section_not_table`, `table_missing_columns`, or `section_deleted`.
- **Business Rules**:
  - The six mandated BACKLOG/ROADMAP table sections are checked for a table carrying every required column; a section carrying bullets or prose fires `section_not_table`, and a table missing one or more required columns fires `table_missing_columns`.
  - A section absent from the document is a missing-H2 question answered by AES605, so this rule only fires when the section is present but wrong-shaped.
  - A non-deletable template section that is entirely absent fires `section_deleted` anchored to the document level; deletable sections (per `TemplateProfile.deletable_sections`) may be removed without a finding.
  - The enforced column sets are transcribed from the shipped `HOW-TO-MAKE-BACKLOG.md` and `HOW-TO-MAKE-ROADMAP.md` templates so the linter cannot reject a document that copied the template verbatim.
  - The ROADMAP checks cover `State Definitions` and `Feature Roll-up` only; `Change Log` and `Dependencies` are BACKLOG sections and are not checked on a ROADMAP.
- **Edge Cases**: A `ROADMAP.md` carrying no `## State Definitions` section at all raises no AES606 finding (the section is absent, not wrong-shaped).
- **Error Handling**: An unreadable document yields no findings rather than a false violation.

### FR-DOC-007: Required-Document Presence (AES606)

| Aspect         | Detail                                      |
|----------------|---------------------------------------------|
| Description    | {FR-DOC-007 description}                     |
| Business Rules | {business rules}                          |
| Edge Cases     | {edge cases}                              |

- **Description**: The six required root documents exist at a real workspace root.
- **Input**: The audit context's document set and the audit root, for the workspace-root marker check.
- **Output**: A list of `DocFinding` values, each carrying `AES606` and `missing_required_doc`.
- **Business Rules**:
  - The required-document set is `PRD.md`, `ROADMAP.md`, `README.md`, `AGENTS.md`, `ARCHITECTURE.md`, and `CONTRIBUTING.md`; each missing one fires `missing_required_doc` anchored to line 0.
  - The check is root-gated: it fires only when the audit root carries `Cargo.toml`, `pyproject.toml`, or `package.json`; a sub-directory audit is self-contained and does not owe the root document set.
  - `DESIGN.md`, `FRD.md`, `DATA.md`, and root `BACKLOG.md` are per-feature files and are intentionally absent from the required set.
- **Edge Cases**: A workspace root with no marker files is treated as a sub-directory audit and the check stays silent.
- **Error Handling**: An unreadable document yields no findings rather than a false violation.

### FR-DOC-008: Member-Folder Document Presence (AES606)

| Aspect         | Detail                                      |
|----------------|---------------------------------------------|
| Description    | {FR-DOC-008 description}                     |
| Business Rules | {business rules}                          |
| Edge Cases     | {edge cases}                              |

- **Description**: Every workspace member folder contains its mandated documentation pair based on its architectural role, classified by objective crate structure.
- **Input**: The `members: Vec<MemberDescriptor>` field of `DocAuditContext`, pre-populated by the shared utility collector (no I/O inside the capability), plus the audit root for the workspace-root / member-root / sub-folder gating decision.
- **Output**: A list of `DocFinding` values, each carrying `AES606` and `missing_member_doc`. Surfaced in **both** `lint-arwaky-cli docs .` and `lint-arwaky-cli check .` (via the adjusted scope filter in the surface check action; Decision Q2).
- **Business Rules**:
  - A member's role is derived from *crate structure*, never from the presence of the document under audit:
    1. **Kernel** — a directory named `shared` (or `kernel`). Owes `DATA.md` + `BACKLOG.md`.
    2. **Feature** — the folder contains an orchestrator file matching the feature orchestrator pattern, or its `Cargo.toml`/source tree exposes a capability/agent module. Owes `FRD.md` + `BACKLOG.md`.
    3. **Surface** — the folder is surface-dominated (contains `surface_*` files and/or its `Cargo.toml` dependencies are dominated by the surface crate set) and is neither kernel nor feature. Owes `DESIGN.md` + `BACKLOG.md`.
    4. **None** — anything else (benchmarks, examples, a workspace-only root crate) owes no doc pair.
  - The audit is root-gated by the same `WORKSPACE_ROOT_MARKERS` / `is_workspace_root()` model as FR-DOC-007, with three explicit cases:
    - **Root = workspace root**: scan `crates/`, `modules/`, `packages/` and evaluate every classified member.
    - **Root = a member folder** (e.g. `docs crates/billing`): classify that folder directly and audit only its doc pair.
    - **Root = an internal subfolder** (e.g. `docs crates/shared/src`): the check stays silent; a nested source tree is not a member.
  - Each finding anchors to line `0` of the missing document path and names the member, its expected role, the missing file, and the exact scaffold command.
- **Edge Cases**:
  - A new surface crate that has scaffolded its source but not yet created `DESIGN.md` is still classified `Surface` (because classification keys on `surface_*` files, not on the missing doc) and still fires `missing_member_doc` — no silent escape.
  - A pure utility/build-only crate is classified `None` and produces no findings.
- **Error Handling**: Member discovery is a filesystem walk in the shared utility layer; listing or read failures are recorded on the `MemberDescriptor` (absent entry) and produce no crash. The capability itself, which is pure in-memory, emits nothing for a descriptor that failed to read.

### FR-DOC-009: Template Body & Placeholder Conformance (AES606)

| Aspect         | Detail                                      |
|----------------|---------------------------------------------|
| Description    | {FR-DOC-009 description}                     |
| Business Rules | {business rules}                          |
| Edge Cases     | {edge cases}                              |

- **Description**: Every recognized document whose headings and sections follow the shipped HOW-TO template carries no unfilled placeholder tokens, and no non-deletable template section has been removed.
- **Input**: Every recognized document in the audit context, compared against the corresponding `TemplateProfile` and embedded template text.
- **Output**: A list of `DocFinding` values, each carrying `AES606` and one of `unfilled_placeholder` or `section_deleted`.
- **Business Rules**:
  - The placeholder scan is performed by the structured scanner in the shared markdown utility, which first blanks fenced code blocks, inline backtick spans, autolinks (`<https://…>`, `<mailto:…>`), and known HTML tags; surviving prose is matched against the template's hard-coded `placeholder_catalog` (Decision Q3).
  - An unfilled placeholder token from the catalog (e.g. `<feature-name>`, `<YYYY-MM-DD>`, `<link to root PRD.md>`, `<N>`, or curly slots like `{architecture scanner command}`) that survives non-prose stripping fires `unfilled_placeholder`, anchored to the exact 1-based line.
  - A token that is a *filled* value is not in the catalog and will not be flagged.
  - If a non-deletable template section (per the `TemplateProfile`) is entirely missing from the document body, the checker fires `section_deleted` (complementing AES605's `h2_missing` by anchoring to template fidelity). Deletable sections may be removed without a finding.
  - **Placeholder-state semantics**: a document with **all placeholders populated** produces 0 findings under this requirement; a document copied verbatim from a template with placeholders still in place **does** produce `unfilled_placeholder` findings — that is the rule's intended behavior, converting "copied the template, forgot to fill it in" into a visible violation.
- **Edge Cases**:
  - A document with zero content (0 bytes) is caught by AES605 (`h1_count`) and is skipped here to avoid duplicate findings.
  - Code or types that look like angle tokens (`Vec<DocFinding>`, `Result<bool, Error>`, `<br>`) are blanked by the non-prose strip and never flagged.
- **Error Handling**: The scanner is pure string matching against the catalog; absent catalog entries leave prose unflagged (fail-safe).

## Public Interface

### Protocol API

| Method | Input | Output | Error | Event | Description |
|--------|-------|--------|-------|-------|-------------|
| `audit_fr_format` | `DocAuditContext` | `Vec<DocFinding>` | — | — | FR-ID format, FR fields, FR/protocol-class parity, and promised-API-method existence (AES601). |
| `audit_section_structure` | `DocAuditContext` | `Vec<DocFinding>` | — | — | Template section order and mandated section shape (AES602). |
| `audit_spec_purity` | `DocAuditContext` | `Vec<DocFinding>` | — | — | Status leaks and source-file names in a specification (AES603). |
| `audit_crosslinks` | `DocAuditContext` | `Vec<DocFinding>` | — | — | Reference crosslinks and state-vocabulary placement (AES604). |
| `audit_doc_heading` | `DocAuditContext` | `Vec<DocFinding>` | — | — | H1 count and closed H2 set per document template (AES605). |
| `audit_template_conformance` | `DocAuditContext` | `Vec<DocFinding>` | — | — | BACKLOG/ROADMAP table-shape conformance (AES606). |
| `audit_required_docs` | `DocAuditContext` | `Vec<DocFinding>` | — | — | Required root-document presence (AES606). |
| `audit_member_docs` | `DocAuditContext` | `Vec<DocFinding>` | — | — | Required member-folder document presence (AES606). |
| `audit_template_body` | `DocAuditContext` | `Vec<DocFinding>` | — | — | Template body fidelity and unfilled placeholder detection (AES606). |

### Aggregate API

| Method | Input | Output | Error | Event | Description |
|--------|-------|--------|-------|-------|-------------|
| `execute` | `DocRequest` | `DocResponse` | — | — | Walks the document chain once, fans out to the nine protocol seams, and returns the merged, deduplicated, stable-sorted findings. |

## Reference

- Backlog: [BACKLOG.md](BACKLOG.md) — real condition for this feature; this file is specification only.
- PRD: [PRD.md](../../PRD.md)
- Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)

| Roadmap | {Link to ROADMAP.md} | [{link}](ROADMAP.md) |
