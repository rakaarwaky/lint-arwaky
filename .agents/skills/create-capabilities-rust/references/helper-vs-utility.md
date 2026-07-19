# Helper vs Utility Decision

The boundary is not only about `&self`.

The real question is:

> Does this function know about specific business/domain rules, or is it just a blind reusable tool?
>
> AND
>
> Is it used only by this struct, or by multiple modules?

## Keep as Private Helper in Block 3

Keep the function inside the capabilities file if ANY of these is true:

1. It contains business/domain rules.
2. It knows AES-specific patterns, layer names, suffixes, violation codes, or taxonomy conventions.
3. It accesses `self.field` or instance state.
4. It is tightly coupled to this capability only.
5. It is a factory method such as `new()` or builder method.
6. It is stateless but only used by this one struct and is domain-specific.

Example:

```rust
impl ContractRoleChecker {
    fn resolve_scope(scope: &str) -> (&str, Vec<&str>) {
        // Domain-specific parsing logic.
        // Even without `&self`, this can remain a private helper
        // if only this checker uses it.
    }
}
```

## Extract to Utility (`*_utility.rs`)

Extract the function to shared taxonomy utility ONLY if ALL of these are true:

1. Stateless: no `&self`, no struct field access.
2. Pure: input A always produces output B.
3. No side effects: no I/O, no network, no database, no global mutation.
4. Domain-agnostic: does not know business rules.
5. Reusable: useful for multiple capabilities/infrastructures/modules.

Example:

```rust
// shared/code_analysis/taxonomy_string_utility.rs
pub fn match_whole_token(haystack: &str, needle: &str) -> bool {
    // generic token matching
}
```

## I/O Blocker

A function may be stateless, but if it performs I/O, it MUST NOT become a taxonomy utility.

It also MUST NOT stay in capabilities.

```rust
// BAD in capabilities layer
fn read_config(file_path: &str) -> Option<String> {
    std::fs::read_to_string(file_path).ok()
}
```

Rule:

```text
Stateless + I/O = infrastructure/port implementation
NOT taxonomy utility
NOT capabilities layer
```

## Decision Tree

```text
Found reusable code in capabilities?
  │
  ├─ Does it know business/domain rules?
  │   └─ YES → keep as private helper in Block 3
  │
  ├─ Does it need &self or struct state?
  │   └─ YES → keep as helper/method in Block 3
  │
  ├─ Does it perform I/O or side effects?
  │   └─ YES → move to infrastructure/port implementation
  │
  └─ Is it stateless, pure, domain-agnostic, and reusable?
      └─ YES → extract to shared taxonomy utility
```
