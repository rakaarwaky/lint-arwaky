# FRD — Config System
## Feature Overview
[lint-arwaky-cli] --> [Dispatcher] --> [ConfigOrchestrator] --> [IConfigOrchestratorAggregate]
[IConfigOrchestratorAggregate] --> [naming-rules / quality-rules / role-rules / import-rules / orphan-rules / external-lint]

The **Config System** feature discovers, loads, merges, and validates the lint-arwaky configuration file for a given project, then publishes a typed configuration aggregate that the downstream rule crates consume. It is the single front door to `lint_arwaky.config.yaml` discovery and to multi-workspace member enumeration; it is called by the dispatcher and by every rule crate that needs thresholds or ignore patterns, and it calls the shared filesystem reader for file I/O. The feature exposes exactly one aggregate trait and one orchestrator; every public behaviour below maps one-for-one to one of the three capability seams behind it.

## Functional Requirements
### FR-ConfigSystem-001: Discover and Load Config
**Description:** The config-system feature shall locate the first valid configuration file for a requested language and project root, following a fixed priority chain, and return the first match to the caller; when no file is found it shall return no match so that the caller falls back to embedded defaults without an error.

#### Business Logic

- The first match in the chain wins; no cross-level merge happens at discovery.
- A raw YAML string passed to the content reader without an `adapters:` key shall yield an empty adapter list so the caller's defaults apply.
- Deduplicated paths: when listing config files in the root, each path appears at most once.
- Symlink targets that resolve outside the project root shall be refused; a crafted language value shall not be able to escape the root.

#### Edge Cases

- An empty config string yields no adapters.
- A symlink target outside the root is rejected outright.
- Parent-directory search stops at the depth bound and never leaves the tree.

#### Technical Specification
| Aspect | Detail |
|--------|--------|
| **Input** | `project_root: &FilePath`, `language: ConfigLanguage`; for the content reader, a raw YAML `content: &str`. Source: dispatcher / CLI caller. |
| **Output** | `Result<Option<ConfigSource>, ConfigError>` for the discovery reader; deduplicated `Vec<FilePath>` for the lister; `Vec<AdapterEntry>` for the content reader. Destination: dispatcher / caller. |
| **Protocol** | `IConfigReadProtocol` |
| **Method** | `read_config` |
| **Event** | `ConfigDiscoveryCompleted` |
| **Error** | Malformed YAML reaches the caller as `ConfigError` only when the parse path opts into it; missing files and empty vectors are returned as `None`/empty, never as hard errors. Traversal and injection attempts are rejected outright. |

#### Test
| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| Project root carries `lint_arwaky.config.yaml` | The feature reads the config | The root file is returned as the first match in the chain |
| A parent directory (within the depth bound) carries `lint_arwaky.config.yaml` | The feature reads the config | The parent file is returned as the first match in the chain |
| No config file exists anywhere in the chain | The feature reads the config | The result is `None`, not an error |
| A raw YAML string without an `adapters:` key | The content reader parses it | An empty `Vec<AdapterEntry>` is returned |
| A config file is a symlink that resolves outside the project root | The feature reads the config | The file is refused, and an error is returned |

#### Metric
| Target Metric | Value |
|---------------|-------|
| Discovery success rate (config found on first correct priority level) | 100% |
| Default fallback when no config exists (empty result, not error) | 100% |
| Symlink escape rejections | 100% |

### FR-ConfigSystem-002: Detect Multi-Workspace Members
**Description:** The config-system feature shall detect the type of workspace at a given root, probe whether a candidate path is a workspace, and enumerate the immediate member directories under the layout roots so that each member can be scanned independently.

#### Business Logic

- Members are the immediate directories under each layout root; the feature probes marker files first and folder structure second.
- An empty but valid workspace root returns zero members rather than an error.
- Unreadable roots behave like empty roots; no I/O failure is surfaced to the caller.

#### Edge Cases

- An empty but valid workspace root returns an empty member list rather than an error.
- One root holding several layout folders contributes all of them to the member enumeration.
- A marker file alone without layout folders is detected as a typed project but reports non-workspace until layout roots exist.

#### Technical Specification
| Aspect | Detail |
|--------|--------|
| **Input** | `root: &FilePath` for member enumeration; any path for `detect` / `is_workspace`. Source: dispatcher / caller. |
| **Output** | `Vec<FilePath>` of member directories for enumeration; `WorkspaceType` for detection; a boolean for the workspace probe. Destination: dispatcher / caller. |
| **Protocol** | `IWorkspaceMembersProtocol` |
| **Method** | `discover_workspace_members` |
| **Event** | `WorkspaceMembersDiscovered` |
| **Error** | No I/O failure is surfaced: an unreadable root behaves like an empty one; enumeration returns an empty vector rather than an error. |

#### Test
| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| A Rust workspace with `Cargo.toml` and `crates/` | The feature enumerates members | All `crates/<member>` directories are returned |
| An empty but valid workspace root | The feature enumerates members | An empty `Vec<FilePath>` is returned, not an error |
| A plain directory with no marker files | The feature probes workspace type | `NonWorkspace` is returned |
| One root holding both `crates/` and `packages/` | The feature enumerates members | Members from both layout roots are returned |

#### Metric
| Target Metric | Value |
|---------------|-------|
| Member enumeration accuracy (zero false roots, zero misses) | 100% |
| Empty workspace returns empty list (never an error) | 100% |

### FR-ConfigSystem-003: Merge Parsed Config with Defaults
**Description:** The config-system feature shall parse a YAML or TOML configuration file, overlay the parsed user rules onto the embedded defaults with rule-based layer merging, and return the merged result plus any warnings collected during parsing.

#### Business Logic

- Merging overlays user rules onto embedded defaults so a user rule wins on conflict and untouched layers keep their defaults.
- TOML reads only the `tool.lint-arwaky` section; a missing section returns `None` rather than an error.
- Warnings never downgrade a successful merge.
- Underscore key aliases in TOML are accepted.
- Score thresholds outside 0–100 fail later validation at the orchestrator.

#### Edge Cases

- A TOML file without the tool section returns `None`.
- Score thresholds outside the valid range are accepted by the merge and rejected by the downstream validator.

#### Technical Specification
| Aspect | Detail |
|--------|--------|
| **Input** | `path: &FilePath` pointing at a YAML or TOML config, or a raw YAML `yaml_str: &str` for the warning-carrying parse. Source: dispatcher / caller. |
| **Output** | `Result<ProjectConfig, ConfigError>` for file parses; `(ArchitectureConfig, Vec<String>)` for the warning parse; merged config plus warnings from the merge. Destination: dispatcher / caller. |
| **Protocol** | `IConfigMergeProtocol` |
| **Method** | `parse_yaml_config` |
| **Event** | `ConfigMerged` |
| **Error** | File-not-found and invalid YAML/TOML return an error; a TOML without the tool section returns `None`. Warnings never downgrade a successful merge. |

#### Test
| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| A valid YAML config exists | The feature parses and merges it | `ProjectConfig` with user rules overlaid on defaults is returned |
| No config file exists | The feature loads the config | Embedded defaults are returned without error |
| An empty YAML file exists | The feature parses it | Defaults are kept |
| A TOML file without the `tool.lint-arwaky` section | The feature parses the TOML | `None` is returned, not an error |
| A YAML file that does not exist | The feature parses it | A file-not-found error is returned |

#### Metric
| Target Metric | Value |
|---------------|-------|
| Default fallback when no config exists | 100% |
| Unexplained warnings in a successful merge | 0 |

## Public Interface
| Aspect     | Detail |
|------------|--------|
| Agent      | `ConfigOrchestrator` |
| Aggregate  | `IConfigOrchestratorAggregate` |
| Method     | `execute` |
| Request    | `ConfigRequest` — variants `LoadProjectConfig`, `LoadForLanguage`, `ReadConfig`, `DiscoverWorkspaces`, `LoadSync`, `IgnoredPaths`, `IgnoredPathsForLanguage` |
| Response   | `ConfigResponse` — matching variants carrying `ConfigResult`, `Option<ConfigSource>`, `Vec<WorkspaceInfo>`, `ArchitectureConfig`, or `PatternList` |

## Reference
| Artifact     | Purpose                                         | Link |
|--------------|-------------------------------------------------|------|
| PRD          | Business goals, user stories, and scope         | [PRD.md](../../PRD.md) |
| Backlog      | Task tracking, epics, and implementation status | [BACKLOG.md](./BACKLOG.md) |
| Roadmap      | Timeline, phasing, and dependencies             | [ROADMAP.md](../../ROADMAP.md) |
| Architecture | System boundaries, tech stack, and infra        | [ARCHITECTURE.md](../../ARCHITECTURE.md) |
