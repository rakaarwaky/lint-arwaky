# 📄 Feature Requirements Document (FRD)
**Feature Name:** Display Version (`version`)  
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
This document defines the display version CLI command `version`. It prints the current Lint Arwaky application version, build metadata, and optional system information. Provides a standard way for users and CI pipelines to verify which version is installed.

### 2.2 Scope
**In-Scope:**
- `lint-arwaky-cli version` — display application version
- Version string: semantic version (x.y.z)
- Build metadata: commit SHA, build date, Rust compiler version
- License information
- `--verbose` flag for detailed build information

**Out-of-Scope:**
- Version checking against remote (no "update available" check)
- Package manager integration

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **Semantic version** | Version in MAJOR.MINOR.PATCH format |
| **Build metadata** | Commit hash, build timestamp, rustc version |
| **CLAP** | Command Line Argument Parser crate used for CLI |

## 3. Feature Overview
### 3.1 Background & Problem
There was no standard way to check which version of Lint Arwaky was installed. Users had to check Cargo.toml or guess from behavior. CI pipelines could not log the version for debugging.

### 3.2 Business Goals
- Provide standard `--version` / `version` command
- Include build metadata for debugging
- Follow semantic versioning conventions
- Minimal output by default, verbose on request

### 3.3 Target Users
- **All Users**: Verify installed version
- **DevOps Engineers**: Log version in CI pipeline output
- **Bug Reporters**: Include version in bug reports

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a user, I want to run `lint-arwaky-cli version` to see which version I have installed.
- **US-002:** As a developer, I want `--verbose` to show build commit and date, so I can identify the exact build in bug reports.
- **US-003:** As a CI pipeline, I want the version command to output clean text for log capture.

### 4.2 Use Cases & Workflow
**Version Display:**
```
lint-arwaky-cli version
  │
  └─► Print: "Lint Arwaky v1.10.2"

lint-arwaky-cli version --verbose
  │
  └─► Print:
        Lint Arwaky v1.10.2
        Commit: a1b2c3d4e5f6
        Build Date: 2026-06-09
        Rustc: 1.78.0 (9b00956e5 2024-04-29)
        License: MIT
```

### 4.3 Business Rules
- Version string from `CARGO_PKG_VERSION` env var
- Commit SHA from `VERGEN_GIT_SHA` or `git rev-parse HEAD` at build time
- Build date from `VERGEN_BUILD_TIMESTAMP` or chrono
- `-V` / `--version` flag also supported for CLI conventions
- No network calls for version check

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Version output | < 10ms |
| NFR-002 | Offline capability | 100% — no network required |

## 6. UI/UX Requirements
CLI output:
```
$ lint-arwaky-cli version
Lint Arwaky v1.10.2

$ lint-arwaky-cli --version
Lint Arwaky v1.10.2

$ lint-arwaky-cli version --verbose
Lint Arwaky v1.10.2
  Commit:    a1b2c3d4e5f6
  Built:     2026-06-09T10:30:00Z
  Rustc:     1.78.0 (9b00956e5 2024-04-29)
  License:   MIT
  Profile:   release
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Lint Arwaky installed | `lint-arwaky-cli version` runs | Version printed, exit 0 | Pending Review |
| AC-002 | `--version` flag used | `lint-arwaky-cli --version` runs | Same output as `version` command | Pending Review |
| AC-003 | `--verbose` flag used | `version --verbose` runs | Version + commit + build date displayed | Pending Review |

## 8. Empirical Findings (Code Audit)

### 8.1 Current Implementation

| Component | Location | Lines | Status |
|-----------|----------|-------|--------|
| CLI version command | `cli-commands/surface_version_command.rs` | — | **FULLY IMPLEMENTED** — prints VERSION constant |
| Build metadata | `cli-commands/taxonomy_build_constant.rs` | — | **FULLY IMPLEMENTED** — CARGO_PKG_VERSION + optional vergen |
| CLI --version flag | `cli-commands/surface_main_handler.rs` | — | **FULLY IMPLEMENTED** — clap `#[command(version)]` |

### 8.2 Bugs Found

1. **Version string uses `CARGO_PKG_VERSION` which may not match release tag** — if the crate version is not bumped on release, the version string is wrong
   - **Impact**: Version shown may not match git tag
   - **Fix**: Enforce version bump in release workflow

2. **--verbose requires build-time `vergen` dependency** — without vergen, commit SHA shows "unknown"
   - **Impact**: Inconsistent verbose output across builds
   - **Fix**: Fallback to `git rev-parse HEAD` at runtime (best-effort)

### 8.3 What Needs to Be Added

- **Runtime git fallback**: If vergen not available, try `git rev-parse HEAD` at runtime
- **Release workflow**: Automated version bump and tag on release

### 8.4 What to Keep

- **Simple default output** ✅ — clean version string
- **--verbose option** ✅ — detailed build info
- **--version flag** ✅ — standard CLI convention

### 8.5 Empirical Evidence from Test Projects

- `lint-arwaky-cli version` prints "Lint Arwaky v1.10.2"
- `lint-arwaky-cli --version` same output via clap
- Pending Review: Runtime git fallback for commit SHA

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| CARGO_PKG_VERSION | Cargo environment variable | Must be manually bumped | Automated release workflow |
| vergen (optional) | Build-time git metadata | Not available in all builds | Runtime fallback to `git` |

## 10. Appendices
- `src-rust/cli-commands/surface_version_command.rs` — CLI version command
- `src-rust/cli-commands/taxonomy_build_constant.rs` — Build metadata constants
- `Cargo.toml` — version field
