# 📄 Feature Requirements Document (FRD)
**Feature Name:** Mandatory Struct/Trait Definition Checker (AES009)
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
This document defines the AES009 rule that requires every file to define at least one struct, enum, or trait. Files without a type definition violate the principle that each file encapsulates a distinct data type.

### 2.2 Scope
**In-Scope:**
- Detecting struct/enum/trait/class definitions per file
- Rust: `struct`, `enum`, `trait`, `pub struct`, `pub enum`, `pub trait`
- Python: `class`
- JS/TS: `class`, `export class`, `export default class`
- Skipping barrel files, entry points, and `_constant` files
- HIGH severity violations

**Out-of-Scope:**
- Naming rules (AES003 — separate FRD)
- Content validation beyond type detection

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **AES009** | Rule code for missing struct/trait definition |
| **check_mandatory_class_definition()** | Main detection method |
| **file_has_class_definition()** | Content scan for struct/enum/trait/class keywords |

## 3. Feature Overview
### 3.1 Background & Problem
Files could exist without defining any struct, enum, or trait — containing only loose functions, constants, or side effects. This violated the principle that each file should encapsulate a coherent type.

### 3.2 Business Goals
- Every file must define at least one type
- Prevent loose functions without struct/trait encapsulation
- Skip barrel files and constant-only files

### 3.3 Target Users
- **Developers**: Reminded to wrap functions in structs/traits
- **Architects**: Enforce type-oriented encapsulation

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer, I want to be warned if my file has no struct or trait definition, so I encapsulate my logic properly.

### 4.2 Use Cases & Workflow
**Detection:**
```
File: layer-rules/capabilities_loose_functions.rs
  Content: "fn do_something() { ... }"
  → No "struct", "enum", "trait", or "class" found
  → AES009 HIGH violation

File: layer-rules/capabilities_import_checker.rs
  Content: "pub struct ImportChecker; impl Checker for ImportChecker { ... }"
  → "pub struct" found
  → No violation
```

**Exceptions:** `__init__.py`, `main.py`, `mod.rs`, `lib.rs`, `_constant` files

### 4.3 Business Rules
- Severity: HIGH
- Skip barrel files, entry points, and `_constant` files
- Configurable via YAML `mandatory_class_definition` flag
- Root layer has `mandatory_class_definition: false`

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Detection per file | < 5ms |

## 6. UI/UX Requirements
```
AES009 HIGH - src-rust/layer-rules/capabilities_loose_functions.rs
  AES009 MANDATORY_CLASS_DEFINITION: File is missing a struct, enum, or trait definition.
  WHY? Encapsulation in structs/traits is required for proper modularization.
  FIX: Group functions into a struct or implement a Trait.
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | File with only functions (no struct/trait) | `check_mandatory_class_definition()` runs | AES009 HIGH flagged | Pending Review |
| AC-002 | File with struct definition | `check_mandatory_class_definition()` runs | No violation | Pending Review |
| AC-003 | Barrel file (mod.rs) | `check_mandatory_class_definition()` runs | Skipped | Pending Review |
| AC-004 | _constant file | `check_mandatory_class_definition()` runs | Skipped (AES033 precedence) | Pending Review |

## 8. Empirical Findings (Code Audit)

### 8.1 Current Implementation

| Component | Location | Lines | Status |
|-----------|----------|-------|--------|
| `check_mandatory_class_definition()` | `code-analysis/capabilities_metric_checker.rs:256` | 51 lines | Active — called at coordinator line 145 per-file |
| `file_has_class_definition()` | `code-analysis/capabilities_metric_checker.rs:59` | 20 lines | Helper — line-by-line string search |
| YAML config | `lint_arwaky.config.rust.yaml:127-133` | — | `mandatory_class_definition: true` for global, `false` for root |
| Unit tests | `capabilities_metric_checker.rs:309-519` | 210 lines | Test `count_lines`, `get_basename`, `check_line_counts` — **NOT** `check_mandatory_class_definition` |
| Test fixture | `test-project-rust/` | — | **None present** |

The implementation:
1. Skips barrel files and entry points (`mod.rs`, `__init__.py`, `main.py`, `lib.rs`, `py.typed`) — line 265-270
2. Skips `_constant` files (AES033 takes precedence) — line 273-275
3. Returns early if `mandatory_class_definition` config flag is `false` (e.g., root layer) — line 282-284
4. Checks exception list — line 286-288
5. Calls `file_has_class_definition()` — a string search for `struct`, `enum`, `trait`, `class` keywords prefixed by newline or at file start
6. If no definition found → AES009 HIGH with line 0

### 8.2 Bugs Found

| # | Bug | Location | Impact | Fix |
|---|-----|----------|--------|-----|
| B1 | **Brittle class definition detection** | `file_has_class_definition()` lines 59-78 | Only checks for `\nstruct ` (newline + keyword + space) or `starts_with("struct ")`. A file starting with a doc comment (`/// Foo\nstruct Foo;`) or with a visibility modifier like `pub(crate) struct Foo;` would be missed. Also, `starts_with("pub struct ")` fails if the file starts with whitespace or a BOM. | Use a regex like `r"(?m)^\s*(pub(\s*\(.*?\))?\s+)?(struct|trait|enum)\s"` for Rust, and `r"(?m)^\s*(export\s+default\s+)?class\s"` for Python/JS. |
| B2 | **Zero line number** | `capabilities_metric_checker.rs:304` — `make_result(file, 0, ...)` | AES009 violations report line 0. | Use `LineNumber::new(1)`. |
| B3 | **No unit test for `file_has_class_definition`** | No `#[test]` for lines 59-78 | 20-line function with Rust, Python, JS detection has zero test coverage. Tricky edge cases (trait vs `struct`, leading whitespace, no trailing space after keyword) are untested. | Add tests for Rust `struct`, `pub struct`, `pub(crate) struct`, `trait`, `pub trait`, `enum`, `pub enum`, Python `class`, JS `export class`. |
| B4 | **No unit test for `check_mandatory_class_definition`** | No `#[test]` for lines 256-306 | Despite 210 lines of unit tests in the file, `check_mandatory_class_definition` has zero coverage. Exception list, barrel skip, constant skip, config flag check all untested. | Add tempdir-based tests exercising all early-return paths and a violation case. |
| B5 | **No test fixture** | `test-project-rust/` | No file without a class/struct/trait exists in the test project to trigger AES009. | Add a file like `capabilities/only_functions_processor.rs` with only `fn` declarations and expect AES009. |
| B6 | **FRD appendix path outdated** | `FRD_021.md:109` | References `layer-rules/capabilities_metric_checker.rs:188` — actual file is at `code-analysis/capabilities_metric_checker.rs:256`. | Update appendix path. |

### 8.3 What Needs to Be Added

1. **Robust regex-based detection** for `file_has_class_definition()` — covering visibility modifiers, doc comments, leading whitespace, and multi-line attributes.
2. **Unit tests** for `file_has_class_definition()` — at least 8 test cases covering all supported languages and visibility modifiers.
3. **Unit tests** for `check_mandatory_class_definition()` — barrel skip, constant skip, exception list, disabled config, violation case.
4. **Integration test fixture** — a `.rs` file with only `fn` declarations (no struct/trait/enum) and assert AES009.
5. **Config parity for Python/JS** — Python and JS configs should have `mandatory_class_definition` if the checker supports those languages.

### 8.4 What to Keep

1. **_constant file exemption** — correctly skips `_constant` files per AES033 purity rules (line 273).
2. **Config-driven enforcement** — `mandatory_class_definition` flag per layer allows root layer exemption (YAML line 167).
3. **Multi-language detection** — the helper handles Rust (`struct`/`trait`/`enum`), Python (`class`), and JS (`export class`).
4. **Barrel/entry-point skip** — correct exemption for `mod.rs`, `__init__.py`, `main.py`, `lib.rs`.
5. **Exception list** — layer-specific exceptions are checked before running the content scan.

### 8.5 Empirical Evidence from Test Projects

| Project | File | Expected | Actual (current) | Notes |
|---------|------|----------|------------------|-------|
| `test-project-rust` | — | AES009 HIGH | ❌ **No test fixture exists** | No file-without-struct in the test project. |
| `self-lint` | `capabilities_metric_checker.rs` | No violation | ✅ | Contains `struct ArchMetricChecker`. |
| `self-lint` | `taxonomy_lint_vo.rs` | No violation | ✅ | Contains `pub struct LintResult`. |
| `self-lint` | `mod.rs` | Skipped | ✅ | `mod.rs` matched at line 267. |
| `self-lint` | `taxonomy_impure_constant.rs` (hypothetical `_constant` file) | Skipped | ✅ | `_constant.rs` suffix matched at line 273. |
| `self-lint` | Root-layer file (e.g., `main.rs`) | Skipped (config) | ✅ | Root layer has `mandatory_class_definition: false`. |
| `self-lint` | File with `pub(crate) struct Foo;` | No violation | ❌ **False positive** (B1) | `starts_with("pub struct ")` doesn't match `pub(crate)`. |
| `self-lint` | File with `/// doc\nstruct Foo;` | No violation | ✅ | `\nstruct ` matches after the doc comment. |

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| YAML config | `mandatory_class_definition` flag | Missing flag = no enforcement | Built-in default |
| Content regex | Rust/Python/JS keyword detection | False negative on complex generics | Conservative regex |

## 10. Appendices
- `src-rust/layer-rules/capabilities_metric_checker.rs:188` — `check_mandatory_class_definition()`
- `lint_arwaky.config.rust.yaml` — Global `Mandatory Struct or Trait Definition` rule
