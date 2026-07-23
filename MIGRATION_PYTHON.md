# AES Migration Guide — Python

> Step-by-step guide for migrating a Python project to AES architecture.
> Workspace structure: `modules/` with pyproject.toml.

See [ARCHITECTURE.md](ARCHITECTURE.md) for layer rules and [README.md](README.md) for project usage.

## Workspace Structure

```
project-root/
├── pyproject.toml           ← workspace root config
├── modules/
│   ├── shared/              ← shared taxonomy + contract + utility types
│   │   ├── pyproject.toml
│   │   └── src/
│   │       ├── __init__.py
│   │       ├── common/          ← truly shared across ALL features
│   │       │   ├── __init__.py
│   │       │   ├── taxonomy_common_vo.py
│   │       │   ├── taxonomy_path_vo.py
│   │       │   └── ...
│   │       └── user/            ← shared types for user feature (domain folder)
│   │           ├── __init__.py
│   │           ├── taxonomy_user_vo.py
│   │           ├── taxonomy_user_error.py
│   │           ├── taxonomy_user_constant.py
│   │           ├── contract_user_protocol.py
│   │           ├── contract_user_aggregate.py
│   │           └── utility_user_hasher.py
│   │
│   ├── user/                ← feature module
│   │   ├── pyproject.toml
│   │   └── src/
│   │       ├── __init__.py
│   │       ├── capabilities_user_checker.py     ← business logic capability
│   │       ├── capabilities_user_repository.py  ← external adaptation capability
│   │       ├── agent_user_orchestrator.py       ← agent layer (orchestrator)
│   │       ├── surface_user_command.py          ← surfaces layer
│   │       └── root_user_container.py           ← root container
│   └── order/
│       └── src/
│           └── ...
└── src/
    └── root_cli_main_entry.py   ← CLI entry point (at workspace root)
```

**Key rules:**

- All 7 layers coexist in each feature slice. Stable domain taxonomy, contracts, and utilities live under `modules/shared/src/<feature>/`. Orchestration, capabilities, and surfaces live in the feature module.
- Entry points (`root_*_entry.py`) live at workspace root or `src/`.
- Shared types go in `modules/shared/`.

---

## Prerequisites

```bash
pip install lint-arwaky-cli
lint-arwaky-cli version
lint-arwaky-cli scan your-project/
```

---

## Phase 0: Audit

```bash
lint-arwaky-cli scan your-project/
find your-project/modules -name "*.py" | wc -l
```

- Violations < 10 → full migration in one session
- Violations 10-50 → phased migration
- Violations > 50 → start with taxonomy only

---

## Phase 1: Taxonomy Layer

Define Value Objects, Errors, Events, and compile-time Constants under the `shared` member.

### Step 1.1: Identify Domain Types

```bash
grep -rn "^class " your-project/modules/*/src/ | grep -v test | grep -v __init__
```

### Step 1.2: Create Value Objects

```python
# modules/shared/src/user/taxonomy_user_vo.py
"""User value object — immutable domain data container."""

from dataclasses import dataclass


@dataclass(frozen=True)
class UserVO:
    id: str
    name: str
    email: str
```

### Step 1.3: Create Constants

```python
# modules/shared/src/user/taxonomy_user_constant.py
"""User constants — compile-time literal values."""

MAX_RETRY_COUNT = 3
DEFAULT_TIMEOUT_MS = 5000
```

### Step 1.4: Create Error Types

```python
# modules/shared/src/user/taxonomy_user_error.py
"""User domain-level errors."""


class UserError(Exception):
    """Base error for user domain."""


class UserNotFoundError(UserError):
    def __init__(self, user_id: str):
        super().__init__(f"User not found: {user_id}")


class InvalidEmailError(UserError):
    def __init__(self, email: str):
        super().__init__(f"Invalid email: {email}")
```

---

## Phase 2: Contract Layer

Contracts define public interfaces (Protocols and Aggregates) without exposing implementation.

### Step 2.1: Create Protocols (inbound/outbound interfaces)

Define protocol interfaces implemented by Capabilities (both business calculation and external adapters) and consumed by the Agent.

```python
# modules/shared/src/user/contract_user_protocol.py
"""User contract protocols."""

from abc import ABC, abstractmethod
from typing import Optional
from .taxonomy_user_vo import UserVO


class IUserProtocol(ABC):
    @abstractmethod
    def check_valid_email(self, email: str) -> bool:
        ...


class IUserRepositoryProtocol(ABC):
    @abstractmethod
    def find_by_id(self, user_id: str) -> Optional[UserVO]:
        ...

    @abstractmethod
    def save(self, user: UserVO) -> None:
        ...

    @abstractmethod
    def delete(self, user_id: str) -> None:
        ...
```

### Step 2.2: Create Aggregates (facades)

Define aggregate facades implemented by the Agent and consumed by Surfaces.

```python
# modules/shared/src/user/contract_user_aggregate.py
"""User contract aggregate facade."""

from abc import ABC, abstractmethod
from .taxonomy_user_vo import UserVO


class IUserAggregate(ABC):
    @abstractmethod
    def get_user(self, user_id: str) -> UserVO:
        ...

    @abstractmethod
    def create_user(self, name: str, email: str) -> UserVO:
        ...

    @abstractmethod
    def delete_user(self, user_id: str) -> None:
        ...
```

---

## Phase 3: Utility Layer

Utility contains low-level technical mechanics. It must contain only **stateless standalone functions** (no stateful objects, no behavior, no contract implementation, and no business decisions).

### Step 3.1: Create Technical Utilities

Extract reusable technical actions (e.g. parsing, hash computation, formatting) into the Utility layer inside the `shared` member.

```python
# modules/shared/src/user/utility_user_hasher.py
"""User utility functions."""


def hash_user_token(input_str: str) -> str:
    # stateless technical operation
    return f"hash_{input_str}"
```

---

## Phase 4: Capabilities Layer

Capabilities contain concrete behavior implementations. This includes business logic (validations, computations) and external adaptation (database repositories, network integration, third-party clients).

- Must implement one domain protocol ABC defined in Contract.
- Must use dependency injection for collaborator services.
- Must not import or depend on other Capabilities.

### Step 4.1: Create Business Logic Capability

```python
# modules/user/src/capabilities_user_checker.py
"""Validates user domain rules — pure business logic."""

from shared.user.contract_user_protocol import IUserProtocol


class UserChecker(IUserProtocol):
    def check_valid_email(self, email: str) -> bool:
        return "@" in email and "." in email
```

### Step 4.2: Create External Adaptation Capability (formerly Infrastructure)

```python
# modules/user/src/capabilities_user_repository.py
"""User persistence repository — implements IUserRepositoryProtocol for database."""

from typing import Optional
from shared.user.contract_user_protocol import IUserRepositoryProtocol
from shared.user.taxonomy_user_vo import UserVO


class UserRepository(IUserRepositoryProtocol):
    def __init__(self, db_path: str):
        self._db_path = db_path

    def find_by_id(self, user_id: str) -> Optional[UserVO]:
        # Actual database call here using local state or shared utilities
        raise NotImplementedError("Query DB")

    def save(self, user: UserVO) -> None:
        raise NotImplementedError("Insert/update user")

    def delete(self, user_id: str) -> None:
        raise NotImplementedError("Delete user")
```

---

## Phase 5: Agent Layer

Orchestrates sequential execution, branching, looping, and error handling. Ignorant of concrete capability and utility implementations (coordinates only via contract protocols injected at constructor time).

```python
# modules/user/src/agent_user_orchestrator.py
"""User orchestration — coordinates user-related operations."""

import uuid
from shared.user.contract_user_aggregate import IUserAggregate
from shared.user.contract_user_protocol import IUserProtocol, IUserRepositoryProtocol
from shared.user.taxonomy_user_vo import UserVO
from shared.user.taxonomy_user_error import InvalidEmailError, UserNotFoundError


class UserOrchestrator(IUserAggregate):
    def __init__(self, checker: IUserProtocol, repository: IUserRepositoryProtocol):
        self._checker = checker
        self._repository = repository

    def get_user(self, user_id: str) -> UserVO:
        user = self._repository.find_by_id(user_id)
        if user is None:
            raise UserNotFoundError(user_id)
        return user

    def create_user(self, name: str, email: str) -> UserVO:
        if not self._checker.check_valid_email(email):
            raise InvalidEmailError(email)
        user = UserVO(id=str(uuid.uuid4()), name=name, email=email)
        self._repository.save(user)
        return user

    def delete_user(self, user_id: str) -> None:
        self._repository.delete(user_id)
```

---

## Phase 6: Surface Layer

Translates user-facing inputs into actions, delegating execution to the Agent orchestrator.

```python
# modules/user/src/surface_user_command.py
"""CLI command surface for user operations."""

from typing import List
from shared.user.contract_user_aggregate import IUserAggregate


class UserCommand:
    def __init__(self, orchestrator: IUserAggregate):
        self._orchestrator = orchestrator

    def run(self, args: List[str]) -> str:
        if not args:
            return "Usage: user <get|create> [args...]"

        action = args[0]
        if action == "get":
            if len(args) < 2:
                raise ValueError("Missing user ID")
            user = self._orchestrator.get_user(args[1])
            return f"User: {user.name} <{user.email}>"
        elif action == "create":
            if len(args) < 3:
                raise ValueError("Missing name or email")
            user = self._orchestrator.create_user(args[1], args[2])
            return f"Created user: {user.id}"
        else:
            return "Usage: user <get|create> [args...]"
```

---

## Phase 7: Root Layer

Wires concrete implementations to contracts and bootstraps the system.

### Container

```python
# modules/user/src/root_user_container.py
"""User feature DI container."""

from .agent_user_orchestrator import UserOrchestrator
from .capabilities_user_checker import UserChecker
from .capabilities_user_repository import UserRepository
from shared.user.contract_user_aggregate import IUserAggregate


class UserContainer:
    def __init__(self, db_path: str):
        checker = UserChecker()
        repository = UserRepository(db_path)
        self._orchestrator = UserOrchestrator(checker, repository)

    @property
    def orchestrator(self) -> IUserAggregate:
        return self._orchestrator
```

### Entry Point

```python
# src/root_cli_main_entry.py
"""CLI Main entry point."""

import sys
from modules.user.src.root_user_container import UserContainer
from modules.user.src.surface_user_command import UserCommand


def main() -> None:
    container = UserContainer("data.db")
    command = UserCommand(container.orchestrator)

    args = sys.argv[1:]
    try:
        output = command.run(args)
        print(output)
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)


if __name__ == "__main__":
    main()
```

---

## Phase 8: Verify

```bash
lint-arwaky-cli scan your-project/
pytest
black --check . && flake8 .
```

---

## File Naming Reference

| Layer        | Pattern                              | Example                        |
| ------------ | ------------------------------------ | ------------------------------ |
| taxonomy     | `taxonomy_<concept>_<suffix>.py`     | `taxonomy_user_vo.py`          |
| contract     | `contract_<concept>_<suffix>.py`     | `contract_user_protocol.py`    |
| utility      | `utility_<concept>_<suffix>.py`      | `utility_user_hasher.py`       |
| capabilities | `capabilities_<concept>_<suffix>.py` | `capabilities_user_checker.py` |
| agent        | `agent_<concept>_orchestrator.py`    | `agent_user_orchestrator.py`   |
| surface      | `surface_<concept>_<suffix>.py`      | `surface_user_command.py`      |
| root         | `root_<concept>_<suffix>.py`         | `root_user_container.py`       |

---

## Import Rules

```
taxonomy_     → taxonomy_*
contract_     → taxonomy_*
utility_      → taxonomy_*
capabilities_ → taxonomy_*, contract_*, utility_*
agent_        → taxonomy_*, contract_*
surface_      → taxonomy_*, contract_*, utility_*
root_         → ALL layers
```

**NEVER:** capabilities → agent, agent → surface, surface → capabilities, capability → capability.

---

## Troubleshooting

| Violation  | Fix                                               |
| ---------- | ------------------------------------------------- |
| AES101     | Rename to `layer_concept_suffix`                  |
| AES102     | Change suffix to match layer's allowed list       |
| AES201     | Remove forbidden import, use contract interface   |
| AES202     | Add missing import per layer requirements         |
| AES303     | Add struct/enum/trait definition                  |
| AES304     | Remove `#[allow]`, `unwrap()`, `panic!`           |
| AES401     | Move primitives to VO, constants to `_constant`   |
| AES402     | Replace primitive types with VO types in contract |
| AES403     | Implement protocol trait in capability            |
| AES404     | Move stateless helper functions to Utility        |
| AES501-506 | Wire in container or remove dead code             |
