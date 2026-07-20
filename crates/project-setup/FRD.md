# FRD — project-setup

## Feature Goal
The project-setup crate provides scaffolding facilities and doctor checks for new projects. It initializes projects with AES-compliant directory layouts, sets up MCP configuration, and provides CI/CD templates for lint_arwaky integration.

## Requirements & Scope
- In scope:
  - init — create the AES directory structure (taxonomy, contract, utility, capabilities, agent, surface, root).
  - doctor — check whether a project is ready for lint_arwaky.
  - mcp-config — create MCP configuration for AI integration.
  - ci-templates — provide GitHub Actions templates or CI scripts.
- Out of scope:
  - Running the linter — it scaffolds and configures; invoking the checks is done through the CLI/server.
  - Dependency updates or security audits — those are separate operational tasks.

## Success Indicators
- [ ] Structure correctness — directories and files created per AES patterns (Taxonomy → Contract → Utility → Capabilities → Agent → Surface → Root).
- [ ] Template accuracy — created templates ready to use and meet standards.
- [ ] CI integration — CI workflows usable immediately without modification.
