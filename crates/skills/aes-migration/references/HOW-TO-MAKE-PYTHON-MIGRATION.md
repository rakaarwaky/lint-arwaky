# HOW TO MAKE MIGRATION PYTHON

> **Purpose**: Guide phased migration of legacy Python projects into AES layered architecture — taxonomy → contract → utility → capabilities → agent → surface → root.
>
> **Audience**: Agents and engineers executing a migration to AES.
>
> **Scope**: Python `modules/` workspaces; all 7 AES layers; 9 migration phases (0–8).
>
> **Location**: Project root; shared layers under `modules/shared/src/<domain>/`, feature layers under `modules/<feature>/src/`, entry at `modules/root_<name>_entry.py`.
>
> **Length**: 9 phases; duration depends on violation count.

---

## Rules

1. **Phase 0 first — audit before touching anything.** Run `lint-arwaky-cli scan .`; record baseline violations to choose strategy.
2. **Taxonomy before contract before capabilities.** VOs must exist before protocols can reference them.
3. **Protocol = one method per feature.** Never a mega `execute(op, …)` that dispatches multiple features.
4. **Aggregate = many methods, one per export.** Rich consumer surface; surface/root call specific verbs.
5. **Utility is stateless free functions only.** No classes, no state, no upward imports (AES404).
6. **Capabilities implement protocol; agent implements aggregate.** No cross-layer imports (AES201).
7. **Surface calls aggregate; never imports agent or capabilities.** Dependency arrow points down.
8. **Root wires everything; never contains business logic.** Container constructs; entry bootstraps.
9. **Verify at every phase.** `lint-arwaky-cli scan <layer-dir>` → 0 before moving to next phase.
10. **Every file needs at least 5 lines.** AES302 fires below that. Docstrings and comments count.

---

## Workflow

1. **Phase 0 — Audit** — run `lint-arwaky-cli scan .`; record baseline.
2. **Phase 1 — Taxonomy** — extract VOs, errors, constants.
3. **Phase 2 — Contract** — create protocol (1 method) + aggregate (many exports).
4. **Phase 3 — Utility** — extract stateless helpers to shared.
5. **Phase 4 — Capabilities** — implement protocols with business logic.
6. **Phase 5 — Agent** — implement aggregate, delegate to capabilities.
7. **Phase 6 — Surface** — map I/O, call aggregate.
8. **Phase 7 — Root** — wire containers, bootstrap entry.
9. **Phase 8 — Verify** — full scan → 0 violations; tests green.

---

## AES Dependency Model

AES uses **dependency injection** as the inter-layer wiring mechanism.
Layers do not import each other directly — they import from **contract**
and receive dependencies via constructor injection:

```
                    ┌──────────────────────────────────┐
                    │             root                  │
                    │  (DI wiring — wires everything)   │
                    └──────┬───────────────────────────┘
                           │
              ┌────────────┼─────────────┐
              ▼            ▼             ▼
         ┌────────┐  ┌─────────┐  ┌──────────────┐
         │surface │  │  agent  │  │ capabilities │
         └───┬────┘  └────┬────┘  └──────┬───────┘
             │            │              │
             ▼            ▼              ▼
        ┌──────────────────────────────────────────┐
        │      contract (protocol / aggregate)      │
        └──────────────────┬───────────────────────┘
                           ▼
                  ┌──────────────────┐
                  │    taxonomy       │
                  └──────────────────┘

         utility ←── flexible, imports taxonomy only
```

**Key principles:**
- Agent does **not** import capabilities — it receives them via constructor injection.
- Surface does **not** import agent — it receives the orchestrator via constructor injection.
- Capabilities **implements** protocol ABCs. Agent **implements** aggregate ABCs.
- Utility is flexible — imports taxonomy only, imported by capabilities/agent/surface.
- Python DI pattern: pass ABC instances via `__init__` constructor parameters.
- Import boundaries are the target architecture. Enforcement is partial — see Verify.

---

## Workspace Structure

```
project-root/
├── pyproject.toml           ← workspace root config
├── lint_arwaky.config.yaml  ← AES config (created in Phase 0)
├── modules/
│   ├── shared/              ← shared taxonomy + contract + utility types
│   │   ├── pyproject.toml
│   │   └── src/
│   │       ├── __init__.py       ← barrel: re-exports every shared type
│   │       ├── common/           ← truly shared across ALL features
│   │       │   └── __init__.py
│   │       └── <domain>/         ← shared types per feature domain
│   │           ├── __init__.py
│   │           ├── taxonomy_<concept>_vo.py
│   │           ├── taxonomy_<concept>_error.py
│   │           ├── taxonomy_<concept>_constant.py
│   │           ├── contract_<concept>_protocol.py
│   │           ├── contract_<concept>_aggregate.py
│   │           └── utility_<concept>_<role>.py
│   │
│   ├── <feature>/           ← feature module: capabilities only
│   │   ├── pyproject.toml
│   │   └── src/
│   │       ├── __init__.py
│   │       └── capability_<concept>_<role>.py
│   │
│   ├── agent_<domain>_orchestrator.py  ← agent layer, directly under modules/
│   ├── root_<domain>_container.py      ← root layer, directly under modules/
│   ├── root_<name>_entry.py            ← entry point (file inside modules/)
│   └── <surface_group>/
│       └── src/
│           ├── __init__.py
│           └── surface_<concept>_<role>.py
│
└── tests/
```

**Key rules:**
- Shared layers (taxonomy, contract, utility) live under `modules/shared/src/<domain>/`.
- Capabilities live under their feature module: `modules/<feature>/src/`.
- Agent, root, and entry live **directly under `modules/`**, not inside a feature module.
- Surface lives in its own group directory, e.g. `modules/cli/src/`.
- `modules/shared/src/common/` holds types shared across ALL features.
- Every package directory needs `__init__.py` (barrel — skipped by lint).

> **Naming note:** the layer prefix is `capability_` (singular) in the reference
> workspace, not `capabilities_`. Both are accepted by the linter's flexible
> capabilities suffix list; match whatever your existing project already uses
> and stay consistent. `capabilities_` is the documented canonical form in
> ARCHITECTURE.md.

---

## Prerequisites

```bash
# Install lint-arwaky
pip install lint-arwaky-cli

# Verify installation
lint-arwaky-cli version
# Expected: Lint Arwaky v2.0.0+

# Install external linters (optional, for external lint checks)
pip install ruff mypy bandit
lint-arwaky-cli install
```

---

## Phase 0: Audit & Config Setup

> **Skill:** `aes-lint-arwaky` — load for audit commands and violation analysis.

### Step 1: Initialize Config

```bash
cd your-project/
lint-arwaky-cli init
```

This creates `lint_arwaky.config.yaml` with default AES rules.

### Step 2: Run Initial Audit

```bash
lint-arwaky-cli scan .
```

### Step 3: Assess Migration Scope

| Violations | Strategy                                                    |
| ------------ | ------------------------------------------------------------- |
| < 10       | Full migration in one session                               |
| 10–50     | Phased migration (Phase 1 → 8)                             |
| > 50       | Start with taxonomy only (Phase 1), re-audit, then continue |

### Step 4: Count Files

```bash
find modules -name "*.py" | grep -v __init__ | grep -v __pycache__ | wc -l
```

---

## Phase 1: Taxonomy Layer

> **Skill:** `aes-taxonomy` — load for VOs, errors, constants, entities, events.

Define Value Objects, Errors, Events, and Constants under
`modules/shared/src/<domain>/`.

### Steps

1. Identify domain types:
   ```bash
   grep -rn "^class " modules/*/src/ | grep -v test | grep -v __init__
   ```
2. Load `aes-taxonomy` skill.
3. Create taxonomy files following skill templates.
4. Re-export from the domain `__init__.py`.
5. Import them from a `contract_*.py` file — see Orphan reachability below.

### Orphan reachability — read this before Phase 2

A taxonomy file that is only referenced by barrels and other taxonomy files
is reported as an orphan:

```
[AES501] 'taxonomy_user_vo' is not reachable and not imported by higher layers.
WHY? only imported by lower-layer files (__init__.py, taxonomy_user_error.py)
FIX: Import 'taxonomy_user_vo' from a _entry file AND a contract_* or higher-layer file.
```

Barrel registration alone does **not** clear this. The shared barrel satisfies
the "has importers" half; you also need a `contract_*` file to import the
taxonomy type. Writing the Phase 2 contract resolves it naturally — verify at
the end of Phase 2, not at the end of Phase 1.

### Example

```python
# modules/shared/src/user/taxonomy_user_vo.py
"""User domain value objects."""

from dataclasses import dataclass


@dataclass(frozen=True)
class UserId:
    """User identifier value object."""
    _value: str

    def __post_init__(self) -> None:
        if not self._value.strip():
            raise ValueError("UserId cannot be empty")

    @property
    def value(self) -> str:
        return self._value
```

```python
# modules/shared/src/user/taxonomy_user_error.py
"""User domain errors."""


class UserError(Exception):
    """Base error for user domain."""


class UserNotFoundError(UserError):
    """Raised when a user is not found."""

    def __init__(self, user_id: "UserId") -> None:
        super().__init__(f"User not found: {user_id.value}")
```

```python
# modules/shared/src/user/taxonomy_user_constant.py
"""User domain constants."""

# Maximum accepted username length.
MAX_USERNAME_LENGTH: int = 128

# Minimum accepted password length.
MIN_PASSWORD_LENGTH: int = 8

# Default page size for list responses.
DEFAULT_PAGE_SIZE: int = 50
```

> The comment lines above each constant are not decoration — they keep the file
> above the AES302 five-line minimum. A bare three-constant file fails.

### Rules Enforced

- **AES101**: Filename must be `taxonomy_<concept>_<suffix>.py` (snake_case, 3+ words). Machine-checked.
- **AES102**: Suffix must be `vo`, `entity`, `error`, `event`, or `constant`. Machine-checked.
- **AES302**: File must be at least 5 lines. Machine-checked.
- **AES401**: `_constant` files must contain only module-level assignments — no `class`, no `def`. Machine-checked.
- **AES401**: Raw primitives in entity/event/error fields. **Partially checked** — see Verify.
- **AES501**: Orphan detection. Machine-checked; see reachability note above.

---

## Phase 2: Contract Layer

> **Skill:** `aes-contract` — load for protocol and aggregate ABCs.

Contracts define public interfaces (Protocols and Aggregates) using
`abc.ABC` without exposing implementation.

### Steps

1. Load `aes-contract` skill.
2. Create protocol ABCs (inbound/outbound) under `modules/shared/src/<domain>/`.
3. Create aggregate facade ABCs under `modules/shared/src/<domain>/`.
4. Re-export from the domain `__init__.py`.
5. **Import every taxonomy type you created in Phase 1** — this clears AES501.
6. Verify: `lint-arwaky-cli scan modules/shared/src/<domain>`.

### Example

```python
# modules/shared/src/user/contract_user_protocol.py
"""User repository protocol contract."""

from abc import ABC, abstractmethod
from typing import Optional

from modules.shared.src.user.taxonomy_user_vo import UserId, Email, User
from modules.shared.src.user.taxonomy_user_error import UserError


class IUserRepositoryProtocol(ABC):
    """Protocol for user repository operations.
    Implemented by capabilities layer.
    """

    @abstractmethod
    def find_by_id(self, user_id: UserId) -> Optional[User]:
        ...

    @abstractmethod
    def find_by_email(self, email: Email) -> Optional[User]:
        ...

    @abstractmethod
    def save(self, user: User) -> None:
        ...
```

```python
# modules/shared/src/user/contract_user_aggregate.py
"""User aggregate facade contract."""

from abc import ABC, abstractmethod

from modules.shared.src.user.taxonomy_user_vo import UserId, UserResponse
from modules.shared.src.user.taxonomy_user_error import UserError


class IUserAggregate(ABC):
    """Aggregate facade for user operations.
    Implemented by agent layer.
    """

    @abstractmethod
    def get_user(self, user_id: UserId) -> UserResponse:
        ...

    @abstractmethod
    def register_user(self, command: "RegisterCommand") -> UserResponse:
        ...
```

### Rules Enforced

- **AES102**: Suffix must be `protocol` or `aggregate`. Machine-checked.
- **AES302**: File must be at least 5 lines. Machine-checked.
- **AES402**: No raw primitives in method signatures. **Partially checked.**
- **AES201**: Protocol must not import aggregate; aggregate may import protocol. Partially checked — see Verify.

---

## Phase 3: Utility Layer

> **Skill:** `aes-utility` — load for stateless standalone functions.

Utility contains low-level technical mechanics — **stateless standalone
functions only**. No classes.

### Steps

1. Identify reusable stateless functions across modules.
2. Load `aes-utility` skill.
3. Create utility files under `modules/shared/src/<domain>/`.
4. Re-export from the domain `__init__.py`.
5. Verify: `lint-arwaky-cli scan modules/shared/src/<domain>`.

### Example

```python
# modules/shared/src/user/utility_user_validator.py
"""User validation utilities. Stateless functions only — no classes."""

import re
import uuid

from modules.shared.src.user.taxonomy_user_vo import Email


def validate_email(email: Email) -> bool:
    """Validate email format."""
    pattern = r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"
    return bool(re.match(pattern, email.value))


def normalize_email(email: Email) -> Email:
    """Normalize email to lowercase."""
    return Email(value=email.value.lower())


def generate_user_id() -> str:
    """Generate a UUID-based user identifier."""
    return str(uuid.uuid4())
```

### Rules Enforced

- **AES102**: Suffix is flexible, but forbidden suffixes apply (`vo`, `entity`, `protocol`, `aggregate`, etc.). Machine-checked.
- **AES302**: File must be at least 5 lines. Machine-checked.
- **AES404**: No `class` definitions. **Partially checked.**
- **AES201**: Utility may import taxonomy only. Partially checked — see Verify.

---

## Phase 4: Capabilities Layer

> **Skill:** `aes-capabilities` — load for business logic and external adaptation.

Capabilities contain concrete behavior implementations. They **implement
protocol ABCs** defined in the contract layer via inheritance.

### Steps

1. Load `aes-capabilities` skill.
2. Create business logic capabilities (inherit protocol ABCs).
3. Create external adaptation capabilities (repositories, clients).
4. Verify: `lint-arwaky-cli scan modules/<feature>/src`.

### Example

```python
# modules/user/src/capability_user_repository.py
"""User repository capability — implements IUserRepositoryProtocol."""

from typing import Optional

from modules.shared.src.user.contract_user_protocol import IUserRepositoryProtocol
from modules.shared.src.user.taxonomy_user_vo import UserId, Email, User
from modules.shared.src.user.taxonomy_user_error import UserNotFoundError


class UserRepository(IUserRepositoryProtocol):
    """Concrete user repository backed by database."""

    def __init__(self, db_connection: "DatabaseConnection") -> None:
        self._db = db_connection

    def find_by_id(self, user_id: UserId) -> Optional[User]:
        row = self._db.query("SELECT * FROM users WHERE id = %s", user_id.value)
        if row is None:
            return None
        return User.from_row(row)

    def find_by_email(self, email: Email) -> Optional[User]:
        row = self._db.query("SELECT * FROM users WHERE email = %s", email.value)
        if row is None:
            return None
        return User.from_row(row)

    def save(self, user: User) -> None:
        self._db.execute(
            "INSERT INTO users (id, email, name) VALUES (%s, %s, %s)",
            user.id.value, user.email.value, user.name.value,
        )
```

### Rules Enforced

- **AES102**: Suffix is flexible (forbidden: `vo`, `entity`, `protocol`, `aggregate`, `utility`). Machine-checked.
- **AES302**: File must be at least 5 lines. Machine-checked.
- **AES201**: Capabilities must not import agent, surface, other capabilities. Partially checked.
- **AES202**: Must import taxonomy and contract(protocol). Partially checked.
- **AES403**: At least 1 class must inherit a protocol ABC; max 3 classes per file. Partially checked.

---

## Phase 5: Agent Layer

> **Skill:** `aes-agent` — load for orchestration logic.

Orchestrates sequential execution, branching, looping, and error handling.
**Implements aggregate ABCs** defined in the contract layer.

### Steps

1. Load `aes-agent` skill.
2. Create orchestrator class inheriting aggregate ABC.
3. Inject protocol dependencies via constructor.
4. Verify: `lint-arwaky-cli scan modules`.

### Example

```python
# modules/agent_user_orchestrator.py
"""User orchestrator — implements IUserAggregate."""

from modules.shared.src.user.contract_user_aggregate import IUserAggregate
from modules.shared.src.user.contract_user_protocol import IUserRepositoryProtocol
from modules.shared.src.user.taxonomy_user_vo import UserId, UserResponse
from modules.shared.src.user.taxonomy_user_error import UserNotFoundError


class UserOrchestrator(IUserAggregate):
    """Orchestrates user operations via injected repository."""

    def __init__(self, repository: IUserRepositoryProtocol) -> None:
        self._repository = repository

    def get_user(self, user_id: UserId) -> UserResponse:
        user = self._repository.find_by_id(user_id)
        if user is None:
            raise UserNotFoundError(user_id)
        return UserResponse.from_user(user)

    def register_user(self, command: "RegisterCommand") -> UserResponse:
        # orchestration: validate → check duplicate → save → return
        existing = self._repository.find_by_email(command.email)
        if existing is not None:
            raise UserAlreadyExistsError(command.email)
        user = User.create(command)
        self._repository.save(user)
        return UserResponse.from_user(user)
```

### Rules Enforced

- **AES102**: Suffix must be `orchestrator`. Machine-checked.
- **AES302**: File must be at least 5 lines. Machine-checked.
- **AES201**: Agent must not import capabilities, surface. Partially checked.
- **AES202**: Must import taxonomy and contract(aggregate). Partially checked.
- **AES405**: At least 1 class must inherit an aggregate ABC; max 3 classes per file. Partially checked.

---

## Phase 6: Surface Layer

> **Skill:** `aes-surface` — load for user-facing input translation.

Translates user-facing inputs into actions, delegating to the Agent
orchestrator via aggregate ABC.

### Surface Classification

| Category    | Suffixes                                      | Rules                                                                                                 |
| ------------- | ----------------------------------------------- | ------------------------------------------------------------------------------------------------------- |
| **Smart**   | `_command`, `_controller`, `_page`, `_router` | May contain orchestration logic. Global limit: 15 functions.                                          |
| **Utility** | `_hook`, `_store`, `_action`, `_screen`       | Supports smart surfaces. Max 10 methods, 80 lines/method, 3 nesting depth, 3 control-flow statements. |
| **Passive** | `_component`, `_view`, `_layout`, others      | Presentation only. Same limits as Utility.                                                            |

### Steps

1. Load `aes-surface` skill.
2. Create surface classes (commands, handlers, endpoints).
3. Inject aggregate ABC via constructor.
4. Verify: `lint-arwaky-cli scan modules/<surface_group>/src`.

### Example

```python
# modules/cli/src/surface_user_command.py
"""User command surface — delegates to aggregate."""

from modules.shared.src.user.contract_user_aggregate import IUserAggregate
from modules.shared.src.user.taxonomy_user_vo import UserId, UserResponse


class GetUserCommand:
    """Command to retrieve a user by ID."""

    def __init__(self, aggregate: IUserAggregate) -> None:
        self._aggregate = aggregate

    def execute(self, user_id: UserId) -> UserResponse:
        return self._aggregate.get_user(user_id)
```

### Rules Enforced

- **AES102**: Suffix must be in the surface allow-list. Machine-checked.
- **AES302**: File must be at least 5 lines. Machine-checked.
- **AES201**: Surface must not import agent, capabilities, contract(protocol). Partially checked.
- **AES406**: Function/method count, method length, nesting depth, control-flow limits. Partially checked.

---

## Phase 7: Root Layer

> **Skill:** `aes-root` — load for DI container and entry point wiring.

Wires concrete implementations to contracts and bootstraps the system.
Root is the **only layer** allowed to import all other layers.

### Steps

1. Load `aes-root` skill.
2. Create DI container wiring: capabilities → orchestrator → surface.
3. Create entry point at `modules/root_<name>_entry.py`.
4. Verify: `lint-arwaky-cli scan modules`.

### Example

```python
# modules/root_user_container.py
"""User DI container — wires all layers."""

from modules.shared.src.user.contract_user_protocol import IUserRepositoryProtocol
from modules.shared.src.user.contract_user_aggregate import IUserAggregate
from modules.user.src.capability_user_repository import UserRepository
from modules.agent_user_orchestrator import UserOrchestrator
from modules.cli.src.surface_user_command import GetUserCommand


class UserContainer:
    """DI container for user feature."""

    def __init__(self, db_connection: "DatabaseConnection") -> None:
        # Wire: capabilities → agent → surface
        repository: IUserRepositoryProtocol = UserRepository(db_connection)
        orchestrator: IUserAggregate = UserOrchestrator(repository)
        self.get_user_command = GetUserCommand(orchestrator)
```

```python
# modules/root_app_entry.py
"""Application entry point."""

import sys
import os

sys.path.insert(0, os.path.dirname(__file__))

from root_user_container import UserContainer


def main() -> None:
    db = create_database_connection()
    container = UserContainer(db)
    # start application...


if __name__ == "__main__":
    main()
```

> The `sys.path` insert plus the bare `from root_user_container import ...` is
> the entry-point convention in the reference workspace. It keeps the entry
> runnable as a script. Use absolute imports from the project root instead if
> your packaging setup requires it.

### Rules Enforced

- **AES102**: Suffix must be `entry` or `container`. Machine-checked.
- **AES201**: Root may import all layers. Partially checked.
- Root layer files are **skipped** by role-rules (AES401–406) and orphan-detector.

---

## Phase 8: Verify & CI Gate

> **Skill:** `aes-lint-arwaky` — load for final build verification.

### Step 1: Full AES Scan

```bash
lint-arwaky-cli scan .
```

**Target: 0 violations.**

### Step 2: Run Tests

```bash
pytest
```

### Step 3: External Lint

```bash
ruff check .
ruff format --check .
mypy modules/
bandit -r modules/
```

### Step 4: CI Gate

```bash
lint-arwaky-cli ci . --threshold 0
```

**Exit code 0** = all checks pass. **Exit code 1** = violations found.

---

## Import Rules Quick Reference

| Source Layer   | May Import                             | Must NOT Import                                       |
| ---------------- | ---------------------------------------- | ------------------------------------------------------- |
| `taxonomy`     | taxonomy                               | contract, utility, capabilities, agent, surface, root |
| `contract`     | taxonomy, contract                     | utility, capabilities, agent, surface, root           |
| `utility`      | taxonomy                               | contract, capabilities, agent, surface, root          |
| `capabilities` | taxonomy, contract, utility            | capabilities, agent, surface, root                    |
| `agent`        | taxonomy, contract, utility            | capabilities, surface, root                           |
| `surface`      | taxonomy, contract(aggregate), utility | agent, capabilities, contract(protocol), root         |
| `root`         | ALL layers                             | —                                                    |

This table is the target architecture. Some rows are not reliably
machine-checked — see Verify for which ones the linter actually enforces.

---

## Supplementary Skills (Post-Migration)

| Skill                      | When to Use                                            |
| ---------------------------- | --------------------------------------------------------- |
| `aes-docs`                 | Add docstrings, type hints after migration             |
| `aes-testing-suite`        | Generate test suites                                   |

---

## File Naming Reference

| Layer        | Pattern                              | Allowed Suffixes                                                                                              |
| -------------- | -------------------------------------- | --------------------------------------------------------------------------------------------------------------- |
| taxonomy     | `taxonomy_<concept>_<suffix>.py`     | `vo`, `entity`, `error`, `event`, `constant`                                                                  |
| contract     | `contract_<concept>_<suffix>.py`     | `protocol`, `aggregate`                                                                                       |
| utility      | `utility_<concept>_<suffix>.py`      | flexible (forbidden: `vo`, `entity`, `protocol`, `aggregate`)                                                  |
| capabilities | `capability_<concept>_<suffix>.py`   | flexible (forbidden: `vo`, `entity`, `protocol`, `aggregate`, `utility`)                                       |
| agent        | `agent_<concept>_orchestrator.py`    | `orchestrator`                                                                                                |
| surface      | `surface_<concept>_<suffix>.py`      | `command`, `controller`, `page`, `router`, `hook`, `store`, `action`, `screen`, `component`, `view`, `layout` |
| root         | `root_<concept>_<suffix>.py`         | `entry`, `container`                                                                                          |

---

## Troubleshooting

### Common Violations and Fixes

| Code        | Violation                                               | Fix                                                     |
| ------------- | --------------------------------------------------------- | --------------------------------------------------------- |
| AES101      | Filename not snake_case or < 3 words                    | Rename to `prefix_concept_suffix.py`                     |
| AES102      | Wrong suffix for layer                                  | Change suffix to match layer's allow-list               |
| AES201      | Forbidden cross-layer import                            | Route through contract layer; use constructor injection |
| AES202      | Missing mandatory import                                | Add required taxonomy/contract import                   |
| AES203      | Unused import                                           | Remove the import                                       |
| AES204      | Dummy function (`_use_*`, `dummy_*`)                    | Remove dummy function and the import it fakes           |
| AES205      | Circular dependency                                     | Break cycle via contract layer abstraction              |
| AES301      | File > 1000 lines                                       | Split into smaller files                                |
| AES302      | File < 5 lines                                          | Add a docstring and descriptive comments                |
| AES304      | `# type: ignore`, `# noqa`, `raise NotImplementedError` | Fix the type error; implement the method                |
| AES401      | Raw primitive in taxonomy                               | Wrap in Value Object (`@dataclass(frozen=True)`)        |
| AES403      | Capability missing protocol inheritance                 | Add `class Foo(IProtocol)`                               |
| AES404      | Class in utility file                                   | Move class to taxonomy; keep only `def` functions        |
| AES405      | Agent missing aggregate inheritance                     | Add `class Foo(IAggregate)`                              |
| AES406      | Too many functions in surface                           | Split into smaller surface files                        |
| AES501–506 | Orphan file                                             | Wire into container or remove                           |

### Parse Errors

If `lint-arwaky-cli` reports `PARSE_WARN` for a file, the file has a syntax
error that prevents AST parsing. Fix the syntax error first, then re-scan.

### Config Not Found

If no config file is found, lint-arwaky uses embedded defaults. Run
`lint-arwaky-cli init` to create an explicit config file.

### Python-Specific: Import Style

The reference workspace uses absolute imports from the `modules` root:

```python
# ✅ Correct — absolute import
from modules.shared.src.user.taxonomy_user_vo import UserId
```

Sibling taxonomy files may also use relative imports within the domain
package — the shared barrel does:

```python
# ✅ Acceptable inside the same domain package
from .taxonomy_user_vo import UserId
```

The `aes-taxonomy` templates use relative imports for sibling references and
the shared barrel does the same. Either style works; stay consistent within a
package.

---

## Section Contract

| Section | Why it belongs here |
| ------- | ------------------- |
| Rules (numbered list) | Each rule prevents one specific migration failure mode. |
| Workflow (phases 0–8) | Explicit phase order; agents know which layer they are on. |
| Dependency model diagram | Grounds every import decision in the diagram. |
| Workspace structure | Shows where every layer's files live in a Python workspace. |
| Phase examples | Copy-paste-ready skeletons so the agent never guesses the structure. |
| Rules Enforced per phase | Names the AES code and whether it is machine-checked. |
| Orphan reachability note | Prevents the Phase 1 → Phase 2 AES501 dead end. |
| Import matrix | Quick lookup for which layer may import which. |
| Troubleshooting table | Maps violation codes to specific fixes. |

---

## Verify

```bash
# Phase 0 — baseline
lint-arwaky-cli scan .

# Per phase (replace <dir> with the layer directory being migrated)
lint-arwaky-cli scan modules/shared/src/<domain>  # taxonomy + contract + utility
lint-arwaky-cli scan modules/<feature>/src       # capabilities
lint-arwaky-cli scan modules                     # agent + root + entry

# Final gate
lint-arwaky-cli scan .   # must be 0
python -m compileall -q modules   # compile fallback
pytest
```

### What the linter reliably catches

```text
AES101 — filename pattern
AES102 — layer suffix correctness
AES203 — unused imports
AES204 — dummy functions
AES205 — circular dependencies
AES301/AES302 — file length bounds
AES401 — non-constant declarations inside _constant files
AES501–AES506 — orphan files and reachability
```

### What is convention, not enforcement

```text
Import boundary rules (AES201/AES202) — the import matrix above is the target
  architecture. Do not treat a clean scan as proof the boundaries hold.

Primitive rules (AES401 on fields) — the auditor only inspects lines that end in
  punctuation (, ; } ) :) or contain an arrow (-> ). A conventional dataclass
  field written as `name: str` is skipped entirely. VO files are not scanned for
  primitives at all, since the check is gated on the _entity/_error/_event
  suffixes. Wrap domain fields in VOs by discipline, not by expectation of a
  violation.

Class and inheritance rules (AES403/AES404/AES405) — verify these by reading.
```

A pass means the mechanically-checkable rules are satisfied. The boundary and
primitive rules above are the reader's responsibility.

Related: [HOW-TO-MAKE-RUST-MIGRATION.md](HOW-TO-MAKE-RUST-MIGRATION.md), [HOW-TO-MAKE-TYPESCRIPT-MIGRATION.md](HOW-TO-MAKE-TYPESCRIPT-MIGRATION.md)
