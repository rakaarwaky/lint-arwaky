# Agentic Engineering System Architecture

## 1. Purpose

The Agentic Engineering System is a layered, AI-native architecture pattern. It keeps domain models stable, business logic readable, technical detail isolated, and layer boundaries explicit enough for both humans and AI agents to modify the system safely.

---

## 2. Workspace Organization

The architecture supports multi-language workspaces.

| Term               | Meaning                                                          |
| ------------------ | -----------------------------------------------------------------|
| Project Workspaces | Project root containing all configuration and language members   |
| Workspace Member   | One self-contained crate, package, or module inside the workspace|
| Crates directory   | Rust workspace members                                           |
| Packages directory | TypeScript or JavaScript packages                                |
| Modules directory  | Python modules                                                   |

---

## 3. Naming Convention

File names must communicate three parts:

1. Layer as prefix
2. Concern as middle name
3. Role as suffix

The parts are joined by underscores, followed by the normal file extension for the language.

`layer_concern_role.rs/py/ts`

---

## 4. Vertical Slicing Layout

AI agents frequently make this mistake. Do NOT create `surface/`, `taxonomy/`,
`contract/`, `capabilities/`, `utility/`, `agent/` folders. The correct structure
groups files by feature, with layers as filenames, not directories.

### Features member

*Example* `crates|packages|modules/<name-features>/`

```text
capabilities_<concern>_<role>.rs/py/ts           ← capabilities layer
agent_<concern>_orchestrator.rs/py/ts            ← agent layer
```

A feature folder holds capabilities and agent files only.

A feature folder documents itself with two files beside its source:

- `FRD.md` — what the feature does
- `BACKLOG.md` — where its work stands

### Surface member

*Example* `crates|packages|modules/<name-surfaces>/`

```text
surface_<concern>_<role>.rs/py/ts                ← surface layer
```

A surface folder holds surfaces only.

A surface folder is named for the kind of surface it serves: `api`, `mcp`, `cli`, `desktop`, `tui`.

A surface folder documents itself with two files beside its source:

- `DESIGN.md` — what the surface design looks like
- `BACKLOG.md` — where its work stands

### Shared member

`crates|packages|modules/shared/<common>or<domain-folder>`

```text
contract_<concern>_protocol.rs/py/ts             ← contract layer
contract_<concern>_aggregate.rs/py/ts            ← contract layer
taxonomy_<concern>_vo.rs/py/ts                   ← taxonomy layer
taxonomy_<concern>_event.rs/py/ts                ← taxonomy layer
taxonomy_<concern>_entity.rs/py/ts               ← taxonomy layer
taxonomy_<concern>_constant.rs/py/ts             ← taxonomy layer
utility_<concern>_<role>.rs/py/ts                ← utility layer 
```

`shared` is locked to taxonomy, utility, and contract files.

`shared` groups by domain: `shared/common/` for generic files, or a domain folder for domain-specific ones.

### General Workspace Layout

```text
project-root/                             <- Project workspace root
│
├── crates|packages|modules/              <- workspace members
│   ├── shared/                           <- SHARED: Taxonomy + Contract + Utility (all features)
│   │   └── src/
│   │       ├── taxonomy_<domain>_vo.rs/py/ts 
│   │       ├── taxonomy_<domain>_entity.rs/py/ts 
│   │       ├── contract_<domain>_protocol.rs/py/ts 
│   │       └── utility_<domain>_<role>.rs/py/ts 
│   │
│   ├── <feature-a>/                      <- FEATURE: <feature-a description>
│   │   └── src/
│   │       ├── agent_<feature-a>_orchestrator.rs/py/ts         <- Agent
│   │       ├── capabilities_<feature-a>_<role>.rs/py/ts        <- Capabilities
│   │       ├── capabilities_<feature-a>_<role>.rs/py/ts        <- Capabilities
│   │       ├── root_<feature-a>_container.rs/py/ts             <- Root
│   │       └── lib.rs
│   │
│   ├── <feature-b>/                      <- FEATURE: <feature-b description>
│   │   └── src/
│   │       ├── agent_<feature-b>_orchestrator.rs/py/ts         <- Agent
│   │       ├── capabilities_<feature-b>_<role>.rs/py/ts        <- Capabilities
│   │       ├── root_<feature-b>_container.rs/py/ts             <- Root
│   │       └── lib.rs
│   │
│   ├── <surface-a>/                      <- SURFACE: <surface-a description>
│   │   └── src/
│   │       ├── surface_<surface-a>_<role>.rs/py/ts             <- Surface
│   │       └── lib.rs
│   │
│   ├── <surface-b>/                      <- SURFACE: <surface-b description>
│   │   └── src/
│   │       ├── surface_<surface-b>_<role>.rs/py/ts             <- Surface
│   │       └── lib.rs
│   │
│   └── ...
├── Cargo.toml                    
├── package.json
└── pyproject.toml
```

#### Summary

| Folder kind | Carries                                   | Documents                 |
| ----------- | ----------------------------------------- | --------------------------|
| `shared/`   | `taxonomy_*`, `utility_*`, `contract_*`   | `DATA.md` + `BACKLOG.md`  |
| feature     | `capabilities_*` + `agent_*_orchestrator` | `FRD.md` + `BACKLOG.md`   |
| surface     | `surface_*`                               | `DESIGN.md` + `BACKLOG.md`|

---

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

---

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

---

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

---

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

---

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

---

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

---

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

---

## 12. Finding rendering notes

Group 6 (Document Invariants) and Group 7 (Folder Structure) findings carry a
`violation_type` field (snake_case) that appears in the JSON report. The
per-finding `why` and `fix` strings in `DocFinding` / `StructureFinding` are
the authoritative explanations — the linter sets them at emission time.
`RULES_AES.md` documents only the violation-type names and when each fires; it
deliberately omits the `why`/`fix` rendering notes to avoid drift. Use the
JSON `why` and `fix` fields in scan output for remediation guidance.
