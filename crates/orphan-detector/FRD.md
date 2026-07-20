# Feature Requirement Document (FRD) - Orphan Detector

See [RULES_AES.md](../../.agents/rules/RULES_AES.md) for AES501-AES506 details and [ARCHITECTURE.md](../../../ARCHITECTURE.md) for layer rules.

## 1. Feature Goal

The goal of the `orphan-detector` module is to identify dead, unused, or unreachable code components across the layers of the 7-layer architecture. By building an import reachability graph starting from valid entry points (such as containers, binary entries, or main files), it flags any architecture component that has been orphaned, preventing codebase bloat and keeping the system maintainable.

## 2. Scope & Requirements

The `orphan-detector` module scans all source code files and evaluates reachability based on the following specifications:

- **AES501: Taxonomy Orphan Checker**

  - **Requirement**: Taxonomy layer files (e.g. `taxonomy_`) must be reachable from contracts, capabilities, or orchestrators.
- **AES502: Contract Orphan Checker**

  - **Requirement**: Contract files (e.g. `contract_`) must have at least one active implementation in the capabilities or utility layers.
- **AES503: Capabilities Orphan Checker**

  - **Requirement**: Capability files (e.g. `capabilities_`) must be instantiated or imported by orchestrators or other capability files.
- **AES504: Utility Orphan Checker**

  - **Requirement**: Utility files (e.g. `utility_`) must be wired into root containers or imported by capabilities/agents that consume their functions.
- **AES505: Agent Orphan Checker**

  - **Requirement**: Agent orchestrator files (e.g. `agent_`) must be called by surface layer files or binary entry points.
- **AES506: Surface Orphan Checker**

  - **Requirement**: Surface layer files (e.g. `surface_`) must be registered in the routing system or called from main entries.

---

## 3. Success Indicators

The success of the `orphan-detector` module is measured by:

- **Dead Code Identification**: 100% detection of unused or unreachable architectural files.
- **Zero False Warnings on Valid Code**: Valid components transitively reachable from entry points must never be flagged as orphans.
- **Configuration Flexibility**: Correctly respects rule-specific exceptions and ignored path patterns.
- **Performance**: Building and traversing the import graph must be highly efficient, taking less than a second even for larger multi-crate projects.
- **Workspace Cleanliness**: Keeps the production binary lightweight and clean of deprecated/unused components.
