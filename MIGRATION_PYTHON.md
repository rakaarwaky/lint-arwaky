# AES Migration Guide — Python

> Step-by-step guide for migrating a Python project to AES architecture.
> Workspace structure: `modules/` with pyproject.toml.

## Workspace Structure

```
project-root/
├── pyproject.toml           ← workspace root config
├── modules/
│   ├── shared/              ← shared taxonomy + contract types
│   │   ├── pyproject.toml
│   │   └── src/
│   │       ├── __init__.py
│   │       ├── taxonomy_common_vo.py
│   │       ├── contract_common_port.py
│   │       └── ...
│   ├── user/                ← feature module
│   │   ├── pyproject.toml
│   │   └── src/
│   │       ├── __init__.py
│   │       ├── taxonomy_user_vo.py
│   │       ├── taxonomy_user_error.py
│   │       ├── taxonomy_user_constant.py
│   │       ├── contract_user_port.py
│   │       ├── contract_user_protocol.py
│   │       ├── contract_user_aggregate.py
│   │       ├── capabilities_user_checker.py
│   │       ├── infrastructure_user_adapter.py
│   │       ├── agent_user_orchestrator.py
│   │       ├── surface_user_command.py
│   │       └── root_user_container.py
│   └── order/
│       └── src/
│           └── ...
└── src/
    └── root_cli_main_entry.py   ← CLI entry point (at workspace root)
```

**Key rules:**

- All 7 layers coexist in each feature module, differentiated by filename prefix.
- Entry points (`root_*_entry.py`) live at workspace root or `src/`.
- Shared types go in `modules/shared/`.

---

## Prerequisites

```bash
pip install lint-arwaky-cli
lint-arwaky-cli version
lint-arwaky-cli check your-project/
```

---

## Phase 0: Audit

```bash
lint-arwaky-cli check your-project/
find your-project/modules -name "*.py" | wc -l
```

---

## Phase 1: Taxonomy Layer

### Step 1.1: Identify Domain Types

```bash
grep -rn "^class " your-project/modules/*/src/ | grep -v test | grep -v __init__
```

### Step 1.2: Create Value Objects

**Before:** `modules/user/src/user.py` (class + logic mixed)
**After:** `modules/user/src/taxonomy_user_vo.py` (pure data)

```python
# modules/user/src/taxonomy_user_vo.py
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
# modules/user/src/taxonomy_user_constant.py
"""Application constants."""

MAX_RETRY_COUNT: int = 3
DEFAULT_TIMEOUT_MS: int = 5000
API_VERSION: str = "v1"
```

### Step 1.4: Create Error Types

```python
# modules/user/src/taxonomy_user_error.py
"""Domain error types."""

from typing import Optional


class UserError(Exception):
    def __init__(self, message: str, code: Optional[str] = None):
        super().__init__(message)
        self.code = code


class UserNotFoundError(UserError):
    def __init__(self, user_id: str):
        super().__init__(f"User not found: {user_id}", code="USER_NOT_FOUND")


class InvalidEmailError(UserError):
    def __init__(self, email: str):
        super().__init__(f"Invalid email: {email}", code="INVALID_EMAIL")
```

---

## Phase 2: Contract Layer

### Step 2.1: Identify Outbound Dependencies

```bash
grep -rn "open(\|requests\.\|sqlite3\|redis\|psycopg\|pymongo" your-project/modules/*/src/
```

### Step 2.2: Create Ports (outbound interfaces)

```python
# modules/user/src/contract_user_port.py
"""Outbound port for user persistence — implemented by infrastructure."""

from abc import ABC, abstractmethod
from typing import Optional
from .taxonomy_user_vo import UserVO


class IUserPort(ABC):
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

### Step 2.3: Create Protocols (inbound interfaces)

```python
# modules/user/src/contract_user_protocol.py
"""Inbound protocol for user operations — implemented by capabilities."""

from abc import ABC, abstractmethod
from .taxonomy_user_vo import UserVO


class IUserProtocol(ABC):
    @abstractmethod
    def get_user(self, user_id: str) -> UserVO:
        ...

    @abstractmethod
    def create_user(self, name: str, email: str) -> UserVO:
        ...
```

### Step 2.4: Create Aggregates (facades)

```python
# modules/user/src/contract_user_aggregate.py
"""Aggregate facade — combines user protocols for surface layer."""

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

## Phase 3: Capabilities Layer

Business logic only. No infrastructure imports.

```python
# modules/user/src/capabilities_user_checker.py
"""Validates user domain rules — pure business logic."""

from .contract_user_protocol import IUserProtocol
from .taxonomy_user_vo import UserVO


class UserChecker:
    def __init__(self, user_protocol: IUserProtocol):
        self._user_protocol = user_protocol

    def validate_email(self, email: str) -> bool:
        return "@" in email and "." in email

    def check_unique_email(self, email: str) -> bool:
        existing = self._user_protocol.get_by_email(email)
        return existing is None
```

---

## Phase 4: Infrastructure Layer

Each adapter implements a port.

```python
# modules/user/src/infrastructure_user_adapter.py
"""User persistence adapter — implements IUserPort for database."""

from typing import Optional
from .contract_user_port import IUserPort
from .taxonomy_user_vo import UserVO


class UserAdapter(IUserPort):
    def __init__(self, db_path: str):
        self._db_path = db_path

    def find_by_id(self, user_id: str) -> Optional[UserVO]:
        # Actual database call here
        raise NotImplementedError("Implement database query")

    def save(self, user: UserVO) -> None:
        raise NotImplementedError("Implement database insert/update")

    def delete(self, user_id: str) -> None:
        raise NotImplementedError("Implement database delete")
```

---

## Phase 5: Agent Layer

Orchestrator coordinates capabilities and infrastructure.

```python
# modules/user/src/agent_user_orchestrator.py
"""User orchestration — coordinates user-related operations."""

import uuid
from .contract_user_aggregate import IUserAggregate
from .contract_user_port import IUserPort
from .capabilities_user_checker import UserChecker
from .taxonomy_user_vo import UserVO
from .taxonomy_user_error import InvalidEmailError, UserNotFoundError


class UserOrchestrator(IUserAggregate):
    def __init__(self, checker: UserChecker, port: IUserPort):
        self._checker = checker
        self._port = port

    def get_user(self, user_id: str) -> UserVO:
        user = self._port.find_by_id(user_id)
        if user is None:
            raise UserNotFoundError(user_id)
        return user

    def create_user(self, name: str, email: str) -> UserVO:
        if not self._checker.validate_email(email):
            raise InvalidEmailError(email)
        user = UserVO(id=str(uuid.uuid4()), name=name, email=email)
        self._port.save(user)
        return user

    def delete_user(self, user_id: str) -> None:
        self._port.delete(user_id)
```

---

## Phase 6: Surface Layer

CLI commands, API handlers. Delegates to orchestrator.

```python
# modules/user/src/surface_user_command.py
"""CLI command for user operations."""

from typing import List
from .contract_user_aggregate import IUserAggregate


class UserCommand:
    def __init__(self, orchestrator: IUserAggregate):
        self._orchestrator = orchestrator

    def run(self, args: List[str]) -> str:
        if not args:
            return "Usage: user <get|create> [args...]"

        action = args[0]
        if action == "get":
            user_id = args[1] if len(args) > 1 else ""
            user = self._orchestrator.get_user(user_id)
            return f"User: {user.name} <{user.email}>"
        elif action == "create":
            name = args[1] if len(args) > 1 else ""
            email = args[2] if len(args) > 2 else ""
            user = self._orchestrator.create_user(name, email)
            return f"Created: {user.id}"
        else:
            return f"Unknown action: {action}"
```

---

## Phase 7: Root Layer

DI container wires everything. Entry point bootstraps.

### Container (inside feature module)

```python
# modules/user/src/root_user_container.py
"""DI container — wires all user-related dependencies."""

from .infrastructure_user_adapter import UserAdapter
from .capabilities_user_checker import UserChecker
from .agent_user_orchestrator import UserOrchestrator


class UserContainer:
    def __init__(self, db_path: str):
        self._port = UserAdapter(db_path)
        self._checker = UserChecker(user_protocol=self._port)
        self._orchestrator = UserOrchestrator(
            checker=self._checker,
            port=self._port,
        )

    @property
    def orchestrator(self) -> UserOrchestrator:
        return self._orchestrator
```

### Entry Point (at workspace root)

```python
# src/root_cli_main_entry.py
"""CLI entry point — bootstraps the application."""

import sys
from modules.user.src.root_user_container import UserContainer
from modules.user.src.surface_user_command import UserCommand


def main():
    container = UserContainer("data.db")
    command = UserCommand(container.orchestrator)

    args = sys.argv[1:]
    try:
        result = command.run(args)
        print(result)
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
```

---

## Phase 8: Verify

```bash
lint-arwaky-cli check your-project/
python -m pytest
ruff check .
mypy .
```

---

## File Naming Reference

| Layer          | Pattern                                | Example                          |
| -------------- | -------------------------------------- | -------------------------------- |
| taxonomy       | `taxonomy_<concept>_<suffix>.py`       | `taxonomy_user_vo.py`            |
| contract       | `contract_<concept>_<suffix>.py`       | `contract_user_port.py`          |
| capabilities   | `capabilities_<concept>_<suffix>.py`   | `capabilities_user_checker.py`   |
| infrastructure | `infrastructure_<concept>_<suffix>.py` | `infrastructure_user_adapter.py` |
| agent          | `agent_<concept>_orchestrator.py`      | `agent_user_orchestrator.py`     |
| surface        | `surface_<concept>_<suffix>.py`        | `surface_user_command.py`        |
| root           | `root_<concept>_<suffix>.py`           | `root_user_container.py`         |

---

## Import Rules

```
taxonomy_       → taxonomy_*
contract_       → taxonomy_*, contract_*
capabilities_   → taxonomy_*, contract_*
infrastructure_ → taxonomy_*, contract_*
agent_          → taxonomy_*, contract_aggregate_*, contract_port_*, contract_protocol_*
surface_        → taxonomy_*, contract_aggregate_*
root_           → ALL layers
```

**NEVER:** capabilities → infrastructure, agent → surface, surface → capabilities.

---

## Troubleshooting

| Violation  | Fix                                               |
| ---------- | ------------------------------------------------- |
| AES101     | Rename to `layer_concept_suffix`                  |
| AES102     | Change suffix to match layer's allowed list       |
| AES201     | Remove forbidden import, use contract interface   |
| AES202     | Add missing import per layer requirements         |
| AES303     | Add class/function definition                     |
| AES304     | Remove `# noqa`, `type: ignore`                   |
| AES401     | Move primitives to VO, constants to `_constant`   |
| AES402     | Replace primitive types with VO types in contract |
| AES403     | Implement protocol class in capability            |
| AES404     | Implement port class in infrastructure            |
| AES501-506 | Wire in container or remove dead code             |
