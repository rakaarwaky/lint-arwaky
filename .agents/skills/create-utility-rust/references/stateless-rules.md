# Stateless Rules (Utility)

## Rule 1: No Struct, No Impl

Utility files must NOT contain:

- `pub struct` definitions
- `impl` blocks
- `&self` method receivers

Bad:

```rust
pub struct UtilityHelper {
    cache: HashMap<String, String>,
}

impl UtilityHelper {
    fn process(&self, input: &str) -> String {
        // BAD — has state
    }
}
```

Good:

```rust
pub fn process(input: &str) -> String {
    // pure function, no state
    input.to_uppercase()
}
```

## Rule 2: Pure Functions Only

A utility function must satisfy:

- **Deterministic:** same input → same output every time
- **No side effects:** no I/O (unless domain-agnostic + reusable), no network, no database
- **No randomness:** no `rand`, no `thread_rng`
- **No global state:** no `static mut`, no `lazy_static` mutation

Bad:

```rust
pub fn get_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
```

Good:

```rust
pub fn normalize_path(path: &str) -> String {
    path.replace('\\', "/").trim_start_matches("./").to_string()
}
```

## Rule 3: Domain Agnostic

Utility functions must NOT know about:

- Architecture layer names (agent, capabilities, contract, etc.)
- Business domain rules (naming conventions, import policies)
- Specific capability logic (how a checker validates)

Bad:

```rust
// BAD — knows about architecture layers
pub fn is_in_agent_layer(filename: &str) -> bool {
    filename.starts_with("agent_")
}
```

Good:

```rust
// GOOD — generic string operation
pub fn starts_with_prefix(prefix: &str, filename: &str) -> bool {
    filename.starts_with(prefix)
}
```

## Rule 4: Reusable Across Modules

Utility functions must be useful for multiple modules.

If a function is only used by one capability or one agent → keep as private helper in Block 3.

Decision Tree:

```text
Found reusable code?
  │
  ├─ Used by ≥2 modules?
  │   ├─ YES → check stateless + pure + domain-agnostic
  │   │         └─ All YES → extract to utility
  │   └─ NO → keep as private helper in Block 3
```

## I/O Exception

Utility CAN perform I/O if ALL conditions are met:

1. Stateless (no `&self`, no struct fields)
2. Domain-agnostic (no business knowledge)
3. Reusable across multiple modules

Good examples:

```rust
// OK — stateless, domain-agnostic, reusable
pub fn walk_source_files(dir: &Path, extensions: &[&str]) -> Vec<PathBuf> {
    let mut files = Vec::new();
    // I/O is allowed here
}
```
