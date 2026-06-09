# 📄 Feature Requirements Document (FRD)
**Feature Name:** Hermes Integration (`setup hermes [--remove]`)  
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
This document defines the Hermes integration CLI command `setup hermes [--remove]`. Hermes is an AI agent communication protocol that enables Lint Arwaky to send notifications and receive commands from AI agents through a shared message bus. This feature installs/uninstalls the Hermes hook for agent-to-linter communication.

### 2.2 Scope
**In-Scope:**
- `lint-arwaky-cli setup hermes` — install Hermes integration hooks
- `lint-arwaky-cli setup hermes --remove` — uninstall Hermes hooks
- Communication channel registration for lint events
- JSON-RPC message relay between Hermes and MCP server
- Lifecycle management (install, verify, remove)

**Out-of-Scope:**
- Hermes protocol specification (external dependency)
- MCP server configuration (handled by FR-062)
- AI agent implementation

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **Hermes** | AI agent communication protocol/message bus |
| **Hermes hook** | Registered endpoint for Hermes message routing |
| **Message bus** | Shared channel for agent-to-linter communication |
| **Event relay** | Transforms lint events into Hermes messages |

## 3. Feature Overview
### 3.1 Background & Problem
AI agents need a standardized way to communicate with Lint Arwaky beyond direct MCP calls. Hermes provides a message bus where agents can subscribe to lint events (e.g., "check complete", "violation found") and send commands (e.g., "run check on file X"). Without Hermes integration, each agent would need custom polling or webhook logic.

### 3.2 Business Goals
- Enable event-driven agent-to-linter communication
- Support push notifications for lint events
- Provide clean install/remove lifecycle
- Integrate with existing MCP server as the message relay

### 3.3 Target Users
- **AI Agent Developers**: Build agents that react to lint events
- **AI Agents**: Subscribe to lint events via Hermes message bus
- **Developers**: Enable AI agent collaboration on code quality

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As an AI agent, I want to receive notifications when `check` completes, so I can react to new violations.
- **US-002:** As a developer, I want to run `setup hermes` to enable agent communication, so AI tools can interact with the linter.
- **US-003:** As a developer, I want `setup hermes --remove` to cleanly uninstall the integration, so I can disable agent communication when not needed.

### 4.2 Use Cases & Workflow
**Install Pipeline:**
```
lint-arwaky-cli setup hermes
  │
  ├─► 1. Check Hermes availability
  │     ├── hermes binary on PATH?
  │     └── Hermes config in project?
  │
  ├─► 2. Register lint event channels
  │     ├── channel: lint.check.completed
  │     ├── channel: lint.violation.detected
  │     ├── channel: lint.fix.applied
  │     └── channel: lint.scan.completed
  │
  ├─► 3. Install MCP-Hermes relay
  │     └── Bridge between MCP server and Hermes message bus
  │
  └─► 4. Verify installation
        └── "Hermes integration active: 4 channels registered"
```

**Remove Pipeline:**
```
lint-arwaky-cli setup hermes --remove
  │
  ├─► 1. Unregister all lint event channels
  ├─► 2. Remove MCP-Hermes relay bridge
  └─► 3. Verify removal
        └── "Hermes integration removed"
```

**Event Flow:**
```
┌─────────────┐     ┌──────────────┐     ┌────────────┐
│ Lint Arwaky │────►│ MCP Server   │────►│ Hermes Bus │
│ (check done)│     │ (event relay)│     │            │
└─────────────┘     └──────────────┘     └─────┬──────┘
                                               │
                                    ┌──────────▼──────────┐
                                    │ AI Agent (subscriber)│
                                    │ "lint.violation.*"   │
                                    └─────────────────────┘
```

### 4.3 Business Rules
- Hermes binary must be on PATH for installation
- Channels are prefixed with `lint.` namespace
- Each event carries JSON payload with violation/result data
- Removing integration does not affect MCP server functionality
- Installation is idempotent (safe to run multiple times)

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Hermes hook installation | < 1s |
| NFR-002 | Event relay latency (MCP → Hermes) | < 10ms |
| NFR-003 | Zero impact on lint performance when Hermes unavailable | Guaranteed |

## 6. UI/UX Requirements
CLI output:
```
$ lint-arwaky-cli setup hermes
🔗 Installing Hermes Integration
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Checking Hermes availability...
  ✓ hermes 0.3.1 — found at /usr/local/bin/hermes

Registering event channels:
  ✓ lint.check.completed
  ✓ lint.violation.detected
  ✓ lint.fix.applied
  ✓ lint.scan.completed

Installing MCP-Hermes relay...
  ✓ Relay bridge active

✅ Hermes integration installed
  4 channels registered, relay bridge active
```

Remove output:
```
$ lint-arwaky-cli setup hermes --remove
🔗 Removing Hermes Integration

Unregistering event channels:
  ✓ lint.check.completed — removed
  ✓ lint.violation.detected — removed
  ✓ lint.fix.applied — removed
  ✓ lint.scan.completed — removed

Removing MCP-Hermes relay...
  ✓ Relay bridge removed

✅ Hermes integration removed
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Hermes binary on PATH | `setup hermes` runs | Channels registered, relay installed | Pending Review |
| AC-002 | Hermes not installed | `setup hermes` runs | Error: Hermes not found, instructions shown | Pending Review |
| AC-003 | Integration already installed | `setup hermes` runs | Idempotent: no duplicate registration | Pending Review |
| AC-004 | Active integration | `setup hermes --remove` runs | Channels unregistered, relay removed | Pending Review |
| AC-005 | After remove, check completes | `check .` runs | No Hermes-related errors | Pending Review |

## 8. Empirical Findings (Code Audit)

### 8.1 Current Implementation

| Component | Location | Lines | Status |
|-----------|----------|-------|--------|
| CLI hermes command | `project-setup/surface_hermes_command.rs` | — | **FULLY IMPLEMENTED** |
| Hermes installer | `project-setup/capabilities_hermes_installer.rs` | — | **FULLY IMPLEMENTED** — channel registration |
| Hermes remover | `project-setup/capabilities_hermes_remover.rs` | — | **FULLY IMPLEMENTED** — channel cleanup |
| Event relay | `mcp-server/agent_hermes_relay.rs` | — | **FULLY IMPLEMENTED** — MCP → Hermes bridge |

### 8.2 Bugs Found

1. **Channel registration fails silently if Hermes is unreachable** — the installer attempts to register channels but does not verify they were created
   - **Impact**: Installation reports success but channels are not actually registered
   - **Fix**: Verify each channel registration with a confirm request

2. **--remove does not clean up relay state** — the MCP-Hermes relay remains in memory after removal
   - **Impact**: Stale relay may try to send to unregistered channels
   - **Fix**: Send shutdown signal to relay on remove

3. **Hermes binary detection only checks PATH** — similar to FR-062, common install locations are missed
   - **Impact**: Hermes not found when installed by npm/pip
   - **Fix**: Check `node_modules/.bin/hermes` and `pip show hermes` locations

### 8.3 What Needs to Be Added

- **Channel verification**: Confirm registration with Hermes bus
- **Relay shutdown**: Clean relay state on `--remove`
- **Extended binary search**: Check npm/pip install locations

### 8.4 What to Keep

- **Install/remove lifecycle** ✅ — clean, idempotent operations
- **Event channels** ✅ — 4 well-named channels for lint events
- **MCP relay bridge** ✅ — correct event transformation

### 8.5 Empirical Evidence from Test Projects

- `setup hermes` installs and registers channels against Hermes v0.3.1
- Events from `check .` are relayed to Hermes bus
- `setup hermes --remove` cleans up without errors
- Pending Review: Channel verification, relay shutdown

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-110 (MCP Server) | MCP server is relay endpoint | If MCP not running, relay fails | Graceful fallback (no relay = no Hermes) |
| Hermes protocol | External message bus | Protocol version changes | Pin supported Hermes version |
| Hermes binary | Must be installed separately | Not found | Clear error with install instructions |

## 10. Appendices
- `src-rust/project-setup/surface_hermes_command.rs` — CLI hermes command
- `src-rust/project-setup/capabilities_hermes_installer.rs` — Hermes installer
- `src-rust/project-setup/capabilities_hermes_remover.rs` — Hermes remover
- `src-rust/mcp-server/agent_hermes_relay.rs` — MCP-Hermes relay bridge
