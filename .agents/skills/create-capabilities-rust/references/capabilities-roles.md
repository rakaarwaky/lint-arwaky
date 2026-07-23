# AES403 — Capabilities Roles

The `CapabilitiesRoleChecker` enforces 3 rules per file.

## Rule 1: Internal Helpers Are Allowed

Structs without a trait impl are **allowed** and never flagged. These are internal helper types (cache, builder, config) that don't implement protocol behavior. They must not start with `_`.

```rust
// ✅ ALLOWED — internal helper, no trait impl needed
struct Cache {
    data: Vec<u8>,
}

impl Cache {
    fn new() -> Self {
        Self { data: vec![] }
    }
}
```

## Rule 2: At Least One Implementor Required

The file MUST have at least one struct that implements a protocol trait via `impl <Trait> for <StructName>`. If no struct implements a protocol trait → flag `CapabilityNoImplementor` (AES403, Severity MEDIUM).

```rust
// ✅ PASSING — implements IAgentRoleChecker
pub struct MyChecker;

impl IAgentRoleChecker for MyChecker {
    fn check_container(&self, _s: &str, _v: &mut Vec<LintResult>) {}
}

// ❌ FAILING — no impl Trait for Struct
struct HelperA { x: i32 }
struct HelperB { y: String }
```

## Rule 3: Maximum 3 Types Per File

Total struct + enum count must not exceed **3**. If exceeded → flag `CapabilityTooManyTypes` (AES403, Severity HIGH). This includes all public/private structs and enums in the file.

```rust
// ✅ PASSING — exactly 3 types (1 impl + 1 struct + 1 enum)
pub struct Cap {}
impl IChecker for Cap {}

struct Helper {}
enum Status { Active, Inactive }

// ❌ FAILING — 4 types exceeds limit
pub struct Cap {}
impl IChecker for Cap {}
struct A {}
struct B {}
enum C { X, Y }
```

## Detection Patterns

| Language | Implementor Pattern            | Non-Implementor           |
| ---------- | -------------------------------- | --------------------------- |
| Rust     | `impl Trait for Struct`        | Standalone`impl Struct`   |
| Python   | `class Name(Protocol):`        | Standalone`class Name:`   |
| TS       | `class Name implements IProto` | Standalone`class Name {}` |

## Guard Check

The guard check requires `_protocol` import only:

```rust
// ✅ Guard passes — imports from _protocol module
use shared::role_rules::contract_agent_role_protocol::IAgentRoleChecker;

// ❌ Guard fails — no _protocol import → CapabilityNoProtocol
struct MyChecker;
impl Default for MyChecker {}
```
