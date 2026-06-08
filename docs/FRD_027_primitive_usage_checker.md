# 📄 Feature Requirements Document (FRD)
**Feature Name:** Primitive Usage Checker (AES006)  
**Product:** Lint Arwaky v1.10.2  
**Author:** Raka  
**Date:** 08/06/2026  
**Version:** v1.0  

## 1. Document Control
| Version | Date | Author | Description of Changes | Approved By |
|---------|------|--------|----------------------|-------------|
| v1.0 | 08/06/2026 | Raka | Initial document creation | [Stakeholder] |

## 2. Introduction
### 2.1 Purpose
This document defines the AES006 rule that detects raw primitive type usage in domain types. Entities, errors, events, and contract interfaces must use Value Objects instead of raw primitives (e.g., `String`, `i32`, `str`, `int`), following Domain-Driven Design principles. VO and Constant files are exempt.

### 2.2 Scope
**In-Scope:**
- Primitive detection in taxonomy(entity), taxonomy(error), taxonomy(event)
- Primitive detection in contract(port), contract(protocol)
- Exemption for taxonomy(vo) and taxonomy(constant) — primitives allowed
- Three languages: Rust, Python, JavaScript/TypeScript
- HIGH severity

**Out-of-Scope:**
- Files outside taxonomy and contract layers (capabilities, infrastructure, agent, surfaces)
- Auto-fixing violations
- Automatic primitive-to-VO conversion

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **AES006** | Rule code for primitive usage violation |
| **check_primitive_usage()** | Active inline checker in `lint_checking_coordinator.rs` |
| **DomainTypeRuleChecker** | Orphan capability in `domain_type_checker.rs` (unused) |
| **no_primitives** | Config flag controlling which scopes are checked |
| **CORE_PRIMITIVE_TYPES** | Primitive type constant in `naming_symbols_constant.rs` |
| **VO (Value Object)** | Domain type wrapping primitives with business rules |

## 3. Feature Overview
### 3.1 Background & Problem
Before AES006, domain entities, errors, and events could use raw primitives like `String name` or `int age` directly. This violates DDD principles where all domain types should use Value Objects with validation and business rules. There was no automated mechanism to catch these violations.

### 3.2 Business Goals
- Ensure all domain entities use Value Objects instead of primitives
- Prevent primitive leakage in domain errors and events
- Contract interfaces must use domain types, not primitives
- Exempt VO and Constant files (they legitimately wrap/expose primitives)

### 3.3 Target Users
- **Developers**: Get feedback when using primitives in domain types
- **Domain Architects**: Enforce DDD purity in the taxonomy layer

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer, I want to be warned when I use `String` or `i32` in an entity, so I create proper Value Objects.
- **US-002:** As a developer, I do NOT want to be warned when I use primitives in a VO file, since VOs are meant to wrap primitives.
- **US-003:** As an architect, I want to enable/disable this rule per scope via YAML (`no_primitives` flag).

### 4.2 Use Cases & Workflow
**Current Detection Pipeline (inline checker):**
```
File: taxonomy/massive_domain_entity.rs

1. Does path contain "/taxonomy/"? → YES ✅
2. For each line:
   a. Does line contain ":" and end with "," or "}"? → YES (field def)
   b. Extract type after ":"
   c. Is type in ["String","i32","bool",...]? → YES
   d. Flag AES006 HIGH
```

**Correct Detection Pipeline (should be):**
```
File: taxonomy/massive_domain_entity.rs

1. Detect layer: taxonomy(entity)
2. Is no_primitives = true? → YES ✅ (config line 190)
3. Parse file via AST scanner → extract all field types
4. Any fields with primitive types? → String, i32 — YES ❌
5. Flag AES006 HIGH

File: taxonomy/address_vo.rs (VO — exempt)

1. Detect layer: taxonomy(vo)
2. Is no_primitives = false? → YES (config line 186)
3. Skip — no check performed ✅
```

### 4.3 Business Rules
- Severity: HIGH
- Scopes checked: `taxonomy(entity)`, `taxonomy(error)`, `taxonomy(event)`, `contract`
- Scopes NOT checked: `taxonomy(vo)`, `taxonomy(constant)`, `capabilities`, `infrastructure`, `agent`, `surfaces`
- Violation message per scope from YAML
- Rust primitives: `String, i8-i128, u8-u128, f32, f64, bool, char, Vec<, HashMap<, Option<, Result<`
- Python primitives: `str, int, float, bool, list, dict, tuple, set`
- JS/TS primitives: `string, number, boolean, any, object, Array, Record`

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Detection per file | < 20ms |
| NFR-002 | False positive rate (VO files) | 0% |
| NFR-003 | Cross-language support | Rust, Python, JS/TS |
| NFR-004 | Config-driven scoping | Reflects `no_primitives` in YAML |

## 6. UI/UX Requirements
```
AES006 HIGH - test-project-rust/src-rust/taxonomy/massive_domain_entity.rs:5
  AES006 PRIMITIVE_USAGE: Direct primitive 'String' in taxonomy.

AES006 HIGH - test-project-python/src-python/taxonomy/raw_entity.py:5
  AES006 PRIMITIVE_USAGE: Direct primitive 'str' in taxonomy.
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Rust entity with `String`, `i32` | Primitive check runs | AES006 HIGH flagged | ✅ (inline checker) |
| AC-002 | Python entity with `str`, `int` | Primitive check runs | AES006 HIGH flagged | ❌ **Not checked by inline checker** |
| AC-003 | JS/TS entity with `string`, `number` | Primitive check runs | AES006 HIGH flagged | ❌ **Not checked at all** |
| AC-004 | Rust VO file with `String` | Primitive check runs | **NOT** flagged (exempt) | ❌ **Inline checker flags it — false positive** |
| AC-005 | Rust constant file with primitives | Primitive check runs | **NOT** flagged (exempt) | ❌ **Inline checker flags it — false positive** |
| AC-006 | File outside taxonomy | Primitive check runs | **NOT** checked | ✅ (inline early return) |
| AC-007 | Config `no_primitives: false` for a scope | Primitive check runs | **NOT** flagged for that scope | ❌ **Config flag never consumed** |

## 8. Empirical Findings (Code Audit)

### 8.1 Four Implementations — Only One Active

#### 8.1.1 Inline Checker (ACTIVE) — `lint_checking_coordinator.rs:177-191`
```rust
fn check_primitive_usage(file: &str, content: &str, violations: &mut Vec<LintResult>) {
    if !file.contains("/taxonomy/") { return; }
    let primitives = ["String","i8","i16",...,"Result<"];
    for (i, line) in content.lines().enumerate() {
        let t = line.trim();
        if t.contains(':') && (t.ends_with(',') || t.ends_with('}')) {
            // heuristic: extract type after ':', check against primitive list
            ...
        }
    }
}
```
**Status**: ✅ Active, called in `run_all_checks()` line 47.

#### 8.1.2 DomainTypeRuleChecker (ORPHAN) — `domain_type_checker.rs:24-92`
```rust
pub fn find_primitive_violations(&self, file_path: &str, primitive_types: &[&str]) -> Vec<PrimitiveViolation>
```
**Status**: ❌ **Never called** by any pipeline. Dead code.

#### 8.1.3 AST Scanners (UNUSED for AES006) — `ast_rust_scanner.rs`, `ast_py_scanner.rs`, `ast_js_scanner.rs`
All scanners implement `find_primitive_violations()` via `ISourceParserPort`.
**Status**: ❌ **Never called for AES006**. Scanners only used for import extraction, symbol detection, etc.

#### 8.1.4 PythonPrimitiveChecker (EMPTY STUB) — `python_primitive_checker.rs`
```rust
pub struct PythonPrimitiveChecker;
impl PythonPrimitiveChecker {
    pub fn new() -> Self { Self }
}
```
**Status**: ❌ **Hollow stub** — 9 lines, empty struct with no methods. Originally planned for generated Python primitive checker but never implemented.

### 8.2 Bugs Found

1. **`no_primitives` config flag NEVER consumed by checker** (CRITICAL)
   - YAML config defines `no_primitives: true/false` per scope (lines 184-242)
   - Inline checker at `lint_checking_coordinator.rs:177-191` uses a hardcoded primitive list
   - `LayerDefinition.no_primitives` is set in `architecture_compliance_orchestrator.rs:385-387` but never read by any checker
   - **Impact**: all taxonomy files are checked uniformly regardless of config

2. **VO and Constant files are falsely flagged** (CRITICAL)
   - Config: `taxonomy(vo): no_primitives: false` (line 186)
   - Config: `taxonomy(constant): no_primitives: false` (line 215)
   - Inline checker checks ALL files under `/taxonomy/` without discrimination
   - **Impact**: false positives on legitimate VO and Constant files

3. **Rust-only support** (CRITICAL)
   - Primitive list is hardcoded for Rust: `String, i32, Vec<`, etc.
   - Python files under `/taxonomy/` are not detected
   - JS/TS files are not detected at all

4. **Fragile regex heuristic**
   - `line.contains(':') && (line.ends_with(',') || line.ends_with('}'))` — only matches struct fields
   - Does NOT catch: tuple struct fields, function signatures, const/static types, associated types
   - `fn process(&self) -> bool { true }` gets flagged because `: bool` and `}` ✅ (accidental match)
   - `let x: String = ...` is NOT flagged ❌ (false negative)

5. **`CORE_PRIMITIVE_TYPES` unused**
   - `naming_symbols_constant.rs:10`: `pub const CORE_PRIMITIVE_TYPES: &[&str] = &["str", "int", "float"]`
   - Defined for Python but never referenced by any checker

6. **`architecture_internal_checker.rs:145` — dead-end TODO comment**
   - `// Note: no_primitives check (AES006) requires AST parsing of class attributes. That is delegated to the main ArchitectureRulesEvaluator which has AST access.`
   - `ArchitectureRulesEvaluator` does not exist — delegation target is mythical

### 8.3 What Needs to Be Added

1. **Proper checker consuming `no_primitives` config**
   - Read `LayerDefinition.no_primitives` to decide whether a file needs checking
   - Expand primitive list per language
   - Integrate with `ISourceParserPort::find_primitive_violations()` from existing AST scanners

2. **Python and JavaScript support**
   - Activate `find_primitive_violations()` in `ast_py_scanner.rs` and `ast_js_scanner.rs`
   - Route through `SourceParserOrchestrator` by file extension

3. **Unit tests** for all primitive checking logic
   - Rust struct fields
   - Python class attributes
   - JS/TS class properties
   - VO exemption (negative test)
   - Config `no_primitives: false` (negative test)

4. **More comprehensive test fixtures**
   - Rust: entity with primitives (`String`, `i32`, `bool`)
   - Rust: VO with primitives (must be exempt — negative test)
   - Python: entity with `str`, `int`
   - JS/TS: entity with `string`, `number`

### 8.4 What Needs to Be Removed

1. **`domain_type_checker.rs` — orphan/dead code**
   - `DomainTypeRuleChecker` is never called
   - Either wire it into the pipeline or delete it in favor of integrated solution

2. **`python_primitive_checker.rs` — empty stub**
   - 9 lines, empty struct, no useful functionality
   - Either implement fully or delete

### 8.5 What to Keep

- **Inline checker as fallback**: until proper implementation lands, inline checker still catches Rust taxonomy primitives (with false positives)
- **YAML config structure**: `no_primitives`, scope definitions, and violation messages are already correct — only wiring is needed
- **`CORE_PRIMITIVE_TYPES` constant**: needs expansion and integration, but the concept is correct
- **`ISourceParserPort::find_primitive_violations`** interface: already defined and implemented in all three scanners

### 8.6 Empirical Evidence from Test Projects

**AES006 detected in TEST.md ✅** (line 96 — AES006 in 30 unique codes)
Test fixtures:
- `test-project-python/taxonomy/raw_entity.py` — `str`, `int` violations ✅
- `test-project-python/taxonomy/raw_error.py` — `int`, `str` violations ✅
- `test-project-rust/taxonomy/massive_domain_entity.rs` — `String`, `i32` (also triggers AES006)

**Missing:**
- Rust-specific entity fixture with explicit primitives (separate file)
- JS/TS entity fixture with primitives
- Rust VO file with primitives (for negative test — must NOT flag)

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-003 (AST Parsing) | Scanners for type detection per language | Scanners not wired for AES006 | Integrate via ISourceParserPort |
| Config YAML `no_primitives` | Enable/disable flag per scope | Flag not consumed → false positives | Wire config to checker logic |
| SourceParserOrchestrator | Extension-based parser routing | Inline checker is Rust-only | Use orchestrator for multi-lang |
| ArchitectureComplianceOrchestrator | Layer definition builder | `no_primitives` parsed but unused | Connect to coordinator |

## 10. Architecture Concept

### 10.1 Correct Data Flow
```
lint_arwaky.config.rust.yaml
    ↓ (parsing)
LayerDefinition { no_primitives: true/false }
    ↓ (wiring) ──── should be ────→ check_primitive_usage()
                                      ↓
                              SourceParserOrchestrator
                              ├── .rs → ASTRustScanner.find_primitive_violations(fields)
                              ├── .py → ASTPythonScanner.find_primitive_violations(attrs)
                              └── .js → ASTJSScanner.find_primitive_violations(props)
                                      ↓
                              LintResult (AES006 HIGH)
```

### 10.2 Current Gap
```
Config (no_primitives) ────???───→ Inline Checker (hardcoded, Rust only)
                                        ↓
                               DomainTypeRuleChecker (orphan, Python-focused)
                                        ↓
                               AST Scanners (unused for AES006)
                                        ↓
                               PythonPrimitiveChecker (empty stub)
```

## 11. Appendices
- `src-rust/agent/lint_checking_coordinator.rs:177` — Inline checker (ACTIVE)
- `src-rust/capabilities/domain_type_checker.rs:24` — Orphan capability (UNUSED)
- `src-rust/infrastructure/python_primitive_checker.rs` — Empty stub
- `src-rust/infrastructure/ast_rust_scanner.rs:368` — Rust scanner (UNUSED for AES006)
- `src-rust/infrastructure/ast_py_scanner.rs:394` — Python scanner (UNUSED for AES006)
- `src-rust/infrastructure/ast_js_scanner.rs:465` — JS scanner (UNUSED for AES006)
- `src-rust/infrastructure/source_parser_adapter.rs:80` — Orchestrator (UNUSED for AES006)
- `src-rust/contract/source_parser_port.rs:11` — ISourceParserPort::find_primitive_violations
- `src-rust/taxonomy/naming_symbols_constant.rs:10` — CORE_PRIMITIVE_TYPES
- `src-rust/taxonomy/layer_definition_vo.rs:33` — no_primitives field
- `src-rust/taxonomy/architecture_rule_vo.rs:25` — no_primitives in rule definition
- `lint_arwaky.config.rust.yaml:184-242` — Per-scope config
- `test-project-python/src-python/taxonomy/raw_entity.py` — Test fixture
- `test-project-python/src-python/taxonomy/raw_error.py` — Test fixture
- `test-project-rust/src-rust/taxonomy/massive_domain_entity.rs` — Test fixture
