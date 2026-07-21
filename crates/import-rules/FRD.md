# FRD — import-rules

## Feature Goal

The import-rules crate enforces correct structural boundaries and unidirectional dependency flows. It prevents spaghetti architecture, circular dependencies, and dead/unused imports by validating every import statement against a predefined layer-hierarchy matrix.

## Requirements & Scope

- AES201 Layer Dependency Violation (Unidirectional Flow)
  - Requirement: Restricts imports based on the layer hierarchy. Lower layers (e.g., taxonomy_, contract_) must never import higher layers (e.g., capabilities_, utility_, agent_, surface_).
  - Layer Boundary: utility_ and capabilities_ must not import each other directly; .
- AES202 Mandatory Layer Imports
  - Requirement: Verifies that specific layers contain required imports (e.g., ensuring a capability layer file correctly imports its corresponding contract trait, or that a surface entry imports its container).
- AES203 Unused Import Detection
  - Requirement: Detects and flags imported symbols that are never referenced anywhere within the file body.
- AES204 Dummy or Forbidden Imports
  - Requirement: Detects imports that point to mock, dummy, or forbidden packages/modules in production configurations.
- AES205 Circular Dependency Cycle Detection
  - Requirement: Builds a dependency graph of imports across all workspace files and detects cycles (e.g., File A imports B, B imports C, C imports A). Circular dependencies must be flagged.

## Success Indicators

- [ ] Zero dependency cycles — all import cycle loops are detected and resolved.
- [ ] Strict unidirectional flow — complete blocking of cross-layer violations (e.g., taxonomy files importing orchestration layer code).
- [ ] Cleaner namespace — prompt warning of unused symbols to maintain clean, lean namespaces.
- [ ] High performance — graph cycle detection runs within milliseconds using optimized cycle-finding algorithms (e.g., Tarjan's or simple DFS-based cycle detection).
