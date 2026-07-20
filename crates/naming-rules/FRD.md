# FRD — naming-rules

> Stateless document. Describes the IDEAL TARGET only. Never record progress, status, or current-state notes.

## Feature Goal

The `naming-rules` feature enforces strict naming conventions across the codebase to ensure consistency, readability, and adherence to the 7-layer architecture (Taxonomy → Contract → Utility → Capabilities → Agent → Surface → Root). By validating that files and identifiers conform to structural and semantic naming patterns, it prevents naming chaos and lets developers recognize a component's architectural role from its name alone.

## Scope & Requirements

The `naming-rules` feature audits naming compliance of every scanned source file against the following rules:

### AES101 — Naming Convention Consistency

- **Requirement**: Every file stem (basename without extension) MUST be `snake_case` (lowercase ASCII letters and underscores only), follow the `prefix_concept_suffix` pattern, and contain at least 2 words (prefix + suffix) to avoid cryptic names (e.g. `db.rs` is flagged; `capabilities_db_connector.rs` is accepted).
- **Scope**: All scanned source files (Rust, Python, JavaScript, TypeScript).
- **Exceptions**: `main.rs`, `lib.rs`, `mod.rs`, `root_*_entry.rs` (`root_cli_main_entry.rs`, `root_mcp_main_entry.rs`, `root_tui_main_entry.rs`), `root_composition_container.rs`, `__init__.py`, `index.ts`, `index.js`, barrel/entry files.

### AES102 — Suffix/Prefix Layer Alignment

- **Requirement**: A file's architectural layer is identified by its `prefix_`. Its `suffix` MUST align with that layer's suffix policy.
- **Scope** (prefix → suffix policy):

| Layer prefix      | Policy   | Allowed suffixes (non-exhaustive)                                                                                                              | Forbidden suffixes                                                                                                    |
| ----------------- | -------- | ---------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------- |
| `taxonomy_`     | strict   | `_vo`, `_entity`, `_error`, `_event`, `_constant`, `_utility`, `_helper`                                                         | —                                                                                                                    |
| `contract_`     | strict   | `_protocol`, `_aggregate`                                                                                                                  | —                                                                                                                    |
| `utility_`      | flexible | any role suffix describing the technical responsibility (`_reader`, `_writer`, `_parser`, `_formatter`, …)                            | `_vo`, `_entity`, `_error`, `_event`, `_constant`, `_protocol`, `_aggregate`                            |
| `capabilities_` | flexible | `_checker`, `_analyzer`, `_processor`, `_validator`, `_resolver`, `_calculator`, `_extractor`, `_reporter`, … (role-based)    | `_vo`, `_entity`, `_error`, `_event`, `_constant`, `_utility`, `_helper`, `_protocol`, `_aggregate` |
| `agent_`        | strict   | `_orchestrator`                                                                                                                              | —                                                                                                                    |
| `surface_`      | strict   | `_command`, `_controller`, `_page`, `_view`, `_component`, `_router`, `_layout`, `_hook`, `_store`, `_action`, `_screen` | —                                                                                                                    |
| `root_`         | strict   | `_container`, `_entry`                                                                                                                     | —                                                                                                                    |

## Success Indicators

- [ ] **Accuracy** — Zero false positives: valid `snake_case` stems and correct layer suffixes are never flagged, while invalid ones are caught 100% of the time.
- [ ] **Coverage** — Rust, Python, JavaScript, and TypeScript files are all checked according to configuration.
- [ ] **Layer completeness** — Every canonical layer prefix (`taxonomy_`, `contract_`, `utility_`, `capabilities_`, `agent_`, `surface_`, `root_`) is validated, with `utility_` covering the former `infrastructure_` concerns.
- [ ] **Reporting** — Violations are reported with precise location mappings consumable by the central CLI/MCP runner.
