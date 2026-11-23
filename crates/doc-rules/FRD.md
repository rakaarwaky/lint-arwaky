# FRD — Doc Rules

## Feature Overview

[lint-arwaky-cli docs .] --> [DocOrchestrator] --> [ten per-doc capabilities]
[per-doc capability] --> [shared doc-rules layer]

The **Doc Rules** feature audits every recognized Markdown document against AES601–AES610 and emits one finding per violation. `lint-arwaky-cli docs <root>` discovers the document chain once, hands the same audit context to each capability, merges their findings, and stamps every finding with a machine-readable `violation_type`. The feature owns zero implementation detail — it is a spec layer, not a build layer.

Each functional requirement below implements exactly one capability behind one AES code, so an FR ID maps one-for-one to an AES rule code.

**Severity.** Every finding carries a `Severity` chosen from its `violation_type` via `severity_for_violation`. The mapping keeps the ARCHITECTURE.md §19 HIGH default and marks structural drift MEDIUM:

| Violation kind | Severity | Rationale |
|----------------|----------|-----------|
| `line_drift`, `h1_count`, `h2_missing`, `h2_unexpected` | MEDIUM | Template drift is a structural gap, not an architectural-policy breach. |
| `frontmatter_missing`, `missing_required_doc`, `missing_member_doc`, `forbidden_member_doc` | HIGH | A document the workspace contract requires is absent, misplaced, or unparseable; the document chain is broken. |
| every other `violation_type` (spec purity, ID format, crosslinks, table shape) | HIGH | ARCHITECTURE.md §19 default: spec purity and ID alignment are architectural policy. |

**Whole-document position convention.** Findings whose scope is the whole document (crosslink misses, frontmatter, h1/h2 sweep, line drift) use line `0` in the `DocFinding` position. The text formatter renders location as `doc` when `line <= 0` and as `doc:line` when `line > 0`; the `:0` suffix is suppressed. Messages that name a specific line offset omit a redundant `line N` phrase because the position field already carries it.

**Config and strict_override.** `DocAuditContext` accepts an optional `DocRulesConfig` and a `StrictModeOverride`. The resolved behavior:

| Field | Status | Effect |
|-------|--------|--------|
| `strict_member_docs` | Active | When true (or when `StrictModeOverride::ForceStrict` is set), the member-doc check requires every workspace member to carry its full document pair; a missing member doc fires `missing_member_doc`. |
| `strict_placeholders` | Future | Parsed from the `docs:` YAML block and threaded through `DocAuditContext::effective_strict_placeholders`, but no capability consumes it yet. Held for a future template-placeholder sweep. |

`StrictModeOverride::ForceStrict` forces both flags true regardless of the YAML; `UseConfigFile` defers to the parsed `docs:` block.

**Violation vocabulary.** The full violation-type vocabulary is defined once in the doc-rules taxonomy constants and consumed through `DOC_VIOLATION_NAMES`. The AES6xx doc-rules capabilities emit only the subset below; the AESD0x rule families (FR_FORMAT, SECTION_STRUCTURE, SPEC_PURITY, CROSSLINKS, DOC_STRUCTURE, TEMPLATE_CONFORMANCE) share the same constants.

| Violation type | AES code(s) | Meaning |
|----------------|-------------|---------|
| `line_drift` | AES601, AES603 | A line diverges from the canonical template. |
| `h1_count` | AES601–AES610 | The document has zero or multiple H1 headings. |
| `h2_missing` | AES601–AES610 | A required H2 section is absent. |
| `h2_unexpected` | AES601, AES604 | An H2 outside the template's required+allowed set appears. |
| `frontmatter_missing` | AES601 | AGENTS.md does not open with `---` YAML frontmatter. |
| `table_missing_columns` | AES602, AES604 | A fixed-shape table lacks a required column. |
| `table_extra_columns` | AES602, AES604 | A fixed-shape table carries an undeclared column. |
| `table_header_repeated` | AES602, AES604 | A table body repeats its header row as a data row. |
| `section_not_table` | AES602, AES604 | A section that must be a table carries prose or bullets. |
| `entry_missing_row` | AES601, AES606, AES609 | A fixed-row table is missing a required row label. |
| `table_not_in_template` | AES610 | A table appears under a section the template does not mandate. |
| `h3_unexpected` | AES610 | A Design Requirements H3 does not match `DS-<NAME>-NNN`. |
| `id_missing_feature_prefix` | AES608, AES609 | An ID heading lacks its feature prefix. |
| `source_file_named` | AES602, AES603, AES605, AES608, AES609 | Spec prose names a concrete source file with a language extension. |
| `status_leak` | AES602, AES605 | Spec prose leaks build status vocabulary. |
| `pr_roadmap_id_missing` | AES606 | A PRD PR-ID has no ROADMAP Roadmap Status entry. |
| `roadmap_pr_id_not_pr` | AES606 | A ROADMAP PR-ID is not declared in the PRD. |
| `backlog_id_not_fr` | AES607 | A BACKLOG row ID does not match its paired spec's ID family. |
| `backlog_id_missing` | AES607 | A declared spec ID has no BACKLOG row. |
| `backlog_qualifier_mismatch` | AES607 | A BACKLOG entry token does not match any declared spec ID. |
| `missing_project_requirements` | AES605 | PRD.md lacks a Project Requirements section. |
| `missing_required_doc` | orchestrator | A required chain document is absent from the workspace root. |
| `missing_member_doc` | orchestrator (strict) | A workspace member lacks its required document pair under `strict_member_docs`. |

**Glossary.**

| Term | Definition |
|------|------------|
| **AES code** | A rule identifier `AES6xx` scoped to one document type; each FR-DOC-NNN maps one-for-one to one AES6xx code. |
| **Canonical template** | The reference `template.md` under the shared templates directory that a document's lines are drift-checked against; compiled into the doc-rules constants at build time. |
| **Document chain** | The fixed set of root-level Markdown artifacts (`AGENTS.md`, `README.md`, `ARCHITECTURE.md`, `CONTRIBUTING.md`, `PRD.md`, `ROADMAP.md`, `BACKLOG.md`, `FRD.md`, `DATA.md`, `DESIGN.md`) the doc-rules audit walks. |
| **Drift check** | The line-by-line comparison of a document body against its canonical template's required lines. |
| **Finding** | One `DocFinding` value carrying `code`, `violation_type`, `doc`, `line`, `severity`, `why`, and `fix`. |
| **Heading defect** | A structural heading problem (`h1_count`, `h2_missing`, `h2_unexpected`, `h3_unexpected`) produced by the heading detector. |
| **Member doc** | A non-root document (e.g. a crate-local `README.md` or `BACKLOG.md`) whose required-pair enforcement is gated on `strict_member_docs`. |
| **PR-ID parity** | The cross-document invariant that every `PR-<PROJECT>-NNN` in `PRD.md` has a matching `ROADMAP.md` Roadmap Status entry and vice versa. |
| **Required link** | A crosslink a document must carry in its Reference section; the set is defined by the required-links helper per document type. |
| **Severity** | The `Severity` enum value (`INFO`, `LOW`, `MEDIUM`, `HIGH`, `CRITICAL`) a finding carries; chosen by `severity_for_violation` from its `violation_type`. |
| **Stamp** | The act of overwriting a finding's `doc` field with the relative path of the document currently being audited; performed once per document via `context.stamp(document, &mut document_findings)`. |
| **Strict mode** | A `StrictModeOverride` (`UseConfigFile` or `ForceStrict`) that resolves `strict_member_docs` and `strict_placeholders` into effective booleans on `DocAuditContext`. |
| **Violation type** | The machine-readable subtype string (e.g. `line_drift`, `source_file_named`) a finding carries in `violation_type`; one of the constants in the doc-rules taxonomy. |

## Functional Requirements

### FR-DOC-001: Audit AGENTS.md (AES601)

**Description:** The `docs` command shall audit `AGENTS.md` (AES601) and report a finding whenever any line drifts from the canonical template, or when an H2 heading falls outside the template's required set.

#### Business Logic

- Shall require the Runtime, Project Structure, Commands, and Reference sections verbatim
- Shall name the AES 7-layer naming convention and pinned Rust edition 2024 toolchain
- Shall list every CI gate command with scope and CI job
- Shall link to `README.md`, `CONTRIBUTING.md`, `ARCHITECTURE.md`, `ROADMAP.md`, `PRD.md` in Reference
- Shall open with a `---` delimited YAML frontmatter block carrying at least the `title`, `updated`, and `version` keys; the block must be the very first line of the file
- Shall drift-check every line against the canonical template at `crates/shared/template/agents/template.md` (compiled into `consts::AGENTS_TEMPLATE_LINES`)

#### Edge Cases

- A line diverging from the canonical template fires `line_drift`
- An H2 not in the template fires `h2_unexpected`
- An empty Runtime section fires `empty_section`
- `AGENTS.md` missing the `---` YAML frontmatter block fires `frontmatter_missing`

#### Technical Specification

| Aspect | Detail |
|--------|--------|
| **Input** | `AGENTS.md` document text and its file path |
| **Output** | `Vec<DocFinding>` for line-drift and heading violations |
| **Protocol** | `IAgentsDocProtocol` |
| **Method** | `audit_agents` |
| **Event** | `DocAudited { doc, findings }` |
| **Error** | `LINE_DRIFT`, `H2_UNEXPECTED`; violations `line_drift`, `h2_unexpected`, `empty_section`, `frontmatter_missing` |

#### Test

| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| `AGENTS.md` line diverges from the template | The audit checks line drift | A `line_drift` finding fires |
| `AGENTS.md` carries an off-template H2 | The audit checks heading structure | An `h2_unexpected` finding fires |
| `AGENTS.md` opens without a `---` YAML frontmatter block | The audit checks the first line | A `frontmatter_missing` finding fires |

#### Metric

| Target Metric | Value |
|---------------|-------|
| AGENTS template line-drift violations | 0 |
| Off-template H2 headings | 0 |
| `AGENTS.md` files missing YAML frontmatter | 0 |

### FR-DOC-002: Audit README.md (AES602)

**Description:** The `docs` command shall audit `README.md` (AES602) and report a finding whenever its `Quick Start` table lacks the fixed `Step | Action | Command` columns, or when spec prose names a concrete source file or leaks build status.

#### Business Logic

- Shall require a `Quick Start` table with columns `Step`, `Action`, `Command`
- Shall describe what the tool does, every command, and how to install
- Shall reject spec prose that names a source file with a language extension
- Shall link to `ARCHITECTURE.md`, `TEST.md`, `CONTRIBUTING.md`, `LICENSE` in Reference

#### Edge Cases

- Quick Start table missing `Command` fires `table_missing_columns`
- Prose that names a source file with a language extension fires `source_file_named`
- Prose stating "released as vX.Y.Z" fires `status_leak`

#### Technical Specification

| Aspect | Detail |
|--------|--------|
| **Input** | `README.md` document text and its file path |
| **Output** | `Vec<DocFinding>` for table-shape and purity violations |
| **Protocol** | `IReadmeDocProtocol` |
| **Method** | `audit_readme` |
| **Event** | `DocAudited { doc, findings }` |
| **Error** | `TABLE_MISSING_COLUMNS`, `STATUS_LEAK`; violations `table_missing_columns`, `source_file_named`, `status_leak` |

#### Test

| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| Quick Start table lacks `Command` column | The audit checks table shape | A `table_missing_columns` finding fires |
| README prose names a source file with a language extension | The audit scans `README.md` | A `source_file_named` finding fires |

#### Metric

| Target Metric | Value |
|---------------|-------|
| Quick Start table missing-column violations | 0 |
| Status vocabulary leaks in README prose | 0 |
| Source-file references in README prose | 0 |

### FR-DOC-003: Audit ARCHITECTURE.md (AES603)

**Description:** The `docs` command shall audit `ARCHITECTURE.md` (AES603) and report a finding whenever a required H2 section is missing, or when spec prose names a concrete source file.

#### Business Logic

- Shall define the 7-layer AES architecture: Taxonomy, Contract, Utility, Capabilities, Agent, Surface, Root
- Shall give the import matrix with allowed, mandatory, forbidden dependencies per scope
- Shall list member types (feature, surface, shared) and their document pairs
- Shall link to `PRD.md` and `ROADMAP.md` in Reference

#### Edge Cases

- Missing required H2 fires `h2_missing`
- Prose that names a source file with a language extension fires `source_file_named`

#### Technical Specification

| Aspect | Detail |
|--------|--------|
| **Input** | `ARCHITECTURE.md` document text and its file path |
| **Output** | `Vec<DocFinding>` for heading-structure and purity violations |
| **Protocol** | `IArchitectureDocProtocol` |
| **Method** | `audit_architecture` |
| **Event** | `DocAudited { doc, findings }` |
| **Error** | `H2_MISSING`, `LINE_DRIFT`; violations `h2_missing`, `source_file_named`, `line_drift` |

#### Test

| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| `ARCHITECTURE.md` lacks a required H2 section | The audit checks heading structure | An `h2_missing` finding fires |
| `ARCHITECTURE.md` names a concrete source file | The audit scans `ARCHITECTURE.md` | A `source_file_named` finding fires |

#### Metric

| Target Metric | Value |
|---------------|-------|
| Missing required H2 sections | 0 |
| Source-file references in architecture prose | 0 |
| Line-drift violations against the ARCHITECTURE template | 0 |

### FR-DOC-004: Audit CONTRIBUTING.md (AES604)

**Description:** The `docs` command shall audit `CONTRIBUTING.md` (AES604) and report a finding whenever its `Workflow` table lacks fixed columns, or when an H2 falls outside the template.

#### Business Logic

- Shall define worktree discipline, no-bypass fixes, and test-first changes
- Shall give a commit convention table with Type, Usage, Example
- Shall carry Workflow and Commit Convention tables with fixed columns
- Shall link to `PRD.md` and `ARCHITECTURE.md` in Reference

#### Edge Cases

- Workflow table missing `Detail` fires `table_missing_columns`
- Off-template H2 fires `h2_unexpected`

#### Technical Specification

| Aspect | Detail |
|--------|--------|
| **Input** | `CONTRIBUTING.md` document text and its file path |
| **Output** | `Vec<DocFinding>` for table-shape and heading violations |
| **Protocol** | `IContributingDocProtocol` |
| **Method** | `audit_contributing` |
| **Event** | `DocAudited { doc, findings }` |
| **Error** | `TABLE_MISSING_COLUMNS`, `H2_UNEXPECTED`; violations `table_missing_columns`, `h2_unexpected` |

#### Test

| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| Workflow table lacks `Detail` column | The audit checks table shape | A `table_missing_columns` finding fires |
| `CONTRIBUTING.md` has an off-template H2 | The audit checks heading structure | An `h2_unexpected` finding fires |

#### Metric

| Target Metric | Value |
|---------------|-------|
| Workflow/Commit table missing-column violations | 0 |
| Off-template H2 headings | 0 |

### FR-DOC-005: Audit PRD.md (AES605)

**Description:** The `docs` command shall audit `PRD.md` (AES605) and report a finding whenever its `Project Requirements` section is missing, when its spec prose names a concrete source file, or when it leaks build status vocabulary.

#### Business Logic

- Shall require a `Project Requirements` section stating business goals, user stories, and scope
- Shall reject spec prose that names a source file with a language extension
- Shall reject status vocabulary (shipped, backlog, in progress, done) in requirement statements
- Shall link to `ROADMAP.md` and `ARCHITECTURE.md` in its Reference section

#### Edge Cases

- `PRD.md` without `Project Requirements` fires `missing_project_requirements`
- `PRD.md` scope prose that names a source file with a language extension fires `source_file_named`
- `PRD.md` stating a release version in requirement prose fires `status_leak`

#### Technical Specification

| Aspect | Detail |
|--------|--------|
| **Input** | `PRD.md` document text and its file path |
| **Output** | `Vec<DocFinding>` with `violation_type` per rule breached |
| **Protocol** | `IPrdDocProtocol` |
| **Method** | `audit_prd` |
| **Event** | `DocAudited { doc, findings }` |
| **Error** | `MISSING_PROJECT_REQUIREMENTS`, `STATUS_LEAK`, `SOURCE_FILE_NAMED`; violations `missing_project_requirements`, `source_file_named`, `status_leak` |

#### Test

| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| `PRD.md` lacks `Project Requirements` | The audit runs `audit_prd` | A `missing_project_requirements` finding fires |
| `PRD.md` names a source file with a language extension in scope prose | The audit scans `PRD.md` | A `source_file_named` finding fires |
| `PRD.md` states a release version in a requirement | The audit scans `PRD.md` | A `status_leak` finding fires |

#### Metric

| Target Metric | Value |
|---------------|-------|
| `PRD.md` missing `Project Requirements` | 0 |
| `source_file_named` findings from `PRD.md` | 0 |
| `status_leak` findings from `PRD.md` | 0 |
| Off-template H2 headings in `PRD.md` | 0 |

### FR-DOC-006: Audit ROADMAP.md (AES606)

**Description:** The `docs` command shall audit `ROADMAP.md` (AES606) and report a finding whenever a `Roadmap Status` entry declares an ID with no matching `PR-<PROJECT>-NNN` in `PRD.md`, or when `PRD.md` declares a PR-ID with no matching roadmap entry.

#### Business Logic

- Shall require every `Roadmap Status` entry to carry Priority, State, Dependencies, Updated
- Shall enforce PR-ID parity between `PRD.md` `Project Requirements` and `ROADMAP.md` `Roadmap Status`
- Shall link to `PRD.md` and `ARCHITECTURE.md` in its Reference section

#### Edge Cases

- ROADMAP entry `PR-<PROJECT>-NNN` with no matching entry in PRD fires `pr_roadmap_id_missing`
- PRD declares `PR-<PROJECT>-NNN` with no roadmap entry fires `roadmap_pr_id_not_pr`
- Roadmap row missing `State` fires `entry_missing_row`

#### Technical Specification

| Aspect | Detail |
|--------|--------|
| **Input** | `ROADMAP.md` and `PRD.md` document texts |
| **Output** | `Vec<DocFinding>` for parity and row-shape violations |
| **Protocol** | `IRoadmapDocProtocol` |
| **Method** | `audit_roadmap` |
| **Event** | `DocAudited { doc, findings }` |
| **Error** | `PR_ROADMAP_ID_MISSING`, `ROADMAP_PR_ID_NOT_PR`; violations `pr_roadmap_id_missing`, `roadmap_pr_id_not_pr`, `entry_missing_row` |

#### Test

| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| PRD declares `PR-<PROJECT>-NNN` but ROADMAP has no entry | The audit runs parity check | A `pr_roadmap_id_missing` finding fires |
| ROADMAP has `PR-<PROJECT>-MMM` not in PRD | The audit runs parity check | A `roadmap_pr_id_not_pr` finding fires |

#### Metric

| Target Metric | Value |
|---------------|-------|
| PRD-to-ROADMAP PR-ID parity violations | 0 |
| ROADMAP entries without a PRD declaration | 0 |
| ROADMAP rows missing a required column | 0 |

### FR-DOC-007: Audit BACKLOG.md (AES607)

**Description:** The `docs` command shall audit `BACKLOG.md` (AES607) and report a finding whenever a `## Backlog` row's ID cell is not a valid `FR-<FEATURE>-NNN` ID, or when a declared FR ID has no matching backlog row.

#### Business Logic

- Shall require one `## Backlog` row per declared FR, DS, DT, or PR ID
- Shall require each row's ID cell to match its spec document's ID family
- Shall require State, Done, In Progress, Risk, Updated columns per row
- Shall link to `FRD.md` in its Reference section

#### Edge Cases

- Row ID `FR-DOC-001` with no feature prefix fires `backlog_id_not_fr`
- Declared `FR-DOC-009` with no matching backlog row fires `backlog_id_missing`

#### Technical Specification

| Aspect | Detail |
|--------|--------|
| **Input** | `BACKLOG.md` and its paired spec document (`FRD.md`, `DATA.md`, `DESIGN.md`) |
| **Output** | `Vec<DocFinding>` for ID-alignment violations |
| **Protocol** | `IBacklogDocProtocol` |
| **Method** | `audit_backlog` |
| **Event** | `DocAudited { doc, findings }` |
| **Error** | `BACKLOG_ID_NOT_FR`, `BACKLOG_ID_MISSING`; violations `backlog_id_not_fr`, `backlog_id_missing`, `backlog_qualifier_mismatch` |

#### Test

| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| `FRD.md` declares `FR-DOC-009` but `BACKLOG.md` has no row | The audit runs ID alignment | A `backlog_id_missing` finding fires |
| `BACKLOG.md` row ID reads `FR-DOC-001` | The audit scans `BACKLOG.md` | A `backlog_id_not_fr` finding fires |

#### Metric

| Target Metric | Value |
|---------------|-------|
| BACKLOG rows with mismatched ID family | 0 |
| Declared spec IDs missing a BACKLOG row | 0 |

### FR-DOC-008: Audit DATA.md (AES608)

**Description:** The `docs` command shall audit `DATA.md` (AES608) and report a finding whenever a data ID uses a bare `DT-NNN` without the `SHARED` qualifier, or when spec prose names a concrete source file.

#### Business Logic

- Shall require Data Overview, Data Requirements, and Reference sections
- Shall declare `DT-SHARED-NNN` items, one per data concept
- Shall keep data concepts free from technical and behavioural concerns
- Shall link to `PRD.md`, `BACKLOG.md`, `ROADMAP.md`, `ARCHITECTURE.md` in Reference

#### Edge Cases

- Heading `DT-001` without `SHARED` fires `id_missing_feature_prefix`
- Prose that names a source file with a language extension fires `source_file_named`

#### Technical Specification

| Aspect | Detail |
|--------|--------|
| **Input** | `DATA.md` document text and its file path |
| **Output** | `Vec<DocFinding>` for ID-format and purity violations |
| **Protocol** | `IDataDocProtocol` |
| **Method** | `audit_data` |
| **Event** | `DocAudited { doc, findings }` |
| **Error** | `ID_MISSING_FEATURE_PREFIX`, `SOURCE_FILE_NAMED`; violations `id_missing_feature_prefix`, `source_file_named` |

#### Test

| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| DATA heading reads `DT-001` with no `SHARED` qualifier | The audit checks ID format | An `id_missing_feature_prefix` finding fires |
| DATA prose names a struct or module file | The audit scans `DATA.md` | A `source_file_named` finding fires |

#### Metric

| Target Metric | Value |
|---------------|-------|
| DATA IDs missing the `SHARED` qualifier | 0 |
| Source-file references in data contract prose | 0 |
| Off-template H2 headings in `DATA.md` | 0 |

### FR-DOC-009: Audit FRD.md (AES609)

**Description:** The `docs` command shall audit each `FRD.md` entry (AES609) and report a finding whenever an FR heading lacks the `FR-<FEATURE>-NNN` prefix, when its `Technical Specification` table omits any of the six fixed rows, or when spec prose names a concrete source file.

#### Business Logic

- Shall require every FR heading to match `FR-<FEATURE-NAME-ID>-NNN: <name>`
- Shall require exactly six `Technical Specification` rows: Input, Output, Protocol, Method, Event, Error
- Shall reject spec prose that names a source file with a language extension
- Shall link to `PRD.md` and `BACKLOG.md` in its Reference section

#### Edge Cases

- Heading `FR-DOC-001: Do The Thing` fires `id_missing_feature_prefix`
- Technical Specification missing the `Event` row fires `entry_missing_row`
- Prose naming the orchestrator source file fires `source_file_named`

#### Technical Specification

| Aspect | Detail |
|--------|--------|
| **Input** | `FRD.md` document text and its file path |
| **Output** | `Vec<DocFinding>` for format and table-shape violations |
| **Protocol** | `IFrdDocProtocol` |
| **Method** | `audit_frd` |
| **Event** | `DocAudited { doc, findings }` |
| **Error** | `ID_MISSING_FEATURE_PREFIX`, `SOURCE_FILE_NAMED`; violations `id_missing_feature_prefix`, `entry_missing_row`, `source_file_named` |

#### Test

| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| Heading `FR-DOC-001: Do The Thing` with no feature prefix | The audit scans `FRD.md` | An `id_missing_feature_prefix` finding fires |
| Technical Specification omits `Event` | The audit checks entry tables | An `entry_missing_row` finding fires |

#### Metric

| Target Metric | Value |
|---------------|-------|
| FR-ID headings missing a feature prefix | 0 |
| Entry-table missing-row violations | 0 |
| Spec prose naming a source file | 0 |

### FR-DOC-010: Audit DESIGN.md (AES610)

**Description:** The `docs` command shall audit `DESIGN.md` (AES610) and report a finding whenever a `Design Requirements` H3 does not match the `DS-<NAME>-NNN` pattern, or when a table appears under a section the template does not mandate.

#### Business Logic

- Shall require Design Overview, Design Requirements, and Reference sections
- Shall sanction H3 headings only under Design Requirements, matching `DS-<NAME>-NNN`
- Shall link to `PRD.md`, `BACKLOG.md`, `ROADMAP.md`, `ARCHITECTURE.md` in Reference
- Shall permit no H3 under Design Overview or Reference

#### Edge Cases

- H3 `Tokens` under Design Requirements fires `h3_unexpected`
- A table under Design Requirements fires `table_not_in_template`

#### Technical Specification

| Aspect | Detail |
|--------|--------|
| **Input** | `DESIGN.md` document text and its file path |
| **Output** | `Vec<DocFinding>` for H3-sweep and table-conformance violations |
| **Protocol** | `IDesignDocProtocol` |
| **Method** | `audit_design` |
| **Event** | `DocAudited { doc, findings }` |
| **Error** | `H3_UNEXPECTED`, `TABLE_NOT_IN_TEMPLATE`; violations `h3_unexpected`, `table_not_in_template` |

#### Test

| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| Design Requirements H3 reads `Tokens` instead of `DS-<NAME>-NNN` | The audit runs the H3 sweep | An `h3_unexpected` finding fires |
| A table appears under Design Requirements | The audit checks template conformance | A `table_not_in_template` finding fires |

#### Metric

| Target Metric | Value |
|---------------|-------|
| Design Requirements H3s outside the DS pattern | 0 |
| Tables under sections the template does not mandate | 0 |

## Public Interface

| Aspect     | Detail            |
|------------|-------------------|
| Agent      | `DocOrchestrator` |
| Aggregate  | `IDocRunnerAggregate` |
| Method     | `execute`         |
| Request    | `DocRequest::AuditAll` |
| Response   | `DocResponse::Findings` |

The response carries `audited` (every document the orchestrator opened and checked, as workspace-relative paths) and `absent` (required chain documents the workspace contract expects but the root scan did not find). Callers that only need findings may destructure `{ findings, .. }`.

**Output samples.** One 80-column text-renderer sample per AES code. The renderer groups by member then file; each finding prints `[SEVERITY] (location)`, `CODE:violation_name`, `WHY: …`, and `FIX: …`.

```text
[HIGH] (AGENTS.md:42)
AES601:line_drift
WHY: AGENTS.md line 42 drifts from the canonical template.
FIX: Restore the canonical Runtime row from the template.

[MED] (AGENTS.md)
AES601:h2_unexpected
WHY: AGENTS.md carries an off-template H2 section the agent contract does not declare.
FIX: Remove the off-template H2 or move its content under a declared section.

[HIGH] (AGENTS.md)
AES601:frontmatter_missing
WHY: AGENTS.md must open with a `---` YAML frontmatter block carrying title, updated, and version.
FIX: Add a `---` YAML frontmatter block at the top of AGENTS.md.

[MED] (AGENTS.md)
AES601:h1_count
WHY: AGENTS.md must declare exactly one `# AGENTS` H1; none was found.
FIX: Add a single top-level `# AGENTS` heading near the top of the file.
```

```text
[HIGH] (README.md:88)
AES602:source_file_named
WHY: README.md prose names a concrete source file with a language extension; spec prose must stay implementation-agnostic.
FIX: Replace the source-file reference with a conceptual description of what the module does.

[HIGH] (README.md:95)
AES602:status_leak
WHY: README.md prose states a release version; spec prose must not leak build status vocabulary.
FIX: Remove the version statement; status belongs in ROADMAP.md, not README.md.

[HIGH] (README.md)
AES602:table_missing_columns
WHY: README.md Quick Start table must carry columns Step | Action | Command.
FIX: Add the missing Command column to the Quick Start table.
```

```text
[MED] (ARCHITECTURE.md)
AES603:h2_missing
WHY: ARCHITECTURE.md is missing a required H2 section the 7-layer contract mandates.
FIX: Add the missing H2 section with the template's required heading text.

[HIGH] (ARCHITECTURE.md:61)
AES603:source_file_named
WHY: ARCHITECTURE.md prose names a concrete source file with a language extension; the import matrix must stay path-agnostic.
FIX: Replace the source-file reference with the layer-and-crate name.
```

```text
[HIGH] (CONTRIBUTING.md:44)
AES604:table_missing_columns
WHY: CONTRIBUTING.md Workflow table must carry columns Detail.
FIX: Add the missing Detail column to the Workflow table.

[MED] (CONTRIBUTING.md)
AES604:h2_unexpected
WHY: CONTRIBUTING.md carries an off-template H2 section the workflow contract does not declare.
FIX: Remove the off-template H2 or fold its content under a declared section.
```

```text
[HIGH] (PRD.md:31)
AES605:source_file_named
WHY: PRD.md scope prose names a concrete source file with a language extension; requirements must stay implementation-agnostic.
FIX: Replace the source-file reference with a conceptual description.

[HIGH] (PRD.md:58)
AES605:status_leak
WHY: PRD.md requirement prose states a release version; requirements must not leak build status vocabulary.
FIX: Remove the version statement from the requirement; track status in ROADMAP.md.

[HIGH] (PRD.md)
AES605:missing_project_requirements
WHY: PRD.md must carry a Project Requirements section stating business goals, user stories, and scope.
FIX: Add a `## Project Requirements` section with the required subsections.
```

```text
[HIGH] (ROADMAP.md:27)
AES606:pr_roadmap_id_missing
WHY: PRD.md declares PR-<PROJECT>-NNN but ROADMAP.md has no matching Roadmap Status entry; parity is broken.
FIX: Add a Roadmap Status row for PR-<PROJECT>-NNN.

[HIGH] (ROADMAP.md:44)
AES606:roadmap_pr_id_not_pr
WHY: ROADMAP.md declares PR-<PROJECT>-MMM but PRD.md has no matching Project Requirements entry.
FIX: Remove the orphan ROADMAP row or add the PR-<PROJECT>-MMM entry to PRD.md.

[HIGH] (ROADMAP.md:52)
AES606:entry_missing_row
WHY: ROADMAP.md Roadmap Status row is missing the State column value.
FIX: Fill in the State cell for the row.
```

```text
[HIGH] (BACKLOG.md:19)
AES607:backlog_id_not_fr
WHY: BACKLOG.md row ID FR-DOC-001 does not match the paired spec document's ID family.
FIX: Correct the row ID to match the FRD.md feature prefix.

[HIGH] (BACKLOG.md)
AES607:backlog_id_missing
WHY: FRD.md declares FR-DOC-009 but BACKLOG.md has no matching row.
FIX: Add a `## Backlog` row for FR-DOC-009.
```

```text
[HIGH] (DATA.md:33)
AES608:id_missing_feature_prefix
WHY: DATA.md heading DT-001 lacks the SHARED qualifier; data IDs must read DT-SHARED-NNN.
FIX: Rename the heading to DT-SHARED-NNN.

[HIGH] (DATA.md:57)
AES608:source_file_named
WHY: DATA.md prose names a concrete source file with a language extension; data contracts must stay implementation-agnostic.
FIX: Replace the source-file reference with the data concept name.
```

```text
[HIGH] (FRD.md:74)
AES609:id_missing_feature_prefix
WHY: FRD.md heading FR-DOC-001: Do The Thing lacks the FR-<FEATURE> prefix.
FIX: Rename the heading to FR-DOC-001: <name>.

[HIGH] (FRD.md:92)
AES609:entry_missing_row
WHY: FRD.md Technical Specification table is missing the Event row.
FIX: Add the Event row to the Technical Specification table.

[HIGH] (FRD.md:120)
AES609:source_file_named
WHY: FRD.md prose names a concrete source file with a language extension; spec prose must stay implementation-agnostic.
FIX: Replace the source-file reference with the capability name.
```

```text
[MED] (DESIGN.md:48)
AES610:h3_unexpected
WHY: DESIGN.md Design Requirements carries an H3 that does not match the DS-<NAME>-NNN pattern.
FIX: Rename the H3 to match DS-<NAME>-NNN or remove it.

[HIGH] (DESIGN.md:72)
AES610:table_not_in_template
WHY: A table appears under a section the DESIGN template does not mandate.
FIX: Remove the table or move its content to a template-sanctioned section.
```

```text
Total: N violations
```

## Reference

| Artifact     | Purpose                                         | Link |
|--------------|-------------------------------------------------|------|
| PRD          | Business goals, user stories, and scope         | [PRD.md](../../PRD.md) |
| Backlog      | Task tracking, epics, and implementation status | [BACKLOG.md](BACKLOG.md) |
| Roadmap      | Timeline, phasing, and dependencies             | [ROADMAP.md](../../ROADMAP.md) |
| Architecture | System boundaries, tech stack, and infra        | [ARCHITECTURE.md](../../ARCHITECTURE.md) |
