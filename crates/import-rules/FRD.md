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

| Function                    | Input                      | Output          | Description  |
| --------------------------- | -------------------------- | --------------- | ------------ |
| `check_layer_violation()`   | File path, imports         | Vec<Diagnostic> | Check AES201 |
| `check_mandatory_imports()` | File path, imports         | Vec<Diagnostic> | Check AES202 |
| `check_unused_imports()`    | File path, imports, usages | Vec<Diagnostic> | Check AES203 |
| `check_dummy_imports()`     | File path, imports         | Vec<Diagnostic> | Check AES204 |
| `check_circular_deps()`     | All files, imports         | Vec<Diagnostic> | Check AES205 |

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

---

## Appendix A: YAML Configuration Schema

The import-rules crate reads its configuration from `lint_arwaky.config.<language>.yaml` files. Below is the schema for the `architecture` section relevant to import rules.

### Top-Level Structure

```yaml
architecture:
  enabled: true                    # Master switch for all architecture rules
  rules:                           # Map of rule codes to their configurations
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
  enabled: true                    # Enable/disable this specific rule
  scope:                           # List of layer prefixes this rule applies to
    - "taxonomy"
    - "contract"
    - "utility"
    - "capabilities"
    - "agent"
    - "surface"
    - "root"
  exceptions:                      # Filenames to skip (basename match)
    - "main.rs"
    - "lib.rs"
    - "mod.rs"
  conditions:                      # (AES201/AES202 only) Layer-specific rules
    - scope: "taxonomy(vo)"        # File scope pattern
      allowed: ["taxonomy"]        # Layers this scope can import from
      mandatory: null              # Required imports (null = none)
      forbidden:                   # Layers this scope cannot import from
        - "agent"
        - "surface"
        - "contract"
        - "utility"
        - "capabilities"
        - "root"
```

### Scope Pattern Syntax

Scope patterns use parentheses to specify sub-layers:

| Pattern | Meaning |
|---------|---------|
| `taxonomy` | All taxonomy files |
| `taxonomy(vo)` | Only taxonomy value objects |
| `taxonomy(entity,error,event)` | Taxonomy entities, errors, and events |
| `contract(protocol)` | Only contract protocols |
| `contract(aggregate)` | Only contract aggregates |
| `capabilities` | All capability files |
| `agent(orchestrator)` | Only agent orchestrators |
| `surface(command\|controller\|page)` | Smart surfaces |
| `surface(hook\|store\|action\|screen\|router)` | Utility surfaces |
| `surface(component\|view\|layout)` | Passive surfaces |

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

The `ImportOrchestrator::collect_files()` method discovers source files for analysis. Here is the algorithm:

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

| Extension | Language |
|-----------|----------|
| `.rs` | Rust |
| `.py` | Python |
| `.js`, `.jsx` | JavaScript |
| `.ts`, `.tsx` | TypeScript |

### Layer Detection

After file collection, each file's architectural layer is detected from its filename prefix:

| Filename Pattern | Detected Layer |
|------------------|----------------|
| `taxonomy_*.rs` | taxonomy |
| `contract_*.rs` | contract |
| `capabilities_*.rs` | capabilities |
| `utility_*.rs` | utility |
| `agent_*.rs` | agent |
| `surface_*.rs` | surface |
| `root_*.rs` | root |

Files without a recognized prefix are skipped by layer-dependent rules (AES201, AES202) but still checked by layer-agnostic rules (AES203, AES204).
