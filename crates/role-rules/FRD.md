# FRD — role-rules

## Feature Goal

The role-rules crate enforces architectural boundaries and responsibility rules for each layer (Taxonomy, Contract, Utility, Capabilities, Agent, Surface, Root) as defined by the 7-layer architecture standard. It ensures components behave exactly according to their architectural roles (contracts define protocols, utility provides stateless technical functions, capabilities implement protocols, agents coordinate, taxonomy stays pure).

## Requirements & Scope

- AES401 Taxonomy Purity and Primitives
  - Requirement 1: Taxonomy _constant files must only contain pure constant declarations (pub const, pub static in Rust, or global constants in Python/JS). No logic or variables allowed.
  - Requirement 2: Taxonomy types (Value Objects, entities) must not expose raw primitive types (e.g., raw String, i32, bool) in their public interfaces; they must encapsulate them using strongly-typed domain primitives.
- AES402 Contract Primitive Restriction
  - Requirement: Public method signatures within contract_ traits, protocols, or aggregates must not use raw primitive types. They must receive and return domain-specific Value Objects (VOs) or constants to avoid primitive obsession.
- AES403 Capability Protocol Implementation
  - Requirement: Any capability layer component (e.g. ending in _checker, _analyzer) must implement at least one defined contract protocol. They cannot be floating classes/structures without structural contracts.
- AES404 Utility Purity
  - Requirement: The utility_ layer provides stateless standalone functions only. Utility files must NOT implement any contract_ protocol or aggregate, must not hold state, and must not contain business logic or orchestration. (Replaces the former Infrastructure layer, which was removed; its technical mechanics now live here as free functions.)
- AES405 Agent Orchestrator Purity
  - Requirement: Agent orchestrators must not use dynamic, generic, or untyped constructs (such as any in JS/TS or generic Object/dyn Any in Rust) to bypass strict typing. They must maintain explicit orchestration flows.
- AES406 Surface Passive Role
  - Requirement: Surface components (e.g. _command, _controller, _view) must remain passive. They are strictly dispatchers/presenters and must not contain core business logic, validation rules, or state mutation logic.

## Success Indicators

- [ ] Strict role compliance — all structural rules (AES401–406) are audited at compile/scan time with high precision.
- [ ] Architecture purity — developers are alerted immediately when a contract violates the primitive restriction or a capability lacks a protocol.
- [ ] Precision reporting — reports violations pointing to the exact line and column numbers of the offending syntax.
- [ ] Utility boundary enforcement — every utility_ file is confirmed stateless and contract-free when the feature is complete.
