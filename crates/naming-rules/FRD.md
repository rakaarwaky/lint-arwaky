# FRD — naming-rules (v2.0.0)

> **Scope:** this crate enforces AES101 and AES102 only. Unknown-layer-prefix
> files are skipped by AES102 (no layer → no suffix policy) and validated
> structurally by AES101.

---

## Reference

- Backlog: [BACKLOG.md](BACKLOG.md) — real condition for this feature; this file is specification only.
- PRD: [PRD.md](../../PRD.md)
- Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
- Filesystem crate
- Shared crate

## System Overview

The naming-rules crate enforces strict naming conventions across the codebase to ensure consistency, readability, and adherence to the 7-layer AES architecture. It validates that files conform to structural naming patterns (AES101) and that file prefixes/suffixes are consistent with their architectural layer (AES102), preventing naming chaos and ensuring every file can be correctly assigned to an architectural layer.

File system operations  are handled by the external `filesystem` crate via `IFilesystemAggregate`. The surface layer fetches the pre-populated file list from `filesystem.file_list()` and passes it to the naming orchestrator via `run_audit_with_entries(&[FileEntry])`. The naming-rules crate performs zero I/O — it receives data and delegates analysis to its internal checkers.

### Architecture & Data Flow

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

### FR-NAMINGRULES-001: Naming Convention (AES101)

- **Description**: Every file stem must be snake_case with at least N underscore-separated words in `prefix_concept_suffix` pattern.
- **Input**: Pre-populated `&[FileEntry]` from filesystem aggregate (via surface), architecture configuration, layer map.
- **Output**:

  - AES101 diagnostic if naming structure is invalid.
- **Business Rules**:

  - Must be snake_case: lowercase ASCII letters (`a-z`), digits (`0-9`), and underscores only. No uppercase, no hyphens, no dots.
  - Must follow `prefix_concept_suffix` pattern with minimum N words (configurable via `config.naming.word_count.value`, default 3.
  - Validation regex: `^[a-z0-9]+(_[a-z0-9]+){N-1,}$` — compiled once per word count and cached in a static `OnceLock` table (one slot per word count 1–10).
  - Exceptions — three sources, evaluated in order; any match skips the file:
    1. **Barrel files and entry points** (the module entry point, the library entry point, the package entry point, the barrel file, the build script).
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

### FR-NAMINGRULES-002: Suffix/Prefix Validation (AES102)

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
  - **Barrel files and entry points** (the module entry point, the library entry point, the binary entry point, the package entry point, the barrel file, the build script) are skipped.
  - **Rule-level exceptions** (`config.rules[].exceptions.values`) — global exceptions that apply to all layers for a given rule code.
  - **Definition-level exceptions** (`layers[].exceptions.values`) — per-layer exceptions that apply only when the file's layer is recognized.
  - **Layer detection** via the layer detection utility using filename prefix.
- **Per-Layer Suffix Policies** (source of truth: YAML config):

  | Layer        | Policy   | Allowed Suffixes                                                                                              | Forbidden Suffixes                                                                            |
  | -------------- | ---------- | --------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------- |
  | taxonomy     | strict   | `vo`, `entity`, `error`, `event`, `constant`                                                                  | —                                                                                            |
  | contract     | strict   | `protocol`, `aggregate`                                                                                       | —                                                                                            |
  | utility      | flexible | *(any)*                                                                                                       | `vo`, `entity`, `error`, `event`, `constant`, `protocol`, `aggregate`                         |
  | capabilities | flexible | *(any)*                                                                                                       | `vo`, `entity`, `error`, `event`, `constant`, `constants`, `protocol`, `aggregate`, `utility` |
  | agent        | strict   | `orchestrator`                                                                                                | —                                                                                            |
  | surface      | strict   | `command`, `controller`, `page`, `view`, `component`, `router`, `layout`, `hook`, `store`, `action`, `screen` | —                                                                                            |
  | root         | strict   | `entry`, `container`                                                                                          | —                                                                                            |
- **Edge Cases**:

  - Files with no suffix (single-word stem after prefix, e.g., `taxonomy_user`) → fails strict policy check (AES102 `SuffixMismatch`).
  - Multiple valid suffixes for a layer (e.g., taxonomy allows `_vo`, `_entity`, `_error`, `_event`, `_constant`) → all pass.
  - Custom or unknown layers without a definition → skipped (no definition means no suffix policy).
  - Prefix-suffix mismatch across layers (e.g., `contract_user_vo`) → AES102 (`PrefixSuffixMismatch`).
  - The build script is skipped (in exceptions list).
- **Error Handling**: Emit AES102 with the layer name, used suffix, prefix, and the full allowed/forbidden lists. For prefix-suffix mismatch, include expected layer and actual suffix layer.

---

## API Contract

### Protocol API

| Method | Input | Output | Error | Event | Description |
| --- | --- | --- | --- | --- | --- |
| `run_audit_with_entries` | `&[FileEntry]`, architecture config, layer map | Lint results | Parsing or configuration errors | — | Single composite entry point: converts FileEntry to FilePathList, filters empty-content entries, and runs AES101/AES102 checks. |

### Aggregate API

| Method | Input | Output | Error | Event | Description |
| --- | --- | --- | --- | --- | --- |
| `run_audit_with_entries` | `&[FileEntry]`, config, layer map | Lint results | Configuration error | — | Aggregate orchestrator method; performs FileEntry → FilePathList conversion and dispatches to sub-checkers. |
| `check_naming_convention` | `FilePathList`, config, layer map | AES101 violations | None | — | Validate that every stem is snake_case with the configured minimum word count. |
| `check_suffix_prefix` | `FilePathList`, config, layer map | AES102 violations | None | — | Validate suffix alignment with the layer's policy and prefix-suffix cross-consistency. |

---

## Integration Points

| System | Direction | Purpose | Failure mode |
| --- | --- | --- | --- |
| Configuration system (shared crate) | in | Read architecture YAML for layer definitions, naming rules, exceptions, and ignored paths | Missing or invalid config → error propagation to caller |
| Taxonomy definitions (shared crate) | in | Provide layer map and layer name value objects for prefix-based layer detection | Undefined layer → file skipped without suffix check |
| Path value objects (shared crate) | in | Supply barrel and entry-point detection helpers | Parse failure → file excluded from audit |
| Filesystem aggregate (filesystem crate) | in | Provide pre-populated `FileEntry[]` list and path-existence checks | Aggregate unavailable → audit cannot run; caller must handle |
| Surface layer | out | Receives `LintResult` and persists/report output | Report formatting error → logged; audit continues |

---

## Non-functional Requirements

| Metric | Target | Measurement method |
| --- | --- | --- |
| Performance | Walk and check 1,000 source files in < 1 second | Measure wall-clock time over a 1,000-file workspace |
| Memory | O(1) per file for checker state; regex cache is a static `OnceLock` table with one slot per word count 1–10 | Profile RSS during audit of 1,000 files |
| Accuracy | Zero false positives for correctly named files; zero false negatives for naming or suffix/prefix violations | Run audit against a known-clean workspace and a known-violating workspace |

---

## Test Scenarios

- A valid snake_case file with three or more words and a recognized layer prefix (`taxonomy_user_vo`) produces no violation.
- A file with uppercase characters in the stem (`Taxonomy_User_Vo`) produces an AES101 diagnostic for invalid snake_case.
- A file with only two words (`taxonomy_user`) produces an AES101 diagnostic for too few words.
- A file with hyphens in the stem (`taxonomy-user-vo`) produces an AES101 diagnostic for invalid separator.
- A file with dots in the stem (`taxonomy.user.vo`) produces an AES101 diagnostic for invalid character.
- A barrel file (the module entry point, the package entry point, or the barrel file) produces no violation as an exception.
- A file in the exception list (such as the binary entry point or the library entry point) produces no violation as an exception.
- A structurally valid file with three words while `min_words` is configured to 5 produces an AES101 diagnostic for being below the configured minimum.
- A file with an unrecognized prefix (`foobar_user_vo`) produces no violation, because an unknown prefix is out of scope: AES101 passes and AES102 skips.
- A file with digits in a segment (`taxonomy_v2_vo`) produces no violation, because digits are allowed.
- A file whose prefix and suffix both belong to the taxonomy layer with the suffix in the strict allow-list (`taxonomy_user_vo`) produces no violation.
- A file whose suffix belongs to the contract layer but whose prefix is taxonomy (`taxonomy_user_protocol`) produces an AES102 prefix-suffix mismatch.
- A file whose suffix belongs to the taxonomy layer but whose prefix is contract (`contract_user_vo`) produces an AES102 prefix-suffix mismatch.
- A file in the agent layer with a suffix outside its strict allow-list (`agent_user_helper`) produces an AES102 suffix mismatch, because the agent layer requires the `orchestrator` suffix.
- A file in the utility layer with a suffix that is flexible and not forbidden (`utility_user_helper`) produces no violation.
- A file in the utility layer with a suffix on the forbidden list (`utility_user_protocol`) produces an AES102 forbidden suffix.
- A file in the capabilities layer with a suffix on the forbidden list (`capabilities_user_vo`) produces an AES102 forbidden suffix.
- A file in the capabilities layer with a suffix that is flexible and not forbidden (`capabilities_user_checker`) produces no violation.
- A file in the surface layer with a suffix in its strict allow-list (`surface_user_command`) produces no violation.
- A file in the surface layer with a suffix outside its strict allow-list (`surface_user_helper`) produces an AES102 suffix mismatch.
- A file in the root layer with a suffix in its strict allow-list (`root_app_entry`) produces no violation.
- A file in the root layer with a suffix outside its strict allow-list (`root_app_helper`) produces an AES102 suffix mismatch.
- The build script produces no violation, because it is in the exceptions list.
- A file in the exception list for its layer produces no violation as an exception.
- A file with no suffix beyond the prefix (`taxonomy_user`) produces an AES102 suffix mismatch, because strict policy requires a suffix.
- When rule AES101 is disabled in configuration, no AES101 violations are emitted.
- When rule AES102 is disabled in configuration, no AES102 violations are emitted.
- A file present in the exceptions list produces no violation for that file.

---

## Assumptions & Constraints

- Layer hierarchy and naming policies are defined in the architecture configuration YAML.
- File naming follows AES conventions (`prefix_concept_suffix` pattern).
- Exceptions are configurable per rule in the rule's `exceptions` list.
- Ignored paths (`node_modules`, `.git`, `target`) are excluded from scanning by the filesystem crate.
- The crate receives pre-populated `&[FileEntry]` from the surface layer (which fetches from `filesystem.file_list()`). No file walking, directory traversal, or filesystem I/O is performed internally.
- Layer detection is based on filename prefix (hardcoded AES convention: `taxonomy_*`, `contract_*`, `utility_*`, `capabilities_*`, `agent_*`, `surface_*`, `root_*`).
- Naming validation is a prerequisite for import-rules layer detection. Files that fail naming validation may cause incorrect layer assignment in downstream crates.

---

## Glossary

- **AES**: Agentic Engineering System — the 7-layer architecture framework.
- **Layer**: Architectural boundary (taxonomy, contract, utility, capabilities, agent, surface, root).
- **Suffix**: Last underscore-separated token in the filename indicating role (`vo`, `protocol`, `orchestrator`, `checker`, etc.).
- **Prefix**: First underscore-separated token in the filename identifying the architectural layer (`taxonomy`, `contract`, `utility`, etc.).
- **Stem**: Filename without extension (e.g., `capabilities_user_checker`).
- **Strict suffix policy**: Layer requires suffix to be in an explicit allow-list. Any other suffix is rejected.
- **Flexible suffix policy**: Layer allows any suffix EXCEPT those in the forbidden list.
- **Forbidden suffix**: Suffix explicitly banned for a layer (belongs to another layer's domain).
- **Prefix-suffix mismatch**: File prefix indicates one layer but suffix belongs to a different layer's suffix set.
- **Filesystem crate**: External crate that handles file walking, directory traversal, and file filtering. Caches `FileEntry[]` in `OnceLock`. Surface layer fetches via `file_list()` and passes to naming-rules.
- **Unreadable skip**: Files with empty content (parse failures from the filesystem crate) are skipped silently; no separate warning diagnostic is emitted.

---

## Appendix A: YAML Configuration Schema

### Top-Level Structure

```yaml
architecture:
  enabled: true
  rules:
    AES101: { ... }
    AES102: { ... }
```

### Suffix Policy Schema

```yaml
layers:
  <layer_name>:
    suffix:
      - strict: ["<suffix>", ...]      # Only these suffixes allowed
      # OR
      - flexible: []                    # Any suffix allowed (except forbidden)
      - forbidden: ["<suffix>", ...]   # These suffixes are banned
```

**Policy semantics**:

- `strict`: Whitelist. Suffix MUST be in the list. Anything else → AES102 `SuffixMismatch`.
- `flexible`: Open. Any suffix allowed UNLESS in `forbidden` list. Forbidden → AES102 `SuffixForbidden`.
- `strict` and `flexible` are mutually exclusive per layer. `forbidden` can coexist with `flexible`.

---
