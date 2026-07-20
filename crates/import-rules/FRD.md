# FRD — import-rules

## Feature Goal
The import-rules crate enforces correct structural boundaries and unidirectional dependency flows. It prevents spaghetti architecture, circular dependencies, and dead/unused imports by validating every import statement against a predefined layer-hierarchy matrix.

## Requirements & Scope
- AES201 Layer Dependency Violation (Unidirectional Flow) — restrict imports by layer hierarchy; lower layers (taxonomy_, contract_) must never import higher layers; utility_ and capabilities_ must not import each other directly, only via contract_ traits.
- AES202 Mandatory Layer Imports — verify required layer imports (e.g. capability imports its contract trait; surface entry imports its container).
- AES203 Unused Import Detection — flag imported symbols never referenced in the file body.
- AES204 Dummy or Forbidden Imports — detect imports pointing to mock, dummy, or forbidden packages in production configs.
- AES205 Circular Dependency Cycle Detection — build an import dependency graph across the workspace and flag cycles.

## Success Indicators
- [ ] Zero dependency cycles — all import cycle loops are detected and resolved.
- [ ] Strict unidirectional flow — complete blocking of cross-layer violations.
- [ ] Cleaner namespace — prompt warning of unused symbols.
- [ ] High performance — graph cycle detection runs within milliseconds (e.g. Tarjan's or DFS-based).
