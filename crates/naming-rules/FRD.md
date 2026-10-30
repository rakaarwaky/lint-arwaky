# FRD — naming-rules (v2.1.0)

> **Scope:** this crate enforces AES101 (stem shape), AES102 (layer suffix), and
> AES103 (test/bench file prefix). Unknown-layer-prefix files are skipped by
> AES102 (no layer → no suffix policy) and validated structurally by AES101.
> AES103 is the only rule here that reads `tests/` and `benches/`, which the
> default discovery walk prunes, so its input arrives as a second file set.

---
## Feature Overview

The naming-rules crate enforces strict naming conventions across the codebase to ensure consistency, readability, and adherence to the 7-layer AES architecture. It validates that files conform to structural naming patterns (AES101) and that file prefixes/suffixes are consistent with their architectural layer (AES102), preventing naming chaos and ensuring every file can be correctly assigned to an architectural layer.

File system operations are handled by the external `filesystem` crate via `IFilesystemAggregate`. The surface layer fetches the pre-populated file list from `filesystem.file_list()` and passes it to the naming orchestrator via `run_audit_with_entries(&[FileEntry])`. The naming-rules crate performs zero I/O — it receives data and delegates analysis to its internal checkers.

- **Architecture & Data Flow**

```mermaid
flowchart TD
    A["Surface"] -->|"filesystem.path_exists()"| FS["filesystem_aggregate"]
    A -->|"filesystem.file_list()"| FS
    A -->|"run_audit_with_entries(&[FileEntry])"| B["naming_aggregate"]
    B --> C["naming_orchestrator"]

    subgraph FS ["filesystem crate (external)"]
        FS --> E["file_walker"]
        E --> G["FileEntry[]"]
        G -.->|"cached in OnceLock"| FS
    end

    C --> H1["naming_convention_check"]
    C --> H2["suffix_prefix_check"]

    H1 --> I["Violations"]
    H2 --> I

    I --> J["LintResult"]
    J --> C
    C --> B
    B -->|output| A

```

---

## Functional Requirements

### FR-NamingRules-001: Naming Convention (AES101)

#### Description

| No | Detail |
| --- | --- |
| 1 | Validate file stem shapes against the AES naming convention for each source language. |
| 2 | — |
| ∞ | — |

#### Business Rules

| No | Detail |
| --- | --- |
| 1 | AES101 checks the stem (not the extension) for legal shape: mixed-case identifiers are expected; all-uppercase stems are violations; files are classified by directory so support-directory files are treated as flat; windows-style separators classify the same as forward slashes; barrels in support directories are skipped. |
| 2 | Violation messages report the observed shape. |
| ∞ | — |

#### Edge Cases

| No | Detail |
| --- | --- |
| 1 | All uppercase produces a violation. |
| 2 | Windows separator path classified identically. |
| 3 | Barrel in support dir skipped. |
| 4 | File outside relevant directories not read at all. |
| 5 | Unreadable file yields no finding rather than failing the audit. |
| ∞ | — |

#### Capability

| No | Detail |
| --- | --- |
| 1 | NamingConventionChecker validates stem shape. |

#### Protocol

| No | Detail |
| --- | --- |
| 1 | INamingConventionProtocol. |

#### Method

| No | Detail |
| --- | --- |
| 1 | Stem-shape check over a file path returning violations with the observed shape. |
| 2 | — |
| ∞ | — |

#### Input

| No | Detail |
| --- | --- |
| 1 | File path (and optionally content for classification). |
| 2 | Source is NamingRequest::RunAuditWithEntries or RunAuditWithTestEntries via INamingRunnerAggregate::execute. |

#### Output

| No | Detail |
| --- | --- |
| 1 | Lint results with AES101 codes inside NamingResponse::Audit. |
| 2 | — |
| ∞ | — |

#### Error

| No | Detail |
| --- | --- |
| 1 | Missing file yields no finding for that path. |
| 2 | Classification never panics on either separator style. |

#### Event

| No | Detail |
| --- | --- |
| 1 | None; audit is synchronous. |
| 2 | — |
| ∞ | — |

#### Test

| No | Detail |
| --- | --- |
| 1 | all_uppercase_produces_violation, a_windows_separator_path_is_classified_the_same_way, and a_file_under_the_support_dir_is_treated_as_flat assert shape and classification. |
| 2 | — |
| ∞ | — |

#### Metric

| No | Detail |
| --- | --- |
| 1 | All seeded stem-shape fixtures produce exactly the expected violation count. |
| 2 | Separator-style variants agree on classification. |
| ∞ | — |

### FR-NamingRules-002: Suffix/Prefix Validation (AES102)

#### Description

| No | Detail |
| --- | --- |
| 1 | Enforce suffix and prefix policies for special file roles (barrels, registries, regression guards). |
| 2 | — |
| ∞ | — |

#### Business Rules

| No | Detail |
| --- | --- |
| 1 | AES102 checks each file for a legal prefix/suffix descriptor for its role: an allowed suffix yields no violation; a file with no legal prefix produces a violation explaining the missing descriptor; barrel files are skipped (not flagged); a regression guard name is legal even though it is not a category; support directories are flat scope. |
| 2 | — |
| ∞ | — |

#### Edge Cases

| No | Detail |
| --- | --- |
| 1 | Barrel file skipped. |
| 2 | File with no legal prefix produces a violation. |
| 3 | A regression-guard name passes despite not matching category prefixes. |
| 4 | Empty content file (no descriptors found) is handled without panic. |
| 5 | File outside policy directories is never read. |
| ∞ | — |

#### Capability

| No | Detail |
| --- | --- |
| 1 | SuffixPolicyChecker validates prefix/suffix descriptors. |

#### Protocol

| No | Detail |
| --- | --- |
| 1 | ISuffixPolicyProtocol. |

#### Method

| No | Detail |
| --- | --- |
| 1 | Suffix/prefix policy check for a file returning violations naming the missing policy. |
| 2 | — |
| ∞ | — |

#### Input

| No | Detail |
| --- | --- |
| 1 | File path plus content. |
| 2 | Source is NamingRequest through the audit entry points. |

#### Output

| No | Detail |
| --- | --- |
| 1 | Lint results with AES102 codes inside NamingResponse::Audit. |
| 2 | — |
| ∞ | — |

#### Error

| No | Detail |
| --- | --- |
| 1 | Unclassified file yields no finding rather than a false positive. |
| 2 | — |

#### Event

| No | Detail |
| --- | --- |
| 1 | None. |
| 2 | — |
| ∞ | — |

#### Test

| No | Detail |
| --- | --- |
| 1 | allowed_suffix_no_violation, a_file_with_no_legal_prefix_produces_a_violation, aes102_barrel_file_skipped, and a_regression_guard_is_a_legal_name_even_though_it_is_not_a_category assert policy outcomes. |
| 2 | — |
| ∞ | — |

#### Metric

| No | Detail |
| --- | --- |
| 1 | Barrel fixtures contribute zero AES102 findings. |
| 2 | No-legal-prefix fixture yields exactly one; regression-guard fixture yields zero. |
| ∞ | — |

### FR-NamingRules-003: Test File Prefix Discipline (AES103)

#### Description

| No | Detail |
| --- | --- |
| 1 | Keep test and bench files disciplined to legal prefixes and correct directories. |
| 2 | — |
| ∞ | — |

#### Business Rules

| No | Detail |
| --- | --- |
| 1 | AES103 classifies files under tests/ and benches/: a test named inside benches reports to move it; a bench named inside tests reports to move it; a mismatched prefix says move rather than rename; a file with no legal prefix in a test directory is named with its legal prefixes in the message; nested depth is reported with flattening guidance; benchmarks belong in benches/; support subdirectories under tests are accepted so shared helpers survive; unclassified paths are reported for classification rather than assumed. |
| 2 | — |
| ∞ | — |

#### Edge Cases

| No | Detail |
| --- | --- |
| 1 | Benchmark inside tests says move it. |
| 2 | Test inside benches says move it. |
| 3 | Bench prefix inside tests says move rather than rename. |
| 4 | Benchmarks accepted in benches dir. |
| 5 | A benches subdirectory is reported. |
| 6 | Outside-directory files are not read at all. |
| 7 | Activity of one directory never reads the other. |
| ∞ | — |

#### Capability

| No | Detail |
| --- | --- |
| 1 | TestFilePrefixChecker validates test/bench prefix discipline. |

#### Protocol

| No | Detail |
| --- | --- |
| 1 | ITestFilePrefixProtocol. |

#### Method

| No | Detail |
| --- | --- |
| 1 | Test/bench placement and prefix check returning violations with move-or-rename guidance. |
| 2 | — |
| ∞ | — |

#### Input

| No | Detail |
| --- | --- |
| 1 | File path. |
| 2 | Source is NamingRequest::RunAuditWithTestEntries. |

#### Output

| No | Detail |
| --- | --- |
| 1 | Lint results with AES103 codes inside NamingResponse::Audit. |
| 2 | — |
| ∞ | — |

#### Error

| No | Detail |
| --- | --- |
| 1 | Path that cannot be classified reports need-for-classification instead of guessing. |
| 2 | No panic. |

#### Event

| No | Detail |
| --- | --- |
| 1 | None. |
| 2 | — |
| ∞ | — |

#### Test

| No | Detail |
| --- | --- |
| 1 | a_benchmark_named_inside_tests_says_move_it, a_bench_prefix_inside_tests_says_move_rather_than_rename, an_unprefixed_test_file_is_named_with_its_legal_prefixes, a_support_directory_is_accepted_so_shared_helpers_survive, and a_nested_file_reports_the_depth_it_sits_at assert guidance. |
| 2 | — |
| ∞ | — |

#### Metric

| No | Detail |
| --- | --- |
| 1 | Misplacement fixtures each produce the documented message family. |
| 2 | Support-dir and benches-dir legal fixtures produce zero findings. |
| ∞ | — |

## Public Interface

| Aspect | Detail|
|--------|-------|
| Agent | NamingOrchestrator — the agent-layer orchestrator behind the naming-rule stage of the check command. |
| Aggregate | INamingRunnerAggregate — the feature's single front door. |
| Method | execute(&self, request: NamingRequest) -> NamingResponse |
| Request | NamingRequest::{RunAuditWithEntries, RunAuditWithTestEntries, Name} |
| Response | NamingResponse::{Audit, Name} |

## Reference

| Artifact | Purpose | Link|
|----------|---------|------|
| BACKLOG.md | Backlog tracking | [BACKLOG.md](BACKLOG.md)|
| PRD.md | Product requirements | [PRD.md](../../PRD.md)|
| FRD.md | Feature requirements | [FRD.md](FRD.md)|
| ROADMAP.md | Roadmap status | [ROADMAP.md](../../ROADMAP.md)|
| ARCHITECTURE.md | Layer contract | [ARCHITECTURE.md](../../ARCHITECTURE.md)|
