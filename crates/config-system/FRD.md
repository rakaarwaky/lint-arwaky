# Feature Requirement Document (FRD) - Config System

See [README.md](../../../README.md) for config commands and [ARCHITECTURE.md](../../../ARCHITECTURE.md) for architecture context.

## 1. Feature Goal

The primary purpose of the `config-system` module is to manage lint_arwaky configuration by implementing a loading, parsing, validation, and workspace detection system. This module is responsible for reading `lint_arwaky.config.*.yaml` files and merging them with project-level overrides.

## 2. Requirements & Scope

The `config-system` module is responsible for configuration based on the following specifications:

- **ConfigLoadingOrchestrator**: Coordinates the configuration loading process from various sources.
- **ConfigRulesValidator**: Validates loaded configuration rules against the defined schema.
- **WorkspaceDetector**: Detects Rust workspace roots based on Cargo.toml or common project roots.
- **ConfigParserProvider**: Provides parsers for YAML, TOML (Cargo.toml), and other configuration formats.
- **ConfigYamlReader**: Reads and parses the main YAML configuration file.
- **MultiProjectOrchestrator**: Manages configuration for multiple projects/workspaces simultaneously.


## 3. Success Indicators

The success of the `config-system` module is measured by:

- **Discovery Reliability**: Workspaces are correctly detected from various project structures.
- **Validation Accuracy**: Invalid configurations are rejected with clear error messages.
- **Merge Correctness**: Project-level overrides are merged correctly without conflicts.
- **Rule Conformance**: When complete, the module itself complies with AES rules in its source code.
