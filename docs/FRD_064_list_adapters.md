# 📄 Feature Requirements Document (FRD)
**Feature Name:** List Adapters (`adapters`)  
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
This document defines the list adapters CLI command `adapters`. It displays all available external linting adapters that Lint Arwaky can use during `scan` operations, including their supported languages, installation status, version, and capabilities.

### 2.2 Scope
**In-Scope:**
- `lint-arwaky-cli adapters` — list all registered external adapters
- Adapter name, language, type, status (installed/not found), version
- Capability listing (lint, fix, format per adapter)
- Installation status check (is the tool available on PATH?)
- Filter by language with `--language` flag

**Out-of-Scope:**
- Installing adapters (handled by package managers)
- Running adapters (handled by `scan` in FR-056)
- Configuring adapter options

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **External adapter** | Bridge to a third-party linting/formatting tool |
| **ILinterAdapterPort** | Trait that all external adapters implement |
| **Adapter status** | Available / Not Found / Version Mismatch |
| **Capability** | lint / fix / format — what the adapter can do |

## 3. Feature Overview
### 3.1 Background & Problem
Developers didn't know which external linters Lint Arwaky could integrate with. The `scan` command would silently skip missing tools, and there was no way to see what adapters were available, which tools were installed, or what each adapter could do.

### 3.2 Business Goals
- Provide visibility into all available external adapters
- Show installation status for each tool
- Report versions for debugging compatibility
- List capabilities per adapter (lint, fix, format)

### 3.3 Target Users
- **Developers**: Check which external linters are available for their project
- **DevOps Engineers**: Verify CI environment has all required tools
- **AI Agents**: Query available adapters before running scan

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a Rust developer, I want to run `adapters` to see if clippy is available and its version.
- **US-002:** As a developer, I want to filter adapters by language with `adapters --language python`, so I only see relevant tools.
- **US-003:** As a DevOps engineer, I want to see which adapters support `fix` capability, so I can enable auto-fixing in CI.

### 4.2 Use Cases & Workflow
**List Adapters Pipeline:**
```
lint-arwaky-cli adapters
  │
  ├─► 1. Query adapter registry (ILinterAdapterPort implementations)
  │
  ├─► 2. For each adapter:
  │     ├── Check tool availability (which/version)
  │     └── Determine capabilities (lint, fix, format)
  │
  └─► 3. Display table grouped by language
```

**Adapter Registry Contents:**
| Adapter | Language | Type | Capabilities |
|---------|----------|------|-------------|
| ClippyAdapter | Rust | linter | lint, fix |
| RustfmtAdapter | Rust | formatter | format |
| RuffAdapter | Python | linter | lint, fix |
| ESLintAdapter | JavaScript | linter | lint, fix |
| TscAdapter | TypeScript | type-checker | lint |
| PrettierAdapter | JavaScript | formatter | format |

### 4.3 Business Rules
- Adapters are registered at compile time in the adapter registry
- Status is checked at runtime (not cached from previous scans)
- `--language <lang>` filter shows only adapters for that language
- Capabilities derived from `ILinterAdapterPort` trait methods implemented
- Version is displayed only if tool is found on PATH

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Registry query + status check (6 adapters) | < 1s |
| NFR-002 | Version accuracy | 100% |
| NFR-003 | Zero false negative for installed tools | 100% |

## 6. UI/UX Requirements
CLI output:
```
$ lint-arwaky-cli adapters
🔌 Available External Adapters
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Rust:
  │ Adapter     │ Status    │ Version    │ Capabilities      │
  ├─────────────┼───────────┼────────────┼───────────────────┤
  │ clippy      │ ✅ Ready  │ 0.1.78     │ lint, fix         │
  │ rustfmt     │ ✅ Ready  │ 1.7.0      │ format            │

Python:
  │ Adapter     │ Status    │ Version    │ Capabilities      │
  ├─────────────┼───────────┼────────────┼───────────────────┤
  │ ruff        │ ✅ Ready  │ 0.4.8      │ lint, fix         │

JavaScript/TypeScript:
  │ Adapter     │ Status        │ Version    │ Capabilities      │
  ├─────────────┼───────────────┼────────────┼───────────────────┤
  │ eslint      │ ❌ Not found  │ —          │ lint, fix         │
  │ tsc         │ ❌ Not found  │ —          │ lint              │
  │ prettier    │ ⚠️  Optional  │ —          │ format            │

Use `setup doctor` for detailed environment diagnostics.
```

Filtered output:
```
$ lint-arwaky-cli adapters --language rust
🔌 Rust Adapters
  clippy   v0.1.78   ✅ lint, fix
  rustfmt  v1.7.0    ✅ format
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | All 6 adapters registered | `adapters` runs | All adapters listed with status | Pending Review |
| AC-002 | clippy installed | `adapters` runs | clippy shows ✅ Ready + version | Pending Review |
| AC-003 | eslint not installed | `adapters` runs | eslint shows ❌ Not found | Pending Review |
| AC-004 | `--language rust` flag | `adapters --language rust` runs | Only Rust adapters shown | Pending Review |
| AC-005 | All capabilities listed | `adapters` runs | Each adapter shows correct capabilities | Pending Review |

## 8. Empirical Findings (Code Audit)

### 8.1 Current Implementation

| Component | Location | Lines | Status |
|-----------|----------|-------|--------|
| CLI adapters command | `cli-commands/surface_adapters_command.rs` | — | **FULLY IMPLEMENTED** |
| Adapter registry | `language-adapters/contract_adapter_registry.rs` | — | **FULLY IMPLEMENTED** — compile-time registration |
| Adapter status check | `language-adapters/capabilities_adapter_checker.rs` | — | **FULLY IMPLEMENTED** — PATH + version query |
| Display formatter | `cli-commands/capabilities_adapter_formatter.rs` | — | **FULLY IMPLEMENTED** — table output |

### 8.2 Bugs Found

1. **Adapter registry is hardcoded** — new adapters cannot be added via plugin system
   - **Impact**: Adding a new linter requires code change and recompile
   - **Fix**: Future: plugin system for dynamic adapter registration (currently out of scope)

2. **Status check uses `which` with 1s timeout per adapter** — on slow systems, `adapters` can take 6+ seconds
   - **Impact**: Slow feedback for users
   - **Fix**: Parallel status checks with 500ms timeout per adapter

3. **Version display for eslint is parser-dependent** — `eslint --version` outputs different formats based on Node.js version
   - **Impact**: Some eslint versions display "v8.56.0" while others display "8.56.0"
   - **Fix**: Strip leading 'v' character, use semver parsing

### 8.3 What Needs to Be Added

- **Parallel checks**: Run adapter status checks concurrently
- **Semver normalization**: Consistent version format display
- **Plugin registration**: Long-term: dynamic adapter loading

### 8.4 What to Keep

- **Clean table output** ✅ — grouped by language, readable
- **--language filter** ✅ — quick filtering
- **Status and capabilities** ✅ — accurate per-adapter

### 8.5 Empirical Evidence from Test Projects

- `lint-arwaky-cli adapters` correctly lists all 6 adapters with installation status
- Status checks match actual tool availability on the system
- Pending Review: Parallel checks, version normalization

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-056 (Scan) | Adapter registry shared with scan | Registry changes affect scan | Interface stability |
| External tools | All listed tools must be checked | Slow checks on network filesystem | Parallel + timeout |

## 10. Appendices
- `src-rust/cli-commands/surface_adapters_command.rs` — CLI adapters command
- `src-rust/language-adapters/contract_adapter_registry.rs` — Adapter registry
- `src-rust/language-adapters/capabilities_adapter_checker.rs` — Status checker
- `src-rust/cli-commands/capabilities_adapter_formatter.rs` — Display formatter
