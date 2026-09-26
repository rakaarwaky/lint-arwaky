# HOW TO MAKE ARCHITECTURE.md

> **Purpose**: Document the layered architecture — what each layer is, what it may depend on,
> and what it must not touch — so both humans and agents modify the system safely.
>
> **Audience**: Engineers, Tech Lead, AI agents that read or generate structural code.
>
> **Scope**: The architectural rules that govern the workspace. Exactly one per project root.
>
> **Location**: Project root (or at `README.md` level when the repo has a single crate/package).
>
> **Length**: 50–500 lines (flat budget shared by every document type).

---

## Rules

1. **Write rules, not descriptions.** Every section must say what an agent or engineer
   is allowed to do or not do. A paragraph that reads like marketing text instead of
   an enforcement rule is the bug in this file.
2. **Dependencies are explicit.** Every layer states exactly what it may import from,
   and exactly what it may not. Vague "follows best practices" lines are replaced
   with named-layer restrictions (`Layer X depends only on Layer Y`).
3. **No per-file exceptions without a reason.** When a layer has a special case,
   state the exception as its own subsection and name the failure mode it prevents.
4. **One diagram per page.** ASCII trees are acceptable. Embedding large PNG or SVG
   images violates the length budget and breaks copy-paste for agents.
5. **Do not restate code.** If a pattern is visible in the source already, the
   architecture file explains *why* the pattern exists, not the implementation.
6. **Name the naming convention.** File names must communicate three parts —
   layer, concern, role — joined by underscores, followed by the language extension:
   `layer_concern_role.<ext>`.
7. **No implementation detail.** Concrete class names, API signatures, and library
   choices belong in `FRD.md`. This file owns the boundary rules; FRD owns the
   per-feature mechanics.
8. **Nothing about the current build.** Whether a layer shipped or not is a backlog
   claim with evidence, not an architecture sentence.

---

## Workflow

1. **Create file** → `ARCHITECTURE.md` at repo root (note: spelling `ARCHITECHTURE` is
   intentionally variant here only if your team prefers it; the checker uses the
   heading text, not the file name).
2. **Section: Purpose** — one paragraph stating what the architecture guards against.
3. **Section: Workspace Organization** — define the workspace terms and member types.
4. **Section: Naming Convention** — the `layer_concern_role.<ext>` rule and its
   language suffixes.
5. **Section: Vertical Slicing Folder Structure** — feature layout and shared layout.
6. **Section per layer** — Taxonomy, Contract, Utility, Capabilities, Agent, Surface,
   Root. Each section: Purpose, Components, Dependencies, Special Rules.
7. **Verify** → `aa check docs` passes; every dependency table has a line for each
   named layer.

## Template

Copy, fill, delete nothing.

```markdown
# <Project> Architecture

## 1. Purpose

<One paragraph. Name the failure modes this architecture prevents: tangled imports,
undiscoverable behavior, agent misrouting. Keep it to what the boundary rules guard.>

## 2. Workspace Organization

| Term               | Meaning                                                       |
| ------------------ | ------------------------------------------------------------- |
| Project Workspace  | The root containing all configuration and language members    |
| Workspace Member   | One self-contained crate, package, or module inside the work- |
|                    | space                                                           |
| Shared / kernel    | Cross-cutting domain models and contracts, never a feature    |

## 3. Naming Convention

File names communicate three parts, joined by underscores, then the language extension:

`layer_concern_role.<ext>`

| Part    | Where it lives         | Example                        |
| ------- | ---------------------- | ------------------------------ |
| Layer   | Prefix                 | `taxonomy_`, `contract_`, ...  |
| Concern | Middle                 | `tool`, `doc`, `x509`          |
| Role    | Suffix                 | `vo`, `protocol`, `orchestrator` |

Exceptions: `main.rs`, `lib.rs`, `mod.rs`, `__init__.py`, `index.ts`, `index.js`.

## 4. Vertical Slicing Folder Structure

#### Feature member

```
modules/<feature-name>/src/
├── agent_<feature>_orchestrator.py        # Agent layer — only allowed role
├── capabilities_<feature>_<role>.py       # Capabilities layer
├── utility_<feature>_<role>.py            # Utility layer
└── root_<feature>_container.py            # Root layer
```

Do **not** create `surface/`, `taxonomy/`, `contract/`, `capabilities/`, `utility/`,
`agent/` folders. Layers live in file names, not directories.

#### Shared member

```
modules/shared/src/
├── taxonomy_<domain>_vo.py
├── taxonomy_<domain>_event.py
├── taxonomy_<domain>_error.py
├── contract_<domain>_protocol.py
└── contract_<domain>_aggregate.py
```

`shared/common/` holds generic utilities not tied to a domain.

### General Workspace Layout

```
project-root/
├── modules/
│   ├── shared/src/           <- Taxonomy + Contract (all features)
│   ├── <feature-a>/src/
│   ├── <feature-b>/src/
│   └── ...
├── PRD.md
├── README.md
└── AGENTS.md
```

## 5. Taxonomy Layer

### Purpose

<Taxonomy is the domain foundation. It defines the stable language — immutable data
concepts, stateful entities, events, errors, constants — without technical or
behavioral concerns.>

### Components

| Role      | Meaning                              |
| --------- | ------------------------------------ |
| Value obj | Immutable data concept               |
| Entity    | Stateful domain concept with identity|
| Event     | Immutable domain fact                |
| Error     | Domain-level error                   |
| Constant  | Compile-time literal value           |

### Dependencies

Taxonomy depends on nothing.

### Special Rules

- Entities, Events, and Errors use Value Objects / Constants, not primitives
  (bool / str are exempt).
- Constants must be compile-time values.
- Taxonomy must not contain business rules, infrastructure, or imports from other layers.

## 6. Contract Layer

### Purpose

<Contract defines public behavior without exposing implementation. Callers depend
on stable interfaces, not concrete logic.>

### Components

| Role      | Meaning                                                             |
| --------- | ------------------------------------------------------------------- |
| Protocol  | Interface consumed by Agent, implemented by Capabilities            |
| Aggregate | Facade consumed by Surface, implemented by Agent                    |

### Dependencies

Contract may depend on Taxonomy only.

### Special Rules

- Protocol defines behavior only — no implementation.
- Aggregate hides Capabilities from Surface.

## 7. Utility Layer

### Purpose

<Utility contains reusable low-level mechanics shareable across capabilities, so
Capabilities remain clean.>

### Dependencies

Utility may depend only on Taxonomy.

### Special Rules

- Stateless standalone functions only.
- No stateful objects, behavior definitions, or contract implementations.
- No business decisions. May perform technical operations.
- May not implement any contract.

## 8. Capabilities Layer

### Purpose

<C apabilities contain concrete implementation — both pure business logic and
external adaptations — isolated behind Contracts.>

### Dependencies

- May depend on Taxonomy, Contract, and Utility.
- Must **not** depend on or import other Capabilities.

### Special Rules

- **No inter-capability dependency.** Standalone execution units.
- **Pipeline aggregation** is orchestrated by the Agent layer, not by Capabilities themselves.
- **Shared logic extraction (DRY):** cross-capability mechanics move to Utility.
- **Contract implementation:** must implement the `protocol_` defined in Contract.
- **State ownership:** Capabilities own business and technical state in their scope.
- **No domain definition:** consume Taxonomy; never define Entities or Value Objects.

## 9. Agent Layer

### Purpose

<Agent coordinates multiple capabilities into executable flows. Controls sequence
and movement, not business calculation.>

### Dependencies

Agent may depend only on Taxonomy, Contract, and Utility.

### Allowed Flow Control

| Type              | Purpose                            |
| ----------------- | ---------------------------------- |
| Sequential        | Run steps in order                 |
| Looping           | Process multiple items or events   |
| Branching         | Choose path based on result        |
| Error handling    | Recover, abort, continue, escalate |
| Timeout/cancel    | Stop long-running or async work    |

### Special Rules

- Depends on Contract, not concrete implementations.
- Completely ignorant of Capabilities' implementations.
- Must not calculate business results.
- Must not define domain models.
- Allowed role: **orchestrator** only.

## 10. Surface Layer

### Purpose

<Surface is the outer boundary — user-facing or external interaction translated
into architectural actions.>

### Dependencies

Smart surfaces may depend on Taxonomy, Contract Aggregate, and Utility.
Utility and passive surfaces depend on Taxonomy only.

### Special Rules

- Smart surfaces consume Contract Aggregates to reach capabilities/agent.
- Only smart surfaces may import Utility.
- Utility and passive surfaces must not import Capabilities, Contract, or Agent.
- No business calculation or orchestration lives here.

## 11. Root Layer

### Purpose

<Root assembles the system by wiring concrete implementations to contracts and
starting the application.>

### Components

| Role      | Meaning                                                       |
| --------- | ------------------------------------------------------------- |
| Container | Wires one feature: capabilities to protocol and aggregate     |
| Entry     | Bootstraps and composes feature containers                    |

### Dependencies

Root may depend on all layers.

### Special Rules

- May instantiate and wire components.
- Must not contain business logic.
- Must not contain orchestration policy.
- Must not contain technical parsing or user interface behavior.
```

---

## Section Contract

Every section is required unless marked optional. Each exists for one reason.

| Section                        | Why it belongs here                                                                        |
| ------------------------------ | ------------------------------------------------------------------------------------------ |
| Purpose                        | Names the failure modes the architecture prevents. Watch for descriptive prose instead of rules. |
| Workspace Organization         | Defines terms so engineers and agents share a vocabulary. Watch for unstated implicit terms.   |
| Naming Convention              | Makes file names self-documenting. Watch for missing exceptions list.                          |
| Vertical Slicing Layout        | Shows where each file lands; prevents the `surface/` folder mistake. Watch for missing shared layout. |
| Taxonomy Layer                 | The foundation — if it drifts, everything above drifts. Watch for missing dependency rule.     |
| Contract Layer                 | The stability boundary. Watch for missing "no implementation" guard.                           |
| Utility Layer                  | Shared mechanics that keep Capabilities clean. Watch for business logic leaking into Utility.  |
| Capabilities Layer             | The behavior implementation. Watch for inter-capability imports.                               |
| Agent Layer                    | The orchestrator — sequence, not business. Watch for business calculation here.                |
| Surface Layer                  | The outer boundary. Watch for orchestration or business logic leaking into Surface.            |
| Root Layer                     | The composition entry. Watch for business logic in the container.                              |

---

## Verify

```bash
aa check docs .
# Checks: dead-link, absolute-path, secret-in-docs, doc-length.
# Manual: every layer states its allowed dependencies; no implementation detail;
#   file names follow layer_concern_role.ext; no per-file exceptions without a subsection.
```
