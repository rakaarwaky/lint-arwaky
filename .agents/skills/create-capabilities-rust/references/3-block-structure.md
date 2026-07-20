# The 3-Block Structure

Every implementation file MUST follow this order:

1. **Block 1 — Struct Definition**
2. **Block 2 — Domain Protocol Trait Implementation**
3. **Block 3 — Constructors, Std Traits, and Private Helpers**

## Block 1 — Struct Definition

```rust
pub struct Capabilities<NameCapability>;
```

Or with dependencies:

```rust
pub struct Capabilities<NameCapability> {
    collaborator: Arc<dyn I<NameCollaborator>Protocol>,
    store: Arc<dyn I<NameStore>Protocol>,
    policy: <NamePolicy>VO,
}
```

## Block 2 — Public Contract

Block 2 is RESERVED for the domain protocol trait ONLY.

```rust
impl I<NameCapability>Protocol for Capabilities<NameCapability> {
    fn execute(
        &self,
        input: &<DomainVO>,
        output: &mut Vec<<ResultVO>>,
    ) {
        // domain behavior
    }
}
```

Do NOT put these in Block 2:

```rust
impl Default for Capabilities<NameCapability>
impl Clone for Capabilities<NameCapability>
impl Debug for Capabilities<NameCapability>
impl Display for Capabilities<NameCapability>
impl From<...> for Capabilities<NameCapability>
```

Those belong in Block 3.

## Block 3 — Constructors, Std Traits, and Helpers

Block 3 contains:

- `new()`
- builders
- `Default`, `Clone`, `Debug`, `Display`
- other std trait impls
- private helper methods
- domain-specific associated functions used only by this struct

```rust
impl Default for Capabilities<NameCapability> {
    fn default() -> Self {
        Self
    }
}

impl Capabilities<NameCapability> {
    pub fn new() -> Self {
        Self
    }

    fn effective_threshold(&self, input: &<DomainVO>) -> <Threshold>VO {
        // private helper
    }
}
```

Block 3 MUST NOT:

- define domain models (Entities, Value Objects) — that is **No Domain Definition** (ARCHITECTURE §8); consume them from Taxonomy instead.
- perform orchestration — no flow control across capabilities, no error-escalation policy (**No Orchestration**, ARCHITECTURE §8).
- duplicate technical mechanics that belong in a Utility standalone function (**DRY**, ARCHITECTURE §8).

## Trait Placement Decision Rule

```text
Trait impl found in a capabilities file?
  │
  ├─ Is it the domain protocol? (I<Name>Protocol)
  │   └─ YES → Block 2
  │
  └─ Is it a std/derive/utility trait? (Default, Clone, Debug, Display, From, etc.)
      └─ YES → Block 3
```

## Method Placement Decision Rule

```text
Method / function found in a capabilities file?
  │
  ├─ Is it defined as the domain protocol trait method?
  │   └─ YES → Block 2
  │
  ├─ Is it a std/derive trait impl?
  │   └─ YES → Block 3
  │
  ├─ Is it a factory method? (new(), builder)
  │   └─ YES → Block 3
  │
  └─ Is it a private helper?
      └─ YES → Block 3
```
