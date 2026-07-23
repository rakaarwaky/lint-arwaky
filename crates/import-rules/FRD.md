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

### FR-004a: Dummy Import Detection (AES204)

- **Description**: Detects imports, functions, and trait implementations that are dummy/stub code existing only to suppress unused-import warnings.
- **Input**: Rust, Python, JS/TS source files
- **Output**: List of dummy import diagnostics
- **Business Rules**:
  - Imported symbols placed inside `_use_*` dummy functions are flagged
  - Dummy functions (`fn _use_*`, `def _use_*`, `function _use*`) are flagged
  - Trait implementations with empty bodies are flagged
  - Taxonomy VOs used only in dummy functions (not in real logic) are flagged
- **Edge Cases**: Re-exports, pub use statements
- **Error Handling**: Emit AES204 diagnostic with dummy symbol name and line number

### FR-004b: Forbidden Import Detection (AES201)

- **Description**: Detects imports that violate layer boundary rules defined in YAML configuration.
- **Input**: Rust, Python, JS/TS source files
- **Output**: List of forbidden import diagnostics
- **Business Rules**:
  - Import path must not match forbidden patterns from config
  - Layer-specific forbidden rules are enforced per scope pattern
- **Edge Cases**: Conditional imports, feature flags
- **Error Handling**: Emit AES201 diagnostic with forbidden import path

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
Import rule (value object)
  - pattern: string
  - message: string
  - severity: severity level

Layer hierarchy
  - taxonomy: list of strings
  - contract: list of strings
  - utility: list of strings
  - capabilities: list of strings
  - agent: list of strings
  - surface: list of strings
  - root: list of strings
```

## API Contract

| Function                           | Input                      | Output          | Description  |
| ---------------------------------- | -------------------------- | --------------- | ------------ |
| Check for forbidden imports        | File path, imports         | Vec<Diagnostic> | Check AES201 |
| Check for mandatory imports        | File path, imports         | Vec<Diagnostic> | Check AES202 |
| Check for unused imports           | File path, imports, usages | Vec<Diagnostic> | Check AES203 |
| Check for dummy imports            | File path, imports         | Vec<Diagnostic> | Check AES204 |
| Check for circular dependencies    | All files, imports         | Vec<Diagnostic> | Check AES205 |

## Integration Points

- **Internal**: config-system (YAML rules), shared/common (utility functions for file reading, layer detection, import parsing)
- **External**: None

## Non-functional Requirements (Detailed)

- Performance: Check 1000 files in < 2 seconds (validated via criterion benchmark in the benchmark suite)
- Memory: O(n) where n = number of imports
- Accuracy: Zero false positives for valid imports

## Test Scenarios / QA Checklist

### AES201 — Forbidden Import

| Test Case                   | Input                                                      | Expected Output           |
| --------------------------- | ---------------------------------------------------------- | ------------------------- |
| taxonomy imports contract   | `taxonomy_vo.rs` with `use contract_protocol::*`           | AES201 CRITICAL violation |
| capabilities imports agent  | `capabilities_checker.rs` with `use agent_orchestrator::*` | AES201 CRITICAL violation |
| valid unidirectional import | `capabilities_checker.rs` with `use taxonomy_vo::*`        | No violation              |

### AES202 — Mandatory Import

| Test Case               | Input                                             | Expected Output       |
| ----------------------- | ------------------------------------------------- | --------------------- |
| missing contract import | `capabilities_checker.rs` without protocol import | AES202 HIGH violation |
| present contract import | `capabilities_checker.rs` with protocol import    | No violation          |

### AES203 — Unused Import

| Test Case     | Input                                          | Expected Output         |
| ------------- | ---------------------------------------------- | ----------------------- |
| unused symbol | File with `use foo::Bar;` but `Bar` never used | AES203 MEDIUM violation |
| used symbol   | File with `use foo::Bar;` and `Bar` referenced | No violation            |

### AES204 — Dummy Import

| Test Case        | Input                                | Expected Output       |
| ---------------- | ------------------------------------ | --------------------- |
| dummy function   | File with `fn _use_imports() {}`     | AES204 HIGH violation |
| empty trait impl | File with `impl Trait for Struct {}` | AES204 HIGH violation |

### AES205 — Circular Dependency

| Test Case    | Input                    | Expected Output           |
| ------------ | ------------------------ | ------------------------- |
| direct cycle | A imports B, B imports A | AES205 CRITICAL violation |
| no cycle     | A imports B, B imports C | No violation              |

## Assumptions & Constraints

- Layer hierarchy is defined in config YAML
- Import statements are parsed via utility functions in shared/common (regex-based extraction, not full AST parsing)
- Workspace structure follows AES conventions

## Glossary

- **AES**: Agentic Engineering System
- **Layer**: Architectural boundary (taxonomy, contract, utility, capabilities, agent, surface, root)
- **Diagnostic**: Violation report with file path, line, column, rule code
- **Dummy Import**: Import that exists only to suppress unused-import warnings, placed inside `_use_*` functions
- **Forbidden Import**: Import that violates layer boundary rules defined in YAML configuration

## Reference

- PRD: [PRD.md](../../PRD.md)

---

## Appendix A: YAML Configuration Schema

The import-rules crate reads its configuration from `lint_arwaky.config.<language>.yaml` files. Below is the schema for the `architecture` section relevant to import rules.

### Top-Level Structure

```yaml
architecture:
  enabled: true # Master switch for all architecture rules
  rules: # Map of rule codes to their configurations
    AES201: { ... }
    AES202: { ... }
    AES203: { ... }
    AES204: { ... }
    AES205: { ... }
```

### Rule Configuration Schema

Each rule (AES201–AES205) follows this schema:

```yaml
AES201:
  enabled: true # Enable/disable this specific rule
  scope: # List of layer prefixes this rule applies to
    - "taxonomy"
    - "contract"
    - "utility"
    - "capabilities"
    - "agent"
    - "surface"
    - "root"
  exceptions: # Filenames to skip (basename match)
    - "main.rs"
    - "lib.rs"
    - "mod.rs"
  conditions: # (AES201/AES202 only) Layer-specific rules
    - scope: "taxonomy(vo)" # File scope pattern
      allowed: ["taxonomy"] # Layers this scope can import from
      mandatory: null # Required imports (null = none)
      forbidden: # Layers this scope cannot import from
        - "agent"
        - "surface"
        - "contract"
        - "utility"
        - "capabilities"
        - "root"
```

### Scope Pattern Syntax

Scope patterns use parentheses to specify sub-layers:

| Pattern                                        | Meaning                               |
| ---------------------------------------------- | ------------------------------------- |
| `taxonomy`                                     | All taxonomy files                    |
| `taxonomy(vo)`                                 | Only taxonomy value objects           |
| `taxonomy(entity,error,event)`                 | Taxonomy entities, errors, and events |
| `contract(protocol)`                           | Only contract protocols               |
| `contract(aggregate)`                          | Only contract aggregates              |
| `capabilities`                                 | All capability files                  |
| `agent(orchestrator)`                          | Only agent orchestrators              |
| `surface(command\|controller\|page)`           | Smart surfaces                        |
| `surface(hook\|store\|action\|screen\|router)` | Utility surfaces                      |
| `surface(component\|view\|layout)`             | Passive surfaces                      |

### Layer Hierarchy (Default)

The default layer hierarchy for import rules:

```
taxonomy (lowest)
  └── contract
       └── utility
       └── capabilities
            └── agent
                 └── surface
                      └── root (highest)
```

### Example: Minimal Config

```yaml
architecture:
  enabled: true
  rules:
    AES201:
      enabled: true
      scope: ["taxonomy", "contract", "capabilities"]
      exceptions: ["main.rs", "lib.rs"]
    AES202:
      enabled: true
      scope: ["capabilities"]
      exceptions: ["main.rs"]
    AES203:
      enabled: true
      scope: ["taxonomy", "contract", "capabilities"]
    AES204:
      enabled: true
      scope: ["taxonomy", "contract", "capabilities"]
    AES205:
      enabled: true
      scope: ["taxonomy", "contract", "capabilities"]
```

---

## Appendix B: File Discovery Algorithm

The file collection method of the import analysis orchestrator discovers source files for analysis. Here is the algorithm:

### Algorithm

```
collect_files(target_path):
  if target_path is a file:
    return [target_path]
  if target_path is a directory:
    return walk_dir(target_path)

walk_dir(dir, is_subdir=false):
  if is_subdir and is_ignored(dir):
    return []
  results = []
  for entry in read_dir(dir):
    if entry is a directory:
      results.extend(walk_dir(entry, is_subdir=true))
    else if entry is a file:
      if is_ignored(entry):
        continue
      if entry.extension in ["rs", "py", "js", "ts", "jsx", "tsx"]:
        results.append(entry)
  return results
```

### Ignore Rules

Files and directories are skipped if they match any of these criteria:

1. **Config-level ignores**: Paths listed in `ignored_paths` in the YAML config
2. **Default skip directories**: `.git`, `node_modules`, `target`, `dist`, `build`, `.venv`, `__pycache__`
3. **Hidden directories**: Any directory starting with `.` (e.g., `.github`, `.vscode`)
4. **File extension**: Only files with extensions `rs`, `py`, `js`, `ts`, `jsx`, `tsx` are collected

### Language Detection

Language is determined by file extension:

| Extension     | Language   |
| ------------- | ---------- |
| `.rs`         | Rust       |
| `.py`         | Python     |
| `.js`, `.jsx` | JavaScript |
| `.ts`, `.tsx` | TypeScript |

### Layer Detection

After file collection, each file's architectural layer is detected from its filename prefix:

| Filename Pattern    | Detected Layer |
| ------------------- | -------------- |
| `taxonomy_*.rs`     | taxonomy       |
| `contract_*.rs`     | contract       |
| `capabilities_*.rs` | capabilities   |
| `utility_*.rs`      | utility        |
| `agent_*.rs`        | agent          |
| `surface_*.rs`      | surface        |
| `root_*.rs`         | root           |

Files without a recognized prefix are skipped by layer-dependent rules (AES201, AES202) but still checked by layer-agnostic rules (AES203, AES204).
