# DATA — Shared

## Reference

- PRD: [PRD.md](../../PRD.md)
- Backlog: [BACKLOG.md](BACKLOG.md)

## Data Overview

The shared kernel manages cross-cutting value objects, constants, and contracts
used by all feature and surface folders. It holds no business logic — only
types, utilities, and interface definitions that other layers depend on.

## Value Objects

| ID | Field | Type | Description |
|---|---|---|---|
| DO-001 | Severity | vo | Lint finding severity level (error/warning/info) |
| DO-002 | ViolationItem | entity | Canonical representation of a single architecture violation |
| DO-003 | StructureFinding | vo | Structural violation found during folder layout audit |
| DO-004 | DocFinding | vo | Documentation invariant violation |

Each row's attributes are listed below. The attribute tables are the
specification the value objects satisfy: a field added, removed, or retyped in
the shared kernel is a change to this document in the same edit, and the
cross-document consistency gate in CI fails the build when the two drift
apart.

### DO-001: Severity — Attributes

A closed enumeration, not a record. Ranks are contiguous from zero so an
ordering comparison is a subtraction.

| Value | Rank | Wire form | Meaning |
|---|---|---|---|
| INFO | 0 | `info` | Advisory; never fails a gate on its own |
| LOW | 1 | `low` | Cosmetic or stylistic drift |
| MEDIUM | 2 | `medium` | A real defect with a bounded blast radius |
| HIGH | 3 | `high` | A structural or document-invariant breach |
| CRITICAL | 4 | `critical` | A boundary violation; the highest rank a rule may report |

### DO-002: ViolationItem — Attributes

| Field | Type | Description |
|---|---|---|
| code | ErrorCode | The AES or tool-native rule code that fired |
| file | FilePath | The file the violation was found in |
| line | LineNumber | 1-based source line; zero when the finding is file-level |
| column | ColumnNumber | 1-based source column; zero when the rule reports no column |
| message | LintMessage | Human-readable violation detail |
| severity | Severity | DO-001 rank carried through to every output format |

### DO-003: StructureFinding — Attributes

| Field | Type | Description |
|---|---|---|
| code | Text | The structure invariant that fired (AES701–AES703) |
| violation_type | Text | Machine-parseable subtype, so a consumer routes without reading prose |
| file | Text | The folder or file the finding points at, as the caller supplied it |
| message | Text | Human-readable detail about what drifted |

A structure finding carries no line, no column, and no severity of its own: a
folder-layout breach has no source position, and the projection to DO-002
assigns MEDIUM. "Text" here means a plain string, not a typed value object:
DO-003 and DO-004 predate the DO-002 value-object set and stay primitive so the
rule crates can report without depending on the common taxonomy.

### DO-004: DocFinding — Attributes

| Field | Type | Description |
|---|---|---|
| code | Text | The document invariant that fired (AES601–AES605) |
| violation_type | Text | Machine-parseable subtype, so a consumer routes without reading prose |
| doc | Text | The document the finding belongs to, relative to the audit root |
| message | Text | Human-readable detail about what drifted |
| line | Count | 1-based line in the document; zero or one for document-level findings |
| severity | Severity | Always HIGH — every document invariant is an architectural policy gap |

### Relationships

| From | To | Kind | Rule |
|---|---|---|---|
| LintResult | DO-002 ViolationItem | derivation (1:1) | A violation item is a projection of a lint result, copying code, file, line, column, message, and severity. It is never created independently of the result it narrows. |
| DO-003 StructureFinding | DO-002 ViolationItem | projection (1:1) | The dispatcher maps a structure finding onto a violation item, defaulting line and column to zero and severity to MEDIUM. |
| DO-004 DocFinding | DO-002 ViolationItem | projection (1:1) | A doc finding maps onto a violation item for JSON/SARIF output: `doc` joined to the audit root becomes the file path, column becomes 1, and severity stays HIGH. |
| DO-002 ViolationItem | DO-001 Severity | composition | Every violation item carries exactly one severity; severity is shared by all three finding types. |

## Assumptions & Constraints

- The shared folder is a dependency sink — no other layer may import from features or surfaces.
- Value object field names are stable; renaming requires updating all consumers.
- Severity ranks must remain contiguous integers starting from zero.
- The attribute tables above are normative. A value object's field set and this document change together, and the doc-consistency check in CI compares the two.
- "Count" denotes a non-negative whole number and "Text" a plain string; every other type name in the attribute tables is a shared value object.
