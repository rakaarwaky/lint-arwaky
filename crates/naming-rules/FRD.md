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

- **Description**: Every file stem must be snake_case with at least N underscore-separated words in `prefix_concept_suffix` pattern.
- **Input**: Pre-populated `&[FileEntry]` from filesystem aggregate (via surface), architecture configuration, layer map.
- **Output**:

  - AES101 diagnostic if naming structure is invalid.
- **Business Rules**:

  - Must be snake_case: lowercase ASCII letters (`a-z`), digits (`0-9`), and underscores only. No uppercase, no hyphens, no dots.
  - Must follow `prefix_concept_suffix` pattern with minimum N words (configurable via `config.naming.word_count.value`, default 3.
  - Validation regex: `^[a-z0-9]+(_[a-z0-9]+){N-1,}$` — compiled once per word count and cached in a static `OnceLock` table for word counts 1–10; counts above 10 build the pattern on demand so the configured minimum is honored exactly.
  - Exceptions — three sources, evaluated in order; any match skips the file:
    1. **Barrel files and entry points** — module barrels, library roots, binary entries, package markers, and index barrels.
    2. **Rule-level exceptions** (`config.rules[].exceptions.values`) — global exceptions that apply to all layers for a given rule code.
    3. **Definition-level exceptions** (`layers[].exceptions.values`) — per-layer exceptions that apply only when the file's layer is recognized.
- **Edge Cases**:

  - Files with uppercase letters → AES101 (invalid snake_case).
  - Files with hyphens (`taxonomy-user-vo`) → AES101 (invalid separator).
  - Files with dots (`taxonomy.user.vo`) → AES101 (invalid character).
  - Abbreviations like `db` or `http` → allowed as long as lowercase and underscore-separated.
  - Digits in segments (`taxonomy_v2_vo`) → allowed.
  - Files with fewer than N words → AES101 (too few words).
- **Error Handling**:

  - Emit AES101 with the invalid stem, expected pattern, and minimum word count.
  - Files with empty content (parse failures from filesystem crate) are skipped.

---

### FR-NamingRules-002: Suffix/Prefix Validation (AES102)

- **Description**: File suffix must align with the architectural layer indicated by its prefix, and file prefix must be consistent with its suffix. Forbidden suffixes from other layers are rejected. Prefix-suffix cross-validation ensures a file's layer identity is internally consistent.
- **Input**: Pre-populated `&[FileEntry]` from filesystem aggregate (via surface), architecture configuration with per-layer suffix policies, layer map.
- **Output**: AES102 diagnostic if suffix is forbidden, mismatches the layer's allowed list, or is inconsistent with the prefix.
- **Business Rules**:

  - **Suffix extraction**: The suffix is the last underscore-separated token from the stem (e.g., `taxonomy_user_vo` → suffix = `vo`).
  - **Prefix extraction**: The prefix is the first underscore-separated token from the stem (e.g., `taxonomy_user_vo` → prefix = `taxonomy`).
  - **Suffix policy per layer** (from config `layers` definition):

    - `strict`: Only suffixes in the explicit allow-list are permitted. Any other suffix → AES102 (`SuffixMismatch`).
    - `flexible`: Any suffix is allowed EXCEPT those in the `forbidden` list. Forbidden suffix → AES102 (`SuffixForbidden`).
    - If a layer has no suffix definition, suffix checking is skipped for that layer.
  - **Prefix-suffix cross-validation**:

    - The detected prefix determines the expected layer.
    - The suffix must belong to that layer's allowed suffix set.
    - If the suffix belongs to a DIFFERENT layer's suffix set, AES102 is emitted with both the expected and actual layer.
    - Example: `taxonomy_user_protocol` → prefix = `taxonomy`, suffix = `protocol`. But `protocol` belongs to `contract` layer → AES102 (`PrefixSuffixMismatch`: expected taxonomy suffix, got contract suffix).
  - **Forbidden suffix enforcement**:

    - If a suffix appears in the layer's `forbidden` list, it is immediately rejected (AES102 with `SuffixForbidden`), regardless of flexible/strict policy.
  - **Barrel files and entry points** — module barrels, library roots, binary entries, package markers, index barrels, and packaging build scripts — are skipped.
  - **Rule-level exceptions** (`config.rules[].exceptions.values`) — global exceptions that apply to all layers for a given rule code.
  - **Definition-level exceptions** (`layers[].exceptions.values`) — per-layer exceptions that apply only when the file's layer is recognized.
  - **Layer detection** via the layer detection utility using filename prefix.
- **Per-Layer Suffix Policies** (source of truth: YAML config):

  | Layer        | Policy   | Allowed Suffixes                                                                                              | Forbidden Suffixes                                                                           |
  | -------------- | ---------- | --------------------------------------------------------------------------------------------------------------- | -----------------------------------------------------------------------------------------------|
  | taxonomy     | strict   | `vo`, `entity`, `error`, `event`, `constant`                                                                  | —                                                                                           |
  | contract     | strict   | `protocol`, `aggregate`                                                                                       | —                                                                                           |
  | utility      | flexible | *(any)*                                                                                                       | `vo`, `entity`, `error`, `event`, `constant`, `protocol`, `aggregate`                        |
  | capabilities | flexible | *(any)*                                                                                                       | `vo`, `entity`, `error`, `event`, `constant`, `constants`, `protocol`, `aggregate`, `utility`|
  | agent        | strict   | `orchestrator`                                                                                                | —                                                                                           |
  | surface      | strict   | `command`, `controller`, `page`, `view`, `component`, `router`, `layout`, `hook`, `store`, `action`, `screen` | —                                                                                           |
  | root         | strict   | `entry`, `container`                                                                                          | —                                                                                           |

- **Edge Cases**:

  - Files with no suffix (single-word stem after prefix, e.g., `taxonomy_user`) → fails strict policy check (AES102 `SuffixMismatch`).
  - Multiple valid suffixes for a layer (e.g., taxonomy allows `_vo`, `_entity`, `_error`, `_event`, `_constant`) → all pass.
  - Custom or unknown layers without a definition → skipped (no definition means no suffix policy).
  - Prefix-suffix mismatch across layers (e.g., `contract_user_vo`) → AES102 (`PrefixSuffixMismatch`).
  - Packaging build scripts are skipped (in exceptions list).
- **Error Handling**: Emit AES102 with the layer name, used suffix, prefix, and the full allowed/forbidden lists. For prefix-suffix mismatch, include expected layer and actual suffix layer.

---

### FR-NamingRules-003: Test File Prefix Discipline (AES103)

- **Description**: Every file inside `tests/` and `benches/` must be typed by its file-name prefix, because the prefix is the virtual folder — there is no subdirectory left to express the type in.
- **Input**: The test and bench file entries discovered by the filesystem aggregate's `tests/`/`benches/` walk, the architecture configuration, and the layer map.
- **Output**: AES103 diagnostic naming the offending prefix set, the directory, and whether the fix is a rename or a move.
- **Business Rules**:

  - **Slot classification** reads the containing directory, not the path depth. A file directly in `tests/` is a test file, one directly in `benches/` is a benchmark, and anything else is out of scope.
  - **Legal prefixes in `tests/`** are the seven test types (`contract_`, `unit_`, `integration_`, `dogfood_`, `smoke_`, `e2e_`, `acceptance_`) plus the support set (`regression_`, `behavioral_`, `mock_`, `fixture_`).
  - **Legal prefix in `benches/`** is `bench_` alone.
  - **Flat layout**: neither directory may nest a subdirectory, with one registered exception — a `common/` support directory, which Rust reaches through a module declaration. A support module cannot sit flat beside the test files without colliding with them.
  - **Barrels are skipped.** A Rust module barrel, a Python package marker, or a JavaScript index file names no test of its own; the same exemption AES101 grants them applies here.
  - **Rule-level exceptions** (`config.rules[].exceptions.values`) skip a file by bare name, evaluated before any classification.
  - **Support prefixes satisfy AES103 but not AES704.** A `regression_*` guard is a legal file name and no required category; category presence is the structure rule's question.
- **Edge Cases**:

  - A prefix belonging to the *other* directory (`bench_*` inside `tests/`) → AES103 distinguishing a move from a rename.
  - No legal prefix at all → AES103 listing the legal set for that directory.
  - A file nested below `tests/common/` → treated as flat, since that is where shared support code goes.
  - A workspace with no `tests/` or `benches/` at all → no AES103 finding; the absence of a suite is AES704's question.
- **Error Handling**: Emit AES103 at the file, naming the directory, the legal prefix set, and whether the fix is a rename or a move. Files with empty content are skipped by the same unreadable proxy AES101 and AES102 use.

---

## Public Interface

### Protocol API

| Method | Input | Output | Error | Event | Description|
|---|---|---|---|---|---|
| `check_file_naming` | &ArchitectureConfig, &LayerMapVO, &FilePathList, &FilePath, &mut LintResultList | `` | — | — | Check file naming.|
| `check_domain_suffixes` | &ArchitectureConfig, &LayerMapVO, &FilePathList, &FilePath, &mut LintResultList | `` | — | — | Check domain suffixes.|
| `check_test_file_prefixes` | &ArchitectureConfig, &LayerMapVO, &FilePathList, &FilePath, &mut LintResultList | `` | — | — | Check test/bench file prefixes.|

### Aggregate API

| Method | Input | Output | Error | Event | Description|
|---|---|---|---|---|---|
| `execute` | NamingRequest | `NamingResponse` | — | — | Single composite entry point over the feature.|

## Reference

- Backlog: [BACKLOG.md](BACKLOG.md) — real condition for this feature; this file is specification only.
- PRD: [PRD.md](../../PRD.md)
- Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
- Filesystem crate
- Shared crate

| Roadmap | {Link to ROADMAP.md} | [{link}](ROADMAP.md)|
