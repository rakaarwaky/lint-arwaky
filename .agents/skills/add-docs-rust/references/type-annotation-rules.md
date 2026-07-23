# Type Annotation Rules

## Purpose

Type annotations provide type safety and IDE support.

## Rules

1. **All function parameters** — must have type annotations
2. **All function return types** — must have type annotations
3. **Use traits** — for abstract behavior
4. **Use enums** — for sum types
5. **Use structs** — for product types
6. **Use generics** — for reusable types
7. **Use `Result<T, E>`** — for error handling

## Template

```rust
// Struct for product types
struct ImportRule {
    pattern: String,
    message: String,
}

// Enum for sum types
enum ValidationResult {
    Valid,
    Invalid(String),
}

// Function with type annotations
fn validate(data: &HashMap<String, Value>) -> Result<(bool, String), Error> {
    // ...
}

// Generic function
fn first<T>(items: &[T]) -> Option<&T> {
    items.first()
}
```

## Anti-Patterns

- ❌ Missing type annotations → add types to all parameters and returns
- ❌ Using `unwrap()` without reason → handle errors properly
- ❌ Missing trait for abstract behavior → define trait
- ❌ Missing enum for sum types → define enum
