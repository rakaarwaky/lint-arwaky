# Feature Requirements Document (FRD)
**Feature Name:** Constant Purity Checker (AES033)  
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
This document defines the AES033 rule that ensures Taxonomy `_constant` files contain ONLY constant declarations (`pub const` / `pub static`). The rule is implemented in `check_constant()` within `TaxonomyRoleChecker` in `role-rules/capabilities_taxonomyrole_checker.rs`. AES033 enforces the architectural purity boundary around the Taxonomy `_constant` role — the only Taxonomy role permitted to expose raw primitives.

### 2.2 Scope
**In-Scope:**
- Detection of `pub fn`, `fn`, `pub struct`, `struct`, `pub enum`, `enum`, `pub mod`, `mod`, `pub use`, `use`, `pub trait`, `trait`, `pub type`, `type`, and `impl` declarations in `_constant` files
- Support for both Rust (`.rs`) and Python (`.py`) constant files
- Skipping of comments, docstrings, attributes (`#[...]`), and blank lines
- Imports (`use`, `pub(crate) use`) are explicitly allowed (they reference types used in `pub const` signatures)
- HIGH severity reporting

**Out-of-Scope:**
- Checking constant value correctness (type, mutability, naming)
- Enforcement in non-constant taxonomy files (`_vo`, `_entity`, `_error`, `_event`)
- File-level lint exemptions (no `#[allow(...)]` is allowed per AES014)

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **AES033** | Rule code for constant purity violation |
| **check_constant()** | Main detection method in `TaxonomyRoleChecker` |
| **TaxonomyRoleChecker** | Struct in `role-rules/capabilities_taxonomyrole_checker.rs` |
| **_constant file** | A file ending with `_constant.rs` or `_constant.py` in the taxonomy layer |
| **Non-constant declaration** | Any `pub fn`, `pub struct`, `pub enum`, `pub mod`, `pub use`, `pub trait`, `pub type`, `impl`, or their non-pub variants |

## 3. Feature Overview
### 3.1 Background & Problem
The AES architecture defines five Taxonomy roles: `_vo`, `_entity`, `_error`, `_event`, and `_constant`. Each role has a specific purpose, and the `_constant` role is uniquely permitted to expose raw primitive values (`pub const` / `pub static`). Over time, developers may add functions, structs, enums, or trait implementations to `_constant` files, collapsing the role boundary and reintroducing the primitive leakage that AES006 (Primitive Usage Checker) prevents in other taxonomy roles.

### 3.2 Business Goals
- Preserve the architectural purity of the `_constant` taxonomy role
- Prevent function definitions, struct declarations, and trait implementations from leaking into constant files
- Ensure `_constant` files remain focused on constant value declarations only
- Allow imports (needed for type references in `pub const` signatures)

### 3.3 Target Users
- **Developers**: Get automatic feedback when they accidentally add non-constant code to a `_constant` file
- **Architects**: Maintain the strict role separation within the Taxonomy layer

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer, I want to be warned when I add a `pub fn` or `fn` declaration to a `_constant` file.
- **US-002:** As a developer, I want to be warned when I add a `pub struct`, `pub enum`, `pub mod`, or `impl` block to a `_constant` file.
- **US-003:** As a developer, I want imports (`use`) to be allowed in `_constant` files (needed for type references).
- **US-004:** As an architect, I want comments and attributes to be skipped (they don't break constant purity).

### 4.2 Use Cases & Workflow
**Detection Pipeline:**
```
File: shared-common/taxonomy_violation_constant.rs

1. Extract basename from file path
2. Does basename end with "_constant.rs" or "_constant.py"? → NO → SKIP
3. Read file content
4. For each line:
   a. Skip blank lines
   b. Skip comments (//, #) and docstrings
   c. Skip attributes (#[derive(...)], #[cfg(...)], etc.)
   d. Is line "pub const " or "pub static "? → ALLOW → continue
   e. Is line "use " or "pub(crate) use "? → ALLOW → continue
   f. Is line any of:
      - "pub struct " / "struct "
      - "pub enum " / "enum "
      - "pub fn " / "fn "
      - "impl "
      - "pub mod " / "mod "
      - "pub trait " / "trait "
      - "pub use "
      - "pub type " / "type "
      → YES → AES033 HIGH flagged
```

### 4.3 Business Rules
- Severity: HIGH
- Only applies to files ending with `_constant.rs` or `_constant.py`
- Comments (`//`, `#`), attributes (`#[...]`), and blank lines are skipped
- `pub const` and `pub static` declarations are explicitly allowed
- `use` and `pub(crate) use` imports are explicitly allowed (they support constant type signatures)
- Line-level matching is case-sensitive and whitespace-sensitive
- No file-level exemptions (AES014 forbids `#[allow(...)]`)

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Detection per file | < 1ms (line-by-line scan) |
| NFR-002 | False positive rate | 0% for pure constant files |
| NFR-003 | False negative rate | 0% for files with non-constant declarations |

## 6. UI/UX Requirements
```
AES033 HIGH - src-rust/shared-common/taxonomy_violation_constant.rs
  AES033 CONSTANT_PURITY: _constant file contains non-constant declaration.
  WHY? Constants are the only Taxonomy role permitted to expose raw primitives. Mixing in structs/enums/functions collapses the role boundary and reintroduces primitive leakage.
  FIX: Move non-constant declarations to the appropriate file (_vo, _entity, or capability module) and keep the _constant file restricted to pub const / pub static declarations only.
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | `_constant.rs` file with `pub fn helper()` | `check_constant()` runs | AES033 HIGH flagged | Pending Review |
| AC-002 | `_constant.rs` file with `pub struct Config` | `check_constant()` runs | AES033 HIGH flagged | Pending Review |
| AC-003 | `_constant.rs` file with `impl Default for X` | `check_constant()` runs | AES033 HIGH flagged | Pending Review |
| AC-004 | `_constant.rs` file with `pub const FOO: &str = "bar"` | `check_constant()` runs | No AES033 | Pending Review |
| AC-005 | `_constant.py` file with `def helper():` | `check_constant()` runs | AES033 HIGH flagged | Pending Review |
| AC-006 | `_constant.rs` file with `use crate::Foo` (import) | `check_constant()` runs | No AES033 | Pending Review |
| AC-007 | Non-constant file (`taxonomy_order_vo.rs`) | `check_constant()` runs | Skipped | Pending Review |
| AC-008 | `_constant.rs` with only comments and consts | `check_constant()` runs | No AES033 | Pending Review |

## 8. Empirical Findings (Code Audit)

### 8.1 Current Implementation
- **Location**: `src-rust/role-rules/capabilities_taxonomyrole_checker.rs:63-85`
- **Status**: **FULLY IMPLEMENTED** — not a stub
- `check_constant()` method in `TaxonomyRoleChecker`:
  - Loaded and invoked from `agent_checking_coordinator.rs` at line 50 via `TaxonomyRoleChecker::new().check_constant(file, violations)`
  - Line-by-line scan with explicit allowlist for `pub const`, `pub static`, `use`, and `pub(crate) use`
  - Detects `pub struct`, `struct`, `pub enum`, `enum`, `pub fn`, `fn`, `impl`, `pub mod`, `mod`, `pub trait`, `trait`, `pub use`, `pub type`, `type` variants
- FRD_001 (6-layer architecture) confirms AES033 as **✅ IMPLEMENTED** at line 207

### 8.2 Bugs Found

1. **`pub use` is both allowlisted AND detected** (`taxonomyrole_checker.rs:71,78`)
   - Line 71: `if t.starts_with("use ") || t.starts_with("pub(crate) use ") { continue; }`
   - Line 78: `|| t.starts_with("pub use ")`
   - **Impact**: `pub use` is first skipped at line 71 (via `use ` prefix), BUT line 78 catches it again unnecessarily
   - This is a logic no-op (already skipped), but confusing for maintenance
   - **Fix**: remove `|| t.starts_with("pub use ")` from line 78 detection list, or add explicit comment

2. **Attribute detection is too broad** (`taxonomyrole_checker.rs:69`)
   - `if t.starts_with("#[") { continue; }`
   - This skips `#[cfg(test)]`, `#[derive(Debug)]`, etc.
   - BUT also skips anything starting with `#[` including possible false negatives
   - More critically, it does NOT prevent an `impl` block preceded by `#[derive(...)]` from passing if the `impl` starts at the same line
   - **Impact**: multi-line `impl` blocks starting with `#[derive]` on previous line may still be caught (correct), but edge cases exist
   - **Fix**: add robustness for multi-line attributes using a state machine

3. **No Python `class` detection** (`taxonomyrole_checker.rs`)
   - Detects `class` keyword only if `pub struct`/`struct`/`pub enum`/`enum` matches
   - In Python, `_constant.py` files could contain `class SomeClass:` which would NOT be detected
   - **Impact**: AES033 does not flag Python class definitions in `_constant.py`
   - **Fix**: add `t.starts_with("class ")` to the detection list

4. **No unit tests for `check_constant()`**
   - `capabilities_taxonomyrole_checker.rs` has no `#[cfg(test)]` module
   - **Impact**: no automated verification of constant purity detection logic
   - **Fix**: add unit tests with inline `_constant.rs` fixtures

### 8.3 What Needs to Be Added
- **Python `class` detection**: add `|| t.starts_with("class ")` to the forbidden patterns
- **Multi-line attribute handling**: track attribute state across lines
- **Unit tests**: `#[cfg(test)]` module with inline constant file content strings
- **Integration test fixture**: Python `_constant.py` with a function or class definition

### 8.4 What to Keep
- **Clean allowlist logic** Pending Review (lines 70-71: `pub const`/`pub static`/`use` pass through)
- **Layer/role separation** Pending Review (in TaxonomyRoleChecker, not mixed with other checks)
- **Line-by-line efficiency** Pending Review (O(n) scan, no regex overhead)
- **Comment, blank, and attribute skipping** Pending Review (line 69)
- **Coordinator integration** Pending Review (agent_checking_coordinator.rs:50)

### 8.5 Empirical Evidence from Test Projects
- `test-project-rust/src-rust/shared-common/taxonomy_violation_constant.rs`:
  - Contains `pub const AES033_CONSTANT_PURITY` at line 49 (correct)
  - Contains `pub fn aes003_naming_convention(...)` at line 52 (VIOLATION — function in a `_constant` file!)
  - **Expected**: AES033 HIGH flagged on line 52
  - **Current status**: pending test execution
- `test-project-python/src-python/taxonomy/` — No `_constant.py` fixture found

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-003 (Parsing) | File reading and line-by-line scan | File unreadable → skip (silent) | Log warning on read failure |
| Naming convention (AES003) | `_constant` suffix in filename | If file doesn't follow naming convention, AES003 fires first | Sequential check order: naming → role |
| AES009 (Mandatory struct/trait) | `_constant` files are exempt from AES009 | AES009 correctly skips `_constant` files | Verified in FRD_021 AC-004 |
| Test fixtures | Rust `_constant.rs` with violation | `taxonomy_violation_constant.rs` has `pub fn` at line 52 | Existing fixture works for AES033 |
| Config YAML | Severity/HIGH for AES033 | Not configurable per project | Add `severity` field for AES033 in YAML |

## 10. Appendices
- `src-rust/role-rules/capabilities_taxonomyrole_checker.rs:63` — `check_constant()`
- `src-rust/code-analysis/agent_checking_coordinator.rs:50` — Invocation via `TaxonomyRoleChecker`
- `src-rust/shared-common/taxonomy_violation_constant.rs` — Test fixture (contains `pub fn` violation at line 52)
- `lint_arwaky.config.rust.yaml:209` — AES033 config message
- `lint_arwaky.config.python.yaml:224` — Python AES033 message
- `lint_arwaky.config.javascript.yaml:220` — JavaScript AES033 message
- `docs/FRD_021_mandatory_struct_trait.md:96` — AES033 exemption for AES009
