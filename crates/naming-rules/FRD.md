# FRD — naming-rules

## System Overview

The naming-rules crate enforces strict naming conventions across the codebase to ensure consistency, readability, and adherence to the 7-layer architecture. It validates that files and identifiers conform to structural and semantic naming patterns, preventing naming chaos and ensuring every file can be correctly assigned to an architectural layer.

```
┌──────────────────────────────────────────────────────────────┐
│                     NamingOrchestrator                       │
│  (agent layer — walks files, delegates to checkers)          │
├──────────────────────┬───────────────────────────────────────┤
│ NamingConvention-    │ SuffixPrefixChecker                  │
│ Checker (AES101)     │ (AES102)                              │
│ - snake_case regex   │ - allowed/forbidden suffix per layer  │
│ - min 3 words        │ - strict suffix policy enforcement    │
│ - unknown prefix     │ - unknown suffix detection            │
├──────────────────────┴───────────────────────────────────────┤
│ Shared utilities: layer detection, stem/suffix extraction    │
│ Config: ArchitectureConfig → layer definitions, naming rules │
└──────────────────────────────────────────────────────────────┘
```

## Functional Requirements

### FR-001: Naming Convention Consistency (AES101)

- **Description**: Every file stem must be snake_case with at least 3 underscore-separated words in `prefix_concept_suffix` pattern.
- **Input**: File path
- **Output**: AES101 diagnostic if invalid, or AES000 (unknown prefix) if no layer can be detected
- **Business Rules**:
  - Must be snake_case (lowercase ASCII + underscores)
  - Must follow `prefix_concept_suffix` pattern (minimum N words, configurable, default 3)
  - Word count is read from `config.naming.word_count.value`; falls back to 3 if non-positive
  - A dynamic regex `^[a-z0-9.]+(_[a-z0-9.]+){N-1,}$` is compiled once and cached per word count
  - If the file has no recognized layer prefix, AES000 is emitted with the unknown prefix and a list of allowed prefixes
  - Exceptions: barrel files (`mod.rs`, `__init__.py`, `index.ts`, `index.js`) and any file listed in `LayerDefinition.exceptions` are skipped
- **Edge Cases**:
  - Files with uppercase letters, hyphens, or dots instead of underscores — caught by regex
  - Abbreviations like `db` or `http` — allowed as long as they are lowercase and separated by underscores
  - Files in unknown directories (no detectable layer) — fall back to AES000 unknown prefix check
- **Error Handling**: Emit AES101 with the invalid stem, expected pattern, and minimum word count; emit AES000 with the unrecognized prefix and list of valid prefixes

### FR-002: Suffix/Prefix Layer Alignment (AES102)

- **Description**: File suffix must align with the architectural layer it belongs to; forbidden suffixes from other layers are rejected.
- **Input**: File path
- **Output**: AES102 diagnostic if suffix is forbidden or mismatches the layer's allowed list
- **Business Rules**:
  - Each layer has an allowed suffix list and a forbidden suffix list defined in `LayerDefinition.naming`
  - Suffix is extracted as the last underscore-separated token from the stem
  - If a suffix appears in the layer's `forbidden_suffix` list, it is immediately rejected (AES102 with `SuffixForbidden`)
  - If the layer uses `suffix_policy = strict`, only suffixes in the `allowed_suffix` list are permitted (AES102 with `SuffixMismatch`)
  - Barrel files and entry points are skipped
  - Files in the layer's exception list are skipped
  - Layers detected via `detect_layer_from_prefix` + `resolve_specialized_layer` for sub-layer routing
- **Edge Cases**:
  - Files with no suffix (suffix = None) — fails strict policy check
  - Multiple valid suffixes for a layer (e.g., taxonomy allows `_vo`, `_entity`, `_error`, `_event`, `_constant`) — all pass
  - Custom or unknown layers without a definition — skipped (no def means no suffix policy)
- **Error Handling**: Emit AES102 with the layer name, used suffix, and the full allowed/forbidden lists

## Data Model / Entity Relationship

```
NamingViolation (VO)
  ├── NamingConvention { min_words, separator, reason }
  ├── SuffixForbidden { layer_name, forbidden_suffix, reason }
  ├── SuffixMismatch { layer_name, used_suffix, allowed: Vec<String>, reason }
  └── UnknownPrefix { prefix, allowed: Vec<String>, reason }

LayerMapVO
  └── values: HashMap<LayerNameVO, LayerDefinition>
        ├── naming: NamingConfig
        │     ├── allowed_suffix: Vec<String>
        │     ├── forbidden_suffix: Vec<String>
        │     └── suffix_policy: String ("strict" | "flexible")
        ├── exceptions: Vec<String>  (filenames to skip)
        └── code_analysis: CodeAnalysisConfig

ArchitectureConfig
  └── naming: NamingConfigVO
        └── word_count: { value: i64 }  (min words, default 3)
```

## API Contract

| Function | Input | Output | Description |
| --- | --- | --- | --- |
| `INamingConventionChecker::check_file_naming()` | config, layer_map, files, root_dir, results | Mutates results | Scan all files; emit AES101/AES000 for naming violations |
| `ISuffixPrefixChecker::check_domain_suffixes()` | config, layer_map, files, root_dir, results | Mutates results | Scan all files; emit AES102 for forbidden/mismatched suffixes |
| `INamingRunnerAggregate::run_audit()` | target: FilePath | `Result<Vec<LintResult>, ScanError>` | Walk directory, filter source files, run both checkers |
| `NamingConventionChecker::naming_regex()` | min_words: usize | Option<Static Regex> | Build/cache regex for given min word count |
| `NamingConventionChecker::min_words_from_config()` | config: &ArchitectureConfig | usize | Extract min words with fallback to 3 |

## Integration Points

- **Internal**:
  - `shared::config_system` — reads `ArchitectureConfig` YAML for layer definitions, naming rules, exceptions, ignored paths
  - `shared::taxonomy_definition_vo` — `LayerMapVO`, `LayerDefinition`, `LayerNameVO` for layer metadata
  - `shared::common::utility_layer_detector` — filename prefix detection and specialized layer resolution
  - `shared::common::taxonomy_path_vo` — barrel/entry-point detection via `FilePath::is_barrel_file()` / `is_entry_point()`
- **External**: None

## Non-functional Requirements (Detailed)

- Performance: Walk and check 1000 source files in < 1 second (regex compiled once, O(n) per file)
- Memory: O(1) per file for checker state; regex cache limited to 10 static slots (word counts 0–9)
- Accuracy: Zero false positives for files that match the naming pattern and have valid layer suffixes

## Test Scenarios / QA Checklist

| # | Input | Expected Output | Rule |
|---|-------|-----------------|------|
| 1 | `capabilities_user_checker.rs` | No violation (valid 3-word snake_case with layer prefix) | AES101 pass |
| 2 | `capabilities_UserChecker.rs` | AES101 — uppercase characters violate snake_case | AES101 |
| 3 | `capabilities_user.rs` | AES101 — only 2 words (min is 3) | AES101 |
| 4 | `capabilities-user-checker.rs` | AES101 — hyphens instead of underscores | AES101 |
| 5 | `main.rs` | No violation (barrel/entry exception) | exception |
| 6 | `mod.rs` | No violation (barrel/entry exception) | exception |
| 7 | `capabilities_user_checker.rs` (taxonomy layer, suffix `_checker` not in allowed list) | AES102 — suffix mismatch | AES102 |
| 8 | `taxonomy_user_vo.rs` (taxonomy layer, suffix `_vo` allowed) | No violation | AES102 pass |
| 9 | `agent_helper.rs` (agent layer, `_helper` is forbidden) | AES102 — forbidden suffix | AES102 |
| 10 | `custom_foo_bar.rs` (prefix `custom` not in LAYER_PREFIXES) | AES000 — unknown prefix | AES000 |
| 11 | `capabilities_user_checker.rs` (min_words configured to 5) | AES101 — only 3 words, need 5 | AES101 |
| 12 | `root_container.rs` (root layer, suffix `_container` allowed) | No violation | AES102 pass |

## Assumptions & Constraints

- Layer hierarchy and naming policies are defined in the ArchitectureConfig YAML
- File naming follows AES conventions (prefix_layer_concept_suffix pattern)
- Exceptions are configurable per layer in `LayerDefinition.exceptions`
- Ignored paths (node_modules, .git, target) are excluded from scanning
- The crate operates on a pre-filtered list of source files (no binary or non-lintable files)

## Glossary

- **AES**: Agentic Engineering System — the 7-layer architecture framework
- **Layer**: Architectural boundary (taxonomy, contract, utility, capabilities, agent, surface, root)
- **Suffix**: File name ending indicating role (`_vo`, `_protocol`, `_orchestrator`, `_checker`, etc.)
- **Prefix**: First underscore-separated word in the filename identifying the architectural layer
- **Stem**: Filename without extension (e.g., `capabilities_user_checker`)
- **Strict suffix policy**: Layer requires suffix to be in an explicit allow-list
- **Forbidden suffix**: Suffix explicitly banned for a layer (belongs to another layer's domain)

## Reference

- PRD: [PRD.md](../../PRD.md)
