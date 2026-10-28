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
   Root. Each section: Overview, Component, Rules.
7. **Verify** → `lint-arwaky-cli docs` passes; every dependency table has a line for each
   named layer.

## Template

Copy, fill, delete nothing.

```markdown
# <Project> Architecture

## 1. Purpose

<One paragraph. Name the failure modes this architecture prevents: tangled imports,
undiscoverable behavior, agent misrouting. Keep it to what the boundary rules guard.>

## 2. Workspace Organization

| Term               | Meaning                                                     |
| ------------------ | -------------------------------------------------------------|
| Project Workspace  | The root containing all configuration and language members  |
| Workspace Member   | One self-contained crate, package, or module inside the work-|
|                    | space                                                         |
| Shared / kernel    | Cross-cutting domain models and contracts, never a feature  |

## 3. Naming Convention

File names communicate three parts, joined by underscores, then the language extension:

`layer_concern_role.<ext>`

| Part    | Where it lives         | Example                      |
| ------- | ---------------------- | ------------------------------|
| Layer   | Prefix                 | `taxonomy_`, `contract_`, ...|
| Concern | Middle                 | `tool`, `doc`, `x509`        |
| Role    | Suffix                 | `vo`, `protocol`, `orchestrator`|

Exceptions: `main.rs`, `lib.rs`, `mod.rs`, `__init__.py`, `index.ts`, `index.js`.

## 4. Vertical Slicing Folder Structure

#### Feature member

```

modules/<feature-name>/src/
├── agent_<feature>*orchestrator.py        # Agent layer — only allowed role
├── capabilities*<feature>*<role>.py       # Capabilities layer
├── utility*<feature>*<role>.py            # Utility layer
└── root*<feature>_container.py            # Root layer

```text

Do **not** create `surface/`, `taxonomy/`, `contract/`, `capabilities/`, `utility/`,
`agent/` folders. Layers live in file names, not directories.

#### Shared member

```

modules/shared/src/
├── taxonomy_<domain>*vo.py
├── taxonomy*<domain>*event.py
├── taxonomy*<domain>*error.py
├── contract*<domain>*protocol.py
└── contract*<domain>_aggregate.py

```text

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

```text

## 5. Taxonomy Layer

### Overview

| Aspect       | Detail                                                                                                                                  |
| ------------ | --------------------------------------------------------------------------------------------------------------------------------------- |
| Purpose      | Taxonomy is the domain foundation layer. It defines the stable language of the domain and must remain free from technical or behavioral concerns. |
| Dependencies | Taxonomy depends on nothing.                                                                                                            |

### Component

| Role         | Meaning                               | Allowed Dependencies | Forbidden Dependencies |
| ------------ | ------------------------------------- | -------------------- | ---------------------- |
| Value object | Immutable data concept                | None                 | All other layers       |
| Entity       | Stateful domain concept with identity | None                 | All other layers       |
| Event        | Immutable domain fact                 | None                 | All other layers       |
| Error        | Domain-level error                    | None                 | All other layers       |
| Constant     | Compile-time literal value            | None                 | All other layers       |

### Rules

| Rules                      | Detail                                                                                                    |
| -------------------------- | --------------------------------------------------------------------------------------------------------- |
| Primitive usage            | Value objects and Constants may use all primitive types.                                                  |
| No raw primitives          | Entities, Events, and Errors must use Value objects/Constants instead of primitive types (bool/str is an exception). |
| Compile-time constants     | Constants must be compile-time values.                                                                    |
| Purity                     | Taxonomy must not contain business rules, infrastructure, or imports from other layers.                   |

## 6. Contract Layer

### Overview

| Aspect       | Detail                                                                                                                                      |
| ------------ | ------------------------------------------------------------------------------------------------------------------------------------------- |
| Purpose      | Contract defines the public behavior of the system without exposing implementation. It allows callers to depend on stable interfaces instead of concrete logic. |
| Dependencies | Contract may depend on Taxonomy only.                                                                                                       |

### Component

| Role      | Meaning                                                                                           | Allowed Dependencies | Forbidden Dependencies                      |
| --------- | ------------------------------------------------------------------------------------------------- | -------------------- | ------------------------------------------- |
| Protocol  | Interface defining inbound behavior. It is implemented by Capabilities and consumed by the Agent. | Taxonomy             | Utility, Capabilities, Agent, Surface, Root |
| Aggregate | Facade definition implemented by Agent, used by Surface to access feature behavior.               | Taxonomy             | Utility, Capabilities, Agent, Surface, Root |

### Rules

| Rules              | Detail                                              |
| ------------------ | --------------------------------------------------- |
| Behavior only      | Protocol defines behavior only, without implementation. |
| Capability hiding  | Aggregate hides Capabilities from Surface.          |

## 7. Utility Layer

### Overview

| Aspect       | Detail                                                                                                                                  |
| ------------ | --------------------------------------------------------------------------------------------------------------------------------------- |
| Purpose      | Utility contains reusable low-level mechanics that can be shared across capabilities. It exists so that Capabilities can remain clean. |
| Role Naming  | Utility role suffixes are unlimited. The role name is chosen based on demand and must describe the technical responsibility and concern of the file. |
| Dependencies | Utility may depend only on Taxonomy.                                                                                                    |
| Location     | Utility files live only in the `shared/` kernel folder (AES701).                                                                        |

### Component

| Concern                 | Responsibility                                      | Allowed Dependencies | Forbidden Dependencies                      |
| ----------------------- | --------------------------------------------------- | -------------------- | ------------------------------------------- |
| File discovery          | Walk directories, detect files, apply ignore        | Taxonomy             | Contract, Capabilities, Agent, Surface, Root |
| External tool execution | Run linters, compilers, formatters, analyzers       | Taxonomy             | Contract, Capabilities, Agent, Surface, Root |
| Parsing and matching    | Parse text, match patterns, extract structured data | Taxonomy             | Contract, Capabilities, Agent, Surface, Root |
| Path normalization      | Normalize paths across platforms                    | Taxonomy             | Contract, Capabilities, Agent, Surface, Root |
| System operations       | Handle process or environment mechanics             | Taxonomy             | Contract, Capabilities, Agent, Surface, Root |

### Rules

| Rules                    | Detail                                                                                                                                                       |
| ------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Stateless functions      | Utility must use stateless standalone functions only.                                                                                                        |
| No objects or contracts  | Utility must not contain stateful objects, behavior definitions, or contract implementations.                                                                |
| No business decisions    | Utility must not make business decisions. It may perform technical operations if needed.                                                                     |
| Technical scope          | Utility role names may expand freely, but the layer must remain technical and standalone.                                                                    |
| Kernel-only location     | Feature and surface crates consume utility functions via import; they do not own or co-locate `utility_*` files.                                             |
| Misplacement             | A `utility_*` file in a feature folder is misplaced (AES702 `feature_has_forbidden_files`); in a surface folder it is misplaced (AES703 `surface_has_misplaced_files`). |

## 8. Capabilities Layer

### Overview

| Aspect       | Detail                                                                                                                                                                                                                                                       |
| ------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Purpose      | Capabilities contain the concrete implementation of the system's behavior. This layer encapsulates both **pure business logic** (computations, validations) and **external adaptations** (database access, third-party API calls, infrastructure mechanics). By hiding these behind Contracts, behavior stays modular, swappable, and isolated from orchestration. |
| Role Naming  | Capabilities role suffixes are unlimited. The role name is chosen based on demand and must describe the technical responsibility and concern of the file.                                                                                                    |
| Dependencies | Capabilities may depend on Taxonomy, Contract, and Utility. Capabilities must not depend on or import other Capabilities.                                                                                                                                    |

### Component

| Category                | Concern        | Responsibility                                 | Allowed Dependencies             | Forbidden Dependencies                 |
| ----------------------- | -------------- | ---------------------------------------------- | -------------------------------- | -------------------------------------- |
| **Business Logic**      | Validation     | Check domain conditions or input correctness   | Taxonomy, Contract, Utility      | Other Capabilities, Agent, Surface, Root |
|                         | Computation    | Calculate scores, totals, or derived values    | Taxonomy, Contract, Utility      | Other Capabilities, Agent, Surface, Root |
|                         | Transformation | Map, filter, reduce, or reshape data           | Taxonomy, Contract, Utility      | Other Capabilities, Agent, Surface, Root |
|                         | Resolution     | Apply rules and decide outcomes                | Taxonomy, Contract, Utility      | Other Capabilities, Agent, Surface, Root |
|                         | Assessment     | Judge severity, compliance, grade, or quality  | Taxonomy, Contract, Utility      | Other Capabilities, Agent, Surface, Root |
| **External Adaptation** | Repository     | Fetch or persist domain entities to a database | Taxonomy, Contract, Utility      | Other Capabilities, Agent, Surface, Root |
|                         | Integration    | Communicate with third-party services or APIs  | Taxonomy, Contract, Utility      | Other Capabilities, Agent, Surface, Root |
|                         | Provider       | Generate data from external systems            | Taxonomy, Contract, Utility      | Other Capabilities, Agent, Surface, Root |

### Rules

| Rules                         | Detail                                                                                                                                                              |
| ----------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| No inter-capability dependency | Capabilities must never import or call other Capabilities directly. They are standalone execution units.                                                           |
| Pipeline aggregation          | Multiple Capabilities (e.g., A for data fetching, B for business calculation) are composed into a sequential pipeline by the **Agent Layer**, not by themselves.    |
| Shared logic extraction (DRY) | If multiple Capabilities need the same technical mechanics, extract them into a reusable standalone function in the **Utility Layer**. Do not duplicate technical code. |
| Contract implementation       | Capabilities must implement the `protocol_` defined in the Contract Layer.                                                                                          |
| State ownership               | Capabilities own business and technical state within their execution scope.                                                                                         |
| No domain definition          | Capabilities must not define domain models (Entities, Value Objects); they only consume Taxonomy.                                                                   |

## 9. Agent Layer

### Overview

| Aspect       | Detail                                                                                                    |
| ------------ | --------------------------------------------------------------------------------------------------------- |
| Purpose      | Agent coordinates multiple capabilities into executable flows. It controls sequence and movement, not business calculation. |
| Allowed Role | The only Agent role is orchestrator.                                                                      |
| Dependencies | Agent may depend only on Taxonomy, Contract, and Utility.                                                 |

### Component

| Flow Type               | Purpose                               | Allowed Dependencies        | Forbidden Dependencies          |
| ----------------------- | ------------------------------------- | --------------------------- | ------------------------------- |
| Sequential execution    | Run steps in order                    | Taxonomy, Contract, Utility | Capabilities, Surface, Root     |
| Looping                 | Process multiple items or events      | Taxonomy, Contract, Utility | Capabilities, Surface, Root     |
| Branching               | Choose path based on result           | Taxonomy, Contract, Utility | Capabilities, Surface, Root     |
| Error handling          | Recover, abort, continue, or escalate | Taxonomy, Contract, Utility | Capabilities, Surface, Root     |
| Timeout or cancellation | Stop long-running or asynchronous work | Taxonomy, Contract, Utility | Capabilities, Surface, Root     |

### Rules

| Rules                    | Detail                                                                              |
| ------------------------ | ----------------------------------------------------------------------------------- |
| Contract dependency      | Agent must depend on Contract, not concrete implementations.                        |
| Capability ignorance     | Agent must not use and must be completely ignorant of Capabilities implementations. |
| No business calculation  | Agent must not calculate business results.                                          |
| No domain definition     | Agent must not define domain models.                                                |

## 10. Surface Layer

### Overview

| Aspect        | Detail                                                                                                                                         |
| ------------- | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| Purpose       | Surface is the outer boundary of the system. It handles user-facing or external-facing interaction and translates it into architectural actions. |
| Allowed Roles | command, controller, page, view, component, router, layout, hook, store, action, screen                                                        |

### Component

| Group            | Roles                             | Allowed Dependencies                  | Forbidden Dependencies                                       | Detail                                          |
| ---------------- | --------------------------------- | ------------------------------------- | ------------------------------------------------------------ | ----------------------------------------------- |
| Smart surfaces   | command, controller, page, router | Taxonomy, Contract Aggregate, Utility | Agent, Capabilities, Contract Protocol, Root                 | May initiate feature behavior through aggregate |
| Utility surfaces | hook, store, action, screen       | Taxonomy                              | Agent, Capabilities, Contract, Utility, Other surfaces, Root | Support smart surfaces, data/state only         |
| Passive surfaces | component, view, layout           | Taxonomy                              | Agent, Contract, Capabilities, Other surfaces, Root          | Presentation-only, no logic or orchestration    |

### Rules

| Rules                         | Detail                                                                                  |
| ----------------------------- | --------------------------------------------------------------------------------------- |
| Aggregate consumption         | Smart surfaces must consume Contract Aggregates to reach capabilities/agent.            |
| Utility import restriction    | Only smart surfaces may import Utility layer files.                                     |
| Direct dependency restriction | Utility and passive surfaces must not import Capabilities, Contract, or Agent directly. |
| Business logic restriction    | Surfaces must not contain business calculation or orchestration.                        |

## 11. Root Layer

### Overview

| Aspect       | Detail                                                                                                                                   |
| ------------ | ---------------------------------------------------------------------------------------------------------------------------------------- |
| Purpose      | Root is the composition layer. It assembles the system by connecting concrete implementations to contracts and starting the application. |
| Dependencies | Root may depend on all layers.                                                                                                           |

### Component

| Role      | Allowed Dependencies | Forbidden Dependencies | Detail                                                                             |
| --------- | -------------------- | ---------------------- | ---------------------------------------------------------------------------------- |
| Container | All layers           | None                   | Wires one feature by connecting Capabilities to Contract protocols and aggregates. |
| Entry     | All layers           | None                   | Bootstraps the application and composes feature containers.                        |

### Rules

| Rules                    | Detail                                                              |
| ------------------------ | ------------------------------------------------------------------- |
| Wiring                   | Root may instantiate and wire components.                           |
| No business logic        | Root must not contain business logic.                               |
| No orchestration         | Root must not contain orchestration policy.                         |
| No technical/UI behavior | Root must not contain technical parsing or user interface behavior. |
```

---

## Section Contract

Every section is required unless marked optional. Each exists for one reason.

| Section                        | Why it belongs here                                                                      |
| ------------------------------ | ------------------------------------------------------------------------------------------|
| Purpose                        | Names the failure modes the architecture prevents. Watch for descriptive prose instead of rules.|
| Workspace Organization         | Defines terms so engineers and agents share a vocabulary. Watch for unstated implicit terms. |
| Naming Convention              | Makes file names self-documenting. Watch for missing exceptions list.                        |
| Vertical Slicing Layout        | Shows where each file lands; prevents the `surface/` folder mistake. Watch for missing shared layout.|
| Taxonomy Layer                 | The foundation — if it drifts, everything above drifts. Watch for missing dependency rule.   |
| Contract Layer                 | The stability boundary. Watch for missing "no implementation" guard.                         |
| Utility Layer                  | Shared mechanics that keep Capabilities clean. Watch for business logic leaking into Utility.|
| Capabilities Layer             | The behavior implementation. Watch for inter-capability imports.                             |
| Agent Layer                    | The orchestrator — sequence, not business. Watch for business calculation here.              |
| Surface Layer                  | The outer boundary. Watch for orchestration or business logic leaking into Surface.          |
| Root Layer                     | The composition entry. Watch for business logic in the container.                            |

---

## Verify

```bash
lint-arwaky-cli docs .
# Checks: dead-link, absolute-path, secret-in-docs, doc-length.
# Manual: every layer states its allowed dependencies; no implementation detail;
#   file names follow layer_concern_role.ext; no per-file exceptions without a subsection.
```
