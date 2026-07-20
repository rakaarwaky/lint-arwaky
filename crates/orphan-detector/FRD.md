# FRD — orphan-detector

## Feature Goal
The orphan-detector crate identifies dead, unused, or unreachable code components across the 7-layer architecture. By building an import reachability graph from valid entry points (containers, binary entries, main files), it flags any architecture component that has been orphaned — preventing codebase bloat and keeping the system maintainable.

## Requirements & Scope
- In scope:
  - AES501 Taxonomy Orphan Checker — taxonomy_ files must be reachable from contracts, capabilities, or orchestrators.
  - AES502 Contract Orphan Checker — contract_ files must have at least one active implementation in capabilities or utility.
  - AES503 Capabilities Orphan Checker — capabilities_ files must be instantiated or imported by orchestrators or other capability files.
  - AES504 Utility Orphan Checker — utility_ files must be wired into root containers or imported by consuming capabilities/agents.
  - AES505 Agent Orphan Checker — agent_ orchestrators must be called by surface layers or binary entry points.
  - AES506 Surface Orphan Checker — surface_ files must be registered in the routing system or called from main entries.
  - Configurable exceptions and ignored path patterns.
- Out of scope:
  - Building the reachability graph from entry points — that is a separate detector; this crate consumes import facts only.
  - Checking per-file metrics or naming — those are independent analysis stages.

## Success Indicators
- [ ] Dead code identification — 100% detection of unused/unreachable architectural files.
- [ ] Zero false warnings on valid code — transitively reachable components never flagged as orphans.
- [ ] Configuration flexibility — correctly respects rule-specific exceptions and ignored paths.
- [ ] Performance — import graph built and traversed in under a second for larger multi-crate projects.
- [ ] Workspace cleanliness — production binary stays free of deprecated/unused components.
