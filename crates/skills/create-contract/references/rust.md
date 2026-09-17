# Contract — Rust

File: `contract_<concept>_<suffix>.rs`. Contract = pure trait definitions. No default implementations.

## Import rules

**Allowed imports:** taxonomy types, other contract types.
**Forbidden:** capabilities, agents, surface, root.

## Rules (Rust)

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
```

### Aggregate trait — extended cache form

> Carried verbatim from the pre-merge Rust contract skill. **This template is malformed in
> the source** (a duplicated `pub trait I<Name>Aggregate` header containing the literal placeholder
> text `... (3 duplicate lines)`, and method names that are `<Name>_<Name>`). Read it as an intent
> sketch — an aggregate that also composes its protocol and exposes a bounded file cache — and fix
> the header before copying it.

```rust
pub trait I<Name>Aggregate:
    I<Name>rotocol
    + I<Name>Protocol
  ... (3 duplicate lines)
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
