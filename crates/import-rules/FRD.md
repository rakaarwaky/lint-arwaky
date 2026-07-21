# FRD — import-rules

## System Overview

The import-rules crate enforces correct structural boundaries and unidirectional dependency flows. It prevents spaghetti architecture, circular dependencies, and dead/unused imports by validating every import statement against a predefined layer-hierarchy matrix.

## Functional Requirements

### FR-001: Layer Dependency Violation (AES201)

- **Description**: Restricts imports based on the layer hierarchy. Lower layers must never import higher layers.
- **Input**: Rust, Python, JS/TS source files
- **Output**: List of layer violation diagnostics
- **Business Rules**:
  - taxonomy_ must not import contract_, utility_, capabilities_, agent_, surface_, root_
  - contract_ must not import utility_, capabilities_, agent_, surface_, root_
  - utility_ and capabilities_ must not import each other directly
- **Edge Cases**: Circular imports across layers
- **Error Handling**: Emit AES201 diagnostic with file path and line number

### FR-002: Mandatory Layer Imports (AES202)

- **Description**: Verifies that specific layers contain required imports.
- **Input**: Rust, Python, JS/TS source files
- **Output**: List of missing import diagnostics
- **Business Rules**:
  - Capability files must import their corresponding contract trait
  - Surface entries must import their container
- **Edge Cases**: Files with multiple roles
- **Error Handling**: Emit AES202 diagnostic with expected import

### FR-003: Unused Import Detection (AES203)

- **Description**: Detects and flags imported symbols that are never referenced within the file body.
- **Input**: Rust, Python, JS/TS source files
- **Output**: List of unused import diagnostics
- **Business Rules**:
  - Symbol must be referenced at least once after import
  - Wildcard imports (`use foo::*`) are flagged
- **Edge Cases**: Re-exports, pub use statements
- **Error Handling**: Emit AES203 diagnostic with unused symbol name

### FR-004: Dummy or Forbidden Imports (AES204)

- **Description**: Detects imports that point to mock, dummy, or forbidden packages/modules.
- **Input**: Rust, Python, JS/TS source files
- **Output**: List of forbidden import diagnostics
- **Business Rules**:
  - Import path must not match forbidden patterns
  - Test-only imports in production code are flagged
- **Edge Cases**: Conditional imports, feature flags
- **Error Handling**: Emit AES204 diagnostic with forbidden import path

### FR-005: Circular Dependency Detection (AES205)

- **Description**: Builds a dependency graph of imports across all workspace files and detects cycles.
- **Input**: All workspace source files
- **Output**: List of circular dependency diagnostics
- **Business Rules**:
  - Direct cycles (A → B → A) are flagged
  - Indirect cycles (A → B → C → A) are flagged
- **Edge Cases**: Self-imports, conditional cycles
- **Error Handling**: Emit AES205 diagnostic with cycle path

## Data Model / Entity Relationship

```
ImportRuleVO {
    pattern: String
    message: String
    severity: Severity
}

LayerHierarchy {
    taxonomy: Vec<String>
    contract: Vec<String>
    utility: Vec<String>
    capabilities: Vec<String>
    agent: Vec<String>
    surface: Vec<String>
    root: Vec<String>
}
```

## API Contract

| Function | Input | Output | Description |
|----------|-------|--------|-------------|
| `check_layer_violation()` | File path, imports | Vec<Diagnostic> | Check AES201 |
| `check_mandatory_imports()` | File path, imports | Vec<Diagnostic> | Check AES202 |
| `check_unused_imports()` | File path, imports, usages | Vec<Diagnostic> | Check AES203 |
| `check_dummy_imports()` | File path, imports | Vec<Diagnostic> | Check AES204 |
| `check_circular_deps()` | All files, imports | Vec<Diagnostic> | Check AES205 |

## Integration Points

- **Internal**: config-system (YAML rules), code-analysis (file reading)
- **External**: None

## Non-functional Requirements (Detailed)

- Performance: Check 1000 files in < 2 seconds
- Memory: O(n) where n = number of imports
- Accuracy: Zero false positives for valid imports

## Test Scenarios / QA Checklist

- [ ] Valid unidirectional import passes
- [ ] Cross-layer import fails with AES201
- [ ] Unused import detected with AES203
- [ ] Circular dependency detected with AES205
- [ ] Dummy import detected with AES204

## Assumptions & Constraints

- Layer hierarchy is defined in config YAML
- Import statements are parsed via regex (not AST)
- Workspace structure follows AES conventions

## Glossary

- **AES**: Agentic Engineering System
- **Layer**: Architectural boundary (taxonomy, contract, utility, capabilities, agent, surface, root)
- **Diagnostic**: Violation report with file path, line, column, rule code

## Reference

- PRD: [PRD.md](../../PRD.md)
