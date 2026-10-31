# FRD — Project Setup

## Feature Overview

Project setup delivers per-client MCP configuration, per-language `.env` files, language detection, and linter-adapter installation through one `execute` seam on the setup aggregate. Surface entry points route `SetupRequest` variants to the matching capability; capabilities write MCP JSON, `.env` content, config files, and invoke pip/npm installs; the aggregate returns one `SetupResponse` per request.

## Functional Requirements

### FR-ProjectSetup-001: Generate MCP Configuration

**Description:** The feature shall generate a per-client MCP client configuration file for any supported target client (Claude, Cursor, Windsurf, Copilot, Hermes, VSCode, or All), wrapping the server entries under the correct key (`mcp_servers` vs `mcpServers`) per client convention, and returning the rendered config content so the caller can write it to disk. Each target variant produces one config file; `All` generates every target in a single call.

#### Business Logic

- One request variant per supported target client generates exactly one config file for that client.
- Generated content parses as valid JSON/TOML for the target and wraps server entries under the client's convention key (`mcp_servers` vs `mcpServers`).
- `McpConfigAll` generates every target config in one call, wrapping each target under its own key.
- Unknown targets are rejected rather than silently falling back to a default.
- Generated server lists omit entries whose command list is empty.

#### Edge Cases

- `McpConfigAll` produces seven distinct target configs (one per client) in one response.
- Header/comment lines are preserved where the target format permits comments.
- An unknown target returns an error instead of writing a wrong-shaped file.

#### Technical Specification

| Aspect | Detail |
|--------|--------|
| **Input** | Target client identifier; source is `SetupRequest::{GenerateMcpConfig, McpConfigClaude, McpConfigCursor, McpConfigWindsurf, McpConfigCopilot, McpConfigHermes, McpConfigVscode, McpConfigAll}`. |
| **Output** | `SetupResponse::Mcp` carrying per-target config content ready to write. |
| **Protocol** | `IMcpConfigGenerationProtocol` |
| **Method** | `generate_mcp_config`, `mcp_config_claude`, `mcp_config_cursor`, `mcp_config_windsurf`, `mcp_config_copilot`, `mcp_config_hermes`, `mcp_config_vscode`, `mcp_config_all` |
| **Event** | None. |
| **Error** | Unknown target returns an error description rather than writing a wrong-shaped file. |

#### Test

| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| A supported target client is requested | The caller invokes `generate_mcp_config` for that target | One config file with servers wrapped under the client's key is returned |
| `McpConfigAll` is requested | The caller invokes `mcp_config_all` | Seven target configs are returned, one per client, each under its key |
| An unknown target is requested | The caller invokes a variant for an unsupported client | An error description is returned instead of a config file |

#### Metric

| Target Metric | Value |
|---------------|-------|
| Target configs per `McpConfigAll` call | 7 |
| Unknown-target acceptance rate | 0 |

### FR-ProjectSetup-002: Generate Environment File

**Description:** The feature shall generate a project `.env` file containing a documented header and a phantom-root (HOME-style) entry for hermetic runs, producing deterministic output that round-trips through write without clobbering an existing env file silently.

#### Business Logic

- Generated env content contains a header comment explaining the file.
- Generated env content contains a phantom-root entry (HOME-style variable) for hermetic runs.
- Env generation is deterministic: identical project context yields identical bytes on every run.
- Existing env content is not clobbered silently; overwrite is governed by `WriteConfigFile`.

#### Edge Cases

- Empty project path still produces a well-formed env body with header and phantom-root entry.
- Repeated generation of the same project yields byte-identical content.

#### Technical Specification

| Aspect | Detail |
|--------|--------|
| **Input** | Project path/context; source is `SetupRequest::GenerateEnv`. |
| **Output** | `SetupResponse::Env` with `.env`-style file content. |
| **Protocol** | `IEnvGenerationProtocol` |
| **Method** | `generate_env` |
| **Event** | None. |
| **Error** | Generation itself never fails; write failures fall to `FR-ProjectSetup-006`. |

#### Test

| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| A project context is provided | The caller invokes `generate_env` | Returned content contains a header comment and a phantom-root entry |
| The same project is generated twice | The caller invokes `generate_env` twice with identical input | Output bytes are identical on both runs |
| An empty project path is given | The caller invokes `generate_env` with an empty path | A well-formed env body (header + phantom root) is produced |

#### Metric

| Target Metric | Value |
|---------------|-------|
| Byte-identical regeneration rate | 100% |
| Header-and-root presence | Every generated env |

### FR-ProjectSetup-003: Detect Languages

**Description:** The feature shall detect the project's primary language (`detect_language`, returning `Language` or `None`) and all present language groups (`detect_languages`, returning every language group present), scanning the project layout (manifests, source extensions). Unknown or empty projects return none rather than an error; detection results feed template selection.

#### Business Logic

- `detect_language` returns the primary language or `None` when no language is detected.
- `detect_languages` returns every language group present in the project.
- Detection scans project layout (manifests, source extensions) and result feeds template selection.
- Unknown/empty project returns `None`/empty rather than an error.

#### Edge Cases

- Unreadable project tree yields empty results rather than panicking.
- Multiple languages present: `detect_languages` returns all groups; `detect_language` returns one deterministic primary.

#### Technical Specification

| Aspect | Detail |
|--------|--------|
| **Input** | Project root path; source is `SetupRequest::{DetectLanguage, DetectLanguages}`. |
| **Output** | `SetupResponse::{Language, Languages}`. |
| **Protocol** | `ILanguageDetectionProtocol` |
| **Method** | `detect_language`, `detect_languages` |
| **Event** | None. |
| **Error** | No signal yields `None`/empty rather than an error. |

#### Test

| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| A seeded fixture tree of one language is scanned | The caller invokes `detect_language` | That language is reported |
| A mixed-language fixture tree is scanned | The caller invokes `detect_languages` | All present language groups are reported |
| An unknown/empty project is scanned | The caller invokes `detect_language` | `None` or empty result is returned, not an error |

#### Metric

| Target Metric | Value |
|---------------|-------|
| Seeded single-language fixtures correctly identified | 100% |
| Mixed-fixture group coverage | All present groups |

### FR-ProjectSetup-004: Install Adapters

**Description:** The feature shall install linter adapters for detected languages via npm (JavaScript/TypeScript) or pip (Python). Installing an empty package list returns `Ok` without spawning a package manager; failed package-manager exits are reported as failures rather than panics.

#### Business Logic

- `InstallPythonAdapters` runs pip install of the Python adapter set.
- `InstallJavascriptAdapters` runs npm install of the JavaScript adapter set.
- Empty package list returns `Ok` without invoking a package manager.
- Installers are idempotent: already-installed packages reinstall cleanly.
- Sudo variants prepend privilege escalation; otherwise behave like plain variants.

#### Edge Cases

- Empty list returns `Ok` with no subprocess spawned.
- Non-zero package-manager exit returns failure status with captured output; caller sees failure, not panic.
- Already-installed packages reinstall cleanly (idempotent).
- Sudo variants behave like plain variants otherwise.

#### Technical Specification

| Aspect | Detail |
|--------|--------|
| **Input** | Package list plus sudo flag; source is `SetupRequest::{InstallPythonAdapters, InstallJavascriptAdapters}`. |
| **Output** | `SetupResponse::Installed` with per-run status. |
| **Protocol** | `IAdapterInstallationProtocol` |
| **Method** | `install_python_packages`, `install_npm_packages` |
| **Event** | None. |
| **Error** | Non-zero package-manager exit returns failure status with captured output. |

#### Test

| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| Empty package list is given to pip | The caller invokes `install_python_packages` with an empty list | `Ok` is returned and no pip subprocess is spawned |
| Empty package list is given to npm with sudo | The caller invokes `install_npm_packages` with sudo and an empty list | `Ok` is returned and no npm subprocess is spawned |
| Package manager exits non-zero | The caller invokes an adapter install with a failing package | Failure status with captured output is returned, not a panic |

#### Metric

| Target Metric | Value |
|---------------|-------|
| Empty-list invocations spawning a package manager | 0 |
| Status truthfulness vs process exit | 100% |

### FR-ProjectSetup-005: Load Config Templates

**Description:** The feature shall load the embedded config template for a requested key, returning byte-stable template content. Unknown keys return a not-found description rather than empty filler; empty keys return not-found; template existence does not depend on project emptiness (write policy decides merge).

#### Business Logic

- `GetConfigTemplate` returns the embedded template for the requested key.
- Template content is byte-stable across runs.
- Unknown template key returns a not-found description instead of empty filler.
- Empty key returns not-found.

#### Edge Cases

- Unknown key yields a not-found-style description instead of empty filler.
- Empty key returns not-found.
- Template exists but project is non-empty still returns template (write policy decides merge).

#### Technical Specification

| Aspect | Detail |
|--------|--------|
| **Input** | Template key; source is `SetupRequest::{GetConfigTemplate, GetEmbeddedSkills}`. |
| **Output** | `SetupResponse::{Template, Skills}`. |
| **Protocol** | `IConfigTemplateProtocol` |
| **Method** | `get_config_template` |
| **Event** | None. |
| **Error** | Unknown key yields a not-found-style description instead of empty filler. |

#### Test

| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| A known template key is requested | The caller invokes `get_config_template` | Non-empty byte-stable template content is returned |
| An unknown template key is requested | The caller invokes `get_config_template` with a bogus key | A not-found description is returned, not empty filler |
| An empty key is requested | The caller invokes `get_config_template` with an empty key | A not-found result is returned |

#### Metric

| Target Metric | Value |
|---------------|-------|
| Known-key non-empty byte-stable rate | 100% |
| Unknown-key not-found rate | 100% |

### FR-ProjectSetup-006: Write Config Files

**Description:** The feature shall write configuration files to disk from generated or templated content, creating parent directories as needed. Writes avoid partial files on normal error paths; existing files are overwritten only when the request implies it. `CreateGlobalConfigDir` ensures the user-level config directory exists.

#### Business Logic

- `WriteConfigFile` writes content to the target path, creating parent dirs as needed.
- Writes are atomic enough to avoid partial files on normal error paths.
- Existing files are overwritten only when the request implies it.
- `CreateGlobalConfigDir` ensures the user-level config directory exists; on an existing dir it is a no-op success.

#### Edge Cases

- Missing parent directories are created on write.
- Read-only targets fail with an error description rather than leaving a partial file.
- Same content written twice leaves the file byte-identical.
- `CreateGlobalConfigDir` on an existing dir is a no-op success.

#### Technical Specification

| Aspect | Detail |
|--------|--------|
| **Input** | Target path plus content; source is `SetupRequest::{WriteConfigFile, CreateGlobalConfigDir}`. |
| **Output** | `SetupResponse::{ConfigWritten, ConfigDir}`. |
| **Protocol** | `IConfigWritingProtocol` |
| **Method** | `write_config_file`, `create_global_config_dir` |
| **Event** | None. |
| **Error** | Permission or path errors return a failure description; no partial file left behind. |

#### Test

| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| Content is written to a path whose parent dirs do not exist | The caller invokes `write_config_file` | Parent dirs are created and the file is written |
| The same content is written twice | The caller invokes `write_config_file` twice with identical content | The file is byte-identical on both runs |
| A read-only target is written | The caller invokes `write_config_file` on a read-only path | An error description is returned, no partial file left behind |

#### Metric

| Target Metric | Value |
|---------------|-------|
| Written files parsing as intended format | 100% |
| E2e round-trip byte match | Every seeded round trip |

### FR-ProjectSetup-007: Run Pre-flight Check

**Description:** The feature shall run pre-project-setup checks before any generation or installation proceeds. `PreFlightCheck` verifies prerequisites (working directory usable, required tools discovered, no blocking conflicts) and returns a pass/fail report with reasons; generation flows consult it first.

#### Business Logic

- `PreFlightCheck` verifies prerequisites and returns a pass/fail report with reasons.
- Missing prerequisite reports the failing item without performing generation.
- Clean environment passes.
- Generation flows consult the pre-flight report before proceeding.
- `CheckHttp` probes a URL and reports reachability.

#### Edge Cases

- Partial failures list each failed item rather than stopping at the first.
- Failing check returns a failure report (not an exception); caller decides whether to proceed.

#### Technical Specification

| Aspect | Detail |
|--------|--------|
| **Input** | Project path and optional HTTP target; source is `SetupRequest::{PreFlightCheck, CheckHttp}`. |
| **Output** | `SetupResponse::{PreFlight, CheckHttp}` with per-item status. |
| **Protocol** | `IPreFlightProtocol` |
| **Method** | `pre_flight_check`, `check_http` |
| **Event** | None. |
| **Error** | Failing check returns a failure report (not an exception); caller decides whether to proceed. |

#### Test

| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| A clean environment is checked | The caller invokes `pre_flight_check` | All-green pass report |
| A missing-tool environment is checked | The caller invokes `pre_flight_check` | Failing report naming the missing item |
| Partial failures occur | The caller invokes `pre_flight_check` | Each failed item is listed |

#### Metric

| Target Metric | Value |
|---------------|-------|
| Passing environment all-green rate | 100% |
| Seeded missing-tool failing reports | Every seeded environment |

### FR-ProjectSetup-008: Probe File Existence

**Description:** The feature shall probe whether a config file already exists before writing. `FileExists` returns a boolean existence check for the given path; callers use it to avoid clobbering user files. Non-existent path returns `Exists=false` without error; existing file returns `Exists=true`; unreadable parent returns false rather than panicking.

#### Business Logic

- `FileExists` returns `Exists=true` when the path points to an existing file.
- `FileExists` returns `Exists=false` when the path does not exist.
- Callers use the result to avoid clobbering user files.

#### Edge Cases

- Non-existent path returns `Exists=false` without error.
- Existing file returns `Exists=true`.
- Path inside an unreadable parent returns false rather than panicking.
- Round trip after create returns true.

#### Technical Specification

| Aspect | Detail |
|--------|--------|
| **Input** | File path; source is `SetupRequest::FileExists`. |
| **Output** | `SetupResponse::Exists`. |
| **Protocol** | `IFilePathExistenceProtocol` |
| **Method** | `file_exists` |
| **Event** | None. |
| **Error** | Unreadable parents resolve to false, not an error. |

#### Test

| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| A non-existent path is probed | The caller invokes `file_exists` | `Exists=false` without error |
| An existing file is probed | The caller invokes `file_exists` | `Exists=true` |
| A file is created then probed | The caller creates then invokes `file_exists` | `Exists=true` |

#### Metric

| Target Metric | Value |
|---------------|-------|
| Existence probe agreement with on-disk state | Every seeded fixture path |

## Public Interface

| Aspect | Detail |
|--------|--------|
| Agent | SetupManagementOrchestrator |
| Aggregate | `ISetupAggregate` |
| Method | `execute` |
| Request | `SetupRequest` |
| Response | `SetupResponse` |

## Reference

| Artifact | Purpose | Link |
|----------|---------|------|
| PRD | Business goals, user stories, and scope | [PRD.md](../../PRD.md) |
| Backlog | Task tracking, epics, and implementation status | [BACKLOG.md](./BACKLOG.md) |
| Roadmap | Timeline, phasing, and dependencies | [ROADMAP.md](../../ROADMAP.md) |
| Architecture | System boundaries, tech stack, and infra | [ARCHITECTURE.md](../../ARCHITECTURE.md) |
