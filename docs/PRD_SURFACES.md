# PRD — Surfaces Layer
> **Vision**: The edge — thin, passive interfaces for humans and AI agents

## Layer Identity

**Layer**: Surfaces (External Interfaces / Entry Points)
**Path**: `src-rust/surfaces/`
**Role**: CLI command definitions (clap), MCP server handlers (JSON-RPC 2.0), output controllers
**Dependency Rule**: Can import from `agent`, `taxonomy`, and `contract(aggregate)` only. Three surface categories with different rules:
- **Smart Surfaces** (`_page`, `_command`, `_handler`, `_controller`, `_entry`): import agent + taxonomy + contract(aggregate)
- **Utility Surfaces** (`_hook`, `_store`, `_provider`, `_router`): import agent + taxonomy + contract(aggregate) + passive surfaces
- **Passive Surfaces** (`_component`, `_layout`, `_view`): import taxonomy ONLY
ZERO imports to `capabilities` or `infrastructure` for ALL categories.

## 1. Strategic Goal

Surfaces must become **the thin, passive entry point** for every interaction with the system — both human (CLI) and AI agent (MCP). Every surface file parses input, delegates to Agent via `ServiceContainerAggregate`, and returns formatted output. Zero business logic lives here. Zero infrastructure concerns leak through.

## 2. Component Blueprint

### 2.1 CLI Core Commands

All CLI commands use `clap` 4.6 with derive macros. Every command:
1. Parses args via clap
2. Gets DI container → requests ServiceContainerAggregate
3. Delegates to agent orchestrator
4. Formats and prints output

| Command | Args | Delegates To | Output |
|---------|------|-------------|--------|
| `check` | `[path]`, `--git-diff` | `ArchitectureLintOrchestrator` | Formatted report |
| `scan` | `[path]` | `AnalysisOrchestrator` (deep) | Detailed report |
| `fix` | `[path]` | `LintFixOrchestrator` | Fix summary |
| `report` | `[path]`, `-f/--format` | `ReportCommandsOrchestrator` | Text / JSON / SARIF / JUnit |
| `ci` | `[path]`, `-t/--threshold` | `ArchitectureLintOrchestrator` | Score + exit code |
| `version` | — | Inline | Version string |
| `adapters` | — | DI container | Adapter table |
| `security` | `[path]` | Linter adapters | Security report |
| `cancel` | `<job-id>` | `PipelineJobRegistry` | Confirmation |

#### Command Comparison: `check` vs `scan`

| Aspect | `check` | `scan` |
|--------|---------|--------|
| **Scope** | AES architecture self-lint only | All adapters + all languages |
| **Checkers** | Naming, imports, metrics, orphans, cycles | Plus: Ruff, MyPy, Bandit, ESLint, Clippy |
| **Speed** | Fast (Rust AST only) | Slow (runs all external tools) |
| **Output** | AES compliance report | Full multi-language report |
| **Use case** | Daily dev, pre-commit, CI gate | Deep audit, release readiness |
| **Config needed** | YAML or default AES config | Requires external tools installed |

### 2.2 CLI Analysis Commands

| Command | Args | Delegates To |
|---------|------|-------------|
| `complexity` | `[path]` | `AnalysisOrchestrator` |
| `duplicates` | `[path]` | `AnalysisOrchestrator` |
| `trends` | `[path]` | `AnalysisOrchestrator` |
| `dependencies` | `[path]` | `AnalysisOrchestrator` |

### 2.3 CLI Setup Commands

| Command | Args | Delegates To |
|---------|------|-------------|
| `setup init` | — | `SetupManagementOrchestrator` |
| `setup doctor` | — | `SetupManagementOrchestrator` |
| `setup mcp-config` | `<client>` | `SetupManagementOrchestrator` |
| `setup hermes` | `--remove` | `SetupManagementOrchestrator` |

### 2.4 CLI Dev Commands

| Command | Args | Delegates To |
|---------|------|-------------|
| `diff` | `<path1> <path2>` | `DevCommandsOrchestrator` |
| `suggest` | `[path]`, `--ai` | `DevCommandsOrchestrator` |
| `import` | `<config-file>` | `DevCommandsOrchestrator` |
| `export` | `-f/--format` | `DevCommandsOrchestrator` |
| `config` | — | `DevCommandsOrchestrator` |

### 2.5 CLI Git & Multi Commands

| Command | Args | Delegates To |
|---------|------|-------------|
| `git-diff` | `--base <ref>` | `GitCommandsOrchestrator` |
| `multi-project` | `<paths...>` | `MultiProjectOrchestrator` |
| `plugin` | `<action> [name]` | `PluginCommandsOrchestrator` |
| `install-hook` | — | `HookManagementOrchestrator` |
| `uninstall-hook` | — | `HookManagementOrchestrator` |

### 2.6 CLI Maintenance Commands

| Command | Args | Delegates To |
|---------|------|-------------|
| `watch` | `[path]` | `WatchCommandsOrchestrator` |

### 2.7 MCP Server

**Protocol**: JSON-RPC 2.0 over stdin/stdout

**Lifecycle**:
```
Client → initialize          → Server returns protocolVersion + capabilities
Client → tools/list          → Server returns 5 tool definitions with JSON Schema
Client → tools/call (tool)   → Server dispatches to handler → returns result
Client → notifications/...   → Server handles (e.g., cancellation)
```

**Required Tools**:

| Tool Name | Input Schema | Behavior |
|-----------|-------------|----------|
| `execute_command` | `{ action: string, args?: object }` | Execute any CLI command via DI container |
| `list_commands` | `{ domain?: string }` | Return registered command catalog |
| `commands_schema` | `{ tool_name?: string }` | Return JSON Schema for tool(s) |
| `read_skill_context` | `{ section?: string }` | Return SKILL.md content |
| `health_check` | `{}` | Return adapter + system health |

**Server Initialization Response**:
```json
{
    "protocolVersion": "2024-11-05",
    "capabilities": { "tools": { "listChanged": false } },
    "serverInfo": { "name": "lint-arwaky", "version": "1.10.2" }
}
```

### 2.8 MCP Handler Files

| File | Responsibility |
|------|---------------|
| `mcp_main_entry.rs` | Tokio async main — reads stdin JSON-RPC, dispatches to handler |
| `mcp_server_handler.rs` | Server schema, JSON-RPC method routing, lifespan |
| `mcp_tools_command.rs` | Tool implementations (execute_command, list_commands, etc.) |
| `mcp_tools_handler.rs` | Tool registration + schema building |
| `mcp_command_handler.rs` | Command catalog building, command listing, skill context reading |
| `mcp_execute_command.rs` | CLI command execution via ServiceContainerAggregate |
| `mcp_client_handler.rs` | Desktop MCP client config generation |
| `mcp_health_handler.rs` | System health check |
| `mcp_job_handler.rs` | Job management (status, cancel) |

### 2.9 Surface Controllers

| Controller | Responsibility |
|------------|---------------|
| `cli_output_controller.rs` | Output routing: tee stdout, write file, format selection |
| `cli_setup_controller.rs` | Setup flow: generate .env, MCP configs for Claude/VSCode/Hermes |
| `cli_main_handler.rs` | CLI entry point orchestration |
| `syspath_bootstrap_handler.rs` | System PATH bootstrap for IDE integration |

## 3. Surface Design Rules

### 3.1 Three Surface Categories

The YAML config defines THREE surface categories with different dependency rules:

| Category | Suffixes | Dependency Rule |
|----------|----------|-----------------|
| **Smart Surfaces** (entry points) | `_page`, `_command`, `_handler`, `_controller`, `_entry` | Import: agent, taxonomy, contract(aggregate), sibling surfaces. Forbidden: infrastructure, capabilities, contract(port), contract(protocol) |
| **Utility Surfaces** (helpers) | `_hook`, `_store`, `_provider`, `_router` | Import: agent, taxonomy, contract(aggregate), passive surfaces. Forbidden: infrastructure, capabilities, contract(port), contract(protocol), smart surfaces |
| **Passive Surfaces** (dumb views) | `_component`, `_layout`, `_view` | Import: taxonomy ONLY. Forbidden: agent, contract, infrastructure, capabilities, all other surfaces |

### 3.2 Allowed Suffixes

YAML config defines these surface suffixes: `_page`, `_command`, `_handler`, `_controller`, `_router`, `_component`, `_layout`, `_view`, `_entry`, `_hook`, `_store`, `_provider`

### 3.3 Passive Surface (AES019 / AES022)

```
✅ ACCEPTABLE in surfaces:
  - Parse CLI args (clap)
  - Call container.get_architecture_linter()
  - Call linter.run_self_lint(path)
  - println!("{}", report)

❌ FORBIDDEN in surfaces:
  - Importing ArchComplianceAnalyzer directly
  - Importing RuffAdapter directly
  - Implementing business logic
  - Performing file I/O directly (use port)
```

### 3.2 Dependency Rules (AES023)

```
✅ ALLOWED imports:     agent (for DI container), taxonomy, contract(aggregate), sibling surfaces
❌ FORBIDDEN imports:   capabilities, infrastructure, contract(port), contract(protocol)
```

### 3.3 MCP Tool Schema (AES025)

Every MCP tool must declare:
- Docstring → becomes `description` in `tools/list`
- Typed parameters → each field has `type` + `description`
- Valid JSON Schema → `inputSchema` with `type: "object"` + `properties`

## 4. Architectural Rules

| Rule | Constraint |
|------|------------|
| AES019 | Zero domain logic — only parse, delegate, return |
| AES022 | Surfaces MUST be passive I/O boundaries |
| AES023 | Zero imports to capabilities or infrastructure |
| AES025 | MCP tools MUST have docstrings + typed params + valid JSON Schema |
| AES007 | All contract imports via barrel |

## 5. Non-Functional Targets

| Metric | Target |
|--------|--------|
| CLI startup time | < 500ms |
| MCP server startup | < 2s |
| CLI commands | 20+ subcommands across all categories |
| MCP tools | 5 tools with valid JSON Schema |
| Output formats | 4: text, JSON, SARIF 2.1.0, JUnit XML |
| Exit codes | 0 = pass, 1 = violations found |
| Cap/infra imports | ZERO |

## 6. Success Criteria

A Surfaces layer is **complete** when:
- ALL 20+ CLI commands parse, execute, and produce correct output
- `check` runs full AES self-lint and returns formatted report
- `fix` applies safe auto-fixes (or reports what would be fixed)
- `report` outputs correct text, JSON, SARIF, and JUnit formats
- `setup doctor` detects all relevant environment tools
- `setup init` creates working default config
- MCP server correctly handles: initialize → tools/list → tools/call lifecycle
- MCP server returns valid JSON Schema for all 5 tools
- `watch` polls and re-lints continuously on interval
- `git-diff` lints only changed files since base ref
- `multi-project` lints multiple paths correctly
- `install-hook` + `uninstall-hook` work correctly
- Zero imports to capabilities or infrastructure modules
- Zero business logic in surface files
- Consistent DI container usage across CLI and MCP
