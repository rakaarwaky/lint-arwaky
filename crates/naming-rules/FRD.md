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

### FR-NamingRules-001: Naming Convention (AES101)

- **Description**: Every file stem must be snake_case with at least N underscore-separated words in `prefix_concept_suffix` pattern.
- **Input**: Pre-populated `&[FileEntry]` from filesystem aggregate (via surface), architecture configuration, layer map.
- **Output**:

  - AES101 diagnostic if naming structure is invalid.
- **Business Rules**:

  - Must be snake_case: lowercase ASCII letters (`a-z`), digits (`0-9`), and underscores only. No uppercase, no hyphens, no dots.
  - Must follow `prefix_concept_suffix` pattern with minimum N words (configurable via `config.naming.word_count.value`, default 3.
  - Validation regex: `^[a-z0-9]+(_[a-z0-9]+){N-1,}$` — compiled once per word count and cached in a static `OnceLock` table (one slot per word count 1–10).
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
  - Packaging build scripts are skipped (in exceptions list).
- **Error Handling**: Emit AES102 with the layer name, used suffix, prefix, and the full allowed/forbidden lists. For prefix-suffix mismatch, include expected layer and actual suffix layer.

---

## API Contract

### Protocol API

| Method | Input | Output | Error | Event | Description |
|---|---|---|---|---|---|
| `check_file_naming` | &ArchitectureConfig, &LayerMapVO, &FilePathList, &FilePath, &mut LintResultList | `` | — | — | Check file naming. |
| `check_domain_suffixes` | &ArchitectureConfig, &LayerMapVO, &FilePathList, &FilePath, &mut LintResultList | `` | — | — | Check domain suffixes. |

### Aggregate API

| Method | Input | Output | Error | Event | Description |
|---|---|---|---|---|---|
| `execute` | NamingRequest | `NamingResponse` | — | — | Single composite entry point over the feature. |

## Integration Points

| System | Direction | Purpose | Failure mode |
| --- | --- | --- | --- |
| `shared` config module | in | Supply layer definitions, naming rules, exceptions, and ignored paths | A layer is absent from config → files under that prefix are validated structurally only |
| `shared` taxonomy module | in | Supply the layer map and layer-name value objects | A prefix maps to no layer → the file is skipped by the suffix policy and checked structurally |
| `shared` path module | in | Supply barrel and entry-point detection used by the exception list | A path cannot be classified → the default exception list applies |
| `filesystem` aggregate | in | Walk the workspace, filter by extension, and apply ignore rules | A file cannot be read → it is excluded from the discovered set and the rest of the walk proceeds |
| Surface layer | in | Hand over the pre-fetched file entries | The caller supplies no entries → the orchestrator returns an empty result set and performs no I/O of its own |
| Layer prefix map | out (internal) | Map a filename prefix to its architectural layer | A prefix is unknown → the file is skipped by the suffix policy rather than misclassified |

## Non-functional Requirements

| Metric | Target | Measurement method |
| --- | --- | --- |
| 1,000-file check | Under 1 s | Time a full walk and check over a 1,000-file workspace |
| Per-file cost | O(n) in the file's content length | Scale one file's length tenfold and confirm linear growth |
| Checker state | O(1) per file | Measure retained state during a scan and confirm it does not grow with file count |
| Regex cache | One compiled slot per word count from 1 to 10, populated lazily | Read the cache size before and after a scan and confirm it matches the distinct word counts seen |
| Naming accuracy | Zero false positives on correctly named files | Assert the good-workspace fixtures report zero violations |
| Suffix-policy completeness | Zero false negatives against the configured suffix policy | Assert every file violating the configured policy is reported |
| Determinism | Results depend only on file content, never on iteration order | Run the same scan twice and compare the finding sets byte for byte |
| Zero I/O | The rule performs no filesystem access of its own | Run the check with filesystem access denied and assert it still produces findings |

## Test Scenarios / QA Checklist

Each scenario is stated below as a table of cases: the input condition and the expected result.

- **AES101 — Naming Convention** — e.g. Valid snake_case file, 3+ words, recognized layer prefix (`taxonomy_user_vo`) → No violation
- **AES102 — Suffix/Prefix Validation** — e.g. `taxonomy_user_vo` — prefix taxonomy, suffix vo (in strict allow-list) → No violation
- **Configuration** — e.g. Rule AES101 disabled in config → No AES101 violations

### AES101 — Naming Convention

| # | Input Scenario | Expected Output |
| - | - | - |
| 1 | Valid snake_case file, 3+ words, recognized layer prefix (`taxonomy_user_vo`) | No violation |
| 2 | File with uppercase characters in stem (`Taxonomy_User_Vo`) | AES101 — invalid snake_case |
| 3 | File with only 2 words (`taxonomy_user`) | AES101 — too few words |
| 4 | File with hyphens (`taxonomy-user-vo`) | AES101 — invalid separator |
| 5 | File with dots (`taxonomy.user.vo`) | AES101 — invalid character |
| 6 | Barrel file (module barrel, package marker, or index barrel) | No violation — exception |
| 7 | File in exception list (binary entry or library root) | No violation — exception |
| 8 | Valid file but`min_words` config set to 5, file has 3 words | AES101 — below configured min |
| 9 | File with unrecognized prefix (`foobar_user_vo`) | No violation — unknown prefix out of scope (AES101 pass, AES102 skip) |
| 10 | File with digits in segment (`taxonomy_v2_vo`) | No violation (digits allowed) |

### AES102 — Suffix/Prefix Validation

| # | Input Scenario | Expected Output |
| - | - | - |
| 1 | `taxonomy_user_vo` — prefix taxonomy, suffix vo (in strict allow-list) | No violation |
| 2 | `taxonomy_user_protocol` — prefix taxonomy, suffix protocol (belongs to contract) | AES102 — prefix-suffix mismatch |
| 3 | `contract_user_vo` — prefix contract, suffix vo (belongs to taxonomy) | AES102 — prefix-suffix mismatch |
| 4 | `agent_user_helper` — prefix agent, suffix helper (not in strict allow-list) | AES102 — suffix mismatch (agent requires`orchestrator`) |
| 5 | `utility_user_helper` — prefix utility, suffix helper (flexible, not forbidden) | No violation |
| 6 | `utility_user_protocol` — prefix utility, suffix protocol (in forbidden list) | AES102 — forbidden suffix |
| 7 | `capabilities_user_vo` — prefix capabilities, suffix vo (in forbidden list) | AES102 — forbidden suffix |
| 8 | `capabilities_user_checker` — prefix capabilities, suffix checker (flexible, not forbidden) | No violation |
| 9 | `surface_user_command` — prefix surface, suffix command (in strict allow-list) | No violation |
| 10 | `surface_user_helper` — prefix surface, suffix helper (not in strict allow-list) | AES102 — suffix mismatch |
| 11 | `root_app_entry` — prefix root, suffix entry (in strict allow-list) | No violation |
| 12 | `root_app_helper` — prefix root, suffix helper (not in strict allow-list) | AES102 — suffix mismatch |
| 13 | Packaging build script | No violation — exception |
| 14 | File in exception list for its layer | No violation — exception |
| 15 | `taxonomy_user` — no suffix (single word after prefix) | AES102 — suffix mismatch (strict policy requires suffix) |

### Configuration

| # | Scenario | Expected |
| - | - | - |
| 1 | Rule AES101 disabled in config | No AES101 violations |
| 2 | Rule AES102 disabled in config | No AES102 violations |
| 3 | File in exceptions list | No violation for that file |

> **Per-rule toggling:** Each rule can be enabled/disabled via `config.rules[].enabled` (boolean, default `true`). The orchestrator checks this field via `is_rule_enabled()` before invoking the corresponding checker. When `enabled: false`, the checker is not called and zero violations are emitted for that rule.

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

- **AES**: Agentic Engineering System — the 7-layer architecture framework
- **Layer**: Architectural boundary (taxonomy, contract, utility, capabilities, agent, surface, root)
- **Suffix**: Last underscore-separated token in the filename indicating role (`vo`, `protocol`, `orchestrator`, `checker`, etc.)
- **Prefix**: First underscore-separated token in the filename identifying the architectural layer (`taxonomy`, `contract`, `utility`, etc.)
- **Stem**: Filename without extension (e.g.,`capabilities_user_checker`)
- **Strict suffix policy**: Layer requires suffix to be in an explicit allow-list. Any other suffix is rejected.
- **Flexible suffix policy**: Layer allows any suffix EXCEPT those in the forbidden list.
- **Forbidden suffix**: Suffix explicitly banned for a layer (belongs to another layer's domain)
- **Prefix-suffix mismatch**: File prefix indicates one layer but suffix belongs to a different layer's suffix set
- **Filesystem crate**: External crate that handles file walking, directory traversal, and file filtering. Caches`FileEntry[]` in `OnceLock`. Surface layer fetches via `file_list()` and passes to naming-rules.
- **Unreadable skip**: Files with empty content (parse failures from the filesystem crate) are skipped silently; no separate warning diagnostic is emitted.

---

### Appendix A: YAML Configuration Schema

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
