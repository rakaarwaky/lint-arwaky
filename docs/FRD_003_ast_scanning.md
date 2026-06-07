# 📄 Feature Requirements Document (FRD)
**Feature Name:** Source Code Parsing (Rust + Python + JS/TS)  
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
This document defines the source code parsing system that extracts imports, symbols, and metrics from Rust, Python, and JavaScript/TypeScript files. It specifies the `ISourceParserPort` interface (17 methods), three language-specific implementations, and the composite `SourceParserOrchestrator` that routes by file extension.

### 2.2 Scope
**In-Scope:**
- 17 parser methods: import extraction, symbol detection, primitive violation detection, barrel detection, entry point detection, control flow counting
- Three language scanners: Rust, Python, JavaScript/TypeScript
- Composite orchestrator for extension-based routing
- Regex-based line-by-line parsing (NOT true AST parsers)

**Out-of-Scope:**
- True AST parsing (requires syn/swc/tree-sitter integration)
- Code formatting or transformation
- External tool invocation (ruff, clippy, eslint — handled by linter adapters)

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **ISourceParserPort** | Contract trait with 17 methods for source code analysis |
| **SourceParserOrchestrator** | Composite routes calls to language-specific parsers |
| **ASTRustParserAdapter** | Regex-based Rust line scanner |
| **ASTPythonParserAdapter** | Regex-based Python line scanner |
| **ASTJSParserAdapter** | Regex-based JS/TS line scanner |
| **Barrel file** | `mod.rs`, `__init__.py`, `index.ts` that exports all modules |

## 3. Feature Overview
### 3.1 Background & Problem
Before the centralized source parser, each linter adapter had its own inline parsing logic — import detection regex was duplicated across 10+ files. There was no uniform interface, so callers needed to know which parser to use for each language. Barrel file detection was inconsistent across tools.

### 3.2 Business Goals
- Provide a single interface for all source code analysis needs
- Eliminate duplicated regex patterns across the codebase
- Support 3 languages with consistent API
- Enable barrel/entry point detection for architectural rule enforcement

### 3.3 Target Users
- **Capability Checkers**: Consume parser output for AES rule enforcement
- **Linter Adapters**: Use parser for file-level analysis
- **Architecture Analyzer**: Layer detection and import validation

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As an import checker, I want to call `extract_imports(path)` once, so I can validate import rules without knowing which language the file is in.
- **US-002:** As a naming checker, I want to call `get_stem(path)` and `is_barrel_file(path)`, so I can validate naming conventions.
- **US-003:** As a metric checker, I want to call `get_control_flow_count()` and `get_function_definitions()`, so I can measure file complexity.

### 4.2 Use Cases & Workflow
**Extension-Based Routing:**
```
SourceParserOrchestrator.select_parser(path)
  ├── .rs   → ASTRustParserAdapter
  ├── .ts/.tsx/.js/.jsx → ASTJSParserAdapter
  └── .py (or unknown) → ASTPythonParserAdapter
```

**Import Extraction (Rust Example):**
```
Line: "use crate::taxonomy::FilePath;"
  ├── USE_REGEX: ^(?:pub\s+)?use\s+([^;]+);
  └── Match: "crate::taxonomy::FilePath" → ImportInfo

Line: "use std::collections::HashMap;"
  └── Match: "std::collections::HashMap" → ImportInfo
```

**Symbol Detection (Python Example):**
```
Line: "class LayerDetector:"
  ├── CLASS_REGEX: ^class\s+(\w+)
  └── Match: "LayerDetector" → class symbol

Line: "def detect_layer(self, path: str):"
  ├── DEF_REGEX: ^def\s+(\w+)\s*\(
  └── Match: "detect_layer" → function symbol
```

**Barrel Detection:**
```
File ends with:
  ├── /mod.rs OR /lib.rs → Rust barrel
  ├── /__init__.py → Python barrel
  └── /index.ts OR /index.js OR /index.tsx OR /index.jsx → JS/TS barrel
```

### 4.3 Business Rules
- 17 methods defined in `ISourceParserPort` trait
- Each language adapter implements all 17 methods
- Orchestrator routes by file extension, not content inspection
- Unknown extensions fall back to Python parser

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Parse latency per file | < 50ms |
| NFR-002 | Accuracy for simple patterns | > 90% |
| NFR-003 | Accuracy for complex patterns (generics, macros) | N/A (not supported) |
| NFR-004 | Regex compilation | Static `LazyLock` initialization |

## 6. UI/UX Requirements
No direct UI. Parser is consumed by other components internally.

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | A Rust file with `use crate::taxonomy::...` | `extract_imports()` runs | ImportInfo with path "taxonomy" | ✅ Simple cases work; ⚠️ fails on multi-line/group imports |
| AC-002 | A Python file with `from taxonomy import FilePath` | `extract_imports()` runs | ImportInfo with source "taxonomy" | ✅ Simple cases work; ⚠️ fails on multi-line parenthesized imports |
| AC-003 | A JS file with `import { X } from './module'` | `extract_imports()` runs | ImportInfo with path "./module" | ✅ Simple cases work; ❌ JS `exported` set never populated |
| AC-004 | File `src-rust/capabilities/mod.rs` | `is_barrel_file()` runs | Returns true | ✅ Path-based, not regex |
| AC-005 | File `src-rust/capabilities/__init__.py` | `is_barrel_file()` runs | Returns true | ✅ Path-based |
| AC-006 | File `src-rust/surfaces/index.ts` | `is_barrel_file()` runs | Returns true | ✅ Path-based |
| AC-007 | File `mod.rs` containing `fn main` | `is_entry_point()` runs | Returns true | ✅ Simple keyword match |
| AC-008 | File `cli_main_entry.rs` | `is_entry_point()` runs | Returns true | ✅ Name-based check |
| AC-009 | Any file with class attributes (fields) | `get_class_attributes()` runs | Returns structured attribute data | ❌ STUB — returns empty HashMap in all 3 scanners |
| AC-010 | JS file with `export function foo()` | `is_symbol_exported()` runs | Returns true | ❌ BROKEN — JS `exported` set never populated |

## 8. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| regex crate | All parsing is regex-based | Regex fails on complex patterns | Document limitations; plan syn/swc integration |
| Static LazyLock | Regex compiled once at startup | Memory overhead | Negligible (~50KB for all patterns) |
| Multi-language support | Three parser implementations | Maintenance burden | Shared orchestration layer reduces duplication |

## 9. Appendices
- `src-rust/contract/source_parser_port.rs` — ISourceParserPort trait (17 methods)
- `src-rust/infrastructure/ast_rust_scanner.rs` — Rust scanner (518 lines)
- `src-rust/infrastructure/ast_py_scanner.rs` — Python scanner (569 lines)
- `src-rust/infrastructure/ast_js_scanner.rs` — JS/TS scanner (603 lines)
- `src-rust/infrastructure/source_parser_adapter.rs` — Orchestrator (143 lines)
- `src-rust/agent/dependency_injection_container.rs` — Wiring
