# AES403 — Capabilities Roles

The `CapabilitiesRoleChecker` enforces 3 rules per file.

## Rule 1: Internal Helpers Are Allowed

Classes without ABC inheritance are **allowed** and never flagged. These are internal helper types that don't implement protocol behavior. They must not start with `_`.

```python
# ✅ ALLOWED — internal helper, no ABC inheritance needed
class Cache:
    def __init__(self):
        self.data = []
```

## Rule 2: At Least One Implementor Required

The file MUST have at least one class that inherits from a protocol ABC. If no class inherits an ABC → flag `CapabilityNoImplementor` (AES403, Severity MEDIUM).

```python
# ✅ PASSING — inherits IAgentRoleChecker
class MyChecker(IAgentRoleChecker):
    def check_container(self, source, violations):
        pass

# ❌ FAILING — no ABC inheritance
class HelperA:
    def do_a(self):
        pass

class HelperB:
    def do_b(self):
        pass
```

## Rule 3: Maximum 3 Types Per File

Total class count must not exceed **3**. If exceeded → flag `CapabilityTooManyTypes` (AES403, Severity HIGH).

```python
# ✅ PASSING — exactly 3 classes
class Cap(IAgentRoleChecker):
    def check_container(self, source, violations):
        pass

class Helper:
    def do_work(self):
        pass

class Status:
    pass

# ❌ FAILING — 4 classes exceeds limit
class Cap(IAgentRoleChecker):
    def check_container(self, source, violations):
        pass

class A:
    pass

class B:
    pass

class C:
    pass
```

## Detection Patterns

| Language | Implementor Pattern | Non-Implementor |
|----------|-------------------|-----------------|
| Rust | `impl Trait for Struct` | Standalone `impl Struct` |
| Python | `class Name(Protocol):` | Standalone `class Name:` |
| TS | `class Name implements IProto` | Standalone `class Name {}` |

## Guard Check

The guard check requires `_protocol` import only:

```python
# ✅ Guard passes — imports from _protocol module
from.*capabilities_|from.*agent_|from.*surface_role_protocol import IAgentRoleChecker

# ❌ Guard fails — no _protocol import → CapabilityNoProtocol
class MyChecker:
    def do_work(self):
        pass
```
