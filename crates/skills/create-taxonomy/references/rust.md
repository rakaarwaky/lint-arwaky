# Taxonomy — Rust

Source layout: `crates/shared/src/<domain>/`. File: `taxonomy_<domain>_<type>.rs`.

## Import rules

**Allowed imports:** other taxonomy types, std.
**Forbidden:** capabilities, agents, surface, root, contracts, `std::fs`/network/database (in VOs/entities/errors/events/constants).

## File-name suffix table

| Suffix         | Content                | Key constraint                               |
| ---------------- | ------------------------ | ---------------------------------------------- |
| `_vo.rs`       | Value Objects          | Validate in`new()`, immutable fields, no I/O |
| `_entity.rs`   | Entities with identity | Identity VO field required                   |
| `_error.rs`    | Domain errors          | Implement`std::error::Error` + `Display`     |
| `_event.rs`    | Domain events          | Immutable, VO payload fields                 |
| `_constant.rs` | Compile-time constants | `pub const` only — no functions             |
| `_utility.rs`  | Stateless helpers      | No struct, no`impl`, domain-agnostic         |

## VO primitive rules (AES401)

Forbidden for domain fields: `String`, `i32`..`u64`, `f32`/`f64`, `Vec<String>`.
`bool` and `&str` (for non-domain borrowed input) allowed with care.

## Templates

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

### Entity

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

### Error

```rust
use thiserror::Error;

use crate::<domain>::taxonomy_<name>_vo::<VO>;

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
- **`error_id`** — a stable numeric id such as `1001`, for API/DB/monitoring correlation
- **`error_code`** — a stable string name such as `ORDER_NOT_FOUND`, readable in logs
- **`message`** — a human-readable description derived from the id, code, and VOs

Both the `error_id` and `error_code` stay stable across releases. Renaming or
reformatting `message` is a compatible change; changing either id or code is
breaking. Allocate ids in per-domain blocks (order from 1000, billing from 2000)
to avoid cross-domain collisions.

Never store a raw `String` or `std::io::Error` as a domain payload — the VO,
`error_id`, and `error_code` carry the structure; `message` carries the prose.
```

### Constants

```rust
/// Default value description.
pub const <NAME>_DEFAULT: f64 = 24.0;

/// Minimum value description.
pub const <NAME>_MIN: f64 = 0.5;

/// Filename constant.
pub const <NAME>_FILENAME: &str = "file.json";
```

## Workflow

1. Determine type (VO/Entity/Error/Event/Constant/Utility).
2. Create `taxonomy_<domain>_<type>.rs` in `shared/src/<domain>/`.
3. VOs: `fn new(...) -> Result<Self, DomainError>` or invariant check in `new`.
4. Errors: impl `std::error::Error` + `Display`; expose `error_id`, `error_code`,
   and `message` derived from the stored VOs.
5. Constants: `pub const NAME: Type = value;` only.
6. Register in `mod.rs`.
7. `cargo check -p <crate-name>`.

## Checklist

- [ ]  Correct suffix.
- [ ]  VOs validate on construction; composite VOs use other VOs (no raw primitives).
- [ ]  Errors implement `std::error::Error`, expose `error_id`, `error_code`, and `message`.
- [ ]  Constants are `pub const` pure literal values.
- [ ]  No import from capabilities, agents, surface, root, contracts.
- [ ]  No I/O, network, or database in taxonomy files.
- [ ]  Registered in shared `mod.rs`.
- [ ]  `cargo check -p <crate-name>` passes.
