# 📄 Feature Requirements Document (FRD)
**Feature Name:** Environment Diagnostics (`setup doctor`)  
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
This document defines the environment diagnostics CLI command `setup doctor`. It checks the local development environment for all required and optional tools needed by Lint Arwaky, including Rust toolchain (cargo, clippy, rustfmt), Python (ruff), JavaScript/Node (eslint, tsc, prettier), and system dependencies.

### 2.2 Scope
**In-Scope:**
- `lint-arwaky-cli setup doctor` — run full environment diagnostics
- Tool availability checks: cargo, clippy, rustfmt, ruff, eslint, tsc, prettier
- Version checks for each tool (minimum version requirements)
- Configuration file validation
- Git/Jujutsu VCS detection
- Report: pass/warning/fail per dependency

**Out-of-Scope:**
- Installing missing tools
- Creating config files (handled by FR-061)
- MCP configuration (handled by FR-062)

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **DoctorCheck** | Single environment check with name, status, detail |
| **DoctorReport** | Aggregated report of all checks |
| **CheckStatus** | Pass / Warning / Fail / Not Found |

## 3. Feature Overview
### 3.1 Background & Problem
Users would run `scan` or `fix` commands and get cryptic errors when external tools were missing or outdated. There was no way to validate the development environment before running commands. Debugging failed CI runs often required manual version checks for each tool.

### 3.2 Business Goals
- Provide a single command to validate the entire development environment
- Give clear, actionable messages for each missing or outdated tool
- Check Rust, Python, and JavaScript toolchains
- Report tool versions for debugging

### 3.3 Target Users
- **New Developers**: Run `setup doctor` after cloning the repo to ensure environment is ready
- **DevOps Engineers**: Debug CI environment issues
- **AI Agents**: Verify environment before running lint operations

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a new developer, I want to run `setup doctor` after cloning the repo to see if I have all required tools installed.
- **US-002:** As a developer, I want to see version information for each tool, so I can confirm I'm using a supported version.
- **US-003:** As a DevOps engineer, I want `setup doctor` to check config file validity, so I can debug configuration issues.

### 4.2 Use Cases & Workflow
**Doctor Pipeline:**
```
lint-arwaky-cli setup doctor
  │
  ├─► 1. Rust toolchain
  │     ├── cargo --version         → required
  │     ├── clippy available        → required (for Rust scans)
  │     ├── rustfmt available       → required (for Rust scans)
  │     └── rustc --version         → informational
  │
  ├─► 2. Python toolchain
  │     ├── python3 --version       → optional
  │     └── ruff --version          → required (for Python scans)
  │
  ├─► 3. JavaScript toolchain
  │     ├── node --version          → optional
  │     ├── eslint --version        → required (for JS scans)
  │     ├── tsc --version           → required (for TS scans)
  │     └── prettier --version      → optional
  │
  ├─► 4. Config file check
  │     └── lint_arwaky.config.*.yaml → parse validation
  │
  └─► 5. VCS check
        ├── git --version           → required
        └── jj --version            → optional (recommended)
```

### 4.3 Business Rules
- Checks are grouped by toolchain (Rust, Python, JavaScript, System)
- Required tool missing → FAIL status
- Optional tool missing → WARNING status
- Tool present but outdated version → WARNING status
- Version requirements documented in config or hardcoded minimums

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Full doctor check | < 3s |
| NFR-002 | Version parsing accuracy | 100% |
| NFR-003 | Zero false positives for installed tools | 100% |

## 6. UI/UX Requirements
CLI output:
```
$ lint-arwaky-cli setup doctor
🩺 Lint Arwaky Environment Diagnostics
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

🔧 Rust Toolchain:
  ✓ cargo 1.78.0
  ✓ clippy 0.1.78
  ✓ rustfmt 1.7.0
  ℹ️  rustc 1.78.0

🐍 Python Toolchain:
  ✓ python3 3.12.3
  ✓ ruff 0.4.8

🟨 JavaScript Toolchain:
  ✗ eslint — NOT FOUND (required for JS scans)
  ✗ tsc — NOT FOUND (required for TS scans)
  ⚠️  prettier — NOT FOUND (optional)

⚙️ Configuration:
  ✓ lint_arwaky.config.rust.yaml — valid

📦 VCS:
  ✓ git 2.43.0
  ⚠️  jj — NOT FOUND (optional, recommended)

Result: 2 FAIL, 2 WARNING, 6 PASS
Run `setup init` to create default config.
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | All tools installed and current | `setup doctor` runs | All PASS, exit 0 | Pending Review |
| AC-002 | eslint not installed | `setup doctor` runs | eslint FAIL reported | Pending Review |
| AC-003 | jj not installed | `setup doctor` runs | jj WARNING reported | Pending Review |
| AC-004 | Config file is malformed YAML | `setup doctor` runs | Config FAIL with parse error | Pending Review |
| AC-005 | No config file exists | `setup doctor` runs | Config WARNING with suggestion | Pending Review |

## 8. Empirical Findings (Code Audit)

### 8.1 Current Implementation

| Component | Location | Lines | Status |
|-----------|----------|-------|--------|
| CLI setup doctor command | `project-setup/surface_doctor_command.rs` | — | **FULLY IMPLEMENTED** |
| Doctor orchestrator | `project-setup/agent_doctor_orchestrator.rs` | — | **FULLY IMPLEMENTED** |
| Tool check helpers | `project-setup/capabilities_tool_checker.rs` | — | **FULLY IMPLEMENTED** — spawns `--version` commands |
| Config file validator | `project-setup/capabilities_config_validator.rs` | — | **FULLY IMPLEMENTED** |

### 8.2 Bugs Found

1. **Tool checker uses `which` command but falls back to hardcoded paths** — on NixOS or non-standard setups, tools may be in different locations
   - **Impact**: False negatives for correctly installed tools
   - **Fix**: Use `std::process::Command` with PATH lookup only

2. **eslint and tsc checked globally but project may use npx** — projects with local node_modules have eslint/tsc in `./node_modules/.bin/`
   - **Impact**: False negative for locally installed tools
   - **Fix**: Also check `./node_modules/.bin/<tool>` in project directory

3. **Version parsing uses `split(' ')` on raw output** — `cargo 1.78.0 (abc123 2024-05-04)` version parsing may break if format changes
   - **Impact**: Version comparison may fail on nightly toolchains
   - **Fix**: Use semver-compatible regex parsing

### 8.3 What Needs to Be Added

- **npx/local fallback**: Check local node_modules for eslint/tsc
- **PATH-only lookup**: Remove hardcoded fallback paths
- **Semver parsing**: Robust version extraction with regex

### 8.4 What to Keep

- **All tool checks** ✅ — 10+ tools verified
- **Grouped output** ✅ — readable categories (Rust, Python, JS, Config, VCS)
- **Pass/Warning/Fail status** ✅ — clear per-check indicators
- **Config validation** ✅ — YAML parse check

### 8.5 Empirical Evidence from Test Projects

- `lint-arwaky-cli setup doctor` runs successfully on standard Ubuntu/Debian setup
- Correctly detects missing eslint when only node is installed
- Pending Review: npx fallback, NixOS compatibility

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-002 (Config) | Config file validation | Malformed config = misleading doctor results | Separate parse error from missing |
| External tools | All checked tools must be on PATH | Platform-specific paths | PATH-only lookup |

## 10. Appendices
- `src-rust/project-setup/surface_doctor_command.rs` — CLI doctor command
- `src-rust/project-setup/agent_doctor_orchestrator.rs` — Doctor orchestrator
- `src-rust/project-setup/capabilities_tool_checker.rs` — Tool check helpers
- `src-rust/project-setup/capabilities_config_validator.rs` — Config validator
