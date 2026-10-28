# FRD — shared

## Feature Overview

The shared crate is the kernel every feature and surface folder builds on. It
holds cross-cutting value objects, contracts, and stateless utilities —
severity and violation records, report snapshots and deltas, parsers, path
filters, and formatting helpers — with no business logic of its own. Its
dependency direction is inward only: the kernel never reaches back into a
feature, and its type contracts are specified in the paired DATA document.

## Functional Requirements

### FR-Shared-001: Kernel Taxonomy Contracts

| Aspect | Detail|
|--------|-------|
| Description | Publish the value objects and contract traits every feature and surface shares, as a stable kernel API with no business logic. |
| Business Rules | Severity carries a total ordering with a default of Info; finding and violation records expose their fields losslessly in both directions; contract traits stay object-safe so features compose them behind the aggregate entry points; the kernel depends on nothing above it. |
| Edge Cases | A severity built without an explicit level defaults to Info; round-tripping a record through its serialized form preserves every field; a trait object holding a kernel contract can be shared across threads. |
| Capability | Kernel taxonomy modules for severities, findings, workspace value objects, and contract traits. |
| Protocol | kernel contract traits consumed by feature aggregates; each trait is a capability seam the feature layer implements or receives. |
| Method | field accessors, constructors, ordering, and the trait methods the feature contracts declare. |
| Input | values supplied by feature and surface layers; source is their call sites. |
| Output | typed records and trait objects those layers consume directly. |
| Error | invalid input is rejected at construction with typed errors where the value can be validated. |
| Event | None; kernel values are plain data. |
| Test | severity_variants_exist, severity_default_is_info, filepath_new_and_value, and filepath_normalizes_backslashes assert the kernel contracts. |
| Metric | Every seeded record round-trips without field loss; the severity default is Info on every construction path. |

### FR-Shared-002: Kernel Utility Services

| Aspect | Detail|
|--------|-------|
| Description | Provide the stateless utilities every layer reuses: parsers, path filters, output formatting, and report snapshot helpers. |
| Business Rules | Utilities are stateless and side-effect free, so identical inputs yield identical outputs; path filters normalize separators before comparing; report helpers render the same snapshot the CLI, MCP, and TUI surfaces display; each utility ships with its own unit suite beside it. |
| Edge Cases | A path filter matches correctly across separator styles; a snapshot helper renders an empty snapshot without special-casing; a parser fed malformed input reports it instead of panicking. |
| Capability | Kernel utility modules for parsing, path filtering, output formatting, snapshot/delta rendering, and skill hint resolution. |
| Protocol | —; utilities are free functions and value types, so they expose no seam — the seam count stays with the taxonomy contracts. |
| Method | parse, filter, format, and render functions over kernel value objects. |
| Input | paths, raw text, and snapshot records supplied by calling layers. |
| Output | parsed values, boolean matches, rendered text, and derived snapshots and deltas. |
| Error | malformed input yields a typed error or a default, never a panic. |
| Event | None; utilities run synchronously. |
| Test | the shared utility unit suites (parsers, path filter, formatting, skill hints) assert each helper's contract. |
| Metric | Every seeded path matches identically under either separator style; every seeded snapshot renders one line per member. |

## Public Interface

| Aspect | Detail|
|--------|-------|
| Kernel | taxonomy value objects, contract traits, and stateless utilities under shared/src. |
| Consumers | every feature and surface folder imports kernel types; the kernel imports none of theirs. |
| Specification | the paired DATA document is the source of truth for kernel record relationships. |
| Method | free functions and trait methods declared by the kernel contracts. |
| Error | typed construction errors where a value can be validated at construction. |

## Reference

| Artifact | Purpose | Link|
|----------|---------|------|
| BACKLOG.md | Backlog tracking | [BACKLOG.md](BACKLOG.md)|
| PRD.md | Product requirements | [PRD.md](../../PRD.md)|
| FRD.md | Feature requirements | [FRD.md](FRD.md)|
| ROADMAP.md | Roadmap status | [ROADMAP.md](../../ROADMAP.md)|
| ARCHITECTURE.md | Layer contract | [ARCHITECTURE.md](../../ARCHITECTURE.md)|
