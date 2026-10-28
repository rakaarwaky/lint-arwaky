# FRD — naming-rules (v2.1.0)

> **Scope:** this crate enforces AES101 (stem shape), AES102 (layer suffix), and
> AES103 (test/bench file prefix). Unknown-layer-prefix files are skipped by
> AES102 (no layer → no suffix policy) and validated structurally by AES101.
> AES103 is the only rule here that reads `tests/` and `benches/`, which the
> default discovery walk prunes, so its input arrives as a second file set.

---

## Feature Overview

The naming-rules crate enforces strict naming conventions across the codebase to ensure consistency, readability, and adherence to the 7-layer AES architecture. It validates that files conform to structural naming patterns (AES101) and that file prefixes/suffixes are consistent with their architectural layer (AES102), preventing naming chaos and ensuring every file can be correctly assigned to an architectural layer.

File system operations  are handled by the external `filesystem` crate via `IFilesystemAggregate`. The surface layer fetches the pre-populated file list from `filesystem.file_list()` and passes it to the naming orchestrator via `run_audit_with_entries(&[FileEntry])`. The naming-rules crate performs zero I/O — it receives data and delegates analysis to its internal checkers.

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

| Aspect | Detail|
|--------|-------|
| Description | Validate file stem shapes against the AES naming convention for each source language. |
| Business Rules | AES101 checks the stem (not the extension) for legal shape: mixed-case identifiers are expected; all-uppercase stems are violations; files are classified by directory so support-directory files are treated as flat; windows-style separators classify the same as forward slashes; barrels in support directories are skipped. Violation messages report the observed shape. |
| Edge Cases | All uppercase produces a violation; windows separator path classified identically; barrel in support dir skipped; file outside relevant directories not read at all; unreadable file yields no finding rather than failing the audit. |
| Capability | NamingConventionChecker validates stem shape. |
| Protocol | INamingConventionProtocol. |
| Method | Stem-shape check over a file path returning violations with the observed shape. |
| Input | File path (and optionally content for classification); source is NamingRequest::RunAuditWithEntries or RunAuditWithTestEntries via INamingRunnerAggregate::execute. |
| Output | Lint results with AES101 codes inside NamingResponse::Audit. |
| Error | Missing file yields no finding for that path; classification never panics on either separator style. |
| Event | None; audit is synchronous. |
| Test | all_uppercase_produces_violation, a_windows_separator_path_is_classified_the_same_way, and a_file_under_the_support_dir_is_treated_as_flat assert shape and classification. |
| Metric | All seeded stem-shape fixtures produce exactly the expected violation count; separator-style variants agree on classification. |

### FR-NamingRules-002: Suffix/Prefix Validation (AES102)

| Aspect | Detail|
|--------|-------|
| Description | Enforce suffix and prefix policies for special file roles (barrels, registries, regression guards). |
| Business Rules | AES102 checks each file for a legal prefix/suffix descriptor for its role: an allowed suffix yields no violation; a file with no legal prefix produces a violation explaining the missing descriptor; barrel files are skipped (not flagged); a regression guard name is legal even though it is not a category; support directories are flat scope. |
| Edge Cases | Barrel file skipped; file with no legal prefix produces a violation; a regression-guard name passes despite not matching category prefixes; empty content file (no descriptors found) is handled without panic; file outside policy directories is never read. |
| Capability | SuffixPolicyChecker validates prefix/suffix descriptors. |
| Protocol | ISuffixPolicyProtocol. |
| Method | Suffix/prefix policy check for a file returning violations naming the missing policy. |
| Input | File path plus content; source is NamingRequest through the audit entry points. |
| Output | Lint results with AES102 codes inside NamingResponse::Audit. |
| Error | Unclassified file yields no finding rather than a false positive. |
| Event | None. |
| Test | allowed_suffix_no_violation, a_file_with_no_legal_prefix_produces_a_violation, aes102_barrel_file_skipped, and a_regression_guard_is_a_legal_name_even_though_it_is_not_a_category assert policy outcomes. |
| Metric | Barrel fixtures contribute zero AES102 findings; no-legal-prefix fixture yields exactly one; regression-guard fixture yields zero. |

### FR-NamingRules-003: Test File Prefix Discipline (AES103)

| Aspect | Detail|
|--------|-------|
| Description | Keep test and bench files disciplined to legal prefixes and correct directories. |
| Business Rules | AES103 classifies files under tests/ and benches/: a test named inside benches reports to move it; a bench named inside tests reports to move it; a mismatched prefix says move rather than rename; a file with no legal prefix in a test directory is named with its legal prefixes in the message; nested depth is reported with flattening guidance; benchmarks belong in benches/; support subdirectories under tests are accepted so shared helpers survive; unclassified paths are reported for classification rather than assumed. |
| Edge Cases | Benchmark inside tests says move it; test inside benches says move it; bench prefix inside tests says move rather than rename; benchmarks accepted in benches dir; a benches subdirectory is reported; outside-directory files are not read at all; activity of one directory never reads the other. |
| Capability | TestFilePrefixChecker validates test/bench prefix discipline. |
| Protocol | ITestFilePrefixProtocol. |
| Method | Test/bench placement and prefix check returning violations with move-or-rename guidance. |
| Input | File path; source is NamingRequest::RunAuditWithTestEntries. |
| Output | Lint results with AES103 codes inside NamingResponse::Audit. |
| Error | Path that cannot be classified reports need-for-classification instead of guessing; no panic. |
| Event | None. |
| Test | a_benchmark_named_inside_tests_says_move_it, a_bench_prefix_inside_tests_says_move_rather_than_rename, an_unprefixed_test_file_is_named_with_its_legal_prefixes, a_support_directory_is_accepted_so_shared_helpers_survive, and a_nested_file_reports_the_depth_it_sits_at assert guidance. |
| Metric | Misplacement fixtures each produce the documented message family; support-dir and benches-dir legal fixtures produce zero findings. |

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
