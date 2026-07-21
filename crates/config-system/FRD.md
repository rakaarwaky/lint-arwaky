# FRD — Config System

## Feature Goal

The config-system crate manages lint-arwaky configuration: loading, parsing, validation, and workspace detection. It reads `lint_arwaky.config.*.yaml` files from multiple priority sources, merges them with embedded defaults, and provides a unified configuration facade for all other lint crates.

---

## User Stories

### US-1: Project Config Discovery
> **As a** developer running `lint-arwaky check`,
> I need the system to find my project's config file automatically,
> **so that** linting uses my project-specific AES rules without manual setup.

### US-2: Multi-Language Support
> **As a** polyglot developer,
> I need the system to detect whether my workspace is Rust, Python, or TypeScript and load the correct config,
> **so that** language-appropriate architecture rules are applied.

### US-3: Config Fallback Safety
> **As a** developer without a config file,
> I need sensible defaults so that linting works out of the box,
> **so that** I can start using lint-arwaky immediately.

### US-4: Multi-Workspace Analysis
> **As a** monorepo maintainer,
> I need the system to discover and load configs for all workspace members (crates/, packages/, modules/),
> **so that** each module gets its own ruleset.

### US-5: Config Security
> **As a** security-conscious developer,
> I need config file reads to be confined within the project root and reject symlinks pointing outside,
> **so that** malicious config files cannot read arbitrary files from my system.

---

## Acceptance Criteria

### AC-1: Config Resolution Priority Chain

The config resolution follows this exact priority order (first match wins):

1. **Project-root YAML** — `lint_arwaky.config.{lang}.yaml` in the workspace root
2. **Parent directory YAML** — same filename in parent directories, up to depth 3
3. **XDG user config** — `~/.config/lint-arwaky/lint_arwaky.config.{lang}.yaml`
4. **XDG system dirs** — `/etc/xdg/lint-arwaky/lint_arwaky.config.{lang}.yaml` (and `$XDG_CONFIG_DIRS/*/lint-arwaky/`)
5. **Embedded defaults** — compiled-in YAML from `lint_arwaky.config.*.yaml` files

### AC-2: Language-Aware Config Files

| Language | Config File(s) |
|----------|---------------|
| Rust | `lint_arwaky.config.rust.yaml` |
| Python | `lint_arwaky.config.python.yaml` |
| TypeScript | `lint_arwaky.config.typescript.yaml`, `lint_arwaky.config.javascript.yaml` (fallback) |

TypeScript and JavaScript share the same config file priority. When looking for TypeScript, the system first tries `.typescript.yaml`, then falls back to `.javascript.yaml`.

### AC-3: Error Handling

- `read_config()` returns `Result<Option<ConfigSource>, ConfigError>` — failures are explicit
- YAML parse failures produce warnings, not silent defaults
- Rules with empty `conditions: []` are preserved (not dropped)
- Non-NotFound I/O errors produce warnings via `eprintln!`

### AC-4: Security Constraints

- Config file reads use canonical path resolution to prevent symlink escapes
- Symlinks pointing outside the project root are rejected
- Config files exceeding 1 MiB (`MAX_CONFIG_FILE_SIZE`) are rejected
- XDG_CONFIG_DIRS entries are limited to 8 directories, must be absolute paths

### AC-5: Multi-Workspace Discovery

- `discover_workspace_members()` finds subdirectories under `crates/`, `packages/`, `modules/`
- Uses async I/O (`tokio::fs`) for non-blocking filesystem operations
- Concurrency bounded to 8 concurrent workspace loads via `buffered(8)`
- Parsed configs cached by file path to avoid repeated YAML parsing

---

## Architecture Overview

### Layer Structure (AES Compliance)

```
┌─────────────────────────────────────────┐
│           Surface Layer                 │
│  surface_config_command.rs              │
├─────────────────────────────────────────┤
│           Agent Layer                   │
│  agent_config_orchestrator.rs           │
├─────────────────────────────────────────┤
│        Capabilities Layer               │
│  capabilities_yaml_reader.rs            │
│  capabilities_workspace_detector.rs     │
│  capabilities_rules_validator.rs        │
│  capabilities_parser_provider.rs        │
├─────────────────────────────────────────┤
│         Contract Layer                  │
│  contract_*.rs (protocols + aggregate)  │
├─────────────────────────────────────────┤
│         Taxonomy Layer                  │
│  taxonomy_*.rs (VOs, errors)            │
├─────────────────────────────────────────┤
│         Utility Layer                   │
│  utility_config_*.rs                    │
└─────────────────────────────────────────┘
```

### Key Contracts

| Contract | Purpose |
|----------|---------|
| `IConfigReaderProtocol` | Read config from filesystem (Result-based error handling) |
| `IConfigParserProtocol` | Parse YAML/TOML project configs |
| `IConfigValidatorProtocol` | Validate loaded rules against schema |
| `IWorkspaceDetectorProtocol` | Detect workspace type and discover members |
| `IConfigOrchestratorAggregate` | High-level facade for config loading |

### Key Value Objects

| VO | Purpose |
|----|---------|
| `ArchitectureConfig` | Parsed AES architecture rules |
| `ArchitectureRule` | Individual rule definition |
| `ConfigSource` | Config file with language, path, and raw content |
| `ConfigResult` | Parsed config + source info + warnings |
| `ConfigError` | Structured error for config operations |
| `ConfigLanguage` | Typed enum (Rust/Python/TypeScript) — prevents path injection |
| `WorkspaceInfo` | Workspace member with language and config |

---

## Merge Strategy

### Field-Level Merge Rules (`utility_config_merger.rs`)

1. **Layers** — concatenated; later definitions override earlier ones for the same layer name
2. **Rules** — concatenated; rules are deduplicated by `name` field
3. **Naming** — merged recursively; non-empty values override defaults
4. **Ignored paths** — concatenated and deduplicated

### Conflict Resolution

- When the same layer is defined in multiple configs, the deeper (more specific) config wins
- Rules with duplicate names are deduplicated by keeping the first occurrence
- Empty arrays/objects in a child config do NOT override parent values

---

## Non-Functional Requirements

| ID | Requirement | Target |
|----|-------------|--------|
| NFR-1 | Config read from project root | < 50ms (local filesystem) |
| NFR-2 | Config read from XDG paths | < 100ms (filesystem + env parsing) |
| NFR-3 | Workspace discovery for 10 members | < 500ms (with concurrency bound of 8) |
| NFR-4 | Memory overhead per parsed config | < 10 KB (cached) |
| NFR-5 | Symlink attack detection | O(1) path canonicalization check |

---

## Error/Warning Taxonomy

| Level | Condition | Behavior |
|-------|-----------|----------|
| ERROR | Config file exceeds 1 MiB | Reject with `InvalidData` error |
| ERROR | Symlink points outside root | Reject with `PermissionDenied` error |
| ERROR | Invalid path canonicalization | Reject with IO error |
| WARNING | YAML parse failure | Use defaults, log warning |
| WARNING | Non-NotFound I/O error | Log via `eprintln!`, continue searching |
| WARNING | Config has no layers | Inject defaults, log warning |

---

## Implementation Notes

### Why ConfigLanguage enum?

String-based language parameters allow path injection (`language = "../../etc/passwd"`). The `ConfigLanguage` enum restricts input to exactly Rust, Python, and TypeScript, eliminating this attack vector.

### Why Result<Option<ConfigSource>, ConfigError>?

`Option<ConfigSource>` hides the distinction between "file not found" (normal) and "permission denied" (error). Returning `Result` makes failures explicit and actionable.

### Why buffered(8) in workspace discovery?

Unbounded `join_all()` spawns one future per workspace member. For large monorepos (100+ members), this exhausts file descriptors and memory. `buffered(8)` caps concurrent I/O at 8 handles.

---

## Files Summary

### New files (added in fix plan)
- `taxonomy_config_language_vo.rs` — ConfigLanguage typed enum (P2.2)
- `utility_config_io.rs` — path confinement helper `read_text_within_canonical_root` (P2.1)

### Modified files
- `contract_reader_protocol.rs` — Result-based signatures, ConfigLanguage (P3.1)
- `contract_config_orchestrator_aggregate.rs` — removed accessor methods (P5.1)
- `contract_workspace_detector_protocol.rs` — added `discover_workspace_members` (P1.2)
- `capabilities_yaml_reader.rs` — depth 3, aliases, local-only listing, XDG hardening (P2.3/P4.x)
- `capabilities_workspace_detector_provider.rs` — async I/O, discover_workspace_members (P1.2/P6.1)
- `agent_config_orchestrator.rs` — uses contracts, bounded concurrency, caching (P1.3/P6.2/P6.3)
- `root_config_system_container.rs` — exposes reader via `reader()` method
- `mod.rs` (shared) — registers new modules

---

## Success Indicators

- [x] Discovery reliability — workspaces detected from various project structures
- [x] Validation accuracy — invalid configs rejected with clear errors
- [x] Merge correctness — overrides merged without conflicts
- [x] Security — symlink escapes blocked, path confinement enforced
- [x] Performance — bounded concurrency, config caching, async I/O
- [x] AES compliance — layer violations fixed, parser moved to utility, filesystem moved out of agent
