# HOW TO MAKE TAXONOMY RUST

> **Purpose**: Define the stable language of the domain: value objects, entities, errors, events, and constants.
>
> **Audience**: Agents and engineers scaffolding AES taxonomy files in the shared domain.
>
> **Scope**: Rust taxonomy files named `taxonomy_<domain>_<concept>_<suffix>.rs` using suffixes `_vo`, `_entity`, `_error`, `_event`, `_constant` only.
>
> **Location**: Rust shared domain source root next to contracts:
>
> ```text
> crates/shared/src/<domain>/
> ```
>
> Rust taxonomy files must be registered in the domain `mod.rs`, which is the Rust shared barrel.
>
> **Length**: One taxonomy type per file. Constants files must be at least 5 lines per AES302. No I/O, no upward imports, no primitives for public/domain fields.

---

## Rules

### Import rules

**Allowed imports:**

```text
Other taxonomy types
Pure std modules (e.g., std::fmt, std::result)
thiserror (for error derivation)
serde (for serialization, if needed)
```

**Forbidden imports:**

```text
capabilities
agents
surface
root
contracts
std::fs
std::net
std::process
database crates
I/O crates
once_cell
lazy_static
any crate with side effects
```

`once_cell` and `lazy_static` are forbidden because they introduce runtime
global state into a taxonomy layer that should hold only pure values and types.

Taxonomy files must not perform I/O, network access, database access, filesystem access, environment access, randomness, or time retrieval.

---

### File-name pattern

Use:

```text
taxonomy_<domain>_<concept>_<suffix>.rs
```

Where:

```text
<domain>  = snake_case domain name
<concept> = snake_case concept name; may contain multiple underscores
<suffix>  = one of the allowed taxonomy suffixes
```

The pattern has three placeholders after `taxonomy`, but because `<concept>` may itself be multi-word, the final filename may contain more than three underscore-separated segments.

Examples:

```text
taxonomy_order_order_id_vo.rs
taxonomy_order_money_vo.rs
taxonomy_order_order_entity.rs
taxonomy_order_order_not_found_error.rs
taxonomy_order_order_created_event.rs
taxonomy_order_order_constant.rs
```

The filename must match the taxonomy type it contains.

Example mapping:

```text
taxonomy_order_order_id_vo.rs           -> OrderId
taxonomy_order_order_entity.rs          -> Order
taxonomy_order_order_not_found_error.rs -> OrderError
taxonomy_order_order_created_event.rs   -> OrderCreated
taxonomy_order_order_constant.rs        -> ORDER_* constants
```

Allowed suffixes are exactly:

```text
vo
entity
error
event
constant
```

Anything else fails AES102.

---

### File-name suffix table

| Suffix | Content | Key constraint |
|---|---|---|
| `_vo.rs` | Value Objects | Validate in `new()`, immutable fields, no I/O |
| `_entity.rs` | Entities with identity | Identity VO field required; domain fields must be VOs |
| `_error.rs` | Domain errors | Implement `std::error::Error` + `Display`; store VO fields only |
| `_event.rs` | Domain events | Immutable, VO payload fields only |
| `_constant.rs` | Compile-time constants | `pub const` only — no functions, no I/O |

---

### VO primitive rules (AES401)

AES401 forbids raw primitives as public/domain fields.

There is no `bool` carve-out. `bool` is treated as a primitive.

The implementation primitive set is broader than the most common examples. It includes at least:

```text
String
&str
i8..i128
u8..u128
isize
usize
f32
f64
bool
Vec
HashMap
HashSet
Option
Box
Rc
Arc
```

Public/domain fields in entities, events, errors, and composite VOs must be taxonomy VOs, not primitives.

A leaf VO may privately encapsulate a primitive value (like `String` or `i64`) for validation and representation. That internal encapsulation is a modeling convention.

**Important limitation:** AES401 does **not** scan `_vo` files for primitives. The check is gated on the `_entity`/`_error`/`_event` suffixes. A primitive inside a `_vo` file will not trigger a violation. Trust discipline, not enforcement, for VOs.

Constants are the explicit exception for literal values because constant files contain compile-time literals, not domain fields.

---

### File length (AES302)

Every taxonomy file must be at least **5 lines** or the linter reports:

```text
[AES302] FILE_TOO_SHORT: File contains fewer than the required minimum lines.
FIX: Expand the component or merge this logic into a related module. (min: 5).
```

Doc comments count. For constants, each constant should have a descriptive comment line above it — this keeps the file above the minimum while also documenting intent.

---

## Workflow

1. Determine type:
   ```text
   VO
   Entity
   Error
   Event
   Constant
   ```

2. Create the file:
   ```text
   taxonomy_<domain>_<concept>_<suffix>.rs
   ```
   inside:
   ```text
   crates/shared/src/<domain>/
   ```

3. For VOs:
   ```text
   Validate in new().
   Return Result<Self, DomainError>.
   Ensure fields are immutable (no pub mut or setters).
   No I/O.
   ```

4. For Entities:
   ```text
   Include an identity VO field.
   Use VO fields only for domain state.
   Do not use raw primitives for public/domain fields.
   ```

5. For Errors:
   ```text
   Implement std::error::Error and std::fmt::Display (usually via thiserror).
   Store VO fields only in enum variants.
   Expose an error_id field (stable numeric id) for fast machine branching.
   Expose an error_code field (plain string identifier) for human-readable
     error classification.
   Provide a message field derived from the stored VOs, error id, and error code
     so callers can inspect it without parsing the Display output.
   Do not store raw String or std::io::Error as domain payloads.
   ```

6. For Events:
   ```text
   Use standard structs with pub VO fields.
   Ensure immutability (no setters).
   Name events as past-tense domain facts.
   ```

7. For Constants:
   ```text
   Use pub const only.
   No functions.
   No lazy_static or once_cell.
   No I/O.
   ```

8. Register the module and re-export the public type in the domain `mod.rs`.

9. Verify the crate compiles:
   ```bash
   cargo check -p shared
   ```

---

## Template

### Value Object

```rust
use crate::common::taxonomy_validation_error::ValidationError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct <Name>(String);

impl <Name> {
    pub fn new(value: impl Into<String>) -> Result<Self, ValidationError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(ValidationError::empty("<Name>"));
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for <Name> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
```

Concrete example:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrderId(String);

impl OrderId {
    pub fn new(value: impl Into<String>) -> Result<Self, String> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err("OrderId cannot be empty".to_string());
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for OrderId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
```

---

### Entity

```rust
use crate::<domain>::taxonomy_<domain>_<concept>_id_vo::<Name>Id;
// import other VOs

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct <Name> {
    pub id: <Name>Id,
    // other VO fields
}

impl <Name> {
    pub fn new(id: <Name>Id /*, other VOs */) -> Self {
        Self { id /*, ... */ }
    }
}
```

Concrete example:

```rust
use crate::order::taxonomy_order_order_id_vo::OrderId;
use crate::order::taxonomy_order_money_vo::Money;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Order {
    pub id: OrderId,
    pub total: Money,
}

impl Order {
    pub fn new(id: OrderId, total: Money) -> Self {
        Self { id, total }
    }
}
```

Do not do this:

```rust
pub struct Order {
    pub id: String,
    pub total: f64,
}
```

Entity fields must be taxonomy VOs.

---

### Error

```rust
use thiserror::Error;
use crate::<domain>::taxonomy_<domain>_<concept>_vo::<VO>;

#[derive(Debug, Error)]
pub enum <Name>Error {
    #[error("<Description>: {0}")]
    <Variant>(<VO>),
}

impl <Name>Error {
    /// Stable numeric id. Callers branch on this, never on the message.
    pub fn error_id(&self) -> u16 {
        match self {
            Self::<Variant>(..) => <NNNN>,
        }
    }

    /// Stable machine-readable name. Callers branch on this, never on the message.
    pub fn error_code(&self) -> &'static str {
        match self {
            Self::<Variant>(..) => "<DOMAIN>_<REASON>",
        }
    }

    /// Human-readable description derived from the error id, code, and VOs.
    pub fn message(&self) -> String {
        match self {
            Self::<Variant>(<field>) => format!(
                "{} {}: <Description>: {}",
                self.error_id(),
                self.error_code(),
                <field>
            ),
        }
    }
}
```

Concrete example:

```rust
use thiserror::Error;
use crate::order::taxonomy_order_order_id_vo::OrderId;

#[derive(Debug, Error)]
pub enum OrderError {
    #[error("order not found: {0}")]
    NotFound(OrderId),
}

impl OrderError {
    /// Stable numeric id. Callers branch on this, never on the message.
    pub fn error_id(&self) -> u16 {
        match self {
            Self::NotFound(_) => 1001,
        }
    }

    /// Stable machine-readable name. Callers branch on this, never on the message.
    pub fn error_code(&self) -> &'static str {
        match self {
            Self::NotFound(_) => "ORDER_NOT_FOUND",
        }
    }

    /// Human-readable description derived from the error id, code, and VOs.
    pub fn message(&self) -> String {
        match self {
            Self::NotFound(order_id) => format!(
                "{} {}: order not found: {}",
                self.error_id(),
                self.error_code(),
                order_id
            ),
        }
    }
}
```

Every error must expose all four:
- **Field VO** — programmatic access to the domain data that caused the error
- **Error id** — a stable numeric id such as `1001`, so APIs, database rows,
  and monitoring queries can correlate on a compact value
- **Error code** — a stable string name such as `ORDER_NOT_FOUND`, readable in
  logs without a lookup table
- **Message** — a human-readable description derived from the error id, the
  error code, and the stored VOs

Both the error id and the error code stay stable across releases. Renaming or
reformatting the message is a compatible change; changing the id or the code
is a breaking one. Allocate ids in blocks per domain (order errors from 1000,
billing errors from 2000) so two domains cannot collide on the same number.

Good:

```rust
#[derive(Debug, Error)]
pub enum OrderError {
    #[error("order not found: {0}")]
    NotFound(OrderId),
}

impl OrderError {
    pub fn error_id(&self) -> u16 {
        match self {
            Self::NotFound(_) => 1001,
        }
    }

    pub fn error_code(&self) -> &'static str {
        match self {
            Self::NotFound(_) => "ORDER_NOT_FOUND",
        }
    }
}
```

Bad:

```rust
NotFound(String)                    // raw primitive instead of a VO
IoError(std::io::Error)             // raw std type as a domain payload
Variant(String)                     // raw message payload — no stable error id or code
```

---

### Event

```rust
use crate::<domain>::taxonomy_<domain>_<concept>_vo::<VO>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct <Name> {
    pub <field>: <VO>,
}

impl <Name> {
    pub fn new(<field>: <VO>) -> Self {
        Self { <field> }
    }
}
```

Concrete example:

```rust
use crate::order::taxonomy_order_order_id_vo::OrderId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderCreated {
    pub order_id: OrderId,
}

impl OrderCreated {
    pub fn new(order_id: OrderId) -> Self {
        Self { order_id }
    }
}
```

Events must be immutable and use VO payload fields only.

---

### Constants

Use the singular `_constant` suffix:

```text
taxonomy_<domain>_<concept>_constant.rs
```

Template:

```rust
/// Default value description.
pub const <NAME>_DEFAULT: f64 = 24.0;

/// Minimum value description.
pub const <NAME>_MIN: f64 = 0.5;

/// Filename constant.
pub const <NAME>_FILENAME: &str = "file.json";
```

Concrete example:

```rust
/// Default order refresh interval in hours.
pub const ORDER_REFRESH_INTERVAL_DEFAULT: f64 = 24.0;

/// Minimum order refresh interval in hours.
pub const ORDER_REFRESH_INTERVAL_MIN: f64 = 0.5;

/// Order snapshot filename.
pub const ORDER_SNAPSHOT_FILENAME: &str = "order_snapshot.json";
```

Constants must be pure literals.

Forbidden:

```rust
pub fn get_max_items() -> usize { 100 }
lazy_static! { pub static ref MAX_ITEMS: usize = 100; }
```

> The comment lines above each constant are not decoration — they keep the file
> above the AES302 five-line minimum. A bare two-constant file fails.

---

### Registration

Register each public taxonomy module and re-export the type in the domain `mod.rs`.

Example:

```rust
// crates/shared/src/order/mod.rs

pub mod taxonomy_order_order_id_vo;
pub mod taxonomy_order_order_entity;
pub mod taxonomy_order_order_not_found_error;
pub mod taxonomy_order_order_created_event;
pub mod taxonomy_order_order_constant;

pub use taxonomy_order_order_id_vo::OrderId;
pub use taxonomy_order_order_entity::Order;
pub use taxonomy_order_order_not_found_error::OrderError;
pub use taxonomy_order_order_created_event::OrderCreated;
```

---

## Section Contract

| Check | Enforcement | Why it belongs here |
|---|---|---|
| Correct file pattern: `taxonomy_<domain>_<concept>_<suffix>.rs`. | Machine-checked: AES101. | Required by AES layer rules and the linter; missing it is a defect. |
| Correct suffix: `_vo`, `_entity`, `_error`, `_event`, `_constant`. | Machine-checked: AES102. | Required by AES layer rules and the linter; missing it is a defect. |
| File is at least 5 lines. | Machine-checked: AES302. | Required by AES layer rules and the linter; missing it is a defect. |
| No forbidden layer imports from capabilities, agents, surface, root, contracts. | Partially machine-checked: AES201–AES205. See Verify. | Required by AES layer rules; missing it is a defect. |
| Registered in shared `mod.rs`. | Machine-checked: AES501 orphan check. | Required by AES layer rules and the linter; missing it is a defect. |
| Constant file contains only constant material, not structs/enums/functions. | Machine-checked: AES401 on constant files. | Required by AES layer rules and the linter; missing it is a defect. |
| One taxonomy type per file. | Convention — not machine-checked. The reader verifies this; the linter does not. | Required by AES taxonomy convention; missing it is a defect. |
| VOs validate on construction. | Convention — not machine-checked. The reader verifies this; the linter does not. | Required by AES taxonomy convention; missing it is a defect. |
| VOs are immutable. | Convention — not machine-checked. The reader verifies this; the linter does not. | Required by AES taxonomy convention; missing it is a defect. |
| Leaf VOs encapsulate only the primitive needed for validation. | Convention — not machine-checked. The reader verifies this; the linter does not. | Required by AES taxonomy convention; missing it is a defect. |
| Composite VOs use other VOs, not raw primitives. | Convention — not machine-checked. The reader verifies this; the linter does not. | Required by AES taxonomy convention; missing it is a defect. |
| Public/domain fields avoid primitives. | Machine-checked for `_entity`, `_error`, `_event`. Not checked for `_vo` files. | Required by AES taxonomy convention; missing it is a defect. |
| Entities contain an identity VO field. | Convention — not machine-checked. The reader verifies this; the linter does not. | Required by AES taxonomy convention; missing it is a defect. |
| Entities use VO fields for domain state. | Machine-checked — see primitive note above. | Required by AES taxonomy convention; missing it is a defect. |
| Events are immutable. | Convention — not machine-checked. The reader verifies this; the linter does not. | Required by AES taxonomy convention; missing it is a defect. |
| Events use VO payload fields only. | Machine-checked — see primitive note above. | Required by AES taxonomy convention; missing it is a defect. |
| Errors implement `std::error::Error` + `Display`. | Convention for semantic correctness; structural derivation is checked by compiler. | Required by AES taxonomy convention; missing it is a defect. |
| Errors store VO fields only. | Convention — not machine-checked. The reader verifies this; the linter does not. | Required by AES taxonomy convention; missing it is a defect. |
| Errors expose an `error_id` field (stable numeric id), an `error_code` field (stable string name), and a `message` field derived from their VOs. | Best practice — not machine-checked. Enables callers to branch on `error_id`/`error_code` and to read the description without parsing `Display`. | Required by AES best practice; missing it is a defect. |
| Constants are `pub const` pure literal values. | Convention for literal purity; structural violations may be machine-checked. | Required by AES taxonomy convention; missing it is a defect. |
| No I/O, network, database, filesystem, environment, randomness, or time retrieval. | Convention — not machine-checked fully. The reader verifies this; the linter does not guarantee it. | Required by AES taxonomy convention; missing it is a defect. |
| `cargo check -p <crate-name>` passes. | Manual fallback gate. | Required by the Rust compilation check; missing it is a defect. |

---

## Verify

```bash
lint-arwaky-cli scan <layer-path>
```

Machine-checked areas include:

```text
AES101: filename pattern
AES102: suffix correctness
AES203: unused imports
AES204: dummy functions
AES205: circular dependencies
AES301/AES302: file length bounds
AES401: direct primitives in _entity, _error, _event files
AES501–AES506: orphan files and reachability
```

### What is convention, not enforcement

```text
Import boundary rules (AES201/AES202) — the import matrix above is the target
  architecture. Do not treat a clean scan as proof the boundaries hold.

Primitive rules (AES401 on _vo files) — VOs are not scanned for primitives.
  A `pub amount: f64` inside a `_vo` file will not trigger AES401. Trust
  discipline: keep VO internals private and expose through accessor methods.

Class/inheritance rules (AES403/AES405) — verify these by reading.
```

A pass means the mechanically-checkable rules are satisfied. The boundary and
primitive rules above are the reader's responsibility.

### Orphan reachability — read this before Phase 2

A taxonomy file that is only referenced by barrels and other taxonomy files
is reported as an orphan:

```
[AES501] 'taxonomy_order_order_id_vo' is not reachable and not imported by higher layers.
WHY? only imported by lower-layer files (mod.rs, taxonomy_order_entity.rs)
FIX: Import 'taxonomy_order_order_id_vo' from a contract_* or higher-layer file.
```

Barrel registration alone does **not** clear this. The shared barrel satisfies
the "has importers" half; you also need a `contract_*` file to import the
taxonomy type. However, the contract itself becomes an orphan (`AES502`)
until it is referenced by a capability, agent, or root file. The fix is
structural, not per-file: wire the full chain through root, then rescan.
Individual layer phases are expected to show orphan violations — they clear
once the complete dependency graph is in place. Do not treat an isolated
taxonomy scan as the final gate.

Fallback compile gate:

```bash
cargo check -p shared
```

Or if your crate name differs:

```bash
cargo check -p <crate-name>
```

Related: [HOW-TO-MAKE-PYTHON-TAXONOMY.md](HOW-TO-MAKE-PYTHON-TAXONOMY.md), [HOW-TO-MAKE-TYPESCRIPT-TAXONOMY.md](HOW-TO-MAKE-TYPESCRIPT-TAXONOMY.md)
