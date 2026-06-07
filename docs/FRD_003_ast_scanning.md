# FRD — AST Scanning for Rust, Python, JavaScript/TypeScript

> **PRD Reference**: [FR-003](PRD.md) — AST scanning for Rust, Python, JavaScript/TypeScript
> **Dependency**: FR-001 (6-layer AES architecture)
> **Status**: ⚠️ **BASIC / NOT PRODUCTION-READY** — Regex-based line scanners, NOT true AST parsers. Missing `syn` (Rust), `ast` (Python), `swc`/`tree-sitter` (JS/TS) integration.
> **Self-lint**: `lint-arwaky-cli check .` — scanners work for basic patterns but miss complex cases

## 1. Problem Statement

Before the multi-language AST scanning system, Lint Arwaky had:

| Issue | Description |
|-------|-------------|
| **No source parsing abstraction** | Each linter adapter parsed source files ad-hoc with inline logic |
| **Language-specific logic scattered** | Import extraction, symbol detection, and primitive checks lived in capability files, not infrastructure |
| **No unified parser interface** | Callers had to know which parser to use for each language |
| **Regex patterns duplicated** | Similar regex logic for import detection repeated across adapters |
| **No barrel file detection** | Each tool had its own idea of what a "barrel" file was |
| **No entry point detection** | No unified way to identify `main.rs`, `__init__.py`, `index.ts` |

## 2. Solution Overview

The source parsing system defines a single contract port (`ISourceParserPort`) with 17 methods, implemented by three language-specific **regex-based line scanners** (not true AST parsers) and one composite orchestrator.

| Layer | New/Changed | Responsibility |
|-------|-------------|----------------|
| **Taxonomy** | New VOs + errors | Source content, analysis results, paths, naming symbols, parser errors |
| **Contract** | New port + traits | `ISourceParserPort` — 17 methods covering imports, symbols, primitives, barrel, entry points |
| **Infrastructure** | 4 new files | `ASTRustParserAdapter`, `ASTPythonParserAdapter`, `ASTJSParserAdapter`, `SourceParserOrchestrator` |
| **Agent** | Wiring only | DI container composites all 3 parsers into `SourceParserOrchestrator` |
| **Surfaces** | Indirect | Surfaces access parsers via `ServiceContainerAggregate` only |

### ⚠️ Important Limitation

These are **NOT true AST parsers**. They are **regex-based line-by-line scanners** that:
- Process one line at a time (no multi-line syntax understanding)
- Cannot parse nested generics (`Result<Vec<String>>`)
- Cannot parse multi-line statements
- Cannot parse macros (`vec![]`, `println!`)
- Cannot handle attributes properly (`#[derive(...)]`)
- Produce false positives from string literals containing code patterns
- Have no understanding of scope nesting beyond brace counting

**For production use**, these should be replaced with proper parsers: `syn`/`proc_macro2` for Rust, `python3 -c "import ast"` for Python, `swc` or `tree-sitter` for JS/TS.

### Architecture Flow

```
Capability/Checker
    │
    ▼
ISourceParserPort (contract/source_parser_port.rs)
    │
    ▼
SourceParserOrchestrator (infrastructure/source_parser_adapter.rs)
    │
    ├──► .rs  → RegexRustLineScanner (infrastructure/ast_rust_scanner.rs)
    ├──► .py  → RegexPythonLineScanner (infrastructure/ast_py_scanner.rs)
    └──► .js/.ts/.jsx/.tsx → RegexJSLineScanner (infrastructure/ast_js_scanner.rs)
    │
    ▼
Taxonomy VOs (ImportInfoList, MetadataVO, PrimitiveViolationList, etc.)
```

### Parser Interface (17 Methods)

| # | Method | Returns | Purpose |
|---|--------|---------|---------|
| 1 | `extract_imports` | `Result<ImportInfoList, SourceParserError>` | Extract all import statements |
| 2 | `get_raw_symbols` | `Result<ResponseData, SourceParserError>` | Extract all symbols (classes, fns, vars) |
| 3 | `get_class_attributes` | `ResponseData` | List attributes of classes |
| 4 | `has_all_export` | `SuccessStatus` | Check if file has `__all__` / `pub use *` |
| 5 | `find_primitive_violations` | `PrimitiveViolationList` | Find raw primitives in domain types |
| 6 | `find_unused_imports` | `ImportInfoList` | Detect unused imports |
| 7 | `get_class_definitions` | `Result<MetadataVO, SourceParserError>` | Extract class definitions |
| 8 | `get_function_definitions` | `MetadataVO` | Extract function definitions |
| 9 | `is_symbol_exported` | `SuccessStatus` | Check if a symbol is publicly exported |
| 10 | `get_class_methods` | `MetadataVO` | Extract methods of a class |
| 11 | `get_class_bases_map` | `MetadataVO` | Extract base class inheritance map |
| 12 | `get_assignment_targets` | `MetadataVO` | Extract variable assignment targets |
| 13 | `get_control_flow_count` | `Count` | Count control flow statements |
| 14 | `is_barrel_file` | `BooleanVO` | Check if file is a barrel (`mod.rs`, `__init__.py`, `index.ts`) |
| 15 | `get_stem` | `SymbolName` | Get filename without extension |
| 16 | `is_entry_point` | `BooleanVO` | Check if file is an entry point |
| 17 | `get_supported_extensions` | `PatternList` | List supported file extensions |

## 3. Taxonomy Changes

### Key Types Used by Parser

| Type | File | Description |
|------|------|-------------|
| `ImportInfoList` | `taxonomy/import_info_vo.rs` | List of import statements |
| `MetadataVO` | `taxonomy/metadata_vo.rs` | Generic key-value metadata |
| `PrimitiveViolationList` | `taxonomy/primitive_violation_vo.rs` | Primitive type violations |
| `ResponseData` | `taxonomy/response_data_vo.rs` | Generic symbol data |
| `BooleanVO` | `taxonomy/boolean_vo.rs` | Typed boolean wrapper |
| `SuccessStatus` | `taxonomy/success_status_vo.rs` | Success/failure status |
| `Count` | `taxonomy/count_vo.rs` | Typed integer count |
| `SymbolName` | `taxonomy/symbol_name_vo.rs` | Symbol name wrapper |
| `PatternList` | `taxonomy/pattern_list_vo.rs` | List of glob/file patterns |
| `SourceParserError` | `taxonomy/source_parser_error.rs` | Parser error types |

### Existing VOs Kept Unchanged

- `FilePath` — file path wrapper
- `DirectoryPath` — directory path wrapper

## 4. Contract Changes

### NEW: `ISourceParserPort`

File: `contract/source_parser_port.rs` (28 lines)

```rust
pub trait ISourceParserPort: Send + Sync {
    fn extract_imports(&self, path: &FilePath) -> Result<ImportInfoList, SourceParserError>;
    fn get_raw_symbols(&self, path: &FilePath) -> Result<ResponseData, SourceParserError>;
    fn get_class_attributes(&self, path: &FilePath) -> ResponseData;
    fn has_all_export(&self, path: &FilePath) -> SuccessStatus;
    fn find_primitive_violations(&self, path: &FilePath, primitive_types: &PrimitiveTypeList) -> PrimitiveViolationList;
    fn find_unused_imports(&self, path: &FilePath) -> ImportInfoList;
    fn get_class_definitions(&self, path: &FilePath) -> Result<MetadataVO, SourceParserError>;
    fn get_function_definitions(&self, path: &FilePath) -> MetadataVO;
    fn is_symbol_exported(&self, path: &FilePath, symbol: &SymbolName) -> SuccessStatus;
    fn get_class_methods(&self, path: &FilePath) -> MetadataVO;
    fn get_class_bases_map(&self, path: &FilePath) -> MetadataVO;
    fn get_assignment_targets(&self, path: &FilePath) -> MetadataVO;
    fn get_control_flow_count(&self, path: &FilePath) -> Count;
    fn is_barrel_file(&self, path: &FilePath) -> BooleanVO;
    fn get_stem(&self, path: &FilePath) -> SymbolName;
    fn is_entry_point(&self, path: &FilePath) -> BooleanVO;
    fn get_supported_extensions(&self) -> PatternList;
}
```

## 5. Infrastructure Changes

### ⚠️ `ASTRustParserAdapter` — Regex Line Scanner

File: `infrastructure/ast_rust_scanner.rs` (518 lines)

**Regex-based line scanner, NOT a true Rust AST parser.** Uses `syn`/`proc_macro2` equivalents? **No.** Simply walks lines with regex patterns.

Known limitations:
- Cannot parse `use std::collections::{HashMap, VecDeque};` — group imports in braces
- Cannot parse nested generics: `Result<Vec<String>>`
- Cannot `impl<T: Clone + Debug> Foo<T>` — complex generics
- Misses multi-line use statements spanning multiple lines
- Cannot parse attribute macros: `#[derive(Debug, Clone, Serialize)]`
- String literals containing `fn main()` produce false positives

What it handles:
- Simple `use crate::foo::bar;` patterns
- `struct Foo`, `enum Bar`, `trait Baz` on single line
- Simple `fn foo() {}` definitions
- `impl Foo for Bar` on single line
- Basic `if/for/while/match/loop` keyword counting
- `/mod.rs` and `/lib.rs` barrel detection
- `fn main` and `fn main_entry` entry point detection

### ⚠️ `ASTPythonParserAdapter` — Regex Line Scanner

File: `infrastructure/ast_py_scanner.rs` (569 lines)

**Regex-based line scanner, NOT using Python `ast` module.** Could call `python3 -c "import ast..."` but does not.

Known limitations:
- Cannot parse indented `from x import (\n    y,\n    z\n)` — multi-line imports
- Cannot parse f-strings or string literals with code patterns
- Indentation-based scope is approximated with brace-like counting, not Python's indentation rules
- Nested classes/functions produce wrong parent tracking
- Cannot parse decorators `@dataclass` properly
- Missing `match/case` (Python 3.10+) pattern matching

What it handles:
- `import X` and simple `from X import Y` statements
- `class X(Base):` single-line class definitions
- `def func():` function start detection
- `__all__` export list detection
- Type annotation `: int`, `-> str` detection on assignments
- `__init__.py` barrel detection
- Basic `if/for/while/try/except/with/async for` counting

### ⚠️ `ASTJSParserAdapter` — Regex Line Scanner

File: `infrastructure/ast_js_scanner.rs` (603 lines)

**Regex-based line scanner, NOT using `swc`, `tree-sitter`, or `@babel/parser`.**

Known limitations:
- Cannot parse `import { Foo as Bar, Baz } from 'module'` — complex destructured imports
- Cannot parse TypeScript generics: `function foo<T extends Bar>(x: T): T`
- Cannot parse JSX/TSX properly — treats `<Foo>` as comparison operator
- Template literals `` `import ${x} from 'y'` `` produce false positives
- Arrow functions `const foo = () => {}` not detected as functions
- Cannot handle `export default class` properly
- Dynamic imports `import('module')` are missed

What it handles:
- Simple `import X from 'module'` with single/double quotes
- Simple `const X = require('module')` CommonJS patterns
- `class Foo extends Bar` single-line class with base
- `function foo()` traditional function declarations
- Block comments `/* */` skipping
- `/index.ts`, `/index.js`, `/index.tsx`, `/index.jsx` barrel detection
- Basic `if/for/while/switch/catch` keyword counting

### `SourceParserOrchestrator`

File: `infrastructure/source_parser_adapter.rs` (143 lines)

Composite pattern routing by file extension:
- `.rs` → `ASTRustParserAdapter` (regex scanner)
- `.py` → `ASTPythonParserAdapter` (regex scanner)
- `.js/.ts/.jsx/.tsx` → `ASTJSParserAdapter` (regex scanner)
- Unknown → Python (fallback)

All 17 `ISourceParserPort` methods delegated via `select_parser()`. Quality limited by underlying regex scanners.

## 6. Capability Changes

No new capability files. Existing checkers consume `ISourceParserPort` via DI:

| Checker | Uses Parser For |
|---------|----------------|
| `architecture_import_checker.rs` | `extract_imports`, `is_barrel_file` |
| `architecture_naming_checker.rs` | `get_raw_symbols`, `get_stem` |
| `architecture_internal_checker.rs` | `has_all_export`, `is_symbol_exported` |
| `architecture_metric_checker.rs` | `get_control_flow_count`, `get_function_definitions` |
| `architecture_compliance_analyzer.rs` | `is_entry_point` |
| `surface_hierarchy_checker.rs` | `get_class_definitions`, `get_function_definitions` |
| `architecture_cycle_analyzer.rs` | `extract_imports` |
| `architecture_orphan_analyzer.rs` | `get_raw_symbols` |

## 7. Agent Changes

### Updated: `DependencyInjectionContainer`

File: `agent/dependency_injection_container.rs`

```rust
let rust_parser = Arc::new(ASTRustParserAdapter::new());
let python_parser = Arc::new(ASTPythonParserAdapter::new());
let js_parser = Arc::new(ASTJSParserAdapter::new());

let source_parser: Arc<dyn ISourceParserPort> = Arc::new(
    SourceParserOrchestrator::new(
        Box::new(python_parser),
        Box::new(rust_parser),
        Box::new(js_parser),
    ),
);
```

## 8. Surface Changes

No surface changes. All parser access through `ServiceContainerAggregate`.

## 9. Files Summary

### New Files (4 Infrastructure, 1 Contract)

| File | Layer | Lines |
|------|-------|-------|
| `contract/source_parser_port.rs` | Contract | 28 |
| `infrastructure/ast_rust_scanner.rs` | Infrastructure | 518 |
| `infrastructure/ast_py_scanner.rs` | Infrastructure | 569 |
| `infrastructure/ast_js_scanner.rs` | Infrastructure | 603 |
| `infrastructure/source_parser_adapter.rs` | Infrastructure | 143 |

### Modified Files (1)

| File | Change |
|------|--------|
| `agent/dependency_injection_container.rs` | Wire 3 parsers + `SourceParserOrchestrator` |

## 10. AES Compliance

| Rule | Compliance |
|------|------------|
| AES001 | Infrastructure parsers import only taxonomy/contract — no capability/agent/surface imports |
| AES002 | Each parser implements `ISourceParserPort` mandatory contract |
| AES003 | Filenames: `ast_rust_scanner`, `ast_py_scanner`, `ast_js_scanner`, `source_parser_adapter` — 3-word pattern |
| AES008 | Contract files use `_port` suffix |
| AES011 | Infrastructure files use `_scanner`, `_adapter` — allowed per layer |
| AES012 | Barrel re-exports required in `infrastructure/mod.rs` |
| AES027 | Every logic file implements a contract trait (`ISourceParserPort`) |
| AES022 | Parser adapters are passive infrastructure — no surface imports |
| AES023 | Surfaces access parsers only through `ServiceContainerAggregate` |

## 11. Implementation Order

1. **Taxonomy**: Define parser-related types (`ImportInfoList`, `MetadataVO`, `PrimitiveViolationList`, `SourceParserError`)
2. **Contract**: Create `ISourceParserPort` with 17 methods
3. **Infrastructure**: Implement `ASTRustParserAdapter`, `ASTPythonParserAdapter`, `ASTJSParserAdapter`
4. **Infrastructure**: Implement `SourceParserOrchestrator` composite
5. **Agent**: Wire parsers in `DependencyInjectionContainer`
6. **Test**: Verify all 17 methods work for each language

## 12. Acceptance Criteria

| # | Kriteria | Status |
|---|----------|--------|
| AC001 | `extract_imports` correctly parses `use`, `import`, `require` for all 3 languages | ⚠️ Partial — fails on multi-line, group imports, generics |
| AC002 | `is_barrel_file` detects `mod.rs`, `__init__.py`, `index.ts` correctly | ✅ Works (path-based, not regex) |
| AC003 | `find_primitive_violations` catches raw primitives in domain types | ⚠️ Partial — regex false positives from strings |
| AC004 | `SourceParserOrchestrator` routes `.rs` → Rust, `.py` → Python, `.js/.ts` → JS | ✅ Works (extension-based routing) |
| AC005 | `is_entry_point` detects `main.rs`, `__main__`, entry scripts | ⚠️ Partial — keyword matching on single line |
| AC006 | `get_control_flow_count` returns accurate count for all languages | ⚠️ Partial — counts keywords, not actual control flow paths |
| AC007 | `get_class_bases_map` extracts inheritance hierarchy for Python/JS | ⚠️ Partial — single inheritance only, fails on complex bases |
| AC008 | Barrel + entry point detection works for all 3 language conventions | ✅ Works (path-based) |
| AC009 | True AST parsing via `syn` (Rust), `ast` (Python), `swc`/`tree-sitter` (JS/TS) | ❌ Missing — all scanners are regex-based, not AST |
| AC010 | Production-ready: no regex false positives from string literals | ❌ Missing — string contents are not filtered |
| AC011 | `cargo check --bin lint-arwaky-cli` lulus tanpa error baru | ✅ |
| AC012 | `cargo test` passes | ✅ |
