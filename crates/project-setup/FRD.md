# FRD — project-setup (v3.0.0)

---

## Feature Overview

The project-setup crate provides scaffolding facilities and adapter installation
for new and existing projects. It detects project languages (Rust, Python,
JavaScript/TypeScript), generates MCP configuration for different AI clients,
generates `.env` files, installs linter adapters via pip/npm, and manages XDG
config directories.

The crate follows the AES 7-layer architecture. Four capability seams provide
MCP config generation, environment file creation, language detection, and adapter
installation. Auxiliary technical operations are delegated through the aggregate
contract and the injected utility protocols.

> **Note**: Environment diagnostics (`doctor`, `diagnose`) are handled by the
> **maintenance** crate, not project-setup. This crate focuses on scaffolding
> and installation only.

- **Architecture & Data Flow**

```mermaid
flowchart TD
    A["Surface"] -->|input| B["setup aggregate"]
    B --> C{"action"}

    C -->|"init/mcp-config"| D["mcp config generator"]
    C -->|"init/env"| E["env file generator"]
    C -->|"init/detect"| F["language detector"]
    C -->|"install"| G["adapter installer"]

    D --> H["MCP JSON"]
    E --> I[".env file"]
    F --> J["Language list"]
    G --> K["pip / npm"]
    K --> L["Success / Error"]

    H --> B
    I --> B
    J --> B
    L --> B
    B -->|output| A

```

---

## Functional Requirements

### FR-ProjectSetup-001: MCP Configuration Generation

| Aspect | Detail|
|--------|-------|
| Description | |
| Business Rules | |
| Edge Cases | |
| Capability | |
| Protocol | |
| Method | |
| Input | |
| Output | |
| Error | |
| Event | |
| Test | |
| Metric | |

### FR-ProjectSetup-002: Environment File Generation

| Aspect | Detail|
|--------|-------|
| Description | |
| Business Rules | |
| Edge Cases | |
| Capability | |
| Protocol | |
| Method | |
| Input | |
| Output | |
| Error | |
| Event | |
| Test | |
| Metric | |

### FR-ProjectSetup-003: Language Detection

| Aspect | Detail|
|--------|-------|
| Description | |
| Business Rules | |
| Edge Cases | |
| Capability | |
| Protocol | |
| Method | |
| Input | |
| Output | |
| Error | |
| Event | |
| Test | |
| Metric | |

### FR-ProjectSetup-004: Adapter Installation

| Aspect | Detail|
|--------|-------|
| Description | |
| Business Rules | |
| Edge Cases | |
| Capability | |
| Protocol | |
| Method | |
| Input | |
| Output | |
| Error | |
| Event | |
| Test | |
| Metric | |

### FR-ProjectSetup-005: Config Template Loading

| Aspect | Detail|
|--------|-------|
| Description | |
| Business Rules | |
| Edge Cases | |
| Capability | |
| Protocol | |
| Method | |
| Input | |
| Output | |
| Error | |
| Event | |
| Test | |
| Metric | |

### FR-ProjectSetup-006: Config File Writing

| Aspect | Detail|
|--------|-------|
| Description | |
| Business Rules | |
| Edge Cases | |
| Capability | |
| Protocol | |
| Method | |
| Input | |
| Output | |
| Error | |
| Event | |
| Test | |
| Metric | |

### FR-ProjectSetup-007: Pre-flight Check

| Aspect | Detail|
|--------|-------|
| Description | |
| Business Rules | |
| Edge Cases | |
| Capability | |
| Protocol | |
| Method | |
| Input | |
| Output | |
| Error | |
| Event | |
| Test | |
| Metric | |

### FR-ProjectSetup-008: File Existence Probe

| Aspect | Detail|
|--------|-------|
| Description | |
| Business Rules | |
| Edge Cases | |
| Capability | |
| Protocol | |
| Method | |
| Input | |
| Output | |
| Error | |
| Event | |
| Test | |
| Metric | |

## Public Interface

| Aspect | Detail|
|--------|-------|
| — | — |

## Reference

| Artifact | Purpose | Link|
|----------|---------|------|
| BACKLOG.md | Backlog tracking | [BACKLOG.md](BACKLOG.md)|
| PRD.md | Product requirements | [PRD.md](../../PRD.md)|
| FRD.md | Feature requirements | [FRD.md](FRD.md)|
| ROADMAP.md | Roadmap status | [ROADMAP.md](../../ROADMAP.md)|
| ARCHITECTURE.md | Layer contract | [ARCHITECTURE.md](../../ARCHITECTURE.md)|
