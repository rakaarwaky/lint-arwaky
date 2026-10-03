# Agentic Engineering System Architecture

## 1. Purpose

The Agentic Engineering System is a layered, AI-native architecture pattern. It keeps domain models stable, business logic readable, technical detail isolated, and layer boundaries explicit enough for both humans and AI agents to modify the system safely.

---

## 2. Workspace Organization

The architecture supports multi-language workspaces.

| Term               | Meaning                                                           |
| ------------------ | ----------------------------------------------------------------- |
| Project Workspaces | Project root containing all configuration and language members    |
| Workspace Member   | One self-contained crate, package, or module inside the workspace |
| Crates directory   | Rust workspace members                                            |
| Packages directory | TypeScript or JavaScript packages                                 |
| Modules directory  | Python modules                                                    |

### Shared-Kernel Scope (issue #572)

`crates/shared/src/*` is the locked feature kernel: taxonomy, contract, and
utility types that **more than one feature** consumes. Surface-specific state
does not belong there:

| Submodule          | Status                                    | Reason |
| ------------------ | ----------------------------------------- | ------ |
| `tui`              | Moved into `crates/tui`                   | TUI crate was the only consumer |
| `mcp_server`       | Moved into `crates/mcp-server`            | MCP server crate was the only consumer |
| `cli_commands`     | Stays in `shared`                         | `LintResult`/`ScanReport` taxonomy is consumed by 10+ crates (rule crates, dispatcher, report-formatter, cli-commands, root) |
| `report_formatter` | Stays in `shared`                         | `IReportFormatterAggregate` is a cross-crate contract: implemented by `report-formatter`, consumed by `cli-commands` |

When auditing a `shared` submodule, relocate it only if it has a single
consumer; otherwise document the consumers here.

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

_Example feature crate `crates|packages|modules/<name-features>/`_

```text
capabilities_<concern>_<role>.rs/py/ts           ← capabilities layer
agent_<concern>_orchestrator.rs/py/ts            ← agent layer
```

A feature folder holds capabilities and agent files only. A `surface_*`,
`utility_*`, `taxonomy_*`, or `contract_*` file inside a feature folder is
misplaced (**AES702** `feature_has_forbidden_files`). Surface files belong in
a surface folder; utility files belong in the `shared/` kernel folder.

Exceptions: `main.rs`, `lib.rs`, `mod.rs`, `__init__.py`, `index.ts`, `index.js`.

### Shared member

`crates|packages|modules/shared/<common>or<domain-folder>`

```text
contract_<concern>_protocol.rs/py/ts             ← contract layer
contract_<concern>_aggregate.rs/py/ts            ← contract layer
taxonomy_<concern>_vo.rs/py/ts                   ← taxonomy layer
taxonomy_<concern>_event.rs/py/ts                ← taxonomy layer
taxonomy_<concern>_entity.rs/py/ts               ← taxonomy layer
taxonomy_<concern>_constant.rs/py/ts             ← taxonomy layer
utility_<concern>_<role>.rs/py/ts                ← utility layer (shared folder only, AES701)
```

`shared` folder groups by domain. Use `shared/common/` for generic files.

The shared folder is the only place utility files may live. A `utility_*`
file in a feature folder fires **AES702** (`feature_has_forbidden_files`);
in a surface folder fires **AES703** (`surface_has_misplaced_files`).
Feature and surface crates consume utility functions through their layer's
import rules; they do not own or co-locate utility files.

### General Workspace Layout

```text
project-root/                             <- Project workspace root
│
├── crates|packages|modules/              <- workspace members
│   ├── shared/                           <- SHARED: Taxonomy + Contract (all features)
│   │   └── src/
│   │       ├── taxonomy_<domain>_vo.rs/py/ts 
│   │       ├── taxonomy_<domain>_entity.rs/py/ts 
│   │       ├── taxonomy_<domain>_event.rs/py/ts 
│   │       ├── taxonomy_<domain>_error.rs/py/ts 
│   │       ├── taxonomy_<domain>_constant.rs/py/ts 
│   │       ├── contract_<domain>_protocol.rs/py/ts 
│   │       └── contract_<domain>_aggregate.rs/py/ts 
│   │
│   ├── <feature-a>/                      <- FEATURE: <feature-a description>
│   │   └── src/
│   │       ├── agent_<feature-a>_orchestrator.rs/py/ts         <- Agent
│   │       ├── capabilities_<feature-a>_<role>.rs/py/ts        <- Capabilities
│   │       ├── capabilities_<feature-a>_<role>.rs/py/ts        <- Capabilities
│   │       ├── utility_<feature-a>_<role>.rs/py/ts             <- Utility
│   │       ├── utility_<feature-a>_<role>.rs/py/ts             <- Utility
│   │       ├── root_<feature-a>_container.rs/py/ts            <- Root
│   │       └── lib.rs
│   │
│   ├── <feature-b>/                      <- FEATURE: <feature-b description>
│   │   └── src/
│   │       ├── agent_<feature-b>_orchestrator.rs/py/ts         <- Agent
│   │       ├── capabilities_<feature-b>_<role>.rs/py/ts        <- Capabilities
│   │       ├── utility_<feature-b>_<role>.rs/py/ts             <- Utility
│   │       ├── root_<feature-b>_container.rs/py/ts             <- Root
│   │       └── lib.rs
│   │
│   ├── <feature-c>/                      <- FEATURE: <feature-c description>
│   │   └── src/
│   │       ├── surface_<feature-c>_<role>.rs/py/ts             <- Surface
│   │       └── lib.rs
│   │
│   └── ...
│
│
├── Cargo.toml                    
├── package.json
└── pyproject.toml
```

### Folder structure rules (AES701–AES704)

Each member directory (`crates/`, `modules/`, `packages/`) organizes its code into
three kinds of folders: `shared`, feature folders, and surface folders. The
structure-rules group audits that layout, plus the test suite every feature owes.

#### `shared/` — the locked kernel

`shared/` is locked to taxonomy, utility, and contract files. It groups by domain:
`shared/common/` for generic files, or a domain folder for domain-specific ones. A
`capabilities_*`, `agent_*`, or `surface_*` file found here is misplaced
(**AES701**) and must move to a feature folder.

#### Feature folders

A feature folder is named after the feature it serves (`crates/calculator/`). It
must carry at least one `agent_*_orchestrator` file and at least one
`capabilities_*` file; a folder with only one side is incomplete (**AES702**). The
check is per folder — a member-level orchestrator routes across the features but
does not answer for them, so a feature folder owes an orchestrator of its own.

A feature folder documents itself with two files beside its source:

- `FRD.md` — what the feature does
- `BACKLOG.md` — where its work stands

A feature folder carrying neither document fires **AES702**. A folder carrying no
capabilities and no orchestrator is not a feature and owes no document pair.

#### Surface folders

A surface folder is named for the kind of surface it serves: `api`, `mcp`, `cli`,
`desktop`, `tui`. It carries surface files only — a `capabilities_*`, `agent_*`,
or `utility_*` file inside one is misplaced (**AES703**). Root wiring and
barrel files (`lib.rs`, `mod.rs`, `Cargo.toml`, ...) are permitted alongside
the surfaces. A `utility_*` file must live in the `shared/` kernel folder;
move it there if it appears in a surface folder.

A surface folder carries `DESIGN.md`, recording the surface's kind, its entry
points, and the states a user sees. Its level-2 headings are fixed by the
template in `crates/shared/skills/aes-docs/references/HOW-TO-MAKE-DESIGN.md`
and enforced as a closed set by **AES605** — `Brand & Style`, `Components`, and
`Reference` — so a file copied from that template passes with no edit. The
per-surface contract lives in level-3 subsections under them, which stay
free-form. A surface-dominated folder without a `DESIGN.md` fires **AES703**.

#### Test suites

A feature folder that owns source also owes a test suite: `tests/` carrying one
file per test type, and `benches/` carrying at least one benchmark. The
`aes-testing-suite` skill fixes the layout — the flat file-name prefix **is** the
virtual folder, so there is no subdirectory to express the type in:

| Directory  | One file per type                                                                 |
| ---------- | --------------------------------------------------------------------------------- |
| `tests/`   | `contract_`, `unit_`, `integration_`, `dogfood_`, `smoke_`, `e2e_`, `acceptance_` |
| `benches/` | `bench_`                                                                          |

A missing category fires **AES704** (**AES103** covers the naming half: whether a
file's prefix is legal and whether the directory nests). Support prefixes —
`regression_`, `behavioral_`, `mock_`, `fixture_` — are legal names and satisfy no
category, so a folder carrying only regression guards still owes all seven.

#### Summary

| Folder kind  | Carries                                   | Documents                        | Rules   |
| ------------ | ----------------------------------------- | -------------------------------- | ------- |
| `shared/`    | `taxonomy_*`, `utility_*`, `contract_*`   | none                             | AES701  |
| feature      | `capabilities_*` + `agent_*_orchestrator` | `FRD.md` + `BACKLOG.md`          | AES702  |
| surface      | `surface_*` (+ utility, root, barrels)    | `DESIGN.md`                      | AES703  |
| feature      | `tests/` per category + `benches/bench_`  | —                                | AES704  |

---

## 5. Taxonomy Layer

### Purpose

Taxonomy is the domain foundation layer. It defines the stable language of the domain and must remain free from technical or behavioral concerns.

### Components

| Role         | Meaning                               |
| ------------ | ------------------------------------- |
| Value object | Immutable data concept                |
| Entity       | Stateful domain concept with identity |
| Event        | Immutable domain fact                 |
| Error        | Domain-level error                    |
| Constant     | Compile-time literal value            |

### Dependencies

Taxonomy depends on nothing.

### Special Rules

- Value objects and Constants may use all primitive types.
- Entities, Events, and Errors must use Value objects/Constants instead of primitive types (bool/str is an exception).
- Constants must be compile-time values.
- Taxonomy must not contain business rules, infrastructure, or imports from other layers.

---

## 6. Contract Layer

### Purpose

Contract defines the public behavior of the system without exposing implementation. It allows callers to depend on stable interfaces instead of concrete logic.

### Components

| Role      | Meaning                                                                                           |
| --------- | ------------------------------------------------------------------------------------------------- |
| Protocol  | Interface defining inbound behavior. It is implemented by Capabilities and consumed by the Agent. |
| Aggregate | Facade definition implemented by Agent, used by Surface to access feature behavior.               |

### Dependencies

Contract may depend on Taxonomy only.

### Special Rules

- Protocol defines behavior only without implementation.
- Aggregate hides Capabilities from Surface.

---

## 7. Utility Layer

### Purpose

Utility contains reusable low-level mechanics that can be shared cross capabilities. It exists so that Capabilities can remain clean.

### Role Naming

Utility role suffixes are unlimited. The role name is chosen based on demand and must describe the technical responsibility and concern of the file.

### Dependencies

Utility may depend only on Taxonomy.

Utility files live only in the `shared/` kernel folder (AES701). Feature
and surface crates consume utility functions via import; they do not own
or co-locate `utility_*` files. A `utility_*` file in a feature folder is
misplaced (AES702 `feature_has_forbidden_files`); a `utility_*` file in a
surface folder is misplaced (AES703 `surface_has_misplaced_files`).

### Technical Concern Examples

| Concern                 | Responsibility                                      |
| ----------------------- | --------------------------------------------------- |
| File discovery          | Walk directories, detect files, apply ignore        |
| External tool execution | Run linters, compilers, formatters, analyzers       |
| Parsing and matching    | Parse text, match patterns, extract structured data |
| Path normalization      | Normalize paths across platforms                    |
| System operations       | Handle process or environment mechanics             |

### Special Rules

- Utility must use stateless standalone functions only.
- Utility must not contain stateful objects, behavior definitions, or contract implementations.
- Utility must not make business decisions.
- Utility may perform technical operations if needed.
- Utility must not implement any contract.
- Utility role names may expand freely, but the layer must remain technical and standalone.
- Utility must use stateless standalone functions only.

---

## 8. Capabilities Layer

### Purpose

Capabilities contain the concrete implementation of the system's behavior. This layer encapsulates both **pure business logic** (computations, validations) and **external adaptations** (database access, third-party API calls, infrastructure mechanics). By hiding these implementations behind Contracts, the system keeps its behavior modular, swappable, and fully isolated from orchestration.

### Role Naming

Capabilities role suffixes are unlimited. The role name is chosen based on demand and must describe the technical responsibility and concern of the file.

### Dependencies

- Capabilities may depend on Taxonomy, Contract, and Utility.
- Capabilities must not depend on or import other Capabilities.

### Concern Examples

Capabilities generally handle two types of concerns:

| Category                      | Concern        | Responsibility                                 |
| ----------------------------- | -------------- | ---------------------------------------------- |
| **Business Logic**      | Validation     | Check domain conditions or input correctness   |
|                               | Computation    | Calculate scores, totals, or derived values    |
|                               | Transformation | Map, filter, reduce, or reshape data           |
|                               | Resolution     | Apply rules and decide outcomes                |
|                               | Assessment     | Judge severity, compliance, grade, or quality  |
| **External Adaptation** | Repository     | Fetch or persist domain entities to a database |
|                               | Integration    | Communicate with third-party services or APIs  |
|                               | Provider       | Generate data from external systems            |

### Special Rules

- **No Inter-Capability Dependency:** Capabilities must never import or call other Capabilities directly. They are standalone execution units.
- **Pipeline Aggregation:** Multiple Capabilities (e.g., Capability A for data fetching, Capability B for business calculation) are designed to be composed into a sequential pipeline by the **Agent Layer**, not by themselves.
- **Shared Logic Extraction (DRY):** If multiple Capabilities require the same technical mechanics or functions, that logic must be extracted into a reusable standalone function in the **Utility Layer**. Capabilities must not duplicate technical code (Don't Repeat Yourself).
- **Contract Implementation:** Capabilities must implement the `protocol_` defined in the Contract Layer.
- **State Ownership:** Capabilities are the owners of business and technical state within their execution scope.
- **No Domain Definition:** Capabilities must not define domain models (Entities, Value Objects); they only consume Taxonomy.

---

## 9. Agent Layer

### Purpose

Agent coordinates multiple capabilities into executable flows. It controls sequence and movement, not business calculation.

### Allowed Role

The only Agent role is orchestrator.

### Dependencies

Agent may depend only on Taxonomy, Contract, and Utility.

### Allowed Flow Control

| Flow Type               | Purpose                                |
| ----------------------- | -------------------------------------- |
| Sequential execution    | Run steps in order                     |
| Looping                 | Process multiple items or events       |
| Branching               | Choose path based on result            |
| Error handling          | Recover, abort, continue, or escalate  |
| Timeout or cancellation | Stop long-running or asynchronous work |

### Special Rules

- Agent must depend on Contract, not concrete implementations.
- Agent must not use and must be completely ignorant of Capabilities implementations.
- Agent must not calculate business results.
- Agent must not define domain models.

### Cross-Feature Router (`dispatcher`)

Every feature crate owns a single-feature `agent_<feature>_orchestrator`. The
workspace-level `dispatcher` crate is the **cross-feature** surface router:
it sequences per-feature Contract Aggregates and Protocols into executable
flows (scan, ci, fix, setup, ...). The dispatcher itself performs no business
calculation; it delegates every domain decision to the feature's aggregate.

| Role                     | File pattern                        | Allowed Imports                                     | Forbidden Imports          |
| ------------------------ | ---------------------------------- | --------------------------------------------------- | --------------------------- |
| Cross-feature router     | `surface_<concern>_action`         | Taxonomy, Contract (aggregate + protocol), Utility  | Capabilities, Surface, Root |

"Utility" in the allowed-imports column refers to `utility_*` files in the
`shared/` kernel only (AES701). The dispatcher crate does not own or co-locate
utility files; it imports them from `shared`.

Rules:

- Dispatcher files are named `surface_<concern>_action.rs`. The `_action`
  suffix in the dispatcher crate denotes a **cross-feature router**, not a
  Taxonomy-only Utility surface. The AES201 exception below governs this.
- Surface crates (cli-commands, mcp-server, tui) currently depend on the
  `dispatcher` crate directly. The `IOrchestrationAggregate` contract is the
  planned indirection; until it lands, Surface crates import
  `dispatcher::<concern>_action::collect_*` directly.
- Dispatcher is exempt from the Utility-surface import restriction in
  §10: the dispatcher crate's `surface_*_action` files are the sole permitted
  surface files that may import Contract Aggregates and Protocols, because
  their role is cross-feature routing, not passive data/state.

**AES201 exception — cross-feature router:**

| Scope                        | Allowed Imports                                   | Mandatory Imports | Forbidden Imports         |
| ---------------------------- | ------------------------------------------------- | ----------------- | ------------------------- |
| `surface(action)` (dispatcher crate only) | Taxonomy, Contract (aggregate + protocol), Utility | None               | Capabilities, Surface, Root |

---

## 10. Surface Layer

### Purpose

Surface is the outer boundary of the system. It handles user-facing or external-facing interaction and translates it into architectural actions.

### Allowed Roles

Surface roles include:

- command
- controller
- page
- view
- component
- router
- layout
- hook
- store
- action
- screen

### Surface Groups

| Group            | Roles                             | Allowed Dependencies                | Forbidden Dependencies                                     | Rule                                           |
| ---------------- | --------------------------------- | ----------------------------------- | ---------------------------------------------------------- | ---------------------------------------------- |
| Smart surfaces   | command, controller, page, router | Taxonomy, Contract Aggregate, Utility | Agent, Capabilities, Contract Protocol, Root              | May initiate feature behavior through aggregate |
| Utility surfaces | hook, store, action, screen       | Taxonomy                            | Agent, Capabilities, Contract, Utility, Other surfaces, Root | Support smart surfaces, data/state only         |
| Passive surfaces | component, view, layout           | Taxonomy                            | Agent, Contract, Capabilities, Other surfaces, Root        | Presentation-only, no logic or orchestration    |
| Cross-feature router | action (dispatcher crate only)   | Taxonomy, Contract (aggregate + protocol), Utility | Agent, Capabilities, Surface, Root | See §9 AES201 exception                          |

### Special Rules

- Smart surfaces must consume Contract Aggregates to reach capabilities/agent.
- Only smart surfaces may import Utility layer files.
- Utility and passive surfaces must not import Capabilities, Contract, or Agent directly.
- Surfaces must not contain business calculation or orchestration.

---

## 11. Root Layer

### Purpose

Root is the composition layer. It assembles the system by connecting concrete implementations to contracts and starting the application.

### Components

| Role      | Meaning                                                                           |
| --------- | --------------------------------------------------------------------------------- |
| Container | Wires one feature by connecting Capabilities to Contract protocols and aggregates |
| Entry     | Bootstraps the application and composes feature containers                        |

### Dependencies

Root may depend on all layers.

### Special Rules

- Root may instantiate and wire components.
- Root must not contain business logic.
- Root must not contain orchestration policy.
- Root must not contain technical parsing or user interface behavior.

---

## 12. Dependency Policy

Which crates may carry heavyweight dependencies is policy, not accident:

| Dependency | Allowed in | Why |
| ---------- | ---------- | --- |
| `tokio` (async runtime) | `file-watch`, `shared-file-watch` (notify's async debouncer), `mcp-server` (rmcp SDK), and the root package (MCP/TUI binary entries) | These seams are inherently async |
| `rayon` | The parallel lint rule crates and `shared-filesystem` | CPU-bound parallel analysis is the documented performance architecture |
| `tokio`/`rayon` in `tui` | **Not allowed** | The TUI's concurrency is `std::thread` + `std::sync::mpsc` by design ("no async runtime" in README's performance principle) |

The README's "no async runtime" principle means exactly this: no crate outside
the allowlist above may depend on an async runtime. The advisory
`dependency-hygiene.yml` CI job (cargo-machete) catches unused dependencies so
an unjustified runtime cannot silently reappear.

---
