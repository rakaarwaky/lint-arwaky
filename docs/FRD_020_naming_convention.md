# 📄 Feature Requirements Document (FRD)
**Feature Name:** Naming Convention Checker (AES003)
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
This document defines the AES003 rule that enforces the `[layer]_[concept]_[suffix]` naming pattern. Every file must follow `[layer]_[concept]_[suffix].rs` pattern where the layer prefix communicates architectural role, concept communicates domain purpose, and suffix communicates file type.

### 2.2 Scope
**In-Scope:**
- `[layer]_[concept]_[suffix]` prefix-based naming pattern
- Stem extraction and regex validation
- Barrel file and entry point exemption
- Layer-specific exception list
- HIGH severity violations

**Out-of-Scope:**
- File suffix rules (AES008/AES010/AES011 — separate FRDs)
- Symbol naming inside files

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **AES003** | Rule code for naming convention violation |
| **check_file_naming()** | Main detection method |
| **get_stem()** | Removes file extension |
| **Prefix-based pattern** | `^(taxonomy_|contract_|capabilities_|infrastructure_|agent_|surface_)[a-z]+_[a-z]+$` |

## 3. Feature Overview
### 3.1 Background & Problem
Files had arbitrary names with no structural meaning — `helpers.rs`, `utils.rs`, `types.rs`. Without enforced naming, a file's domain and architectural role was not visible from its path.

### 3.2 Business Goals
- Every filename communicates domain + concept + role
- Enforce consistent prefix-based pattern across codebase
- Provide clear rename guidance

### 3.3 Target Users
- **Developers**: Guided to use correct naming pattern
- **Architects**: Standardize naming across the codebase

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer adding a file, I want to be told if my filename doesn't follow the `[layer]_[concept]_[suffix]` pattern.

### 4.2 Use Cases & Workflow
**Validation:**
```
"capabilities_import_checker.rs" → stem "capabilities_import_checker"
  → regex ^(taxonomy_|contract_|capabilities_|infrastructure_|agent_|surface_)[a-z]+_[a-z]+$ → MATCH Pending Review

"helpers_utils.rs" → stem "helpers_utils"
  → regex ^(taxonomy_|contract_|capabilities_|infrastructure_|agent_|surface_)[a-z]+_[a-z]+$ → NO MATCH (no layer prefix) Pending Review
```

**Exceptions:** `main.rs`, `lib.rs`, `mod.rs`, `__init__.py`, `index.ts`, `index.js`

### 4.3 Business Rules
- Severity: HIGH
- Default pattern: `[layer]_[concept]_[suffix]` (configurable via YAML)
- Barrel files and entry points are skipped
- Layer-specific exception list honored

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Detection per file | < 5ms |
| NFR-002 | False positive rate | < 1% |

## 6. UI/UX Requirements
```
AES003 HIGH - src-rust/shared-common/helpers_utils.rs
  AES003 NAMING_CONVENTION: Filename does not follow the [layer]_[concept]_[suffix] pattern.
  WHY? Prefix-based naming ensures architectural consistency across 26 feature folders.
  FIX: Rename to something like taxonomy_helpers_utils.rs.
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | File without layer prefix | `check_file_naming()` runs | AES003 HIGH flagged | Pending Review |
| AC-002 | File with valid [layer]_[concept]_[suffix] name | `check_file_naming()` runs | No violation | Pending Review |
| AC-003 | Barrel file (mod.rs) | `check_file_naming()` runs | Skipped | Pending Review |
| AC-004 | Entry point (main.rs) | `check_file_naming()` runs | Skipped | Pending Review |

## 8. Empirical Findings (Code Audit)

### 8.1 Current Implementation

| Component | Location | Lines | Status |
|-----------|----------|-------|--------|
| `check_file_naming()` (primary) | `naming-rules/capabilities_naming_checker.rs:89` | 57 lines | Active — called at coordinator line 146 per-file |
| `check_file_naming()` (duplicate) | `naming-rules/capabilities_convention_checker.rs:93` | 80 lines | **Not called** — dead code in the coordination flow |
| YAML config | `lint_arwaky.config.rust.yaml:100-106` | — | `word_count: 3`, exceptions list |
| Test fixture | `test-project-rust/` | — | **None present** — no AES003 fixture exists |

The primary implementation:
1. Skips barrel files (`mod.rs`, `__init__.py`, `index.ts`) — line 98
2. Skips entry points (`main.rs`, `lib.rs`, `app.py`, etc.) — line 98
3. Checks layer-specific exception list — line 103
4. Resolves expected word count from layer definition or global config (default 3) — lines 108-116
5. Builds regex `^[a-z0-9]+(_[a-z0-9]+){{}}$` where `N = expected_word_count - 1` — line 119
6. If filename stem doesn't match → AES003 HIGH — line 142

### 8.2 Bugs Found

| # | Bug | Location | Impact | Fix |
|---|-----|----------|--------|-----|
| B1 | **Regex uses generic word-count, not layer prefix** | `capabilities_naming_checker.rs:119` | The FRD specifies a layer-prefix-aware pattern (`^(taxonomy_\|contract_\|...)[a-z]+_[a-z]+$`) but the code uses a generic word-count regex (`^[a-z0-9]+(_[a-z0-9]+){{}}$`). Any 3-word filename passes, even without a layer prefix, e.g., `helpers_utils_types.rs` would pass despite having no `taxonomy_` or `capabilities_` prefix. | Change regex to require a known layer prefix from `PREFIX_MAP` (defined at line 440 in the import checker) as the first word. |
| B2 | **Zero line number** | `capabilities_naming_checker.rs:28` — `make_result` | All AES003 violations report line 0, even though the violation is clearly file-level. | Use `LineNumber::new(1)`. |
| B3 | **Dead-code duplicate implementation** | `capabilities_convention_checker.rs:93` | A second `check_file_naming()` implementation exists (word-count-based) but is never called from the coordinator. It's 80 lines of dead code that creates maintenance confusion. | Either remove it or refactor both into a single path. |
| B4 | **No AES003 test fixture** | `test-project-rust/` | Unlike AES010/AES011 which have fixtures, AES003 has zero test coverage. No file in the test project exercises the naming convention check. | Add a file like `bad_name.rs` (no layer prefix, no underscores) and expect AES003. |
| B5 | **FRD appendix path outdated** | `FRD_020.md:106` | References `layer-rules/capabilities_naming_checker.rs:65` — actual file is at `naming-rules/capabilities_naming_checker.rs:89`. | Update appendix path. |

### 8.3 What Needs to Be Added

1. **Layer-prefix validation in regex** — require the first underscore-delimited word to be a recognized layer prefix (`taxonomy`, `contract`, `capabilities`, `infrastructure`, `agent`, `surface`).
2. **Unit tests**:
   - 3-word valid name (e.g., `capabilities_import_checker.rs`) → no violation
   - 2-word name (`helpers_utils.rs`) → AES003 HIGH
   - 4-word name (`taxonomy_some_long_name.rs`) → AES003 HIGH
   - Barrel file `mod.rs` → skipped
   - Entry point `main.rs` → skipped
   - Exception file → skipped
   - Name without layer prefix (`utils_helpers.rs`) → AES003 HIGH
3. **Integration test fixture** — add a file with a bad naming pattern and wire it into the test runner.
4. **Consolidate two implementations** — remove the dead duplicate at `capabilities_convention_checker.rs:93` or merge the logic.

### 8.4 What to Keep

1. **Barrel/entry-point skipping** — correct exemption for `mod.rs`, `main.rs`, `lib.rs`, `__init__.py`, etc.
2. **Exception list** — per-layer exceptions are correctly checked before the regex runs.
3. **Config-driven word count** — word count is configurable per layer with fallback to global default.
4. **Config message override** — `word_count_violation_message` from YAML is used when available.
5. **CLI integration** — called per-file from the central coordinator, ensuring blanket coverage.

### 8.5 Empirical Evidence from Test Projects

| Project | File | Expected | Actual (current) | Notes |
|---------|------|----------|------------------|-------|
| `test-project-rust` | — | AES003 HIGH | ❌ **No test fixture exists** | No file with wrong word count or missing layer prefix exists. |
| `self-lint` | `capabilities_naming_checker.rs` | No violation | ✅ | `capabilities_naming_checker` → 3 words ✓ |
| `self-lint` | `agent_checking_coordinator.rs` | No violation | ✅ | `agent_checking_coordinator` → 3 words ✓ |
| `self-lint` | `taxonomy_lint_vo.rs` | No violation | ✅ | `taxonomy_lint_vo` → 3 words ✓ |
| `self-lint` | `helpers_utils.rs` (hypothetical) | AES003 HIGH | ❌ **Would NOT fire** (B1) | Generic regex `^[a-z0-9]+(_[a-z0-9]+){2}$` matches `helpers_utils` (2 underscores → 3 words). But missing layer prefix. |
| `self-lint` | `mod.rs` | Skipped | ✅ | `is_barrel_file()` returns true. |
| `self-lint` | `main.rs` | Skipped | ✅ | `is_entry_point()` returns true. |
| `self-lint` | `main.rs`, `lib.rs` | Skipped | ✅ | Correctly exempted. |

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| YAML config | Pattern from config | Missing config = default [layer]_[concept]_[suffix] | Built-in default |
| regex crate | Naming regex validation | Complex patterns may fail | Simple prefix pattern only |

## 10. Appendices
- `src-rust/layer-rules/capabilities_naming_checker.rs:65` — `check_file_naming()`
- `lint_arwaky.config.rust.yaml` — Global `Naming Convention` rule
