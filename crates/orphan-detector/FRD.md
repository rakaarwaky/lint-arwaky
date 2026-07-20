# FRD — orphan-detector

## Feature Goal
The orphan-detector crate identifies dead, unused, or unreachable code components across the 7-layer architecture. By building an import reachability graph starting from valid entry points (containers, binary entries, main files), it flags any architecture component that has been orphaned, preventing codebase bloat and keeping the system maintainable.

## Requirements & Scope
- AES501 Taxonomy Orphan Checker
  - Requirement: Taxonomy layer files (e.g. taxonomy_) must be reachable from contracts, capabilities, or orchestrators.
- AES502 Contract Orphan Checker
  - Requirement: Contract files (e.g. contract_) must have at least one active implementation in the capabilities or utility layers.
- AES503 Capabilities Orphan Checker
  - Requirement: Capability files (e.g. capabilities_) must be instantiated or imported by orchestrators or other capability files.
- AES504 Utility Orphan Checker
  - Requirement: Utility files (e.g. utility_) must be wired into root containers or imported by capabilities/agents that consume their functions.
- AES505 Agent Orphan Checker
  - Requirement: Agent orchestrator files (e.g. agent_) must be called by surface layer files or binary entry points.
- AES506 Surface Orphan Checker
  - Requirement: Surface layer files (e.g. surface_) must be registered in the routing system or called from main entries.
- Configurable exceptions and ignored path patterns.

## Success Indicators
- [ ] Dead code identification — 100% detection of unused or unreachable architectural files.
- [ ] Zero false warnings on valid code — valid components transitively reachable from entry points must never be flagged as orphans.
- [ ] Configuration flexibility — correctly respects rule-specific exceptions and ignored path patterns.
- [ ] Performance — building and traversing the import graph must be highly efficient, taking less than a second even for larger multi-crate projects.
- [ ] Workspace cleanliness — keeps the production binary lightweight and clean of deprecated/unused components.
