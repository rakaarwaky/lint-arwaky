# Feature Requirement Document (FRD) - Import Rules

See [RULES_AES.md](../../.agents/rules/RULES_AES.md) for AES201-AES205 details and [ARCHITECTURE.md](../../../ARCHITECTURE.md) for layer rules.

## 1. Feature Goal

The primary goal of the `import-rules` module is to enforce correct structural boundaries and unidirectional dependency flows. It prevents spaghetti architecture, circular dependencies, and dead/unused imports by validating every import statement against a predefined layer-hierarchy matrix.

## 2. Scope & Requirements

The `import-rules` module analyzes import paths and validates compliance using the following specifications:

- **AES201: Layer Dependency Violation (Unidirectional Flow)**

  - **Requirement**: Restricts imports based on the layer hierarchy. Lower layers (e.g., `taxonomy_`, `contract_`) must never import higher layers (e.g., `capabilities_`, `utility_`, `agent_`, `surface_`).
  - **Layer Boundary**: `utility_` and `capabilities_` must not import each other directly; they must interact through `contract_` traits.
- **AES202: Mandatory Layer Imports**

  - **Requirement**: Verifies that specific layers contain required imports (e.g., ensuring a capability layer file correctly imports its corresponding contract trait, or that a surface entry imports its container).
- **AES203: Unused Import Detection**

  - **Requirement**: Detects and flags imported symbols that are never referenced anywhere within the file body.
- **AES204: Dummy or Forbidden Imports**

  - **Requirement**: Detects imports that point to mock, dummy, or forbidden packages/modules in production configurations.
- **AES205: Circular Dependency Cycle Detection**

  - **Requirement**: Builds a dependency graph of imports across all workspace files and detects cycles (e.g., File A imports B, B imports C, C imports A). Circular dependencies must be flagged.

---

## 3. Success Indicators

The success of the `import-rules` module is measured by:

- **Zero Dependency Cycles**: All import cycle loops are detected and resolved.
- **Strict Unidirectional Flow**: Complete blocking of cross-layer violations (e.g., taxonomy files importing orchestration layer code).
- **Cleaner Namespace**: Prompt warning of unused symbols to maintain clean, lean namespaces.
- **High Performance**: Graph cycle detection runs within milliseconds using optimized cycle-finding algorithms (e.g., Tarjan's or simple DFS-based cycle detection).
