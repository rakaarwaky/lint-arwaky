# Feature Requirements Document (FRD)
**Feature Name:** Capability Method Existence Checker (AES030)  
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
This document defines the AES030 rule that detects capability method references in a dispatch catalog or command map that do not exist on the target class. The rule is implemented in `_verify_capability_references()` within `DispatchRoutingChecker` in `cli-transport/capabilities_routing_processor.rs`. AES030 ensures that every method name referenced in `COMMAND_CATALOG` or dispatch routing configuration matches an actual method defined on the corresponding capability struct/class.

### 2.2 Scope
**In-Scope:**
- Static analysis of capability references parsed from dispatch/routing configuration
- Method name verification against class definitions extracted from scanned files
- Support for both Python dispatch catalogs (dict-based COMMAND_CATALOG) and Rust trait-based dispatch routing
- Cross-reference detection: method referenced but class not found in any scanned file
- MEDIUM severity reporting

**Out-of-Scope:**
- Runtime dispatch validation (static analysis only)
- Dynamic method resolution (e.g., `getattr`, `__call__`, method_missing patterns)
- Auto-fixing catalog entries

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **AES030** | Rule code for capability method existence violation |
| **DispatchRoutingChecker** | Main checker struct in `capabilities_routing_processor.rs` |
| **COMMAND_CATALOG** | Dictionary or map that routes action names to capability class.method pairs |
| **CapabilityReference** | Parsed `(class_name, method_name)` pair from routing configuration |
| **ClassDefinitionMap** | Map of class names to their defined method lists |

## 3. Feature Overview
### 3.1 Background & Problem
Before AES030, there was no verification that method names referenced in dispatch catalogs actually existed on the target capability classes. A typo in a catalog entry (e.g., `"UserManager.processOrder"` when the method is `process_order`) would not be caught until runtime, causing a 500 error in production. As the system grows to hundreds of capability classes and thousands of dispatch routes, manual verification becomes impractical.

### 3.2 Business Goals
- Eliminate runtime dispatch failures caused by method name mismatches
- Enforce a single source of truth: the capability class definition
- Provide actionable messages that suggest the correct method name
- Support both Python dict-based and Rust trait-based dispatch patterns

### 3.3 Target Users
- **Developers**: Get automatic feedback on catalog-to-class method mismatches before deployment
- **Architects**: Enforce that all dispatch routes reference valid capability methods at lint time

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer, I want to be warned when a COMMAND_CATALOG entry references a method that doesn't exist on the capability class.
- **US-002:** As a developer, I want the violation message to list the actual methods defined on the class so I can pick the correct name.
- **US-003:** As a developer, I want to be warned when a COMMAND_CATALOG references a capability class that doesn't exist in the codebase.

### 4.2 Use Cases & Workflow
**Detection Pipeline:**
```
File: agent/dispatch_orchestrator.py (contains COMMAND_CATALOG)

1. Scan all Python files for class definitions → extract method lists
2. Parse COMMAND_CATALOG entries via regex:
   r#"["']capability["']\s*:\s*["']([A-Za-z_]\w*)\.([A-Za-z_]\w*)["']"#
3. For each (class_name, method_name) pair:
   a. Is class_name in ClassDefinitionMap?
   b. If YES → Is method_name in the class's method list?
      - NO → AES030 MEDIUM: "Method 'processOrder' not found on class 'UserManager'"
   c. If NO → AES030 MEDIUM: "Capability class 'UserManager' not found in any scanned file"
```

### 4.3 Business Rules
- Severity: MEDIUM
- Detection is cross-file: catalog entries in one file, class definitions in another
- Only `.py` files are scanned for class definitions (Python dispatch catalogs)
- Regex pattern for capability references: `"capability": "ClassName.methodName"`
- If a class has zero methods defined, reports "(none)" in the message
- Empty capability refs list skips the check entirely

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Detection per project | < 100ms for 1000 files |
| NFR-002 | False positive rate | 0% for valid catalogs |
| NFR-003 | False negative rate | 0% for mismatched names |

## 6. UI/UX Requirements
```
AES030 MEDIUM - src-rust/cli-transport/agent/dispatch_orchestrator.py
  AES030 CAPABILITY_METHOD_NOT_FOUND: Method 'processOrder' not found on class 'UserManager'. Defined methods: process_order, cancel_order, get_history.
  WHY? Mismatched method names between dispatch catalog and capability class cause runtime dispatch failures.
  FIX: Sync the method name in the dispatch catalog with the actual method defined on the capability struct/trait.

AES030 MEDIUM - src-rust/cli-transport/agent/dispatch_orchestrator.py
  AES030 CAPABILITY_METHOD_NOT_FOUND: Capability class 'UnknownHandler' not found in any scanned file. Referenced from COMMAND_CATALOG but no class definition exists.
  WHY? A referenced capability class does not exist in the codebase.
  FIX: Create the capability class or remove the reference from the dispatch catalog.
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Catalog references "UserManager.processOrder" but class has "process_order" | `_verify_capability_references()` runs | AES030 MEDIUM flagged | Pending Review |
| AC-002 | Catalog references "UserManager" class that doesn't exist | `_verify_capability_references()` runs | AES030 MEDIUM flagged (class not found) | Pending Review |
| AC-003 | Catalog references "UserManager.process_order" and method exists | `_verify_capability_references()` runs | No AES030 | Pending Review |
| AC-004 | No capability references exist | `verify_capability_references()` runs | Skipped | Pending Review |
| AC-005 | Python class with zero methods is referenced | Verify checker runs | Reports "(none)" in message | Pending Review |

## 8. Empirical Findings (Code Audit)

### 8.1 Current Implementation
- **Location**: `src-rust/cli-transport/capabilities_routing_processor.rs:223-265`
- **AES030 portion**: `routing_processor.rs:223-265` — `_verify_capability_references()`
- **Status**: **FULLY IMPLEMENTED** — not a stub
- Parses capability refs via regex `CAPABILITY_REF_PATTERN` (line 91-93) from Python files only
- Class methods extracted via `extract_class_methods()` (line 61-88) using regex `class\s+(\w+)` and `def\s+(\w+)\s*\(`
- Invoked from `check_capability_routing()` (line 119)

### 8.2 Bugs Found

1. **Only supports Python files** (`routing_processor.rs:154`)
   - `if !path.ends_with(".py") { continue; }`
   - Rust dispatch catalogs (trait-based, struct-based) are NOT scanned
   - **Impact**: AES030 does not check Rust dispatch routing at all
   - **Fix**: Add Rust file scanning with `impl TraitName for StructName` pattern

2. **Heuristic regex may miss catalog entries** (`routing_processor.rs:91-93`)
   - `CAPABILITY_REF_PATTERN` only matches `"capability": "ClassName.methodName"` format
   - Does NOT match YAML-based catalogs, inline dict definitions with single quotes mixed, or TOML config
   - **Impact**: some catalog formats bypass detection entirely
   - **Fix**: support additional patterns or parse config files directly

3. **Duplicate AES030 checker in layer-rules** (`capabilities_layer_checker.rs:62-74`)
   - `ArchLayerChecker` in `layer-rules/` also emits AES030 for structs without trait impls
   - This is a DIFFERENT definition: it checks `struct Foo` without `impl IFoo`
   - Two different checks under the SAME rule code → confusion
   - **Impact**: rule semantics are overloaded (dispatch method vs. trait impl existence)

4. **`_verify_capability_references` not invoked for Rust projects**
   - The pipeline in `check_capability_routing()` (line 110) only reaches `_verify_capability_references` via the async path
   - The `IDispatchRoutingProtocol` implementation (line 459-467) is a `todo!()` stub
   - **Impact**: the only active path is through `DispatchRoutingChecker::check_capability_routing()` which requires an `IAnalyzer`
   - **Fix**: wire the sync `IDispatchRoutingProtocol` trait or remove it

### 8.3 What Needs to Be Added
- **Rust file support**: extend `_check_capability_by_layer` to scan `.rs` files for struct/trait definitions
- **Multiple catalog format support**: add parsers for YAML, inline dict, and TOML dispatch configs
- **Unify AES030 semantics**: decide whether AES030 means "dispatch method not found" OR "struct without trait impl" — one must be renamed
- **Integration test fixtures**: create a Python test file with a COMMAND_CATALOG containing a wrong method name

### 8.4 What to Keep
- **Class definition extraction** Pending Review (Python regex-based, works for Python-only projects)
- **Class-not-found detection** Pending Review (line 253-264, useful for missing imports)
- **Method list in violation message** Pending Review (line 248-251, actionable DX)
- **Bottleneck + missing-VO integration** Pending Review (same pipeline)

### 8.5 Empirical Evidence from Test Projects
- `test-project-python/` — No existing COMMAND_CATALOG fixture found
- `test-project-rust/` — No existing dispatch routing fixture found
- **No test fixture exercises AES030** — needs to be created

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-003 (Source Parsing) | Class definition extraction | Python-only regex heuristic | Extend to Rust |
| Config YAML | Severity/MEDIUM definition | Rule severity not configurable per project | Add `severity` field for AES030 |
| Test fixtures | COMMAND_CATALOG with wrong method | No fixture exists | Create Python dispatch fixture |
| capabilities_layer_checker | Duplicate AES030 definition | Rule semantic confusion | Unify or rename one |

## 10. Appendices
- `src-rust/cli-transport/capabilities_routing_processor.rs:223` — `_verify_capability_references()`
- `src-rust/cli-transport/capabilities_routing_processor.rs:91` — `CAPABILITY_REF_PATTERN`
- `src-rust/cli-transport/capabilities_routing_processor.rs:61` — `extract_class_methods()`
- `src-rust/layer-rules/capabilities_layer_checker.rs:62` — Duplicate AES030 (struct+trait impl)
- `src-rust/code-analysis/agent_checking_coordinator.rs:30` — Import path
- `lint_arwaky.config.rust.yaml:256` — AES030 config message
