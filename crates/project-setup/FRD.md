# Feature Requirement Document (FRD) - Project Setup

See [README.md](../../../README.md) for setup commands and [DEPLOY.md](../../../DEPLOY.md) for install options.

## 1. Feature Goal

The `project-setup` module provides scaffolding facilities and doctor checks for new projects. This module initializes projects with AES-compliant directory layouts, sets up MCP configuration, and provides CI/CD templates for lint_arwaky integration.

## 2. Requirements & Scope

The `project-setup` module is responsible for setup based on the following specifications:

### Component Specifications

- **SetupManagementOrchestrator**: Coordinates the project initialization process.
- **SetupManagementProcessor**: Processes templates and files that need to be created.
- **SetupInstallerAdapter**: Adapter for filesystem operations and template installation.

### Features

- **init**: Create AES directory structure (taxonomy, contract, utility, capabilities, agent, surface, root).
- **doctor**: Check whether a project is ready for lint_arwaky.
- **mcp-config**: Create MCP configuration for AI integration.
- **ci-templates**: Provide GitHub Actions templates or CI scripts.

### Inputs

- Target path for initialization.
- Setup parameters (language, framework, template).

### Outputs

- Created template files.
- Initialization status and error messages when needed.

---

## 3. Success Indicators

The success of the `project-setup` module is measured by:

- **Structure Correctness**: Directories and files are created according to AES patterns (Taxonomy → Contract → Utility → Capabilities → Agent → Surface → Root).
- **Template Accuracy**: Created templates are ready to use and meet standards.
- **CI Integration**: CI workflows can be used immediately without modification.
