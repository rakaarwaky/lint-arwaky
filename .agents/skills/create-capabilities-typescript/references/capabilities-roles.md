# AES403 — Capabilities Roles

The `CapabilitiesRoleChecker` enforces 3 rules per file.

## Rule 1: Internal Helpers Are Allowed

Classes without `implements` are **allowed** and never flagged. These are internal helper types that don't implement protocol behavior. They must not start with `_`.

```typescript
// ✅ ALLOWED — internal helper, no implements keyword needed
class Cache {
  private data: Array<string>;
}
```

## Rule 2: At Least One Implementor Required

The file MUST have at least one class that implements a protocol interface via `implements`. If no class implements an interface → flag `CapabilityNoImplementor` (AES403, Severity MEDIUM).

```typescript
// ✅ PASSING — implements IAgentRoleChecker
class MyChecker implements IAgentRoleChecker {
  checkContainer(source: string, violations: any[]) {}
}

// ❌ FAILING — no implements keyword
class HelperA {
  doA() {}
}

class HelperB {
  doB() {}
}
```

## Rule 3: Maximum 3 Types Per File

Total type declarations (class, interface, enum) must not exceed **3**. If exceeded → flag `CapabilityTooManyTypes` (AES403, Severity HIGH).

**Important:** `type` aliases are NOT counted — they are not new types.

```typescript
// ✅ PASSING — exactly 3 types (1 class + 1 interface + 1 enum)
class Cap implements IAgentRoleChecker {
  checkContainer(source: string, violations: any[]) {}
}

interface Helper {}

enum Status { Active, Inactive }

// ❌ FAILING — 4 types exceeds limit
class Cap implements IAgentRoleChecker {
  checkContainer(source: string, violations: any[]) {}
}

class A {}
class B {}
enum C { X, Y }
```

## Detection Patterns

| Language | Implementor Pattern | Non-Implementor |
|----------|-------------------|-----------------|
| Rust | `impl Trait for Struct` | Standalone `impl Struct` |
| Python | `class Name(Protocol):` | Standalone `class Name:` |
| TS | `class Name implements IProto` | Standalone `class Name {}` |

## Guard Check

The guard check requires `_protocol` import only:

```typescript
// ✅ Guard passes — imports from _protocol module
import { IAgentRoleChecker } from 'shared/role_rules/contract_agent_role_protocol';

// ❌ Guard fails — no _protocol import → CapabilityNoProtocol
export class MyChecker {
  doWork() {}
}
```
