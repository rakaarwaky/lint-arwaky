# Doc Rules — FRD

## Reference

- Backlog: [BACKLOG.md](BACKLOG.md) — real condition for this feature; this file is specification only.
- PRD: [PRD.md](../../PRD.md)
- Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)

## System Overview

The doc-rules crate enforces the AES document-invariant contract across the workspace. It audits every recognized `.md` document against five rules (AES601–AES605): FR-ID format and field completeness with FR/protocol class parity, section structure shape, spec purity, crosslink integrity, and document heading structure. Findings carry a machine-readable `violation_type` so consumers can route on them without parsing prose.

Each rule is one capability behind one protocol seam, so the five requirements below map one-for-one onto the five capability seams the agent coordinates. The agent walks the document chain once, hands every capability the same audit context, and merges what they report; the filesystem walk itself lives in the shared utility layer, because an agent may not perform I/O. No external tool is involved.

**Path scoping.** The audit request carries one path, and that path is the **audit root**, not a filter. This differs from `scan`/`check`, which index the whole workspace and then narrow the findings to the target: `docs <path>` discovers the document chain starting at `<path>` and audits only what it finds there. Two consequences follow for a sub-directory run. First, root-level documents outside `<path>` are not audited at all. Second, crosslink and master-document invariants (AES604) resolve against `<path>` — a crate directory has no sibling `PRD.md`/`ROADMAP.md`, so the invariants that look for them behave as they would in a workspace that has none. A whole-workspace audit is the intended default; a sub-directory run is a narrower, self-contained audit, not a filtered view of the full one.

## Functional Requirements

### FR-DOC-001: FR Format, Protocol-Class Parity, and API-Method Existence (AES601)

- **Description**: Every requirement heading states its feature prefix and all six contract fields, an FRD declares exactly as many requirements as its feature's contract module declares capability seams, and every method its API tables promise is one the contract module declares.
- **Input**: An `FRD.md` or `DATA.md` inside the audit context, plus the audit root for resolving the feature's shared contract module.
- **Output**: A list of `DocFinding` values, each carrying `AES601` and one of `id_missing_feature_prefix`, `field_missing`, `protocol_count_mismatch`, or `api_method_not_found`.
- **Business Rules**:
  - A requirement ID must read `FR-<FEATURE>-NNN: <imperative name>`; a bare `FR-NNN` reports `id_missing_feature_prefix`.
  - The same shared FR-ID pattern drives the ID check, the field check, and the parity counter, so an accepted ID set cannot drift between them.
  - Every requirement states all six fields: Description, Input, Output, Business Rules, Edge Cases, Error Handling.
  - One protocol class is one capability seam and one seam is one requirement, so the FRD's requirement count must equal the feature's `I*Protocol` count; aggregate traits are not seams and are excluded.
  - The mismatch message names both counts and both fix directions.
  - Each row of the `Protocol API` and `Aggregate API` tables promises a method to an integrator; a method no protocol or aggregate trait declares reports `api_method_not_found`.
  - The method finding names both remedies: delete the promise from the table, or declare the method in the contract module.
  - Only the first cell of a table row is a promise. A method named in a row's prose or in a body paragraph is not one, so a doc that discusses a name the code never declares stays silent.
  - The finding anchors to its own table row's line, not to the enclosing heading, so a report points at the row a reader has to fix.
  - The check applies to `FRD.md` and `DATA.md`; parity and the method check apply to `FRD.md` alone, because only an FRD names a feature's contract module.
- **Edge Cases**: A feature crate whose name uses `-` is resolved to the `_` spelling of its shared module. A feature with no shared contract module cannot mismatch and cannot owe a method, so both checks stay silent. A method declared on any trait in the module satisfies a row, so a seam's declaration and the aggregate's `execute` are read the same way.
- **Error Handling**: An unreadable contract module yields no parity or method finding rather than a false violation. The parity finding is anchored to the requirements section, falling back to the first line when the document has none; a method finding is anchored to its own row.

### FR-DOC-002: Section Structure (AES602)

- **Description**: A requirement document reads in template order, and every section the template mandates carries the body shape the template promises.
- **Input**: An `FRD.md` or `DATA.md` inside the audit context, parsed into its level-1 and level-2 sections once per document.
- **Output**: A list of `DocFinding` values, each carrying `AES602` and one of `order_violation`, `api_no_subsection`, `api_h3_unexpected`, `api_subsection_no_table`, `api_subsection_duplicated`, `integration_not_table`, `nfr_not_table`, `scenarios_empty`, or `glossary_empty`.
- **Business Rules**:
  - Sections must appear in the FRD template order; a reordering reports `order_violation` anchored to the first out-of-order section.
  - API Contract carries exactly `### Protocol API` then `### Aggregate API`, once each, in that order, and no other level-3 heading.
  - Each of the two API Contract subsections must own its own column-complete table; describing the seam in prose is not an answer.
  - Integration Points and Non-functional Requirements must be tables with their required columns.
  - Test Scenarios and Glossary must each carry at least one bullet item.
  - The section tree is parsed once per document and read by all six checks, because re-parsing per check was the cost this rule used to pay.
  - The full shape applies to `FRD.md`; `DATA.md` is held to the order rule only.
- **Edge Cases**: A document with no API Contract section is silent on the API subsections rather than reported for a section the template left optional. A duplicated subsection is reported once per extra occurrence.
- **Error Handling**: A document whose parse yields no sections produces no findings; an unparseable document is a parse skip, never a violation.

### FR-DOC-003: Spec Purity (AES603)

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

## API Contract

### Protocol API

| Method | Input | Output | Error | Event | Description |
|--------|-------|--------|-------|-------|-------------|
| `audit_fr_format` | `DocAuditContext` | `Vec<DocFinding>` | — | — | FR-ID format, FR fields, FR/protocol-class parity, and promised-API-method existence (AES601). |
| `audit_section_structure` | `DocAuditContext` | `Vec<DocFinding>` | — | — | Template section order and mandated section shape (AES602). |
| `audit_spec_purity` | `DocAuditContext` | `Vec<DocFinding>` | — | — | Status leaks and source-file names in a specification (AES603). |
| `audit_crosslinks` | `DocAuditContext` | `Vec<DocFinding>` | — | — | Reference crosslinks and state-vocabulary placement (AES604). |
| `audit_doc_heading` | `DocAuditContext` | `Vec<DocFinding>` | — | — | H1 count and closed H2 set per document template (AES605). |

### Aggregate API

| Method | Input | Output | Error | Event | Description |
|--------|-------|--------|-------|-------|-------------|
| `execute` | `DocRequest` | `DocResponse` | — | — | Walks the document chain once, fans out to the five protocol seams, and returns the merged, deduplicated, stable-sorted findings. |

## Integration Points

| System | Direction | Purpose | Failure mode |
|--------|-----------|---------|--------------|
| `dispatcher` | in | Aggregates the doc audit and surfaces it through the `docs` subcommand | A missing dispatcher hook drops the entire doc group silently |
| `filesystem` | in | Supplies file entries for all `.md` documents under the workspace root | File-walk failures produce no findings rather than crashing the audit |
| `shared::taxonomy` | in | Supplies violation types, rule codes, heading contracts, and document name constants | A missing constant causes a compile-time failure at the aggregate boundary |

## Non-functional Requirements

| Metric | Target | Measurement method |
|--------|--------|---------------------|
| Read scope | Doc rules read the documents under audit plus, for the parity check only, the audited feature's shared contract module, read-only | Run a scan and confirm the group opens only `.md` files and the contract source files in `crates/shared/src/<module>/` |
| Finding precision | Every finding names a file and a line number | Invoke the docs command and confirm each finding resolves to a line |
| Schema completeness | Every finding carries a 1-based `line` (0 for document-level findings), a HIGH `severity` per RULES_AES, and maps to a shared `ViolationItem` | Call `DocFinding::to_violation_item` and confirm the `line` and `severity` fields survive the JSON round trip |
| Determinism | Two runs over the same tree report the same findings in the same order | Run the docs command twice and compare the outputs |

## Test Scenarios

- A document misses a mandatory H2 → AES605 fires naming the missing headings.
- Requirement count drifts from the contract seams → AES601 fires stating both counts and both fix directions.
- An FRD carries a bare FR-ID without a feature prefix → AES601 fires naming the line and missing prefix.
- An FRD requirement is missing a required field → AES601 fires naming the field and the line.
- An FRD API table promises a method no protocol or aggregate trait declares → AES601 fires naming the method, the table, and both remedies.
- A spec names a concrete source-file path → AES603 fires naming the line and the reference.
- A spec carries a status leak (checkbox item, an implementation-state claim, or a progress percentage) → AES603 fires naming the pattern.
- An FRD omits its BACKLOG.md link in Reference → AES604 fires.
- An FRD omits its PRD.md link in Reference → AES604 fires.
- An FRD's line-anchored finding maps to a `ViolationItem` with matching `line` and `severity` → `DocFinding::to_violation_item` preserves both fields through the JSON/SARIF round trip.
- A feature backlog restates a root-state section → AES604 fires.
- A conforming workspace produces 0 doc findings on `lint-arwaky-cli docs .` → exit code 0.
- `docs` run against a single crate directory → only that directory's documents are audited, and the crosslink invariants resolve against that directory as the root, not against the workspace root.
- `docs` run against a directory holding no recognized document → 0 findings and exit code 0, the same as a clean audit; the target being empty is not itself a finding.
- Each of the five protocol seams has its own capability, injected into the agent → the aggregate reaches all five rules through one `execute` call, and no capability is wired directly into the agent.
- A document that violates two rules at once → both rule codes appear in one response, proving the agent merges rather than short-circuits on the first capability that reports.

## Assumptions & Constraints

- Document recognition is limited to the known set (`FRD.md`, `BACKLOG.md`, `PRD.md`, `ROADMAP.md`, `README.md`, `AGENTS.md`, `ARCHITECTURE.md`, `CONTRIBUTING.md`). Unknown documents are ignored.
- H2 matching is case-insensitive and uses leading-word comparison, so `## API Contract` and `## API contract:` both match.
- The FR/protocol parity check and the API-method check only apply to features that have a corresponding shared contract module (`crates/shared/src/<feature-module>/`).
- Aggregate traits are always excluded from the protocol class count regardless of naming.
- One protocol class is one capability seam is one requirement: the five doc rules declare five seams, so this document declares five requirements and the two counts stay equal.
- The document walk is shared utility work, not agent work: an agent coordinates in-memory protocols and may not touch the filesystem.
- Fenced code blocks are stripped before H1 counting so shell comments like `# Tests (matches CI "Tests" job)` do not count as headings.
- The root master document is `ROADMAP.md`; a legacy root `BACKLOG.md` is accepted during migration.

## Glossary

- **Spec document**: A promise-bearing document (FRD, PRD, ROADMAP) that must be stateless and not name source files.
- **Status document**: A report-bearing document (BACKLOG, ROADMAP) that records real workspace condition with re-runnable evidence.
- **Feature folder**: A directory under `crates/`, `modules/`, or `packages/` that contains an `agent_*_orchestrator` file plus its doc pair.
- **Protocol class**: A capability-seam trait declared as `pub trait I*Protocol: Send + Sync`.
- **Audit context**: The shared per-run state every capability receives — the recognised documents, the audit root, and the root master text.
- **Aggregate trait**: A composite entry-point trait (e.g., `IScannerAggregate`) that is not a capability seam.
- **Parse skip**: A file that cannot be read or parsed is silently excluded; it is never counted as a violation.
- **State vocabulary**: Sections that define `State`, `Health`, or progress markers and belong only in the root master document.
