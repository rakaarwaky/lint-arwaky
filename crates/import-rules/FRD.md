# FRD — import-rules

## Feature Overview

    [CLI `check` command] --> [import-rules aggregate] --> [import orchestrator]
    [import orchestrator] --> [forbidden, mandatory, unused, dummy, cycle checkers]
    [checkers] --> [lint violations]

The **import-rules** feature sits in the capabilities layer of the AES 7-layer architecture. It is invoked by the `check` command stage dedicated to import-rule auditing, and in turn calls the forbidden-layer checker, the mandatory-import checker, the unused-import checker, the dummy-constructor checker, and the circular-dependency analyzer. The feature performs zero I/O: all file discovery, content reading, and AST parsing are handled upstream by the filesystem aggregate, and the import orchestrator receives pre-fetched entries, imports, and used identifiers. All rule behavior is governed by YAML configuration; the crate makes no assumptions about allowed or forbidden dependencies beyond what the config declares.

## Functional Requirements

### FR-ImportRules-001: Layer Dependency Violation

**Description:** The import orchestrator shall reject every import whose source module and target module cross a forbidden layer boundary as declared in the config-driven dependency matrix. For each violating import it shall emit a finding whose code names the rule and whose location is the import's reported source line, not the internal import index.

#### Business Logic

- AES201 rejects a lower layer importing a higher one; agent, surface, capabilities, and taxonomy are each forbidden from specific importers, and utility remains allowed from taxonomy and capabilities.
- The entry's first path segment determines the layer; index fallback applies only when the entry's line number is zero.
- Both Python and Rust import sources report the original source line, not the internal import index.
- Every forbidden import in a file produces its own finding; no silent coalescing.

#### Edge Cases

- A forbidden import whose entry line is zero falls back to the import index and still produces a locatable finding.
- Allowed pairs (taxonomy importing utility, capabilities importing taxonomy or utility) produce zero AES201 findings.
- Multiple forbidden imports in one file each produce a separate finding; line extraction prefers the reported source line and degrades to the entry index only when line info is absent.

#### Technical Specification

| Aspect | Detail |
|--------|--------|
| **Input** | File path plus parsed import entries and optional file entries, delivered through the aggregate's audit request variants. |
| **Output** | Findings with code AES201, carrying the source module, target module, and source line number; returned inside the audit response. |
| **Protocol** | `IImportForbiddenProtocol` |
| **Method** | `check` |
| **Event** | None; audit is synchronous. |
| **Error** | An unreadable file contributes no AES201 findings rather than failing the audit; a zero entry line still yields a locatable finding. |

#### Test

| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| An agent-layer file imports a surface-layer module | The forbidden-layer check runs | One AES201 finding is reported at the import's source line |
| A taxonomy-layer file imports a utility-layer module | The forbidden-layer check runs | No AES201 finding is reported (allowed pair) |
| A forbidden import has an entry line of zero | The forbidden-layer check runs | An AES201 finding is reported using the import index as the location |
| A file contains two forbidden imports | The forbidden-layer check runs | Two AES201 findings are reported, one per import, each at its source line |

#### Metric

| Target Metric | Value |
|---------------|-------|
| AES201 finding rate for seeded forbidden pairs | 100% |
| AES201 false-positive rate for seeded allowed pairs | 0% |
| AES201 source-line accuracy for seeded imports | 100% |

### FR-ImportRules-002: Mandatory Layer Imports

**Description:** The import orchestrator shall require imports that a layer must carry to function, and report a finding whenever such a mandated dependency is absent from a classified file. Files outside the mandated scope shall be skipped.

#### Business Logic

- AES202 fires when a file classified into a layer that mandates a specific dependency does not contain that import.
- The check operates per file classification; files outside the mandated scope produce no AES202 finding.
- A file that already imports the mandated dependency produces no finding.

#### Edge Cases

- A clean fixture (all mandates satisfied) produces zero AES202 findings.
- A missing or empty input file contributes no AES202 findings rather than failing the audit.
- Unclassified files are ignored.

#### Technical Specification

| Aspect | Detail |
|--------|--------|
| **Input** | File path plus its parsed import list, delivered through the import audit pipeline. |
| **Output** | Findings with code AES202, naming the missing dependency; returned inside the audit response. |
| **Protocol** | `IImportMandatoryProtocol` |
| **Method** | `check` |
| **Event** | None; audit is synchronous. |
| **Error** | A missing or empty input file yields no AES202 findings for that file rather than failing the run. |

#### Test

| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| A file classified into a layer that mandates an import lacks that import | The mandatory-import check runs | One AES202 finding is reported naming the missing dependency |
| A file already imports the mandated dependency | The mandatory-import check runs | No AES202 finding is reported |
| A clean fixture satisfies all mandates | The mandatory-import check runs | Zero AES202 findings are reported |

#### Metric

| Target Metric | Value |
|---------------|-------|
| AES202 finding rate for seeded missing-mandate fixtures | 100% |
| AES202 false-positive rate for clean fixtures | 0% |

### FR-ImportRules-003: Unused Import Detection

**Description:** The import orchestrator shall flag imports whose symbol never appears in the file body outside the import statement itself. An aliased import is considered used only when referenced by its alias. Barrel and mod-re-export index files are excluded from this check.

#### Business Logic

- AES203 flags imports whose symbol never appears outside the import statement; Python plain imports use the first-segment binding.
- An aliased import must be referenced via its alias to count as used; usage of the original name does not count.
- Truly unused aliases are flagged.
- Barrel, mod-re-export, and index files are skipped entirely.
- Future-dated imports (imports that will become used after a pending edit) are never flagged.
- An empty file or a file with no imports yields no finding.

#### Edge Cases

- An actual unused import is flagged exactly once, even when multiple unused imports exist.
- An aliased Python import used only via its alias produces no finding; a truly unused alias is flagged.
- Barrel, mod-re-export, and index files produce no AES203 findings.
- A parse failure of the file body yields no AES203 findings for that file rather than false positives.

#### Technical Specification

| Aspect | Detail |
|--------|--------|
| **Input** | File path plus parsed import entries and used identifiers, delivered through the aggregate's audit request variants. |
| **Output** | Findings with code AES203, naming the unused binding and its source line; returned inside the audit response. |
| **Protocol** | `IUnusedImportProtocol` |
| **Method** | `check` |
| **Event** | None; audit is synchronous. |
| **Error** | A parse failure of the body yields no AES203 findings for that file instead of false positives. |

#### Test

| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| A file imports a symbol that is never referenced in the body | The unused-import check runs | One AES203 finding is reported for that import |
| A Python import is aliased and referenced only by its alias | The unused-import check runs | No AES203 finding is reported |
| A Python import is aliased and never referenced | The unused-import check runs | One AES203 finding is reported |
| A mod-re-export index file contains unused-looking symbols | The unused-import check runs | No AES203 findings are reported |
| A file contains two unused imports | The unused-import check runs | Two AES203 findings are reported, one per import |

#### Metric

| Target Metric | Value |
|---------------|-------|
| AES203 finding rate for seeded unused-import fixtures | 100% |
| AES203 false-positive rate for skipped-file families | 0% |
| AES203 false-positive rate for aliased-but-used imports | 0% |

### FR-ImportRules-004: Dummy Import Detection

**Description:** The import orchestrator shall catch dummy imports, dummy functions, and dummy trait implementations that exist only to satisfy a checker or a linter rather than to express real behavior.

#### Business Logic

- AES204 flags imports pointing at symbols that are never callably used.
- AES204 flags functions whose bodies are empty stubs.
- AES204 flags trait implementations with no real method bodies beyond placeholders.
- Clean files produce no findings.

#### Edge Cases

- A clean fixture yields zero AES204 findings.
- A file with a stub function still yields a finding for that stub.
- An unreadable file yields no AES204 findings.
- A mixed file with multiple dummy constructs reports each dummy independently.
- A parse failure skips that file's dummy analysis rather than failing the audit.

#### Technical Specification

| Aspect | Detail |
|--------|--------|
| **Input** | File path plus parsed content, delivered through the import audit pipeline. |
| **Output** | Findings with code AES204, naming the dummy construct and its location; returned inside the audit response. |
| **Protocol** | `IDummyImportCheckerProtocol` |
| **Method** | `check` |
| **Event** | None; audit is synchronous. |
| **Error** | A parse failure skips the file's dummy analysis rather than failing the audit. |

#### Test

| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| A clean file contains no dummy constructs | The dummy-check runs | Zero AES204 findings are reported |
| A file contains a stub function with an empty body | The dummy-check runs | One AES204 finding is reported for the stub |
| A file contains a trait implementation with placeholder method bodies | The dummy-check runs | One AES204 finding is reported for the dummy impl |
| An unreadable file is passed to the audit | The dummy-check runs | No AES204 findings are reported for that file; the audit completes |

#### Metric

| Target Metric | Value |
|---------------|-------|
| AES204 finding rate for seeded dummy-constructor fixtures | 100% |
| AES204 false-positive rate for clean fixtures | 0% |

### FR-ImportRules-005: Circular Dependency Detection

**Description:** The import orchestrator shall detect circular cross-layer dependencies among parsed import edges. Module paths are normalized to layer segments, a directed graph is built, and strongly connected components are reported as findings.

#### Business Logic

- AES205 normalizes module paths to layer segments, stripping the crate prefix when present.
- A directed graph is built from the normalized edges.
- Strongly connected components of the graph are reported as cycle findings.
- Two-node, three-node, and cross-layer mutual cycles are all detected.
- An acyclic graph reports no cycles.
- An audit disabled by configuration returns an empty result.

#### Edge Cases

- An empty edge set yields no cycle findings.
- Mutual cross-layer cycles are detected.
- Normalization with and without a crate prefix both resolve to the same layer.
- Unparseable edge entries are dropped; the scan still completes on the remaining edges.
- A disabled configuration returns an empty audit without failing.
- A missing entry in the graph does not cause a panic.

#### Technical Specification

| Aspect | Detail |
|--------|--------|
| **Input** | Import edge list (or file discovery followed by edges), delivered through the aggregate's audit request variants. |
| **Output** | Findings with code AES205, naming each detected cycle; an empty list when acyclic or when the rule is disabled. Returned inside the audit response. |
| **Protocol** | `ICycleImportProtocol` |
| **Method** | `check` |
| **Event** | None; audit is synchronous. |
| **Error** | Unparseable edge entries are dropped; the scan completes on the remaining edges. |

#### Test

| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| A two-node cycle exists between two layers | The cycle scan runs | At least one AES205 finding is reported |
| A three-node cycle exists among three layers | The cycle scan runs | At least one AES205 finding is reported |
| The import graph is acyclic | The cycle scan runs | Zero AES205 findings are reported |
| The cycle rule is disabled by configuration | The cycle scan runs | The audit returns an empty result |
| A module path with a crate prefix and the same path without it are both present | Normalization runs | Both resolve to the same layer segment |

#### Metric

| Target Metric | Value |
|---------------|-------|
| AES205 detection rate for seeded cycle fixtures | 100% |
| AES205 false-positive rate for acyclic fixtures | 0% |

## Public Interface

| Aspect | Detail |
|--------|--------|
| Agent | `ImportOrchestrator` |
| Aggregate | `IImportRunnerAggregate` |
| Method | `execute` |
| Request | `ImportRequest::{RunAudit, RunAuditWithEntries, RunAuditWithEntriesAndImports, Name}` |
| Response | `ImportResponse::{Audit, AuditEntries, Name}` |

## Reference

| Artifact | Purpose | Link |
|----------|---------|------|
| PRD | Business goals, user stories, and scope | [PRD.md](../../PRD.md) |
| Backlog | Task tracking, epics, and implementation status | [BACKLOG.md](./BACKLOG.md) |
| Roadmap | Timeline, phasing, and dependencies | [ROADMAP.md](../../ROADMAP.md) |
| Architecture | System boundaries, tech stack, and infra | [ARCHITECTURE.md](../../ARCHITECTURE.md) |
