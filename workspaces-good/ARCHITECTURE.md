# Calculator Architecture

## 1. Purpose

This architecture guards against tangled imports, undiscoverable behavior, and agent misrouting across the calculator workspace. Every layer states what it may depend on and what it must not touch.

## 2. Workspace Organization

| Term               | Meaning                                                      |
| ------------------ | -------------------------------------------------------------|
| Project Workspace  | The root containing all configuration and language members   |
| Workspace Member   | One self-contained crate, package, or module inside the workspace|
| Shared / layer    | Cross-cutting domain models and contracts, never a feature   |

## 3. Naming Convention

File names communicate three parts, joined by underscores, then the language extension:

`layer_concern_role.<ext>`

| Part    | Where it lives         | Example                       |
| ------- | ---------------------- | ------------------------------|
| Layer   | Prefix                 | `taxonomy_`, `contract_`, ... |
| Concern | Middle                 | `calculator`, `addition`, ... |
| Role    | Suffix                 | `vo`, `protocol`, `orchestrator`|

Exceptions: `main.rs`, `lib.rs`, `mod.rs`, `__init__.py`, `index.ts`, `index.js`.

## 4. Vertical Slicing Layout

### Feature member

```

modules/<feature>/src/
├── agent_<feature>_orchestrator.py        # Agent layer — only allowed role
├── capabilities_<feature>_<role>.py       # Capabilities layer
└── root_<feature>_container.py            # Root layer

```

### Shared member

```

modules/shared/src/
├── taxonomy_<domain>_vo.py
├── contract_<domain>_protocol.py
└── contract_<domain>_aggregate.py

```

### General Workspace Layout

```

project-root/
├── crates/
│   ├── shared/src/           <- Rust taxonomy + contract (all features)
│   ├── <feature>/src/        <- Rust feature source
│   └── cli/                  <- Rust CLI surface
├── modules/
│   ├── shared/src/           <- Python taxonomy + contract (all features)
│   ├── <feature>/src/        <- Python feature source
│   └── cli/                  <- Python CLI surface
├── packages/
│   ├── shared/src/           <- TypeScript taxonomy + contract (all features)
│   ├── <feature>/src/        <- TypeScript feature source
│   └── cli/                  <- TypeScript CLI surface
├── PRD.md
├── README.md
└── AGENTS.md

```

## 5. Taxonomy Layer

### Purpose

Taxonomy is the domain foundation. It defines the stable language — immutable data concepts, stateful entities, events, errors, constants — without technical or behavioral concerns.

### Components

| Role      | Meaning                             |
| --------- | ------------------------------------|
| Value obj | Immutable data concept              |
| Entity    | Stateful domain concept with identity|
| Event     | Immutable domain fact               |
| Error     | Domain-level error                  |
| Constant  | Compile-time literal value          |

### Dependencies

Taxonomy depends on nothing.

### Special Rules

- Entities, Events, and Errors use Value Objects / Constants, not primitives (bool / str are exempt).
- Constants must be compile-time values.
- Taxonomy must not contain business rules, infrastructure, or imports from other layers.

## 6. Contract Layer

### Purpose

Contract defines public behavior without exposing implementation. Callers depend on stable interfaces, not concrete logic.

### Components

| Role      | Meaning                                                            |
| --------- | -------------------------------------------------------------------|
| Protocol  | Interface consumed by Agent, implemented by Capabilities           |
| Aggregate | Facade consumed by Surface, implemented by Agent                   |

### Dependencies

Contract may depend on Taxonomy only.

### Special Rules

- Protocol defines behavior only — no implementation.
- Aggregate hides Capabilities from Surface.

## 7. Utility Layer

### Purpose

Utility contains reusable low-level mechanics shareable across capabilities, so Capabilities remain clean.

### Dependencies

Utility may depend only on Taxonomy.

### Special Rules

- Stateless standalone functions only.
- No stateful objects, behavior definitions, or contract implementations.
- No business decisions. May perform technical operations.
- May not implement any contract.

## 8. Capabilities Layer

### Purpose

Capabilities contain concrete implementation — both pure business logic and external adaptations — isolated behind Contracts.

### Dependencies

- May depend on Taxonomy, Contract, and Utility.
- Must not depend on or import other Capabilities.

### Special Rules

- No inter-capability dependency. Standalone execution units.
- Pipeline aggregation is orchestrated by the Agent layer, not by Capabilities themselves.
- Shared logic extraction (DRY): cross-capability mechanics move to Utility.
- Contract implementation: must implement the `protocol_` defined in Contract.
- State ownership: Capabilities own business and technical state in their scope.
- No domain definition: consume Taxonomy; never define Entities or Value Objects.

## 9. Agent Layer

### Purpose

Agent coordinates multiple capabilities into executable flows. Controls sequence and movement, not business calculation.

### Dependencies

Agent may depend only on Taxonomy, Contract, and Utility.

### Allowed Flow Control

| Type              | Purpose                           |
| ----------------- | ----------------------------------|
| Sequential        | Run steps in order                |
| Looping           | Process multiple items or events  |
| Branching         | Choose path based on result       |
| Error handling    | Recover, abort, continue, escalate|
| Timeout/cancel    | Stop long-running or async work   |

### Special Rules

- Depends on Contract, not concrete implementations.
- Completely ignorant of Capabilities' implementations.
- Must not calculate business results.
- Must not define domain models.
- Allowed role: **orchestrator** only.

## 10. Surface Layer

### Purpose

Surface is the outer boundary — user-facing or external interaction translated into architectural actions.

### Dependencies

Smart surfaces may depend on Taxonomy, Contract Aggregate, and Utility. Utility and passive surfaces depend on Taxonomy only.

### Special Rules

- Smart surfaces consume Contract Aggregates to reach capabilities/agent.
- Only smart surfaces may import Utility.
- Utility and passive surfaces must not import Capabilities, Contract, or Agent.
- No business calculation or orchestration lives here.

## 11. Root Layer

### Purpose

Root assembles the system by wiring concrete implementations to contracts and starting the application.

### Components

| Role      | Meaning                                                      |
| --------- | -------------------------------------------------------------|
| Container | Wires one feature: capabilities to protocol and aggregate    |
| Entry     | Bootstraps and composes feature containers                   |

### Dependencies

Root may depend on all layers.

### Special Rules

- May instantiate and wire components.
- Must not contain business logic.
- Must not contain orchestration policy.
- Must not contain technical parsing or user interface behavior.
