---
name: create-contract-rust
description: "Create and validate Rust contract layer files in shared domain: pure trait definitions for protocols and aggregates. Contracts define public promises only, with no implementation, no layer imports, and domain-safe VO-based signatures."
metadata:
  tags: [rust, aes, contract, protocol, aggregate, trait, vo]
  triggers:
    - "create contract rust"
    - "add contract rust"
    - "create protocol rust"
    - "create aggregate rust"
    - "contract missing rust"
    - "validate contract rust"
    - "check contract rust"
  dependencies: []
  related:
    - create-capabilities-rust
    - create-agent-rust
    - create-taxonomy-rust
---
# create-contract-rust

Contract = pure trait definitions. No default implementations. File: `contract_<concept>_<suffix>.rs`.

**Allowed imports:** taxonomy types, other contract types.
**Forbidden:** capabilities, agents, surface, root.

## Contract Roles


| Suffix       | Implemented By | Used By |
| -------------- | ---------------- | --------- |
| `_protocol`  | Capabilities   | Agent   |
| `_aggregate` | Agent          | Surface |

Naming: `I<Name>Protocol`, `I<Name>Aggregate`.

## Rules

- `pub trait` only — methods end with `;`, no bodies.
- No private helper signatures.
- All methods type-annotated.
- Object-safe by default.
- Signatures use shared VOs — no `String`/`i32`..`u64`/`f32`/`f64`/`Vec<String>` for domain values.
- `bool` and `&str` (for non-domain input) allowed with care.
- Register in shared `mod.rs`.

## Templates

### Protocol trait

```rust
use shared::<domain>::taxonomy_<name>_vo::<VO>;

pub trait I<Name>Protocol: Send + Sync {
    fn method_name(
        &self,
        param: &VO,
    );
}
```

### Aggregate trait

```rust
use shared::<domain>::taxonomy_<name>_vo::<VO>;

pub trait I<Name>Aggregate: Send + Sync {
    fn execute(
        &self,
        request: &ScanRequest,
    ) -> Vec<LintResult>;
}

pub trait I<Name>Aggregate:
    I<Name>rotocol
    + I<Name>Protocol
    + I<Name>Protocol
    + I<Name>Protocol
    + I<Name>Protocol
{
    /// All discovered source files .
    fn <Name>_<Name>(&self) -> &[FileEntry];

    /// Read file content from bounded cache.
    fn <Name>_<Name>(&self, path: &FilePath) -> ContentString;

    /// Get cached file content (after scan).
    fn <Name>_<Name>(&self, path: &Path) -> Option<String>;

    /// Check if a file is in the cache.
    fn <Name>_<Name>(&self, path: &Path) -> bool;
}
```

### mod.rs

```rust
// <domain> — contract traits for <domain> operations
pub mod contract_<name>_protocol;
pub mod contract_<name>_aggregate;
```

## Workflow

1. Which layer implements this? Capabilities → `_protocol`. Agent → `_aggregate`.
2. Golden Rule: only methods called by outer layers go in the trait.
3. Create `contract_<concept>_<suffix>.rs` in shared domain.
4. Register in `mod.rs`.
5. `cargo check -p <crate-name>`.

## Checklist

- [ ]  Correct suffix `_protocol` or `_aggregate`.
- [ ]  `pub trait` only — no default method bodies.
- [ ]  All methods type-annotated.
- [ ]  No imports from capabilities, agents, surface.
- [ ]  Signatures use shared VOs.
- [ ]  Registered in shared `mod.rs`.
- [ ]  `cargo check -p <crate-name>` passes.
