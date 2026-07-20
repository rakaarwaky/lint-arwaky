# FRD — role-rules

## Feature Goal
The role-rules crate enforces architectural boundaries and responsibility rules for each layer (Taxonomy, Contract, Utility, Capabilities, Agent, Surface, Root) as defined by the 7-layer architecture standard. It ensures components behave exactly according to their architectural roles (contracts define protocols, utility provides stateless technical functions, capabilities implement protocols, agents coordinate, taxonomy stays pure).

## Requirements & Scope
- AES401 Taxonomy Purity and Primitives — taxonomy _constant files contain pure constants only; taxonomy types must not expose raw primitives in public interfaces (use strongly-typed domain primitives).
- AES402 Contract Primitive Restriction — public signatures in contract_ traits/protocols/aggregates must use domain VOs/constants, not raw primitives.
- AES403 Capability Protocol Implementation — every capability component must implement at least one contract protocol.
- AES404 Utility Purity — utility_ provides stateless standalone functions only; must not implement contract_ protocols/aggregates, hold state, or contain business logic/orchestration (replaces the former infrastructure_ layer).
- AES405 Agent Orchestrator Purity — agent orchestrators must not use dynamic/untyped constructs (any, dyn Any, Object) to bypass strict typing.
- AES406 Surface Passive Role — surface components (_command, _controller, _view) stay passive: dispatchers/presenters only, no business logic/validation/state mutation.

## Success Indicators
- [ ] Strict role compliance — AES401–406 audited at compile/scan time with high precision.
- [ ] Architecture purity — immediate alerts when a contract violates primitive restriction or a capability lacks a protocol.
- [ ] Precision reporting — violations point to exact line and column of the offending syntax.
- [ ] Utility boundary enforcement — every utility_ file confirmed stateless and contract-free when complete.
