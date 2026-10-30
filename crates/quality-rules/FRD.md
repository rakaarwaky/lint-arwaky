# FRD — quality-rules (v2.0.0)

---

## Feature Overview

The quality-rules crate enforces general code quality, formatting limits, and clean-coding policies. It protects the codebase from bloated files, empty structures, duplicate blocks, and bypass annotations while guaranteeing zero tolerance for warning/error suppressions.

File discovery, raw content reads, and AST parsing are handled by the external `filesystem` aggregate (`IFilesystemAggregate`). The Surface calls `filesystem.build_file_index(root)` to populate caches, then passes pre-fetched `&[FileEntry]` to the quality-rules orchestrator via `run_audit_with_entries`. The quality-rules crate does zero I/O — it only performs business logic analysis on pre-fetched data.

- **Architecture & Data Flow**

```mermaid
flowchart TD
    A["Surface"] -->|"build_file_index(root)"| D["filesystem_aggregate\n(external crate)"]
    A -->|"file_list()"| D

    subgraph FS ["filesystem crate (external)"]
        D --> E1["file_walker"]
        D --> E2["AST parser\n(parse_metadata)"]
        E1 --> G1["FileEntry[]\n+ content_map"]
        E2 --> G1
    end

    G1 -->|"return"| D
    D -->|"FileEntry[]\n(pre-fetched)"| A

    A -->|"run_audit_with_entries(&[FileEntry])"| B["quality_aggregate"]
    B --> C["quality_orchestrator\n(zero I/O)"]

    C --> H1["line_count_check"]
    C --> H2["definition_check"]
    C --> H3["bypass_detection"]
    C --> H4["duplication_analysis"]

    H1 --> I["Violations"]
    H2 --> I
    H3 --> I
    H4 --> I

    I --> J["LintResult"]
    J --> C
    C --> B
    B -->|output| A

```

---

## Functional Requirements

### FR-QualityRules-001: Maximum File Line Count (AES301)

#### Description

| No | Detail |
| --- | --- |
| 1 | Flag source files whose line count exceeds the configured maximum. |
| 2 | — |

#### Business Rules

| No | Detail |
| --- | --- |
| 1 | AES301 fires FileTooLarge when a file's line count is above the max threshold; the violation message contains the observed count and the limit. |
| 2 | Files at exactly the limit are allowed. |

#### Edge Cases

| No | Detail |
| --- | --- |
| 1 | A file one line over the limit violates; a file at the limit does not; unreadable file yields no finding. |
| 2 | Empty path list yields no findings; message content includes the line count for diagnosis. |

#### Capability

| No | Detail |
| --- | --- |
| 1 | ArchLineChecker counts lines for both AES301 and AES302. |

#### Protocol

| No | Detail |
| --- | --- |
| 1 | ILineCheckerProtocol. |

#### Method

| No | Detail |
| --- | --- |
| 1 | Line-count check against the configured maximum returning an AES301 violation on overflow. |
| 2 | — |

#### Input

| No | Detail |
| --- | --- |
| 1 | File path plus content (or line count); source is CodeAnalysisRequest::RunAnalysis via ICodeAnalysisAggregate::execute. |
| 2 | — |

#### Output

| No | Detail |
| --- | --- |
| 1 | Violation (FileTooLarge) inside CodeAnalysisResponse::Analysis. |
| 2 | — |

#### Error

| No | Detail |
| --- | --- |
| 1 | Unreadable file reports no finding for that path rather than failing the analysis. |
| 2 | — |

#### Event

| No | Detail |
| --- | --- |
| 1 | None; analysis is synchronous. |
| 2 | — |

#### Test

| No | Detail |
| --- | --- |
| 1 | file_exceeding_max_lines_produces_aes301_violation asserts violation. |
| 2 | aes301_violation_message_contains_details asserts message content. |

#### Metric

| No | Detail |
| --- | --- |
| 1 | Seeded oversized file produces exactly one FileTooLarge finding. |
| 2 | At-limit fixture produces zero. |

### FR-QualityRules-002: Minimum File Line Count (AES302)

#### Description

| No | Detail |
| --- | --- |
| 1 | Flag source files whose line count falls below the configured minimum. |
| 2 | — |

#### Business Rules

| No | Detail |
| --- | --- |
| 1 | AES302 fires FileTooShort when a file's line count is under the min threshold; the violation message contains the observed count and the limit. |
| 2 | Files at exactly the minimum are allowed. |

#### Edge Cases

| No | Detail |
| --- | --- |
| 1 | A file one line under the minimum violates; at-minimum does not; empty file counts as below-minimum and violates. |
| 2 | Unreadable file yields no finding. |

#### Capability

| No | Detail |
| --- | --- |
| 1 | ArchLineChecker (shared with FR-QualityRules-001) counts lines against the minimum. |

#### Protocol

| No | Detail |
| --- | --- |
| 1 | ILineCheckerProtocol. |

#### Method

| No | Detail |
| --- | --- |
| 1 | Line-count check against the configured minimum returning an AES302 violation on shortfall. |
| 2 | — |

#### Input

| No | Detail |
| --- | --- |
| 1 | File path plus content; source is CodeAnalysisRequest::RunAnalysis. |
| 2 | — |

#### Output

| No | Detail |
| --- | --- |
| 1 | Violation (FileTooShort) inside CodeAnalysisResponse::Analysis. |
| 2 | — |

#### Error

| No | Detail |
| --- | --- |
| 1 | Unreadable file reports no finding rather than failing the analysis. |
| 2 | — |

#### Event

| No | Detail |
| --- | --- |
| 1 | None. |
| 2 | — |

#### Test

| No | Detail |
| --- | --- |
| 1 | file_below_min_lines_produces_aes302_violation asserts violation. |
| 2 | aes302_violation_message_contains_line_count asserts message content. |

#### Metric

| No | Detail |
| --- | --- |
| 1 | Seeded undersized file produces exactly one FileTooShort finding. |
| 2 | At-minimum fixture produces zero. |

### FR-QualityRules-003: Mandatory Definitions & Dead Inheritance (AES303)

#### Description

| No | Detail |
| --- | --- |
| 1 | Require struct/enum/trait/class/interface/type definitions and flag dead (never-implemented) inheritance. |
| 2 | — |

#### Business Rules

| No | Detail |
| --- | --- |
| 1 | AES303 sub-check 1 (MandatoryDefinitionChecker) fires when a source file has no mandatory definition kind. |
| 2 | Sub-check 2 (DeadInheritanceChecker) fires DeadInheritance for a struct declared without an impl block; empty class/interface variants are caught per language: Python empty class, JS empty class, JS empty interface all produce AES303. |

#### Edge Cases

| No | Detail |
| --- | --- |
| 1 | File without any struct or class produces AES303; struct without impl produces DeadInheritance. |
| 2 | Python empty class, JS empty class, JS empty interface each produce AES303; a file that has definitions passes quietly. |

#### Capability

| No | Detail |
| --- | --- |
| 1 | MandatoryDefinitionChecker enforces definition presence; DeadInheritanceChecker detects unimplemented inheritance. |

#### Protocol

| No | Detail |
| --- | --- |
| 1 | IMandatoryClassProtocol and IDeadInheritanceProtocol. |

#### Method

| No | Detail |
| --- | --- |
| 1 | Definition-presence check (MandatoryDefinitionChecker) plus dead-inheritance check (DeadInheritanceChecker), both over parsed entries. |
| 2 | — |

#### Input

| No | Detail |
| --- | --- |
| 1 | Parsed file entries (defs, impls); source is CodeAnalysisRequest::RunAnalysis. |
| 2 | — |

#### Output

| No | Detail |
| --- | --- |
| 1 | Violations MandatoryClassDefinition and DeadInheritance inside CodeAnalysisResponse::Analysis. |
| 2 | — |

#### Error

| No | Detail |
| --- | --- |
| 1 | Unparseable file contributes no findings; analysis continues across files. |
| 2 | — |

#### Event

| No | Detail |
| --- | --- |
| 1 | None. |
| 2 | — |

#### Test

| No | Detail |
| --- | --- |
| 1 | file_without_struct_or_class_produces_aes303, python_empty_class_produces_aes303, js_empty_interface_produces_aes303 assert both sub-checks. |
| 2 | unit_struct_without_impl_produces_aes303_dead_inheritance asserts dead-inheritance. |

#### Metric

| No | Detail |
| --- | --- |
| 1 | Each seeded definition-miss fixture produces exactly one definition finding. |
| 2 | Seeded struct-without-impl produces one DeadInheritance finding. |

### FR-QualityRules-004: Bypass Detection (AES304)

#### Description

| No | Detail |
| --- | --- |
| 1 | Detect bypass annotations and panic-style shortcuts that dodge quality checks. |
| 2 | — |

#### Business Rules

| No | Detail |
| --- | --- |
| 1 | AES304 fires per ViolationKind: BypassComment (bypass suppressions), UnwrapExpect, Panic, Todo, Unimplemented. Each occurrence in a scanned file is counted as its own finding with the kind carried in the message. |
| 2 | — |

#### Edge Cases

| No | Detail |
| --- | --- |
| 1 | Multiple bypass sites in one file each fire; a file with none passes; comment-only todo still flags Unimplemented/Todo kind as coded. |
| 2 | Unreadable file yields no finding. |

#### Capability

| No | Detail |
| --- | --- |
| 1 | BypassChecker scans for bypass annotations and panic-style calls. |

#### Protocol

| No | Detail |
| --- | --- |
| 1 | IBypassCheckerProtocol. |

#### Method

| No | Detail |
| --- | --- |
| 1 | Scan file content for bypass markers and panic constructs returning per-kind violations. |
| 2 | — |

#### Input

| No | Detail |
| --- | --- |
| 1 | File content plus path; source is CodeAnalysisRequest::RunAnalysis. |
| 2 | — |

#### Output

| No | Detail |
| --- | --- |
| 1 | Violations (BypassComment, UnwrapExpect, Panic, Todo, Unimplemented) inside CodeAnalysisResponse::Analysis. |
| 2 | — |

#### Error

| No | Detail |
| --- | --- |
| 1 | Unreadable file contributes no findings; no analysis failure occurs. |
| 2 | — |

#### Event

| No | Detail |
| --- | --- |
| 1 | None. |
| 2 | — |

#### Test

| No | Detail |
| --- | --- |
| 1 | Bypass-checker contract tests in the quality suite assert per-kind detection through IBypassCheckerProtocol (ViolationKind enums: UnwrapExpect, Panic, Todo, Unimplemented, BypassComment). |
| 2 | — |

#### Metric

| No | Detail |
| --- | --- |
| 1 | Seeded kind fixtures produce one finding of the matching kind each. |
| 2 | Clean fixture produces zero. |

### FR-QualityRules-005: Duplicate Code Detection (AES305)

#### Description

| No | Detail |
| --- | --- |
| 1 | Detect excessive code duplication across the codebase. |
| 2 | — |

#### Business Rules

| No | Detail |
| --- | --- |
| 1 | AES305 fires CodeDuplication when duplicate regions across scanned files exceed the configured duplication threshold; identical files or near-clones group into a single finding naming the participants. |
| 2 | — |

#### Edge Cases

| No | Detail |
| --- | --- |
| 1 | Fully distinct files yield no duplication finding; exact duplicate pair yields one CodeDuplication finding; a file compared against itself is excluded. |
| 2 | Empty scan set yields no findings. |

#### Capability

| No | Detail |
| --- | --- |
| 1 | CodeDuplicationAnalyzer computes similarity groups. |

#### Protocol

| No | Detail |
| --- | --- |
| 1 | ICodeMetricAnalyzerProtocol. |

#### Method

| No | Detail |
| --- | --- |
| 1 | Duplication analysis over the scanned file set returning CodeDuplication violations. |
| 2 | — |

#### Input

| No | Detail |
| --- | --- |
| 1 | Set of file contents from the analysis request; source is CodeAnalysisRequest::RunAnalysis. |
| 2 | — |

#### Output

| No | Detail |
| --- | --- |
| 1 | Violation (CodeDuplication) inside CodeAnalysisResponse::Analysis. |
| 2 | — |

#### Error

| No | Detail |
| --- | --- |
| 1 | Unreadable file drops out of the comparison set rather than failing the run. |
| 2 | — |

#### Event

| No | Detail |
| --- | --- |
| 1 | None. |
| 2 | — |

#### Test

| No | Detail |
| --- | --- |
| 1 | Duplication contract coverage in the quality suite asserts CodeDuplication on a seeded clone pair and silence on distinct files. |
| 2 | — |

#### Metric

| No | Detail |
| --- | --- |
| 1 | Seeded clone pair produces one CodeDuplication finding. |
| 2 | Distinct-file fixture set produces zero. |

## Public Interface

| Aspect | Detail|
|--------|-------|
| Agent | CodeAnalysisOrchestrator — the agent-layer orchestrator behind the quality-rules stage of the check command. |
| Aggregate | ICodeAnalysisAggregate — the feature's single front door. |
| Method | execute(&self, request: CodeAnalysisRequest) -> CodeAnalysisResponse |
| Request | CodeAnalysisRequest::{RunAnalysis, Name} |
| Response | CodeAnalysisResponse::{Analysis, Name} |

## Reference

| Artifact | Purpose | Link|
|----------|---------|------|
| BACKLOG.md | Backlog tracking | [BACKLOG.md](BACKLOG.md)|
| PRD.md | Product requirements | [PRD.md](../../PRD.md)|
| FRD.md | Feature requirements | [FRD.md](FRD.md)|
| ROADMAP.md | Roadmap status | [ROADMAP.md](../../ROADMAP.md)|
| ARCHITECTURE.md | Layer contract | [ARCHITECTURE.md](../../ARCHITECTURE.md)|
