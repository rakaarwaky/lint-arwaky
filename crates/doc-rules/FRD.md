# FRD — Doc Rules

## Feature Overview

[lint-arwaky-cli docs .] --> [DocOrchestrator] --> [ten per-doc capabilities]
[per-doc capability] --> [shared doc-rules kernel]

The **Doc Rules** feature audits every recognized Markdown document against AES601–AES610 and emits one finding per violation. `lint-arwaky-cli docs <root>` discovers the document chain once, hands the same audit context to each capability, merges their findings, and stamps every finding with a machine-readable `violation_type`. The feature owns zero implementation detail — it is a spec layer, not a build layer.

Each functional requirement below implements exactly one capability behind one AES code, so an FR ID maps one-for-one to an AES rule code.

## Functional Requirements

### FR-DOC-001: Audit AGENTS.md (AES601)

**Description:** The `docs` command shall audit `AGENTS.md` (AES601) and report a finding whenever any line drifts from the canonical template, or when an H2 heading falls outside the template's required set.

#### Business Logic
- Shall require the Runtime, Project Structure, Commands, and Reference sections verbatim
- Shall name the AES 7-layer naming convention and pinned Rust edition 2024 toolchain
- Shall list every CI gate command with scope and CI job
- Shall link to `README.md`, `CONTRIBUTING.md`, `ARCHITECTURE.md`, `ROADMAP.md`, `PRD.md` in Reference

#### Edge Cases
- A line diverging from the canonical template fires `line_drift`
- An H2 not in the template fires `h2_unexpected`
- An empty Runtime section fires `empty_section`

#### Technical Specification

| Aspect | Detail |
|--------|--------|
| **Input** | `AGENTS.md` document text and its file path |
| **Output** | `Vec<DocFinding>` for line-drift and heading violations |
| **Protocol** | `IAgentsDocProtocol` |
| **Method** | `audit_agents` |
| **Event** | `AgentsAudited` |
| **Error** | `LINE_DRIFT`, `H2_UNEXPECTED` |

#### Test

| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| `AGENTS.md` line diverges from the template | The audit checks line drift | A `line_drift` finding fires |
| `AGENTS.md` carries an off-template H2 | The audit checks heading structure | An `h2_unexpected` finding fires |

#### Metric

| Target Metric | Value |
|---------------|-------|
| AGENTS template line-drift violations | 0 |
| Off-template H2 headings | 0 |

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
- Prose stating "released as v3.8.0" fires `status_leak`

#### Technical Specification

| Aspect | Detail |
|--------|--------|
| **Input** | `README.md` document text and its file path |
| **Output** | `Vec<DocFinding>` for table-shape and purity violations |
| **Protocol** | `IReadmeDocProtocol` |
| **Method** | `audit_readme` |
| **Event** | `ReadmeAudited` |
| **Error** | `TABLE_MISSING_COLUMNS`, `STATUS_LEAK` |

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
| **Event** | `ArchitectureAudited` |
| **Error** | `H2_MISSING`, `LINE_DRIFT` |

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
| **Event** | `ContributingAudited` |
| **Error** | `TABLE_MISSING_COLUMNS`, `H2_UNEXPECTED` |

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
- Shall reject spec prose that names a `.rs`, `.py`, `.ts`, or `.tsx` file
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
| **Event** | `PrdAudited` |
| **Error** | `MISSING_PROJECT_REQUIREMENTS`, `STATUS_LEAK`, `SOURCE_FILE_NAMED` |

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

### FR-DOC-006: Audit ROADMAP.md (AES606)

**Description:** The `docs` command shall audit `ROADMAP.md` (AES606) and report a finding whenever a `Roadmap Status` entry declares an ID with no matching `PR-<PROJECT>-NNN` in `PRD.md`, or when `PRD.md` declares a PR-ID with no matching roadmap entry.

#### Business Logic
- Shall require every `Roadmap Status` entry to carry Priority, State, Dependencies, Updated
- Shall enforce PR-ID parity between `PRD.md` `Project Requirements` and `ROADMAP.md` `Roadmap Status`
- Shall link to `PRD.md` and `ARCHITECTURE.md` in its Reference section

#### Edge Cases
- ROADMAP entry `PR-LA-001` with no `PR-LA-001` in PRD fires `pr_roadmap_id_missing`
- PRD declares `PR-LA-016` with no roadmap entry fires `roadmap_pr_id_not_pr`
- Roadmap row missing `State` fires `entry_missing_row`

#### Technical Specification

| Aspect | Detail |
|--------|--------|
| **Input** | `ROADMAP.md` and `PRD.md` document texts |
| **Output** | `Vec<DocFinding>` for parity and row-shape violations |
| **Protocol** | `IRoadmapDocProtocol` |
| **Method** | `audit_roadmap` |
| **Event** | `RoadmapAudited` |
| **Error** | `PR_ROADMAP_ID_MISSING`, `ROADMAP_PR_ID_NOT_PR` |

#### Test

| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| PRD declares `PR-LA-001` but ROADMAP has no entry | The audit runs parity check | A `pr_roadmap_id_missing` finding fires |
| ROADMAP has `PR-LA-099` not in PRD | The audit runs parity check | A `roadmap_pr_id_not_pr` finding fires |

#### Metric

| Target Metric | Value |
|---------------|-------|
| PRD-to-ROADMAP PR-ID parity violations | 0 |
| ROADMAP entries without a PRD declaration | 0 |

### FR-DOC-007: Audit BACKLOG.md (AES607)

**Description:** The `docs` command shall audit `BACKLOG.md` (AES607) and report a finding whenever a `## Backlog` row's ID cell is not a valid `FR-<FEATURE>-NNN` ID, or when a declared FR ID has no matching backlog row.

#### Business Logic
- Shall require one `## Backlog` row per declared FR, DS, DT, or PR ID
- Shall require each row's ID cell to match its spec document's ID family
- Shall require State, Done, In Progress, Risk, Updated columns per row
- Shall link to `FRD.md` in its Reference section

#### Edge Cases
- Row ID `FR-001` with no feature prefix fires `backlog_id_not_fr`
- Declared `FR-DOC-009` with no matching backlog row fires `backlog_id_missing`

#### Technical Specification

| Aspect | Detail |
|--------|--------|
| **Input** | `BACKLOG.md` and its paired spec document (`FRD.md`, `DATA.md`, `DESIGN.md`) |
| **Output** | `Vec<DocFinding>` for ID-alignment violations |
| **Protocol** | `IBacklogDocProtocol` |
| **Method** | `audit_backlog` |
| **Event** | `BacklogAudited` |
| **Error** | `BACKLOG_ID_NOT_FR`, `BACKLOG_ID_MISSING` |

#### Test

| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| `FRD.md` declares `FR-DOC-009` but `BACKLOG.md` has no row | The audit runs ID alignment | A `backlog_id_missing` finding fires |
| `BACKLOG.md` row ID reads `FR-001` | The audit scans `BACKLOG.md` | A `backlog_id_not_fr` finding fires |

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
| **Event** | `DataAudited` |
| **Error** | `ID_MISSING_FEATURE_PREFIX`, `SOURCE_FILE_NAMED` |

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

### FR-DOC-009: Audit FRD.md (AES609)

**Description:** The `docs` command shall audit each `FRD.md` entry (AES609) and report a finding whenever an FR heading lacks the `FR-<FEATURE>-NNN` prefix, when its `Technical Specification` table omits any of the six fixed rows, or when spec prose names a concrete source file.

#### Business Logic
- Shall require every FR heading to match `FR-<FEATURE-NAME-ID>-NNN: <name>`
- Shall require exactly six `Technical Specification` rows: Input, Output, Protocol, Method, Event, Error
- Shall reject spec prose naming `.rs`, `.py`, `.ts`, `.tsx` files
- Shall link to `PRD.md` and `BACKLOG.md` in its Reference section

#### Edge Cases
- Heading `FR-001: Do The Thing` fires `id_missing_feature_prefix`
- Technical Specification missing the `Event` row fires `entry_missing_row`
- Prose naming `agent_doc_orchestrator.rs` fires `source_file_named`

#### Technical Specification

| Aspect | Detail |
|--------|--------|
| **Input** | `FRD.md` document text and its file path |
| **Output** | `Vec<DocFinding>` for format and table-shape violations |
| **Protocol** | `IFrdDocProtocol` |
| **Method** | `audit_frd` |
| **Event** | `FrdAudited` |
| **Error** | `ID_MISSING_FEATURE_PREFIX`, `SOURCE_FILE_NAMED` |

#### Test

| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| Heading `FR-001: Do The Thing` with no feature prefix | The audit scans `FRD.md` | An `id_missing_feature_prefix` finding fires |
| Technical Specification omits `Event` | The audit checks entry tables | An `entry_missing_row` finding fires |

#### Metric

| Target Metric | Value |
|---------------|-------|
| FR-ID headings missing a feature prefix | 0 |
| Entry-table missing-row violations | 0 |

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
| **Event** | `DesignAudited` |
| **Error** | `H3_UNEXPECTED`, `TABLE_NOT_IN_TEMPLATE` |

#### Test

| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| Design Requirements H3 reads `Tokens` instead of `DS-UI-001` | The audit runs the H3 sweep | An `h3_unexpected` finding fires |
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

## Reference

| Artifact     | Purpose                                         | Link |
|--------------|-------------------------------------------------|------|
| PRD          | Business goals, user stories, and scope         | [PRD.md](../../PRD.md) |
| Backlog      | Task tracking, epics, and implementation status | [BACKLOG.md](BACKLOG.md) |
| Roadmap      | Timeline, phasing, and dependencies             | [ROADMAP.md](../../ROADMAP.md) |
| Architecture | System boundaries, tech stack, and infra        | [ARCHITECTURE.md](../../ARCHITECTURE.md) |
