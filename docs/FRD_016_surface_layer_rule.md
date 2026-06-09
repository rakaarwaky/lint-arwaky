# 📄 Feature Requirements Document (FRD)
**Feature Name:** Surface Layer Rule Checker (AES022)  
**Product:** Lint Arwaky v1.10.2  
**Author:** Raka  
**Date:** 09/06/2026  
**Version:** v1.1  

## 1. Document Control
| Version | Date | Author | Description of Changes | Approved By |
|---------|------|--------|----------------------|-------------|
| v1.0 | 08/06/2026 | Raka | Initial document creation | [Stakeholder] |
| v1.1 | 09/06/2026 | Raka | Updated to prefix-based architecture: layers are filename prefixes, not directories; updated file paths for 26 feature folders | [Stakeholder] |

## 2. Introduction
### 2.1 Purpose
This document defines the AES022 rule that enforces surface layer passivity. Surfaces must NOT contain domain logic — they are thin I/O layers that parse input and delegate to capabilities/infrastructure through `ServiceContainerAggregate`.

### 2.2 Scope
**In-Scope:**
- AES018: Surface barrel wiring check (files declared in barrel)
- AES019: Passive surface validation (method count, line count, if-depth)
- Three surface roles: Smart, Utility, Passive
- Threshold enforcement: 10 methods, 80 lines/function, depth 3

**Out-of-Scope:**
- Surface direct import check (AES023 — separate FRD)
- Non-surface layer rules

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **AES022** | Rule code for surface layer enforcement |
| **AES018** | Barrel wiring — utility surfaces must not import Smart surfaces |
| **AES019** | Passive surface — imports taxonomy only, no domain logic |
| **Smart Surface** | `_command`, `_handler`, `_controller`, `_entry` |
| **Utility Surface** | `_hook`, `_store`, `_provider`, `_router` |
| **Passive Surface** | `_component`, `_layout`, `_view` |
| **SurfaceHierarchyChecker** | Capability that enforces AES022 |

## 3. Feature Overview
### 3.1 Background & Problem
Surfaces contained domain logic — CLI commands implemented business algorithms, MCP handlers performed data transformations. There was no enforcement of the passivity principle. Utility surfaces imported Smart surfaces, creating circular dependencies.

### 3.2 Business Goals
- Surfaces must be passive I/O layers only
- Smart surfaces delegate via ServiceContainerAggregate
- Utility surfaces independent of Smart surfaces
- Passive surfaces know taxonomy only

### 3.3 Target Users
- **Developers**: Keep surfaces thin and focused on I/O
- **Architects**: Enforce clean architecture boundaries

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer, I want surfaces with too much domain logic to be flagged, so I keep my CLI commands thin.
- **US-002:** As an architect, I want passive surfaces restricted to taxonomy imports only, so presentation stays separate from logic.

### 4.2 Use Cases & Workflow
**AES018 — Barrel Wiring Check:**
```
For each file with surface_ prefix:
  Is it declared in the feature folder's mod.rs or __init__.py?
    → NOT declared → AES018 violation
```

**AES019 — Passive Surface Check:**
```
File: cli-commands/surface_dashboard_view.rs
  1. Count public methods: 15
  2. Max allowed: 10
  3. 15 > 10 → VIOLATION

  2. Check function body: user_display() = 120 lines
  3. Max allowed: 80
  4. 120 > 80 → VIOLATION

  3. Check if-depth: nested 4 levels
  4. Max depth: 3
  5. 4 > 3 → VIOLATION
```

### 4.3 Business Rules
- Severity: HIGH
- Smart surfaces: must delegate logic, not implement it
- Utility surfaces: must NOT import Smart surfaces
- Passive surfaces: must import taxonomy ONLY
- Thresholds: 10 methods, 80 lines/function, if-depth 3

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Surface detection per file | < 10ms |

## 6. UI/UX Requirements
```
AES022 HIGH - src-rust/cli-commands/surface_dashboard_view.rs
  AES019 PASSIVE_SURFACE_VIOLATION: Surface contains domain logic.
  Found 15 public methods (max 10), function body 120 lines (max 80).
  WHY? Surfaces must be passive I/O layers.
  FIX: Move business logic to capabilities.
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Surface with 15 methods (max 10) | AES019 check runs | Violation flagged | Pending Review |
| AC-002 | Surface with function body 120 lines (max 80) | AES019 check runs | Violation flagged | Pending Review |
| AC-003 | Surface declared in barrel | AES018 check runs | No violation | Pending Review |
| AC-004 | Surface NOT declared in barrel | AES018 check runs | Violation flagged | Pending Review |

## 8. Empirical Findings (Code Audit)

### 8.1 Current Implementation

| Component | Location | Lines | Status |
|-----------|----------|-------|--------|
| `check_surface_hierarchy()` | `layer-rules/capabilities_hierarchy_checker.rs:50` | 321 lines | Active — called at coordinator line 166 |
| AES018 barrel wiring | `lines 64-85` | 21 lines | CRITICAL severity (documented as HIGH) |
| AES019 passive check | `lines 87-268` | 181 lines | CRITICAL severity (documented as HIGH) |
| Helper functions (`is_in_surfaces`, `is_init`, `is_wired`, `stem`, `directory`) | `lines 273-330` | 57 lines | Shared utilities |
| Unit tests | `lines 332-371` | 39 lines | 4 tests — helper functions only |
| Test fixture | `test-project-rust/src-rust/surfaces/complex_busy_handler.py` | 26 lines | Python class with 12 public methods (>10 threshold) |

The implementation handles two sub-rules:
- **AES018**: Checks that every non-init file in `surfaces/` directory is imported/wired in the barrel (`__init__.py`, `mod.rs`, etc.)
- **AES019**: Checks that surface classes are passive — ≤10 public methods, ≤80 lines/method body, ≤3 if-nesting depth

Both emit `CRITICAL` severity (differs from FRD which documents HIGH).

### 8.2 Bugs Found

| # | Bug | Location | Impact | Fix |
|---|-----|----------|--------|-----|
| B1 | **Path-based detection instead of prefix-based** | `is_in_surfaces()` line 274-276 | Uses `f.to_string().contains("/surfaces/")` to identify surface files. After the vertical slicing refactoring, files are organized by **filename prefix** (`surface_*`), not directory. Any `surface_`-prefixed file not physically in a `/surfaces/` directory is silently skipped. | Change to check filename prefix `surface_` or `__surface` per the prefix-based architecture spec. |
| B2 | **Python-only method detection** | Lines 27-28 (`PY_METHOD_RE`), 31 (`PY_CLASS_RE`), 105-151 | The AES019 checker only detects Python `class` and `def` keywords. Rust files with `impl SurfaceHandler for ...` blocks and their methods are completely invisible to the passivity check. Since the project is primarily Rust, this is a critical blind spot. | Add Rust-aware detection for `impl` blocks, `fn` methods, and trait implementations. |
| B3 | **Severity mismatch — CRITICAL vs HIGH** | Lines 81, 264 | Both AES018 and AES019 use `Severity::CRITICAL`, but the FRD documents them as HIGH. CRITICAL severity causes automatic build failure regardless of score. | Downgrade to `Severity::HIGH` to match the FRD specification, unless CRITICAL is intentional — in which case update the FRD. |
| B4 | **No AES018 test fixture** | `test-project-rust/` | AES018 (barrel wiring check) has no test fixture. A file not imported in its barrel would test this path, but none exists. | Add a surface file not wired in its barrel to the test project. |
| B5 | **AES018/AES019 not tested** | Unit tests (lines 332-371) | Only helper functions (`is_in_surfaces`, `is_init`, `stem`, `directory`) are unit-tested. The core AES018 and AES019 detection logic has zero unit test coverage. | Add integration tests for `check_surface_hierarchy()` with known violating files. |
| B6 | **Hardcoded thresholds** | Lines 40-42 | `MAX_PUBLIC_METHODS = 10`, `MAX_FUNCTION_BODY_LINES = 80`, `MAX_IF_DEPTH = 3` are hardcoded constants. The FRD (section 9) flags this as needing YAML config migration, but no progress has been made. | Move thresholds to `lint_arwaky.config.rust.yaml` under a `surface_hierarchy:` section. |

### 8.3 What Needs to Be Added

1. **Prefix-based surface detection** — replace `is_in_surfaces()` path check with filename prefix `surface_` check.
2. **Rust method/class detection** — add regex or AST-based parsing for Rust `impl` blocks, `fn` methods, and trait definitions.
3. **Configurable thresholds** — move `MAX_PUBLIC_METHODS`, `MAX_FUNCTION_BODY_LINES`, `MAX_IF_DEPTH` to YAML config.
4. **Unit tests for AES018 and AES019** — tempdir-based tests with known-good and known-bad surface files.
5. **AES018 barrel wiring fixture** — add a surface file that is deliberately not wired in its barrel.
6. **Integration test assertion** — wire `test-project-rust/src-rust/surfaces/complex_busy_handler.py` into the test runner and assert AES019 is emitted.

### 8.4 What to Keep

1. **Comprehensive heuristics** — method count, body-line length, if-nesting depth are three good proxy metrics for passivity that catch real violations.
2. **Multi-language barrel detection** — `is_wired()` already checks for Python (`import`, `from .`), Rust (`mod`, `use`), and JS/TS (string reference) barrel patterns. This is robust.
3. **Existing unit tests** — 4 tests for helper functions (`is_in_surfaces`, `is_init`, `stem`, `directory`) provide basic coverage.
4. **Test fixture clarity** — `complex_busy_handler.py` with 12 public methods (2 over the threshold) is a well-documented test case.
5. **CLI integration** — called from the central coordinator, ensuring blanket coverage of all scanned files.

### 8.5 Empirical Evidence from Test Projects

| Project | File | Expected | Actual (current) | Notes |
|---------|------|----------|------------------|-------|
| `test-project-rust` | `surfaces/complex_busy_handler.py` | AES019 CRITICAL | ✅ Likely flagged | 12 public methods > 10 threshold. But path-based `is_in_surfaces()` check must pass first. |
| `self-lint` | `cli-commands/surface_check_command.rs` | AES019 check (if Rust had support) | ❌ **Skipped** (B2) | Rust `impl` block with methods — Python-only regex misses this entirely. |
| `self-lint` | Any `surface_`-prefixed file outside `/surfaces/` directory | AES018/AES019 check | ❌ **Skipped** (B1) | Prefix-based files not in `/surfaces/` path are invisible to the checker. |
| `test-project-rust` | (hypothetical) un-wired surface file | AES018 CRITICAL | ❌ **No test exists** (B4) | No fixture exercises the barrel wiring path. |
| `self-lint` | `surfaces/` directory files (if any) | AES018 barrel wiring | ✅ `is_wired()` works | Barrel detection handles Rust `mod` and `use` syntax. |

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-003 (Parsing) | Count methods, lines, depth | Regex inaccuracy affects thresholds | Conservative thresholds |
| Thresholds | 10/80/3 hardcoded | Not configurable via YAML | Plan: move to YAML config |

## 10. Appendices
- `src-rust/layer-rules/capabilities_hierarchy_checker.rs` — Full implementation (351 lines)
- `docs/ARCHITECTURE.md` — Surface layer specification
