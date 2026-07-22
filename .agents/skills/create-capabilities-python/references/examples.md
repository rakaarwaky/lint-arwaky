# Examples

## BAD: Capability Without Protocol (AES403)

````python
class <NameComposer>:

    def compose_frame(self) -> None:
        # public behavior without protocol ABC
        ...

Fix:

```python
class <NameComposer>(I<NameComposer>Protocol):
    def compose_frame(self) -> None:
        # contract implementation
        ...
````

## BAD: I/O in Capabilities (AES404)

```python
class <NameCapability>:
    def process(self) -> None:
        content = open("file.txt").read()  # FORBIDDEN
```

## BAD: Data Class Defined in Layer File

```python
@dataclass
class <NameResult>:
    is_valid: bool
    reason: str
```

Fix: Move to shared taxonomy, then import.

## BAD: Concrete Service Field

```python
class Capabilities<NameCapability>:
    def __init__(self, collaborator: <NameCollaborator>):  # BAD
        self._collaborator = collaborator
```

Fix:

```python
class Capabilities<NameCapability>:
    def __init__(self, collaborator: I<NameCollaborator>Protocol):
        self._collaborator = collaborator
```

## BAD: Orchestration Inside Capability (No Orchestration, §8)

```python
class <NamePipeline>(I<NameCapability>Protocol):
    def run(self) -> None:
        a = self.step_a()          # calls another capability's behavior
        if a.is_ok():
            self.step_b()          # branching between capabilities
        else:
            self.escalate()        # error-escalation policy
```

Fix: remove flow control and cross-capability calls. Let the Agent layer compose the pipeline. The capability executes one responsibility and returns a result.

## BAD: Domain Model Defined in Capability (No Domain Definition, §8)

```python
@dataclass
class <NameResult>:    # domain model defined here = forbidden
    is_valid: bool
    reason: str
```

Fix: define `<NameResult>` as a Taxonomy VO; the capability only consumes and produces it.

## BAD: Dunder Methods in Block 2

```python
class Capabilities<NameCapability>(I<NameCapability>Protocol):
    def __init__(self) -> None: ...

    def __repr__(self) -> str:           # ← Block 2 position, NOT a protocol method
        return "Capabilities<NameCapability>()"

    def execute(self, ...) -> None:  # ← pushed down
        ...
```

Fix: Move `__repr__` to Block 3.

## GOOD: Capability with DI and Shared VO

```python
from shared.<name-feature>.taxonomy_<name-policy>_vo import <NamePolicy>VO
from shared.<name-feature>.contract_<name-store>_protocol import I<NameStore>Protocol
from shared.<name-feature>.contract_<name-collaborator>_protocol import I<NameCollaborator>Protocol
from shared.<name-feature>.contract_<name-capability>_protocol import I<NameCapability>Protocol

class Capabilities<NameCapability>(I<NameCapability>Protocol):
    def __init__(
        self,
        collaborator: I<NameCollaborator>Protocol,
        store: I<NameStore>Protocol,
        policy: <NamePolicy>VO,
    ):
        self._collaborator = collaborator
        self._store = store
        self._policy = policy
```

## GOOD: Correct 3-Block Structure

```python
from shared.<name-feature>.taxonomy_<domain>_vo import <DomainVO>
from shared.<name-feature>.contract_<name-capability>_protocol import I<NameCapability>Protocol
from shared.<name-feature>.taxonomy_<name-utility> import <name>_utility
from shared.<name-feature>.taxonomy_<result>_vo import <ResultVO>

# ─── Block 1: Class Definition & Constructor ──────────────
class Capabilities<NameCapability>(I<NameCapability>Protocol):
    def __init__(self) -> None:
        pass

    # ─── Block 2: Public Contract (domain protocol ONLY) ──
    def execute(
        self,
        input: <DomainVO>,
        output: list[<ResultVO>],
    ) -> None:
        key = input.key()
        if <name>_utility(key):
            return
        # Remaining domain logic...

    # ─── Block 3: Dunder Methods, Factories & Helpers ─────
    def __repr__(self) -> str:
        return "Capabilities<NameCapability>()"

    def __eq__(self, other: object) -> bool:
        return isinstance(other, Capabilities<NameCapability>)

    @classmethod
    def create_default(cls) -> "Capabilities<NameCapability>":
        return cls()
```
