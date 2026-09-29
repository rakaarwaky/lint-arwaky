# Doc Rules — FRD

## Reference

- Backlog: [BACKLOG.md](BACKLOG.md) — real condition for this feature; this file is specification only.
- PRD: [PRD.md](../../PRD.md)
- Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)

## System Overview

The doc-rules crate enforces the AES document-invariant contract across the workspace. It audits every recognized `.md` document against seven rules (AES601–AES607): FR-ID format and field completeness, section structure shape, spec purity, crosslink integrity, feature-folder health, document heading structure, and FR/protocol class parity. Findings carry a machine-readable `violation_type` so consumers can route on them without parsing prose. The auditor walks the filesystem; no external tool is involved.

## Functional Requirements

### FR-DOC-001: Document Invariant Enforcement

- **Description**: Every `.md` document satisfies the AES heading and section contract.
- **Input**: A workspace root and the list of recognized document paths (`FRD.md`, `BACKLOG.md`, `PRD.md`, `ROADMAP.md`, `README.md`, `AGENTS.md`, `ARCHITECTURE.md`, `CONTRIBUTING.md`).
- **Output**: `Vec<LintResult>` carrying one finding per invariant violation.
- **Business Rules**:
  - AES601 validates FR-ID format and required fields in every FRD requirement block.
  - AES602 validates API Contract subsections, Integration Points table shape, NFR table shape, Test Scenarios bullets, and Glossary bullets.
  - AES603 prevents status leaks and source-file names in spec documents.
  - AES604 enforces Reference crosslinks and state-vocab restatement rules.
  - AES606 checks H1/H2 heading structure against per-document contracts.
  - AES607 verifies FRD requirement count equals protocol class count in the shared contract module.
- **Edge Cases**: Root-level legacy `BACKLOG.md` is accepted as a master document during migration. Feature backlogs are forbidden from restating root-state sections.
- **Error Handling**: Unreadable files produce no findings. Missing documents are skipped silently rather than flagged.

### FR-DOC-002: Audit Orchestration

- **Description**: The doc auditor is reachable through a single aggregate entry point that dispatches to the invariant checker.
- **Input**: A `DocRequest` describing the workspace root to audit.
- **Output**: A `DocResponse` carrying a deduplicated, stable-sorted list of findings.
- **Business Rules**: The aggregate orchestrates all invariants under one seam so consumers need only invoke `execute`.
- **Edge Cases**: A request with an empty root produces no findings rather than an error.
- **Error Handling**: Failures inside the checker are propagated as `DocResponse::Findings` with no partial results.

## API Contract

### Protocol API

| Method | Input | Output | Error | Event | Description |
|--------|-------|--------|-------|-------|-------------|
| `audit` | `DocRequest` | `DocResponse` | — | — | Single composite entry point over the doc-rules feature. |

### Aggregate API

| Method | Input | Output | Error | Event | Description |
|--------|-------|--------|-------|-------|-------------|
| `execute` | `DocRequest` | `DocResponse` | — | — | Routes through the capability checker and returns findings. |

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
| Determinism | Two runs over the same tree report the same findings in the same order | Run the docs command twice and compare the outputs |

## Test Scenarios

- A document misses a mandatory H2 → AES606 fires naming the missing headings.
- Requirement count drifts from the contract seams → AES607 fires stating both counts and both fix directions.
- An FRD carries a bare FR-ID without a feature prefix → AES601 fires naming the line and missing prefix.
- An FRD requirement is missing a required field → AES601 fires naming the field and the line.
- A spec names a concrete source-file path → AES603 fires naming the line and the reference.
- A spec carries a status leak (checkbox item, an implementation-state claim, or a progress percentage) → AES603 fires naming the pattern.
- An FRD omits its BACKLOG.md link in Reference → AES604 fires.
- An FRD omits its PRD.md link in Reference → AES604 fires.
- A feature backlog restates a root-state section → AES604 fires.
- A conforming workspace produces 0 doc findings on `lint-arwaky-cli docs .` → exit code 0.

## Assumptions & Constraints

- Document recognition is limited to the known set (`FRD.md`, `BACKLOG.md`, `PRD.md`, `ROADMAP.md`, `README.md`, `AGENTS.md`, `ARCHITECTURE.md`, `CONTRIBUTING.md`). Unknown documents are ignored.
- H2 matching is case-insensitive and uses leading-word comparison, so `## API Contract` and `## API contract:` both match.
- The FR/protocol parity check only applies to features that have a corresponding shared contract module (`crates/shared/src/<feature-module>/`).
- Aggregate traits are always excluded from the protocol class count regardless of naming.
- Fenced code blocks are stripped before H1 counting so shell comments like `# Tests (matches CI "Tests" job)` do not count as headings.
- The root master document is `ROADMAP.md`; a legacy root `BACKLOG.md` is accepted during migration.

## Glossary

- **Spec document**: A promise-bearing document (FRD, PRD, ROADMAP) that must be stateless and not name source files.
- **Status document**: A report-bearing document (BACKLOG, ROADMAP) that records real workspace condition with re-runnable evidence.
- **Feature folder**: A directory under `crates/`, `modules/`, or `packages/` that contains an `agent_*_orchestrator` file plus its doc pair.
- **Protocol class**: A capability-seam trait declared as `pub trait I*Protocol: Send + Sync`.
- **Aggregate trait**: A composite entry-point trait (e.g., `IScannerAggregate`) that is not a capability seam.
- **Parse skip**: A file that cannot be read or parsed is silently excluded; it is never counted as a violation.
- **State vocabulary**: Sections that define `State`, `Health`, or progress markers and belong only in the root master document.
