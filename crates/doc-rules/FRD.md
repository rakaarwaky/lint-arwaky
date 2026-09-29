# Doc Rules — FRD

## Reference

- Backlog: [BACKLOG.md](BACKLOG.md) — real condition for this feature; this file is specification only.
- PRD: [PRD.md](../../PRD.md)
- Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)

## Requirements

| ID | Requirement |
|----|-------------|
| FR-DOC-001 | Every `.md` document satisfies the AES heading and section contract. |
| FR-DOC-002 | A feature folder carrying a doc pair also carries an orchestrator file. |
| FR-DOC-003 | Doc findings are reportable per document with line-level detail. |
| FR-DOC-004 | An FRD's requirement count equals its feature's count of capability-seam protocol classes. |

## Non-functional Requirements

| Metric | Target | Measurement method |
| --- | --- | --- |
| Read scope | Doc rules read the documents under audit plus, for the parity check only, the audited feature's shared contract module, read-only | Run a scan and confirm the group opens only `.md` files and `crates/shared/src/<module>/*.rs` |
| Finding precision | Every finding names a file and a line number | Invoke the docs command and confirm each finding resolves to a line |
| Determinism | Two runs over the same tree report the same findings in the same order | Run the docs command twice and compare the outputs |

## Scenarios

| Scenario | Given | When | Then |
|----------|-------|------|------|
| A document misses a mandatory H2 | A root document lacking a required section | `docs` runs | AES606 fires naming the missing headings. |
| A feature folder has a doc pair but no orchestrator | `FRD.md` + `BACKLOG.md` without an agent file | `docs` runs | AES605 fires naming the folder. |
| Requirement count drifts from the contract seams | An FRD with three requirements and a contract module with two protocol classes | `docs` runs | AES607 fires stating both counts and both fix directions. |

## Integration

The doc audit is aggregated by `dispatcher` and surfaced by the `docs` subcommand.

## API

| Symbol | Kind | Notes |
|--------|------|-------|
| `RootDocRulesContainer::orchestrator()` | Function | Composition root entry. |
| `DocAuditor::audit` | Method | `IDocAuditProtocol` implementation. |
