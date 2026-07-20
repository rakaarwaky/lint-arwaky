# FRD — config-system

## Feature Goal
The config-system crate manages lint_arwaky configuration: loading, parsing, validation, and workspace detection. It reads lint_arwaky.config.*.yaml files and merges them with project-level overrides.

## Requirements & Scope
- ConfigLoadingOrchestrator — Coordinates the configuration loading process from various sources.
- ConfigRulesValidator — Validates loaded configuration rules against the defined schema.
- WorkspaceDetector — Detects Rust workspace roots based on Cargo.toml or common project roots.
- ConfigParserProvider — Provides parsers for YAML, TOML (Cargo.toml), and other configuration formats.
- ConfigYamlReader — Reads and parses the main YAML configuration file.
- MultiProjectOrchestrator — Manages configuration for multiple projects/workspaces simultaneously.

## Success Indicators
- [ ] Discovery reliability — workspaces are correctly detected from various project structures.
- [ ] Validation accuracy — invalid configurations are rejected with clear error messages.
- [ ] Merge correctness — project-level overrides are merged correctly without conflicts.
- [ ] Rule conformance — the crate itself complies with AES rules in its source code when complete.
