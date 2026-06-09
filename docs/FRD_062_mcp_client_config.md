# 📄 Feature Requirements Document (FRD)
**Feature Name:** MCP Client Configuration (`setup mcp-config`)  
**Product:** Lint Arwaky v1.10.2  
**Author:** Raka  
**Date:** 09/06/2026  
**Version:** v1.0

## 1. Document Control
| Version | Date | Author | Description of Changes | Approved By |
|---------|------|--------|----------------------|-------------|
| v1.0 | 09/06/2026 | Raka | Initial document creation | [Stakeholder] |

## 2. Introduction
### 2.1 Purpose
This document defines the MCP client configuration CLI command `setup mcp-config --client <name>`. It generates MCP (Model Context Protocol) client configuration for AI agents (Claude Code, Cursor, Windsurf, GitHub Copilot, etc.) so they can connect to the Lint Arwaky MCP server for autonomous code auditing.

### 2.2 Scope
**In-Scope:**
- `lint-arwaky-cli setup mcp-config --client claude-code` — generate Claude Code config
- `lint-arwaky-cli setup mcp-config --client cursor` — generate Cursor config
- `lint-arwaky-cli setup mcp-config --client windsurf` — generate Windsurf config
- `lint-arwaky-cli setup mcp-config --client copilot` — generate GitHub Copilot config
- Config output to stdout or file with `--output` flag
- MCP server command, args, and environment variable setup

**Out-of-Scope:**
- Installing or starting the MCP server
- Hermes integration (handled by FR-063)
- Environment diagnostics (handled by FR-060)

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **MCP** | Model Context Protocol — standard for AI agent tool integration |
| **MCP server** | Lint Arwaky's JSON-RPC 2.0 server over stdin/stdout |
| **Claude Code** | Anthropic's CLI agent (claude-code) |
| **Cursor** | AI-native IDE by Anysphere |
| **Windsurf** | AI-native IDE by Codeium |
| **GitHub Copilot** | AI pair programmer by GitHub |

## 3. Feature Overview
### 3.1 Background & Problem
Lint Arwaky provides an MCP server (`lint-arwaky-mcp`) that AI agents can use for autonomous code auditing. However, each AI agent has a different configuration format for MCP server registration. Developers had to manually write the correct JSON/YAML config for their agent, leading to setup friction.

### 3.2 Business Goals
- Support all major MCP-compatible AI agents
- Generate correct configuration for each client format
- Reduce MCP setup time to one command
- Output config to stdout or directly to agent's config file location

### 3.3 Target Users
- **AI Agent Users**: Developers who use Claude Code, Cursor, Windsurf, or Copilot
- **AI Agents**: Autonomous setup during initial configuration

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a Claude Code user, I want to run `setup mcp-config --client claude-code` to add Lint Arwaky as an MCP tool.
- **US-002:** As a Cursor user, I want `setup mcp-config --client cursor` to generate the correct JSON for my `.cursor/mcp.json`.
- **US-003:** As a Windsurf user, I want the generated config to include the correct path to the `lint-arwaky-mcp` binary.

### 4.2 Use Cases & Workflow
**MCP Config Generation:**
```
lint-arwaky-cli setup mcp-config --client claude-code
  │
  ├─► 1. Resolve binary path
  │     └── $CARGO_HOME/bin/lint-arwaky-mcp or which lint-arwaky-mcp
  │
  ├─► 2. Generate config for client type
  │     ├── claude-code → ~/.claude/settings.json
  │     ├── cursor → .cursor/mcp.json
  │     ├── windsurf → .windsurf/mcp_config.json
  │     └── copilot → .github/copilot/mcp.json
  │
  └─► 3. Output to stdout or write to file with --output
```

**Generated Config Examples:**

Claude Code (`~/.claude/settings.json`):
```json
{
  "mcpServers": {
    "lint-arwaky": {
      "command": "/home/user/.cargo/bin/lint-arwaky-mcp",
      "args": [],
      "env": {}
    }
  }
}
```

Cursor (`.cursor/mcp.json`):
```json
{
  "mcpServers": {
    "lint-arwaky": {
      "command": "/home/user/.cargo/bin/lint-arwaky-mcp",
      "args": [],
      "env": {}
    }
  }
}
```

Windsurf (`.windsurf/mcp_config.json`):
```json
{
  "config:lint-arwaky": {
    "command": "/home/user/.cargo/bin/lint-arwaky-mcp",
    "args": [],
    "env": {}
  }
}
```

GitHub Copilot (`.github/copilot/mcp.json`):
```json
{
  "inputs": [],
  "server": {
    "command": "/home/user/.cargo/bin/lint-arwaky-mcp",
    "args": [],
    "env": {}
  }
}
```

### 4.3 Business Rules
- Binary path resolved automatically from `which lint-arwaky-mcp`
- `--binary <path>` flag to override binary path
- `--output <file>` flag to write directly to target config file
- Without `--output`, config is printed to stdout with instructions
- Existing config is NOT overwritten unless `--force` flag is provided

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Config generation | < 50ms |
| NFR-002 | Client-specific format accuracy | 100% correct per agent schema |
| NFR-003 | Binary path resolution | Correct for all common install locations |

## 6. UI/UX Requirements
CLI output:
```
$ lint-arwaky-cli setup mcp-config --client claude-code
🔌 MCP Client Configuration
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Client: Claude Code
Binary: /home/user/.cargo/bin/lint-arwaky-mcp

✅ Configuration generated:

{
  "mcpServers": {
    "lint-arwaky": {
      "command": "/home/user/.cargo/bin/lint-arwaky-mcp",
      "args": [],
      "env": {}
    }
  }
}

To install, add this to ~/.claude/settings.json:
  $ lint-arwaky-cli setup mcp-config --client claude-code --output ~/.claude/settings.json
```

With `--output`:
```
$ lint-arwaky-cli setup mcp-config --client cursor --output .cursor/mcp.json
✅ Config written to .cursor/mcp.json
Restart Cursor for changes to take effect.
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Claude Code client requested | `setup mcp-config --client claude-code` | Valid Claude Code MCP JSON output | Pending Review |
| AC-002 | Cursor client requested | `setup mcp-config --client cursor` | Valid Cursor MCP JSON | Pending Review |
| AC-003 | Windsurf client requested | `setup mcp-config --client windsurf` | Valid Windsurf MCP JSON | Pending Review |
| AC-004 | GitHub Copilot client requested | `setup mcp-config --client copilot` | Valid Copilot MCP JSON | Pending Review |
| AC-005 | `--output` flag provided | `setup mcp-config --client claude-code --output file.json` | Config written to file | Pending Review |
| AC-006 | Binary not found on PATH | `setup mcp-config --client claude-code` | Warning with `--binary` suggestion | Pending Review |

## 8. Empirical Findings (Code Audit)

### 8.1 Current Implementation

| Component | Location | Lines | Status |
|-----------|----------|-------|--------|
| CLI mcp-config command | `project-setup/surface_mcpconfig_command.rs` | — | **FULLY IMPLEMENTED** |
| Config generator | `project-setup/capabilities_mcp_config_generator.rs` | — | **FULLY IMPLEMENTED** — client-specific formatters |
| Binary resolver | `project-setup/capabilities_binary_resolver.rs` | — | **FULLY IMPLEMENTED** — PATH search + which |
| Config writer | `project-setup/infrastructure_mcp_writer.rs` | — | **FULLY IMPLEMENTED** |

### 8.2 Bugs Found

1. **Copilot config format differs from MCP spec** — GitHub Copilot uses a different JSON structure (`inputs` + `server`) compared to other MCP clients
   - **Impact**: Generated Copilot config may not be compatible with all Copilot versions
   - **Fix**: Verify Copilot MCP schema against latest GitHub documentation

2. **Binary resolver only checks PATH** — doesn't check `$CARGO_HOME/bin` or `~/.local/bin` which are common install locations
   - **Impact**: Binary not found even though it's installed
   - **Fix**: Add fallback search paths: `$CARGO_HOME/bin`, `~/.cargo/bin`, `~/.local/bin`

3. **No validation of output file path** — `--output /nonexistent/dir/config.json` silently fails
   - **Impact**: User thinks config was written but it wasn't
   - **Fix**: Check parent directory existence before writing

### 8.3 What Needs to Be Added

- **Copilot schema verification**: Match latest GitHub Copilot MCP integration format
- **Fallback binary search**: Check common install directories beyond PATH
- **Output path validation**: Verify parent directory exists, create if needed

### 8.4 What to Keep

- **Client-specific formats** ✅ — 4 clients supported with correct schema
- **Binary resolution** ✅ — finds binary from PATH
- **stdout + --output** ✅ — flexible output options
- **--force flag** ✅ — overwrite protection

### 8.5 Empirical Evidence from Test Projects

- Claude Code config generated and verified working with `lint-arwaky-mcp`
- Cursor config generated and verified with Cursor IDE
- Pending Review: Copilot format, fallback binary paths

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-110 (MCP Server) | MCP server binary must exist | Binary not built | Binary resolution with fallback |
| MCP client schemas | AI agent config formats | Schema changes | Client-specific formatters isolate changes |

## 10. Appendices
- `src-rust/project-setup/surface_mcpconfig_command.rs` — CLI mcp-config command
- `src-rust/project-setup/capabilities_mcp_config_generator.rs` — Config generator
- `src-rust/project-setup/capabilities_binary_resolver.rs` — Binary resolver
- `src-rust/mcp-server/` — MCP server implementation
