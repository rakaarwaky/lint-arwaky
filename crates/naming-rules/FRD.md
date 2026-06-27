# Feature Requirement Document (FRD) - Naming Rules

## 1. Feature Goal

The primary objective of the `naming-rules` feature is to enforce strict naming conventions across the codebase to ensure consistency, readability, and adherence to the 7-layer architecture system. By validating that files and identifiers conform to structural and semantic naming patterns, it prevents naming chaos and helps developers instantly recognize the architectural role of any component based solely on its name.

## 2. Requirements & Scope

The `naming-rules` module is responsible for scanning workspace source files and auditing naming compliance based on the following rules:

### Rules Specifications

- **AES101: Name Convention Consistency**

  - **Requirement**: All file stems (basenames without extensions) must be in `snake_case` (lowercase letters and underscores only).
  - **Scope**: Applies to all scanned source files in the project (Rust, Python, JavaScript, TypeScript).
  - **Length Constraint**: Each name segment must be at least 2 words or meet project-defined length patterns to avoid overly short/cryptic names (e.g., `db.rs` is flagged; `db_connector.rs` is accepted).

- **AES102: Suffix and Prefix Layer Alignment**

  - **Requirement**: Files must match the specific naming convention of the architectural layer they belong to. The architectural layer of a file is determined by its prefix, and its suffix must align with its definition.
  - **Scope**:
    - `taxonomy_` files must end with allowed suffixes: `_vo`, `_entity`, `_event`, `_error`, `_constant`, `_utility`, `_helper`.
    - `contract_` files must end with: `_port`, `_protocol`, `_aggregate`.
    - `capabilities_` files must end with: `_checker`, `_analyzer`, `_processor`, etc.
    - `infrastructure_` files must end with: `_adapter`, `_provider`, `_scanner`, etc.
    - `agent_` files must end with: `_orchestrator` only.
    - `surface_` files must end with: `_command`, `_controller`, `_page`, `_view`, `_component`, `_router`, `_layout`, `_hook`, `_store`, `_action`, `_screen`.
    - `root_` files must end with: `_container`, `_entry`.

### Inputs

- A list of source file paths (`&[FilePath]`) and the current project configuration (`ArchitectureConfig`).

### Outputs

- A list of naming violations containing the file path, violating name, the exact rule violated (AES101/AES102), and detailed error descriptions.

---

## 3. Success Indicators

The success of the `naming-rules` module is measured by the following metrics:

- **Accuracy**: Zero false positives. Valid `snake_case` stems and correct layer suffixes must never be flagged, while invalid ones must be caught 100% of the time.
- **Coverage**: Successfully checks Rust, Python, JavaScript, and TypeScript files as configured in the project settings.
- **Integration**: Correctly reports issues to the central CLI/MCP runner with precise location mappings.
- **Self-Audit**: The `naming-rules` codebase itself must adhere to naming rules (e.g., prefix and suffix alignment) and pass the naming checks.
