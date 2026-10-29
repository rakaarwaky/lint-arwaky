# FRD — config-system

---

## Feature Overview

```mermaid
flowchart TD
    subgraph CS ["config-system crate"]
        A["orchestrator"] --> B["yaml reader\n(config discovery)"]
        A --> C["workspace detector\n(type + members)"]
        A --> D["parser provider\n(merge + validate)"]

        B --> E["ConfigSource"]
        C --> F["WorkspaceType"]
        C --> G["workspace member paths"]
        E --> D
        D -->|"rule-based merge\nwith scoped sub-layers"| H["ArchitectureConfig"]
        H --> I["ValidationResult"]
    end

    J["naming-rules"] -->|"config"| A
    K["quality-rules"] -->|"config"| A
    L["role-rules"] -->|"config"| A
    M["import-rules"] -->|"config"| A
    N["orphan-rules"] -->|"config"| A
    O["external-lint"] -->|"config"| A
```

---

## Functional Requirements

### FR-ConfigSystem-001: Config Discovery

| Aspect | Detail|
|--------|-------|
| Description | Locate and load the first config matching a project root and language, following the 5-level priority chain. |
| Business Rules | Search order: project root, then up to 4 ancestor levels, then the XDG user config, then XDG system dirs (max 8 absolute paths), then embedded defaults — first match wins with no cross-level merge, and the matched file merges with embedded defaults via merge_config_with_defaults (FR-ConfigSystem-003). Local project files are tried first in the language's filename order, then parents within the depth bound; `None` means the caller falls back to embedded defaults; every config file present at the root is listed deduplicated by path; a raw YAML string parses to the adapter entry list, and a string with no `adapters:` key yields an empty vector so defaults apply; symlinks leaving the root are rejected and a language-scoped read blocks path injection. |
| Edge Cases | No config anywhere returns `Ok(None)` rather than an error; an empty config string yields no adapters; a symlink target outside the root is refused; a crafted language value cannot escape the root; parent-directory search stops at the depth bound and never leaves the tree. |
| Capability | `ConfigYamlReader` implements `IConfigReadProtocol` (`read_config`, `list_config_files`, `read_config_from_content`). |
| Protocol | `IConfigReadProtocol` in `shared/config_system`. |
| Method | `read_config(&self, project_root: &FilePath, language: ConfigLanguage) -> Result<Option<ConfigSource>, ConfigError>` |
| Input | `project_root: &FilePath`, `language: ConfigLanguage`, or a raw YAML `content: &str` for the content reader. |
| Output | `Result<Option<ConfigSource>, ConfigError>` — first match in the priority chain, or `None` for fallback; the lister returns deduped paths, the content reader returns `Vec<AdapterEntry>`. |
| Error | Missing files are `None`/empty vectors (never hard errors); malformed content reaches the caller as `ConfigError` only when the parse path opts into it; traversal and injection attempts are rejected outright. |
| Event | `lint-arwaky-cli config show` reads through `ConfigRequest::ReadConfig`/`show`; `check`, `ci`, and `scan` consume the same discovery when they load project config. |
| Test | `acceptance_FR_001` — `us1_config_in_project_root_is_found`, `us1_config_in_parent_directory_is_found`; `acceptance_FR_005` — `us5_symlink_outside_root_is_rejected`, `us5_config_language_prevents_path_injection`; `unit_config_system_yaml_reader` — `read_config_searches_parent_directories_up_to_depth_3`, `read_config_returns_none_when_no_file_found`, `list_config_files_finds_unified_config`. |
| Metric | A project with a config is found on the first correct priority level every run; with none present, loads return `None` from defaults within a single walk of the bounded parent chain. |

### FR-ConfigSystem-002: Multi-Workspace

| Aspect | Detail|
|--------|-------|
| Description | Detect workspace type, probe workspace roots, and enumerate member directories under the layout roots. |
| Business Rules | Workspace type comes from marker files (`Cargo.toml`, `pyproject.toml`, `package.json`, and Python secondary markers); a root is a workspace when it holds `crates/`, `packages/`, or `modules/`; members are the immediate directories of those layout roots; detection probes marker files first and folder structure second. |
| Edge Cases | A plain directory with none of the markers is `NonWorkspace`; an empty but valid workspace root returns zero members rather than an error; one root holding several layout folders contributes all of them; a marker file alone without layout folders is detected as a typed project but reports non-workspace until layout roots exist. |
| Capability | `WorkspaceDetector` implements `IWorkspaceMembersProtocol` (`detect`, `is_workspace`, `discover_workspace_members`). |
| Protocol | `IWorkspaceMembersProtocol` in `shared/config_system`. |
| Method | `discover_workspace_members(&self, root: &FilePath) -> Vec<FilePath>` (with `detect(&self, path: &FilePath) -> WorkspaceType` and `is_workspace(&self, path: &FilePath) -> bool`). |
| Input | `root: &FilePath` — the candidate workspace root (or any path, for `detect`/`is_workspace`). |
| Output | `Vec<FilePath>` — one member directory per layout entry; `WorkspaceType` for detection; a boolean for the workspace probe. |
| Error | No I/O failures are surfaced: an unreadable root behaves like an empty one, and enumeration returns an empty vector rather than an error. |
| Event | `check` runs `ConfigRequest::DiscoverWorkspaces { root }` to scope a run; `ci` and multi-member `scan` rely on the same enumeration. |
| Test | `acceptance_FR_004` — `us4_discovers_all_workspace_member_types`, `us4_each_member_gets_own_config`, `us4_empty_workspace_returns_empty`; `acceptance_FR_002` — per-language workspace config loads; `unit_config_system_workspace_detector` — `detect_rust_workspace_by_cargo_toml`, `detect_typescript_workspace_by_package_json`, `detect_python_workspace_by_pyproject_toml`. |
| Metric | Every workspace with layout roots enumerates exactly its member directories — zero false roots, zero misses — as pinned by `us4_discovers_all_workspace_member_types`. |

### FR-ConfigSystem-003: Config Merger

| Aspect | Detail|
|--------|-------|
| Description | Merge parsed configuration into embedded defaults with rule-based layer merging, including scoped sub-layer creation. |
| Business Rules | YAML parses into `ProjectConfig` (wrapping `ArchitectureConfig`); warnings are collected rather than thrown; merging overlays user rules onto embedded defaults so a user rule wins on conflict and untouched layers keep defaults; TOML reads only the `tool.lint-arwaky` section — a missing section returns `None` rather than an error; an empty config file uses defaults; adapter enablement resolves by first occurrence. |
| Edge Cases | A YAML file that does not exist fails with `ConfigError` file-not-found; invalid YAML fails with a parse error; an empty file keeps defaults; a TOML file without the tool section returns `None`; underscore key aliases are accepted; score thresholds outside 0–100 fail validation later at the orchestrator. |
| Capability | `ConfigParserProvider` implements `IConfigMergeProtocol` (`parse_yaml_config`, `parse_config_yaml_with_warnings`, `merge_config_with_defaults`, `parse_toml_config`). |
| Protocol | `IConfigMergeProtocol` in `shared/config_system`. |
| Method | `merge_config_with_defaults(&self, ...)` alongside `parse_yaml_config(&self, path: &FilePath) -> Result<ProjectConfig, ConfigError>` and `parse_toml_config(&self, path: &FilePath) -> Result<Option<ProjectConfig>, ConfigError>` |
| Input | `path: &FilePath` pointing at a YAML or TOML config, or a raw YAML `yaml_str: &str` for the warning-carrying parse. |
| Output | `Result<ProjectConfig, ConfigError>` for file parses; `(ArchitectureConfig, Vec<String>)` for the warning parse; merged config plus warnings from `merge_config_with_defaults`. |
| Error | File-not-found and invalid-YAML/TOML return `Err(ConfigError)`; a TOML without the tool section returns `Ok(None)`; warnings never downgrade a successful merge. |
| Event | Config loading for every command path that calls `ConfigRequest::LoadProjectConfig`/`LoadSync` (loaded once and cached by the orchestrator before rules apply). |
| Test | `unit_config_system_parser_provider` — `parse_yaml_config_happy_path`, `parse_yaml_config_file_not_found`, `parse_yaml_config_invalid_yaml`, `parse_toml_config_with_tool_section`, `parse_toml_config_returns_none_without_tool_section`; `unit_config_system_config_merger` — `merge_empty_config`, `merge_global_rule`; `acceptance_FR_003` — `us3_no_config_file_uses_defaults`, `us3_defaults_are_valid_and_usable`. |
| Metric | A project with no config always scores through valid embedded defaults (`us3_defaults_are_valid_and_usable`), and every successful merge reports zero unexplained warnings. |

## Public Interface

| Aspect | Detail|
|--------|-------|
| Agent | `ConfigOrchestrator` — the agent-layer orchestrator for config (built from `ConfigOrchestratorDeps`; the companion `ConfigFileReader` agent fronts the file-read entry on the same feature). |
| Aggregate | `IConfigOrchestratorAggregate` — the feature's single front door. |
| Method | `execute(&self, request: ConfigRequest) -> ConfigResponse`. |
| Request | `ConfigRequest` — variants `LoadProjectConfig`, `LoadForLanguage`, `ReadConfig`, `DiscoverWorkspaces`, `LoadSync`, `IgnoredPaths`, `IgnoredPathsForLanguage`. |
| Response | `ConfigResponse` — matching variants carrying `ConfigResult`, `Option<ConfigSource>`, `Vec<WorkspaceInfo>`, `ArchitectureConfig`, or `PatternList`. |

## Reference

| Artifact | Purpose | Link |
|----------|---------|------|
| PRD | Business goals, user stories, and scope | [PRD.md](../../PRD.md) |
| Backlog | Task tracking, epics, and implementation status | [BACKLOG.md](BACKLOG.md) |
| Roadmap | Timeline, phasing, and dependencies | [ROADMAP.md](../../ROADMAP.md) |
| Architecture | System boundaries, tech stack, and infra | [ARCHITECTURE.md](../../ARCHITECTURE.md) |
